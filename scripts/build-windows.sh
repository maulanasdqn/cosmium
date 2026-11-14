#!/usr/bin/env bash
set -euo pipefail

COSMIUM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="$(tr -d '[:space:]' < "$COSMIUM_ROOT/VERSION")"
SRC_DIR="${COSMIUM_ROOT}/src"
PATCHES_DIR="${COSMIUM_ROOT}/patches"
ARGS_GN="${COSMIUM_ROOT}/.config/args.win-x64.gn"

# shellcheck source=../.config/chromium.env
source "${COSMIUM_ROOT}/.config/chromium.env"
BUILD_OUT="${SRC_DIR}/out/cosmium"
JOBS="${JOBS:-$(( $(nproc) - 2 ))}"

ONLY=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --only) ONLY="$2"; shift 2 ;;
    --jobs) JOBS="$2"; shift 2 ;;
    *) echo "Unknown arg: $1"; exit 1 ;;
  esac
done

should_run() {
  [[ -z "$ONLY" || "$ONLY" == "$1" ]]
}

info()  { echo -e "\033[1;34m▸ $*\033[0m"; }
ok()    { echo -e "\033[1;32m✓ $*\033[0m"; }
err()   { echo -e "\033[1;31m✗ $*\033[0m"; exit 1; }

require_vs() {
  local vswhere="/c/Program Files (x86)/Microsoft Visual Studio/Installer/vswhere.exe"
  local vs_path=""
  if [[ -x "$vswhere" ]]; then
    vs_path="$("$vswhere" -latest -products '*' \
      -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 \
      -property installationPath 2>/dev/null | tr -d '\r' || true)"
  fi
  [[ -n "$vs_path" ]] || err "Visual Studio 2026 with the C++ x64 toolset not found"
  local vs_major
  vs_major="$("$vswhere" -path "$vs_path" -property installationVersion | tr -d '\r' | cut -d. -f1)"
  case "$vs_major" in
    18) export vs2026_install="$vs_path" ;;
    17) export vs2022_install="$vs_path" ;;
    *) err "Unsupported Visual Studio version ${vs_major} at ${vs_path}" ;;
  esac
  [[ -d "/c/Program Files (x86)/Windows Kits/10/Include/10.0.28000.0" ]] \
    || err "Windows SDK 10.0.28000 not installed"
  ok "Visual Studio ${vs_major}: ${vs_path}"
}

case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*) ;;
  *) err "build-windows.sh must run from Git Bash on Windows" ;;
esac

export DEPOT_TOOLS_WIN_TOOLCHAIN=0
export PATH="$DEPOT_TOOLS:$PATH"

export GIT_CONFIG_COUNT=3
export GIT_CONFIG_KEY_0=core.longpaths GIT_CONFIG_VALUE_0=true
export GIT_CONFIG_KEY_1=core.autocrlf GIT_CONFIG_VALUE_1=false
export GIT_CONFIG_KEY_2=core.filemode GIT_CONFIG_VALUE_2=false

if [[ ! -d "$DEPOT_TOOLS" ]]; then
  info "Cloning depot_tools..."
  git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git "$DEPOT_TOOLS"
fi

if should_run "fetch"; then
  cp "$COSMIUM_ROOT/.config/gclient-win.py" "$COSMIUM_ROOT/.gclient"
  ok "Set .gclient for win"

  cd "$COSMIUM_ROOT"
  info "Bootstrapping depot_tools..."
  cmd.exe //c "gclient.bat --version" >/dev/null

  info "Syncing Chromium ${VERSION} (shallow, this is long)..."
  cmd.exe //c "gclient.bat sync --revision src@refs/tags/${VERSION} --no-history --shallow --nohooks -D --force --reset"
  ok "Source synced to ${SRC_DIR}"
fi

if should_run "hooks"; then
  require_vs
  cd "$COSMIUM_ROOT"
  info "Running gclient hooks..."
  cmd.exe //c "gclient.bat runhooks"
  ok "Hooks complete"
fi

if should_run "patch"; then
  info "Applying cosmium patches..."
  cd "$SRC_DIR"
  while IFS= read -r patch; do
    patch="$(echo "$patch" | tr -d '[:space:]')"
    [[ -z "$patch" || "$patch" == \#* ]] && continue
    lf_patch="$(mktemp)"
    tr -d '\r' < "$PATCHES_DIR/$patch" > "$lf_patch"
    if git apply --check "$lf_patch" 2>/dev/null; then
      git apply --whitespace=nowarn "$lf_patch"
      ok "  $patch"
    elif git apply --check --reverse "$lf_patch" 2>/dev/null; then
      echo "  ⏭  $patch (already applied)"
    else
      git apply --check -v "$lf_patch" 2>&1 | tail -20
      err "Patch does not apply: $patch"
    fi
    rm -f "$lf_patch"
  done < "$PATCHES_DIR/series"
  ok "Patches done"
fi

if should_run "build"; then
  require_vs
  cd "$SRC_DIR"
  OUT_WIN="out\cosmium"

  info "Running gn gen..."
  mkdir -p "$BUILD_OUT"
  cp "$ARGS_GN" "$BUILD_OUT/args.gn"
  cmd.exe //c "gn.bat gen ${OUT_WIN}"
  ok "Generated build files"

  info "Building chrome with -j${JOBS}..."
  cmd.exe //c "autoninja.bat -C ${OUT_WIN} chrome -j ${JOBS}"
  ok "Build complete!"

  [[ -f "$BUILD_OUT/chrome.exe" ]] && ok "Binary: ${BUILD_OUT}/chrome.exe"
fi

echo ""
info "Cosmium ${VERSION} Windows x64 build finished!"
