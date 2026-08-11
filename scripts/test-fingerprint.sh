#!/usr/bin/env bash
# Run cosmium against a battery of fingerprint probes and produce a pass/fail
# report. Each probe is a self-contained JS expression that should evaluate
# to a known-clean value when the corresponding patch is in place.
#
# Usage:
#   ./scripts/test-fingerprint.sh [path/to/cosmium/binary] [path/to/profile.json]
#
# Defaults:
#   binary  → out/cosmium/chrome
#   profile → profiles/win11_rtx3060_en-us.json

set -euo pipefail
source "$(dirname "$0")/_lib.sh"

BIN="${1:-${BUILD_OUT}/chrome}"
PROFILE="${2:-${COSMIUM_ROOT}/profiles/win11_rtx3060_en-us.json}"

if [[ ! -x "${BIN}" ]]; then
  log_error "Binary not executable: ${BIN}"
  exit 1
fi
if [[ ! -f "${PROFILE}" ]]; then
  log_error "Profile not found: ${PROFILE}"
  exit 1
fi

log_info "Binary:  ${BIN}"
log_info "Profile: ${PROFILE}"

# Probe definitions. Each probe is:
#   id|description|js-expression|expected-pattern (regex)
#
# A probe passes when the evaluated expression matches expected-pattern.
# Expected values reference profile fields by `${profile.path}` — substituted
# from the profile JSON before evaluation.

probes=(
  "webdriver|navigator.webdriver === false|String(navigator.webdriver)|^false$"
  "webdriver_in|'webdriver' in navigator (still true, just value is false)|String('webdriver' in navigator)|^true$"
  "platform|navigator.platform matches profile|navigator.platform|^\${identity.navigator_platform}$"
  "languages|navigator.languages matches profile|JSON.stringify(navigator.languages)|^\${identity_languages_json}$"
  "hwconcurrency|hardwareConcurrency matches profile|String(navigator.hardwareConcurrency)|^\${hardware.hardware_concurrency}$"
  "devicememory|deviceMemory matches profile|String(navigator.deviceMemory)|^\${hardware.device_memory_gb}$"
  "ua_platform|userAgentData.platform matches profile|(await navigator.userAgentData.getHighEntropyValues(['platform'])).platform|^\${identity.client_hints.platform}$"
  "ua_arch|userAgentData.architecture matches profile|(await navigator.userAgentData.getHighEntropyValues(['architecture'])).architecture|^\${identity.client_hints.architecture}$"
  "webgl_vendor|UNMASKED_VENDOR_WEBGL matches profile|(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_VENDOR_WEBGL);})()|^\${gpu.vendor}$"
  "webgl_renderer|UNMASKED_RENDERER_WEBGL matches profile|(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_RENDERER_WEBGL);})()|^\${gpu.renderer}$"
  "webgl_no_swiftshader|WebGL renderer must NOT contain SwiftShader|(()=>{const c=document.createElement('canvas').getContext('webgl');const e=c.getExtension('WEBGL_debug_renderer_info');return c.getParameter(e.UNMASKED_RENDERER_WEBGL);})()|!SwiftShader"
  "media_devices|enumerateDevices returns >0 entries|(await navigator.mediaDevices.enumerateDevices()).length > 0 ? 'true' : 'false'|^true$"
  "voices|speechSynthesis returns >0 voices|String(speechSynthesis.getVoices().length > 0)|^true$"
  "timezone|Intl timezone matches profile|Intl.DateTimeFormat().resolvedOptions().timeZone|^\${locale.timezone}$"
  "screen_dpr|devicePixelRatio matches profile|String(window.devicePixelRatio)|^\${screen.device_pixel_ratio}$"
  "screen_w|screen.width matches profile|String(screen.width)|^\${screen.width}$"
  "screen_h|screen.height matches profile|String(screen.height)|^\${screen.height}$"
  "color_depth|screen.colorDepth matches profile|String(screen.colorDepth)|^\${screen.color_depth}$"
  "audio_sr|AudioContext sampleRate matches profile|String(new AudioContext().sampleRate)|^\${audio.sample_rate}$"
)

