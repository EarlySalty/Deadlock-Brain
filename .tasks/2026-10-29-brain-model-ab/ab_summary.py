import json
import statistics
import sys

MODELS = ("haiku", "opus", "sonnet")


def load_rows(paths):
    rows = {}
    for path in paths:
        for line in open(path):
            row = json.loads(line)
            if row["results"]["haiku"]["status"] != "unavailable":
                rows[row["id"]] = row
    return rows


def p90(values):
    ordered = sorted(values)
    return ordered[min(len(ordered) - 1, int(round(0.9 * (len(ordered) - 1))))]


def summarize(rows, grades):
    out = {}
    for model in MODELS:
        counts = {"pass": 0, "partial": 0, "fail": 0, "silent": 0}
        wrong = 0
        lat = []
        for item_id, row in rows.items():
            lat.append(row["results"][model]["latency_ms"])
            grade = grades.get(item_id, {}).get(model)
            if grade:
                counts[grade["grade"]] += 1
                wrong += grade["wrong_facts"]
        graded = sum(counts.values())
        out[model] = {
            "graded": graded,
            **counts,
            "silent_rate": round(counts["silent"] / graded, 3) if graded else None,
            "wrong_facts": wrong,
            "latency_median_ms": int(statistics.median(lat)),
            "latency_p90_ms": p90(lat),
        }
    return out


def main():
    grades = json.load(open(sys.argv[1]))
    rows = load_rows(sys.argv[2:])
    print(json.dumps({"rows": len(rows), "models": summarize(rows, grades)}, indent=1))


if __name__ == "__main__":
    main()
