# Pre-G5 Technical Review: Deadlock Brain

Stand: 26.09.2026. Rolle: lokaler finaler Integrator und Cutover-Vorbereiter.
Kein Production-Cutover, kein Merge nach `main`, keine produktiven Consumer oder Bots aktiviert, keine öffentlichen Nachrichten, keine Policy oder Pflichtprüfung umgangen, keine fremden Worktrees verändert.

## 1. Autoritativer Commit

| | Wert |
|---|---|
| Autoritativer Brain-Branch | `origin/migration/rust-integration` |
| Ausgangsstand geprüft | `d6cf9bc6fab7579cdee6781c7e59a7738807852a` (entsprach dem erwarteten Head) |
| Arbeitsbranch dieses Reviews | `integration/pre-g5-review-20260926`, eigener Worktree `~/.worktrees/brain-pre-g5-20260926`, direkt auf `d6cf9bc` |
| Einziger späterer Main-Kandidat | PR #40 `migration/rust-integration` nach `main` (Draft, bleibt Draft) |
| Getesteter Code-Commit dieses Reviews | `16963c96d0f8eb6197f21dc234c326adc8966204` (danach nur Dokumentation) |

Gewünschter Graph bleibt `main` ← `migration/rust-integration`. Keine weitere große Mergequelle.

## 2. Was wirklich fertig ist

Regressionsgeprüft auf dem Stand dieses Reviews (nicht neu entworfen):

| Bereich | Nachweis in diesem Review |
|---|---|
| Eigene Brain-PostgreSQL (OS-User, PGDATA, Socket, Port 5446, Rollen) | `ops/brain-postgres/verify-isolation.sh`: 54 von 54 bestanden; in `brain`, `brain_pilot`, `brain_pilot_legacy` nur `plpgsql`, 0 Foreign Server |
| `brain_service` ohne DDL, `brain_migrate` getrennt | Teil der 54 Prüfungen |
| Pooling und Backpressure, 600 Requests 8/16/32 | Wiederholung auf diesem Stand, Abschnitt 11 |
| Default-E2E 18/18 | Wiederholung auf diesem Stand, Abschnitt 11 |
| Backup/Restore | unverändert seit `FINAL_LOCAL_INTEGRATION_REVIEW.md` Abschnitt 8; kein Speicher- oder Schemacode geändert |
| Workspace | `cargo fmt --check` grün, `cargo clippy --workspace --all-targets --locked --offline -D warnings` grün, `cargo test --workspace --locked --offline`: 958 passed, 0 failed, 71 ignored (vorher 943, +15 neue Tests) |
| Skript-Suiten | `test_brain_serve.sh`, `test_brain_storage_upgrade.sh`, `test_brain_core_postgres.sh`, `test_wiki_runtime.sh` (C5 offline, kein Live-Capture), `s12/check-completion.sh`: alle Exit 0 |

TESTNACHWEIS[TW-1]: 958 passed, 71 ignored | Baseline: 0 rot (943 passed, 71 ignored auf `ed06e13` laut `FINAL_LOCAL_INTEGRATION_REVIEW.md`)

Neu gebaut in diesem Review:

| Commit-Thema | Inhalt |
|---|---|
| Legacy nach Kern | Crate `brain-legacy-import`: `brain_legacy` nach `SourceRecordV2` nach `CorpusRelease`, deterministisch und idempotent |
| Generische Revisionierung | `brain-ingestion::document_set`: Dokumentmenge gegen Checkpoint, Revision +1 bei Inhalts- oder Policyänderung, Tombstone bei Wegfall, leerer Lesevorgang bricht ab |
| Feeds | `brain-contracts::feeds` (`brain.feed.patchnotes.v1`, `brain.build_publish.v1`), Crate `brain-feeds` (Patchnotes-Adapter, Build-Publish-Port mit HTTP-Client und Fixture, Deadlock-Assets-API nach Kern) |
| Wiki | `wiki_runtime::stage_into_store` über den normalen `DocumentStorePort` |
| Doku | dieser Bericht, `LEGACY_CORE_MIGRATION.md`, bereinigte Status-, Gate- und Pfadbesitzer-Dateien |

