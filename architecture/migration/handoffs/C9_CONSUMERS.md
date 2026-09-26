# C9 Consumer Wiring

Stand: 2026-09-26

C9 verdrahtet die Consumer gegen den aktuellen typisierten `BrainClient` / `brain-serve`-Vertrag aus `migration/rust-integration`.

Aktuelle Vertragsbasis:

- `origin/migration/rust-integration`: `3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef`
- enthält insbesondere `AnswerStatus::Unavailable` und `AnswerStatus::BuildRejected`
- PR #50 wurde auf diesen Stand neu aufgebaut; die früher mitgeschleppten C2/C3-/C6-Dependency-Merge-Commits sind nicht mehr Bestandteil des C9-Diffs

Es wurden keine Bots gestartet, keine Nachrichten gesendet und keine Deployments ausgeführt. Legacy-Pfade bleiben verfügbar; Typed-/Shadow-Nutzung erfolgt nur über explizite Konfiguration.

| Repo | Branch | Commit | PR | Verifikation |
| --- | --- | --- | --- | --- |
| EarlySalty/Deadlock-Twitch-Bot | `codex/fix-c9-consumer-wiring` | `8e6ee4c7c35172bafa5e5f47799d2aea72cdb1ab` | #984 | Brain-Offline `knowledge`: fmt/test/clippy grün; `self-explainer`: fmt/test grün; `tb-knowledge` komplett grün (23 Unit + 3 Load + 5 Seed); Merge-Policy-Tests 4/4 grün. |
| EarlySalty/Deadlock-Bots | `codex/fix-c9-consumer-wiring` | `e6ec9925dcd462c4204c269dd0a82a6fb7beb716` | #459 | `rust/scripts/check-brain-consumer.sh`: fmt/test/clippy grün; `dl-brain` 12/12; Mode- und blockierende-Shadow-Regressions grün; vollständiges `dl-bot` clippy `-D warnings` grün. |
| EarlySalty/Deadlock-Docs | `codex/fix-c9-consumer-wiring` | `ba4143f8ce30621d993574ccd4d3e2c66b46f5a7` | #4 | `tools/brain-adapter/check.sh`: fmt/test/clippy grün, locked/offline. |
| EarlySalty/Deadlock-2nd-Brain | `codex/fix-c9-consumer-wiring` | `46aa2d37e4564e9d314847f52320016c10f6c47e` | #2 | `tools/brain-adapter/check.sh`: fmt/test/clippy grün, locked/offline. GitHub-Job 36217479678 scheiterte vor jedem Step: `runner_id=0`, leerer Runnername, `steps=[]`; kein belegter Repo-Codefehler. |
| EarlySalty/Deadlock-Brain CLI/MCP | `codex/fix-c9-consumer-wiring` | `7fd5ee13da60d6035beda49fec8f2c0f1545542c` | #50 | `deadlock-brain --all-targets`: 58 Tests grün; `clippy -D warnings` grün. MCP testet `build_rejected` als Domain-Ergebnis und `unavailable` als Error-Result; CLI testet die unveränderte JSON-Statusprojektion beider Wire-Statuswerte. |

## Verdrahtung und Review-Befunde

### Deadlock-Bots

Der echte `brain`-Command-Composition-Root unterstützt ausschließlich:

- `legacy`
- `typed`
- `shadow`

Nicht gesetzter Modus behält den dokumentierten Legacy-Default. Ein gesetzter unbekannter oder leerer `BRAIN_CLIENT_MODE` ist ein Startup-/Konfigurationsfehler und fällt nicht still auf Legacy zurück.

`BrainApiAnswerer` verwendet einen collision-resistenten per-instance Namespace: zum vertrauenswürdigen lokalen Namespace kommt pro Adapterkonstruktion ein zufälliger 128-Bit-Wert, danach erst der lokale Sequenzzähler. Zwei Adapterinstanzen mit gleicher PID bzw. zwei Restarts erzeugen dadurch keine gleichen Request-/Conversation-IDs.

Shadow ist report-only:

- sichtbarer Legacy-Pfad antwortet unabhängig
- Typed-Probe läuft in einem separaten Task
- Typed-Probe besitzt eine eigene harte Timeout-Grenze
- blockierendes oder fehlschlagendes Typed-Backend verzögert bzw. verändert die sichtbare Legacyantwort nicht

