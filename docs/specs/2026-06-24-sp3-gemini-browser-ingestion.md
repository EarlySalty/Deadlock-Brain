# SP3 — Gemini-Browser-Ingestion-Loop (Implementierungs-Ticket)

**Datum:** 2026-06-24
**Status:** Freigegeben zur Umsetzung (Delegation an Codex, Review durch Claude)
**Repo:** Deadlock-Brain — Arbeit im Worktree-Branch `feat/sp3-gemini-ingestion`

## 1. Ziel

Aus den kuratierten YouTube-Quellen (`config/youtube_feeds.json`: Playlist
`PLRaixQ0u4jjmv-q9g1XUgO3G2ppTaSWpS` + 13 Kanäle) frische Videos automatisch
ziehen, jedes Video von einer **eingeloggten Gemini-Abo-Session im Browser
nativ analysieren** lassen (Gemini bekommt die YouTube-URL und schaut das Video
selbst — Audio + Bild), strukturierte Deadlock-Wissens-Claims extrahieren, in
der bestehenden SQLite-DB speichern und per CLI abfragbar machen. Voll
automatisch, fail-soft, idempotent.

## 2. Gesetzte Entscheidungen (NICHT neu verhandeln)

- **Zugang zu Gemini:** Browser-Automation der eingeloggten Abo-Session des
  Users (Playwright, persistentes Profil). **Kein API-Key, kein Gemini-API-Call.**
- **Login-Seeding:** Das Profil `data/gemini_profile/` wird einmalig in einem
  normalen Brave eingeloggt. Google blockt automatisierten Login; Playwright
  nutzt danach nur die gespeicherte Session.
- **Video-Ingestion:** Gemini bekommt die **URL** und analysiert nativ.
  **KEIN Fallback** (kein yt-dlp-Transkript, keine zweite Engine). Schlägt ein
  Video fehl → als `failed` markieren, nächster Lauf nimmt es erneut.
- **Bewiesener Gemini-Ablauf:** 2-Turn. Turn 1 ist natürliche Prosa-Analyse
  (Gemini schaut das Video). Turn 2 wandelt diese Prosa in JSON um
  (reines Text→Text). Direkte strikte JSON-Anforderung in Turn 1 blockiert die
  Video-Analyse.
- **Anti-Detection:** Analyse läuft headful, auch im Cron. Unbeaufsichtigt wird
  dafür `xvfb-run -a` verwendet. Headless ist von Gemini erkennbar und darf für
  die produktive Analyse nicht verwendet werden. Zusätzlich bleiben
  `ignore_default_args=["--enable-automation"]`,
  `--disable-blink-features=AutomationControlled` und der Stealth-Init-Script
  aktiv.
- **Sprachen-/Architektur-Split (Hybrid):**
  - **Rust-Kern** (neues `rust/`-Crate im Repo): gesamte neue Logik — Queue,
    DB-Zugriff, Claim-Parsing, CLI, Loop, Fehlerklassen, Orchestrierung.
  - **Dünner Python-Playwright-Worker** (~150 Zeilen, eine Aufgabe): nur
    „öffne Gemini, schicke URL+Prompt, gib Antworttext zurück".
  - **Hybrid-Naht = gemeinsame SQLite-DB** `data/deadlock_brain.sqlite3`.
    Rust und das bestehende Python-Brain lesen/schreiben dieselbe DB.
- **Quellen-Reuse:** Videoentdeckung bleibt RSS-basiert wie heute; nicht neu
  bauen (aktive YT-Suche ist eine spätere SP).
- **Trust/Gewichtung verschoben:** Channel-Reputation, Pyramiden-Trust-Engine,
  aktive Quellen-Suche sind **NICHT** Teil von SP3 (spätere SPs).

## 3. Repo-Layout (neu anzulegen)

