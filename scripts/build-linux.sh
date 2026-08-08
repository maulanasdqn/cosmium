#!/usr/bin/env bash
# ──────────────────────────────────────────────────────────
# Cosmium Linux x86_64 build script
#
# Prerequisites: run inside `nix-shell` (or have all deps).
# Usage:
#   ./scripts/build-linux.sh              # full pipeline
#   ./scripts/build-linux.sh --only fetch
#   ./scripts/build-linux.sh --only patch
#   ./scripts/build-linux.sh --only build
#   ./scripts/build-linux.sh --jobs 14
# ──────────────────────────────────────────────────────────
set -euo pipefail

COSMIUM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
VERSION="$(cat "$COSMIUM_ROOT/VERSION" | tr -d '[:space:]')"
TARBALL_URL="https://storage.googleapis.com/chromium-browser-official/chromium-${VERSION}.tar.xz"
TARBALL_DIR="/tmp/chromium-dl"
TARBALL_PATH="${TARBALL_DIR}/chromium-${VERSION}.tar.xz"
SRC_DIR="${COSMIUM_ROOT}/src"
PATCHES_DIR="${COSMIUM_ROOT}/patches"
ARGS_GN="${COSMIUM_ROOT}/config/args.gn"
BUILD_OUT="${SRC_DIR}/out/Default"
JOBS="${JOBS:-$(( $(nproc) - 2 ))}"

# ── Parse args ──────────────────────────────────────────
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

# ── 1. Fetch ────────────────────────────────────────────
if should_run "fetch"; then
  info "Fetching Chromium ${VERSION} source tarball..."
  if [[ -d "$SRC_DIR/chrome" ]]; then
    ok "Source already extracted at ${SRC_DIR}"
  else
    mkdir -p "$TARBALL_DIR"
    if [[ ! -f "$TARBALL_PATH" ]]; then
      info "Downloading ${TARBALL_URL} ..."
      curl -# -o "$TARBALL_PATH" "$TARBALL_URL"
    else
      ok "Tarball already downloaded"
    fi

    info "Extracting tarball (this takes a few minutes)..."
    cd "$COSMIUM_ROOT"
    tar xf "$TARBALL_PATH"
    mv "chromium-${VERSION}" src
    ok "Extracted to src/"

    info "Initializing git repo in src/ (needed for patch application)..."
    cd "$SRC_DIR"
    git init -q
    git add -A
    GIT_AUTHOR_NAME="cosmium" GIT_AUTHOR_EMAIL="cosmium@build" \
    GIT_COMMITTER_NAME="cosmium" GIT_COMMITTER_EMAIL="cosmium@build" \
    git commit -q -m "chromium ${VERSION} base"
    ok "Git repo initialized"
  fi

  # Clone depot_tools if missing
  if [[ ! -d "$COSMIUM_ROOT/depot_tools" ]]; then
    info "Cloning depot_tools..."
    git clone https://chromium.googlesource.com/chromium/tools/depot_tools.git \
      "$COSMIUM_ROOT/depot_tools"
  fi
  export PATH="$COSMIUM_ROOT/depot_tools:$PATH"

  # Set up .gclient for linux target
  cp "$COSMIUM_ROOT/config/gclient-linux.py" "$COSMIUM_ROOT/.gclient"
  ok "Set .gclient for linux"

  # Initialize DEPS sub-dirs as git repos so gclient doesn't re-clone
  info "Initializing DEPS sub-directories..."
  python3 - "$SRC_DIR" << 'PYEOF'
import os, re, sys, subprocess

src = sys.argv[1]
with open(os.path.join(src, 'DEPS')) as f:
    content = f.read()

# Match every `'src/...':` dep key regardless of how its value is spelled —
# a dict, a bare URL, or `Var('chromium_git') + '/foo.git'`. The narrower
# value-shape match missed ~120 dirs (angle, boringssl, quiche, …) and
# `gclient runhooks` then failed on the first one that was not a git repo.
paths = set(re.findall(r"^\s*'(src/[^']+)'\s*:", content, re.M))
paths |= set(re.findall(r'^\s*"(src/[^"]+)"\s*:', content, re.M))