# Escape ERE metacharacters so an interpolated profile value compares as a
# literal. Profile values are full of them -- ["en-US","en"] reads as a
# character class, "Google Inc. (NVIDIA)" as a group -- so an exact match
# reported FAIL with got= and expected= printing identical text.
regex_escape() {
  printf '%s' "$1" | sed 's/[][\\.^$*+?(){}|]/\\&/g'
}

# Resolve profile field interpolations. `mode` is "pattern" when the result is
# used as a regex, in which case substituted values are escaped; the probe's JS
# expression side must stay verbatim.
resolve_mode() {
  local mode="$1"
  local expr="$2"
  # Special-case array → JSON.
  local langs_json
  langs_json=$(jq -c '.locale.languages' "${PROFILE}")
  [[ "${mode}" == pattern ]] && langs_json=$(regex_escape "${langs_json}")
  expr="${expr//\$\{identity_languages_json\}/${langs_json}}"

  # Generic ${a.b.c} → jq path.
  while [[ "${expr}" =~ \$\{([a-z_]+(\.[a-z_]+)*)\} ]]; do
    local path="${BASH_REMATCH[1]}"
    local val
    val=$(jq -r ".${path}" "${PROFILE}")
    [[ "${mode}" == pattern ]] && val=$(regex_escape "${val}")
    expr="${expr//\$\{${path}\}/${val}}"
  done
  echo "${expr}"
}

resolve() { resolve_mode literal "$1"; }
resolve_pattern() { resolve_mode pattern "$1"; }

# Build a single HTML page that evaluates every probe and prints results.
tmp=$(mktemp -d)
trap 'rm -rf "${tmp}"' EXIT

cat > "${tmp}/probes.html" <<'EOF'
<!doctype html>
<html><head><meta charset="utf-8"><title>cosmium probes</title></head>
<body><pre id="out">running…</pre>
<script>
// Probes run sequentially, so one that never settles strands every probe
// after it and the page reports nothing at all. navigator.mediaDevices
// .enumerateDevices() does exactly that under --headless. Race each probe
// against a deadline so a hang is reported as one failed probe instead of
// taking the whole run down.
function withDeadline(p, ms) {
  return Promise.race([
    p,
    new Promise((_, reject) =>
      setTimeout(() => reject(new Error('probe timed out after ' + ms + 'ms')), ms)),
  ]);
}
async function run(probes) {
  const out = [];
  for (const [id, desc, expr] of probes) {
    try {
      const fn = new Function('return (async () => (' + expr + '))()');
      const v = await withDeadline(fn(), 3000);
      out.push(JSON.stringify({id, desc, value: String(v), error: null}));
    } catch (e) {
      out.push(JSON.stringify({id, desc, value: null, error: String(e)}));
    }
  }
  document.getElementById('out').textContent = out.join('\n');
}
run(__PROBES__);
</script>
</body></html>
EOF

# Build the JS array of probes.
js_probes="["
sep=""
for p in "${probes[@]}"; do
  IFS='|' read -r id desc expr expected <<< "${p}"
  expr_resolved=$(resolve "${expr}")
  js_probes+="${sep}[$(jq -Rn --arg s "${id}" '$s'),$(jq -Rn --arg s "${desc}" '$s'),$(jq -Rn --arg s "${expr_resolved}" '$s')]"
  sep=","
done
js_probes+="]"

sed -i.bak "s|__PROBES__|${js_probes}|" "${tmp}/probes.html"

