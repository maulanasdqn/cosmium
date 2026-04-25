#!/usr/bin/env bash
# Bundle the built binary + runtime files into a distributable archive.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_src

if [[ ! -x "${BUILD_OUT}/chrome" ]]; then
  log_error "${BUILD_OUT}/chrome not found — run scripts/04-build.sh first"
  exit 1
fi

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

# Stamp the build with version info readable by automation clients.
cat > "${stage}/cosmium.json" <<EOF
{
  "chromium_tag": "${CHROMIUM_TAG}",
  "build_host": "$(uname -srm)",
  "built_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "patches": $(jq -R . < "${COSMIUM_ROOT}/patches/series" | jq -s . 2>/dev/null || echo "[]")
}
EOF

archive="${DIST_DIR}/cosmium-${CHROMIUM_TAG}.tar.zst"
log_info "Compressing to ${archive}"
tar --use-compress-program=zstd -cf "${archive}" -C "${DIST_DIR}" "cosmium-${CHROMIUM_TAG}"

log_ok "Packaged: ${archive}"
log_ok "Size: $(du -h "${archive}" | cut -f1)"
