#!/usr/bin/env bash

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_linux
require_depot_tools

mkdir -p "${COSMIUM_ROOT}"

if [[ ! -d "${CHROMIUM_SRC}" ]]; then
  log_info "Initial fetch — this will take a while (~30min, ~40GB)"
  cd "${COSMIUM_ROOT}"
  fetch --nohooks --no-history chromium
else
  log_info "Source tree exists at ${CHROMIUM_SRC} — running gclient sync"
  cd "${CHROMIUM_SRC}"
  gclient sync --nohooks --with_branch_heads --with_tags
fi

if [[ -z "${COSMIUM_SKIP_BUILD_DEPS:-}" ]]; then
  log_info "Installing Chromium build dependencies (sudo required)"
  cd "${CHROMIUM_SRC}"
  ./build/install-build-deps.sh \
    --no-prompt \
    --no-android \
    --no-chromeos-fonts \
    --no-arm \
    --no-nacl
fi

log_ok "Source fetched at ${CHROMIUM_SRC}"
log_info "Next: ./scripts/02-checkout.sh"
