#!/usr/bin/env bash
# Check out the pinned Chromium tag and run gclient hooks.
# Use this after 01-fetch.sh, and any time VERSION changes.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_linux
require_depot_tools
require_src

log_info "Checking out Chromium tag ${CHROMIUM_TAG}"
cd "${CHROMIUM_SRC}"

# Reset any in-progress patch state before switching tags.
if [[ -f "${COSMIUM_ROOT}/.patches-applied" ]]; then
  log_warn "Patches currently applied — reverting before checkout"
  "${COSMIUM_ROOT}/scripts/reset.sh"
fi

git fetch --tags origin "refs/tags/${CHROMIUM_TAG}:refs/tags/${CHROMIUM_TAG}" 2>/dev/null || true
git checkout "tags/${CHROMIUM_TAG}" -B "cosmium-${CHROMIUM_TAG}"

log_info "Syncing dependencies for ${CHROMIUM_TAG}"
gclient sync --with_branch_heads --with_tags --reset

log_info "Running gclient hooks"
gclient runhooks

log_ok "Checked out ${CHROMIUM_TAG}, ready to patch"
log_info "Next: ./scripts/03-apply-patches.sh"
