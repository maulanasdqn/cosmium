#!/usr/bin/env bash

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_linux
require_depot_tools
require_src

mkdir -p "$(dirname "${BUILD_OUT}")"
cd "${CHROMIUM_SRC}"

log_info "Generating build files at ${BUILD_OUT}"
gn_args=$(grep -v '^[[:space:]]*#' "${COSMIUM_ROOT}/.config/args.gn" \
  | grep -v '^[[:space:]]*$' \
  | tr '\n' ' ')

gn gen "${BUILD_OUT}" --args="${gn_args}"

log_info "Building target=${BUILD_TARGET} (this is the long step)"
if [[ -n "${NINJA_JOBS:-}" ]]; then
  autoninja -C "${BUILD_OUT}" -j "${NINJA_JOBS}" "${BUILD_TARGET}"
else
  autoninja -C "${BUILD_OUT}" "${BUILD_TARGET}"
fi

log_ok "Build complete: ${BUILD_OUT}/chrome"
log_info "Next: ./scripts/05-package.sh"
