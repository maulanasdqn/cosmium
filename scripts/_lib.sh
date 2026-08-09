#!/usr/bin/env bash
# Shared helpers sourced by every script in this dir.

set -euo pipefail

# Resolve the cosmium repo root from the script's own location (not from PWD,
# so scripts work no matter where they are invoked from).
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[1]}")" && pwd)"
COSMIUM_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
export COSMIUM_ROOT

# shellcheck source=../.config/chromium.env
source "${COSMIUM_ROOT}/.config/chromium.env"

# Logging helpers — colored if stdout is a tty, plain otherwise.
if [[ -t 1 ]]; then
  C_INFO=$'\033[1;34m'
  C_OK=$'\033[1;32m'
  C_WARN=$'\033[1;33m'
  C_ERR=$'\033[1;31m'
  C_RESET=$'\033[0m'
else
  C_INFO=""; C_OK=""; C_WARN=""; C_ERR=""; C_RESET=""
fi

log_info()  { printf '%s[cosmium]%s %s\n'  "${C_INFO}" "${C_RESET}" "$*"; }
log_ok()    { printf '%s[cosmium]%s %s\n'  "${C_OK}"   "${C_RESET}" "$*"; }
log_warn()  { printf '%s[cosmium]%s %s\n'  "${C_WARN}" "${C_RESET}" "$*" >&2; }
log_error() { printf '%s[cosmium]%s %s\n'  "${C_ERR}"  "${C_RESET}" "$*" >&2; }

require_linux() {
  if [[ "$(uname -s)" != "Linux" ]]; then
    log_error "Chromium builds only on Linux from this repo. Detected: $(uname -s)"
    log_error "Use docker/Dockerfile.build to cross-build from macOS/Windows hosts."
    exit 1
  fi
}

require_depot_tools() {
  if [[ ! -d "${DEPOT_TOOLS}" ]]; then
    log_error "depot_tools not found at ${DEPOT_TOOLS}. Run scripts/00-prereqs.sh first."
    exit 1
  fi
  export PATH="${DEPOT_TOOLS}:${PATH}"
}

require_src() {
  if [[ ! -d "${CHROMIUM_SRC}" ]]; then
    log_error "Chromium source not found at ${CHROMIUM_SRC}. Run scripts/01-fetch.sh first."
    exit 1
  fi
}
