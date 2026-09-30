"""Thin browser adapter. Persistent credentials are owned by the Rust DB helper.

Private inherited pipes only: no stdout, temporary token files or browser
profiles. Every save uses the revision returned by load to prevent overwrite.
"""

from __future__ import annotations

import json
import os
import select
import subprocess
import time
from pathlib import Path

LIMIT = 1024 * 1024
TIMEOUT = 35.0


def _exchange(operation: str, payload: dict | None = None) -> dict | None:
    helper = os.environ.get(
        "GEMINI_SESSION_HELPER",
        str(
            Path(__file__).resolve().parents[1] / "target/release/dbrain-session-store"
        ),
    )
    read_fd, write_fd = os.pipe()
    inherited = write_fd if operation == "get" else read_fd
    local = read_fd if operation == "get" else write_fd
    env = os.environ.copy()
    env["GEMINI_SESSION_FD"] = str(inherited)
    process = None
    deadline = time.monotonic() + TIMEOUT
    try:
        process = subprocess.Popen(
            [helper, operation],
            env=env,
            pass_fds=(inherited,),
            stdin=subprocess.DEVNULL,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        os.close(inherited)
        inherited = -1
        os.set_blocking(local, False)
        if operation == "get":
            result = bytearray()
            while True:
                remaining = deadline - time.monotonic()
                if remaining <= 0 or not select.select([local], [], [], remaining)[0]:
                    raise RuntimeError("Sitzungsdatenbank antwortet nicht rechtzeitig.")
                chunk = os.read(local, 65536)
                if not chunk:
                    break
                result.extend(chunk)
                if len(result) > LIMIT:
                    raise RuntimeError("Sitzungsdatenbank liefert zu große Daten.")
        else:
            data = json.dumps(payload, ensure_ascii=True).encode("utf-8")
            if len(data) > LIMIT:
                raise RuntimeError("Browser-Sitzung überschreitet die Speichergrenze.")
            sent = 0
            while sent < len(data):
                remaining = deadline - time.monotonic()
                if remaining <= 0 or not select.select([], [local], [], remaining)[1]:
                    raise RuntimeError("Sitzungsdatenbank antwortet nicht rechtzeitig.")
                sent += os.write(local, data[sent : sent + 65536])
        os.close(local)
        local = -1
        code = process.wait(timeout=max(0.1, deadline - time.monotonic()))
        if code:
            raise RuntimeError(
                "Verschlüsselter DB-Sitzungsspeicher nicht verfügbar. Kein Datei-Fallback."
            )
        if operation == "get":
            envelope = json.loads(result)
            if not isinstance(envelope, dict) or not isinstance(
                envelope.get("revision"), int
            ):
                raise RuntimeError(
                    "DB-Sitzungsantwort hat keinen gültigen Versionsstand."
                )
            return envelope
        return None
    finally:
        for fd in (inherited, local):
            if fd >= 0:
                os.close(fd)
        if process is not None and process.poll() is None:
            process.kill()
            process.wait()


def load() -> dict:
    envelope = _exchange("get")
    if envelope is None:
        raise RuntimeError("Sitzungsdatenbank hat nicht geantwortet.")
    return envelope


def save(state: dict, revision: int) -> None:
    _exchange("put", {"state": state, "revision": revision})


class DbBrowserContext:
    """Nonpersistent Chromium context, with an explicit successful-save boundary."""

    def __init__(self, browser, context, revision: int):
        self.browser = browser
        self.context = context
        self.revision = revision

    def __getattr__(self, name):
        return getattr(self.context, name)

    def save(self) -> None:
        # IndexedDB is part of some providers' login state; unsupported browser
        # versions fail instead of silently dropping that part of the session.
        save(self.context.storage_state(indexed_db=True), self.revision)
        self.revision += 1

    def close(self) -> None:
        try:
            self.context.close()
        finally:
            self.browser.close()