```
rust/
  Cargo.toml                # eigenständiges Crate "deadlock-brain-yt" (workspace optional)
  src/
    main.rs                 # CLI-Einstieg (subcommands)
    db.rs                   # SQLite-Zugriff (rusqlite, bundled feature)
    schema.rs               # ensure-schema: CREATE TABLE IF NOT EXISTS, identisch zu Python
    queue.rs                # Video-Auswahl + Status-Übergänge
    claims.rs               # Claim-Datentypen + JSON-Parsing + Speichern
    gemini.rs               # Aufruf des Python-Browser-Workers (Subprozess) + Fehlerklassen
    loop_runner.rs          # Orchestrierung eines Laufs (fail-soft, idempotent)
  python_worker/
    gemini_browser.py       # Playwright-Worker (persistentes Profil)
    requirements.txt        # playwright
scripts/
  run_gemini_ingest.sh      # Cron-tauglicher Lauf via xvfb-run -a, ohne Infisical
```

Das bestehende Python-Paket (`src/deadlock_brain/`) bleibt **unverändert**.

## 4a. Persistenz-Policy — ALLES in die DB (HART)

**Jegliche inhaltliche Persistenz geht ausschließlich in die SQLite-DB**
(`data/deadlock_brain.sqlite3`): Claims, rohe Modellantworten, Prompts,
Provider-/Verifier-Metadaten, Video-/Feed-Metadaten, Lauf-Ergebnisse. **Kein
Ablegen von Daten in losen Dateien** (keine `.json`/`.txt`/`.ndjson`-Dumps unter
`data/raw`, `data/cache`, kein Scratch-File). Der Python-Worker gibt seine
Antwort **nur** auf stdout zurück und schreibt sie NICHT auf Platte; Rust
persistiert sie als DB-Spalte (`model_response_text`). Das Cron-Script dumpt
nichts, es ruft nur das Binary.

**Einzige erlaubte Nicht-DB-Datei:** das Chromium-Login-Profil unter
`data/gemini_profile/` (Playwright-Session-Zustand, technisch nicht
DB-fähig) — gitignored. Sonst nichts.

## 4. DB — Schema-Fidelity ist Pflicht

Rust schreibt in **dieselben** Tabellen, die Python in
`src/deadlock_brain/youtube_learning.py` → `ensure_youtube_tables()` definiert:
`youtube_feed_sources`, `youtube_videos`, `youtube_transcripts`,
`youtube_learning_claims`.

- **`schema.rs` muss die `CREATE TABLE IF NOT EXISTS`-Statements 1:1 aus
  `ensure_youtube_tables()` spiegeln** (gleiche Spalten, gleiche NOT-NULL,
  gleiche Defaults). Vor dem Schreiben dieser Datei die Python-Funktion lesen
  und exakt übernehmen. Keine eigene Schema-Variante erfinden.
- Neue Spalte nur falls zwingend nötig, dann additiv via
  `ALTER TABLE … ADD COLUMN` mit Existenz-Guard. Bevorzugt **keine** neue Spalte:
  `source_channel` kann aus dem Video-Join (`youtube_videos.channel_title`)
  abgeleitet werden.
- Claims tragen `model = "gemini-web"` und `prompt_version =
  "youtube_claims_de_v2"`. Das Dedupe-Gate (Video bereits für
  prompt_version+model verarbeitet → überspringen) muss erhalten bleiben →
  idempotenter Loop.

## 5. Browser-Worker-Vertrag (`gemini_browser.py`)

- **Persistentes Profil:** Playwright `launch_persistent_context` mit fester
  `user_data_dir` (z.B. `data/gemini_profile/`, in `.gitignore`).
- **Login-Setup:** Das dedizierte Profil wird einmalig mit normalem Brave
  gestartet, z.B. mit `--user-data-dir=data/gemini_profile` und
  `--password-store=basic`, und dort manuell in Gemini eingeloggt. Der
  Playwright-Login-Modus bleibt nur als manuelles Hilfswerkzeug; automatisierter
  Google-Login ist nicht zuverlässig.
- **Analyse-Modus:** Aufruf
  `python gemini_browser.py analyze --url <YT_URL>` (Prompt kommt über stdin als
  JSON, um Quoting-Probleme zu vermeiden). Ablauf: gemini.google.com öffnen →
  Prompt per `insert_text` einfügen → Turn 1 senden und natürliche Analyse
  abwarten → Turn 2 mit JSON-Konvertierung senden → Antworttext aus dem DOM
  lesen.
