status: aktiv
Datum: 2026-10-03
Stand: 2026-10-03T07:30:04Z

# C2 Fixrunde 1: eng nötige Manifestkorrektur

C2 hat tatsächliche Logs von b1d9vt707/PID 3350887 gelesen: beide Locks und Gegenproben erreicht, eigener Formatlauf und Formatcheck Exit 0; cargo check Exit 101 mit E0432, unresolved import sha2 in source_versions.rs:5. Wrapper regulär beendet, LOCKS_RELEASED, eigener PID nicht mehr vorhanden. Kein Test-/Clippy-/DB-Erfolg. Dies ist ein tatsächlich gestarteter Compilerlauf, keine Lockwartebehauptung.

Nach Graphify-Abfrage vorhandenes rust/crates/brain-storage/Cargo.toml nachgelesen: sha2.workspace = true steht bislang ausschließlich unter dev-dependencies, Zeile 22. Das produktive vorhandene Revisionsmodul benötigt es als reguläre Abhängigkeit.

C2 erweitert die Eigentumsgrenze genau auf dieses Manifest: dieselbe vorhandene Workspace-Abhängigkeit von dev-dependencies nach dependencies verschieben; dadurch leere dev-dependencies-Sektion gegebenenfalls entfernen. Keine neue Bibliothek, Version, Featurewahl oder sonstige Manifeständerung. Enge daraus abgeleitete vorhandene sha2-/chrono-Lockeinträge darf der Fixer nach Diffkontrolle und grünen Prüfungen mit aufnehmen; keine A/B/D-Registrierung.

Danach denselben laufenden Revisionsfix sicher weiterprüfen. Beide Locks blockierend, frische Probe, maximal zwei Jobs; einen wartenden stabilen Prüfwrapper nicht wegen Wachtimer beenden. Keine parallele C-Quelle oder weitere Prüfung. B-Großzeilen-/CLI-Zielbindungsfix bleibt bis zu deiner geordneten Übergabe separat bei C2.
