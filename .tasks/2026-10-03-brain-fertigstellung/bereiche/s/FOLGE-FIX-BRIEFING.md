status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-s

# Folgerunde: sichere CLI-Wiederaufnahme und sichtbare Fehler

Dieses Briefing startet erst nach dem abgeschlossenen Fixer wf_806b96af-dd4. Keine parallelen Schreiber auf dessen Dateien.

## Ziel und Vertrag

Die unabhängige Intent-Abnahme wf_0114aa23-616 hat am Vorfix-SHA 9a6f3d5f3ae2349d9fb076821381e45a421a0bbe drei konkrete Lücken belegt:

1. Die CLI berechnet bei jeder erneuten Ausführung einen neuen Build. Bei standardmäßig aktivierter AI können sich Beschreibungen oder Annotationen ändern und damit eine neue Anfrage-ID erzeugen. Ein unklarer Publish lässt sich so nicht zuverlässig wiederaufnehmen. Fundstellen main.rs:2364 und :2197.
2. Wiederholte HTTP-429 beim Statuspolling werden verworfen; am Ende meldet die CLI timeout statt Drosselung. Fundstelle build_publish.rs:282-289 am Vorfixstand. Der vorherige Fix kann den Aufbau dieser Schleife ändern, den aktuellen Stand zuerst prüfen.
3. Eine reguläre Veröffentlichung mit fehlender Freigabe druckt BLOCKED und beendet sich erfolgreich. main.rs:2301-2311 am Vorfixstand. Das war bereits vorher so; keine Behauptung über eine gemessene Testbaseline. Ausgabe erhalten, Fehlerexit liefern.

Behebe diese konkreten Publish-Lücken ohne Änderung der Freigaberegeln, Datenquellen, AI-Anbieter oder Modelle. Die erste Fixrunde mit Status-vor-POST, GET-Erholung und Endzustand-vor-Frist erhalten.

## Eigentum

Schreibbereich sind genau drei vorhandene Dateien:

- rust/crates/brain-feeds/src/build_publish.rs
- rust/crates/brain-feeds/tests/build_publish_endpoint.rs
- Publish-bezogene Abschnitte und passende Tests in rust/crates/deadlock-brain/src/main.rs

Keine neuen Crates, externen Abhängigkeiten, Migrationen, Lockfileänderungen oder anderen CLI-Bereiche. Native Worker delegieren nicht weiter. Keine T3-Threads. Keine fremden Änderungen verwerfen. Secrets weder klar lesen noch ausgeben oder schreiben, keine Prozessumgebungen lesen.

## Minimale Wiederaufnahme

Zuerst Graphify und bestehenden Rust-Bestand nach einem passenden Anfrage-/State-Speicher fragen. Bestehende Bausteine wiederverwenden. Es braucht einen unveränderlichen gespeicherten brain.build_publish.v1-Request vor der ersten möglichen Außenwirkung und einen expliziten CLI-Wiederaufnahmepfad, der diesen Request nutzt, ohne Reasoner oder AI erneut laufen zu lassen.

Wenn kein passender Speicher besteht: normale lokale JSON-State-Datei, ohne Zugangsdaten, unter dem vorhandenen State-Pfad oder dem üblichen XDG-State-Verzeichnis. Kein stilles Überschreiben, vorhandene Datei nur bei gleichem validiertem Request akzeptieren. Dateien atomar und mit begrenzter Größe lesen/schreiben; keine Rohinhalte oder Serde-Auszüge im Fehlertext. Einen passenden expliziten Resume-Schalter oder Unterbefehl in den bestehenden Reason-Publish-Bereich setzen. Der ursprüngliche Publish muss den gespeicherten Pfad mit seinem Ergebnis nennen, besonders bei UNCONFIRMED. Die neue Hilfe und Nutzertexte auf Deutsch mit echten Umlauten, humanizer und no-em-dashes. Keine neue Produktentscheidung offenlassen, wenn der vorhandene Stil den Standard vorgibt.

