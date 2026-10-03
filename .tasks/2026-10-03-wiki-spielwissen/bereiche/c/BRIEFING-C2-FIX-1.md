status: aktiv
Datum: 2026-10-03

# C2 Fixrunde 1: Revisionsimport

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration

## 1. Ziel und Vertrag

Du bist ein frischer nativer Blatt-Fixer, kein Teil-Orchestrator und kein unabhängiger Abnahme-Kritiker. Keine weiteren Agenten oder T3-Threads. Auftraggeber C2 a17ac7e9-7f41-44b0-a6e4-901bfafe544f, Haupt /root. Ausschließlich geerbtes GPT 6.1 Sol, Effort high oder medium, keine Modell- oder Settingsänderung. C2-MODELL-GATE.md dokumentiert die tatsächliche Vererbung des unveränderten coder-Startwegs.

Lies REVIEW-C2-1.md hier und den bestehenden CONTRACT.md im übergeordneten Taskordner. Zwei tatsächlich bestätigte Sol-high-Gate-BLOCKs gezielt beheben: lexikografische Reihenfolge opaker Originalrevisionen und Kollision zwischen originalen Wiki-Revisionen und lokalen Store-Revisionen. Bisheriger Code ist vorhandener C-WIP, kein Neubau. Bewahre exakte Originalherkunft, byte-/semantikgetreue Historie, bisherige Release-Pins, idempotenten Wiederholimport, atomare Konflikte, Rechte und bestehende Grenzen. Tatsächliche numerische Wiki-Chronologie muss auch bei URL-Identitäten ältere Köpfe verhindern. Opake Hashes haben keine lexikografisch belegte Zeitordnung. Keine unbelegte feste Zahlenschranke oder angeblich kollisionsfreie Reservierungsband-Lösung. Originale Quelle und interne Store-ID sauber trennen. Unbekannte Inhalte erhalten, keine Herkunft erfinden und keine älteren bekannten Wiki-Köpfe veröffentlichen.

Vor Bestandssuche code-suche laden und Graphify fragen, danach Fundstellen nachlesen. source_versions.rs und knowledge_import.rs enthalten bestätigte Zwillinge. Lies auch die bestehende PgStore-Revisions-/Kopfmechanik und betroffene Tests. Kein zweiter Import- oder Readerpfad, keine Migration ohne tatsächlich erforderlichen Nachweis.

## 2. Schreibzuständigkeit

Semantisches Eigentum: rust/crates/brain-storage/src/source_versions.rs; rust/crates/dbrain-sources/src/knowledge_import.rs; deren vorhandene Inline-Tests und gegebenenfalls eng betroffene bestehende Wissenstests in rust/crates/brain-storage/tests/knowledge_release.rs und rust/crates/dbrain-sources/tests/knowledge_contract.rs. Eng nötige Exportanpassungen in den zugehörigen lib.rs zulässig. Andere Dateien der bestehenden 14 C-WIP-Dateien nur für tatsächlich nötige Compileranpassungen an diesen Änderungen und gezielte Formatierung. Kein globales cargo fmt, keine fremden Quelldateien, keine A/B/D-Registrierung, keine allgemeine Umstrukturierung und kein Limitwachstum. Änderungen außerhalb dieser Grenzen zuerst C2 melden, nicht eigenmächtig ausweiten.

Nicht übernehmen: CLI-PG-Zielbindung und Publish-Zeitstempel-NIT, Releaseinstaller, A, B2, D. Diese bleiben bei C2 und werden später geordnet bearbeitet. C2 schreibt während deiner Quellen-/Prüfphase nicht in deine produktiven Dateien. B2 prüft seinen eigenen vorhandenen Harness und extrahiert, du prüfst oder baust B nicht.

## 3. Arbeitsstand und Git

Absoluter Worktree /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration. Branch feat/brain-wiki-spielwissen-c-integration, HEAD b1b9241805f470427570566faca37fc340d1c04c, Basis 511a347b653beba13c2bf130f4bead7a7196cc2a, bei Übergabe sauber. Alle älteren C1-Worker beendet, beide C2-Prüfer abgeschlossen. Eigener wartender Prüftask b41kfyaib für nötige Codefixes vor erster Lockbestätigung beendet; beide eigenen Wrapper-PIDs 3048665 und 2916328 um 06:19 UTC nicht mehr vorhanden. Kein anderer C-Writer/Compiler aktiv. Fremde Prozesse und Worktrees unangetastet lassen.

Nach tatsächlich grünen passenden Compiler-/Format-/Clippy-/Bestandstestprüfungen darfst du ausschließlich eigene Dateien auf diesem Featurebranch gezielt stagen und mit Modelltrailer committen. Einzelne Git-Schritte, literale absolute Pfade, kein add -A, stash, Reset, Main-Merge, Push, Branch-/Worktree-Löschen oder Deploy. C2 übernimmt Integration und Push. Kein Commit kaputten ungeprüften Codes. Vor einem etwaigen Commit HEAD/Status bestätigen und Commit-SHA exakt melden.

