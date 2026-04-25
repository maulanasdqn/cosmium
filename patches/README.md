# Patches

Each `.patch` is a `git apply`-compatible diff against the Chromium tag pinned in `../VERSION`. Patches are applied in the order listed in [`series`](./series).

## Authoring workflow

```bash
# 1. Start clean
./scripts/reset.sh

# 2. Apply existing patches up to the new one
./scripts/03-apply-patches.sh

# 3. Edit Chromium source directly under src/
$EDITOR src/third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc

# 4. Build to verify it compiles
./scripts/04-build.sh

# 5. Run the fingerprint harness to verify it actually fixes the detection
./scripts/test-fingerprint.sh

# 6. Capture as a patch
./scripts/make-patch.sh 0009-spoof-webgl-vendor-renderer

# 7. Edit the header at the top of the .patch file to document:
#    - what detection vector this fixes
#    - which profile fields are read
#    - link to a test page or detection signature

# 8. Reset and confirm the patch reapplies cleanly from scratch
./scripts/reset.sh
./scripts/03-apply-patches.sh
./scripts/04-build.sh
./scripts/test-fingerprint.sh
```

## Conventions

- **One patch = one detection vector.** Don't bundle. Easier to rebase, easier to selectively disable, easier to bisect when a detection site changes its probe.
- **Read from the profile, never hardcode.** Every spoofed value originates in the JSON profile. Patches change Chromium to *consult* the profile, not to bake values into the binary. The profile loader patch (0001) provides the API every later patch uses.
- **Keep diffs surgical.** Touch the minimum number of files. The smaller the diff, the smaller the rebase pain on the next Chromium tag.
- **Match real Chrome behavior on misses.** When a profile field is absent, fall back to Chromium's default — not to obviously-spoofed sentinel values.

## Patch file header format

The first lines of every patch should document intent. `make-patch.sh` scaffolds this:

```
# cosmium patch: 0009-spoof-webgl-vendor-renderer
# Authored: 2026-04-26
# Chromium base: 135.0.7049.84
#
# Describe the detection vector this patch fixes:
# - target file(s): third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.cc
# - what changed: GetUnmaskedRendererString / GetUnmaskedVendorString consult cosmium profile
# - profile fields read: gpu.vendor, gpu.renderer
# - test: https://browserleaks.com/webgl — UNMASKED_RENDERER_WEBGL must equal profile.gpu.renderer
```

## Design docs (unwritten patches)

The [`design/`](./design) directory holds markdown docs for patches that are planned but not yet authored. Each doc names the target file(s) in the Chromium tree and the detection vector. See [`design/README.md`](./design/README.md) for the full backlog.
