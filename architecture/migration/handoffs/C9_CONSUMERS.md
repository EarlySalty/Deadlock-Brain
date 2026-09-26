# C9 Consumer Wiring

Stand: 2026-09-26

C9 verdrahtet die Consumer gegen den aktuellen typisierten `BrainClient` / `brain-serve`-Vertrag. Die offenen C2/C3- und C6-Vertragsänderungen sind im Brain-C9-Branch enthalten, damit `AnswerStatus::Unavailable` und `AnswerStatus::BuildRejected` Bestandteil desselben Wire-Vertrags sind.

Es wurden keine Bots gestartet, keine Nachrichten gesendet, keine Produktivkonfiguration geändert und keine Deployments ausgeführt. Legacy-Pfade bleiben Default bzw. verfügbar; Umschaltung erfolgt nur explizit über Test-/Shadow-/Typed-Konfiguration.

| Repo | Branch | Commit | PR | Tests |
| --- | --- | --- | --- | --- |
| EarlySalty/Deadlock-Twitch-Bot | `codex/fix-c9-consumer-wiring` | `43d7f83a3661e30bf672605da32bdae49432e5bd` | #984 | Self-Explainer fokussiert: 19/19 grün. `tb-knowledge`: 23 Unit + 3 Load grün; bekannter Seed-Altfehler `stoerung_stream_info_felder_findet_uplink_stoerungen` bleibt unverändert 1/5 rot. |
| EarlySalty/Deadlock-Bots | `codex/fix-c9-consumer-wiring` | `98a7b37c89b0ab8ff6e6c4f8bdb3331debed8ca9` | #459 | `rust/scripts/check-brain-consumer.sh`: fmt/test/clippy grün. |
| EarlySalty/Deadlock-Docs | `codex/fix-c9-consumer-wiring` | `da32813bc8b58394b166db7baf3d35224d5b43db` | #4 | `tools/brain-adapter/check.sh`: fmt/test/clippy grün. |
| EarlySalty/Deadlock-2nd-Brain | `codex/fix-c9-consumer-wiring` | `a958e619fd82ac4ce0b0579aed6ac5286d959f5c` | #2 | `tools/brain-adapter/check.sh`: fmt/test/clippy grün. |
| EarlySalty/Deadlock-Brain CLI/MCP | `codex/fix-c9-consumer-wiring` | `5d2fad67e8304e17bd0569ac60df66669aa76d1a` | #50 | `brain-client --all-targets`: 14/14 grün; `brain-mcp`: 2/2 grün; `deadlock-brain` CLI binary `cargo check` grün. |

## Verdrahtung

### Twitch

Die echte öffentliche Self-Explainer-Route installiert `SelfExplainerBrainRuntime`.

- `legacy` bleibt Default.
- `typed` verwendet für die sichtbare Antwort ausschließlich `AsyncBrainClient -> brain-serve`.
- `shadow` lässt Legacy sichtbar antworten und wertet den typisierten Pfad nur report-only aus.
- Historie wird im typed/shadow-Pfad als begrenzter, ausdrücklich untrusted Conversation-Kontext im typisierten Query mitgeführt; sie bleibt nicht still auf dem alten Brain-Pfad.
- Scopes stammen ausschließlich aus vertrauenswürdiger Runtime-Konfiguration.
- `build_rejected` bleibt sichtbarer erklärender Antworttext.
- `unavailable`, ACL-, Provider- und Budgetfehler lösen im typed-Pfad keinen Legacy-Modell-/RAG-Fallback aus.
- Bestehende JSON-Form, `parts`, Grounding-/Source-Felder, Rate-Limit und nachgelagerte Logging-Wege bleiben erhalten.

### Deadlock-Bots

Der echte `brain`-Command-Composition-Root kann den vorhandenen `AiAnswerer` jetzt explizit als `legacy`, `typed` oder `shadow` verdrahten.

