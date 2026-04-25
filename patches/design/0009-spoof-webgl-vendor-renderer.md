# patch 0009-spoof-webgl-vendor-renderer

The single biggest container tell. Without a GPU, Chromium falls back to SwiftShader and reports literal strings:

```
UNMASKED_VENDOR_WEBGL   = "Google Inc. (Google)"
UNMASKED_RENDERER_WEBGL = "ANGLE (Google, Vulkan 1.3.0 (SwiftShader Device (Subzero) (0x0000C0DE)), SwiftShader driver)"
```

Any anti-bot vendor pattern-matches "SwiftShader" or "0x0000C0DE" and you're flagged before any other check runs.

## Detection vector

```js
const gl = document.createElement('canvas').getContext('webgl');
const ext = gl.getExtension('WEBGL_debug_renderer_info');
console.log(gl.getParameter(ext.UNMASKED_VENDOR_WEBGL));
console.log(gl.getParameter(ext.UNMASKED_RENDERER_WEBGL));
```

WebGL2 has the same parameters; WebGPU exposes adapter info via `GPUAdapterInfo.vendor` and `.architecture`.

## Target files

| File | Change |
|---|---|
| `third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc` | Override `GetUnmaskedVendor` / `GetUnmaskedRenderer` to return profile values |
| `third_party/blink/renderer/modules/webgl/webgl2_rendering_context.cc` | WebGL2 inherits but verify the override path is hit |
| `third_party/blink/renderer/modules/webgpu/gpu_adapter_info.cc` | `GPUAdapterInfo::vendor()` / `architecture()` / `device()` from profile |
| `gpu/config/gpu_info.cc` | If string is sourced from `GPUInfo` rather than asked dynamically, override at construction |

## Profile fields read

```
gpu.vendor          → UNMASKED_VENDOR_WEBGL, GPUAdapterInfo.vendor
gpu.renderer        → UNMASKED_RENDERER_WEBGL
gpu.vendor_id       → not exposed to JS but sometimes leaks via GPUAdapterInfo
gpu.device_id       → same
gpu.webgpu_adapter  → GPUAdapterInfo.{vendor,architecture,device}
```

## Implementation notes

- **Vendor strings come from multiple sources.** When real GPU is present, ANGLE pulls them from the driver. When SwiftShader is the backend, they come from a hardcoded SwiftShader string. Override the *consumer* (`webgl_rendering_context_base.cc`) rather than each *producer* — one patch covers both backends.
- **Don't break the `WEBGL_debug_renderer_info` extension's existence.** Some sites probe `gl.getExtension('WEBGL_debug_renderer_info') === null`; if you remove the extension, that check itself becomes a tell. Keep the extension, override the parameter values.
- **Context attributes.** `gl.getContextAttributes()` reports `failIfMajorPerformanceCaveat`. SwiftShader contexts often advertise a perf caveat that real GPUs don't. May need to mask this too.
- **Pixel-level fingerprints unfixed by this patch.** Even with vendor strings spoofed, software rendering produces *pixel hashes* that don't match real GPU output. That's solved by patch 0013-canvas-deterministic-noise (inject per-profile deterministic noise into the readback), or by running the GPU variant of cosmium with real hardware. This patch alone is not sufficient on `cosmium:cpu`.
- **Worker contexts.** OffscreenCanvas + WebGL in a Worker has a separate code path. Patch reaches it via the same `WebGLRenderingContextBase` but verify.

## Validation

```js
const gl = document.createElement('canvas').getContext('webgl2');
const ext = gl.getExtension('WEBGL_debug_renderer_info');
console.log('vendor:',   gl.getParameter(ext.UNMASKED_VENDOR_WEBGL));
console.log('renderer:', gl.getParameter(ext.UNMASKED_RENDERER_WEBGL));

// Should match profile.gpu.vendor / profile.gpu.renderer EXACTLY,
// not contain "SwiftShader" or "0x0000C0DE".
```

Sites: https://browserleaks.com/webgl, https://webglreport.com/, creepjs WebGL section.
