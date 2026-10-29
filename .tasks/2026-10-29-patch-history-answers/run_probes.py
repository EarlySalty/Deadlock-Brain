import json
import os
import sys
import time
import urllib.error
import urllib.request
import uuid

BASE = os.environ.get("BRAIN_BASE_URL", "http://127.0.0.1:8788")
TOKEN_ENV = "DISCORD_BRAIN_CLIENT_TOKEN"
PROBE_USER_ID = os.environ.get("PROBE_DISCORD_USER_ID", "100000000000000001")
PAUSE_S = float(os.environ.get("PROBE_PAUSE_S", "5"))
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
        request_id = f"probe-ph-{run_id}-{probe['id']}-{index + 1}"
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
                "auto_value_hit": final["status"] == "answered" and verdict(probe, final["answer"]),
            }
            out.write(json.dumps(record, ensure_ascii=False) + "\n")
            out.flush()
            print(probe["id"], final["status"], final["latency_ms"], record["auto_value_hit"], flush=True)


if __name__ == "__main__":
    main()
