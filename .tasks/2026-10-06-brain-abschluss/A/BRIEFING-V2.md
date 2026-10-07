# A-V2: integrierten Discord-Consumer vollständig prüfen

Native Blattrolle Test-Wächter, Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Keine eigenen Reviewer, Subagenten, T3-Threads oder Sessionnachrichten. Source nur lesen.

## Kandidat

`/home/nathanael/.worktrees/brain-a-discord-20261006`, Branch `fix/brain-a-discord-20261006`, HEAD `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1`, remote gesichert. A hat frisches Main e1f11614 regulär integriert, damit B-Invitefix und dessen entfernte Fehlantwort erhalten bleiben. Diff gegen Main nur brain_api.rs. Fixoriginal b29a55a4, Kombinationsgate ALLOW durch claude-opus-5-5 nach technischem Ausfall des ersten Wrappers. Keine weitere Sourceänderung.

## Bereits gemessen

A-F2c meldete 38 Bibliothekstests ohne Fehler, aber zusätzliche breitere DB-/Scrim-Läufe waren rot. A wiederholte nun am integrierten Stand mit tatsächlich aktivem Feature `testing`, `--include-ignored --test-threads=1`, ohne CENTRAL_TEST_DSN. Ergebnis: dl-brain 17 bestanden, dl-central-db 17 bestanden und vier fehlende-Test-DSN-Fehler. Kein grüner Gesamtlauf. Log `/tmp/brain-a-discord-f2c-20261006/f2c-integrated-tests.log`. Diese Messung ist ein eigener Harnessfehler, kein belegter Produktdefekt. Vorherige Zahlen nicht als Beweis aktiver DB-Tests verwenden.

## Auftrag und Eigentum

Vorhandenen privaten Scratch-/Test-DB-Weg lesen und korrekt einrichten, Graphify vor Bestandssuche. Bekannter Vertrag `rust/scripts/central_test_db.sh` und `dl-central-db/src/testing.rs`. Ports 5433/5434 tabu, keine Prod-DSN und keine Produktivdaten/Provideraufrufe. Eigene Socket-/Portwahl, sichere private Artefakte ausschließlich `/tmp/brain-a-discord-integration-proof-20261007/`. Bestehendes `rust/target` enthält Workerartefakte; nicht löschen. Keine Source-, Unit-, Konfig-, Git- oder Runtimeänderung, kein Gate, Merge oder Deploy. Bestehende Verwaltungs-/Buildwerkzeuge dürfen laufen; keine neuen Python-Werkzeuge. Höchstens ein Cargo-Job wegen aktueller Hostlast.

`dl-brain` und `dl-central-db` mit aktivem testing-Feature vollständig als Bibliotheken prüfen; danach passenden dl-bot-Compilerlauf. Die tatsächlich benötigten DB-Tests müssen laufen, keine stillen Skips. Normale fmt-/Clippy-Prüfung der eigenen Änderung ergänzen, bekannte strenge Baselinewarnungen nicht unterdrücken. Ein vorheriger breiter Migratorlauf scheiterte an privater Infisical-Momentaufnahme: vorhandenen echten Scratchvertrag prüfen, keine Zugangskontrolle oder Produktivbindung umgehen. Falls echte Voraussetzungen nicht beschaffbar sind, präzise melden und passende Kompilierung sowie unabhängige Consumerprüfungen trotzdem abschließen.

Bei rot nicht selbst Code ändern. Ursache, genaue Stelle, tatsächlich getesteter Umfang und offene Grenze an A. Keine Baselinebehauptung ohne gemessene Vergleichszahlen. Scratch sicher stoppen, Logs erhalten. Rückgabe mit SHA, Befehlen/Env ohne Secrets, passed/failed/ignored/filtered, Compiler-/fmt-/Clippy-Exit und Artefaktpfaden. Wache 20 Minuten, spätestens 30. Die Wache ist kein Abbruchzeitpunkt. Keine eigenen Kurzlimits für Compiler oder Suites setzen und keinen kalten Lauf zugunsten einer erneuten Kopie beenden. Laufende Prüfungen über die vorhandene Hintergrundmechanik bis zum tatsächlichen Exit verfolgen. Root-Akten schreibt allein A.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-discord-20261006
