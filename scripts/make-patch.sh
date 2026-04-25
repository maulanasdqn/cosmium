#!/usr/bin/env bash
# Helper for authoring a new patch.
#
# Workflow:
#   1. Edit files directly in src/ (the working tree).
#   2. When happy, run: ./scripts/make-patch.sh NNNN-short-name
#      where NNNN is the next number in patches/series.
#   3. The script diffs your edits against HEAD, writes patches/NNNN-short-name.patch,
#      and appends it to patches/series.
#   4. Verify: ./scripts/reset.sh && ./scripts/03-apply-patches.sh && ./scripts/04-build.sh

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

# Use git's default diff (3-way friendly). Includes new files via --no-prefix
# is NOT used because git apply expects a/ and b/ prefixes.
git diff HEAD --binary > "${out}"

if [[ ! -s "${out}" ]]; then
  rm "${out}"
  log_error "No working-tree changes to capture"
  exit 1
fi

# Add a header so future-us knows what this patch was for.
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
