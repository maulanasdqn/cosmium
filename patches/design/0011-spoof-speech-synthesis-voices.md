# patch 0011-spoof-speech-synthesis-voices

`window.speechSynthesis.getVoices()` returns the OS voice list. A Mac UA reporting Windows "Microsoft David", or a Windows UA reporting "Samantha", or any UA returning an empty array (default in headless), is a one-shot detection vector.

## Detection vector

```js
const voices = speechSynthesis.getVoices();
// CreepJS hashes name + lang + voiceURI of every voice.
// Empty array     → headless / container.
// Linux voices    → Linux underneath any UA spoof.
// macOS voices on Windows UA → mismatch.
```

CreepJS bucket: `Speech / local / remote / lang / default`. Output you saw:
```
Speech
local (0): blocked
remote (0): blocked
lang (0): blocked
default: blocked
```
That `(0)` means the array was empty — your patched Chromium has no voice provider configured on Fedora, so `getVoices()` returned nothing. Real macOS would return ~30+ voices.

## Target files

| File | Change |
|---|---|
| `third_party/blink/renderer/modules/speech/speech_synthesis.cc` | `SpeechSynthesis::getVoices()` reads cosmium voice list from a serialized switch instead of dispatching to the platform `TtsController` |
| `third_party/blink/renderer/modules/speech/speech_synthesis_voice.cc` | constructor accepting profile voice entries directly |
| `content/browser/speech/tts_controller_impl.cc` | bypass when cosmium voices present (avoid OS lookup entirely) |

## Profile fields read

```
voices[] → { name, lang, default, localService, voiceURI }
```

## Implementation notes

- **Serialization.** Profile voices are an array of objects. Encode as JSON-in-switch (`--cosmium-voices=[{...},...]`) and parse via `base::JSONReader` once at construction. Stash in a `static base::NoDestructor<Vector<...>>` keyed off the renderer process.
- **`voiceschanged` event.** Real Chrome fires `voiceschanged` once voices load asynchronously from the OS. Cosmium should still fire it (1-tick delay after construction) so feature-detect code that waits for it doesn't hang.
- **`speak()` actually speaking.** Spoofing the *list* is independent of actually producing audio. If `speak()` is called, the patch should resolve the utterance silently after `(text.length / charsPerSecond)` ms — sites that listen for `onend` shouldn't hang.
- **`voiceURI` uniqueness.** Real systems use stable URIs (`com.apple.voice.compact.en-US.Samantha`). Don't generate random ones; use exactly what's in the profile.
- **`default` flag.** Only one voice should be `default: true`. Validator already checks one of them matches primary language — extend to also assert exactly one default.

## Validation

```js
speechSynthesis.addEventListener('voiceschanged', () => {
  const v = speechSynthesis.getVoices();
  console.log(v.length, v[0]?.name, v[0]?.voiceURI);
});
// trigger:
speechSynthesis.getVoices();
```

Expected for macos_m2_en-us profile: 6 voices, first = "Samantha", URI = `com.apple.voice.compact.en-US.Samantha`.

Test sites: https://abrahamjuliot.github.io/creepjs/ (Speech section), https://browserleaks.com/javascript.