Scopes, Channel-Allowlist, Cooldowns, Fragenlänge, Auth und bestehendes Discord-Ausgabeformat bleiben im bisherigen Pfad.

### Twitch

Die echte öffentliche Self-Explainer-Route verwendet den neuen Typed-Brain-Port in `typed` bzw. report-only in `shadow`. Conversation-Historie wird im neuen Query mitgeführt und nicht still auf dem Legacy-Brain-Pfad belassen.

Der zuvor bekannte Basisfehler
`stoerung_stream_info_felder_findet_uplink_stoerungen`
wurde unverändert reproduziert und minimal repariert: `uplink-stoerungen.md` besitzt jetzt die vom bestehenden lexikalischen Selektor vorgesehenen `tip_flags` für Uplink-/Störungs-/OBS-/Stream-Info-Begriffe. Der Test wurde nicht abgeschwächt.

Die Merge-Policy wurde entsprechend dem vorhandenen Vertrauensmodell verschärft:

- `.github/workflows/pr-release-gate.yml` verlangt beide Typed-Brain-Fixture-Checks
- `.github/workflows/dependabot-auto-merge.yml` verlangt dieselben Checks als von GitHub Actions serverseitig erzwungene Required Checks
- Repository-Ruleset `14377032` auf `main` ist aktiv und enthält `required_status_checks` mit `strict_required_status_checks_policy=true`
- zusätzlich zu den bisherigen sechs Release-Checks sind serverseitig erforderlich:
  - `Typed Brain fixtures (knowledge)`
  - `Typed Brain fixtures (self-explainer)`
- `integration_id=15368` bindet diese Required Checks an GitHub Actions
- kein Bypass-Actor ist konfiguriert

Damit kann Native/Dependabot-Auto-Merge nicht aktiviert bzw. durchgeführt werden, wenn die Typed-Brain-Fixtures fehlen oder fehlschlagen.

### Docs

`tools/brain-adapter query ...` verwendet ausschließlich den typisierten BrainClient.

- Scope fest `docs.public`
- Bearer-Token nicht als CLI-Argument
- Loopback-only Endpoint und Client-Timeouts
- kein lokales Modell/RAG, kein Corpus-Publishing
- BrainClient-Pin auf `3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef`

### 2nd-Brain

Der interne Query-Pfad verwendet ausschließlich den typisierten BrainClient.

- interne Scopes explizit und vertrauenswürdig gebunden
- Public-/Wildcard-Bindungen lokal abgewiesen
- kein Corpus-Export, Publishing, lokales RAG oder Modell-Fallback
- BrainClient-Pin auf `3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef`

Der GitHub-Actions-Fehler von PR #2 ist von der Codeverifikation getrennt zu behandeln: Run `36217479678` erhielt keinen Hosted Runner (`runner_id=0`) und führte keinen Step aus (`steps=[]`). Derselbe Checkout ist lokal mit Rust 1.97.1, locked/offline, für fmt/test/clippy grün.

### Brain CLI/MCP

- `deadlock-brain answer` erzeugt einen typisierten `Query` und verwendet `AsyncBrainClient::new_local`
- Token ausschließlich aus `BRAIN_CLIENT_TOKEN`
- Scopes explizit
- `brain-mcp` bindet Scopes nur aus Runtime-Konfiguration
- MCP: `build_rejected` bleibt erfolgreich transportiertes Domain-Ergebnis; `unavailable`, ACL-, Provider- und Budgetfehler sind Error-Results
- CLI: `build_rejected` und `unavailable` bleiben im öffentlichen JSON-Wireformat unverändert erhalten

## Sicherheits-/Betriebsgrenzen

Für diese C9-Abschlussrunde gilt:

- kein Bot gestartet
- keine Discord-/Twitch-/sonstige Nachricht gesendet
- kein Deployment ausgeführt
- keine Produktiv-Credentials für Antworttests verwendet
- keine alten Legacy-Pfade produktiv abgeschaltet
- keine neue Modell-/RAG-Schattenimplementierung eingeführt
