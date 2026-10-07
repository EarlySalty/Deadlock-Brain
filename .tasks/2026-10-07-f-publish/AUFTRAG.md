# Paket F: Veröffentlichung ohne Matchgrenze

Auftrag und Steuerung: kanonische Akte `.tasks/2026-10-06-brain-abschluss/`, Abschnitte vom 07.10.2026 um 05:05 und 05:10. Bericht: `AN_HAUPT-F.md` dort.

Keine Unterthreads. Eigener Worktree von `origin/main` (`bfda408c`), Branch `feat/brain-build-publish-ohne-matchgrenze`.

Veröffentlichung beruht auf aktuellen API-Spielwerten und gültiger Mechanik, nicht auf Matchzahlen oder Familienhäufigkeiten. Keine Rohmatches lesen oder speichern. Bestehender Reasoner, bestehender Antwortweg und bestehende Steam-Veröffentlichung bleiben erhalten.

Prüfung: bestehende Reasoner-/CLI-Tests, eigene Dateien formatieren, Clippy, `gate_hook.py --review`. Beleg: Warden aus eigenem Binary mit `--publish`, echte Build-ID. Kein Release, Install, Neustart oder Tick. Main-Push erst nach Freigabe des Releasefensters; vorher Featurecommit sichern.
