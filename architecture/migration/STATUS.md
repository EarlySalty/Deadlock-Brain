# Projektstatus: Deadlock Brain Rust, Wiki und Daten

Stand: 29.09.2026. Autoritativer integrierter Codehead: `022f8a981c2164f6d8d4302bae2194e100c4f65c` auf `migration/rust-integration`, integriert über PR #59.

Die unabhängige R5-Codeabnahme A+C und das lokale Gesamtgate meldeten GO beziehungsweise ALLOW auf `72db816056fb0ed53810ab77ea4417dc0812e7ca`. Die vollständige lokale Verifikation auf `022f8a981c2164f6d8d4302bae2194e100c4f65c` ist abgeschlossen: Format, Clippy, Workspace-Tests und Release-Build jeweils Exit 0; 1.011 passed, 0 failed, 75 ignored, 0 filtered. Acht zuvor ignorierte Tests wurden separat gezielt bestanden und sind nicht in den 1.011 enthalten. Im Prozess-E2E antworteten bei 8, 16 und 32 Workern jeweils 600/600 Anfragen; Poolmaximum 4. G5-Freigabe bleibt NEIN. Details: Koordinationsbericht `FINAL-VERIFICATION.md`.

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
| Consumer | Lokale Brain-Consumer-Verträge sind integriert. Twitch #984 war vor F2 gemergt und wurde hier nur regressionsgeprüft. Bots #459, Docs #4 und 2nd-Brain #2 sind ungemergt. | Kein produktiver Consumer aktiv. Einzelne frühere Consumer-CI-Läufe starteten wegen Billing nicht. |
| Patchnotes und Steam | Patchnotes-Provider bleibt der bestehende Python-Legacy-Dienst. Steam-Build-Publish-Provider PR #73 und Diagnosefix PR #82 (`f509f85e`) sind integriert. | Kein Deployment, Neustart oder echter Publish; kein Production-Cutover. |

## Gate-Status

| Gate | Status | Begründung |
|---|---|---|
| G0 | teilweise belegt | Das Inventar und der DL-Main-Snapshot vom 26.09. bleiben stichtagsbezogene Nachweise. SLO, Lastprofil und externe Rechte sind nicht freigegeben. |
| G1 | bestanden (lokal) | Verträge und Kernpfade sind im integrierten Code vorhanden. Abschlussverifikation auf `022f8a9`: alle vier Workspace-Gates Exit 0; Details in `PRE_G5_TECHNICAL_REVIEW.md`. |
| G2 | teilweise | Offline-Prüfungen und ein lokaler Legacy-Release liegen vor. Wiki-Realpilot, Providerfreigabe und Replayentscheidung fehlen. |
| G3 | teilweise | Match-, Assets- und Analytics-Pfade sind im Brain-Kern vorhanden. Twitch #984 ist gemergt und nur regressionsgeprüft; Bots #459, Docs #4 und 2nd-Brain #2 sind ungemergt. Steam-Build-Publish-Provider ist offline geprüft und integriert, aber nicht ausgerollt oder publiziert. Patchnotes bleibt Python-Legacy; Legacy-Writer bleiben bis G5 aktiv. |
| G4 | bestanden (lokal) | Verifikation auf `022f8a9`: vier Workspace-Gates Exit 0, 1.011 passed, 75 ignored; acht ignorierte Fälle separat bestanden. Last 600/600 bei 8/16/32 Workern, Poolmaximum 4. GitHub-CI ist separat nicht vollständig grün und kein Merge-Gate. |
| G5 | NEIN | Kein Production-Cutover und keine Betreiberfreigabe. |
| G6 | offen | Betriebskontrolle und Legacy-Ende wurden nicht gestartet. |

## Finale Verifikation und historische Nachweise

Die finale Verifikation lief auf `022f8a981c2164f6d8d4302bae2194e100c4f65c`. Die vier Workspace-Gates waren erfolgreich: Format, Clippy mit `-D warnings`, Workspace-Tests und Release-Build, jeweils Exit 0. Ergebnis: 1.011 passed, 0 failed, 75 ignored, 0 filtered. Acht gezielt aktivierte ignorierte Fälle bestanden separat und sind nicht zu den 1.011 addiert. Im Prozess-E2E wurden bei 8, 16 und 32 Workern jeweils 600/600 Antworten erzielt, bei beobachtetem Poolmaximum 4.

Die Werte 943, 958 und 980 Workspace-Tests, 18/18 E2E sowie frühere 600-Request-Läufe gehören zu früheren Codeheads. Sie sind keine finale Abnahme von `022f8a9`. Einzelne historische Prüfungen bleiben als damalige Evidenz in [PRE_G5_TECHNICAL_REVIEW.md](PRE_G5_TECHNICAL_REVIEW.md), [FINAL_LOCAL_INTEGRATION_REVIEW.md](FINAL_LOCAL_INTEGRATION_REVIEW.md) und [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md) dokumentiert.

Die frische GitHub-CI auf `022f8a9` ist nicht vollständig grün. Im Lauf `36591388159` scheiterten Core-Matrix und integrierte Wiki-, Source- und Replay-Suiten vor Teststart beim Fetch des exakten `haste_core`-Pins `bfb292d4798031350861ad297aa26753267a1ea6`; Core-Job `109485085537` bestätigt den Revision-Fetchfehler. Das ist kein Billing-Fehler. Fehlende Artefakte waren Folgefehler. Scratch-Pilot, Migration composition, Rust compile, Wiki contracts and offline regression sowie integrierte Audit-Checks liefen erfolgreich; Semantic Review wurde übersprungen. Consumer Offline Gate und GitGuardian meldeten FAILURE. Details stehen im Koordinationsbericht `FINAL-CI.md`. GitHub Actions sind kein Merge-Gate. Die `dungers`-Lizenz bleibt unbelegt; lokale Offline-Gates sind keine frische Reproduzierbarkeit.

Datenkopie und Instanzdetails: [BRAIN_DB_MIGRATION_REPORT.md](BRAIN_DB_MIGRATION_REPORT.md), [BRAIN_POSTGRES_ISOLATION.md](BRAIN_POSTGRES_ISOLATION.md). Pfadverantwortung: [PFAD_OWNER.csv](PFAD_OWNER.csv). Genaue Gates: [GATES.csv](GATES.csv).
