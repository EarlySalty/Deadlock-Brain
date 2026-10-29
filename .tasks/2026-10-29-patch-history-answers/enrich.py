import hashlib
import json
import re
import subprocess
import sys

PSQL = "psql -h /run/deadlock-brain-postgresql -p 5446 -U brain_migrate -d brain -At -F '|' -c"


def journal_reasons(since):
    output = subprocess.run(
        ["journalctl", "-u", "brain-serve", "--since", since, "--no-pager", "-o", "cat"],
        capture_output=True, text=True,
    ).stdout
    reasons = {}
    for line in output.splitlines():
        match = re.search(r"\{.*\}", line)
        if not match:
            continue
        try:
            event = json.loads(match.group(0))
        except json.JSONDecodeError:
            continue
        request_id = event.get("request_id", "")
        if not request_id.startswith("probe-ph-"):
            continue
        entry = reasons.setdefault(request_id, {"reasons": set(), "events": set()})
        if event.get("reason"):
            entry["reasons"].add(event["reason"])
        if event.get("event"):
            entry["events"].add(event["event"])
    return reasons


def traces(texts):
    hashes = {hashlib.sha256(text.encode()).hexdigest(): text for text in texts}
    sql = (
        "select encode(question_sha256,'hex'), tool_calls::text, status, latency_ms "
        "from ops.answer_trace where recorded_at > now() - interval '1 day' order by id"
    )
    output = subprocess.run(
        ["sg", "deadlock-brain-db", "-c", f'{PSQL} "{sql}"'],
        capture_output=True, text=True,
    ).stdout
    found = {}
    for line in output.splitlines():
        digest, tools, status, latency = line.split("|", 3)
        if digest in hashes:
            found[hashes[digest]] = {"tools": json.loads(tools), "trace_status": status}
    return found


def main():
    results_path, since = sys.argv[1], sys.argv[2]
    records = [json.loads(line) for line in open(results_path)]
    reasons = journal_reasons(since)
    texts = [turn["text"] for record in records for turn in record["turns"]]
    traced = traces(texts)
    for record in records:
        for turn in record["turns"]:
            entry = reasons.get(turn["request_id"], {"reasons": set(), "events": set()})
            turn["reasons"] = sorted(entry["reasons"])
            turn["events"] = sorted(entry["events"])
            trace = traced.get(turn["text"], {})
            turn["tools"] = trace.get("tools", [])
        print(json.dumps(record, ensure_ascii=False))


if __name__ == "__main__":
    main()
