# Inventur A-I1: Brain-Kern und Betrieb

Stand: 06.10.2026, 22:21 CEST. Read-only Rückgabe des nativen Workers `A-I1`, von Paket A als Akte gespeichert. Kein Deploy, Restart oder Produktivschreibzugriff. Codebelege beziehen sich auf `origin/main`, nicht auf den abweichenden Kanon.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/brain-maintenance/src/integration/runner.rs:668 | Anknüpfung: vorhandene Rust-Wartung, Patchimport und SHA-geprüfter Releaseweg

## Belegte Befunde

1. Brain-Kern halb: Remote-main `d6131cc52711a3e8b02d299704244f8d7dbdbce6`; beide Releasezeiger und laufendes Serve-Binary `e56e075d486a75f83f4954b58d8113588082d3f1`. Fünf neuere Main-Commits fehlen live. Artefakthashes passen zu den Manifesten, Health/Ready HTTP 200. Aktiver Korpus: 61 Wartungsdokumente und ein Firstparty-Dokument, keine Patchfeed- oder Steckbriefquelle. Vorhandenen `ops/brain-release`-Weg verwenden.
2. Wartung/Tageslauf kaputt beziehungsweise nicht bestanden: Wartung seit 22:07 mit Signal 15 beendet, Timer inaktiv. Ursache des Stopps ungeklärt. Im vorgesehenen Tagesfenster vom 04.10., 11:33 bis 05.10., 11:33: 55 Profilfehler, ein Wartungstimeout, fünf Serve-Fehlstarts. Frühere fehlende Source-Scopes sind inzwischen konfiguriert. Konkreten Unterfehler im vorhandenen Profil-/Runnerpfad ermitteln: `entity_profiles.rs:25`, `runner.rs:654`.
3. Patchnotes/Historie halb: Rust-Patchnotes läuft auf seinem Main `a148aa694db62dfd217d6ca09e0ff56f5b3954a5`. Legacy-Sync weiterhin alle fünf Minuten, fester September-Release. Journal meldet zentrale Patch-ID 289, isolierte Brain-DB höchstens ID 288. Dort 33.209 Patchereignisse und 18.581 Änderungszeilen. Vollständiger aktueller Übergang nicht belegt. Vorhandene Pfade `entity_profiles.rs:163` und `dbrain-sources/src/patchnotes_db.rs:41` verbinden, keinen zweiten Feed bauen.
4. Sheet-Sync kaputt trotz Exit 0: letzter Lauf 19:25 meldet fehlgeschlagene Schritte und zwei Fireworks-404. Unit verwendet den veränderten Checkout auf `2734c2d` statt des installierten Releases. Auch Main meldet Batch-Ergebnisse mit `failed > 0` erfolgreich. Anschluss: `deadlock-brain/src/main.rs:3578`, `dbrain-enrich/src/lib.rs:779`. Fehlerstatus durchreichen und Unit am geprüften Release betreiben. Provider reparieren, kein eigenmächtiger Modellwechsel.
5. Legacy-Abschaltung noch nicht freigabereif: PostgreSQL enthält 378 Profile, 64.342 Fakten, 20.067 Projektionen, null abgeleitete Quittungen und null Patchintervalle. `/brain` HTTP 404, Site weiterhin Python. YouTube bereits Rust und ausdrücklich pausiert, kein failed Altdienst mehr. Vorhandene Profil-Auslieferung fertigstellen, Legacy-Sync erst nach belegter Rust-Fortschreibung und bestandenem Tageslauf abschalten.

## Urteil

Vorhandener Rust-Code ist weitgehend integriert. Die durchgehende Profilverarbeitung, Aktivierung, HTML-Auslieferung und historische Fortschreibung sind noch nicht live. Health 200 allein erfüllt den Nutzerauftrag nicht. Wartungsstop und konkrete Git-Dienstabweichung werden vor einem weiteren Produktfix empirisch geklärt.
