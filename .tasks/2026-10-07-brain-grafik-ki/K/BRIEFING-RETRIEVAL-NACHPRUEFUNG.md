# K: Retrievalfixture und echte Nachprüfung nach neuer Freigabe

Status: frischer Worker aab063d35034d0c46 ohne Änderung an tatsächlicher ctx_execute_file-Projektrootgrenze gestoppt. Keine Workerprüfungen. Die zulässigen Cargoaufrufe von K sind unabhängig davon tatsächlich nachgeholt, einschließlich 3/15 roter Retrievalsuite mit ungültigem Release-Lesemanifest. Details PRUEFWEG-CARGO-SLOT.md. Keine Wiederholung oder Umgehung der konkreten Werkzeugablehnung; vor erneuter Fixturearbeit muss die Rootbindung wirklich greifen. Folgender Auftrag dokumentiert den gestoppten Versuch, nicht eine neue Startfreigabe.

## Auftrag und Vertrag

Die bestätigte Weiterbauentscheidung ENTSCHEIDUNG-WEITERBAU-2015.md und der Vorrangabschnitt PAKETE.md erlauben jetzt ausschließlich cargo-slot für Cargoaufrufe; Arbeitsroots ~/.worktrees, ~/repos und /tmp sind freigegeben. Keine alten flock-/Runnerformen. Nutzer verlangt tatsächliche fehlende Prüfungen, keine Neubauten. Private Daten bleiben gesperrt.

Frischer Budgetfix liegt uncommitted in Contracts/Providers/Serve-Tests: gemeinsamer Payloadbauer enthält unveränderte gezählte Wirekontrollen; Legacyhelper liefert das Maximum beider Wireformen. Die vorhandene Retrievalassertion in rust/crates/dbrain-retrieval/tests/chunked_retrieval.rs:203 vergleicht noch mit einem kontrollfeldlosen kompatiblen Payload. Gegen den tatsächlichen gemeinsamen kontrollierten Payload korrigieren: exakte Gleichheit zur konservativen Obergrenze beider Wireformen erhalten, nicht einfach zu einer schwächeren Ungleichheit wechseln. Grenz- und Packingassertionen erhalten, keine Budgeterhöhung, Skips oder neue Produktlogik.

## Eigentum und Zustand

Eigener Root /home/nathanael/.worktrees/brain-k-ki-20261007, detached HEAD 9fc48a08a409cd3e049bc9f26500d5daa42e4711. Ausschließlich chunked_retrieval.rs bearbeiten. Drei bereits fertige fremde Workerdateien im selben eigenen Root nicht ändern, zurücksetzen oder formatieren. Keine aktive Sourcearbeit an diesen Dateien. Kein Git-Schreiben; K integriert und gatet. Kein G-Worktree, Ersatzport oder neuer Rechner. Keine neuen Code-Kommentare.

## Tatsächliche Prüfungen

Zuerst Graphify, dann Fixture und gemeinsame Helpers nachlesen. cargo-slot +1.97.1 mit eigenem absolutem Manifest, SQLX_OFFLINE=true für Compiler/Clippy, `--locked --offline --jobs 3`. Format nur kontrolliert für die eigene Datei. Tatsächliche synthetische vorhandene Prüfungen nachholen: Contracts/Providers mit `--no-fail-fast -- --include-ignored`, Serve-Libraryfilter service::answer_provider_tests mit `--include-ignored`, Retrievaltest chunked_retrieval mit `--include-ignored`. Drei getrennte Läufe, genaue passed/failed/ignored/filtered und Exits/Logs nennen. Keine privaten Originaldaten, Echtkanalproben oder neue Corpusquelle. Eigene Test-Postgresfixture nur wenn wirklich erforderlich; keine Produktion, fremden Prozesse oder Ports 5433/5434. Bei einem neuen Schutzblocker stoppen, nichts verstecken oder umgehen.

## Routing

Frischer nativer Worker, geerbtes Modell, high, keine weitere Delegation. Session 988eeaea-28ee-424c-b362-e250610cde91, teil-k, Versuch 1. Delegator 481426fe-b477-42b3-91c6-901811fcba1d bleibt zuständig. Neue bestätigte Weiterbauentscheidung der Claude-Session 3fcd8f71-443e-48ae-825c-527eb52fbe56; früherer Hauptorchestrator d3a1741e bleibt gestoppt. Keine Sessionkontakte, zusätzlichen T3-Threads oder zentralen Register-/TODO-Eingriffe. Rückgabe an K mit minimalem Diff und tatsächlichen Ergebnissen, nicht nur vorbereitetem Befehl. NEVER read, print or write plaintext secrets. MUST NOT send private user/community data to remote models.
