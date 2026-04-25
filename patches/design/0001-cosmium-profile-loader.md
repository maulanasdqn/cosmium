# patch 0001-cosmium-profile-loader

**Prerequisite for every other patch.** Adds the `--cosmium-profile=<path>` switch, parses the JSON, and exposes a singleton accessor every later patch reads from.

## Detection vector

None directly. Without this, every other patch would have to hardcode spoofed values, which means recompiling for every fingerprint change.

## Target files

| File | Change |
|---|---|
| `chrome/common/chrome_switches.{h,cc}` | Add `kCosmiumProfile` switch constant |
| `content/public/common/content_switches.{h,cc}` | Same switch in content/, so renderer process can read it |
| `chrome/browser/cosmium/cosmium_profile.{h,cc}` | New: `CosmiumProfile` class — singleton, lazy-loaded from JSON, accessor methods |
| `content/renderer/cosmium/cosmium_profile_renderer.{h,cc}` | New: per-renderer mirror — gets profile blob via Mojo IPC at renderer init |
| `content/browser/renderer_host/render_process_host_impl.cc` | Pass `--cosmium-profile` to renderer command line |
| `chrome/app/chrome_main_delegate.cc` | Load profile early in PreSandboxStartup |
| `BUILD.gn` files in the new directories | Wire in the new sources |

## Profile fields read

All of them. This patch is the reader. The accessor surface needs:

```cpp
namespace cosmium {

class Profile {
 public:
  static Profile* GetInstance();  // singleton
  bool IsLoaded() const;

  // Identity
  std::string UserAgent() const;
  ClientHints GetClientHints() const;
  std::string NavigatorPlatform() const;

  // Locale
  std::vector<std::string> Languages() const;
  std::string AcceptLanguage() const;
  std::string Timezone() const;

  // Hardware
  unsigned HardwareConcurrency() const;
  float DeviceMemoryGB() const;
  int MaxTouchPoints() const;

  // GPU
  std::string GpuVendor() const;
  std::string GpuRenderer() const;
  std::string GpuVendorId() const;
  std::string GpuDeviceId() const;

  // Screen
  ScreenInfo GetScreenInfo() const;

  // Audio
  AudioInfo GetAudioInfo() const;

  // Media devices
  std::vector<MediaDeviceInfo> MediaDevices() const;

  // Speech
  std::vector<VoiceInfo> Voices() const;

  // Fonts
  bool IsFontInstalled(const std::string& family) const;

  // WebRTC
  std::string WebRtcIpHandlingPolicy() const;

  // Canvas/audio noise
  uint64_t NoiseSeed() const;
};

}  // namespace cosmium
```

## Implementation notes

- **Process model.** Browser process loads the profile from disk at startup. Renderer processes receive the parsed profile via Mojo IPC at renderer init (cannot read disk after sandboxing). Worker threads inherit the renderer's profile pointer.
- **Sandboxing.** Profile load must happen *before* sandbox is engaged in the browser process. The data crosses the sandbox boundary as a serialized struct, not as a file path.
- **Hot reload: don't.** Profile is read-once at startup. Sites checksum fingerprint stability across sessions; mutating values mid-session is itself a tell. Spawn a new browser instance for a new profile.
- **Validation.** On load failure (missing file, bad JSON, schema mismatch), log loudly and exit with a distinctive error code. Silent fallback to vanilla Chromium values defeats the entire point — every later patch checks `IsLoaded()` and bails to default Chromium behavior on miss, but a missing profile file should be a fatal misconfig, not a silent regression.
- **JSON parsing.** Use Chromium's `base::JSONReader` and `base::Value::Dict`. Don't pull in a third-party JSON library.
- **Cross-origin frames / OOPIFs.** Each renderer process gets the same profile. No need to vary by origin.

## Validation

After this patch, `chrome://version` should show a `Cosmium Profile` line with the loaded profile name. Run:

```
./out/cosmium/chrome --cosmium-profile=profiles/win11_rtx3060_en-us.json --headless=new --dump-dom chrome://version
```

Output should contain `Cosmium Profile: win11_rtx3060_en-us`.
