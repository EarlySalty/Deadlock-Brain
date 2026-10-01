status: aktiv, Quellabgabe ohne CI-Lauf
Datum: 2026-09-30

# B1-R1-N1: Cargo-Pfad in drei isolierten CI-Aufrufen

Getrennter Quellhead `0c56f85cf2c76aec963db3d567009bcd461c19c0` auf `fix/g5-replay-deferred-20260930`, gepusht nach dem B2-Fixquellhead `e878530a476431ff844d22f70cd86780d3cc3de4` und dessen Berichtscommit. Der unabhängige Anschlussbefund steht in `B1-R1-NACHREVIEW.md` des bestehenden Reviewworktrees. Dies ist ein kleiner separater Fix ohne Änderungen an den ursprünglichen B1-Runnern oder dem B2-Importer.

`.github/workflows/brain-serve-c1.yml:75-76` und `.github/workflows/rust-core-verification.yml:69-70,87-88` geben den vor `env -i` per `command -v cargo` aufgelösten ausführbaren Pfad als bereits unterstütztes `BRAIN_TEST_CARGO` weiter. Die isolierte `HOME`-Belegung, der vorhandene Targetcache als Pflichtargument, die übrigen ENV-Werte und die Pipeline-Fehlerweitergabe bleiben erhalten. `scripts/test_brain_serve.sh:15` und `scripts/test_brain_core_postgres.sh:15` verwenden dadurch nicht mehr den leeren Test-HOME als Cargo-Installationsort. Upgrade, Wiki und lokale Wrapper wurden nicht verändert. Es wurde weder ein zweites Cargo installiert noch eine produktive ENV-Konfiguration eingeführt.

Statisch geprüft: genau drei geänderte Aufrufzeilen in zwei Workflowdateien, jeder Shell-Block mit `/bin/bash -n` Exit 0, Cargo-Expansion vor der isolierten HOME-Zuweisung und `git diff --check` sowie `git diff --cached --check` Exit 0. Keine Workflow-, Compiler-, Harness-, Datenbank-, Prozess-, Last- oder Dienstausführung. Der volle Serve-Runner würde im genehmigten Lauf auch den ignorierten B2-Fixturetest und 1.800 Lastanfragen starten. GitHub Actions sind kein Merge-Gate. Eine unabhängige Nachprüfung und die getrennt zugeteilten Laufzeitfenster stehen noch aus.

BESTAND[BS-1]: ja | Fundort: scripts/test_brain_serve.sh:15 | Anknüpfung: vorhandenes internes BRAIN_TEST_CARGO-Feld
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: B1-R1-N1-FIX-ERGEBNIS.md
ORCHESTRIERUNG[OR-1]: Stufe klein | Schritt bau | Artefakt: .tasks/2026-09-30-g5-abschluss/B1-R1-N1-FIX-ERGEBNIS.md