- **Textmodus für Scaffolds:** `python gemini_browser.py analyze-text` nutzt
  denselben 2-Turn-Browserpfad ohne Video-URL. Der Modus ist nicht Teil der
  YouTube-Kernpipeline und dient vorerst nur als Patchnotes-Scaffold.
- **Ausgabe:** **nur** eine JSON-Zeile auf stdout:
  `{"status":"ok","text":"<rohe Modellantwort>"}` oder
  `{"status":"error","kind":"not_logged_in|rate_limited|refused|timeout|unknown","message":"…"}`.
  Exit-Code 0 bei `ok`, ≠0 bei `error`.
- **Selektoren konfigurierbar** (Konstanten-Block oben in der Datei), weil das
  Gemini-DOM die einzige instabile Stelle ist. Menschenähnliches Pacing
  (randomisierte kurze Delays), genau ein Video pro Aufruf.
- **Headful Pflicht:** Der Worker setzt Analyse-Läufe auf `headless=False`.
  Unbeaufsichtigte Läufe müssen deshalb unter `xvfb-run -a` starten.

## 6. Gemini-Prompts (bewiesener 2-Turn-Ablauf)

Turn 1 ist der natürliche Analyse-Prompt aus `rust/src/claims.rs`. Dieser Text
fordert **kein** JSON an, damit Gemini die YouTube-URL als Video verarbeitet:

```
Schau dir dieses YouTube-Video von einem Deadlock-Creator genau an: {URL}

Fasse anschließend zusammen, welche konkreten Gameplay-Erkenntnisse der Creator vermittelt – zum Beispiel Item-Builds, Item-Timings, Hero-Matchups, Mechaniken, Combos oder Meta-Einschätzungen. Ignoriere Smalltalk, Intros, Werbung und Spendenaufrufe. Antworte auf Deutsch in klaren Stichpunkten.
```

Turn 2 ist der JSON-Konvertierungsprompt aus
`rust/python_worker/gemini_browser.py`. Er läuft als reines Text→Text auf Basis
der Antwort aus Turn 1:

```
Wandle die oben genannten Erkenntnisse in genau dieses JSON um. Gib NUR das JSON aus, ohne Markdown, ohne Codeblock, ohne weiteren Text:
{"claims": [{"entity": "Hero, Item oder Fähigkeit", "claim_type": "build|item_timing|matchup|mechanic|combo|meta", "assertion": "klarer deutscher Satz", "patch_context": null, "confidence": 0.0}]}
Enthält die Analyse kein konkretes Deadlock-Gameplay-Wissen, gib {"claims": []} zurück.
```

Wichtig: Prompt-Eingabe im Browser erfolgt per `keyboard.insert_text`, nicht per
`type`, damit Newlines nicht als vorzeitiges Absenden interpretiert werden.

## 7. Claim-JSON (Schema v2)

Pro Claim aus dem Modell: `entity`, `claim_type`, `assertion`, `patch_context`,
`confidence`. `video_id`, `source_channel`, `published_at` stammen aus der
DB-Zeile (nicht vom Modell). Robustes Parsing: JSON auch aus versehentlichem
Markdown-Block extrahieren (Logik aus Python `_parse_model_claims` als Vorlage).
Ungültige/leere Antworten → 0 Claims, Video-Status entsprechend setzen, kein
Crash.

## 8. Fehlerbehandlung / fail-soft / Idempotenz

- Worker-`error/kind=not_logged_in` oder `rate_limited` → **Lauf sauber
  pausieren**: aktuelles Video NICHT als failed verbrennen, ERROR loggen,
  optional konfigurierbaren Discord-Webhook (Env `GEMINI_INGEST_ALERT_WEBHOOK`,
  no-op wenn leer) pingen, Prozess mit ≠0 beenden (Journal sichtbar).
- Worker-`error/kind=refused|timeout|unknown` → Video `learning_status='failed'`
  + Fehlernotiz, weiter zum nächsten Video.