count = 0
for p in sorted(paths):
    local = os.path.join(src, p.replace('src/', '', 1))
    if os.path.isdir(local) and not os.path.isdir(os.path.join(local, '.git')):
        subprocess.run(['git', 'init', '-q'], cwd=local,
                       capture_output=True)
        subprocess.run(['git', 'add', '-A'], cwd=local,
                       capture_output=True)
        env = {**os.environ,
               'GIT_AUTHOR_NAME': 'cosmium',
               'GIT_AUTHOR_EMAIL': 'c@b',
               'GIT_COMMITTER_NAME': 'cosmium',
               'GIT_COMMITTER_EMAIL': 'c@b'}
        subprocess.run(['git', 'commit', '-q', '--allow-empty', '-m', 'base'],
                       cwd=local, capture_output=True, env=env)
        count += 1
print(f"  initialized {count} sub-directories")
PYEOF
  ok "DEPS directories ready"

  # Run gclient hooks (fetches remaining tools)
  info "Running gclient hooks..."
  cd "$COSMIUM_ROOT"
  gclient runhooks 2>&1 | tail -5
  ok "Hooks complete"

  # Download Chromium's clang — MUST run after `gclient runhooks`.
  # third_party/llvm-build/Release+Asserts is a `dep_type: 'gcs'` entry, and
  # runhooks wipes that directory without re-fetching it (only `gclient sync`
  # fetches GCS deps, and this tarball workflow never syncs). Fetching before
  # hooks silently loses the toolchain and `gn gen` then fails with
  # "the actual version is <blank>".
  #
  # Gate on the revision check, not on bin/clang existing: the release tarball
  # ships that path with an EMPTY cr_build_revision stamp, so a file-existence
  # guard would skip the real download.
  if ! python3 "$SRC_DIR/tools/clang/scripts/update.py" --print-revision \
       >/dev/null 2>&1; then
    info "Downloading Clang toolchain..."
    python3 "$SRC_DIR/tools/clang/scripts/update.py"
  fi
  # Verify rather than assume — this is the exact check gn gen runs, so
  # failing here gives a clear error instead of a confusing GN backtrace.
  CLANG_REV="$(python3 "$SRC_DIR/tools/clang/scripts/update.py" --print-revision 2>/dev/null)" \
    || err "Clang toolchain still missing/invalid after update.py"
  ok "Clang toolchain ready (${CLANG_REV})"
fi

# ── 2. Patch ────────────────────────────────────────────
if should_run "patch"; then
  info "Applying cosmium patches..."
  cd "$SRC_DIR"
  while IFS= read -r patch; do
    patch="$(echo "$patch" | tr -d '[:space:]')"
    [[ -z "$patch" || "$patch" == \#* ]] && continue
    if git apply --check "$PATCHES_DIR/$patch" 2>/dev/null; then
      git apply --whitespace=nowarn "$PATCHES_DIR/$patch"
      ok "  $patch"
    else
      echo "  ⏭  $patch (already applied or N/A)"
    fi
  done < "$PATCHES_DIR/series"
  ok "Patches done"
fi

# ── 3. Build ────────────────────────────────────────────
if should_run "build"; then
  export PATH="$COSMIUM_ROOT/depot_tools:$PATH"
  cd "$SRC_DIR"

  info "Running gn gen..."
  mkdir -p "$BUILD_OUT"
  ARGS=$(grep -v '^#' "$ARGS_GN" | grep -v '^$' | tr '\n' ' ')
  gn gen "$BUILD_OUT" --args="$ARGS"
  ok "Generated $(wc -l < "$BUILD_OUT/build.ninja" | tr -d ' ') ninja rules"

  info "Building chrome with -j${JOBS}..."
  ninja -C "$BUILD_OUT" chrome -j"$JOBS"
  ok "Build complete!"

  # Quick verify
  if [[ -f "$BUILD_OUT/chrome" ]]; then
    SIZE=$(du -sh "$BUILD_OUT/chrome" | cut -f1)
    ok "Binary: ${BUILD_OUT}/chrome (${SIZE})"
  fi
fi

echo ""
info "🎉 Cosmium ${VERSION} Linux x86_64 build finished!"
