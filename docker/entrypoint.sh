#!/usr/bin/env bash
# cosmium runtime entrypoint.
#
# Reads cosmium-specific env vars and translates them into chrome flags,
# then exec's chrome with the merged flag set. Anything passed as args to
# `docker run` is appended after our defaults.

set -euo pipefail

# ---- Profile resolution ----
# Either COSMIUM_PROFILE (path) or COSMIUM_PROFILE_NAME (lookup in /profiles).
if [[ -n "${COSMIUM_PROFILE:-}" ]]; then
  profile_path="${COSMIUM_PROFILE}"
elif [[ -n "${COSMIUM_PROFILE_NAME:-}" ]]; then
  profile_path="/profiles/${COSMIUM_PROFILE_NAME}.json"
else
  profile_path=""
fi

if [[ -n "${profile_path}" && ! -f "${profile_path}" ]]; then
  echo "[cosmium] profile not found: ${profile_path}" >&2
  exit 1
fi

# ---- Timezone from profile (or env) ----
# JS `Intl` honors the TZ environment variable — match it to profile.locale.timezone
# so the C++ patch and the runtime env agree.
if [[ -n "${profile_path}" && -z "${TZ:-}" ]]; then
  tz=$(jq -r '.locale.timezone // empty' "${profile_path}" 2>/dev/null || true)
  if [[ -n "${tz}" ]]; then
    export TZ="${tz}"
  fi
fi

# ---- Locale from profile ----
if [[ -n "${profile_path}" ]]; then
  lang=$(jq -r '.locale.languages[0] // empty' "${profile_path}" 2>/dev/null | tr '-' '_' || true)
  if [[ -n "${lang}" ]]; then
    export LANG="${lang}.UTF-8"
    export LC_ALL="${lang}.UTF-8"
  fi
fi

# ---- PulseAudio (CPU variant only — GPU image skips this) ----
if [[ -e /etc/pulse/default.pa ]] && command -v pulseaudio >/dev/null; then
  pulseaudio --start --exit-idle-time=-1 2>/dev/null || true
fi

# ---- Default flags ----
# Keep the sandbox enabled — --no-sandbox is itself a tell on some sites
# that probe sandbox status via Permissions / process-hierarchy heuristics.
default_flags=(
  --headless=new
  --disable-blink-features=AutomationControlled
  --disable-features=Translate,BackForwardCache,InterestFeedContentSuggestions
  --disable-background-timer-throttling
  --disable-backgrounding-occluded-windows
  --disable-renderer-backgrounding
  --no-default-browser-check
  --no-first-run
  --no-sandbox
  --disable-crash-reporter
  --disable-dev-shm-usage
)

if [[ -n "${profile_path}" ]]; then
  default_flags+=(--cosmium-profile="${profile_path}")
fi

# Optional CDP port for any automation client (chromiumoxide, puppeteer, playwright).
if [[ -n "${COSMIUM_CDP_PORT:-}" ]]; then
  default_flags+=(--remote-debugging-port="${COSMIUM_CDP_PORT}")
fi

# Optional proxy passthrough.
if [[ -n "${COSMIUM_PROXY:-}" ]]; then
  default_flags+=(--proxy-server="${COSMIUM_PROXY}")
fi

# Show what we resolved (stderr only — keeps stdout clean for CDP pipe usage).
echo "[cosmium] profile=${profile_path:-<none>} tz=${TZ:-<host>} lang=${LANG:-<host>}" >&2

exec /opt/cosmium/chrome "${default_flags[@]}" "$@"