# Run cosmium headlessly and capture the rendered <pre> contents.
out_dump="${tmp}/dump.html"
# --dump-dom serialises the DOM as soon as load finishes, but every probe runs
# inside an async function, so without --virtual-time-budget the dump captures
# the placeholder "running…" and no probe ever reports. The budget lets virtual
# time run ahead until the pending work drains, then dumps.
# The patches expose one switch per spoofed value; there is no
# --cosmium-profile switch, and passing one made Chromium ignore it silently
# while every profile-derived probe reported the machine's real values. This
# mirrors what the Rust CLI's `cosmium run` does: expand the profile into the
# switches the binary actually reads.
mapfile -t cosmium_flags < <(jq -r '
  [
    "--cosmium-platform=\(.identity.navigator_platform)",
    "--cosmium-ua-platform=\(.identity.client_hints.platform)",
    "--cosmium-languages=\(.locale.languages | join(","))",
    "--cosmium-timezone=\(.locale.timezone)",
    "--cosmium-hardware-concurrency=\(.hardware.hardware_concurrency)",
    "--cosmium-device-memory=\(.hardware.device_memory_gb)",
    "--cosmium-max-touch-points=\(.hardware.max_touch_points)",
    "--cosmium-color-depth=\(.screen.color_depth)",
    "--cosmium-webgl-vendor=\(.gpu.vendor)",
    "--cosmium-webgl-renderer=\(.gpu.renderer)"
  ] | .[]' "${PROFILE}")

"${BIN}" \
  "${cosmium_flags[@]}" \
  --headless=new \
  --disable-gpu-sandbox \
  --no-sandbox \
  --virtual-time-budget="${VIRTUAL_TIME_BUDGET_MS:-10000}" \
  --dump-dom \
  "file://${tmp}/probes.html" > "${out_dump}" 2>/dev/null

# Extract the JSON-per-line probe results.
results=$(grep -oP '\{"id":[^}]+\}' "${out_dump}" || true)
if [[ -z "${results}" ]]; then
  log_error "No probe results captured. Dump:"
  cat "${out_dump}" >&2
  exit 1
fi

# Compare each result against expected pattern.
pass=0
fail=0
echo
printf '%-22s %-8s %s\n' "PROBE" "RESULT" "VALUE"
printf '%-22s %-8s %s\n' "----------------------" "--------" "-----"

for p in "${probes[@]}"; do
  IFS='|' read -r id desc expr expected <<< "${p}"
  expected_resolved=$(resolve_pattern "${expected}")
  line=$(echo "${results}" | grep "\"id\":\"${id}\"" || true)
  if [[ -z "${line}" ]]; then
    printf '%-22s %s%-8s%s %s\n' "${id}" "${C_ERR}" "MISSING" "${C_RESET}" ""
    fail=$((fail + 1))
    continue
  fi
  value=$(echo "${line}" | jq -r '.value // ""')
  err=$(echo "${line}" | jq -r '.error // ""')
  # An expected pattern starting with '!' means "must NOT match the rest".
  # bash's [[ =~ ]] is POSIX ERE and has no negative lookahead, so a pattern
  # like ^(?!.*SwiftShader).*$ is not merely unsupported — it makes bash abort
  # the comparison with "invalid regular expression", so the probe could never
  # report anything but FAIL.
  negate=""
  if [[ "${expected_resolved}" == '!'* ]]; then
    negate="yes"
    expected_resolved="${expected_resolved#!}"
  fi

  if [[ -n "${err}" ]]; then
    printf '%-22s %s%-8s%s %s\n' "${id}" "${C_ERR}" "ERROR" "${C_RESET}" "${err}"
    fail=$((fail + 1))
  elif [[ -n "${negate}" ]]; then
    if [[ "${value}" =~ ${expected_resolved} ]]; then
      printf '%-22s %s%-8s%s got=%s must-not-match=%s\n' "${id}" "${C_ERR}" "FAIL" "${C_RESET}" "${value}" "${expected_resolved}"
      fail=$((fail + 1))
    else
      printf '%-22s %s%-8s%s %s\n' "${id}" "${C_OK}" "PASS" "${C_RESET}" "${value}"
      pass=$((pass + 1))
    fi
  elif [[ "${value}" =~ ${expected_resolved} ]]; then
    printf '%-22s %s%-8s%s %s\n' "${id}" "${C_OK}" "PASS" "${C_RESET}" "${value}"
    pass=$((pass + 1))
  else
    printf '%-22s %s%-8s%s got=%s expected=%s\n' "${id}" "${C_ERR}" "FAIL" "${C_RESET}" "${value}" "${expected_resolved}"
    fail=$((fail + 1))
  fi
done

echo
log_info "passed=${pass} failed=${fail}"
if [[ ${fail} -gt 0 ]]; then
  exit 1
fi