Vor GET/POST den gespeicherten Request vollständig validieren. Bestehende ID und finalen Requesthash unverändert nutzen. Bei vorhandenem Endzustand nichts erneut anlegen; queued/running pollen. Fehlende Anfrage darf der persistente identische POST-Vertrag anlegen. Keine automatischen neuen IDs nach Fehler oder Zeitüberschreitung. Der ID-Suffix ist nicht der endgültige request_sha256.

Bei anhaltendem 429 den bestätigten Drosselungsgrund bis zur nicht bestätigten Quittung durchreichen. Ein späterer bestätigter Erfolg bleibt Erfolg. BLOCKED bleibt sichtbar und führt zu einem Fehlerexit, ohne Publish-Aufruf.

## Arbeitsstand und Freigaben

Worktree /home/nathanael/.worktrees/brain-fertig-s auf feat/brain-fertig-s-20261003. Den tatsächlichen HEAD beim Start erheben; er muss die abgeschlossene erste Fixrunde enthalten. Ausgangsbasis bleibt 511a347b653beba13c2bf130f4bead7a7196cc2a. Steam 9aec0cc897b01b74d417ab9b510314cbbbd02535 nicht ändern. GPT-6.1 Sol erben, xhigh.

Feature-Fixcommit der drei eigenen Dateien und Push nach feat/brain-fertig-s-20261003 erlaubt. Kein main-Merge, keine Releaseumschaltung, kein echter Publish, keine Communitynachricht. Z besitzt Integration und Installation. Verwende den Modelltrailer nach rolle-merge-schleuse.

## Beweisziel und Prüfungen

Deterministische Nachweise: Resume benutzt den gespeicherten Request und startet keine neue Berechnung; veränderte oder ungültige Datei scheitert geschlossen; wiederholte 429 bleiben als rate_limited sichtbar; BLOCKED hat Fehlerexit; bestätigte Endzustände und GET-Erholung aus Runde 1 bleiben erhalten. Bestehende Tests nicht abschwächen. Keine neuen echten Wartezeiten in Tests.

Vor Compilerstarts beide Hostsperren blockierend in Reihenfolge halten: /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/locks/host-checks.lock, danach /tmp/deadlock-cargo-release.lock. HOSTPROBE.md mit frischer NonZombie-Probe und genauer Metadata-Ausnahme befolgen, höchstens zwei Jobs, bei fremden Compilern mit gehaltenen Locks nach 30 Sekunden erneut prüfen. Keine fremden Prozesse stoppen. Nach Ende eigene Kinder und Freigabe belegen.

Toolchain ausdrücklich /home/nathanael/.cargo/bin/cargo, PATH zuerst /home/nathanael/.cargo/bin, SQLX_OFFLINE=true, --locked, -j 2. Passende Suites: brain-feeds --lib build_publish, brain-feeds --test build_publish_endpoint und die vollständige deadlock-brain --bin deadlock-brain-Suite ohne Namensfilter, jeweils --include-ignored. Neue Resume-Tests müssen tatsächlich laufen. Exit UND echte Testanzahl prüfen, kein Erfolg bei Cargo-Fehlertext ohne Resultat. Logs unter bereiche/s/pruefung-v2/folge-* in der gemeinsamen Akte. Baseline nicht erfinden.

Nach Fixcommit Selbstprüfung mit python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-fertig-s --base 511a347b653beba13c2bf130f4bead7a7196cc2a --timeout 600. Standardmodell aus der Gate-Konfiguration, keine Modellübersteuerung. Bei BLOCK die offene Liste melden, keine eigene Orchestrierung.

## Routing

Rückgabewert an Teil-Orchestrator S2, voller SHA, Testanzahlen, Gateurteil, Fundstellen und Grenzen. Hauptauftraggeber Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Status allein teil-s2 unter status/s/2. TODO.md und REGISTER.md nicht schreiben, keine Sessionkoordination. Abschließende frische Intent-Abnahme und tatsächlicher Live-Publish gehören nach dieser Fixrunde zum Teil-Orchestrator.
