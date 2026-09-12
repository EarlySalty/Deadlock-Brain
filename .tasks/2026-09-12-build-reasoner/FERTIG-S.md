# Paket S, Autoren-Scan reaktiviert

Stand: 2026-09-12. Branch: `feat/autoren-scan-reaktivieren`.

## Ursache mit Belegen

Der Null-Lauf war ein Scheduling- und Handler-Fehler, kein belegter GC-Proto-
Fehler:

- `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:17-23` und
  `:29-33` gaben bei beiden Discovery-Task-Typen sofort `skipped: true` und
  `legacy: true` zurück. Die bereits vorhandene echte Scan-Logik begann erst
  bei `:48` in `run_discovery`.
- `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:87-102` rief
  `run_discovery` ausschließlich aus `BUILD_CATALOG_CYCLE` auf.
- `rust/crates/steam-core/src/main.rs:249-283` stellte den einzigen laufenden
  Tages-Timer dar, enqueue-te aber nur `MAINTAIN_BUILD_CATALOG`; ein
  `BUILD_CATALOG_CYCLE` wurde dort nicht eingeplant. Die Dedup-Abfrage in
  `rust/crates/steam-persistence/src/builds.rs:923-935` prüfte ebenfalls nur
  `MAINTAIN_BUILD_CATALOG`.
- `git blame` zeigt, dass die beiden No-op-Stubs mit Commit `211a6474`
  eingeführt wurden. Der Branch stand zu Beginn auf `796bb3c`, also war dieser
  Zustand unverändert vorhanden.
- Der aktuelle User-Journal-Check für `steam-core` und `steam-bot` enthielt
  keinen passenden `DISCOVER_*`-, `BUILD_CATALOG_CYCLE`- oder
  Autoren-Scan-Lauf. Der aktive `steam-core` lief zudem aus
  `/home/nathanael/Documents/Deadlock-Steam-Bot/rust/target/release/steam-core`
  und war damit kein Lauf des bearbeiteten Worktrees.

Die GC-Schnittstelle ist im Repo weiterhin passend vorhanden:
`CMsgClientToGCFindHeroBuilds` hat `author_account_id`, `hero_id`, `language`,
`search_text`, `hero_build_id` und `tags`; die Antwort kennt `k_eSuccess = 1`
und `k_eTooBusy = 2` in
`rust/crates/deadlock-proto/protos/deadlock/citadel_gcmessages_client.proto:1361-1389`.
`run_discovery` behandelte Nicht-1 bereits als sichtbaren Response-Fehler und
wartete mit `GC_REQUEST_DELAY` zwischen Autoren. In den geprüften aktuellen
Journals gab es keinen Autoren-Scan mit Rate-Limit-Code 5. Es gibt daher keinen
Beleg für geänderte Proto-Felder, einen leeren GC-Katalog oder Code 5 als
Ursache des historischen Null-Laufs.

## Fix

- Der 24-Stunden-Scheduler enqueued jetzt `BUILD_CATALOG_CYCLE`; die bestehende
  Dedup-Abfrage berücksichtigt beide Katalog-Task-Typen.
- `DISCOVER_WATCHED_BUILDS` und `DISCOVER_BUILDS_VIA_HEROES` rufen jetzt die
  vorhandene `run_discovery`-Logik auf. Sie lesen aktive
  `watched_build_authors`, fragen den GC seriell mit dem bestehenden Abstand ab,
  persistieren jede Build-Quelle in `tierlist.hero_build_sources` und geben
  Autoren-, Helden-, Neu-, Update- und Fehlerzahlen zurück.
- DB-Lookup- und Upsert-Fehler werden nicht mehr verschluckt. Das Upsert hält
  zusätzlich Autor, Held, Sprache und Veröffentlichungszeit aktuell; `details`
  mit `modCategories` und `abilityOrder` wird aus der GC-Antwort übernommen.
- Es wurden keine Lobby-, Rang- oder sonstigen Poll-Schleifen geändert. Es gab
  keine Migration.

## Autoren-ID-Ermittlung

Die IDs wurden über die bestehende Deadlock-API ermittelt, nicht geraten:

- `Lightbringer`: Steam-Account-ID `13446690`.
- `Situation`: Steam-Account-ID `34634349`.
- `Warden`: Hero-ID `25`.

Die Builds-Suche lieferte außerdem den exakten Buildnamen
`LIGHTBRINGERxSITUATION WARDEN BUILD` mit GC-Build-ID `779996`, Version `31`,
Hero-ID `25` und aktuellem GC-Autor `1650097169` (`juice`). Unter dem
Lightbringer-Autor existiert zusätzlich derselbe exakte Name als Build-ID
`782057`. Der aktuelle GC-Autor des Kollaborations-Builds ist ein
Verifikationshinweis, kein zusätzlicher Eintrag in der unten vorgeschlagenen
Anforderung für Lightbringer und Situation.

