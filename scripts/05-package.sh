#!/usr/bin/env bash
# Bundle the built binary + runtime files into a distributable archive.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_src

# Two entry points build into two different directories. 04-build.sh honours
# BUILD_OUT from .config/chromium.env (out/cosmium, which is also what the Rust
# CLI's COSMIUM_BUILD_OUT and test-fingerprint.sh expect), while the standalone
# build-linux.sh / build-mac.sh hardcode src/out/Default. Packaging only the
# configured path made this script fail outright after a build-linux.sh run.
# Take the configured path when it holds a binary, otherwise fall back.
resolve_build_out() {
  local candidates=("${BUILD_OUT}" "${CHROMIUM_SRC}/out/Default")
  local c
  for c in "${candidates[@]}"; do
    if [[ -x "${c}/chrome" ]]; then
      printf '%s' "${c}"
      return 0
    fi
  done
  log_error "No built chrome binary found. Looked in:"
  for c in "${candidates[@]}"; do
    log_error "  ${c}/chrome"
  done
  log_error "Run scripts/04-build.sh (or scripts/build-linux.sh) first."
  return 1
}

BUILD_OUT="$(resolve_build_out)" || exit 1
log_info "Packaging from ${BUILD_OUT}"

mkdir -p "${DIST_DIR}"
stage="${DIST_DIR}/cosmium-${CHROMIUM_TAG}"
rm -rf "${stage}"
mkdir -p "${stage}"

log_info "Staging runtime files in ${stage}"

# Core binary + the pak/locale files Chromium loads at runtime.
# This list mirrors what real Chrome ships — anything missing means runtime
# crashes, anything extra is dead weight.
files=(
  chrome
  chrome_100_percent.pak
  chrome_200_percent.pak
  chrome_crashpad_handler
  chrome_sandbox        # SUID-able sandbox helper
  icudtl.dat
  resources.pak
  v8_context_snapshot.bin
  libEGL.so
  libGLESv2.so
  libvk_swiftshader.so
  libvulkan.so.1
  vk_swiftshader_icd.json
  ANGLE              # directory
  locales            # directory
)

for f in "${files[@]}"; do
  src="${BUILD_OUT}/${f}"
  if [[ -e "${src}" ]]; then
    cp -R "${src}" "${stage}/"
  else
    log_warn "Missing (skipped): ${f}"
  fi
done

# Permissions for the SUID sandbox helper. The container entrypoint will
# enable it via setuid; here we just preserve the executable bit.
chmod 4755 "${stage}/chrome_sandbox" 2>/dev/null || true

# Record what is actually compiled into this binary, not what patches/series
# lists. The two drift apart whenever series gains entries after the tree was
# patched -- a pull that adds upstream patches, or a build run with --only
# build, which skips the patch step entirely. A manifest that claims patches
# the binary does not contain is worse than no manifest, because downstream
# automation trusts it to decide which evasions are live.
# `git apply --check --reverse` succeeding means the patch is already present.
applied=()
unapplied=()
while read -r patch_name; do
  [[ -z "${patch_name}" || "${patch_name}" == \#* ]] && continue
  if (cd "${CHROMIUM_SRC}" && git apply --check --reverse \
        "${COSMIUM_ROOT}/patches/${patch_name}" >/dev/null 2>&1); then
    applied+=("${patch_name}")
  else
    unapplied+=("${patch_name}")
  fi
done < "${COSMIUM_ROOT}/patches/series"

applied_json=$(printf '%s\n' "${applied[@]+"${applied[@]}"}" \
  | jq -R . | jq -sc 'map(select(. != ""))')
unapplied_json=$(printf '%s\n' "${unapplied[@]+"${unapplied[@]}"}" \
  | jq -R . | jq -sc 'map(select(. != ""))')

if [[ ${#unapplied[@]} -gt 0 ]]; then
  log_warn "${#unapplied[@]} patch(es) in series are NOT in this binary:"
  for u in "${unapplied[@]}"; do log_warn "  ${u}"; done
fi

# Stamp the build with version info readable by automation clients.
cat > "${stage}/cosmium.json" <<EOF
{
  "chromium_tag": "${CHROMIUM_TAG}",
  "build_host": "$(uname -srm)",
  "built_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "patches": ${applied_json},
  "patches_not_applied": ${unapplied_json}
}
EOF

archive="${DIST_DIR}/cosmium-${CHROMIUM_TAG}.tar.zst"
log_info "Compressing to ${archive}"
tar --use-compress-program=zstd -cf "${archive}" -C "${DIST_DIR}" "cosmium-${CHROMIUM_TAG}"

log_ok "Packaged: ${archive}"
log_ok "Size: $(du -h "${archive}" | cut -f1)"
