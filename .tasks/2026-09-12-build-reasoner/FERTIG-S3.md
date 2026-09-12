# FERTIG-S3: Autoren-Scan einmal je Held

Branch `fix/autoren-scan-je-held`, Commit `0cd59bf`
(`fix(steam-core): discovery je held statt je autor`), nach `origin` gepusht.

## Proto- und Testbeleg

`rust/crates/deadlock-proto/protos/deadlock/citadel_gcmessages_client.proto:1361-1368`
definiert `CMsgClientToGCFindHeroBuilds`. `author_account_id` und `hero_id`
sind beide `optional`; `language` ist wiederholt. Der neue Scan setzt nur
`hero_id` und `language: [0, 0]`, das Autorenfeld bleibt leer.

Der lesende Handler-Test in
`rust/crates/steam-core/src/task/handlers/builds/catalog.rs:1185-1211`
prüft je Anfrage einen echten `hero_id` und
`request.author_account_id == None`. Der Zuordnungstest
`catalog.rs:1213-1246` liefert Builds zweier Autoren in einem Heldenblock und
prüft, dass beide Autoren clientseitig korrekt aktualisiert werden. Der
GC-Aufruf ist in diesen Tests simuliert; der Live-Nachweis bleibt beim
Orchestrator nach dem Merge.

## Änderung

- `discovery.rs:19-21,42-88`: Discovery-Payloads akzeptieren `hero_ids`;
  die Planung erzeugt sortierte 100er-Heldenblöcke ohne
  `author_account_id`. Alte Payloads mit `author_account_id` bleiben
  verarbeitbar.
- `discovery.rs:170-420`: Ein Block fragt den GC einmal je Held mit Sprache
  `[0, 0]` ab, dedupliziert und upsertet alle Build-Quellen. Treffer werden
  über `author_account_id` des Builds den beobachteten Autoren zugeordnet.
  Status und Nachricht enthalten Build- und Heldenzahlen; Autoren ohne Treffer
  erhalten `partial` mit `keine Builds im Katalog`. Je Block wird
  `Helden-Scan beendet` mit Helden, Builds und Autoren-Treffern geloggt.
- `catalog.rs:96-116`: Der Zyklus plant Heldenblöcke statt Autorentasks und
  meldet `discoveryUnits`.
- Blockgröße: 100 Helden. Bei rund 3,6 Sekunden je Held sind das rund 360
  Sekunden und damit unter dem bestehenden 480-Sekunden-Taskbudget.

## Task-Zahl

Vorher: 13 Discovery-Tasks je Zyklus für die 13 beobachteten Autoren.
Nachher: `ceil(aktive Helden / 100)` Discovery-Tasks; beim live bekannten
Bestand von 38 Helden also 1 Task statt 13. Maintenance-Tasks bleiben
unverändert.

## Tests und Selbstprüfung

- S2-Baseline aus `FERTIG-S2.md`: 174 Offline-Tests; 34 zentrale
  DB-Katalogtests.
- S3-Endstand: 174 Offline-Tests bestanden, 0 fehlgeschlagen; 36 zentrale
  DB-Katalogtests bestanden, 0 fehlgeschlagen, 235 gefiltert.
- `rustfmt` auf den beiden geänderten Rust-Dateien und `git diff --check`:
  grün.
- Clippy wurde mit `-p steam-core --all-targets --features testing -- -D
  warnings` ausgeführt. Der Lauf bleibt am bereits aus S2 bekannten,
  unveränderten Feld `gc_health.rs:45` (`RateLimitOutcome::cooldown`) hängen;
  im S3-Diff gibt es keine Clippy-Warnung.
- Keine Änderungen an Proto, Lanes, Runner, Migrationen oder Prod-Daten.
