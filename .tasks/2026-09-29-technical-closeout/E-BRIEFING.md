status: aktiv
Datum: 2026-09-29

# Paket E: bestehende Provider-Verträge final prüfen

Gemeinsamer Auftrag: /home/nathanael/.worktrees/brain-technical-closeout-20260929/.tasks/2026-09-29-technical-closeout/AUFTRAG.md. Luna, einziger Thread für E, keine Unteragenten. Intent 562a877b-0939-440a-964d-1145d9e9431a; Bump-up-Format im Auftrag. Keine Code-Kommentare.

Eigener Bericht-Worktree /home/nathanael/.worktrees/brain-pre-g5-providers-20260929
Branch review/pre-g5-providers-20260929, Basis 305df2d36ec7b5d0513d6c0769051b41538d6a1b, clean.

Referenz wörtlich: „Nicht neu implementieren, sondern final überprüfen.“ und „Keinen echten Build veröffentlichen.“

Frisch bestätigter Git-Stand:
- Patchnotes #49 MERGED als 06c50ab4205fe854f2c97969fa5c68ee0669b775, aktuelles origin/main db1d36398d70e64b501d4a720bcfa2ba1b25ab10. Historische Security-Jobs rot, Grund prüfen, nicht Billing unterstellen.
- Steam #73 MERGED als d6b6852e2dbc4341f121464b1e55e394575f6198, aktuelles origin/main 8c3fc6e0e8f5ab1c33a43a181c1eee5667818837. Historische clippy/test/audit-Jobs rot, aktuelle lokale Tests maßgeblich aber Required-Check-Regeln nicht umgehen.
- Schema Bots #461 MERGED als c421d610b0f2ca7308d90a7a342569755ba12050.

Eigene Provider-Testworktrees von den aktuellen Remote-Heads anlegen, nicht fremde dirty Worktrees benutzen: /home/nathanael/.worktrees/patchnotes-brain-verification-20260929 mit review/brain-provider-20260929 und /home/nathanael/.worktrees/steam-brain-verification-20260929 mit review/brain-provider-20260929. Erst lokalen Worktree-/Branchbestand überprüfen. Dort nur gezielte Provider-Tests und kleine tatsächliche Fixes. Kein pauschales Refactoring/Formatieren, kein Legacy-Python fixen. Bei größerem Defekt Fundstelle/Scope für Sol liefern.

Patchnotes brain.feed.patchnotes.v1: Raw Hash/Post-ID/source_revision/export_revision/URL/title/timestamp, response limits, über 5000 Posts, oversized post, Auth/Config, deterministische Ordnung. Brain-Fixture auf exakte Kompatibilität prüfen, Fixturefix in Brain erlaubt, Source-Adapter brain-feeds und dbrain-sources gehören Paket A und dürfen nicht verändert werden.

Steam brain.build_publish.v1: POST /builds/v1/publish und GET /builds/v1/publish/{request_id}, persistente UNIQUE request_id + request_sha256 + steam_task in derselben Transaktion. Gleicher Payload keine zweite Task, anderer Payload 409, Parallelität genau eine Task, Status nach Neustart, queued/running/succeeded/failed, hero_build_id, stabile timestamps, Auth fail closed, oversized/unknown fields. Keine Brain-DB-Credentials in Steam und umgekehrt. Gegen Wegwerf-Postgres/Peer und Mock-Steam testen, niemals echten Steam-Task oder Build absenden. Migration niemals ändern, neue nur bei konkretem Befund.

Graphify zuerst. Test-Wächter lesen. Kein produktiver DB-Zugriff, keine Secrets/Passwort-ENV. Für nicht gestartete GH-Jobs konkrete run annotations/Billing-Belege sichern. Bekannte rote historische Checks nicht als heutigen Codefehler ausgeben.

Bericht .tasks/2026-09-29-technical-closeout/E-REPORT.md im Brain-Worktree. SHA/Befehle/Exitcodes/ignores und konkrete Vertragsnachweise. Eigene Branches commit/push erlaubt. Brain-Berichts-/Fixture-PR nach migration/rust-integration, eventuelle Providerfix-PRs Draft nach main, NIE selbst mergen. Kein Production-Cutover, kein Deploy. Kleine Fixes prüfen und gate_hook.py --review, größere an Orchestrator. Keine generische TODO-Liste als Ersatz für Abnahme.
