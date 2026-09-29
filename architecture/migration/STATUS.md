# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Stand: 29.09.2026. Autoritativer integrierter Codehead: `022f8a981c2164f6d8d4302bae2194e100c4f65c` auf `migration/rust-integration`, integriert über PR #59.

Die unabhängige R5-Codeabnahme A+C und das lokale Gesamtgate meldeten GO beziehungsweise ALLOW auf `72db816056fb0ed53810ab77ea4417dc0812e7ca`. Das ist keine vollständige Workspace-, Release-, PostgreSQL- oder Lastabnahme des integrierten Heads. Der tatsächliche Abschlusslauf auf `022f8a9` ist noch nicht durch `FINAL-VERIFICATION.md` belegt. Bis dieser Bericht vorliegt, bleiben die finalen Testmarker offen.

Kein Production-Cutover, keine produktiven Consumer aktiviert, kein Brain-Merge nach `main`. PR #40 bleibt Draft. Der neue Kern ist nicht produktiv aktiv; der Bestandspfad läuft weiter.

## Aktueller Produkt- und Freigabestand

| Bereich | Aktueller Stand | Grenze oder offener Nachweis |
|---|---|---|
| Match | Private, accountgebundene Daten laufen über den API-Adapter, `SourceRecordV2`, den normalen Store und Release. Identität, Hashprojektion und atomarer Revoke sind Teil des Pfads (`rust/crates/brain-feeds/src/bin/brain-match-ingest.rs:96-107,141-168,170-218`; `rust/crates/brain-feeds/src/deadlock_match.rs:30-80,403-495`). | Kein Production-Cutover. |
| Assets | Assets-Core deckt die V1-Startwerte ab (`rust/crates/brain-feeds/src/deadlock_assets.rs:19-26,41-62,64-92). | Kein Sheet-Fallback. |
| Meta und Population | Typisierte Fakten; die Item-Matchquote stammt aus `hero-stats` (`rust/crates/dbrain-sources/src/analytics_runtime.rs:23-48,103-123,186-213`). | Die Quelle attestiert keine Patchzugehörigkeit. Patchgebundene Anfragen bleiben fail-closed. Keine automatisch freigegebene Buildempfehlung. |
| Deadline und Antwort | Gemeinsames Requestbudget mit abschließendem Kernel-Guard. Der R5-Nachweis auf `72db816` umfasst positive und negative lokale Prüfungen. | Dieser Reviewnachweis ersetzt nicht den ausstehenden vollständigen Lauf auf `022f8a9`. |
| Wiki | Normaler Store- und Releasepfad offline geprüft. | `WIKI_REAL_PILOT_PASSED=NEIN`, da Quellen-, Lizenz- und Aufbewahrungsfreigabe fehlen. |
| Provider | Kein freigegebener Provider-Shadow. | `PROVIDER_SHADOW_PASSED=NEIN`, da Anbieter-, Modell-, Egress- und Budgetfreigabe fehlen. |
| Replay | Kein freigegebenes `.dem`; Decoder und Reportpfad bleiben offline. | Replay V1 oder später ist Betreiberentscheidung. |
| Consumer | Lokale Brain-Consumer-Verträge sind integriert. Twitch #984 ist Regression, keine Aktivierung. Bots #459, Docs #4 und 2nd-Brain #2 sind nicht gemergt. | Kein produktiver Consumer aktiv. Private Consumer-CI startete teilweise wegen Billing nicht. |
| Patchnotes und Steam | Patchnotes-Provider bleibt der bestehende Python-Legacy-Dienst. Steam-Build-Publish-Provider PR #73 und Diagnosefix PR #82 (`f509f85e`) sind integriert. | Kein Deployment, Neustart oder echter Publish; kein Production-Cutover. |

## Gate-Status

| Gate | Status | Begründung |
|---|---|---|
| G0 | teilweise belegt | Das Inventar und der DL-Main-Snapshot vom 26.09. bleiben stichtagsbezogene Nachweise. SLO, Lastprofil und externe Rechte sind nicht freigegeben. |
| G1 | teilweise belegt | Verträge und Kernpfade sind im integrierten Code vorhanden. R5-GO gilt für das geprüfte Delta auf `72db816`; die Abschlussverifikation des gesamten Heads `022f8a9` steht aus. |
| G2 | teilweise | Offline-Prüfungen und ein lokaler Legacy-Release liegen vor. Wiki-Realpilot, Providerfreigabe und Replayentscheidung fehlen. |
| G3 | teilweise | Match-, Assets- und Analytics-Pfade sind im Brain-Kern vorhanden. Consumer sind nicht aktiviert. Steam-Build-Publish-Provider ist integriert, aber nicht ausgerollt oder publiziert. Patchnotes bleibt der bestehende Python-Legacy-Dienst; Legacy-Writer bleiben bis G5 aktiv. |
| G4 | ausstehend für `022f8a9` | Frühere Isolation-, Backup/Restore-, E2E- und Lastwerte sind historische Nachweise. Keine davon gilt als Abschlusslauf auf `022f8a9`. |
| G5 | NEIN | Kein Production-Cutover und keine Betreiberfreigabe. |
| G6 | offen | Betriebskontrolle und Legacy-Ende wurden nicht gestartet. |

## Historische Nachweise

Die Werte 943, 958 und 980 Workspace-Tests, 18/18 E2E sowie frühere 600-Request-Läufe gehören zu früheren Codeheads. Sie sind keine finale Abnahme von `022f8a9`. Einzelne historische Prüfungen bleiben als damalige Evidenz in [PRE_G5_TECHNICAL_REVIEW.md](PRE_G5_TECHNICAL_REVIEW.md), [FINAL_LOCAL_INTEGRATION_REVIEW.md](FINAL_LOCAL_INTEGRATION_REVIEW.md) und [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md) dokumentiert.

Die frische GitHub-CI auf `022f8a9` ist nicht vollständig grün. Im Lauf `36591388159` scheiterten Core-Matrix und integrierte Wiki-, Source- und Replay-Suiten vor Teststart beim Fetch des exakten `haste_core`-Pins `bfb292d4798031350861ad297aa26753267a1ea6`; Core-Job `109485085537` bestätigt den Revision-Fetchfehler. Das ist kein Billing-Fehler. Fehlende Artefakte waren Folgefehler. Scratch-Pilot, Migration composition, Rust compile, Wiki contracts and offline regression sowie integrierte Audit-Checks liefen erfolgreich; Semantic Review wurde übersprungen. Consumer Offline Gate und GitGuardian meldeten FAILURE. Details stehen im Koordinationsbericht `FINAL-CI.md`. GitHub Actions sind kein Merge-Gate. Die `dungers`-Lizenz bleibt unbelegt; lokale Offline-Gates sind keine frische Reproduzierbarkeit.

Datenkopie und Instanzdetails: [BRAIN_DB_MIGRATION_REPORT.md](BRAIN_DB_MIGRATION_REPORT.md), [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md). Pfadverantwortung: [PFAD_OWNER.csv](PFAD_OWNER.csv). Genaue Gates: [GATES.csv](GATES.csv).
