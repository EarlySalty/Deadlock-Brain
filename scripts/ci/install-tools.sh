#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
dest="${RUNNER_TEMP:-/tmp}/brain-ci-tools"
mkdir -p "$dest"
for wanted in "$@"; do
  row=$(awk -v tool="$wanted" '$1 == tool { print }' scripts/ci/tools.tsv)
  test -n "$row"
  read -r name url digest <<< "$row"
  scratch=$(mktemp -d)
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location "$url" -o "$scratch/tool.tar.gz"
  printf '%s  %s\n' "$digest" "$scratch/tool.tar.gz" | sha256sum --check --strict
  tar --extract --gzip --no-same-owner --file "$scratch/tool.tar.gz" --directory "$scratch"
  count=$(find "$scratch" -type f -name "$name" | wc -l)
  test "$count" -eq 1
  candidate=$(find "$scratch" -type f -name "$name")
  install -m 0755 "$candidate" "$dest/$name"
done
if [[ -n "${GITHUB_PATH:-}" ]]; then printf '%s\n' "$dest" >> "$GITHUB_PATH"; fi
printf 'Tools installed in %s\n' "$dest"
