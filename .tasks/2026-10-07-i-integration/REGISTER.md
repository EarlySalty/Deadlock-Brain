# Paket I: Register

status: blockiert, gemeinsamer E-Kandidat in Runde 18 BLOCK; Fixer 13 durch Werkzeug-Schutzgrenze gestoppt, qualifizierte Fachrückgabe am 07.10.2026

Intent-/Auftragsthread: `8827da25-c1f8-44f2-bef8-f3a7b7dd3137`. Delegator: `481426fe-b477-42b3-91c6-901811fcba1d`. Gestoppter Haupt-Orchestrator `d3a1741e-82bc-4a48-865b-2845c663dca7` wird nicht reaktiviert. Fachrückgabe ausschließlich als `AN_HAUPT-I.md`, keine Sessionkontakte oder neuen Threads.

| Bereich | Arbeitsbaum | Branch | bestätigter Ausgang | Status |
| --- | --- | --- | --- | --- |
| E | `/home/nathanael/.worktrees/brain-e-deadlock-api` | `feat/brain-deadlock-api-daten` | `d3d3c5b68bea07a3e11c09572b200bbefd7b55cf` | Produktstand unverändert; Fixer 13 ohne Codeänderung durch Projektroot-Schutzgrenze gestoppt |
| F | `/home/nathanael/.worktrees/brain-f-publish` | `feat/brain-build-publish-ohne-matchgrenze` | `46fd86743589910d7b92a7223bdd6ab0dcf2b7c8` | übernommen, Produktcode unverändert |
| Integrationskandidat | `/home/nathanael/.worktrees/brain-i-release-20261007` | `feat/brain-i-integration-blocked-20261007` | `501d3725e691c713f4b04468fd9d6b77977ae91c` | sauber einschließlich ignorierter Dateien, auf eigenem Remote-Branch gesichert; Gesamt-Gate BLOCK |

## Aktuelle begrenzte Fortsetzung

Entscheidung `ENTSCHEIDUNG-I-PATCH-ORIGINAL.md`, 07.10.2026, 16:09 UTC. Native Fixer 11 und 12 abgeschlossen: konkrete Steam-Originalbindung, HTML-Linkreste und fragmentfreier Bestandslookup korrigiert. Begrenzter Fix b70dc6b6 erhielt ALLOW mit unverändert Claude Opus 5.5. Keine Gesamtfreigabe daraus abgeleitet. Beide Fixes und die erhaltene Core6-Gegenprobe in Kandidat 501d3725, Tree `4cca98fe791105203b8951a4f24c6c4abaf43cab`, übernommen und gemeinsam geprüft.

Gemeinsame Suite: 558 passed, 0 failed, 25 ignored. Format und striktes Clippy einschließlich brain-serve Exit 0. Zusätzlich vorhandene ignorierte Bestands-ID-Scratchprobe ausdrücklich am Kandidaten ausgeführt: 1 passed, 0 failed, 0 ignored, 116 filtered. Fehlende Scratch-DSN im Erstlauf offen dokumentiert; anschließend eigene isolierte Unixsocket-Postgresinstanz ohne TCP genutzt und wieder beendet. Befehle, Logs und Grenzen in `PRUEFUNG-KANDIDAT-501.md`.

TESTNACHWEIS[TW-1]: 558 passed, 25 ignored | Baseline: keine Altfehler behauptet

Gemeinsamer Produkt-Gate gegen `ca4d877f13042c9a7a7023e54f6bf2c688b69ac4`, unverändert Claude Opus 5.5: BLOCK, Exit 1. Neuer berechtigter Lookup-Kern: Queryfreie bestehende Patch-URL wird bei queryhaltigem Feedlink nicht gefunden; auch konkrete Slash-/gleichartige Ereignis-URL-Varianten prüfen. Tatsächlichen SQL-Lookup und abweichende Queryentfernung in post_row nachgelesen. Originalurteil und Einordnung in `REVIEW-RUNDE-18.md`; frühere Runden bleiben in `REVIEW.md` erhalten.

