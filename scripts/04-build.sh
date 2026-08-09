#!/usr/bin/env bash
# Build cosmium. First build is slow (~4-8h on a fast machine).
# Incremental rebuilds after a patch edit are typically minutes.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_linux
require_depot_tools
require_src

# ---- Generate build files ----
mkdir -p "$(dirname "${BUILD_OUT}")"
cd "${CHROMIUM_SRC}"

log_info "Generating build files at ${BUILD_OUT}"
# Pass args.gn via --args="...inline..." so the file in our config dir wins
# even if someone has stale args in out/cosmium/args.gn.
gn_args=$(grep -v '^[[:space:]]*#' "${COSMIUM_ROOT}/.config/args.gn" \
  | grep -v '^[[:space:]]*$' \
  | tr '\n' ' ')

gn gen "${BUILD_OUT}" --args="${gn_args}"

# ---- Build ----
log_info "Building target=${BUILD_TARGET} (this is the long step)"
if [[ -n "${NINJA_JOBS:-}" ]]; then
  autoninja -C "${BUILD_OUT}" -j "${NINJA_JOBS}" "${BUILD_TARGET}"
else
  autoninja -C "${BUILD_OUT}" "${BUILD_TARGET}"
fi

log_ok "Build complete: ${BUILD_OUT}/chrome"
log_info "Next: ./scripts/05-package.sh"
