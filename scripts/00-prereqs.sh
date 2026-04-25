#!/usr/bin/env bash
# Install depot_tools and Chromium build prerequisites.
# Idempotent — safe to re-run.

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

require_linux

# ---- depot_tools ----
if [[ ! -d "${DEPOT_TOOLS}" ]]; then
  log_info "Cloning depot_tools to ${DEPOT_TOOLS}"
  git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git \
    "${DEPOT_TOOLS}"
else
  log_info "depot_tools present — pulling latest"
  git -C "${DEPOT_TOOLS}" pull --ff-only
fi

export PATH="${DEPOT_TOOLS}:${PATH}"

# ---- Build deps ----
# install-build-deps.sh ships in the Chromium tree, but it's also fine to
# bootstrap a minimal subset before the full source fetch. The full script
# runs after fetch in 01-fetch.sh.

if ! command -v git >/dev/null; then
  log_error "git not installed"
  exit 1
fi
if ! command -v python3 >/dev/null; then
  log_error "python3 not installed"
  exit 1
fi
if ! command -v curl >/dev/null; then
  log_error "curl not installed"
  exit 1
fi

# Disable depot_tools' auto-update prompts during scripted builds.
export DEPOT_TOOLS_UPDATE=1

log_ok "Prereqs ready. depot_tools at ${DEPOT_TOOLS}"
log_info "Next: ./scripts/01-fetch.sh"
