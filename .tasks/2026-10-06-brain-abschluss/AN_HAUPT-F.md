# Paket F: Publish-Regel umgesetzt, reale Veröffentlichung noch offen

## Code und gemeinsame Datenbasis

F baut regulär auf E `f3c84fb4ee442196964387347773a75d704ebafd` auf. Der vorübergehend fehlende Storage-Leser ist damit übernommen. Keine E-Dateien kopiert oder verändert, keine Unterthreads und keine Sessionkoordination.

Aktuelle Hero-, Item-, Ability-, Waffen- und Flexslotwerte kommen ausschließlich über `brain_storage::asset_mirror::{latest_mirrored_client_version, load_mirrored_assets}`. Ein gemeinsamer Modellladegang bindet alle Werte an dieselbe Clientversion. Bestehende mechanische Konverter und API-Klassifikation wiederverwendet. Der neueste Assets-Abgleich muss genau diese vollständige Version erfolgreich bestätigen. Ein fehlgeschlagener oder laufender neuerer Abgleich ist kein aktueller Wertebeleg. Prüfung nach Patchbeginn ist nötig; unveränderte Originalwerte dürfen früher erfasst worden sein. Clientversion ist kein behaupteter historischer Balancepatch.

Abgebaut: 100er-Matchgrenze, verpflichtende Familienfreigabe, Staple-Kernzulassung, pauschaler Staple-Vorrang in Kandidaten und Beam, Matchmangel als eigene Confidence-Abwertung, produktive Rohspielerbeobachtungen, eigener Snapshot-Membership-Guard und Item-Card-/Katalogwerte-Fallbacks für aktuelle Werte. Bestehende Inventar-, Skill- und Mechanikprüfungen bleiben. Der neue mechanische Skillorder-Fallback braucht keine Autoren oder Population. Keine Rohmatches gelesen, importiert oder gespeichert.

Reguläres HTTP-Publish, Wiederaufnahme und Queue verlangen dieselbe aktuelle Abnahme. Gespeicherte reguläre Anfragen tragen den ursprünglichen Build-Kontext und müssen exakt dazu passen; Wiederaufnahme rechnet nicht neu und ruft keine KI. Alte Dateien ohne Build-Provenienz senden nicht regulär. Review bleibt sichtbar getrennt, kein Review-Ausweichpfad für Warden.

G nutzt später den vorhandenen Reasoner als Build-Werkzeug. Keine G-Rechenschicht, kein neuer Antwortdienst oder zusätzlicher Wertespiegel in F. F-Dateigrenzen und aktuelle technische Belege stehen in `../2026-10-07-f-publish/REVIEW.md`.

## Verifikation

Finale Suite mit Reasoner, CLI und Storage: 460 passed, 0 failed, 23 bestehend ignored, 0 filtered out. Reasoner: 286 passed, 17 ignored. Striktes Clippy der beiden Zielcrates ohne fremde Dependency-Lints, Rustfmt aller neun eigenen Rust-Dateien und eigener optimierter Debug-Build jeweils Exit 0. Keine Bestandstests gelöscht, abgeschwächt oder neu ignoriert. Ignorierte Postgres-/Live-Tests sind nicht als gelaufen gemeldet.

## Echter Warden-Lauf und verbleibende Grenze

Regulärer `reason build Warden --no-ai --json --publish` mit finalem Leserstand, Exit 1:

```text
Datenfehler: API-Spiegel: Kein vollständiger lokaler Assets-Spiegel vorhanden
```

Lesender SELECT in `deadlock`: 0 vollständige erfolgreiche versionierte Assets-Runs. Keine Veröffentlichung und keine `hero_build_id`. Kein eigener Ingest oder Tick, kein DB-Handeingriff und kein Review-Bypass. Der vorhandene Secret-Launcher wurde ohne Ausgabe von Secretwerten verwendet.

Der alte mechanische Warden-Entwurf wich nachweislich vom häufigen Kaufmuster ab: Opening Rounds und High-Velocity Rounds nicht Kern, Fleetfoot nur optional, mehrere seltene Kandidaten und zwei Items ohne Populationseintrag. Vergleich ausschließlich bestehender Kaufaggregate, keine Einzelmatches und kein Siegquotenversprechen. Dieser alte Vergleich ist kein aktueller Publish-Beleg; Details in `../2026-10-07-f-publish/WARDEN_BELEG.md`.

## Gate und Auslieferungsgrenze

Finaler Featurecommit, eigenes Gate gegen E und Feature-Push noch offen. Kein ALLOW behauptet. Release-Hold bleibt verbindlich; allgemeines Stop-Hook-Feedback hebt ihn nicht auf. Integration nach Freigabe E, F, G. Release, Installation, Neustart und Ingest/Tick bleiben bei live_strecke.

Nach vollständigem lokalen Spiegel regulären Warden-Publish wiederholen und echte Steam-Build-ID belegen. Kein Main-Push, Cleanup oder Self-Settle vor der freigegebenen Integration und dem echten Live-Beleg. Fortsetzungsakte: `../2026-10-07-f-publish/TODO.md`.
