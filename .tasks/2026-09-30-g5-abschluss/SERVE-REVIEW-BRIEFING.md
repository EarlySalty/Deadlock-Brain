status: aktiv
Datum: 2026-09-30

# Bestehender unabhängiger Reviewer: statische Servevorbereitung

Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a. Du bist der einzige Reviewerthread für dieses Paket. Keine Unterthreads oder Unteragenten. Keine Code-Kommentare oder Produktänderungen.

Bisheriger Review ist abgeschlossen. Jetzt eigenständiger statischer Bericht zur Servevorbereitung, nicht den ganzen Produktdiff neu auditieren:
/home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-30-g5-abschluss/SERVE-VORBEREITUNG.md

Gegen tatsächlich vorhandenen Code und Vertrag prüfen: Brainbasis 9a29b81d230c01e5c03423cc34ba34c1074eab69 im Quellworktree /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930. Twitch-Momentaufnahme d828481624d53408e0c0a4c3ed1a8e4a6d421c40 im Repo /home/nathanael/repos/Deadlock-Twitch-Bot ausschließlich eingefroren lesen, nicht dessen wechselnden HEAD als gleich annehmen. Fremde Worktrees nicht ändern.

Eigenen bekannten Berichtworktree /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929 und Branch review/pre-g5-core-abnahme-20260929 nur als Berichtssenke verwenden, nicht als Produktprüfbasis. Letzter eigener Head 035e2a99. Bindung zuerst prüfen.

Fragen: Stimmen Port-/Config-/Secret-/Unit-/Readinessanforderungen mit dem bestehenden Code überein? Vermeidet der Plan eine neue Architektur und eigenmächtige Produktmodellwahl? Sind Rollback, Writer-Fencing und ausstehende DB-/Livebeweise ehrlich getrennt? Fehlt eine konkrete Startvoraussetzung oder behauptet die Vorbereitung zu viel? Livebeobachtungen der Hauptsession sind berichtete Daten, nicht deine eigene Liveprüfung.

Nur statisch, Graphify zuerst, KEIN Cargo/Compiler/Test/Fetch, keine Dienste, Modelle, DB-Verbindungen oder Secrets. Beide Metadatenläufe, Clippy und Formatcheck sind inzwischen bestanden, aber das ist kein Test-/Serve-Livebeweis. Keine erneute Slotfreigabe erfinden. Statischer Test-Safety-Bericht entsteht parallel bei Sol und ist nicht Teil dieses ersten Reviewauftrags.

Ausgabe .tasks/2026-09-30-g5-abschluss/SERVE-VORBEREITUNG-REVIEW.md im eigenen Reviewworktree: statisches GO/BLOCK, nummerierte konkrete Befunde, geprüfte Quellstellen und verbleibende Beweisgrenzen. Nur eigenen Bericht committen/pushen, kein Merge. Bei Blocker an Intent melden: [Bump-up] Paket Servevorbereitung: Grund: ... Erledigt: ... Worktree: ... Offen: ... . Ergebnis/SHA sofort melden. Keine neue Wache.
