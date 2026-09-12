# Paket S, Zwischenbefund vor dem Fix

Stand: 2026-09-12. Branch: `feat/autoren-scan-reaktivieren`.

## Ursache mit Belegen

Der Null-Lauf ist ein Scheduling- und Handler-Fehler, kein belegter GC-Proto-
Fehler:

- `rust/crates/steam-core/src/task/handlers/builds/discovery.rs:17-23` und
  `:29-33` geben bei beiden Discovery-Task-Typen sofort `skipped: true` und
  `legacy: true` zurück. Die bereits vorhandene echte Scan-Logik beginnt erst
  bei `:48` in `run_discovery`.
- `rust/crates/steam-core/src/task/handlers/builds/catalog.rs:87-102`
  ruft `run_discovery` ausschließlich aus `BUILD_CATALOG_CYCLE` auf.
- `rust/crates/steam-core/src/main.rs:249-283` stellt den einzigen laufenden
  Tages-Timer dar, enqueued aber nur `MAINTAIN_BUILD_CATALOG`; ein
  `BUILD_CATALOG_CYCLE` wird dort nicht eingeplant. Damit wird der Autoren-Scan
  im Regelbetrieb nicht erreicht. Die Dedup-Abfrage in
  `rust/crates/steam-persistence/src/builds.rs:923-935` prüft ebenfalls nur
  `MAINTAIN_BUILD_CATALOG`.
- `git blame` zeigt, dass die beiden No-op-Stubs mit Commit `211a6474`
  eingeführt wurden. Der Branch steht auf `796bb3c`, also ist dieser Zustand
  unverändert vorhanden.
- Der aktuelle User-Journal-Check für `steam-core` und `steam-bot` enthält
  keinen passenden `DISCOVER_*`-, `BUILD_CATALOG_CYCLE`- oder
  Autoren-Scan-Lauf. Der aktive `steam-core` läuft zudem aus
  `/home/nathanael/Documents/Deadlock-Steam-Bot/rust/target/release/steam-core`
  und ist damit kein Lauf des bearbeiteten Worktrees.

Die GC-Schnittstelle ist im Repo weiterhin passend vorhanden:
`CMsgClientToGCFindHeroBuilds` hat `author_account_id`, `hero_id`, `language`,
`search_text`, `hero_build_id` und `tags`; die Antwort kennt `k_eSuccess = 1`
und `k_eTooBusy = 2` in
`rust/crates/deadlock-proto/protos/deadlock/citadel_gcmessages_client.proto:1361-1389`.
`run_discovery` behandelt Nicht-1 bereits als sichtbaren Response-Fehler und
wartet mit `GC_REQUEST_DELAY` zwischen Autoren. Es gibt daher vor dem Fix
keinen Beleg dafür, dass geänderte Proto-Felder oder ein leerer GC-Katalog die
Ursache des historischen Null-Laufs sind. Rate-Limit-Code 5 wurde in den
geprüften aktuellen Journals nicht als Autoren-Scan-Antwort gefunden.

## Geplanter Fix

Den Tages-Scheduler auf den bestehenden `BUILD_CATALOG_CYCLE` umstellen, die
beiden manuellen Discovery-Handler an dieselbe `run_discovery`-Logik anbinden,
die vorhandene Persistenz in `tierlist.hero_build_sources` weiterverwenden und
den Lauf mit dem vorhandenen GC-Abstand seriell halten. Danach folgen Tests,
Autoren-ID-Ermittlung und die vollständige Fertigmeldung.
