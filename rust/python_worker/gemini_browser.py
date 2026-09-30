#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import logging
import os
import random
import sys
import time

import db_browser_session

# Browserzustand kommt ausschließlich aus dem verschlüsselten DB-Speicher.
BRAVE_PATH = os.environ.get("GEMINI_BROWSER_PATH", "/opt/brave.com/brave/brave")
LOGGER = logging.getLogger(__name__)
GEMINI_URL = "https://gemini.google.com/app"
NAV_TIMEOUT_MS = 45_000
RESPONSE_TIMEOUT_SECONDS = int(os.environ.get("GEMINI_RESPONSE_TIMEOUT_SECONDS", "300"))

# Turn 2: wandelt die natürliche Analyse aus Turn 1 in striktes Claim-JSON um.
# Reines Text->Text (kein Video-Tool) -> löst den Stopp-Mechanismus nicht aus.
JSON_CONVERT_PROMPT = (
    "Wandle die oben genannten Erkenntnisse in genau dieses JSON um. Gib NUR das JSON aus, "
    "ohne Markdown, ohne Codeblock, ohne weiteren Text:\n"
    '{"claims": [{"entity": "Hero, Item oder Fähigkeit", '
    '"claim_type": "build|item_timing|matchup|mechanic|combo|meta", '
    '"assertion": "klarer deutscher Satz", "patch_context": null, "confidence": 0.0}]}\n'
    'Enthält die Analyse kein konkretes Deadlock-Gameplay-Wissen, gib {"claims": []} zurück.'
)

# TODO: gegen Live-DOM verifizieren.
NEW_CHAT_SELECTORS = [
    "a[aria-label*='New chat']",
    "button[aria-label*='New chat']",
    "a[aria-label*='Neuer Chat']",
    "button[aria-label*='Neuer Chat']",
]
PROMPT_SELECTORS = [
    "rich-textarea div[contenteditable='true']",
    "div[contenteditable='true'][role='textbox']",
    "textarea",
]
SEND_SELECTORS = [
    "button[aria-label*='Send']",
    "button[aria-label*='Senden']",
    "button[data-testid='send-button']",
]
RESPONSE_SELECTORS = [
    "message-content",
    "[data-response-index]",
    ".model-response-text",
]
STREAMING_SELECTORS = [
    "button[aria-label*='Stop generating']",
    "button[aria-label*='Antwort stoppen']",
    "mat-progress-spinner",
]
LOGIN_HINT_SELECTORS = [
    "a[href*='accounts.google.com']",
    "text=Sign in",
    "text=Anmelden",
]
RATE_LIMIT_MARKERS = (
    "rate limit",
    "too many requests",
    "quota",
    "try again later",
    "später erneut",
    "zu viele anfragen",
)
REFUSAL_MARKERS = (
    "i can't assist",
    "i can’t assist",
    "i cannot assist",
    "dabei kann ich nicht helfen",
)


class WorkerFailure(Exception):
    def __init__(self, kind: str, message: str) -> None:
        super().__init__(message)
        self.kind = kind
        self.message = message


def open_brave(playwright, *, headless: bool):
    """Credentials only from the account-bound encrypted database state.

    An incognito context has no persistent cookie/profile directory. Existing
    profile data is never read as a fallback or removed by this worker.
    """
    envelope = db_browser_session.load()
    browser = playwright.chromium.launch(
        headless=headless,
        executable_path=BRAVE_PATH,
        args=["--no-first-run", "--no-default-browser-check"],
    )
    try:
        context = browser.new_context(storage_state=envelope["state"])
        return db_browser_session.DbBrowserContext(
            browser, context, envelope["revision"]
        )
    except Exception:
        browser.close()
        raise


def main() -> int:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("login")
    analyze_parser = subparsers.add_parser("analyze")
    analyze_parser.add_argument("--url", required=True)
    subparsers.add_parser("analyze-text")
    args = parser.parse_args()
    if args.command == "login":
        return login()
    if args.command == "analyze":
        return analyze(args.url)
    return analyze_text()


