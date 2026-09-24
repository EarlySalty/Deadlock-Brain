#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/../.."
good='{"rust":{"result":"success"},"python":{"result":"success"},"rust-security":{"result":"success"},"source-security":{"result":"success"},"workflow-policy":{"result":"success"},"codeql":{"result":"success"}}'
jq -e -f scripts/ci/gate.jq <<< "$good"
for result in failure cancelled skipped neutral timed_out action_required ''; do
  bad=$(jq --arg result "$result" '.rust.result=$result' <<< "$good")
  if jq -e -f scripts/ci/gate.jq <<< "$bad"; then echo "Gate accepted $result"; exit 1; fi
done
for mutation in 'del(.rust)' '.unexpected={"result":"success"}' '{}'; do
  bad=$(jq "$mutation" <<< "$good")
  if jq -e -f scripts/ci/gate.jq <<< "$bad"; then echo 'Gate accepted missing/extra jobs'; exit 1; fi
done
clean='{"version":"2.1.0","runs":[{"tool":{"driver":{"name":"CodeQL","rules":[{"id":"test"}]}},"results":[]}]}'
jq -e -f scripts/ci/sarif.jq <<< "$clean"
for mutation in '.runs=[]' '.runs[0].tool.driver.rules=[]' '.runs[0].results=[{"level":"error"}]' '.runs[0].results=[{}]' '.runs[0].invocations=[{"executionSuccessful":false}]'; do
  bad=$(jq "$mutation" <<< "$clean")
  if jq -e -f scripts/ci/sarif.jq <<< "$bad"; then echo 'SARIF policy accepted invalid report'; exit 1; fi
done
printf 'Gate and SARIF negative controls passed\n'
