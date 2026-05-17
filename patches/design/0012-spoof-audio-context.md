# patch 0012-spoof-audio-context (channel count + sample rate)

`AudioContext` exposes seven fingerprintable scalars: `sampleRate`, `baseLatency`, `outputLatency`, `destination.maxChannelCount`, plus derived per-buffer hashes (CreepJS `Audio` probe: `sum`, `gain`, `freq`, `time`, `trap`).

The "easy" latency surface ships as patch [`0010-spoof-audio-context-latency`](../0010-spoof-audio-context-latency.patch). This doc covers the harder parts: **channel count**, **sample rate**, and the **buffer-hash** that CreepJS computes by rendering a sine through a `DynamicsCompressor`.

## Detection vector

```js
const ctx = new OfflineAudioContext(1, 44100, 44100);
const osc = ctx.createOscillator();
const comp = ctx.createDynamicsCompressor();
osc.connect(comp).connect(ctx.destination);
osc.start(0);
const buf = await ctx.startRendering();
const samples = buf.getChannelData(0).slice(4500, 5000);
// CreepJS reduces samples → "sum: 124.04347527516074"
```

Different CPUs round float math fractionally differently. The bottom bits of the rendered sum encode CPU vendor + libc + audio backend. A Linux container with PulseAudio produces a different sum than macOS CoreAudio.

## Target files

| File | Change |
|---|---|
| `third_party/blink/renderer/modules/webaudio/audio_destination_node.cc` | `maxChannelCount` reads `--cosmium-audio-max-channels` |
| `third_party/blink/renderer/modules/webaudio/base_audio_context.cc` | `sampleRate()` reads `--cosmium-audio-sample-rate` |
| `third_party/blink/renderer/modules/webaudio/offline_audio_context.cc` | renderer adds deterministic jitter from `canvas_noise.seed` to output buffer |
| `third_party/blink/renderer/platform/audio/audio_array.h` (or wrapper) | Per-sample noise injection helper |

## Profile fields read

```
audio.sample_rate         → uint32 (real: 44100 / 48000)
audio.max_channel_count   → uint32 (real: 2)
canvas_noise.seed         → 16-byte hex; seeds a per-render PRNG for sub-LSB jitter
```

## Implementation notes

- **PRNG choice.** Use a small splitmix64 keyed by (`seed`, `renderQuantum`, `channelIndex`). Deterministic across sessions for the same profile — sites that hash twice still see the same value, but the value matches `profile.audio.*` instead of the host.
- **Magnitude of noise.** Real Chrome's `DynamicsCompressor` produces samples in `[-1.0, 1.0]` with quantization at `~1e-7`. Add jitter in `[-1e-8, +1e-8]` — invisible to humans, swamps the platform-specific bottom bits, doesn't break legitimate audio processing.
- **Don't touch playback path.** Patch only the *offline* rendering path that fingerprinters use. Live `AudioContext` with `destination` plugged into hardware should remain bit-exact, otherwise music/calls break.
- **`baseLatency`/`outputLatency`.** Already handled by patch 0010 — this patch additionally exposes the latency PAIR is internally consistent with `sampleRate` (latency = N / sampleRate for some integer N).
- **CreepJS-specific hash.** Their probe slices `[4500, 5000]` from a 44100-sample render. Make sure jitter spans this range or it won't affect the hash.

## Validation

CreepJS `Audio` section should show:
- `sum` differs from host's native render
- `sum` is **stable** across reloads for the same profile (deterministic)
- `sum` differs between two distinct profiles

```js
async function audioFingerprint() {
  const ctx = new OfflineAudioContext(1, 44100, 44100);
  const osc = ctx.createOscillator();
  const comp = ctx.createDynamicsCompressor();
  comp.threshold.value = -50;
  comp.knee.value = 40;
  comp.ratio.value = 12;
  comp.attack.value = 0;
  comp.release.value = 0.25;
  osc.connect(comp);
  comp.connect(ctx.destination);
  osc.start(0);
  const buf = await ctx.startRendering();
  return buf.getChannelData(0).slice(4500, 5000)
    .reduce((s, v) => s + Math.abs(v), 0);
}
audioFingerprint().then(console.log);
```

Run twice with same profile → identical output. Run with different profile → different output. Run on host vs patched → different output.