def login() -> int:
    try:
        from playwright.sync_api import sync_playwright

        with sync_playwright() as playwright:
            context = open_brave(playwright, headless=False)
            page = context.pages[0] if context.pages else context.new_page()
            page.goto(GEMINI_URL, wait_until="domcontentloaded", timeout=NAV_TIMEOUT_MS)
            input(
                "Im Browser bei deinem Gemini-Konto einloggen, danach hier Enter drücken, um die Sitzung zu speichern … "
            )
            context.save()
            context.close()
        return 0
    except Exception:  # noqa: BLE001
        print(
            "Login oder verschlüsselte DB-Speicherung fehlgeschlagen. Konto und Sitzungsdienst prüfen.",
            file=sys.stderr,
        )
        return 1


def analyze(url: str) -> int:
    try:
        payload = read_payload()
        prompt = payload_prompt(payload)
        if not prompt:
            raise WorkerFailure("unknown", "missing prompt")
        if url not in prompt:
            prompt = f"{prompt}\n\n{url}"
        emit({"status": "ok", "text": run_browser_analysis(prompt)})
        return 0
    except WorkerFailure as exc:
        emit({"status": "error", "kind": exc.kind, "message": exc.message})
        return 1
    except Exception:  # noqa: BLE001
        emit(
            {
                "status": "error",
                "kind": "unknown",
                "message": "Browser- oder DB-Sitzungsfehler. Keine Zugangsdaten in Diagnoseausgaben.",
            }
        )
        return 1


def analyze_text() -> int:
    try:
        payload = read_payload()
        prompt = payload_prompt(payload)
        convert_prompt = str(
            payload.get("convert_prompt") or JSON_CONVERT_PROMPT
        ).strip()
        if not prompt:
            raise WorkerFailure("unknown", "missing prompt")
        if not convert_prompt:
            raise WorkerFailure("unknown", "missing convert_prompt")
        emit({"status": "ok", "text": run_browser_two_turn(prompt, convert_prompt)})
        return 0
    except WorkerFailure as exc:
        emit({"status": "error", "kind": exc.kind, "message": exc.message})
        return 1
    except Exception:  # noqa: BLE001
        emit(
            {
                "status": "error",
                "kind": "unknown",
                "message": "Browser- oder DB-Sitzungsfehler. Keine Zugangsdaten in Diagnoseausgaben.",
            }
        )
        return 1


def read_payload() -> dict:
    return json.loads(sys.stdin.read() or "{}")


def payload_prompt(payload: dict) -> str:
    return str(payload.get("prompt") or "").strip()


def run_browser_analysis(prompt: str) -> str:
    return run_browser_two_turn(prompt, JSON_CONVERT_PROMPT)


def run_browser_two_turn(prompt: str, convert_prompt: str) -> str:
    from playwright.sync_api import TimeoutError as PlaywrightTimeoutError
    from playwright.sync_api import sync_playwright

    with sync_playwright() as playwright:
        # Analyse bleibt headful; der unbeaufsichtigte Cron stellt per xvfb-run
        # nur ein virtuelles Display bereit. Headless triggert Geminis Erkennung.
        context = open_brave(playwright, headless=False)
        page = context.pages[0] if context.pages else context.new_page()
        try:
            page.goto(GEMINI_URL, wait_until="domcontentloaded", timeout=NAV_TIMEOUT_MS)
            # Seite settlen lassen, damit Gemini bei Video-Prompts die YouTube-URL
            # als Video erkennt.
            page.wait_for_timeout(6_000)
            if visible(page, LOGIN_HINT_SELECTORS, 500):
                raise WorkerFailure("not_logged_in", "login required")
            prompt_box = first(page, PROMPT_SELECTORS, 15_000)
            if prompt_box is None:
                kind = (
                    "not_logged_in"
                    if visible(page, LOGIN_HINT_SELECTORS, 500)
                    else "unknown"
                )
                raise WorkerFailure(kind, "prompt box not found")
            # Turn 1: natürliche Analyse (KEIN JSON). Bei Video-Prompts lässt das
            # Gemini das Video-Tool benutzen; starres JSON blockiert dort.
            fill_prompt(page, prompt_box, prompt)
            # Gemini Zeit geben, bei Video-Prompts die YouTube-URL als Karte anzuhängen.
            page.wait_for_timeout(9_000)
            before = response_count(page)
            page.keyboard.press("Enter")
            wait_for_response(page, before)
            # Turn 2: Prosa -> striktes JSON (reines Text->Text, kein Stopp-Mechanismus).
            box2 = first(page, PROMPT_SELECTORS, 15_000)
            if box2 is None:
                raise WorkerFailure("unknown", "prompt box not found for json turn")
            fill_prompt(page, box2, convert_prompt)
            before2 = response_count(page)
            page.keyboard.press("Enter")
            result = wait_for_response(page, before2)
            context.save()
            return result
        except PlaywrightTimeoutError as exc:
            raise WorkerFailure("timeout", str(exc)) from exc
        finally:
            context.close()