## 3. Legacy nach Kern: Datenmigration

Vollständiges Inventar aller 49 Tabellen, Kategorien und Feldabbildung: [LEGACY_CORE_MIGRATION.md](LEGACY_CORE_MIGRATION.md).

- Kategorie 1 (Kern): `patch_events`, `patch_event_enrichments`, `entities`, `entity_aliases`. Abgebildet auf `legacy-patchnotes` (348 Prosa-Dokumente) und `legacy-entities` (905 Fakt-Dokumente).
- Kategorie 2 (neu beziehen): 15 Tabellen (Asset-Kataloge und Aktuellstand, Google-Sheet-Spiegel, YouTube).
- Kategorie 3 (abgeleitet, nicht migrieren): 23 Tabellen, darunter alle Population-, Reasoner- und Modellnotiz-Tabellen. Deadlock API bleibt Quelle der Matchdaten.
- Kategorie 4 (Archiv): 7 Tabellen, darunter `forum_claims` (im Altbestand selbst in Quarantäne) und `player_match_decision_notes` (Account-IDs).
- Kategorie 5: keine der 49; die Deadlock-Bots-Tabellen `plan_*`, `feeder_runs` und das Schema `knowledge` wurden nie kopiert.

Unbekannte Werte bleiben unknown: Lizenz, Autorisierungsreferenz, Modus und Spielgültigkeit werden nicht erfunden; die Veröffentlichungszeit wird nie als Gültigkeit verwendet.

Echtlauf gegen die eigene Instanz, Ziel-DB `brain_pilot_legacy`, 34 von 34 Prüfungen (`scripts/run_isolated_legacy_import.sh`, Bericht `~/.local/share/deadlock-brain/legacy-core-import-20260926/`):

- Release `legacy-core-f07ea85c09010285`, 1 253 Dokumente, Snapshot-Digest `3051f2c4eef4874c9b769fdf540a28a31627ed59ccdb2742647ba5d4626ac135`.
- Wiederholung: 0 neue Records, gleiche Release-ID und gleicher Digest, auch nach dem Umbau auf `document_set`.
- 0 Dubletten, 0 Hashabweichungen, Provenienz an jedem Record.
- `brain-serve` antwortet aus dem Release: Entitäts-Fakt und Alias mit Zitat; Unknown, fremder Scope, falscher Patch, unbekannter Modus und fehlender Egress bleiben unbeantwortet; nicht erteilter Scope 403.
- Policy-Revision (Egress nur für Patchnotes) ergibt 348 neue Revisionen und Release `legacy-core-6c158962d92e8151`; Explain über echte Patch-Prosa mit Zitat über eine Loopback-Fixture.
- ACL-Revoke auf echten Daten wirkt sofort auf den gepinnten Release.
- `brain_legacy` unverändert, produktive DB `brain` ohne Kernrecords.

`brain_legacy` ist weiterhin nur Archiv und Importquelle. Keine Laufzeitabhängigkeit.

## 4. Offene und geschlossene Feeds

