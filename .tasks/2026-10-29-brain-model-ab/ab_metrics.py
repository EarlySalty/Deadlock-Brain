import json
import statistics
import subprocess
import sys
from collections import defaultdict

MODELS = ["haiku", "opus", "sonnet"]


def journal(model, since):
    output = subprocess.run(
        ["journalctl", "--user", "-u", f"brain-ab-{model}", "--since", since, "--no-pager", "-o", "cat"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout
    events = defaultdict(list)
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if "request_id" in event:
            events[event["request_id"]].append(event)
    return events


def p90(values):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int(round(0.9 * (len(ordered) - 1))))]


def main():
    run_path, since = sys.argv[1], sys.argv[2]
    rows = [json.loads(line) for line in open(run_path)]
    enriched = []
    summary = {}
    for model in MODELS:
        events = journal(model, since)
        latencies, rounds, tokens_in, tokens_out, gate = [], [], 0, 0, 0
        statuses = defaultdict(int)
        served = set()
        missing = []
        for row in rows:
            result = row["results"][model]
            evs = events.get(result["request_id"], [])
            usage = [e for e in evs if e.get("event") == "brain_provider_usage"]
            served.update(e.get("model") for e in usage)
            result["provider_rounds"] = len(usage)
            result["input_tokens"] = sum(e.get("input_tokens", 0) for e in usage)
            result["output_tokens"] = sum(e.get("output_tokens", 0) for e in usage)
            result["gate_rejections"] = sum(1 for e in evs if e.get("reason") == "value_gate_rejected" and e.get("event") == "brain_tool_round_rejected")
            result["not_answered"] = next((e.get("reason") for e in evs if e.get("event") == "brain_answer_not_answered"), None)
            latencies.append(result["latency_ms"])
            rounds.append(result["provider_rounds"])
            tokens_in += result["input_tokens"]
            tokens_out += result["output_tokens"]
            gate += result["gate_rejections"]
            statuses[result["status"]] += 1
            if not usage:
                missing.append(result["request_id"])
        summary[model] = {
            "served_models": sorted(m for m in served if m),
            "statuses": dict(statuses),
            "latency_median_ms": int(statistics.median(latencies)),
            "latency_p90_ms": p90(latencies),
            "rounds_mean": round(statistics.mean(rounds), 2),
            "rounds_median": statistics.median(rounds),
            "rounds_total": sum(rounds),
            "requests_without_usage": missing,
            "input_tokens": tokens_in,
            "output_tokens": tokens_out,
            "gate_rejections": gate,
        }
    with open(run_path.replace(".jsonl", "-enriched.jsonl"), "w") as out:
        for row in rows:
            out.write(json.dumps(row, ensure_ascii=False) + "\n")
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main()
