# Fixer 15: eine fachliche URL-Runde abgeschlossen

status: erledigt, 2026-10-07; begrenzter Selbst-Gate BLOCK, keine Gesamtfreigabe

## Arbeitsstand und Wirkung

Einziger frischer nativer Fixer a7bef6f8941fa27b9, Ausgang 9d17ee52ec898d52ed7c9e78feae596ad086487a, Fix c0e38302cd85c9c650ca07f658af037cedb81124. Commit enthält ausschließlich rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs. Hauptsession bestätigt nach BLOCK zuerst Status und log -1; nur ihre Akten bleiben WIP. Kein weiterer Fixer, zweiter Discoveryversuch, Main-Push oder Deploy durch den Worker.

Bestehender Lookup normalisiert Query, Fragment und abschließenden Slash für Bestand und stabile neue Kennungen. Vorhandene konkrete Steam-Ereignis-GID verbindet Store- und Community-Ereignisalias. Announcement-GIDs bleiben getrennt. Ursprüngliche Herkunfts-URLs bleiben erhalten. Kein zweiter Parser oder Titel-/Zeitbeleg als Identität.

Die vorhandene echte Scratch-PG-Probe wurde um 12 positive Variantenfälle und 7 negative Identitätsfälle ergänzt. Darunter queryfreier Bestand gegen queryhaltigen Feed, Slash in beiden Richtungen, Store gegen gespeicherten Community-Ereignislink und fremde Ereignis-/Announcement-GIDs. Parserannahme, patch_17, Ereignismetadaten und Source-URL werden mit geprüft. Eigene PostgreSQL-16-Instanz ohne TCP, Unixsocket und vorgeschriebener Datenbankname brain_fixer12_patch_lookup, kein brain.source_documents; Instanz am Ende Exit 0 beendet.

## Tatsächliche Prüfungen

Native Rückgabe: bestehende serielle Suite Exit 0, 558 passed, 0 failed, 25 ignored, 0 filtered. Scratch-PG-Probe ausdrücklich --ignored --exact: Exit 0, 1 passed, 0 failed, 0 ignored, 116 filtered. Format, Compilercheck --all-targets, striktes Clippy einschließlich brain-serve und Diffcheck jeweils Exit 0. Cargo über cargo-slot, --jobs 3, SQLX_OFFLINE=true, Produktiv-DSNs entfernt, Tests --test-threads=1.

Ein Zwischenlauf scheiterte tatsächlich an einer eigenen Slash-Herkunftsregression. Produktcode korrigiert, bestehender Test nicht abgeschwächt, vollständige Suite danach grün. Kein Altfehler behauptet. Erstversuch durch falsche MCP-Logwerkzeugwahl beendet; derselbe Fixer setzte dieselbe noch unvollständige fachliche Runde mit dem bereits am Taskstart ausdrücklich vom Nutzer erlaubten normalen Read für Logs fort. Kein Hook, Recht oder Wrapper geändert.

Logs:
- /tmp/brain-fixer15-tests-verified.log
- /tmp/brain-fixer15-scratch-pg-verified.log
- /tmp/brain-fixer15-fmt-success.log
- /tmp/brain-fixer15-check-verified.log
- /tmp/brain-fixer15-clippy-verified.log
- /tmp/brain-fixer15-self-gate-opus55.log

Scratch-, Clippy- und Gate-Originale von der Hauptsession mit normalem Read geprüft. Native Workerrolle untersagte eigene Berichtdatei; diese Akte schreibt die Integrationsverantwortung anhand tatsächlicher Rückgabe.

TESTNACHWEIS[TW-1]: 558 passed, 25 ignored | Baseline: keine Altfehler behauptet
TESTNACHWEIS[TW-1]: 1 passed, 0 ignored | Baseline: ausdrücklich ausgeführte erweiterte Scratch-PG-Probe

## Tatsächlicher Selbst-Gate

Basis 9d17ee52ec898d52ed7c9e78feae596ad086487a, HEAD c0e38302cd85c9c650ca07f658af037cedb81124, --model claude-opus-5-5, Exit 1:

```text
BLOCK: A store URL does not match an existing Steam event URL for the same patch.
1. `rust/crates/deadlock-brain/src/pg_patchnotes/api_sync.rs:149 | BLOCKING: Reverse Steam alias is missing | When the feed supplies `/news/app/1422450/view/G` but the stored URL is `/app/1422450/event/G`, `identities` contains only the store URL. The lookup misses the existing ID and creates a new one. The reverse case, including query and trailing-slash variants, is covered by the newly added but ignored test.`
```

## Verifikation und nächste vorgeschriebene Integrationsprüfung

Die Hauptsession hat nach Graphify den tatsächlichen Fix-SHA nachgelesen: load_post_row ergänzt in Zeilen 150 bis 154 ausdrücklich https://steamcommunity.com/app/1422450/event/{gid}, wenn die bereits vorhandene Ereignis-GID erkannt wird. Die erweiterte echte PG-Probe enthält den vom Gate beschriebenen Storefeed gegen gespeicherten Community-Ereignislink einschließlich Fragment und Query und wurde ausdrücklich ausgeführt, nicht nur ignoriert in der normalen Suite. Der konkrete neue Befund ist damit am vorliegenden Code und echten PG-Weg nicht bestätigt. Das tatsächliche BLOCK bleibt erhalten, wird nicht als ALLOW ausgegeben oder unverändert neu gewürfelt.

Nutzerauftrag verlangt nach genau dieser einen fachlichen Runde einen echten gemeinsamen Gesamt-Gate für den zusammengeführten Spiegel-/Discovery-Kandidaten gegen aktuellen main, mit demselben Urteilmodell. Dieser eigene Integrationsschritt folgt jetzt; keine zweite Produktfixrunde im Worker. Nur ein neuer tatsächlicher inhaltlicher Gesamtfund löst den Discovery-Schnitt aus. Die bisherigen zwei NITs und der Reader-Gegenbeweis bleiben erhalten. Keine eigenmächtige Mergefreigabe aus Gegenbeweis oder begrenzten Tests.

MERGEPROTOKOLL[MS-1]: 16 Git-Schritte einzeln | Anläufe: 1 | Gate: [claude-opus-5-5] begrenzter Selbst-Gate BLOCK, kein Main-Merge
