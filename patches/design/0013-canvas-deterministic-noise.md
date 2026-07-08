# patch 0013-canvas-deterministic-noise

`HTMLCanvasElement.toDataURL()` and `getImageData()` produce pixel-perfect identical output across two sessions on the same host — that bit-exact reproducibility is the entire basis of canvas fingerprinting. Real users have stable hashes too, but the *value* of the hash leaks the GPU, font renderer, and OS.

Brave-style randomization breaks fingerprinting but introduces a new tell: the hash changes per session. Detectors flip from "I know this device" to "I know this is using anti-fingerprint" — still profilable.

The cosmium answer: **deterministic noise seeded by the profile**, so the hash is stable per profile but doesn't match the host.

## Detection vector

```js
const c = document.createElement('canvas');
c.width = 200; c.height = 50;
const ctx = c.getContext('2d');
ctx.textBaseline = 'top';
ctx.font = '16px Arial';
ctx.fillText('Cwm fjordbank glyphs vext quiz', 2, 15);
const hash = await crypto.subtle.digest('SHA-256',
  Uint8Array.from(c.toDataURL().split(',')[1], c => c.charCodeAt(0)));
```

CreepJS computes two hashes:
- `data:` — `toDataURL()` of the rendered string
- `pixels:` — flat hash of `getImageData()` pixel buffer

Your current output: `data: 1b8e2ea7`, `pixels: a7b4ffc0` — stable, identifies the host across all browsers.

## Target files

| File | Change |
|---|---|
| `third_party/blink/renderer/core/html/canvas/canvas_rendering_context_2d.cc` | Wrap `getImageData()` and `toBlob()` paths to apply post-render noise |
| `third_party/blink/renderer/core/html/canvas/html_canvas_element.cc` | Same for `toDataURL()` |
| `third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc` | `readPixels()` path (WebGL canvas fingerprinting) |
| `third_party/blink/renderer/modules/webgpu/gpu_texture.cc` | Future: WebGPU readback |

## Profile fields read

```
canvas_noise.seed → 32-hex-char (16-byte) PRNG seed
```

Single field. Validator already enforces 32 hex chars.

**Switch (now wired in `apps/engine/src/domain/runtime/flags/switches.rs`):**
`--cosmium-canvas-seed=<32-hex>`. Read it in C++ via
`base::CommandLine::ForCurrentProcess()->GetSwitchValueASCII("cosmium-canvas-seed")`,
mirroring the shipped `0002`/`0010` pattern.

## Implementation notes

- **Where to apply noise.** *After* compositing, *before* the readback returns to JS. The actual rendered pixels on screen stay untouched — only readback APIs see noise. Visual output is identical to vanilla Chrome.
- **PRNG.** Seed splitmix64 with `seed XOR (canvas.width << 16 | canvas.height)`. This makes the noise canvas-dimension-dependent so two different-sized canvases produce different patterns from the same seed (real GPU rounding differs per resolution).
- **Noise distribution.** Per-pixel: flip 0-2 of the bottom 2 bits of each RGB channel. Alpha untouched. ~3% of pixels modified. Invisible to humans, decisive for fingerprint hashes.
- **Determinism.** Two reads of the same canvas in the same session must produce identical output (sites do this as a sanity check). Cache the noise mask per `(seed, width, height)` tuple. Invalidate when canvas content changes via a content-hash check.
- **Cross-frame canvases.** Each frame has its own canvas; same seed, same dimensions → same noise. Sites that render the same probe in two iframes will get matching hashes (real Chrome behavior).
- **`willReadFrequently: true` 2D contexts.** These skip GPU rasterization. Apply noise consistently.
- **WebGL `preserveDrawingBuffer`.** When false (default), readback after compositing is undefined per spec — vanilla Chrome returns zeros. Don't add noise; honor the spec, the zeros are themselves the value.

## Determinism vs. anti-replay tradeoff

Deterministic per-profile means two scrapers using the same profile produce the same hash → trivially linkable to each other. Mitigation: rotate the `canvas_noise.seed` on every `cosmium profile mutate` invocation (LLM should treat seed as ephemeral). Validator should warn if the seed has been unchanged across a session count > 50.

## Validation

```js
async function probe() {
  const c = document.createElement('canvas');
  c.width = 200; c.height = 50;
  const ctx = c.getContext('2d');
  ctx.font = '16px Arial';
  ctx.fillText('cosmium test', 2, 15);
  return c.toDataURL();
}
const a = await probe();
const b = await probe();
console.log('stable:', a === b);          // must be true within a session
console.log('hash:', a.slice(0, 60));     // must differ from host's hash
```

Test sites: https://browserleaks.com/canvas, https://abrahamjuliot.github.io/creepjs/.