## 4. Beweisziel und sichere Prüfung

Belege beide Fehlerfälle, Seiten- und URL-Wiki-Identitäten, bekannte ältere Revision nach unbekannter/lokaler Revision, Wiederholimport auch über getrennte Batches und unveränderte Release-Pins. Neue Tests sind keine allgemeine Pflicht, vorhandene falsche Erwartungen müssen korrigiert werden. Den riskanten PostgreSQL-Pfad nicht nur durch Speicher-Fakes behaupten. Vorhandene isolierte DB-Prüfung kennt BRAIN_CORE_TEST_PG_SOCKET, Port 55439, Rolle brain_core_test, Socketendung /.core-test-pg. scripts/test_brain_serve.sh zeigt den vorhandenen scratch-Setup. Kein produktiver DB-Schreibzugriff und keine Secrets lesen/ausgeben. Schreibendes psql, produktive Schemaänderungen, neue Dienste und neue Python-Skripte verboten.

Vor jedem Cargo-, Rustfmt- oder Clippy-Aufruf HOSTPROBE.md unter /home/nathanael/Documents/.tasks/2026-10-02-offene-branches/ lesen und beide Locks in dieser Reihenfolge blockierend halten: host-checks.lock unter dortigem locks-Verzeichnis, danach /tmp/deadlock-cargo-release.lock. Frische NonZombie-Probe unmittelbar vor Cargo; höchstens zwei Jobs. Nur exakt cargo metadata --format-version 1 --no-deps --manifest-path /absoluter/pfad/Cargo.toml ist die erlaubte Cargo-Probeausnahme. Andere oder unbekannte lebende Compiler sind blockierend. Niemals fremde Compiler stoppen. Repo-Root ist kein Cargo-Root, benutze bestehendes rust/Cargo.toml. Sichere Prüfwrapper erhalten, 20-Minuten-Wache ist kein Abbruchlimit und Lockwartezeit kein Compilerfehler. Keine parallel laufende eigene Prüfung. Ein eigener stabiler Wrapper wird nur für konkrete notwendige Codekorrekturen oder geordneten Handoff beendet. Vollständige Prüf-Ausgaben lokal aufbewahren, echte Exit-Codes und passed/ignored/failed getrennt melden. Ignorierte/nicht ausgeführte DB-Tests sind kein grüner DB-Nachweis.

Nach einem grünen geprüften eigenen Commit deine Änderung selbst über den bestehenden regulären Gate prüfen: python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration --base 511a347b653beba13c2bf130f4bead7a7196cc2a --head <wirklicher-neuer-SHA> --model gpt-6.1-sol --effort high. Tatsächlichen Einzelmodellstart vor Aufruf und ausgewählte Modellparameter des eigenen Kindes nach Start belegen, keine Prozessumgebungen oder Secrets lesen. Kein fremder Default, kein direkter Codexstarter, keine Ausweichkette. Gate-Urteil am korrekten SHA melden. Bei neuem BLOCK keine neue eigenmächtige Fixrunde: Status/Log sichern und Funde C2 für einen frischen Fixer übergeben. Kein ALLOW aus altem WIP oder uncommittierten Änderungen ableiten.

## 5. Rückmeldung

Bericht bevorzugt nach bereiche/c/C2-FIX-1.md im lokalen Koordinationsworktree /home/nathanael/.worktrees/brain-wiki-spielwissen-c/.tasks/2026-10-03-wiki-spielwissen/ schreiben. Falls eine übergeordnete Rollenregel Dateiberichte untersagt, echte Ergebnisse vollständig in Abschlussantwort liefern, C2 sichert sie. Keine zentrale REGISTER.md, TODO.md, UEBERGABE.md oder Statusereignisse schreiben. Einziger Statusproduzent teil-c bleibt C2. Eigene Worker-ID und Prüftask/PID/Lockmarker im Zwischenstand melden. Keine fremden Sessionnachrichten.

Gebaut, geprüft, Gate, gemergt und live getrennt. Algorithmusänderung, exakte Originalherkunft gegenüber Store-ID, betroffene Files/SHA, Risiken und nicht ausgeführte Prüfungen benennen. TESTNACHWEIS[TW-1] und MERGEPROTOKOLL[MS-1] mit tatsächlichen Zahlen. C2 braucht keinen Schluss ohne Wirkung. Bleibt ein stabiler Prüftask auf Locks wartend, nicht nach dem Wachtimer mit einer Fertigmeldung aussteigen; Task-ID/PID und genaue Nichtnachweise zum C2-Zwischenstand, danach weiterarbeiten.
