status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-s

# S2: Antwortinhalte aus Parserfehlern entfernen

## Ziel und Vertrag

Frische Fixrunde für einen bestätigten Rust-/Security-Befund aus wf_2e0843f7-aec: `rust/crates/brain-feeds/src/build_publish.rs:354-355` gibt Serde-Details über `PublishError::InvalidResponse(e.to_string())` zurück. Eine erfolgreiche HTTP-Antwort mit `{"synthetic-private-content":true}` erzeugt dadurch einen Fehler, der den antwortabhängigen Feldnamen enthält. Der gemeinsame Reader betrifft POST und beide GET-Pfade.

Ersetze diesen antwortabhängigen Diagnosetext durch einen festen, inhaltsfreien Fehler. Prüfe passende benachbarte Parsing-/Validierungspfade auf denselben konkreten Leak. Keine hypothetischen Zusatzrefactorings. Ein synthetischer privater Marker darf weder in Display noch Debug des zurückgegebenen Fehlers vorkommen. Nachweis über den vorhandenen echten lokalen HTTP-Testserver, kein echter Produktivaufruf.

Der Befund war bereits vor dem Folgefix vorhanden. Die CLI maskiert diese Details; eine aktuelle CLI-Zugangsdatenoffenlegung ist nicht belegt. Ziel ist der strengere vorhandene Fehlervertrag der Clientbibliothek, kein erfundener Sicherheitsvorfall. Vorherige fünf Publish-Korrekturen und gespeicherte Wiederaufnahme unverändert erhalten.

## Eigentum

Genau zwei Produktiv-/Testdateien dürfen geändert werden:

- rust/crates/brain-feeds/src/build_publish.rs
- rust/crates/brain-feeds/tests/build_publish_endpoint.rs

CLI und übrige Dateien ausschließlich lesen. Keine neuen Crates, Anbieter, Modelle, Migrationen oder Lockfileänderungen. Keine globale Formatierung. Eigene Prüfarbeitsartefakte und neue Logs mit Präfix `redaktion-*` unter dem vorhandenen Bereich `pruefung-v2/` erlaubt, keine älteren Logs überschreiben. Produktiver Code ausschließlich Rust. Gewöhnlicher nativer Worker, keine Delegation und keine T3-Threads. Secrets und Prozessumgebungen nicht lesen, ausgeben oder speichern.

## Arbeitsstand und ausdrückliche Git-Freigabe

Worktree `/home/nathanael/.worktrees/brain-fertig-s`, Branch `feat/brain-fertig-s-20261003`, HEAD `5a831bf69b04951ee1ea7d7c1d5fa2282a787299`. Der vorige Schreiber ist beendet, seine Prüfprozesse sind nicht mehr vorhanden. Seine drei Änderungen wurden von S2 nach 116 tatsächlich bestandenen Tests committiert: 5 Lib-, 24 HTTP- und 87 vollständige CLI-Tests. Noch kein Gate am neuen Commit und noch kein Push dieses Commits.

Commit und Push des eigenen Featurestands sind ausdrücklich durch den echten Nutzerauftrag vom 2026-10-03T15:43:38.636Z erlaubt: „Commits und Push des eigenen Featurestands sind erlaubt. Gekoppelte Änderungen gehen an Z zur gemeinsamen Integration/Abnahme und Releaseinstallation.“ Das ist keine berechnete Skriptfreigabe. Erstelle nach der eigenen Prüfung den Fixcommit der beiden Dateien mit dem Modelltrailer aus rolle-merge-schleuse. Nach erfolgreicher Gate-Selbstprüfung den Featurebranch pushen. Keine main-Mutation, kein Deploy, Releasewechsel oder Publish. Bei tatsächlicher Permission-Ablehnung nicht umgehen oder denselben verbotenen Aufruf wiederholen, sondern den echten Blocker melden.

GPT-6.1 Sol erben, xhigh, keine Modellübersteuerung. Graphify zuerst, dann die benannten Fundstellen prüfen. Die frühere Snapshotabnahme hat CLI-Digest `0bde38d52b47795a45045c592c81f54de7379a18fac0128ee6d078038818c318`; der aktuelle getestete CLI-Digest ist `a18cc9b404776542f34d50fbe6b3ef7e667386095a55a944aa0cae9084169dd3`. Zwischenzeitlich wurde ein atomischer Zähler im temporären Dateinamen ergänzt und eine Match-Formatierung geändert. Kein altes Snapshoturteil als endgültige Inhaltsbindung ausgeben.

## Beweisziel

Passende bestehende Suites unter beiden Hostlocks in Reihenfolge gemäß HOSTPROBE.md. Keine Wartefrist auf reine Sperren, keine fremden Prozesse stoppen, frische konservative NonZombie-/Metadata-Probe, höchstens zwei Compilerjobs. Toolchain /home/nathanael/.cargo/bin/cargo, PATH zuerst /home/nathanael/.cargo/bin, SQLX_OFFLINE=true, CARGO_BUILD_JOBS=2, --locked, -j 2, jeweils --include-ignored.

Brain-feeds --lib build_publish, brain-feeds --test build_publish_endpoint und vollständige deadlock-brain --bin deadlock-brain-Suite ohne Namensfilter. Nenne tatsächliche passed/failed/ignored/filtered-Zahlen und innere Cargo-Exits. Ein äußerer Exit 0 ohne Testresultat genügt nicht. Source-/Lockdigests vor/nach müssen gleich sein. Keine Baseline behaupten. Eigene Kinder beenden und beide FDs schließen.

Nach Fixcommit Selbstprüfung mit `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-fertig-s --base 511a347b653beba13c2bf130f4bead7a7196cc2a --timeout 600`, ohne Modellübersteuerung. Kein BLOCK neu würfeln oder umgehen. Bei BLOCK die genaue Liste an S2 zurückgeben, keinen weiteren Fixer selbst starten. Ein technischer Fehler ohne Urteil ist kein ALLOW.

## Routing

Rückgabe an Teil-Orchestrator S2: voller finaler SHA, Befundfix, tatsächliche Testzahlen, Source-/Lockbindung, Gateurteil und Pushstatus mit Nachweisorten. Hauptauftraggeber Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Status allein teil-s2 unter status/s/2. TODO.md, REGISTER.md und VON_HAUPT.md nicht ändern. Keine Nachrichten in laufende Kontexte. Z integriert und installiert gemeinsam; finaler unabhängiger Intent-Nachweis und Live-Publish bleiben bei S2.
