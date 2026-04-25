#!/usr/bin/env bash
# Apply every patch listed in patches/series, in order.
# Stops on first failure and leaves a .rej for inspection.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_src

SERIES="${COSMIUM_ROOT}/patches/series"
APPLIED_MARK="${COSMIUM_ROOT}/.patches-applied"

if [[ ! -f "${SERIES}" ]]; then
  log_error "patches/series not found"
  exit 1
fi

if [[ -f "${APPLIED_MARK}" ]]; then
  log_warn "Patches already marked applied. Run scripts/reset.sh first if you want to reapply."
  exit 1
fi

cd "${CHROMIUM_SRC}"

count=0
while IFS= read -r line; do
  # Skip blank lines and comments.
  [[ -z "${line}" || "${line}" =~ ^[[:space:]]*# ]] && continue

  patch_path="${COSMIUM_ROOT}/patches/${line}"
  if [[ ! -f "${patch_path}" ]]; then
    log_error "Listed in series but missing: ${line}"
    exit 1
  fi

  log_info "Applying ${line}"
  if ! git apply --3way --whitespace=nowarn "${patch_path}"; then
    log_error "Patch failed: ${line}"
    log_error "Inspect rejects in ${CHROMIUM_SRC}, then either fix the patch or run scripts/reset.sh"
    exit 1
  fi
  count=$((count + 1))
done < "${SERIES}"

touch "${APPLIED_MARK}"
log_ok "Applied ${count} patch(es)"
log_info "Next: ./scripts/04-build.sh"