Frischer nativer Fixer 13, Briefing `FIXER-13-AUFTRAG.md`, ausschließlich bestehender Patch-/URL-/Bestandslookup und echte zugehörige Regressionen. Unterschiedliche Steam-Ereignis-/Announcement-GIDs nicht vermischen, Netz- und Herkunftsgrenzen unverändert. NITs zu unerreichbarem altem Original und Bildlinks bleiben separat offen. G/K-Prüfsperren werden nicht übernommen oder umgangen. Bislang zwei tatsächliche BLOCKs in der neuen begrenzten Fortsetzung; spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe.

## Schutzblocker des Fixers 13

Der frische native Fixer wurde beim Zugriff auf `/home/nathanael/.worktrees/brain-e-deadlock-api/rust/crates/deadlock-brain/src/pg_patchnotes.rs` durch die Werkzeug-Schutzgrenze gestoppt: außerhalb des zugelassenen Projektroots. Ausgang und Branch bestätigt, Graphify abgefragt; keine Änderungen, Commits, Scratchprobe oder Selbst-Gate. Keine aktive eigene Native-Arbeit mehr. Hauptsession bestätigt unveränderten Produkt-HEAD und ausschließlich eigene Aufgabenakten als WIP.

Kein anderer Zugriffsweg, Worker, Toolwrapper, eigener Ersatzfix oder Eingriff in Hooks/Berechtigungen. Dies ist ein tatsächlicher Prüf-/Zugriffsblocker, kein dritter inhaltlicher Gate-BLOCK. Qualifizierte Fachrückgabe in `AN_HAUPT-I.md` und `REVIEW-RUNDE-19-SCHUTZBLOCKER.md`. Für Fortsetzung muss der reguläre frische Fixerkontext den bestehenden E-Worktree als zulässigen Projektroot erhalten; die Hauptsession nimmt keine Schutzlockerung vor. Zentrale Akte bleibt beim Delegator.

## Main und ausstehender Abschluss

Frühere Belegintegration `17974c66` und Schemapinintegration `ca4d877f` separat ALLOW und tatsächlich nach main gepusht. Der vollständige E-Produktstand weiterhin nicht nach main gepusht. Kein Merge bei BLOCK.

Kein regulärer Releasebuild/install, eigener Neustart, produktiver vollständiger Assets-/Patch-/Builddatenimport oder Live-Receipt-/Originalhashbeweis. Read-only-Rust-Liveprobe erneut gegen Kandidat 501 gebaut, Exit 0, noch nicht live ausgeführt. Eigene geprüfte Cargo-Artefakte ohne Löschung nach `/tmp/brain-i-verified-target-20261007.vYUnv5/target` verschoben, damit reguläre Releasequelle tatsächlich vollständig sauber ist. Wrapper und Hooks unverändert.

Keine analytics_runtime-Freigabe. F-Budget/Imbues/ursprünglicher Abbruch und dieselben PurchasePlan-/InventoryEvaluation-Belege noch nicht angeschlossen. Kein F/G-Vertragsbeweis, keine neue Warden-Veröffentlichung oder hero_build_id. F-Baseline am unveränderten 46fd8674: Compiler Exit 0; Composer 28 passed, Planner 8 passed, jeweils 0 failed und 0 ignored. Kein F/G-Anschluss daraus ableiten, keinen heutigen ungeprüften G-Stand behaupten.

Letzter Dienstvorcheck: brain-serve PID 2645590, active; brain-maintenance PID 0, failed, vorbestehender Zustand. Vor tatsächlichem Deploy aktuellen origin/main und Live-Binary-SHA erneut prüfen. Reale DotEnv-CLI-Probe wegen R12 weiterhin ausgelassen, keine Schutz-Hooks umgangen.

Kein Cleanup oder Self-Settle. Eigene Kandidaten/Branches erhalten. Vor späterem Cleanup 20 vorhandene F-Prüf-/Publikationslogs, ignorierte Artefakte und laufende Binary-Pfade prüfen. Fremder kanonischer Branch und WIP unberührt; nur zugewiesenen Hauptbericht nach tatsächlichem Ergebnis ergänzen. Zentrale Akte bleibt beim Delegator.