| Übergang | Alt | Brain-Seite jetzt | Provider-Seite | Status |
|---|---|---|---|---|
| Patchnotes | Brain liest `patchnotes.changelog_posts` per SQL | Vertrag `brain.feed.patchnotes.v1` (Post-ID, Titel, HTTPS-URL, Veröffentlichungszeit, Rohtext, Raw-SHA-256, Source-Revision, Provider, Exportrevision, Exportzeit; `deny_unknown_fields`, Hashprüfung, Limits) und Adapter nach `SourceRecordV2` mit Provenienz | Export im Patchnotes-Bot fehlt (der Bot hat nur eine übersetzte Web-Projektion ohne Rohtext) | offen |
| Steam-Build-Publish | Brain schreibt und liest `steam.steam_tasks` | Vertrag `brain.build_publish.v1` (Request-ID als Idempotenzschlüssel, Held, Build, Payload, Caller; Status queued/running/succeeded/failed, `hero_build_id`, Fehlerklasse, Zeitstempel, Request-Hash-Bindung), `BuildPublishPort`, HTTP-Client (nur HTTPS oder Loopback, Bearer aus Env, `Idempotency-Key`, Antwortlimit 64 KiB, keine Redirects), Fixture mit erlaubten Zustandsübergängen | Endpunkt `POST /builds/v1/publish`, `GET /builds/v1/publish/{id}` im Steam-Bot fehlt | Vertrag bereit, Provider offen |
| Deadlock-Bots nach Brain | Bots schreiben `entity_snapshots`, `current_entity_state`, `hero_catalog`, `item_catalog`, `source_runs` | Entscheidung: Deadlock API direkt als Brain-Quelle. `brain-feeds::deadlock_assets` nutzt den vorhandenen Assets-Adapter (Schema-Pin, OpenAPI-Driftprüfung, Quarantäne, begrenzte Requests) und schreibt Fakt-Dokumente je Entität mit HTTP-Body-Hash in den Kern. Kein Schreibweg für den Bot | Abschalten von `dl-brain api_ingest` gehört zu G5 | Vertrag bereit |

Kein Brain-Code hat ein DL-Main- oder Steam-DB-Credential bekommen. Die alten Direktpfade laufen bis G5 unverändert weiter; sie wurden weder umgestellt noch abgeschaltet.

Deadlock API als externe Quelle: Die Match-, Meta- und Population-Adapter in `dbrain-sources` haben Schema-Pin, Raw-Hash, Provenienz, Quarantäne bei Drift und begrenzte Requests, schreiben aber weiterhin nur in den Altstore über `DEADLOCK_CENTRAL_DSN` (DL-Main). Für V1 ist keine lokale Kopie des Analysebestands nötig; die Adapter brauchen vor G5 entweder eine Kernanbindung oder die Einordnung als reine Abfrage zur Antwortzeit. Kein ClickHouse gebaut.

## 5. Wiki

- Keine Betreiberfreigabe für Capture, Raw-Aufbewahrung und Lizenz gefunden. Deshalb kein Netzwerk-Capture und kein Full-Wiki-Crawl.
- Gebaut: `wiki_runtime::stage_into_store` schreibt Raw-, IR- und Fact-Records als Batches mit Checkpoint und Lease in den normalen Kern-Store und veröffentlicht den `CorpusRelease` dort. Kein Scratch-Marker, kein `wiki_scratch.sql`, keine Legacy-`source_documents`, kein zweiter Store. Mehrere Revisionen einer Seite gehen in aufeinanderfolgende Batches; ein veralteter Capture und ein Revisionskonflikt brechen ab. Test mit dem C5-Fixture: 15 Records, 3 Fakten, Wiederholung 0 neue Records, Delta nur geänderte Records.
- Der alte Scratch-Pfad bleibt als Offline-Regression (`test_wiki_runtime.sh` grün).
- Betreiberblocker: Freigabe für Capture, Raw-Aufbewahrung und Quellenlizenz, dazu Mapping und `ProjectionReview` für echte Felder.

## 6. Provider

Keine ausdrückliche Betreiberentscheidung zu Provider, Modell, Egress, Budget und Credentials gefunden. Kein Modell ausgewählt, kein echter Shadow-Test. Verwendet wurden nur Loopback-Fixtures.

## 7. Replay

Keine freigegebene echte `.dem` vorhanden (gesucht in Home, `/var/lib`, `/srv`, `/tmp`); nichts heruntergeladen, keine Match-ID geraten.

Produktentscheidung für den Betreiber:

