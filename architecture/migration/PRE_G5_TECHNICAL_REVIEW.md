# Pre-G5 Technical Review: Deadlock Brain

Stand: 29.09.2026. Autoritativer integrierter Codehead: `022f8a981c2164f6d8d4302bae2194e100c4f65c`, regulär über PR #59 nach `migration/rust-integration` gemergt.

## Aktueller Abschlussstand

R5-Codeabnahme A+C: GO; lokales Gesamtgate: ALLOW. Beide Befunde gelten für `72db816056fb0ed53810ab77ea4417dc0812e7ca`, nicht als finale Workspace-, Release-, PostgreSQL- oder Lastabnahme des integrierten Heads `022f8a9`. Der vollständige Lauf wartet auf den tatsächlichen Bericht `FINAL-VERIFICATION.md`. Bis dahin sind keine finalen Testmarker oder Pass-Zahlen für `022f8a9` gesetzt.

- Matchdaten sind privat und accountgebunden. Der integrierte Pfad geht über den API-Adapter, `SourceRecordV2`, den normalen Store und Release. Identität, Hashprojektion und atomarer Revoke gehören zum Pfad. Code: `rust/crates/brain-feeds/src/bin/brain-match-ingest.rs:96-107,141-168,170-218` und `rust/crates/brain-feeds/src/deadlock_match.rs:30-80,403-495`.
- Assets-Core deckt V1-Startwerte ab. Es gibt keinen Sheet-Fallback. Code: `rust/crates/brain-feeds/src/deadlock_assets.rs:19-26,41-62,64-92`.
- Meta und Population werden als typisierte Fakten behandelt. Die Item-Matchquote stammt aus `hero-stats`; die Quelle attestiert keine Patchzugehörigkeit. Patchgebundene Anfragen bleiben fail-closed. Code: `rust/crates/dbrain-sources/src/analytics_runtime.rs:23-48,103-123,186-213` und `rust/crates/dbrain-retrieval/src/contract_port.rs:106-114`.
- Gemeinsame Deadline und finaler Kernel-Guard sind im R5-Delta unabhängig geprüft. Der echte lokale Zweitsnapshot-Nachweis und die R5-Suiten gelten für `72db816`, nicht für den später integrierten Gesamtstand. Codebeleg: `rust/crates/brain-kernel/src/execution.rs:62,143-150`; unabhängiger R5-Prüfbericht im Koordinationsartefakt `REVIEW-AC-R5.md`.
- Der Wiki-Pfad über normalen Store und Release ist offline geprüft. `WIKI_REAL_PILOT_PASSED=NEIN` mangels Quellen-, Lizenz- und Aufbewahrungsfreigabe.
- `PROVIDER_SHADOW_PASSED=NEIN` mangels Anbieter-, Modell-, Egress- und Budgetfreigabe. Kein freigegebener `.dem`; Replay V1 oder später ist Betreiberentscheidung.
- Kein Production-Cutover, G5 bleibt `NEIN`. Consumer sind nicht aktiviert. Steam-Publish wurde nicht ausgeführt.

Die folgenden nummerierten Abschnitte dokumentieren frühere Prüfstände. Sie sind historische Evidenz, keine finale Abnahme von `022f8a9`. Dazu gehören insbesondere 943, 958 und 980 Workspace-Tests, 18/18 E2E sowie frühere 600-Request-Werte.

Historischer Stand: 26.09.2026. Rolle: lokaler Integrator und Cutover-Vorbereiter.
Kein Production-Cutover, kein Merge nach `main`, keine produktiven Consumer oder Bots aktiviert, keine öffentlichen Nachrichten, keine Policy oder Pflichtprüfung umgangen, keine fremden Worktrees verändert.

## 1. Historische Review-Commitbasis

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

## Nachtrag vom 26.09.2026: Fortsetzung vor G5

Dieser Nachtrag bewertet die Fortsetzung nach PR #51. Die Abschnitte davor dokumentieren den damaligen geprüften Stand; ihre Testzahlen gelten nicht automatisch für spätere Feature-Branches. PR #51 war vollständig grün. Der zuvor lose Commit `44b5a5981e0dcd0b39000f2d3c3812af2b4eeb71` wurde auf `integration/pre-g5-followup-20260926` erhalten. [PR #52](https://github.com/EarlySalty/Deadlock-Brain/pull/52) wurde mit vollständig grüner CI ausschließlich nach `migration/rust-integration` integriert; der Merge-Commit ist `6391c2ab0a33c6a043a42c8a587d38d8dbb57986`. Er ergänzt das echte Tombstone-Skript, eine Patchnotes-Provider-Fixture und den Brain-Feeds-Contracttest. Der belegte Ausgangsbestand hatte 1 253 Dokumente; Entfernung erzeugte je einen Tombstone für `legacy-entities` und `legacy-patchnotes`, eine Wiederholung null neue Records. Eine leere Entity-Quelle brach ohne Massentombstones ab.

