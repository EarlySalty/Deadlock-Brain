import hashlib
import json
import os
import subprocess
import sys

PSQL = "psql -h /run/deadlock-brain-postgresql -p 5446 -U brain_migrate -d brain -At -F '|' -v ON_ERROR_STOP=1 -c"
TRACE_SLACK_US = 10_000_000
LATENCY_SLACK_MS = 5_000
PREFIX = os.environ.get("PROBE_PREFIX", "probe-ph-")
PRICE_PER_MTOK = {"claude-haiku-5-5": (0.10, 0.50)}


def run(command):
    result = subprocess.run(command, capture_output=True, text=True)
    if result.returncode != 0:
        sys.exit(f"telemetry command failed ({command[0]}): {result.stderr.strip()[:300]}")
    return result.stdout


def journal(since):
    output = run(["journalctl", "-u", "brain-serve", "--since", since, "--no-pager", "-o", "json"])
    requests = {}
    for line in output.splitlines():
        entry = json.loads(line)
        message = entry.get("MESSAGE")
        if not isinstance(message, str) or not message.startswith("{"):
            continue
        try:
            event = json.loads(message)
        except json.JSONDecodeError:
            continue
        request_id = event.get("request_id", "")
        if not request_id.startswith(PREFIX):
            continue
        stamp = int(entry["__REALTIME_TIMESTAMP"])
        item = requests.setdefault(request_id, {"reasons": set(), "events": set(), "first": stamp, "last": stamp, "usage": []})
        item["first"] = min(item["first"], stamp)
        item["last"] = max(item["last"], stamp)
        if event.get("reason"):
            item["reasons"].add(event["reason"])
        if event.get("event"):
            item["events"].add(event["event"])
        if event.get("event") == "brain_provider_usage":
            item["usage"].append((event.get("model"), int(event.get("input_tokens") or 0), int(event.get("output_tokens") or 0)))
    return requests


def traces(start_us, end_us):
    sql = (
        "select id, encode(question_sha256,'hex'), tool_calls::text, status, latency_ms, "
        "(extract(epoch from recorded_at) * 1000000)::bigint from ops.answer_trace "
        f"where recorded_at between to_timestamp({start_us} / 1e6) and to_timestamp({end_us} / 1e6) order by id"
    )
    output = run(["sg", "deadlock-brain-db", "-c", f'{PSQL} "{sql}"'])
    rows = []
    for line in output.splitlines():
        row_id, digest, tools, status, latency, recorded = line.split("|")
        rows.append({
            "id": int(row_id),
            "digest": digest,
            "tools": json.loads(tools),
            "status": status,
            "latency_ms": int(latency),
            "recorded_us": int(recorded),
        })
    return rows


def trace_status(status):
    return status if status in ("answered", "insufficient_evidence") else "failed"


def match(turn, window, rows, used):
    digest = hashlib.sha256(turn["text"].encode()).hexdigest()
    candidates = [
        row for row in rows
        if row["id"] not in used
        and window["first"] <= row["recorded_us"] <= window["last"] + TRACE_SLACK_US
        and row["status"] in (turn["status"], trace_status(turn["status"]))
        and turn["latency_ms"] - LATENCY_SLACK_MS <= row["latency_ms"] <= turn["latency_ms"]
    ]
    exact = [row for row in candidates if row["digest"] == digest]
    if len(exact) == 1:
        return exact[0], "hash_and_window"
    if len(candidates) == 1:
        return candidates[0], "window"
    return None, "ambiguous" if candidates else "missing"


def cost(usage):
    total = 0.0
    unpriced = set()
    for model, tokens_in, tokens_out in usage:
        price = PRICE_PER_MTOK.get(model)
        if price is None:
            unpriced.add(model)
            continue
        total += tokens_in / 1e6 * price[0] + tokens_out / 1e6 * price[1]
    return total, unpriced


def main():
    results_path, since = sys.argv[1], sys.argv[2]
    with open(results_path) as handle:
        records = [json.loads(line) for line in handle]
    requests = journal(since)
    if not requests:
        sys.exit("no probe request ids in the journal window")
    rows = traces(min(r["first"] for r in requests.values()) - TRACE_SLACK_US,
                  max(r["last"] for r in requests.values()) + TRACE_SLACK_US)
    used = set()
    usage = []
    for record in records:
        for turn in record["turns"]:
            window = requests.get(turn["request_id"])
            turn["reasons"] = sorted(window["reasons"]) if window else None
            turn["events"] = sorted(window["events"]) if window else None
            row, how = match(turn, window, rows, used) if window else (None, "no_journal")
            if row:
                used.add(row["id"])
            turn["tools"] = row["tools"] if row else None
            turn["trace_match"] = how
            turn_usage = window["usage"] if window else []
            usage.extend(turn_usage)
            turn["input_tokens"] = sum(u[1] for u in turn_usage)
            turn["output_tokens"] = sum(u[2] for u in turn_usage)
        print(json.dumps(record, ensure_ascii=False))
    total, unpriced = cost(usage)
    summary = {
        "provider_calls": len(usage),
        "input_tokens": sum(u[1] for u in usage),
        "output_tokens": sum(u[2] for u in usage),
        "usd": round(total, 4),
        "unpriced_models": sorted(m or "unknown" for m in unpriced),
    }
    print(json.dumps(summary), file=sys.stderr)


if __name__ == "__main__":
    main()