| Option | Inhalt | Auswirkung |
|---|---|---|
| A: Replay gehört zu V1 | G2 bleibt blockiert, bis eine freigegebene echte `.dem` durch Decoder, Store und Antwortpfad gelaufen ist | Cutover wartet auf Replaydatei, Rechteklärung und Referenzvergleich |
| B: Replay nach V1 | Replay-Funktion bleibt ausdrücklich deaktiviert und wird aus dem V1-Gate-Scope genommen | G2 hängt dann nur noch an Wiki, Provider und Retrieval-Qualität; Replay wird später mit eigenem Gate nachgezogen |

Der Agent trifft diese Entscheidung nicht.

## 8. Consumer-PRs

| PR | Head | mergeable | Pflichtprüfungen | echte Codefehler | Infrastrukturblocker | manuelle Freigabe nötig |
|---|---|---|---|---|---|---|
| Twitch #984 | `d877a9dc` | ja (`BLOCKED`) | Ruleset: Semantic review, Scope-Abgleich, drei Frontend-Gates, Rust SQLx required, zwei Typed-Brain-Fixtures. Alle grün außer Semantic review | keine bekannt | Semantic review: „You have exceeded your monthly quota" | ja: gültiges Semantic review nach Kontingent-Reset; Merge erst nach G5 |
| Bots #459 | `0e3cf65a` | ja (`UNSTABLE`) | keine Rulesets auf `main` sichtbar; Workspace, Knowledge-Eval/Hybrid, Typed-Brain-Fixtures, config grün | keine bekannt | Semantic review Kontingent | ja. **In diesem Review auf Draft gesetzt**: Die `main`-Automation „Auto merge after gates" (`pull_request_target`, Squash-Merge für Nicht-Drafts nach Semantic ALLOW) hätte den PR nach Kontingent-Reset automatisch gemergt. Das widerspricht dem PR-first-Testbetrieb. |
| Docs #4 | `ba4143f8` | ja (`CLEAN`) | Typed Brain adapter fixtures, GitGuardian grün | keine | keine | Merge erst nach G5 |
| 2nd-Brain #2 | `46aa2d37` | ja (`UNSTABLE`) | Typed Brain adapter fixtures rot, GitGuardian grün | keine; lokal grün | Job ohne Runner, 0 Schritte. Annotation: „recent account payments have failed or your spending limit needs to be increased". Versuch 2 identisch | ja: GitHub-Abrechnung klären, dann Lauf wiederholen |

Kein Required Check wurde umgangen, kein Consumer lokal gemergt, Twitch hat nur eine report-only-Automation (kein Merge-Aufruf).

## 9. PR #40

| | Wert |
|---|---|
| Zustand | offen, Draft, `MERGEABLE`, `UNSTABLE` |
| Schutz auf `main` | keine Rulesets, „Branch not protected" (HTTP 404). Es gibt also keine konfigurierte Required-Check-Liste; der Merge-Readiness-Workflow ist report-only |
| Checks auf `d6cf9bc` | alle funktionalen Suiten grün; Semantic review übersprungen (Draft); GitGuardian rot |
| GitGuardian | Incident 37635766, Typ „Username Password", Commit `df2d61f`, `scripts/run_isolated_serve_checks.sh` Zeile 24. Gefunden wurde `"username": "brain_service"` neben `"password_env": "BRAIN_SERVE_PG_PASSWORD"`: ein Umgebungsvariablenname, kein Secretwert. Das echte Passwort kommt nur zur Laufzeit aus Infisical. False Positive |
| Behandlung | Keine History-Umschreibung auf dem gemeinsamen Branch (würde Force-Push brauchen). Der Incident muss im GitGuardian-Dashboard als False Positive oder Test-Credential geschlossen werden; das ist eine Betreiberaktion, weil diese Session keinen GitGuardian-Zugang hat. Die neuen Configs dieses Reviews benutzen `auth_env` statt eines Passwort-Feldnamens; `gitleaks` über die neuen Commits: keine Funde |
| Draft | bleibt Draft, weil technische G5-Blocker offen sind (Abschnitt 12) |
| Merge | nicht gemergt, nicht vorgesehen |

## 10. Superseded Branches

