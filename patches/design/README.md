# Patch design backlog

Each file in this directory is a design doc for a planned C++ patch. Format per file:

```
# patch <number>-<short-name>

## Detection vector
What sites probe, why containers fail it.

## Target files
Specific Chromium source paths and symbols.

## Profile fields read
Which keys from the cosmium profile drive this patch.

## Implementation notes
Subtleties — Worker threads, isolated worlds, ServiceWorker scope, cross-origin frames.

## Validation
How to verify the patch worked — test page URL or detection signature.
```

Once written, replace the `.md` with a `.patch` and add to `../series`.

## Backlog (priority order)

1. [`0001-cosmium-profile-loader.md`](./0001-cosmium-profile-loader.md) — **prerequisite for all others**
2. [`0002-strip-automation-controlled.md`](./0002-strip-automation-controlled.md)
3. [`0003-spoof-client-hints.md`](./0003-spoof-client-hints.md)
4. `0004-spoof-navigator-platform.md`
5. `0005-spoof-navigator-languages.md`
6. `0006-spoof-hardware-concurrency.md`
7. `0007-spoof-device-memory.md`
8. `0008-spoof-screen-properties.md`
9. [`0009-spoof-webgl-vendor-renderer.md`](./0009-spoof-webgl-vendor-renderer.md) — biggest single win for containers
10. `0010-spoof-mediadevices-enumerate.md`
11. `0011-spoof-speech-synthesis-voices.md`
12. `0012-spoof-audio-context.md`
13. `0013-canvas-deterministic-noise.md`
14. `0014-spoof-font-enumeration.md`
15. [`0015-webrtc-ip-policy.md`](./0015-webrtc-ip-policy.md)
16. `0016-strip-cdp-runtime-leaks.md`
17. `0017-strip-headless-new-tells.md`