def first(page, selectors: list[str], timeout: int):
    for selector in selectors:
        locator = page.locator(selector).first
        try:
            locator.wait_for(state="visible", timeout=timeout)
            return locator
        except Exception:  # noqa: BLE001
            LOGGER.debug("Browser-Element noch nicht verfügbar; nächster Selektor.")
            continue
    return None


def click_first(page, selectors: list[str], timeout: int) -> bool:
    locator = first(page, selectors, timeout)
    if locator is None:
        return False
    pace()
    try:
        locator.click(timeout=timeout)
        pace()
        return True
    except Exception:  # noqa: BLE001
        return False


def fill_prompt(page, locator, prompt: str) -> None:
    # insert_text statt type/fill: fügt den Text wie ein Paste in einem Rutsch ein,
    # sodass die Zeilenumbrüche im Prompt NICHT als Enter (= vorzeitiges Absenden)
    # wirken. Genau das war die Ursache für die früheren leeren Antworten.
    locator.click()
    pace()
    page.keyboard.insert_text(prompt)
    pace()


def response_count(page) -> int:
    try:
        return page.locator("message-content").count()
    except Exception:  # noqa: BLE001
        return 0


def wait_for_response(page, min_responses: int = 0) -> str:
    deadline = time.monotonic() + RESPONSE_TIMEOUT_SECONDS
    last_text = ""
    stable_since = time.monotonic()
    while time.monotonic() < deadline:
        if any(marker in body_text(page).lower() for marker in RATE_LIMIT_MARKERS):
            raise WorkerFailure("rate_limited", "rate limit marker detected")
        text = (
            latest_response_text(page) if response_count(page) > min_responses else ""
        )
        if text and text != last_text:
            last_text = text
            stable_since = time.monotonic()
        if (
            last_text
            and time.monotonic() - stable_since >= 4
            and not visible(page, STREAMING_SELECTORS, 250)
        ):
            if any(marker in last_text.lower() for marker in REFUSAL_MARKERS):
                raise WorkerFailure("refused", "refusal marker detected")
            return last_text
        time.sleep(0.8 + random.random() * 0.6)
    raise WorkerFailure("timeout", "response timeout")


def latest_response_text(page) -> str:
    for selector in RESPONSE_SELECTORS:
        try:
            locator = page.locator(selector)
            for index in range(locator.count() - 1, -1, -1):
                text = locator.nth(index).inner_text(timeout=1_000).strip()
                if text:
                    return text
        except Exception:  # noqa: BLE001
            LOGGER.debug("Browser-Element noch nicht verfügbar; nächster Selektor.")
            continue
    return ""


def visible(page, selectors: list[str], timeout: int) -> bool:
    for selector in selectors:
        try:
            if page.locator(selector).first.is_visible(timeout=timeout):
                return True
        except Exception:  # noqa: BLE001
            LOGGER.debug("Browser-Element noch nicht verfügbar; nächster Selektor.")
            continue
    return False


def body_text(page) -> str:
    try:
        return page.locator("body").inner_text(timeout=1_000)
    except Exception:  # noqa: BLE001
        return ""


def pace() -> None:
    time.sleep(0.25 + random.random() * 0.55)


def emit(payload: dict) -> None:
    print(json.dumps(payload, ensure_ascii=False), flush=True)


if __name__ == "__main__":
    raise SystemExit(main())
