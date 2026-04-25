#!/usr/bin/env bash
# Revert all applied patches, returning the source tree to the pinned tag.
# Use this before rebasing onto a new Chromium tag, or to start fresh.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_src

cd "${CHROMIUM_SRC}"

log_info "Reverting working tree to clean ${CHROMIUM_TAG}"
git reset --hard "tags/${CHROMIUM_TAG}"
git clean -fd

rm -f "${COSMIUM_ROOT}/.patches-applied"

log_ok "Source tree clean. Run scripts/03-apply-patches.sh to reapply."