Vorschlag für die neue Prod-Datenpflege, nicht ausgeführt:

```sql
INSERT INTO tierlist.watched_build_authors
    (author_account_id, notes, is_active)
VALUES
    (13446690, 'Lightbringer', true),
    (34634349, 'Situation', true)
ON CONFLICT (author_account_id) DO UPDATE
SET notes = EXCLUDED.notes,
    is_active = EXCLUDED.is_active;
```

Diese SQL-Anweisung wurde nicht gegen Prod ausgeführt.

## Tests und Belege

Baseline war ein frischer Worktree auf `796bb3c`; der gleiche Offline-Testlauf
war dort und im Feature-Worktree erfolgreich.

- `SQLX_OFFLINE=true cargo test -p steam-core -p steam-persistence --manifest-path rust/Cargo.toml`
  Baseline: `174 passed`, Persistence ohne aktivierte DB-Testfeatures: `0`.
  Endstand: `174 passed, 0 failed`; Persistence: `0` ohne DB-Testfeatures.
- Wegwerf-Postgres über
  `rust/scripts/central_test_db.sh`, Migrationen erfolgreich:
  `cargo test -p steam-core --features testing -- task::handlers::builds::catalog::tests --include-ignored`:
  `18 passed, 0 failed`.
- Derselbe Docker-Test für
  `cargo test -p steam-persistence --features testing --test build_catalog_transactions -- --include-ignored`:
  `2 passed, 0 failed`.
- Ein vollständiger Docker-Testlauf scheiterte in einem unabhängigen
  API-Test, weil das zentrale Testschema die Lobby-Tabelle enthält; vor dem
  Abbruch waren `252/253` steam-core-Tests erfolgreich. Dieser Fehler lag nicht
  im geänderten Katalogpfad.
- Striktes Clippy mit `-D warnings` ist im Gesamtpaket weiterhin durch die
  vorbestehenden Warnungen `gc_health.rs:45` und
  `task/handlers/scrim_lobby.rs:852` blockiert. Es wurde kein unabhängiger
  Clippy-Fehler in den geänderten Katalogdateien festgestellt.
- `git diff --check` ist sauber. Ein globaler `rustfmt --check` zeigt nur
  vorbestehende Abweichungen in `gc_health.rs`, `task/handlers/gc.rs` und
  `task/runner.rs`; diese Dateien wurden nicht verändert.

## Live-Proof nach Deployment

Nach Migration beziehungsweise Service-Deployment einmalig ausführen, nicht
in Prod vorab:

```bash
TASK_JSON="$(curl -fsS -X POST http://127.0.0.1:8782/tasks \
  -H 'Content-Type: application/json' \
  -H "X-Internal-Token: ${STEAM_CORE_API_TOKEN}" \
  --data '{"task_type":"BUILD_CATALOG_CYCLE","payload":{},"bot_account_id":1}')"
TASK_ID="$(printf '%s' "$TASK_JSON" | jq -r '.id')"
curl -fsS -H "X-Internal-Token: ${STEAM_CORE_API_TOKEN}" \
  "http://127.0.0.1:8782/tasks/${TASK_ID}"
```

Danach in der zentralen DB prüfen:

```sql
SELECT hero_build_id, hero_id, author_account_id, name, version,
       last_updated_at, details->'modCategories' AS categories,
       details->'abilityOrder' AS ability_order
FROM tierlist.hero_build_sources
WHERE hero_id = 25
  AND lower(name) = lower('LIGHTBRINGERxSITUATION WARDEN BUILD')
ORDER BY last_updated_at DESC;
```

Erwartet wird mindestens der Warden-Eintrag mit den GC-Metadaten; die
Autoren-ID kann beim Kollaborations-Build `1650097169` sein, solange der
GC-Autor nicht auf den beobachteten Autorenaccount normalisiert.

## Branch und Deploy

- Branch: `feat/autoren-scan-reaktivieren`.
- Nur dieser Branch wird gepusht; `main` bleibt unberührt.
- Deploy-Voraussetzung: Build und Service-Neustart des `steam-core` aus diesem
  Branch; keine DB-Migration erforderlich. Danach den Live-Proof ausführen.
- Commit: `f82c21c41052c51b301a4a875880114fb62b6a50`.
- Push: `origin/feat/autoren-scan-reaktivieren` erfolgreich; `main` wurde nicht
  gepusht oder verändert.
