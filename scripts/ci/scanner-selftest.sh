#!/usr/bin/env bash
# Inert, disposable scanner fixtures, never executed or committed.
set -euo pipefail
cd "$(dirname "$0")/../.."
scratch=$(mktemp -d)
expect_finding() {
  set +e
  "$@"
  result=$?
  set -e
  test "$result" -eq 1
}
mkdir "$scratch/python" "$scratch/rust" "$scratch/secrets" "$scratch/deps"
printf '%s\n' 'eval(input())' > "$scratch/python/probe.py"
printf '%s\n' 'fn probe() { client.danger_accept_invalid_certs(true); }' > "$scratch/rust/probe.rs"
expect_finding semgrep scan --config .semgrep.yml --metrics off --disable-version-check --strict --error --no-git-ignore --json-output "$scratch/python.json" "$scratch/python"
jq -e 'any(.results[]; .check_id | endswith("python-dynamic-eval"))' "$scratch/python.json"
expect_finding semgrep scan --config .semgrep.yml --metrics off --disable-version-check --strict --error --no-git-ignore --json-output "$scratch/rust.json" "$scratch/rust"
jq -e 'any(.results[]; .check_id | endswith("rust-disabled-tls"))' "$scratch/rust.json"
expect_finding bandit -r "$scratch/python" -ll
# Fresh, synthetic shape only. Not a real credential and never used for authentication.
printf 'github_token = "ghp_%s"\n' "$(openssl rand -hex 18)" > "$scratch/secrets/probe.txt"
expect_finding gitleaks dir "$scratch/secrets" --redact --no-banner
printf '%s\n' 'cryptography==3.3.1' > "$scratch/deps/requirements.txt"
expect_finding trivy fs --scanners vuln --severity HIGH,CRITICAL --exit-code 1 --format json --output "$scratch/trivy.json" "$scratch/deps"
jq -e '[.Results[]?.Vulnerabilities[]? | select(.Severity == "HIGH" or .Severity == "CRITICAL")] | length > 0' "$scratch/trivy.json"
expect_finding pip-audit --no-deps --disable-pip -r "$scratch/deps/requirements.txt"
printf 'All scanner negative controls rejected the inert fixtures\n'
