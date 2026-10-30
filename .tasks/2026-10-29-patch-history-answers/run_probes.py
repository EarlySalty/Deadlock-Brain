import json
import os
import re
import sys
import time
import urllib.error
import urllib.request
import uuid

BASE = os.environ.get("BRAIN_BASE_URL", "http://127.0.0.1:8788")
TOKEN_ENV = "DISCORD_BRAIN_CLIENT_TOKEN"
PROBE_USER_ID = os.environ.get("PROBE_DISCORD_USER_ID", "100000000000000001")
PAUSE_S = float(os.environ.get("PROBE_PAUSE_S", "5"))
PREFIX = os.environ.get("PROBE_PREFIX", "probe-ph-")
CONTEXT = {
    "platform": "discord",
    "is_thread": False,
    "is_direct_message": False,
    "input_kind": "mention",
}


def query(request_id, text, with_context):
    body = {
        "request_id": request_id,
        "conversation_id": request_id,
        "text": text,
        "requested_scopes": ["bot.public"],
        "profile": "explain",
    }
    if with_context:
        body["answer_context"] = CONTEXT
    return body


def post(payload, token):
    request = urllib.request.Request(
        f"{BASE}/v1/answer",
        data=json.dumps(payload).encode(),
        method="POST",
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json",
            "x-discord-user-id": PROBE_USER_ID,
            "x-discord-read-access": "disabled",
        },
    )
    started = time.monotonic()
    try:
        with urllib.request.urlopen(request, timeout=620) as response:
            data = json.loads(response.read())
            http = response.status
    except urllib.error.HTTPError as error:
        data = {"status": f"http_{error.code}", "text": error.read()[:500].decode(errors="replace")}
        http = error.code
    except Exception as error:
        data = {"status": "client_error", "text": type(error).__name__}
        http = 0
    return http, data, int((time.monotonic() - started) * 1000)


def run_turns(probe, token, run_id):
    history = []
    turns = []
    for index, text in enumerate(probe["turns"]):
        request_id = f"{PREFIX}{run_id}-{probe['id']}-{index + 1}"
        if history:
            payload = {"query": query(request_id, text, False), "user_questions": history[-4:]}
        else:
            payload = query(request_id, text, True)
        http, data, latency = post(payload, token)
        turns.append({
            "request_id": request_id,
            "text": text,
            "http": http,
            "status": data.get("status"),
            "latency_ms": latency,
            "answer": data.get("text", ""),
            "citations": len(data.get("citations", []) or []),
        })
        history.append(text)
        time.sleep(PAUSE_S)
    return turns


def iso_dates(answer):
    return re.sub(
        r"(?<![\d.])(\d{1,2})\.(\d{1,2})\.(\d{4})(?!\d)",
        lambda m: f"{m.group(3)}-{int(m.group(2)):02d}-{int(m.group(1)):02d}",
        answer,
    )


def normalize(answer):
    text = re.sub(r"(?<=\d)[\u00a0\u202f](?=\d{3}\b)", "", iso_dates(answer))
    text = re.sub(r"(?<=\d)\.(?=\d{3}(?!\d))", "", text)
    return re.sub(r"(?<=\d),(?=\d)", ".", text)


def hits(text, alternative):
    if re.fullmatch(r"[\d.]+", alternative):
        return re.search(rf"(?<![\d.]){re.escape(alternative)}(?![\d]|\.\d)", text) is not None
    return alternative.lower() in text.lower()


def grade(probe, final):
    if final["status"] != "answered":
        return "fail"
    text = normalize(final["answer"])
    hit = sum(any(hits(text, alt) for alt in group) for group in probe["expect"])
    if hit == len(probe["expect"]):
        return "pass"
    return "partial" if hit else "fail"


def verdict(probe, answer):
    text = answer.replace(" ", " ")
    hit = any(token in text for token in probe["must_any"])
    miss = any(token in text for token in probe.get("must_not", []))
    return hit and not miss


def main():
    token = os.environ.get(TOKEN_ENV)
    if not token:
        sys.exit(f"{TOKEN_ENV} missing")
    probes_path, out_path = sys.argv[1], sys.argv[2]
    only = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else None
    run_id = uuid.uuid4().hex[:8]
    with open(probes_path) as handle:
        probes = json.load(handle)
    counts = {}
    with open(out_path, "a") as out:
        for probe in probes:
            if only and probe["id"] not in only:
                continue
            turns = run_turns(probe, token, run_id)
            final = turns[-1]
            record = {
                "id": probe["id"],
                "kind": probe["kind"],
                "topic": probe["topic"],
                "expected": probe["expected"],
                "turns": turns,
                "final_status": final["status"],
                "auto_value_hit": final["status"] == "answered" and "must_any" in probe and verdict(probe, final["answer"]),
            }
            if "expect" in probe:
                record["grade"] = grade(probe, final)
                counts[record["grade"]] = counts.get(record["grade"], 0) + 1
            out.write(json.dumps(record, ensure_ascii=False) + "\n")
            out.flush()
            print(probe["id"], final["status"], final["latency_ms"], record.get("grade", record["auto_value_hit"]), flush=True)
    if counts:
        print("summary", run_id, json.dumps(counts, sort_keys=True), flush=True)


if __name__ == "__main__":
    main()
