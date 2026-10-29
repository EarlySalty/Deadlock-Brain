import json
import sys

MODELS = ("haiku", "opus", "sonnet")


def main():
    path = sys.argv[1]
    wanted = set(sys.argv[2].split(",")) if len(sys.argv) > 2 else None
    for line in open(path):
        row = json.loads(line)
        if wanted and row["id"] not in wanted:
            continue
        print("=====", row["id"], row.get("topic"), "| Q:", row["question"][:240])
        if row.get("history"):
            print("HISTORY:", " / ".join(h[:120] for h in row["history"]))
        print("EXPECTED:", json.dumps(row.get("expected"), ensure_ascii=False)[:700])
        for model in MODELS:
            result = row["results"][model]
            answer = (result.get("answer") or "").replace("\n", " ")
            print(f"--[{model}] {result['status']} {result['latency_ms']}ms: {answer[:900]}")


if __name__ == "__main__":
    main()
