#!/usr/bin/env bash
set -euo pipefail

cd "$(git rev-parse --show-toplevel)"

read -r -d '' program <<'AWK' || true
FNR == 1 { block = 0 }
{
  line = $0
  gsub(/"([^"\\]|\\.)*"/, "\"\"", line)
  if (FILENAME !~ /\.rs$/) gsub(/'([^'\\]|\\.)*'/, "''", line)
  gsub(/[A-Za-z]+:\/\//, "", line)
  gsub(/\*\/\*/, "", line)
  if (line ~ /^[[:space:]]*\/\/ @ts-check[[:space:]]*$/) next
  if (block || line ~ /\/\// || line ~ /\/\*/) {
    print FILENAME ":" FNR ": " $0
    block = (line ~ /\/\*/ && line !~ /\*\//) || (block && line !~ /\*\//)
  }
}
AWK

if [ "$#" -gt 0 ]; then
  files=$(printf '%s\n' "$@")
else
  files=$(git ls-files -- '*.rs' '*.ts' '*.tsx' '*.js' '*.mjs' '*.css')
fi

offenders=$(printf '%s\n' "$files" \
  | grep -v '^patches/' \
  | grep -E '\.(rs|ts|tsx|js|mjs|css)$' \
  | while read -r f; do [ -f "$f" ] && printf '%s\n' "$f"; done \
  | xargs -r awk "$program")

if [ -n "$offenders" ]; then
  echo "Comments are not allowed:"
  echo "$offenders"
  exit 1
fi
