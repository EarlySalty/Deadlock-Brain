import json
import os
import sys
import threading
import time
import urllib.error
import urllib.request
import uuid

MODELS = {"haiku": 18891, "opus": 18892, "sonnet": 18893}
TOKEN_ENV = "DISCORD_BRAIN_CLIENT_TOKEN"
PROBE_USER_ID = "100000000000000001"
PAUSE_S = float(os.environ.get("AB_PAUSE_S", "3"))
CONTEXT = {
    "platform": "discord",
    "is_thread": False,
    "is_direct_message": False,
    "input_kind": "mention",
}


def payload(request_id, item):
    query = {
        "request_id": request_id,
        "conversation_id": request_id,
        "text": item["question"],
        "requested_scopes": ["bot.public"],
        "profile": "explain",
    }
    if item["history"]:
        return {"query": query, "user_questions": item["history"]}
    query["answer_context"] = CONTEXT
    return query


def post(port, body, token):
    request = urllib.request.Request(
        f"http://127.0.0.1:{port}/v1/answer",
        data=json.dumps(body).encode(),
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


def run_item(item, token, run_id):
    results = {}

    def one(model, port):
        request_id = f"ab-{run_id}-{model}-{item['id']}"
        http, data, latency = post(port, payload(request_id, item), token)
        results[model] = {
            "request_id": request_id,
            "http": http,
            "status": data.get("status"),
            "latency_ms": latency,
            "answer": data.get("text", ""),
            "citations": len(data.get("citations", []) or []),
        }

    threads = [threading.Thread(target=one, args=(m, p)) for m, p in MODELS.items()]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join()
    return results


def main():
    token = os.environ.get(TOKEN_ENV)
    if not token:
        sys.exit(f"{TOKEN_ENV} missing")
    questions_path, out_path = sys.argv[1], sys.argv[2]
    only = set(sys.argv[3].split(",")) if len(sys.argv) > 3 else None
    run_id = uuid.uuid4().hex[:8]
    with open(questions_path) as handle:
        items = json.load(handle)
    with open(out_path, "a") as out:
        for item in items:
            if only and item["id"] not in only:
                continue
            results = run_item(item, token, run_id)
            out.write(json.dumps({**item, "results": results}, ensure_ascii=False) + "\n")
            out.flush()
            print(item["id"], {m: (r["status"], r["latency_ms"]) for m, r in results.items()}, flush=True)
            time.sleep(PAUSE_S)


if __name__ == "__main__":
    main()
