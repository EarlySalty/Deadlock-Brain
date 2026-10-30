import json
import os
import sys

RANK = {"fail": 0, "partial": 1, "pass": 2}
OUTAGE = ("unavailable", "client_error")


def load(path):
    if not os.path.exists(path):
        return None
    with open(path) as handle:
        return {r["id"]: r for r in map(json.loads, filter(str.strip, handle)) if "grade" in r}


def outage(row):
    status = str(row.get("final_status") or "")
    return status in OUTAGE or status.startswith("http_")


def counts(rows):
    graded = [r["grade"] for r in rows.values() if not outage(r)]
    result = {g: graded.count(g) for g in RANK}
    result["unavailable"] = sum(map(outage, rows.values()))
    return result


def main():
    before, after = load(sys.argv[1]), load(sys.argv[2])
    if after is None:
        print("after missing:", sys.argv[2])
        sys.exit(1)
    unavailable = sorted(i for i, r in after.items() if outage(r))
    print("after", json.dumps(counts(after)))
    if before is None:
        print("before none, no diff")
        worse = []
    else:
        print("before", json.dumps(counts(before)))
        shared = [i for i in before if i in after and not outage(after[i]) and not outage(before[i])]
        rank = lambda rows, i: RANK[rows[i]["grade"]]
        worse = sorted(i for i in shared if rank(after, i) < rank(before, i))
        better = sorted(i for i in shared if rank(after, i) > rank(before, i))
        print("improved", ",".join(better) or "-")
        print("regressed", ",".join(worse) or "-")
    print("unavailable", ",".join(unavailable) or "-")
    sys.exit(1 if worse else 2 if unavailable else 0)


if __name__ == "__main__":
    main()
