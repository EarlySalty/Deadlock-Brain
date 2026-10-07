# Paket F: Build-Veröffentlichung

## 07.10.2026, 05:25: Schnittstelle zu G

F bleibt bei der beauftragten Publish-Regel. Kein neuer Antwortdienst, Dokumentweg oder Parser. G kann später `dbrain_reasoner::reason_build_with_options` als Build-Werkzeug nutzen und den vorhandenen Steam-Publisher aufrufen. F ersetzt weder die G-Rechenschicht noch deren Entitätenmodell. Der gemeinsame Publish-Guard prüft aktive API-Spielwerte, Skills und Kaufkurve, ohne Matchzahl oder Familienfreigabe.

Offene E-Schnittstelle: `client_version` steht im Dokument unter `metadata.adapter.client_version`, Snapshots verweisen über `source_document_id`. Der vorhandene Store behält bei identischem Payload die alte Snapshot-Zeile einschließlich alter Dokumentbindung und Fetchzeit. Damit ist die Zugehörigkeit unveränderter Werte zum neuen Spiegel noch nicht durch den Snapshot allein belegt. F ändert E-Dateien nicht. Eine belegbare aktuelle Spiegelmitgliedschaft wird für den Publish-Guard benötigt; kein eigener Wertespiegel in F.

Der kanonische Bericht im geteilten Checkout wurde gelesen. Sein Update wurde vom Worktree-Isolationsschutz verweigert. Dieser Bericht liegt deshalb im eigenen Worktree und wird mit dem Featurestand gesichert.

## Stand

Eigener Worktree `/home/nathanael/.worktrees/brain-f-publish`, Branch `feat/brain-build-publish-ohne-matchgrenze`, Basis `bfda408c`. Keine Unterthreads.

F ändert Reasoner-Publish, Mechanikplanung ohne Familien-/Populationspflicht, Confidence und Regressionen. `deadlock-brain/src/main.rs` bekommt den bestehenden Pool für die aktuelle Prüfung vor HTTP-Publish. Keine Änderung an A-Antwortlogik, E-Ingest oder G-Rechenschicht.

Abbau: 100er-Publishgrenze, verpflichtende Familienabnahme, Staple-Zulassung als Kernfilter und Matchmangel als Confidence-Abwertung. Produktive Planung lädt keine Rohspielerbeobachtungen mehr. Historische Rohmatchablagen werden nicht erweitert; deren Abbau liegt bei E.

Zusätzlicher Zwillingspfad vor Änderung: `reason resume-publish` sendet gespeicherte Anfragen bisher ohne aktuelle Patchprüfung. F legt den bestehenden Build-Kontext neben der unveränderten Anfrage in der gespeicherten Datei ab. Der gemeinsame HTTP-Sender prüft vor jedem regulären Senden denselben Build erneut gegen die lokale Datenbasis, ohne neu zu planen oder KI aufzurufen. Alte Dateien bleiben lesbar, dürfen ohne Build-Provenienz aber keine neue Veröffentlichung auslösen. Review bleibt ausdrücklich getrennt. Betroffen ist ausschließlich `deadlock-brain/src/main.rs`.

Release-Hold am 07.10.2026 erneut in `A/RELEASEFENSTER.md` geprüft: aktiv. Kein Main-Push, Brain-Release, Install, Neustart oder Tick. Eigene Prüfungen erfolgreich: 401 passed, 0 failed, 19 bestehend ignored; Format aller acht eigenen Rust-Dateien, striktes Clippy der eigenen Zielcrates und eigener optimierter Debug-Build jeweils Exit 0.

## Echter Warden-Lauf: Blocker statt Erfolgsmeldung

Regulärer `--publish` mit finalem Binary scheitert vor dem HTTP-Senden, Exit 1: `Datenfehler: API-Spielwerte sind nicht für den aktiven Patch belegt.` Keine `hero_build_id` und keine Veröffentlichung behauptet. Der Warden-Snapshot vom 02.10.2026 ist älter als der aktive Patch vom 05.10.2026; seine dokumentgebundene Clientversion fehlt. Auch der neueste erfolgreiche Assets-Run vom 06.10.2026 trägt noch keine neue Versions-/Vollständigkeits-Summary. Konkrete SELECT-Belege und der Vergleich mit vorhandenen Kaufaggregaten stehen in `WARDEN_BELEG.md`.

Der mechanische Entwurf enthält unter anderem seltene Kandidaten und zwei Items ohne Populationseintrag. Häufige Opening Rounds und High-Velocity Rounds sind nicht Kern, Fleetfoot nur optional. Das ist ein belegter Unterschied im Kaufmuster, kein Siegquotenversprechen und noch kein gültiger Publish-Beleg. Keine Rohmatches gelesen oder gespeichert.

Offener Abschluss: aktuelle E-Spiegelmitgliedschaft herstellen, denselben regulären Publish erneut abnehmen und echte Build-ID belegen. Kein eigener E-Fix, kein DB-Handeingriff, kein Review-Bypass. Gate und Feature-Sicherung folgen für den geprüften F-Stand.
