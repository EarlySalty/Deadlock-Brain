status: aktiv
Datum: 2026-09-29

# C1: fehlende Prozess-Abnahmefälle ergänzen

## Nachtrag nach lokalem Gate auf ead4a791

R-AC-R2 hat C1 akzeptiert. Das zusätzliche lokale Gate meldet ALLOW, aber zwei konkrete Testlücken, die vor Schlussmessung zu klären sind:
1. process_e2e.rs:974: Alias-Konflikt wird erst nach Widerruf der konkurrierenden Warden-Evidenz geprüft. Damit beweist der Fall nicht sauber zwei gleichzeitig berechtigte widersprüchliche Kandidaten bei Limit 1. Reihenfolge/Zustand am Code prüfen und diesen Pflichtfall vor dem Widerruf mit zwei zugänglichen Konfliktwerten ausführen. Widerruf und ursprüngliche Lastfälle behalten.
2. process_e2e.rs:993: Peer-Ersatz entfernte SCRAM-Erfolg, Ablehnung falscher Passwörter und Log-Redaktion. Vorhandene sichere Regressionen dafür suchen. Falls nicht vorhanden, getrennte Scratch-Abdeckung mit ausschließlich synthetischen, zur Laufzeit erzeugten Testwerten ergänzen, ohne Produktionspasswörter, ENV-Konfiguration oder alten unsicheren Wrapper. Bevorzugten Unix-Socket/Peer-Hauptpfad nicht ersetzen. Wenn die sichere Zusatzabdeckung die Briefinggrenzen überschreitet, präzisen Befund liefern statt still weglassen.

Auf demselben Branch weiterarbeiten, keine neuen Threads/PRs. Bericht um echte Ergebnisse ergänzen, passender Runner und Selbst-Gate erneut. Keine Budgetänderung und kein Merge.

Luna als gezielter Fixer. C-Autor Sol (6b53c923) ist nach Abgabe gesettelt; du übernimmst seinen sauberen Worktree, kein Neubeginn. Du bist der einzige schreibende Thread für C1, keine Unterthreads oder Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. AUFTRAG.md gilt, Graphify zuerst, keine Code-Kommentare, keine Produktion und keine echten Daten/Secrets. Nur eigenen Branch committen/pushen, nicht mergen/deployen.

Worktree /home/nathanael/.worktrees/brain-pre-g5-harness-20260929, Branch fix/pre-g5-harness-20260929, sauberer Abgabehead e671c5b, bestehender Draft-PR #60 gegen migration/rust-integration. Exklusiver Scope: Prozess-E2E und seine vorhandenen Testhelfer/Runner; keine Assets-/Match-/Analytics-/Storage-Produktdateien, Manifeste oder Lockfiles. Kein pauschales Refactoring/Formatieren. A12 und A34 ändern andere Produktpfade parallel.

Unabhängiger Review: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/REVIEW-AC.md (Reviewstand fde7d5d). Ausschließlich R-AC-C1 beheben. A1-A4 sind anderen Threads zugeteilt. C-Delta e540e97..e671c5b umfasst Assets-/Legacy-Testnachträge und Bericht, behebt C1 nicht.

Konkrete fehlende Prüfungen:
1. Start des echten brain-serve gegen inkompatibles Schema (früher Version 99) und leere DB geschlossen abweisen. Nach fehlgeschlagenem Start belegen, dass Service nicht selbst Schema/Migration angelegt hat. Alte Pflichtfälle stehen in 305df2d:scripts/run_isolated_serve_checks.sh:118-126. Unsichere historische Wrapper NICHT wieder aktivieren, sondern in den neuen privaten Wegwerfcluster übertragen.
2. Englischen Alias Guardian über echte Prozessanfrage prüfen, zusätzlich eine tatsächlich unbekannte Entity statt Haze (Haze existiert mit anderer ACL).
3. Konflikt mit Retrieval-Limit 1 durch normalen Prozesspfad prüfen. Bestehende Bibliotheksregression chunked_retrieval.rs:176-200 gibt das fachliche Gegenbeispiel. Nicht nur Unit-Test als E2E zählen.
4. Rückrichtungs-ACL konkret auf Berechtigungsablehnung prüfen, nicht beliebigen ClientError akzeptieren (aktuell process_e2e.rs:542-548).
5. Nach absichtlichem DB-Ausfall und Recovery eine normale fachlich erfolgreiche Antwort desselben wiederhergestellten Serve-Prozesses vor dessen Beenden/Neustart nachweisen, nicht nur Readiness.

Beweisziel: sicherer Runner enthält ursprüngliche und neue Pflichtfälle, Fehlercodes präzise, Budgets und Pool unverändert. Bestehende 600er-Laststufen erhalten. Unabhängiger Reviewer hat auf lokalem Vorabmerge A+C f557064 bereits 600/600 bei 8,16,32 mit Peak4 gemessen; das ist kein finaler Integrationsbeweis und ersetzt den früheren roten Autorenlauf nicht. Nicht durch wiederholtes Messen auf Grün warten oder Budgets lockern.

Passende fmt/clippy und Prozessrunner ausführen, Testfehler ehrlich melden, neue Pflichtfälle separat belegen. Eigene gate_hook.py --review gegen e671c5b vor Abgabe, C1-REPORT.md mit genauen Befehlen/Exitcodes/Commit. Alten C-REPORT als historischen Stand erhalten, keine früheren roten Werte umetikettieren. Bestehenden PR #60 aktualisieren, keinen zweiten PR erstellen. Kein Merge; unabhängige Nachprüfung kommt danach. Bei größerer Grenze: [Bump-up] Paket C1: Grund: ... Erledigt: ... Worktree: ... Offen: ...