- Erfolg mit 0 Claims → `learning_status='no_claims'` (Video gilt als erledigt).
- Erfolg mit ≥1 Claim → speichern, `learning_status='claims_ready'`.
- Jeder Lauf verarbeitet ein konfigurierbares kleines Batch (`--limit`, Default 5).

## 9. Konsum-Haken (minimal, gegen die Sackgasse)

- Neues Rust-Subcommand `deadlock-brain-yt claims <entity> [--pretty]`: zeigt
  gespeicherte Claims zu einem Hero/Item (Join `youtube_learning_claims` ×
  `youtube_videos`, sortiert nach `published_at` desc), inkl. `assertion`,
  `claim_type`, `source_channel`, `confidence`.
- **Bewusst NICHT in SP3:** Verdrahtung der Claims in die Python-Antworten
  (`review_context`/`brain_pipeline`). Das gehört in SP1 (Pyramide) und wird
  hier nur als bekannte Lücke dokumentiert.

## 10. CLI-Oberfläche (Rust)

```
deadlock-brain-yt gemini-login        # ruft Python-Worker login (headful)
deadlock-brain-yt ingest [--limit N]  # ein Lauf: Auswahl → Analyse → Speichern
deadlock-brain-yt claims <entity> [--pretty]
deadlock-brain-yt smoke --url <URL>   # 1 Video end-to-end, gibt rohe + geparste Claims aus
```

`scripts/run_gemini_ingest.sh` kapselt `ingest` für Cron (täglich, kleines
Batch) via `xvfb-run -a`. Der Gemini-Browserpfad benötigt keine Infisical-Secrets.

## 11. Definition of Done

1. `cargo build --release` im `rust/`-Crate fehlerfrei; `cargo clippy` ohne
   Warnungen in neuem Code; **kein `cargo fmt`-Reformat bestehender Dateien**.
2. Unit-Tests (Rust) für: Claim-JSON-Parsing (inkl. Markdown-umhüllt + Müll),
   Status-Übergänge der Queue, Schema-Erzeugung gegen eine Wegwerf-SQLite.
3. Das dedizierte Profil ist einmalig in normalem Brave eingeloggt; Analyse läuft
   headful und kann unbeaufsichtigt unter `xvfb-run -a` starten.
4. `smoke --url <reales Deadlock-Video>` liefert geparste Claims ODER eine
   saubere Fehlerklasse — **kein Crash, kein Hänger**.
5. `ingest --limit 2` schreibt Claims in die DB; erneuter Lauf ist idempotent
   (keine Duplikate, bereits verarbeitete Videos übersprungen).
6. `claims "Mo & Krill"` zeigt die gespeicherten Claims lesbar an.
7. Python-Paket unverändert, bestehende Python-Smoke-Checks weiter grün.

## 12. Bekanntes Risiko (ehrlich)

Die **DOM-Selektoren von gemini.google.com** sind die einzige Stelle, die nicht
blind final geschrieben werden kann — sie brauchen eine Verifikation an der
echten, eingeloggten Seite. Codex baut sie best-effort + konfigurierbar +
liefert `gemini-login`/`smoke`, sodass die finale Selektor-Justierung in einem
kurzen Live-Lauf (durch Claude/User) erfolgt, statt blind. Alles andere
(Rust-Kern, DB, Parsing, CLI, Loop) ist deterministisch testbar und blockiert
nicht auf diese Verifikation.

## 13. Spätere SPs (nur Kontext, nicht jetzt bauen)

- **SP1:** Wissens-Pyramide (L0 Patchnotes/Assets → L1 Stats → L2 Claims),
  Trust-Gewicht, Verdrahtung der Claims in die Brain-Antworten.
- **SP2:** Autonome Trust-Engine (Cross-Check gegen L0/L1, Mehrfach-Quellen).
- **SP4:** Aktive Quellen-Suche (YouTube Data API mit `YOUTUBE_CLIENT_*`) +
  Channel-Reputation-Gewichtung + Cron-Vollautomatik.