Das Tombstone-Skript ist auf PR #52 inzwischen mit Commit `97333f6` auf einen eigenen PostgreSQL-16-Wegwerfcluster über einen privaten Unix-Socket umgestellt. Es liest nur den vorhandenen Legacy-Dump; der produktive PostgreSQL-Dienst, Infisical, Passwort- und DSN-Umgebungsvariablen werden nicht benutzt. Ein neuer Echtlauf bestand T1–T5 erneut: 1 253 Basisdokumente, je ein Entity- und Patch-Tombstone, Wiederholung 0/0, leere Entity-Quelle aus dem erwarteten Grund fail-closed, Heads unverändert und Herkunft ohne Mismatch. Cluster und temporäre Dateien wurden aufgeräumt. Shell-Syntax, Diff-Prüfung und ShellCheck (abzüglich eines Trap-Fehlalarms SC2317) waren grün.

### Sheet-Tabellen und Kernpfad

| Tabelle | Entscheidung für V1 | Kernanbindung |
|---|---|---|
| `sheet_items` | Cache/Spiegel von Item-ID und Namen aus `raw_items_and_abilities`; kanonisch sind Assets-API-Items | Durch `/v1/assets/items` ersetzt; kein Sheet-Import |
| `sheet_tab_rows` | Untypisierte Zeilen auch aus Scratchpad/Calculators | Historisch/deferred; kein Bulk-Import |
| `sheet_raw_heroes` | Spiegel von Helden-ID, Namen, Status und Grundwerten aus `raw_hero_data` | Durch `/v1/assets/heroes` ersetzt; kein Sheet-Import |
| `sheet_heroes_stats` | Abgeleiteter Sheet-Cache für HP, DPS, Feuerrate, Munition, Growth und Falloff | Durch typisierte Assets-API-Heldenfakten ersetzen; kein Sheet-Import |
| `sheet_boons_ap` | Sheet-exklusive Souls-zu-Boon/AP-Reihe ohne belegten V1-Core-Leser | Historisch/deferred; kein V1-Import |
| `sheet_hero_rankings` | Subjektive Community-Wertungen wie Carry und Support | Historisch/deferred; kein kanonischer Spielwert |
| `sheet_shop_bonuses` | Sheet-exklusive Souls-Kosten-zu-Bonus-Reihe ohne belegten V1-Core-Leser | Historisch/deferred; kein V1-Import |

Damit ist keine pauschale Migration der sieben Tabellen gerechtfertigt. Kein Sheet-exklusiver Fakt ist als erforderlicher V1-Core-Fakt belegt; `brain_legacy` bleibt keine Laufzeitquelle. Zum Zeitpunkt dieser Bestandsaufnahme serialisierte der Assets-Adapter nur skalare Top-Level-Felder und ließ verschachtelte Heldenwerte wie `starting_stats.max_health.value` aus. Die fünf für V1 belegten `starting_stats`-Felder wurden anschließend in PR #55 typisiert und mit API-Provenienz angebunden; weitere abgeleitete Sheet-Werte wie DPS und Falloff sind damit nicht pauschal Core-Fakten. Ein später gewünschter Economy-Fakt braucht einen eigenen typisierten Quellenvertrag statt eines Imports aus `sheet_tab_rows`.

### Match, Meta, Population und YouTube

- Match (A, Core-Ingest): konkrete Match-Metadaten und zitierbare Demo-Evidenz als `SourceRecordV2` aus der Deadlock API, mit Account-Scope für personenbezogene Werte. Eine interaktive Abfrage beliebiger aktueller Spieler-Matches ist ein eigener späterer Runtime-Pfad, kein Ersatz für den V1-Faktenimport. Brain erhält weder `DEADLOCK_CENTRAL_DSN` noch eine direkte ClickHouse-Verbindung.
- Meta (B, Runtime-Lookup): zeitabhängige Aggregation aus der vorhandenen Deadlock-API-Analytics, begrenzt nach Patch, Zeitfenster und Umfang und mit Provenienz. Eingefrorene, ausdrücklich als Release-Fakt benötigte Aggregate wären ein späterer separater Import.
- Population (B, Runtime-Lookup): aktuelle Raten und Aggregationen aus derselben Deadlock-API-Analytics, begrenzt und mit Provenienz; keine zweite ClickHouse-Instanz und kein neuer Brain-DB-Pfad.
- YouTube: Für V1 ist kein technisch zwingender Kern-Fakt belegt. Transkripte bleiben deferred, bis der Betreiber Scope, Lizenz und Aufbewahrung entscheidet. Es gibt keinen Legacy-Fallback.

