#!/usr/bin/env bash
set -euo pipefail

args=()
for arg in "$@"; do
  case "${arg}" in
    --user-data-dir=*)
      args+=(--user-data-dir=/tmp/cosmium-profile)
      ;;
    *)
      args+=("${arg}")
      ;;
  esac
done

exec docker run --rm --platform linux/amd64 cosmium:cpu "${args[@]}"
