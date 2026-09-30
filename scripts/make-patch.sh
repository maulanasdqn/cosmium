#!/usr/bin/env bash

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_src

if [[ $# -lt 1 ]]; then
  log_error "usage: $0 NNNN-short-name"
  exit 1
fi

name="$1"
out="${COSMIUM_ROOT}/patches/${name}.patch"

if [[ -f "${out}" ]]; then
  log_error "Already exists: ${out}"
  exit 1
fi

cd "${CHROMIUM_SRC}"
log_info "Generating patch from working-tree changes"

git diff HEAD --binary > "${out}"

if [[ ! -s "${out}" ]]; then
  rm "${out}"
  log_error "No working-tree changes to capture"
  exit 1
fi

tmp="$(mktemp)"
{
  echo "# cosmium patch: ${name}"
  echo "# Authored: $(date -u +%Y-%m-%d)"
  echo "# Chromium base: ${CHROMIUM_TAG}"
  echo "#"
  echo "# Describe the detection vector this patch fixes:"
  echo "# - target file(s):"
  echo "# - what changed:"
  echo "# - profile fields read:"
  echo "# - test:"
  echo "#"
  cat "${out}"
} > "${tmp}"
mv "${tmp}" "${out}"

echo "${name}.patch" >> "${COSMIUM_ROOT}/patches/series"

log_ok "Wrote ${out} and appended to patches/series"
log_info "Edit the header to document the patch, then commit it."