Diese Pfadentscheidungen sind technisch dokumentiert, aber noch nicht vollständig implementiert oder gegen die betreffenden API-Antworten geprüft. Deshalb bleibt `MATCH_META_PATH_READY` vorerst `NEIN`.

### Providerprüfung nach PR #51

Der Patchnotes-Export auf [PR #49](https://github.com/EarlySalty/Deadlock--Patchnotes-Bot/pull/49) implementiert `brain.feed.patchnotes.v1`. Der semantische Abgleich mit der Brain-Fixture bestätigte Feldform, Hash-Bindung, Größen- und Mengenlimits, HTTPS-URL, Duplikat- und Unknown-Field-Ablehnung sowie die deterministische Post-Reihenfolge. Die Prüfung fand zwei Randfehler: Ein übergroßer Rohpost oder mehr als 5 000 Posts wurden als HTTP 500 statt 413 beantwortet; reine Änderungen an Titel, URL oder Veröffentlichungszeit änderten die `source_revision` nicht. Die Korrektur liegt als Commit `77c89fa` auf dem Provider-Branch; lokal bestanden 103 `unittest`-Tests sowie 181 `pytest`-Tests und 30 Untertests. Vor dem produktiven Einsatz bleibt der Auth- und Config-Pfad mit der Workspace-Regel „Secrets aus Infisical, normale Config-Datei für den Rest, keine Environment-Variablen für Config“ abzugleichen. Ein direkter Infisical-Pfad wurde ohne etabliertes Muster nicht als Nebeninfrastruktur erfunden.

Der spätere PR-Head `41f26e9` bereinigt Imports und Formatierung; Ruff-Check, Ruff-Formatprüfung und lokale Tests bestanden erneut. Der unabhängige Python-Review bestätigte die Contract-Implementierung, aber nicht die Betriebsbereitschaft des eigenständigen Servers ohne Infisical-Bootstrap. Der Selbst-Review-Gate lief in ein Grok-Timeout und lieferte keinen Codebefund.

GitGuardian war auf dem Ausgangs-PR grün. Die roten Security-CI-Jobs belegen derzeit keinen Codefehler: GitHub meldete `runner_id=0` und `steps=[]` und annotierte einen Abrechnungs- beziehungsweise Spending-Limit-Blocker. Ein vollständiger CI-Grün-Nachweis für den Provider fehlt damit.

Nach der Provider-Korrektur wurde die Brain-Fixture auf die metadatengebundene `source_revision` und daraus folgende `export_revision` gebracht. Commit `c96575b` auf PR #52 prüft die beiden Revisionen rechnerisch gegen die Provider-Kanonisierung. `cargo test --locked --offline -p brain-feeds` bestand mit der verfügbaren modernen Rust-Toolchain (6 Tests); die CI auf dem neuen PR-Head läuft separat.

### Fakten-Relevanz und Nachweisgrenze

Der Fakten-Relevanz-Fix liegt separat als `cbb270cc6bfdcce5feabc996095818dc9d005b3a` auf `feat/pre-g5-fact-relevance-20260926`. Fokussierte Kernel-Tests und Clippy mit `-D warnings` waren lokal grün. Der Default-Pilot-E2E nutzt das Explain-Profil; dessen alter 18/18-Nachweis prüft die neue Faktenentscheidung daher nicht. Der Legacy-Echtrelease und die vollständigen Skript-Suiten sind auf diesem Commit noch nicht neu belegt.

Die nachfolgenden Fix-Commits `81dc89e`, `efa782c` und `9b8c79a` schließen die im unabhängigen Intent-Review gefundenen Fälle: Identitätszeilen sind keine Faktenfelder; Kernel und Release-Retrieval teilen die deutsche Wortnormalisierung; Semikolon-Aliase werden erkannt; patchneutrale Records bleiben bei passendem Release verwendbar, ausdrücklich falsche Patches und Modi gesperrt. Alias-Mehrdeutigkeit wird über alle für diese Anfrage zulässigen Release-Records statt nur über die Top-Treffer geprüft, einschließlich aktueller Head-Rechte und Revokes. Der unabhängige Review bewertet den stabilen Release-/Head-Zustand mit **JA**. Bei einer gleichzeitigen Rechtefreigabe zwischen zwei Head-Lesevorgängen bleibt eine Race-Möglichkeit; sie ist als Restgrenze vermerkt.

Auf `9b8c79a` bestanden `cargo fmt --all -- --check`, Workspace-Clippy mit `-D warnings`, `cargo test --workspace --locked --offline` und `cargo build --workspace --release --locked --offline` mit der im Repo gepinnten Rust-Version 1.97.1. Das sind Code- und Fixture-Nachweise, keine Wiederholung des Default-E2E oder der Last gegen die getrennte Brain-PG.

Konkreter Grund für die weiterhin ausgelassenen **lokalen** Pilot-, E2E- und Last-Echtläufe: `run_isolated_pilot.sh` und `run_isolated_legacy_import.sh` exportieren DB-Passwörter als Environment-Variablen; `test_brain_serve.sh` setzt Environment-Konfiguration für den Testprozess. Die Workspace-Startanweisung verbietet Environment-Variablen für Config und Secrets im Klartext. Das Tombstone-Skript in PR #52 nutzt diesen Pfad seit `97333f6` nicht mehr und ist erneut real gelaufen; der historische Komplettimport aus Abschnitt 3 ist kein erneuter Lauf des neuen Fakten-Commits. Die spätere C1-CI erbrachte den 600-Request-Nachweis auf dem finalen Head. Vor einer vollständigen lokalen Wiederholung der anderen Skripte ist ein zulässiger Secret-/Config-Transport nötig.

### Steam-Publish-Provider

[Steam-PR #73](https://github.com/EarlySalty/Deadlock-Steam-Bot/pull/73) auf `feat/brain-build-publish-provider-20260926` sichert den zuvor uncommitteten Arbeitsstand und implementiert die zwei Endpunkte für `brain.build_publish.v1`. Der vorgesehene persistente Schlüssel `request_id` wird in derselben PostgreSQL-Transaktion wie `steam_task` angelegt. Ein Unique Constraint ist in der separaten [Schema-PR #461](https://github.com/EarlySalty/Deadlock-Bots/pull/461) enthalten; diese Migration muss vor jeder späteren Provider-Aktivierung angewendet werden. Der Task-Pfad wurde auf `BUILD_PUBLISH_ORIGINAL` korrigiert, damit der Inline-Build-Payload verarbeitet wird, und die `hero_build_id` wird aus dem tatsächlichen Ergebnis gelesen. Es wurde kein Build publiziert.

Die 11 fokussierten HTTP-Contracttests, Formatprüfung und Clippy waren lokal grün. Der exakte vollständige Steam-Testbefehl kompiliert, scheitert aber in 38 bestehenden DB-Tests ohne `CENTRAL_TEST_DSN`. Der Test-Runner bietet dafür nur Environment-Konfiguration; ein solcher Lauf verstieße gegen die Workspace-Regel. Die Race- und Restart-Transaktion ist daher bisher nur mit einem Store-Fixture, nicht gegen eine echte PostgreSQL-Testinstanz belegt. Die GitHub-Actions-Jobs zu PR #73 wurden mit `runner_id=0` und `steps=[]` wegen Billing nicht gestartet; GitGuardian war grün. Persistente Idempotenz ist implementiert, aber die vollständige Test-Abnahme bleibt offen.

Nach unabhängiger Kritik wurde PR #73 mit Commit `42df79c` korrigiert: `updated_at` stammt nun vom gespeicherten `steam_task`-Zustand und ändert sich nicht bei bloßem GET. Ein zusätzlicher ignorierter HTTP-Integrationstest wurde in einer isolierten lokalen PostgreSQL-Datenbank mit Peer-Authentifizierung und normaler Config-Datei tatsächlich ausgeführt: zwei parallele identische POSTs ergaben genau einen Task, ein neu aufgebauter Provider-State konnte den Status lesen, queued/running/succeeded behielten stabile Zeitstempel, ein Payload-Konflikt erzeugte keinen Task. Die Wegwerf-Datenbank und ihre secretfreie Config wurden danach entfernt. Der unabhängige Nachreview fand keinen weiteren Codeblocker; die volle Alt-Suite und GitHub-CI bleiben aus den genannten Gründen ohne Grün-Nachweis. Eine Aktivierung würde zuerst Schema-PR #461 und den zentralen Migrator erfordern und ist hier ausdrücklich nicht erfolgt.

### Gemeinsame Fakten- und Assets-Integration

[PR #53](https://github.com/EarlySalty/Deadlock-Brain/pull/53) und [PR #54](https://github.com/EarlySalty/Deadlock-Brain/pull/54) wurden auf `migration/rust-integration` umgestellt und gemeinsam mit dem neuen Ende-zu-Ende-Test in [PR #55](https://github.com/EarlySalty/Deadlock-Brain/pull/55) geprüft. Der zusammengesetzte Diff verankert Fakten an Entity/Alias und das konkret angefragte Feld. Releaseweite Alias-Mehrdeutigkeit berücksichtigt aktuelle Heads und Rechte. Legacy- und Assets-Heldendokumente teilen nur bei gleicher externer Helden-ID **und** gleichem kanonischem Namen eine Identität. Legacy-Quelle und -ID stammen aus strukturierten Import-Metadaten statt aus frei ergänzbaren Content-Zeilen. Doppelte externe IDs und reservierte Quell-/ID-/Alias-Schlüssel werden abgewiesen; historische numerische `hero`-Metadaten erzeugen keine Namenszeile. Parser-Revision v2 erzwingt die neue Ableitung auch bei vorhandenen Checkpoints. Widersprüchliche Aussagen zum selben Feld werden auch bei einem konfigurierten Retrieval-Limit von eins nicht beantwortet. Ein längerer Feldname wie „max health“ hat Vorrang vor einem nur teilweise passenden „health“; eine angefragte Zahl muss zum Wert dieser Feldzeile gehören. Der typisierte Assets-`fact_key` ist eindeutig an die Wertzeile gebunden.

Der Assets-Feed erhält fünf erforderliche numerische `starting_stats`-Felder als eigene Core-Records. Die Provenienz zeigt über JSON-Pointer auf den tatsächlich erfassten API-Antwortkörper; Tests dereferenzieren die Pointer und prüfen auch umsortierte Helden. Ein kombinierter Release-Kernel-Test schickt die Assets-Fixture und einen realistisch identifizierten Legacy-Helden durch Speicherung, Retrieval und Faktenantwort. Getestet sind richtige und falsche Entity, Feld, Alias, Patch, Modus, Zahl, Provenienz sowie gleichfeldige Legacy/Assets-Konflikte. Der unabhängige fachliche Review und der abschließende unabhängige Bug-/Security-Nachreview gaben auf dem gemeinsamen Code **JA**. Der automatische `gate_hook.py --review` lieferte auf dem gemeinsamen Diff wegen eines Grok-Rate-Limits keinen Codebefund; der unabhängige Nachreview deckte diesen fehlenden Codebefund ab.

Die lokalen gezielten Tests für `brain-contracts`, `brain-legacy-import`, `brain-feeds`, `brain-kernel` und `dbrain-retrieval` bestanden auf dem Integrations-Head `8f247b0` mit Rust 1.97.1. Der abschließende Head `05f5231` schließt zudem den über frei benannte Metadaten-Schlüssel möglichen Alias-Bypass; der unabhängige Bug- und Security-Nachreview gab dafür **JA**. Der echte Tombstone-Test lief auf `05f5231` erneut am vorhandenen Dump in einem isolierten PostgreSQL-16-Cluster: 1 253 Basisdokumente; je ein Entity- und Patch-Tombstone; Wiederholung 0/0; leere Entity-Quelle fail-closed mit 1 253 unveränderten Heads; Herkunftsprüfungen ohne Fehler. Auf diesem finalen Head sind `cargo fmt --all -- --check`, Workspace-Clippy mit `-D warnings`, `cargo test --workspace --locked --offline` und `cargo build --workspace --release --locked --offline` grün. Die GitHub-CI war vollständig grün. Der C1-Job führte mit einem echten `brain-serve`-Prozess gegen Wegwerf-PostgreSQL je 600 Requests bei 8, 16 und 32 Workern aus: jeweils 600 beantwortet, null Client-Fehler, jeweils vier beobachtete Reader-Verbindungen; kein „too many clients“. PR #55 wurde ausschließlich nach `migration/rust-integration` integriert; Merge-Commit `ee4889eae837ebe40ca06543e326824b915ace27`. Die DB-/Serve-/Wiki-Spezialskripte wurden nicht lokal wiederholt, weil ihre bestehende Laufzeitkonfiguration über Environment-Variablen der Workspace-Regel widerspricht. Ein alter 18/18-E2E-Nachweis wird nicht als Test dieses neuen Heads ausgegeben.

### Zwischenzeitlich extern gemergte Provider-PRs

Während dieses Reviews wurden Patchnotes-PR #49, Steam-PR #73 und Schema-PR #461 über den GitHub-Account `EarlySalty` nach den jeweiligen `main`-Branches gemergt (19:14–19:16 UTC). Diese Merges und ein Deploy waren kein Schritt dieser Pre-G5-Arbeit. Der finale Patchnotes-Head `0c9fd6f` ergänzt Infisical-/systemd-Credential-Zugriff und begrenzte reine DB-Lesezugriffe; der Feed-Vertrag und die Brain-Fixture bleiben semantisch konsistent. Lokal bestanden 107 Unittests und 187 Pytest-Tests plus 30 Untertests. Vier geänderte Dateien sind bei `ruff format --check` rot; die GitHub-Security-Jobs wurden wegen Billing nicht gestartet (`runner_id=0`, keine Schritte), GitGuardian ist grün. Das ist kein belegter Security-Codefehler, aber kein CI-Grün-Nachweis.

Der finale Steam-Head `c12e9ab` ergänzt gegenüber dem zuvor geprüften `42df79c` ausschließlich die Ablehnung von `payload.hero_build_id` mit HTTP 422 vor dem Queue-Insert und den zugehörigen Test. Persistente Transaktion/Unique Constraint, Auth, Body-Grenze, Status und Zeitstempel blieben unverändert; der Squash-Merge-Baum entspricht dem PR-Head. Auf dem gemergten Stand bestanden Formatierung, Clippy und zwölf fokussierte HTTP-Tests. Der DB-abhängige Volltest ohne Environment-Konfiguration und der echte PostgreSQL-Race-/Restart-Test auf diesem **neuen** Head sind nicht erneut belegt. Der Schema-Merge enthält dieselbe Migration wie PR #461. Kein Build wurde von dieser Arbeit in Steam veröffentlicht.

### Maßgebliche Bewertung dieser Fortsetzung

Die folgenden Marker gelten für den Integrationsstand nach PR #55 und ersetzen die gleichnamigen historischen Marker weiter oben. „Bereit“ meint hier nur den belegten technischen Pfad, keine G5- oder Produktionsfreigabe; `SHEET_CORE_PATH_READY` bezieht sich auf die fünf erforderlichen V1-`starting_stats`-Felder, nicht auf sämtliche abgeleiteten Sheet-Kennzahlen. Die Patchnotes-Security-Jobs wurden von GitHub wegen Billing gar nicht gestartet; die Steam-Vollsuite verlangt eine nach Workspace-Regel unzulässige Environment-Konfiguration. Match, Meta, Population und YouTube sind dokumentiert, aber ohne Betreiberentscheidung und vollständige Implementierung nicht G5-fertig. PR #40 bleibt Draft, der supersedete Cutover-Branch bleibt unberührt, und kein Consumer wurde aktiviert.

AUTHORITATIVE_INTEGRATION_COMMIT: ee4889eae837ebe40ca06543e326824b915ace27

DETACHED_44B5A59_PRESERVED: JA

PATCHNOTES_PROVIDER_IMPLEMENTED: JA
PATCHNOTES_PROVIDER_CI_GREEN: BLOCKED_BY_GITHUB_BILLING

STEAM_PROVIDER_IMPLEMENTED: JA
STEAM_PROVIDER_PERSISTENT_IDEMPOTENCY: JA
STEAM_PROVIDER_TESTS_GREEN: NEIN

FACT_RELEVANCE_HARDENED: JA

SHEET_CORE_PATH_READY: JA
YOUTUBE_V1_DECISION_REQUIRED: JA
MATCH_META_PATH_READY: NEIN

LEGACY_CORE_MIGRATION_READY: JA
BRAIN_DB_ISOLATED: JA
600_REQUEST_TEST_PASSED: JA
DEFAULT_E2E_PASSED: NEIN

G1_READY: JA
G2_READY: NEIN
G3_READY: NEIN
G4_READY: NEIN

TECHNICALLY_READY_FOR_G5_REVIEW: NEIN
PRODUCTION_CUTOVER_READY: NEIN
