#!/usr/bin/env bash

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_src

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

files=(
  chrome
  chrome_100_percent.pak
  chrome_200_percent.pak
  chrome_crashpad_handler
  chrome_sandbox
  icudtl.dat
  resources.pak
  v8_context_snapshot.bin
  libEGL.so
  libGLESv2.so
  libvk_swiftshader.so
  libvulkan.so.1
  vk_swiftshader_icd.json
  ANGLE
  locales
)

for f in "${files[@]}"; do
  src="${BUILD_OUT}/${f}"
  if [[ -e "${src}" ]]; then
    cp -R "${src}" "${stage}/"
  else
    log_warn "Missing (skipped): ${f}"
  fi
done

chmod 4755 "${stage}/chrome_sandbox" 2>/dev/null || true

log_info "Writing license notices"
cp "${CHROMIUM_SRC}/LICENSE" "${stage}/LICENSE.chromium"
cp "${COSMIUM_ROOT}/LICENSE" "${stage}/LICENSE.cosmium"
(cd "${CHROMIUM_SRC}" && PATH="${DEPOT_TOOLS}:${PATH}" python3 tools/licenses/licenses.py \
  license_file --format txt --target-os linux \
  --gn-out-dir "${BUILD_OUT}" --gn-target //chrome:chrome \
  "${stage}/THIRD_PARTY_LICENSES.txt") \
  || { log_error "Could not generate third-party license notices"; exit 1; }
cp "${COSMIUM_ROOT}/NOTICE" "${stage}/NOTICE"

strip_tool="${CHROMIUM_SRC}/third_party/llvm-build/Release+Asserts/bin/llvm-strip"
[[ -x "${strip_tool}" ]] || strip_tool="$(command -v llvm-strip || command -v strip || true)"
if [[ -n "${strip_tool}" ]]; then
  log_info "Stripping debug symbols with ${strip_tool}"
  for bin in chrome chrome_crashpad_handler libEGL.so libGLESv2.so libvk_swiftshader.so libvulkan.so.1; do
    [[ -f "${stage}/${bin}" ]] && "${strip_tool}" --strip-debug "${stage}/${bin}"
  done
else
  log_warn "No strip tool found, shipping unstripped binaries"
fi

applied=()
unapplied=()
while read -r patch_name; do
  [[ -z "${patch_name}" || "${patch_name}" == \#* ]] && continue
  patch_path="${COSMIUM_ROOT}/patches/${patch_name}"
  if (cd "${CHROMIUM_SRC}" && git apply --check --reverse "${patch_path}" \
        >/dev/null 2>&1); then
    applied+=("${patch_name}")
  elif (cd "${CHROMIUM_SRC}" && git apply --check "${patch_path}" \
        >/dev/null 2>&1); then
    unapplied+=("${patch_name}")
  else
    applied+=("${patch_name}")
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

cat > "${stage}/cosmium.json" <<EOF
{
  "chromium_tag": "${CHROMIUM_TAG}",
  "build_host": "$(uname -srm)",
  "built_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "patches": ${applied_json},
  "patches_not_applied": ${unapplied_json}
}
EOF

archive="${DIST_DIR}/cosmium-${CHROMIUM_TAG}-linux-x86_64.tar.gz"
log_info "Compressing to ${archive}"
gz="$(command -v pigz || command -v gzip)"
tar --use-compress-program="${gz} -9" -cf "${archive}" -C "${DIST_DIR}" "cosmium-${CHROMIUM_TAG}"

latest="${DIST_DIR}/cosmium-browser-linux-x86_64.tar.gz"
cp "${archive}" "${latest}"
for f in "${archive}" "${latest}"; do
  (cd "${DIST_DIR}" && sha256sum "$(basename "${f}")" > "$(basename "${f}").sha256")
done

log_ok "Packaged: ${archive}"
log_ok "Latest alias: ${latest}"
log_ok "Size: $(du -h "${archive}" | cut -f1)"
