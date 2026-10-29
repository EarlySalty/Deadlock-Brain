import json
import sys

RANK = {"fail": 0, "partial": 1, "pass": 2}


def load(path):
    with open(path) as handle:
        return {r["id"]: r.get("grade", "fail") for r in map(json.loads, handle) if "grade" in r}


def main():
    before, after = load(sys.argv[1]), load(sys.argv[2])
    worse = sorted(i for i in before if i in after and RANK[after[i]] < RANK[before[i]])
    better = sorted(i for i in before if i in after and RANK[after[i]] > RANK[before[i]])
    for name, grades in (("before", before), ("after", after)):
        counts = {g: list(grades.values()).count(g) for g in RANK}
        print(name, json.dumps(counts))
    print("improved", ",".join(better) or "-")
    print("regressed", ",".join(worse) or "-")
    sys.exit(1 if worse else 0)


if __name__ == "__main__":
    main()
