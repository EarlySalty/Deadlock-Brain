# Paket I: Register

status: blockiert, qualifizierte Fachrückgabe am 2026-10-07

Intent-/Auftragsthread: `8827da25-c1f8-44f2-bef8-f3a7b7dd3137`. Delegator: `481426fe-b477-42b3-91c6-901811fcba1d`. Haupt-Orchestrator: `d3a1741e-82bc-4a48-865b-2845c663dca7`. Übergabe ausschließlich als `AN_HAUPT-I.md`, keine Sessionkontakte.

| Bereich | Arbeitsbaum | Branch | bestätigter Produktstand | Status |
| --- | --- | --- | --- | --- |
| E | `/home/nathanael/.worktrees/brain-e-deadlock-api` | `feat/brain-deadlock-api-daten` | `5f4e3668acb4531cfd31efdaa88e63626f772f84` | Produktcode geprüft; zusätzliche Reader-Gegenprobe und aktualisierte Akte, kein Produktfix |
| F | `/home/nathanael/.worktrees/brain-f-publish` | `feat/brain-build-publish-ohne-matchgrenze` | `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8` | übernommen, Produktcode unverändert |
| Integrationskandidat | `/home/nathanael/.worktrees/brain-i-release-20261007` | `feat/brain-i-integration-blocked-20261007` | `b63569afbf2bc6f686564063d769dbbcf0a5ef90` | sauber, auf eigenem Remote-Branch gesichert; Produkt-Gate BLOCK; kein Deploy |

## Tatsächlicher Main-Stand dieser Integration

Belegintegration `17974c66` und Schemapinintegration `ca4d877f` erhielten separat ALLOW und wurden nach main gepusht. Der vollständige E-Produktkandidat `b63569af` wurde nicht nach main gepusht. Kein allgemeines „kein Main-Push“ behaupten. Beide erlaubten Teilstufen bleiben erhalten.

Vor der zusätzlichen Gegenprobe waren E-Tree und Kandidaten-Tree identisch: `c1d4b6a250737142b2f97f240bbad8c7e42d304b`. Am E-Stand mit diesem Tree bestanden 553 Fälle, 0 failed, 24 ignored, 32 Testtargets. Original `/tmp/brain-i-e-main-integration-tests.log`. Format Exit 0 und striktes Clippy einschließlich `brain-serve` Exit 0, Originale `/tmp/brain-i-e-main-integration-fmt.log` und `/tmp/brain-i-e-main-integration-clippy.log`. Diese vollständige Suite stammt vor der zusätzlichen Regression, nicht vom neuen Dokument-/Testcommit.

TESTNACHWEIS[TW-1]: 553 passed, 24 ignored | Baseline: keine Altfehler behauptet

## Aktueller Gate-Restkern

Unverändert Claude Opus 5.5 blockiert den gemeinsamen Produktkandidaten wegen angeblich verdeckter globaler Assets nach späterem Core6-Import. Der konkret genannte Kern ist durch Codeprüfung und echte Scratch-PG-Probe widerlegt: INNER JOIN vor LIMIT; alle 13 Value-/Receipt-Kombinationen bleiben verfügbar und nennen den tatsächlich passenden Original-Run. Finale Gegenprobe 1 passed, 0 failed, 0 ignored, 225 filtered; Format und striktes Clippy Exit 0. `NACHWEIS-CORE6-GLOBAL.md` enthält Befehle, Grenzen und Originalausgabe. Nur Tests ergänzt, Produktcode nicht verändert.

Fünf weitere erfolglose Fortsetzungs-BLOCKs seit Runde 8 sind erreicht (9, 10, 11, 12, 14). Kein Fixer 11, kein Modellwechsel und kein neuer Gate-Anlauf. Das Urteil ist nicht übersteuert. Fachliche Rückgabe: Gate-/Spec-Konflikt anhand der Gegenprobe klären, statt Core6-Kompatibilität zu brechen oder einen bereits vorhandenen Reader-Fallback erneut zu bauen. Details und wörtliches Urteil in `REVIEW.md`. Zehn native Fixer abgeschlossen; keine aktiven eigenen Agenten.

## Noch nicht geliefert

Kein regulärer Releasebuild/install, Neustart, produktiver vollständiger Assets-/Patch-/Builddatenimport oder Live-Receipt-/Originalhashbeweis. Die Rust-Liveprobe ist gebaut, aber nicht ausgeführt. Keine analytics_runtime-Freigabe. F-Budget/Imbues/ursprünglicher Abbruch und dieselben PurchasePlan-/InventoryEvaluation-Belege noch nicht angeschlossen. Kein F/G-Vertragsbeweis, keine neue Warden-Veröffentlichung oder hero_build_id. Kein Cleanup und kein Self-Settle.

F-Baseline am unveränderten `46fd8674`: Compiler Exit 0; Composer 28 passed, Planner 8 passed, jeweils 0 failed und 0 ignored. Originale `/tmp/brain-i-f-baseline-check.log`, `/tmp/brain-i-f-composer-baseline.log`, `/tmp/brain-i-f-planner-baseline.log`. Kein F/G-Anschluss daraus ableiten. G nicht verändert; früher lesend geprüfter committed Stand `54f76ba0` ist keine Behauptung über den heutigen G-Lieferstand.

Letzter Prozessvorcheck: `brain-serve` PID 2645590, exe unter `/opt/deadlock-brain/maintenance-releases/bfda408cb988722ddceadb56bca5b72e12d12731/brain-serve`. Frühere Health-/Ready-Proben waren HTTP 200, JSON, `ok`/`ready`; kein eigener Deploybeweis. Der alte Wrapperplan auf `3ceb504d` ist nach der gestuften Integration nicht mehr gültig. Vor jedem späteren Deploy tatsächlichen aktuellen origin/main und Live-Binary neu prüfen.

Die durch R12 verweigerte DotEnv-CLI-Probe bleibt ausgelassen. Keine Schutz-Hooks umgangen. Vor späterem Cleanup die 20 vorhandenen F-Prüf-/Publikationslogs lokal erhalten und sämtliche ignorierten Artefakte sowie laufende Binary-Pfade erneut prüfen. Fremder kanonischer Branch und fremder WIP bleiben unberührt; einzig zugewiesener Hauptbericht wird dort aktualisiert.
