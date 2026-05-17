# patch 0014-spoof-font-enumeration

Font enumeration is the highest-entropy non-graphical fingerprint. CreepJS probes 51 fonts via the bounding-box trick (measure "x" in `font: 16px Like undefined` vs `font: 16px 'Helvetica', Like undefined`) and reports which 8/51 are installed plus a list of present families. Linux-distro fonts on a macOS UA → game over.

## Detection vector

Three probes:

1. **`document.fonts.check('16px Helvetica')`** (CSS Font Loading API). Returns true if installed.
2. **Bounding-box measurement**. Render `<canvas><span style="font:16px Foo,monospace">x</span>`; measured width depends on whether `Foo` exists. Loop over candidate fonts; build set.
3. **Chrome 103+ Font Access API** — `window.queryLocalFonts()`. Permission-gated but returns full list when granted.

CreepJS output you saw:
```
load (8/51): ...
apps: unsupported
Jomolhari, Liberation Mono, Noto Color Emoji, Noto Sans Canadian Aboriginal Regular,
OpenSymbol, STIX Two Math Regular, STIX Two Text Regular, Source Code Pro
```
Pure Fedora Linux font stack. Profile claims macOS — instant kill.

## Target files

| File | Change |
|---|---|
| `third_party/blink/renderer/core/css/css_font_face_source.cc` | Hook `IsLocalFontAvailable()` to consult cosmium font list |
| `third_party/blink/renderer/platform/fonts/font_cache.cc` | `PlatformFontExists` filter |
| `third_party/blink/renderer/platform/fonts/skia/font_cache_skia.cc` (Linux) | `GetFontPlatformDataFromUniqueNameLookup` filter |
| `third_party/blink/renderer/modules/font_access/font_enumeration_table.cc` | `queryLocalFonts()` returns only profile fonts |
| `third_party/blink/renderer/core/css/local_font_face_source.cc` | `IsLocalFontFamilyValidForCSS` filter |

## Profile fields read

```
fonts.installed[] → list of canonical family names exactly as JS sees them
```

Validator (already shipped) checks the list contains platform staples and contains no cross-platform leaks. Patch enforces what validator promises.

## Implementation notes

- **Allow-list, not deny-list.** Only fonts in `profile.fonts.installed` resolve to "exists". Everything else returns "not found", including system fallbacks (`-apple-system`, `system-ui`).
- **Generic families.** `serif`, `sans-serif`, `monospace`, `cursive`, `fantasy`, `system-ui`, `-apple-system`, `BlinkMacSystemFont` MUST still resolve — these are CSS keywords, not font names. Map them deterministically to a profile font (e.g. `monospace → Menlo` for macOS, `→ Consolas` for Windows, `→ DejaVu Sans Mono` for Linux). Failure to map breaks page rendering.
- **Rendering when font genuinely missing on host.** The host filesystem may not have "Helvetica" available even though the profile claims it. Resolve via fallback: pick the first profile font present on disk and silently substitute. Visual differs from real Mac, but the fingerprint signal — *which fonts exist* — matches.
- **Bundle a small font set.** Ship `Liberation Sans` + `Liberation Serif` + `Liberation Mono` in the cosmium build and use them as universal substitutes. Reliable cross-distro fallback.
- **`document.fonts.ready` and async font loading.** Must complete normally with the spoofed set — sites that block on `ready` should not hang.
- **Case sensitivity.** Real Chrome is case-insensitive for `check('16px Helvetica')` vs `check('16px helvetica')`. Match by lowercasing both sides.
- **Bounding-box measurement.** Cannot be fully spoofed without also patching `TextMetrics` to use the substituted font's *original* metrics. Without that, the width when rendering "Helvetica" will be DejaVu's width — measurable. Mitigation: ship metric tables for the top ~100 fonts that profiles commonly claim, and have the substituted renderer scale to those metrics. Heavy lift; phase 2.

## Worker contexts

`OffscreenCanvas` + `FontFaceSet` work the same way in workers. Patch must apply at the platform layer (`FontCache`), not at the Document layer, or workers leak the real font list.

## Validation

```js
const probes = ['Helvetica', 'Segoe UI', 'San Francisco', 'DejaVu Sans', 'Times New Roman', 'Liberation Mono'];
for (const f of probes) {
  console.log(f, document.fonts.check(`16px "${f}"`));
}
// for a macos profile: Helvetica/SF true; Segoe/DejaVu false
```

Test sites: https://abrahamjuliot.github.io/creepjs/ (Fonts section), https://browserleaks.com/fonts.