| Branch | Verhältnis zu `migration/rust-integration` | Einordnung |
|---|---|---|
| `feat/brain-rust-cutover-20260919` (`c8ad3ef`) | nicht Vorfahr, 34 Commits voraus, keiner patchgleich enthalten | superseded für den neuen Kern. Inhalt ist Arbeit am Legacy-Laufzeitpfad (Python-Entfernung, Patch-Evidence-Review mit drei Migrationen `2026-09-18-patch-evidence*`, Rust-MCP-Transport, YouTube-Rustpfad, Planpaket-Kopie). Keine der deployten Legacy-Releases (`12814d9`, `92db8ee`, `b306fa9`, `e1fbd129`) stammt aus diesem Branch; alle vier sind Vorfahren von `main` und des Integrationsbranches. Nichts davon wird für den Kern gebraucht. Nicht mergen; wer die Legacy-Funktionen später will, übernimmt sie gezielt per eigenem PR |
| `feat/brain-final-integration-20260921`, `feat/brain-release-completion-20260921` | enthalten den Cutover-Branch | superseded, gleiche Begründung |
| `feat/brain-runtime-release-20260921` | 1 Commit voraus | superseded (Legacy-Release-Notiz) |
| `integration/final-local-20260926` | 1 Commit nicht enthalten (Idle-Prune) | superseded laut `FINAL_LOCAL_INTEGRATION_REVIEW.md` Abschnitt 11 |
| `codex/core-completion-20260925`, `codex/consumer-ci-completion-20260925`, `codex/external-sources-completion-20260925`, `codex/wiki-completion-20260925`, `codex/replay-decoder-completion-20260925` | Vorfahren | vollständig enthalten |

Keine Branches oder Worktrees gelöscht.

## 11. Aktuelle Gates

Regression auf diesem Stand (`scripts/run_isolated_load.sh`, Pool 4, Instanz 5446, `max_connections=40` unverändert, Bericht `~/.local/share/deadlock-brain/loadtest-20260926-pre-g5/`):

| Messgröße | 8 Worker | 16 Worker | 32 Worker |
|---|---|---|---|
| Default-E2E nach erzwungenem DB-Neustart | 18/18, `failed_cases: []` | 18/18 | 18/18 |
| Last: answered von 600 | 600 | 600 | 600 |
| Peak `brain_service` in `pg_stat_activity` | 4 | 4 | 4 |
| neue `brain_service`-Verbindungen laut Serverlog | 8 | 8 | 8 |
| „too many clients" / Rollendeckel erreicht | 0 / 0 | 0 / 0 | 0 / 0 |
| p50 / p95 / p99 | 15,5 / 25,0 / 38,3 ms | 37,3 / 67,6 / 131,0 ms | 111,1 / 202,7 / 308,4 ms |

Die Latenzen liegen über denen des Laufs vom Vormittag (p99 27 / 111 / 174 ms). Der geänderte Code berührt den Anfragepfad nicht; die Abweichung wird der Hostlast zugeordnet und ist kein Pass-Kriterium. Verbindungsverhalten und Fehlerbild sind identisch.

| Gate | Bewertung | Begründung |
|---|---|---|
| G0 | teilweise | Inventar und Runtime belegt; SLO und Lastprofil nicht vom Betreiber freigegeben |
| G1 | bereit | Verträge integriert und genutzt, neue Verträge `brain.feed.patchnotes.v1` und `brain.build_publish.v1` versioniert, Pfadbesitzer aktualisiert |
| G2 | nicht bereit | Echter Wissenspfad aus Legacy-Daten beantwortbar, aber: kein echter Wiki-Pilot, kein freigegebener Provider, Replay-Entscheidung offen, Fakt-Profil ohne Relevanzschwelle (ein gemeinsamer Term genügt für eine Faktenantwort) |
| G3 | nicht bereit | Patchnotes und Entitäten im Kern (nur Pilot-DB), Consumer-Parität lokal. Es fehlen Kernanbindungen für Sheet- und YouTube-Daten, die Provider-Seiten von Patchnotes-Feed und Build-Publish, die Kernanbindung oder Einordnung der Match-/Meta-Adapter und die Abschaltung der Direkt-Writer und -Reader |
| G4 | bereit (lokal) | Last, Isolation, Pool und E2E auf diesem Stand erneut grün; Backup/Restore unverändert belegt |
| G5 | nicht selbst freigegeben | Betreiberentscheidung |

