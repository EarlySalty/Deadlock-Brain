status: aktiv, Anschlussfix nach separater B2-Quellabgabe im selben Worker
Datum: 2026-09-30

# B1-R1: Cacheargument durch bestehende Aufrufer führen

Intent562a877b-0939-440a-964d-1145d9e9431a. Einziger Autor bleibt Sol66adf9ee-bc03-4ff3-91da-73cd8efc5e72, Worktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930, Branch fix/g5-replay-deferred-20260930. Keine Unterthreads oder Unteragenten. Laufendes B2-WIP erhalten, zunächst B2 als getrennten eingefrorenen Quellhead und Bericht sichern, danach diesen Anschlussfix separat committen. Kein Reset oder Neuaufbau.

## Bestätigter Befund

Unabhängiger Bericht eb6ac35a3ee089bc655f10248a95e918fe9d96e6 im Reviewworktree /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929, .tasks/2026-09-30-g5-abschluss/B1-HARNESS-REVIEW.md. Urteil BLOCK ausschließlich für B1-R1. Fünf Runner auf0a6e095 sind statisch korrekt und bash-n-geprüft; bestehende Aufrufer übergeben jedoch kein Pflichtargument. Hauptsession hat nach Graphifyversuch die eingefrorenen Aufrufer unabhängig bestätigt:

- scripts/run_isolated_load.sh:4 führt `exec "$ROOT/scripts/test_brain_serve.sh"` ohne Weitergabe aus.
- scripts/check_brain_core.sh:39,46 ruft Core-PG ohne Targetargument auf.
- .github/workflows/brain-serve-c1.yml:75 ruft Serve ohne Targetargument auf.
- .github/workflows/rust-core-verification.yml:69,76,85 ruft Core-PG, Upgrade und Serve ohne Targetargument auf.
- .github/workflows/wiki-runtime-pilot.yml:48 ruft Wiki ohne Targetargument auf.
- rust/crates/brain-storage/tests/fixtures/storage_v1/README.md:22 zeigt nur `/path/to/cargo`; das einzelne Argument wird jetzt als Targetverzeichnis interpretiert.

Folge: aktive Prüfpfade enden vor Clusterstart mit Usage/Exit2 statt ihren Vertrag zu prüfen. GitHub Actions werden dadurch nicht zum Merge-Gate.

## Enger Scope und Wirkung

Genau die beiden bestehenden Wrapper, drei genannten Workflowdateien und das Reproduktionsbeispiel sowie eigener Fixbericht. Keine Rustprodukt-/Lock-/Dependencyänderung in diesem separaten Fix, keine neuen Skripte oder Kommentare. Deaktivierte historische Runner bleiben deaktiviert. Bestehende Argumentvalidierung nicht abschwächen.

Absolutes vorhandenes Targetverzeichnis ausdrücklich durchreichen. Kein heimlicher Defaultcache oder Fallback bei fehlendem Argument, kein neuer Hostcache. Bei Wrappern mit eigenen Compileraufrufen darf der explizite Targetvertrag nicht an vorgelagerten Aufrufen verloren gehen; vorhandene Aufrufe entsprechend an denselben Cache, Toolchain1.97.1, locked/offline und einen Job binden. Keine umfassende Umgestaltung unbeteiligter Workflowjobs. Im CI den durch bestehende Build-/Cache-Schritte tatsächlich erzeugten absoluten Targetpfad verwenden, nicht den hostlokalen Worktreepfad einbauen. Reproduktionsbeispiel zeigt beide Argumente in korrekter Reihenfolge. Fehlerstatus und Pipefail erhalten.

Nur statische Quellprüfung, git diff --check und bash -n. Keine Wrapperausführung, kein Cargo, keine Tests, Rollenfixtures, DB-Verbindung oder Schreibvorgänge, Produktprozesse oder Deploys. Keine Secret-/ENV-Dateien lesen oder neue Betreiber-ENV-Konfiguration. Der Twitch-Rollout wurde laut Nutzer nach SQLx-Peer-Fehler auf593cfb6c zurückgerollt; enger Twitch-Hotfixa82cbe5f läuft beim Integrator. Bis zu dessen konkreter Slotrückgabe bleibt der Brain-Laufzeithalt bestehen. Keine Twitchdateien oder Dienste anfassen, keine allgemeine whoami-/SQLx-Reparatur in diesen Fix aufnehmen.

## Abgabe und Review

B1-R1-FIX-ERGEBNIS.md mit Basis, eigenem Quellcommit, vollständig erfassten Aufrufern, statischen Gegenbeweisen und exakten späteren Befehlen/Laufklassen. Nur eigene Dateien stagen, committen und auf eigenen Branch pushen. B2- und B1-R1-Quellheads getrennt nennen, keine historischen Rusttests auf neue Quellen übertragen. Anschließend bestehender unabhängiger Reviewer prüft das eingefrorene Anschlussdelta, keine automatische Ausführung. Fehlende Quell-/Policygrundlage für B2 bleibt separat als konkreter Pflichtinput dokumentiert.
