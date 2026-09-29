status: aktiv
Datum: 2026-09-29

# A12: Match-Vertragsfixes aus unabhängiger Abnahme

Luna, einzig schreibender Thread dieses Pakets. Keine Unterthreads oder Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a. AUFTRAG.md gilt, keine Code-Kommentare, Graphify zuerst. Keine Produktion, echten Matchabfragen, Downloads oder Merges. Nur eigener Branch committen/pushen. Weder Tests abschwächen noch Quarantäne umgehen.

Eigener Worktree /home/nathanael/.worktrees/brain-pre-g5-match-fix-20260929, Branch fix/pre-g5-match-contract-20260929, clean auf 34a2507. Relevanter A-Produktcode 8488891, Bericht in A-REPORT.md. Unabhängige Mängelliste /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/REVIEW-AC.md, eingefrorener Reviewcommit e5de1dd.

Exakt R-AC-A1 und A2 beheben:
- A1: Die URL setzt keinen include_player_*-Parameter, aber der Validator verlangt players. Tatsächlicher Upstream `deadlock-api/deadlock-api`, Datei api/src/routes/v1/matches/bulk_metadata.rs am Reviewpin 290cedba6cca7d8a07015e9feefecd22de27643c: ohne explizite Spielerprojektion kommt kein players-Array. include_player_info ist default false. Passenden Parameter explizit anfordern und identisch in Locatorprüfung und positiven Fixtures verwenden. Quellenvertrag lesend prüfen, keine erfundene API-Form.
- A2: Match-Gegenproben mit hero_id als Objekt, unbekanntem new_private_players und widersprüchlichem/ungültigem accountId wurden ACCEPT. Der lokale OpenAPI-Hash ist kein Inhaltsvalidator. Tatsächlich angeforderte und gespeicherte Projektion strikt typisieren/validieren; unbekannte Felder/Typen und ungültige Identitätsaliase vor CoreDocument/Batch ablehnen oder quarantänisieren. Nicht den kompletten ungeprüften Roh-Row als Fakt ausliefern. Provenienz/Raw-Hash und bestehende Idempotenz/Revoke-Semantik erhalten.

Exklusiver Dateiscope: rust/crates/brain-feeds/src/deadlock_match.rs, src/bin/brain-match-ingest.rs, tests/match_store.rs und bei zwingendem Bedarf rust/crates/dbrain-sources/src/deadlock_api.rs sowie zugehörige Match-Fixtures. Keine Analytics-, Serve-, Storage-, Manifest- oder Lockfile-Änderungen. Diese Pfade gehören parallel A34. Neue Abhängigkeit nicht nötig, bei Konflikt melden statt fremde Dateien anfassen. Keine globale Formatierung.

Beweisziel: upstreamgetreuer Query-zu-Antwort-Test positiv mit Spielerprojektion; fehlende Projektion oder fremde/ungültige Spielerstruktur geschlossen abgewiesen; alle im Review angegebenen Drift-Gegenfälle rot im alten und abgewiesen im korrigierten Pfad; bestehender privater Store-/Release-/Revoke-Vertrag weiter grün. Lokale Fixtures/Wegwerf-PG statt echter Spieler. Passende cargo fmt/clippy/test locked offline und eigener gate_hook.py --review gegen Basis 34a2507 vor Abgabe, exakte Befehle/Exitcodes im A12-REPORT.md unter .tasks/2026-09-29-technical-closeout/.

Abgabe: eigener Branch gepusht, Commits und Bericht. Keinen separaten PR eröffnen: Orchestrator übernimmt nur diese Fixcommits in den bestehenden A-PR #59. Niemals in den A-Worktree schreiben, nicht eigenständig mergen. Bump-up bei größerer Grenze: [Bump-up] Paket A12: Grund: ... Erledigt: ... Worktree: ... Offen: ...

## Nachprüfung Runde 2

Reviewer 52c34332 meldet nach eigener Gegenprobe am 2026-09-29 um 13:03 UTC: A1 bleibt BLOCK. Die erzeugte URL aktiviert zusätzlich standardmäßige Match-Infofelder, die dein strikter Projektionsvalidator verwirft. A2 weist die drei ursprünglichen Drift-Gegenfälle jetzt korrekt ab. A1 auf deinem bestehenden Stand 93468b0 weiter beheben.

Nicht erneut nur einen positiven Fixture-Row von Hand schreiben. Sämtliche tatsächlich wirksamen Auswahlparameter samt Upstream-Defaults am bereits belegten API-Pin prüfen und die vollständige resultierende SELECT-/Antwortform mit dem Validator abgleichen. Unbenötigte Match-Infofelder explizit per echtem API-Parameter abwählen oder ihre vertraglich belegte Projektion strikt validieren. Querybuilder, Locator und Tests müssen dieselbe Projektion beschreiben. Keine Unknown-Field-Prüfung abschwächen. Regression für den vom Reviewer reproduzierten Default-Feldfall ergänzen; keine echte Matchabfrage. Derselbe Dateizaun, Test-/Gate-/Pushweg, kein neuer Thread/PR und kein Merge.

