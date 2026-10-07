status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-s

# S2: unabhängige Rust- und Sicherheitsprüfung am Dateisnapshot

## Ziel und Vertrag

Prüfe den Sicherheits- und Wiederaufnahmevertrag des vorhandenen Folgefixes. Der finale Intent-Nachweis und der Merge-Gate bleiben separate Schritte. Referenzen: FOLGE-FIX-BRIEFING.md, REVIEW.md und DEPLOYVERTRAG.md im selben Bereich. Keine Communitynachricht und kein echter Publish.

Der Client muss vor POST den hashgebundenen Bestandsstatus prüfen, eine verlorene POST-Antwort durch GET erholen und bereits bestätigte Endzustände erhalten. Anhaltende Drosselung muss in der nicht bestätigten Quittung erkennbar bleiben. Die CLI muss die vollständige unveränderliche Anfrage vor erster Außenwirkung sichern und eine explizite Wiederaufnahme ohne Reasoner oder AI ermöglichen. BLOCKED darf keinen erfolgreichen Exit oder HTTP-Aufruf verursachen.

Besonders prüfen: begrenztes und atomisches Dateilesen/-schreiben, Konflikte statt stiller Überschreibung, Schema- und Hashprüfung, Erhaltung von request_id und finalem request_sha256, Dateipfade und Dateirechte, Fehlertexte ohne Rohinhalte oder Zugangsdaten, Redirect-/URL-Grenzen und Isolation des Blocking-Clients. Keine hypothetischen Funde ohne konkrete Eingabe und Auswirkung. Den ID-Suffix nicht mit dem endgültigen Requesthash verwechseln.

## Eigentum und eingefrorene Grundlage

Produktivdateien ausschließlich lesen:

- /home/nathanael/.worktrees/brain-fertig-s/rust/crates/brain-feeds/src/build_publish.rs
- /home/nathanael/.worktrees/brain-fertig-s/rust/crates/brain-feeds/tests/build_publish_endpoint.rs
- /home/nathanael/.worktrees/brain-fertig-s/rust/crates/deadlock-brain/src/main.rs

Der Folgefixer wf_67a90ad5-45a besitzt diese Dateien weiterhin und wartet derzeit mit seinem Prüfwrapper auf Hostlocks. Du erhältst kein Schreibrecht daran. Fertige vor der Prüfung einen stabilen eigenen Lesesnapshot der drei Dateien im eigenen Unterordner `pruefung-v2/sicherheits-snapshot/` an. Gleiche die vollständigen Dateidigestwerte vor und nach der Erfassung ab. Prüfe anschließend die eingefrorenen Kopien, nicht bewegliche Dateien. Erfassung abbrechen, falls sich die Dateien währenddessen verändern. Eigene Snapshots, Manifest und Rohbericht dürfen dort abgelegt werden, keine bestehenden Artefakte überschreiben.

Ergebnis nennt jeden ursprünglichen Pfad mit SHA256 des tatsächlich geprüften Inhalts. Nur bei genau gleichem Inhalt kann S2 später die Prüfung an den finalen Commit binden. Keine endgültige SHA- oder Testbindung erfinden. Bei verändertem Finalstand bleibt die Abnahme eine Momentaufnahme.

Keine Cargo-/Rustc-/Clippy-/Fmt-Aufrufe, keine Builds, Dienste, DB-, HTTP- oder Git-Mutationen. Der Compilerweg ist hier ausdrücklich nicht Teil deiner Rolle; die alte System-Cargo-Version ist ungeeignet. Gewöhnlicher nativer Worker: keine Delegation und keine weiteren T3-Threads. Secrets und Prozessumgebungen nicht lesen, ausgeben oder speichern. TODO.md, REGISTER.md und VON_HAUPT.md nicht verändern.

## Arbeitsstand

Brain-HEAD zuletzt `148e1a58a485def587f763c76c9ec7a9ff06040f`, Branch `feat/brain-fertig-s-20261003`, genau die drei Dateien aus dem Folgefix geändert. Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. Es gibt noch keinen Folgefixcommit, keine Integrationsfreigabe und keine Nachlauftestzahlen. Frühere 29 Tests und Gate ALLOW betreffen nur 148e1a58. GPT-6.1 Sol erben, Effort xhigh, keine Modellübersteuerung.

Graphify zuerst. Die Hauptsession hat am 18:04-UTC-Stand den globalen Graph nach HttpBuildPublishClient gefragt; kein Symboltreffer. Kein vollständiger Graph-Neuaufbau. Danach die benannten, belegten Dateien und ihr vorhandenes Diff untersuchen. Verträge dürfen lesend gezielt nachgeschlagen werden; keine Bestandssuche ohne Graphify.

## Beweisziel

Rückgabe: geprüftes Manifest, konkrete bestätigte Befunde mit ursprünglichem Pfad und Zeile, Eingabe beziehungsweise Zustand und beobachtbarem Fehler. Falls keine: ausdrücklich keine bestätigten Befunde im geprüften Snapshot. Keine pauschale Freigabe für den Live- oder Integrationsstand. Alle fünf bisher dokumentierten Befunde aus REVIEW.md im aktuellen Snapshot gegenprüfen, neue Speicher-/Wiederaufnahmerisiken unabhängig suchen. Testcode lesen, aber keine Tests ausführen oder daraus Laufnachweise ableiten.

## Routing

An Teil-Orchestrator S2 dieser nativen Session berichten. Hauptauftraggeber Codex /root, T3 e6c19079-657e-4db9-80bd-8e1313e7f785. Status allein teil-s2 unter status/s/2. Keine Sessionkoordination oder Nachrichten in laufende Kontexte. Rohbericht hier im eigenen Snapshotordner, Zusammenfassung als Werkzeugrückgabe. Fachliche Übergabe, finaler Gate und Live-Publish bleiben bei S2 beziehungsweise Z gemäß bestehendem Vertrag.
