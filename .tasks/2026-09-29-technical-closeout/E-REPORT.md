status: erledigt
Datum: 2026-09-29

# Paket E: Provider- und Fixture-Prüfung

## Stand

- Patchnotes: `db1d36398d70e64b501d4a720bcfa2ba1b25ab10` (`origin/main` zum Prüfzeitpunkt).
- Steam: `8c3fc6e0e8f5ab1c33a43a181c1eee5667818837` (`origin/main` zum Prüfzeitpunkt).
- Brain: `305df2d36ec7b5d0513d6c0769051b41538d6a1b` (`review/pre-g5-providers-20260929`).
- Provider-Worktrees sind sauber. Steam-Testkonfiguration `.github/ci/test-db.conf` wurde auf `postgresql://ci@127.0.0.1:5432/steam_ci` zurückgesetzt; der Diff ist leer.
- Provider- und Brain-Produktivcode blieb unverändert. Es gab keinen echten Steam-GC-Aufruf, keinen Build-Publish, Deploy oder Merge.

## Vertragsnachweise

1. Patchnotes `brain.feed.patchnotes.v1`: Provider validiert die Feed-Felder, prüft Raw-Hash, Post-ID sowie `source_revision` und `export_revision`, sortiert stabil und begrenzt Posts und Antwortgröße. Die Brain-Fixture wird vom Brain-Adapter erneut gehasht und erfolgreich verarbeitet.
2. Steam `brain.build_publish.v1`: POST und GET, Authentifizierung, Schema- und Größenprüfung, Request-Hash-Konflikt als 409, Datenbank-Transaktion für Task und Request-Mapping sowie persistenter Status sind abgedeckt. Der DB-Test belegt parallele gleiche Anfragen mit genau einem Task und Statusabruf nach Rekonstruktion. Migration und Tabellen-Constraints wurden gelesen.
3. Die Steam-Task- und GC-Antwortpfade wurden statisch geprüft. GC-Fehler laufen als Taskfehler zurück, werden in `runner.rs:384` protokolliert und als `FAILED` persistiert. Die HTTP-/DB-Tests rufen den Steam-Game-Coordinator nicht auf. Damit ist die Task-Warteschlange geprüft, ein realer äußerer Publish-Effekt nicht.

## Befunde

1. **Patchnotes-Fehlerdiagnostik:** `patchnotes-brain-verification-20260929/brain_feed.py:328-329` fängt unerwartete Feed-/DB-Fehler ohne Logeintrag und antwortet mit 503. Die Antwort bleibt fail-closed und verrät keine Treiberdetails, aber Betreiber sehen Ursache und Kontext nicht im Provider-Log. Wiederholte Exception-Stellen in `patchnotes_db.py:94-109` betreffen Poolstart und sind getrennt davon geprüft. Zwillingssuche: ein Laufzeit-Catch in `get_feed`; Parsing-Catches sind keine Datenbankpfade.
2. **Steam-Fehlerdiagnostik:** `steam-brain-verification-20260929/rust/crates/steam-web/src/routes/builds.rs:275-280,306-319,350-356` verwerfen die Store-Fehlertexte beim Mapping auf HTTP 500. Die drei Datenbankgrenzen (Einreihen, Status per Request-ID, Task-Outcome) weisen denselben Fehlerpfad auf. Die Speicherung bleibt fehlgeschlagen sichtbar, der HTTP-Server protokolliert den DB-Grund dort jedoch nicht. Gleiche Request-ID ist beim Einreihen idempotent wiederholbar.
3. **Grenze des Steam-Nachweises:** Die Tests belegen persistente API-Idempotenz und Taskzustände, nicht die Wiederholbarkeit eines unklaren Steam-GC-Timeouts. Der echte GC-Pfad wurde aus Sicherheitsgründen nicht ausgeführt; seine externe Wirkung bleibt unbewiesen.

## Testnachweis

- Patchnotes, Arbeitsverzeichnis `/home/nathanael/.worktrees/patchnotes-brain-verification-20260929`: `uv run --with aiohttp==3.14.3 python -m unittest discover -s tests -p 'test_brain_feed.py' -v`; Exit 0, 33 Tests.
- Brain, Arbeitsverzeichnis `/home/nathanael/.worktrees/brain-pre-g5-providers-20260929/rust`: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p brain-feeds --locked --offline -- --include-ignored`; Exit 0, 9 bestanden.
- Steam, Arbeitsverzeichnis `/home/nathanael/.worktrees/steam-brain-verification-20260929/rust`: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p steam-web --features testing --locked --offline -- --include-ignored`; Exit 0, 85 bestanden.
- Steam-Vertrag, gleiches Arbeitsverzeichnis: `PATH=/home/nathanael/.cargo/bin:$PATH cargo +1.97.1 test -p api-contract --locked --offline -- --include-ignored`; Exit 0, 24 bestanden.
- Summe: 151 bestanden, 0 fehlgeschlagen, 0 ignoriert. Historische Vorläufe scheiterten an falschem unittest-Modulpfad, fehlendem `aiohttp`, ungültigem Unix-Socket-URI beziehungsweise der lokalen CI-Postgres-Authentifizierung. Die korrigierten isolierten Läufe liefen gegen Wegwerf-Postgres mit Peer-Authentifizierung; es wurden keine Passwort- oder DSN-Umgebungsvariablen gesetzt.

ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt done | Artefakt: .tasks/2026-09-29-technical-closeout/E-REPORT.md
TESTNACHWEIS[TW-1]: 151 passed, 0 ignored | Baseline: 0 rot
WIRKUNGSPRUEFUNG[WP-1]: 3 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 1/1 geprüft
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: .tasks/2026-09-29-technical-closeout/E-REPORT.md
