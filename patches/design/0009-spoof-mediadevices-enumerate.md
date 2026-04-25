# patch 0009-spoof-mediadevices-enumerate (deferred)

Empty `navigator.mediaDevices.enumerateDevices()` is THE container tell — but the patch is significantly more complex than the others and warrants verification against the real source tree before authoring.

## Why deferred

`MediaDevices::enumerateDevices()` doesn't return values directly — it dispatches a Mojo IPC to the browser process and resolves a promise via the `DevicesEnumerated` callback. Two patch points are possible:

1. **At the IPC dispatch site** (`media_devices.cc:432`) — short-circuit the Mojo call and resolve with synthetic data. Requires constructing `MediaDeviceInfo` and `InputDeviceInfo` garbage-collected objects directly, navigating the `ScriptPromiseResolverWithTracker` resolution path.
2. **At the callback site** (`media_devices.cc:1275`, `DevicesEnumerated`) — accept the real (empty) Mojo response, then mutate the `Vector<Vector<WebMediaDeviceInfo>>` before the loop that builds the JS-visible objects.

Both paths require knowing the exact ordering of the `mojom::blink::MediaDeviceType` enum (kMediaAudioInput vs kMediaVideoInput vs kMediaAudioOutput indices) which can shift between Chromium versions. Getting this wrong injects audio devices into the video slot or vice versa — visible, detectable, worse than no patch.

## Recommended path forward

1. After the build pipeline produces a working binary with the other 8 patches applied, verify on real detection sites (creepjs, browserleaks) which categories of mediaDevices need spoofing for the user's specific scraping targets.
2. If empty mediaDevices is actually blocking specific sites, fetch `services/audio/public/cpp/...` and `third_party/blink/public/common/mediastream/media_devices.mojom` from the same pinned tag, confirm enum ordering, then author the patch against the callback site (option 2) — safer than option 1 because the JS-visible flow is unchanged, only the input data changes.
3. Switch interface (proposed): `--cosmium-fake-media-devices` as a boolean flag that injects a fixed-but-believable set (1 audioinput, 1 audiooutput, 1 videoinput, all with empty labels matching real-Chrome-pre-permission behavior).

## CDP fallback in the meantime

While this patch is deferred, mrscraper-rs can spoof mediaDevices via CDP injection at session start:

```js
Object.defineProperty(MediaDevices.prototype, 'enumerateDevices', {
  value: () => Promise.resolve([
    { kind: 'audioinput', label: '', deviceId: 'default', groupId: 'g1', toJSON: () => ({}) },
    { kind: 'audiooutput', label: '', deviceId: 'default', groupId: 'g1', toJSON: () => ({}) },
    { kind: 'videoinput', label: '', deviceId: 'v1', groupId: 'g2', toJSON: () => ({}) },
  ]),
});
```

CDP-level spoofing is detectable (override leaks via `enumerateDevices.toString()`) but works for less sophisticated detectors. The C++ patch is the eventual answer.