- Legacy bleibt Default.
- Typed verwendet `BrainApiAnswerer -> AsyncBrainClient -> brain-serve`.
- Channel-Allowlist, Open-Test-Grenzen, Cooldown, Fragenlänge und Discord-Ausgabe bleiben im bestehenden Command-Pfad.
- `build_rejected` wird über das bestehende Antwortformat ausgegeben.
- `unavailable` wird als Backendfehler behandelt, ohne stillen Legacy-Fallback im typed-Modus.
- Der Shadow-Wrapper protokolliert nur Ergebnisarten; der typed Adapter enthält keinen direkten Modell-/RAG-Pfad.
- Das bestehende `.github/workflows/pr-release-gate.yml` hat im finalen C9-Diff **keine Änderung**. C9 aktiviert keine Merge-Automatik.

### Docs

`tools/brain-adapter query ...` ist ein tatsächlich nutzbarer Query-Pfad und verwendet ausschließlich den gepinnten typisierten BrainClient.

- Scope fest `docs.public`.
- Bearer-Token nicht als CLI-Argument.
- Loopback-only Endpoint und Client-Timeouts.
- Kein Corpus-Import, kein Publishing, kein lokales RAG/Modell.

### 2nd-Brain

`tools/brain-adapter query ...` ist der tatsächliche interne Query-Pfad.

- Interne Scopes müssen explizit und vertrauenswürdig gebunden werden.
- Public-/Wildcard-Bindungen werden lokal abgewiesen; serverseitige ACL bleibt autoritativ.
- Kein Corpus-Export, Publishing, lokales RAG oder Modell-Fallback.

### Brain CLI/MCP

- `deadlock-brain answer` erzeugt einen typisierten `Query` und verwendet `AsyncBrainClient::new_local`.
- Token kommt ausschließlich aus `BRAIN_CLIENT_TOKEN`; Scopes müssen explizit angegeben werden.
- `brain-mcp` stellt `brain_answer` bereit und bindet Scopes ausschließlich aus der MCP-Runtime-Konfiguration.
- MCP kennzeichnet `unavailable`, ACL-, Provider- und Budgetfehler als Error-Result; `build_rejected` bleibt ein typisiertes, erfolgreich transportiertes Domain-Ergebnis.

## Noch nötige lokale Tests

Diese Tests wurden bewusst **nicht** gegen Produktion ausgeführt:

1. Twitch: isolierte `brain-serve`-Instanz mit nichtproduktivem Public-Token; `legacy/shadow/typed`, History-Auflösung, Timeout, ACL-Widerruf und Ausgabeparität über die echte HTTP-Route prüfen.
2. Deadlock-Bots: isolierte Command-Composition mit Fake-/Test-Discord-Transport und nichtproduktivem Brain-Token; Allowlist, Cooldown, `build_rejected`, `unavailable` und Shadow-Latenz prüfen. Vollständige DB-Integration nur mit separatem `CENTRAL_TEST_DSN`.
3. Docs: einen echten nichtproduktiven `docs.public`-Token gegen isoliertes `brain-serve` prüfen, inklusive ACL-Fehler, Timeout und Citation-Ausgabe.
4. 2nd-Brain: isolierte interne ACL-, Rollenwechsel-/Widerrufs-, Conversation-Isolation- und Provider-Egress-Prüfung mit ausschließlich freigegebenen Testdaten.
5. Brain CLI/MCP: End-to-End gegen isoliertes `brain-serve` für `answered`, `insufficient_evidence`, `build_rejected`, `unavailable`, falsche Scopes und Timeout.

## Bekannter nicht-C9-Fehler

Im Twitch-Repo bleibt `tb-knowledge/tests/seed.rs::stoerung_stream_info_felder_findet_uplink_stoerungen` auf dem aktuellen Basisstand rot. Die C9-spezifischen Knowledge-Unit-/Load-Tests und die Self-Explainer-Routenregressionen sind grün; C9 ändert den Seed-Selector bzw. Corpus nicht.