## 12. Konkrete G5-Blocker

Technisch:

1. Provider-Export `brain.feed.patchnotes.v1` im Patchnotes-Bot (PR im Repo `EarlySalty/Deadlock--Patchnotes-Bot`).
2. Publish-Endpunkt `brain.build_publish.v1` im Steam-Bot (PR im Repo Deadlock-Steam-Bot), danach Umstellung von `dbrain-reasoner::publish` auf den Port.
3. Kernanbindung für Google-Sheet-Daten (Heldenwerte) und Entscheidung für YouTube-Transkripte.
4. Match-/Meta-Adapter der Deadlock API an den Kern binden oder ausdrücklich als Abfrage zur Antwortzeit festlegen.
5. Relevanzschwelle im Fakt-Profil des Kernels.
6. Runner und Timer für Legacy-Import, Feeds und Assets-Adapter gegen die produktive DB `brain` (erst mit G5 ausführen).
7. Integration dieses Review-Branches in `migration/rust-integration` und ein grüner CI-Lauf auf PR #40 mit dem neuen Head.

Betreiberentscheidungen und externe Blocker:

8. Wiki: Freigabe für Capture, Raw-Aufbewahrung und Lizenz, dann kleiner Pilot mit wenigen Heldenseiten.
9. Provider: Provider, Modell, Egress, Budget und Credentials, dann Shadow-Test.
10. Replay: Option A oder B aus Abschnitt 7.
11. GitGuardian-Incident 37635766 als False Positive schließen.
12. Semantic-Review-Kontingent für Twitch #984 und Bots #459; GitHub-Abrechnung für 2nd-Brain #2.
13. Deadlock-Bots: Auto-Merge-Automation auf `main` pausieren oder auf report-only umstellen, bevor #459 wieder aus Draft kommt (Aufgabe der Bots-Repo-Session laut PR-first-Entscheidung).

## Marker

LEGACY_CORE_MIGRATION_READY: JA

PATCHNOTES_FEED_READY: NEIN

STEAM_PUBLISH_CONTRACT_READY: JA

BOT_INGEST_CONTRACT_READY: JA

BRAIN_DB_ISOLATED: JA

DB_POOLING_VERIFIED: JA

600_REQUEST_TEST_PASSED: JA

DEFAULT_E2E_PASSED: JA

WIKI_REAL_PILOT_PASSED: NEIN

PROVIDER_SHADOW_PASSED: NEIN

REAL_REPLAY_PASSED: NEIN

CONSUMER_PRS_READY: NEIN

PR40_ALL_REQUIRED_CHECKS_GREEN: NEIN

G1_READY: JA
G2_READY: NEIN
G3_READY: NEIN
G4_READY: JA

TECHNICALLY_READY_FOR_G5_REVIEW: NEIN

PRODUCTION_CUTOVER_READY: NEIN

Erläuterung zu den JA-Markern der Verträge: `LEGACY_CORE_MIGRATION_READY` bedeutet, dass der Importpfad für die Kategorie-1-Daten deterministisch, idempotent und an echten Daten belegt ist; der Import in die produktive DB `brain` ist G5. `STEAM_PUBLISH_CONTRACT_READY` und `BOT_INGEST_CONTRACT_READY` beziehen sich auf Vertrag, Client beziehungsweise Adapter und Fixture-Transport; die Provider-Seite des Steam-Bots und die Abschaltung des Bot-Writers stehen in Abschnitt 12.

`TECHNICALLY_READY_FOR_G5_REVIEW: NEIN` ist keine G5-Entscheidung und löst weder Merge noch Deploy aus.
