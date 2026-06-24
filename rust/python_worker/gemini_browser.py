#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
import random
import sys
import time
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
# Dediziertes Profil für den Wegwerf-Gemini-Account (NICHT das tägliche Brave-Profil).
# Einmalig in einem normalen Brave mit diesem Profil einloggen; Google blockt den
# automatisierten Login zuverlässig. Der unbeaufsichtigte Lauf bleibt headful und
# nutzt xvfb-run als virtuelles Display.
DEFAULT_PROFILE_DIR = REPO_ROOT / "data/gemini_profile"
PROFILE_DIR = Path(os.environ.get("GEMINI_PROFILE_DIR", str(DEFAULT_PROFILE_DIR)))
BRAVE_PATH = os.environ.get("GEMINI_BROWSER_PATH", "/opt/brave.com/brave/brave")
GEMINI_URL = "https://gemini.google.com/app"
NAV_TIMEOUT_MS = 45_000
RESPONSE_TIMEOUT_SECONDS = int(os.environ.get("GEMINI_RESPONSE_TIMEOUT_SECONDS", "300"))

# Tarnung: Automatisierungs-Signale verstecken, sonst degradiert Gemini das
# Video-Feature (behandelt die YouTube-URL nur als Text).
STEALTH_SCRIPT = """
Object.defineProperty(navigator, 'webdriver', {get: () => undefined});
window.chrome = window.chrome || { runtime: {} };
Object.defineProperty(navigator, 'plugins', {get: () => [1,2,3,4,5]});
Object.defineProperty(navigator, 'languages', {get: () => ['de-DE','de','en-US','en']});
"""

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
NEW_CHAT_SELECTORS = ["a[aria-label*='New chat']", "button[aria-label*='New chat']", "a[aria-label*='Neuer Chat']", "button[aria-label*='Neuer Chat']"]
PROMPT_SELECTORS = ["rich-textarea div[contenteditable='true']", "div[contenteditable='true'][role='textbox']", "textarea"]
SEND_SELECTORS = ["button[aria-label*='Send']", "button[aria-label*='Senden']", "button[data-testid='send-button']"]
RESPONSE_SELECTORS = ["message-content", "[data-response-index]", ".model-response-text"]
STREAMING_SELECTORS = ["button[aria-label*='Stop generating']", "button[aria-label*='Antwort stoppen']", "mat-progress-spinner"]
LOGIN_HINT_SELECTORS = ["a[href*='accounts.google.com']", "text=Sign in", "text=Anmelden"]
RATE_LIMIT_MARKERS = ("rate limit", "too many requests", "quota", "try again later", "später erneut", "zu viele anfragen")
REFUSAL_MARKERS = ("i can't assist", "i can’t assist", "i cannot assist", "dabei kann ich nicht helfen")


class WorkerFailure(Exception):
    def __init__(self, kind: str, message: str) -> None:
        super().__init__(message)
        self.kind = kind
        self.message = message


def open_brave(playwright, *, headless: bool):
    """Startet das echte Brave-Profil mit dem Brave-Binary.

    Brave muss geschlossen sein – andernfalls ist das Profil gesperrt
    (Chromium-SingletonLock) und der Lauf pausiert sauber statt abzustürzen.
    """
    if (PROFILE_DIR / "SingletonLock").exists():
        raise WorkerFailure(
            "profile_locked",
            "Brave läuft bereits mit diesem Profil. Bitte Brave vollständig schließen und erneut starten.",
        )
    context = playwright.chromium.launch_persistent_context(
        user_data_dir=str(PROFILE_DIR),
        headless=headless,
        executable_path=BRAVE_PATH,
        # --password-store=basic: Cookies keyring-unabhängig verschlüsseln, damit die
        # einmal eingeloggte Session auch im xvfb-Cron lesbar bleibt.
        # --disable-blink-features=AutomationControlled + ignore_default_args: setzt
        # navigator.webdriver auf false und entfernt die Automatisierungs-Infobar.
        args=["--password-store=basic", "--no-first-run", "--no-default-browser-check",
              "--disable-blink-features=AutomationControlled"],
        ignore_default_args=["--enable-automation"],
    )
    context.add_init_script(STEALTH_SCRIPT)
    return context


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

        PROFILE_DIR.mkdir(parents=True, exist_ok=True)
        with sync_playwright() as playwright:
            context = open_brave(playwright, headless=False)
            page = context.pages[0] if context.pages else context.new_page()
            page.goto(GEMINI_URL, wait_until="domcontentloaded", timeout=NAV_TIMEOUT_MS)
            input("Im Browser bei deinem Gemini-Konto einloggen, danach hier Enter drücken, um die Sitzung zu speichern … ")
            context.close()
        return 0
    except Exception as exc:  # noqa: BLE001
        print(f"Login fehlgeschlagen: {exc}", file=sys.stderr)
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
    except Exception as exc:  # noqa: BLE001
        emit({"status": "error", "kind": "unknown", "message": str(exc)})
        return 1


def analyze_text() -> int:
    try:
        payload = read_payload()
        prompt = payload_prompt(payload)
        convert_prompt = str(payload.get("convert_prompt") or JSON_CONVERT_PROMPT).strip()
        if not prompt:
            raise WorkerFailure("unknown", "missing prompt")
        if not convert_prompt:
            raise WorkerFailure("unknown", "missing convert_prompt")
        emit({"status": "ok", "text": run_browser_two_turn(prompt, convert_prompt)})
        return 0
    except WorkerFailure as exc:
        emit({"status": "error", "kind": exc.kind, "message": exc.message})
        return 1
    except Exception as exc:  # noqa: BLE001
        emit({"status": "error", "kind": "unknown", "message": str(exc)})
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

    PROFILE_DIR.mkdir(parents=True, exist_ok=True)
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
                kind = "not_logged_in" if visible(page, LOGIN_HINT_SELECTORS, 500) else "unknown"
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
            return wait_for_response(page, before2)
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
            pass
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
        text = latest_response_text(page) if response_count(page) > min_responses else ""
        if text and text != last_text:
            last_text = text
            stable_since = time.monotonic()
        if last_text and time.monotonic() - stable_since >= 4 and not visible(page, STREAMING_SELECTORS, 250):
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
            pass
    return ""


def visible(page, selectors: list[str], timeout: int) -> bool:
    for selector in selectors:
        try:
            if page.locator(selector).first.is_visible(timeout=timeout):
                return True
        except Exception:  # noqa: BLE001
            pass
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
