# Review-A (Runde 1): build-reasoner, Paket A (Fundament)

status: NACHBESSERN (2026-09-12)

Geprueft: Branch `feat/build-reasoner-a` im Worktree
`/home/nathanael/.worktrees/deadlock-brain-a`, Diff `29bb842..HEAD` (Commit
3bef602). Lesend, kein Code geaendert, kein Branch angefasst. Vergleichsbasis:
ARCHITEKTUR.md (1, 2, 5, 6, 9), Briefing-A, VORCHECK-ERGEBNIS.md, PAKETE.md.

## Testnachweis

```
cargo fmt -p dbrain-reasoner -- --check    -> exit 0
cargo clippy -p dbrain-reasoner --all-targets -> exit 0, keine Warnung
cargo test -p dbrain-reasoner              -> exit 0
```

TESTNACHWEIS[TW-1]: 7 passed, 1 ignored | Baseline: neues Crate, 0 rot

Der ignorierte Test (`loads_warden_and_reference_items_from_real_snapshot`)
braucht `DEADLOCK_CENTRAL_DSN`. In dieser Review-Session war kein
Infisical-Service-Token gesetzt (`scripts/export_infisical_env.py` bricht mit
fehlender Pflicht-Env ab), der Lauf gegen echte Daten war hier also nicht
moeglich. Die Folge steht in Mangel 4.

## Spec-Treue der Typen

`types.rs` deckt Abschnitt 5 vollstaendig und namens- wie feldgenau ab: alle
Enums, Structs, `ReasonerConfig`, `ReasonerCtx`, `ReasonerError`, bis hin zu
`MetaIndex`/`MetaSupport` (laut Spec in `types.rs`, Konstruktion spaeter in C).
Keine Abweichung, B und C koennen gegen diese Typen bauen.

## Maengel

### 1. Modellbildung liegt in A statt B (blockierend fuer den Start von B)

`data.rs` baut die fertigen B-Modelle selbst: `load_hero_model` gibt
`Result<HeroModel>` zurueck und ruft intern `hero_model`, `ability_model`,
`weapon_profile`, `ability_roles`, `scaling_stats` (data.rs:208-443);
`load_item_models` gibt `Result<Vec<ItemModel>>` und enthaelt `classify_condition`,
`property_values`, `passive_property_values` (data.rs:102-206, 522-683).

ARCHITEKTUR Abschnitt 6 (Prosa nach dem Signaturblock) und Abschnitt 7 weisen
`hero::build_hero_model`, `item::build_item_model`, `classify_condition`
ausdruecklich Paket B zu ("liefern rohe serde_json::Value-Payloads plus
Katalogzeilen; die Modellbildung liegt in Paket B. Damit haengt A nicht an B"),
Abschnitt 9 Schritt 1 und 2 ebenso ("load_hero_model plus hero::build_hero_model
erzeugen HeroModel"). Abschnitt 13 verlangt, dass B dabei auf
`dbrain-learn::build_optimizer` aufsetzt, statt die Payload-Auswertung neu zu
schreiben.

Bewertung: Die Signaturen in Abschnitt 6 sagen `-> Result<HeroModel>` bzw.
`-> Result<Vec<ItemModel>>`, und Briefing-A Punkt 3 sagt "gibt A-Typen zurueck".
Der Worker hat die Signatur und das Briefing wortwoertlich umgesetzt, die
Entscheidung ist also vertretbar. Sie widerspricht aber der Prosa derselben
Abschnitte und reimplementiert die Payload-Auswertung von Hand, ohne die in
Abschnitt 13 vorgeschriebene Wiederverwendung.

Folge fuer B: `build_hero_model`, `build_item_model` und `classify_condition`
haben keinen Aufrufer mehr oder doppeln A. B muss neu zugeschnitten werden
(nur Scoring und Mechanik) oder A muss rohe Payloads liefern. Das ist eine
Orchestrator-Entscheidung und muss vor dem Start von B fallen. Vorschlag:
B auf `score_items`/`mechanics` beschneiden, die Modellbildung in A belassen und
die Prosa in Abschnitt 6/7/9 angleichen; die fehlende `dbrain-learn`-
Wiederverwendung als bewusste Schuld vermerken.

### 2. Patch-Events ohne Alias-Aufloesung (wichtig)

`load_patch_events` (data.rs:713-728) matcht nur den einen kanonischen
Heldennamen: `lower(pe.entity_name)=lower($1) OR lower(pee.secondary_entity_name)
=lower($1)`. `hero_name` (data.rs:492-513) liefert nur `canonical_name` bzw.
`payload->>'name'`, keine Aliase.

Briefing-A Punkt 2 und der Vorcheck (Abschnitt 4) verlangen Alias-Aufloesung
ueber `brain.entities` plus `brain.entity_aliases` (Alias-Kinds
canonical/snapshot_name/class_name_short), genau wie der bestehende Lesepfad
`build_optimizer.rs:2403`. Folge: Events, die unter einem Alias oder
`class_name_short` erfasst sind, fehlen; Paket C verliert Eingabe fuer das
Patch-Delta bei Helden mit abweichenden Namen. Fuer Warden aktuell folgenlos
(keine Events), aber die Ladeschicht ist das Fundament fuer alle Helden.
Vorschlag: Namensmenge wie in build_optimizer aufbauen und `entity_name = ANY($1)`
binden.

### 3. load_author_builds: ORDER BY auf nicht existierende Spalten (wichtig, latent)

Die ORDER-BY-Klausel (data.rs:826) nutzt `hbs.last_updated_ts` und
`hbs.publish_ts`. Laut Vorcheck Abschnitt 5 heissen die realen Spalten von
`tierlist.hero_build_sources` `published_at`, `last_updated_at`, `fetched_at`,
`last_seen_at`; `last_updated_ts`/`publish_ts` existieren nicht. Zusaetzlich
mischt `COALESCE(... , 0)` timestamptz mit Integer 0, was einen Typfehler gibt.

Der `table_exists`-Guard davor faengt den Fall ab, dass der zentrale Pool das
`tierlist`-Schema gar nicht sieht (dann leere Rueckgabe). Sobald die Tabelle
erreichbar ist, schlaegt die Query zur Laufzeit fehl. Vorschlag:
`ORDER BY COALESCE(hbs.last_seen_at, hbs.fetched_at, hbs.published_at) DESC`,
ohne die Pseudo-Spalten und ohne den Integer 0.

### 4. hero_stat_values/hero_stat_profiles-Schema unbelegt (wichtig)

`load_hero_stat_values` (data.rs:866-884) liest `stat_key`, `numeric_value`,
`entity_id`, `profile_id` aus `brain.hero_stat_values` und `id`/`entity_id` aus
`brain.hero_stat_profiles`. Briefing-A Punkt 3 verlangt ausdruecklich die
Verifikation dieser Spalten am echten Schema (Entscheidung 2 in PAKETE.md). Der
Vorcheck deckt `hero_item_synergies` ab (passt zur Query in data.rs:890), die
beiden `hero_stat_*`-Tabellen aber nicht.

`table_exists` schuetzt nur vor einer fehlenden Tabelle. Existiert die Tabelle
mit anderen Spaltennamen, bricht `load_hero_model` zur Laufzeit ab, weil
`load_hero_model` diesen Pfad immer aufruft (data.rs:518). Da der echte Ladelauf
hier mangels DSN nicht moeglich war, bleibt das unbelegt. Vor Freigabe gegen das
echte Schema bestaetigen oder die Spaltennamen in der Fertigmeldung mit Nachweis
nennen.

### 5. reasoning_effort = "none" nicht gegen den Endpunkt belegt (nit)

`ai_roles.rs:77` setzt `reasoning_effort = Some("none")`. Das Feld ist in diesem
Diff neu in `ChatCompletionRequest`; es gibt keinen bestehenden Aufruf im Repo,
der belegt, dass der Fireworks/Deepseek-Endpunkt den Literalwert "none"
akzeptiert (ueblich sind low/medium/high). Der Test prueft nur die
Serialisierung, nicht die Annahme durch die API. Falls abgelehnt, antwortet jede
KI-Rolle mit Fehler und das Build faellt auf den No-Text-Pfad zurueck. Gegen
einen echten Call oder die Provider-Doku verifizieren.

### 6. Item-Analyst-Filter nicht erzwungen (nit, Koordination fuer C)

Abschnitt 9 Abbruchregel verlangt, dass ein Item-Analyst-Text verworfen wird,
wenn er ein Item nennt, das nicht im Build ist. `parse_item_analyst_response`
(ai_roles.rs:180) kann den Build nicht kennen und filtert nicht. Das ist korrekt
in A nicht loesbar, muss aber in Paket C beim Zusammenfuehren der Warum-Texte
passieren. Hier nur als Weitergabe an C.

### 7. snapshot(): Fuzzy-Treffer kann Exakttreffer ueberholen (nit)

`snapshot` (data.rs:446-456) ODER-verknuepft Exaktvergleich und
`canonical_name ILIKE %name%` mit `ORDER BY fetched_at DESC, id DESC LIMIT 1`.
Ein neuerer Fuzzy-Treffer (anderer Held, der den Namen enthaelt) kann damit
einen aelteren Exakttreffer verdraengen. Mit den aktuellen Daten geringes Risiko.
Sauberer: erst exakt, Fuzzy nur bei Fehltreffer.

## Zur Kenntnis (keine Maengel)

- Orchestrierungs-Fassade (`reason_build`, `reason_patch_impact`,
  `reason_backtest`, Abschnitt 5) fehlt in `lib.rs`. Erwartet: die Bodies haengen
  an B/C, die Delegator-Integration traegt sie nach. Kein Handlungsbedarf fuer A.
- `load_synergies` ist zusaetzlich exportiert (nicht im Signaturblock Abschnitt 6,
  aber in der Datenfluss-Tabelle Abschnitt 4 und von C genutzt). Zulaessige
  Ergaenzung, Query passt zum Schema.
- Sicherheit/Hygiene sauber: keine Secrets, keine Code-Kommentare, keine neuen
  externen Crates (nur Workspace-Deps), kein Modellname im Code, bestehende
  Aufrufer unveraendert (`response_format`/`reasoning_effort` auf None gesetzt).

## Urteil

NACHBESSERN. Blocker vor dem Start von B: Mangel 1 (Orchestrator-Entscheidung zum
Paketschnitt). A-eigene Korrekturen: Mangel 2, 3, 4. Der Rest sind Nits und
Koordinationshinweise.

## Was B und C sofort wissen muessen

- Die fertigen Modelle (`HeroModel`, `Vec<ItemModel>`) kommen bereits aus
  `data.rs`, inklusive `classify_condition` und Ability-Rollen-Heuristik. B darf
  `build_hero_model`/`build_item_model`/`classify_condition` nicht doppelt bauen,
  bis der Paketschnitt entschieden ist (Mangel 1).
- `load_patch_events` liefert derzeit nur Events zum kanonischen Namen, keine
  Aliase (Mangel 2). C sollte sich darauf noch nicht verlassen.
- `load_author_builds` liefert auf dem zentralen Pool leer (Tabelle nicht
  sichtbar) und hat einen latenten SQL-Fehler (Mangel 3). Der Backtest in C hat
  damit vorerst keine Autoren-Basis, das deckt sich mit der bekannten Lage
  (Autoren-Scan tot, Paket S).
- Item-Analyst-Texte muessen in C gegen die Build-Mitgliedschaft gefiltert
  werden (Mangel 6).

## Fixrunde 1

Abgeschlossen am 12.09.2026 im Worktree
`/home/nathanael/.worktrees/deadlock-brain-a`, Branch `feat/build-reasoner-a`.
Code-Commit: `fc74b71`. Ein Thread, keine Unter-Agenten. Mangel 1 bleibt gemäß
Fix-Briefing entschieden; `types.rs` und die Modellbildung wurden nicht geändert.
Die folgenden Zeilen beziehen sich auf `rust/crates/dbrain-reasoner/src/`.

| Mangel | Datei:Zeile | Ergebnis im Stand `fc74b71` |
|---|---|---|
| 2 | `data.rs:713`, `data.rs:492` | Namensmenge aus `brain.entities.canonical_name` und `brain.entity_aliases.alias` für `canonical`, `snapshot_name`, `class_name_short`. Vergleich per `lower(...) = ANY($1)` für Event und Enrichment. Spiel-ID wird über `primary_external_id` auf die interne Entity aufgelöst. Warden: Spiel-ID 25, Entity-ID 175035. Snapshot-Fallback bleibt bei fehlender Entity erhalten. |
| 3 | `data.rs:817`, `data.rs:798` | Ausschließlich `tierlist.hero_build_sources`; fehlende Tabelle oder Spalte ergibt einen DB-Fehler statt leerer Daten. Sortierung nach `COALESCE(last_updated_at, published_at) DESC NULLS LAST`, anschließend `version DESC NULLS LAST`. Zeitstempel werden als Epoch-Sekunden für die bestehenden `Option<i64>`-Felder gelesen, SQL-NULL bleibt `None`. |
| 4 | `data.rs:860`, `data.rs:1036` | Spalten und Typen über `information_schema.columns` am echten Infisical-DSN bestätigt. Stats und Profile werden über die interne Entity-ID statt fälschlich über die Spiel-ID zugeordnet. Echtdaten-Test ausgeführt und um Stats, Patch-Zeilen und Autoren-Builds erweitert; seine Verbindungen erzwingen `default_transaction_read_only=on`. |
| 5 | `ai_roles.rs:77` | `reasoning_effort="none"` bleibt unverändert. Die offizielle Fireworks-Dokumentation bestätigt ausdrücklich für DeepSeek V4, dass `none` das Denken abschaltet. Ein kleiner Aufruf über `AiClient::from_env()` und den bestehenden Rollen-Request-Builder wurde zusätzlich versucht, erhielt jedoch HTTP 404 mit Fehlerparameter `model` und `Model not found, inaccessible, and/or not deployed`. Damit kein erfolgreicher Live-Nachweis; der laut Briefing zulässige Dokumentationsnachweis trägt die Entscheidung. |
| 6 | `ai_roles.rs:180` | Keine Änderung am Parser und keine neue API. Der Reviewer schreibt ausdrücklich: „Hier nur als Weitergabe an C.“ Paket C muss beim Zusammenführen der Warum-Texte die IDs gegen das fertige Build prüfen und fremde Item-Texte verwerfen. |
| 7 | `data.rs:445` | Ein exakter `canonical_name` hat vor exaktem Payload-Namen und Fuzzy-Treffern Vorrang. Erst innerhalb derselben Trefferklasse entscheiden `fetched_at` und `id`. |

Nachweis zu Mangel 5: [Fireworks Create Chat Completion, reasoning_effort](https://docs.fireworks.ai/api-reference/post-chatcompletions#body-reasoning-effort),
abgerufen am 12.09.2026. Keine Änderung von Modell oder Provider. Der
fehlgeschlagene Live-Aufruf war eine einmalige Probe und gehört nicht zur
abschließenden Testsuite. Die Modellverfügbarkeit bleibt als Betriebspunkt für
den Orchestrator offen.

Schema-Nachweis zu Mangel 4:

- `brain.hero_stat_values`: `stat_key text`, `numeric_value double precision`,
  `entity_id bigint`, `profile_id bigint` sind vorhanden.
- `brain.hero_stat_profiles`: `id bigint`, `entity_id bigint` sind vorhanden.
- `brain.entities.primary_external_id` ist `text`; der Join verwendet die
  Spiel-ID als Text, danach die interne `id bigint`.
- `tierlist.hero_build_sources`: `last_updated_at` und `published_at` sind
  `timestamp with time zone`, `version` ist `bigint`.

Testnachweis:

| Lauf | Bestanden | Fehlgeschlagen | Ignoriert |
|---|---:|---:|---:|
| Baseline `3bef602`, ohne DSN | 7 | 0 | 1 |
| Vier neue Regressionstests vor dem Fix, isolierter Postgres | 0 | 4 | 0 |
| Dieselben vier Regressionstests nach dem Fix | 4 | 0 | 0 |
| Endstand ohne DSN, `cargo test -p dbrain-reasoner` | 7 | 0 | 5 |
| Endstand mit echtem DSN und separatem Scratch-DSN, `cargo test -p dbrain-reasoner -- --include-ignored --nocapture` | 12 | 0 | 0 |

Alle vier neuen Regressionstests wurden einzeln rot und anschließend grün:
Alias-Auflösung, Autoren-Schema/Zeitstempel, Entity-Zuordnung der Stats und
Snapshot-Priorität. Der Schreibtest-Pool verwendet ausschließlich
`REASONER_SCRATCH_DSN` und prüft vor Schemaänderungen den Datenbanknamen
`reasoner_a_fix`. Die echte Datenbank wird ausschließlich lesend benutzt.

Der Echtdaten-Test allein: 1 bestanden, 0 fehlgeschlagen. Geladen wurden
1 Held (Warden), 251 Items einschließlich Veil Walker, Mercurial Magnum,
Siphon Bullets und Quicksilver Reload, 283 Stat-Zeilen, 416 Patch-Zeilen
einschließlich Enrichments und 3 Autoren-Builds. Die Stat-Zahl zählt geladene
Zeilen über vorhandene Profile, keine eindeutigen Stats eines einzelnen Profils.
Doc-Tests: 0. `cargo fmt -p dbrain-reasoner -- --check`,
`cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` und
`git diff --check`: jeweils Exit 0.

Selbstprüfung gegen Mängel 2 bis 7 abgeschlossen. Geändert wurde ausschließlich
`data.rs` einschließlich seiner Tests. Dieser Anhang liegt auftragsgemäß in der
hier angegebenen Review-Datei; Git-Commit und Push erfolgen nur auf dem eigenen
Feature-Branch, kein Merge, Push oder Deploy von `main`.

Zugriffsvorfall: Beim ersten `psql`-Verbindungsversuch gab eine Fehlermeldung
versehentlich einen Teil des DSN samt Zugangsdaten im Tool-Ausgabekanal aus.
Spätere Verbindungsfehler wurden unterdrückt. Die Zugangsdaten wurden nicht in
Dateien oder Commits gespeichert. Rotation der betroffenen DB-Zugangsdaten ist
empfohlen und wurde nicht eigenmächtig vorgenommen.

## Review Runde 2

Geprüft am 12.09.2026 gegen die Mängelliste, Stand `fc74b71` im Worktree
`/home/nathanael/.worktrees/deadlock-brain-a` (Diff `3bef602..HEAD`). Lesend,
kein Code, kein Branch. Ein Thread, keine Unter-Agenten.

Nebenwirkung auf Signaturen: keine. Der Diff berührt ausschließlich
`data.rs` (153 Zeilen plus, 26 minus); `types.rs` und `ai_roles.rs` sind
unverändert. B und C bauen weiter gegen dieselben Typen.

Testnachweis (eigener Lauf, ohne DSN, stable-Toolchain aus rustup, da das
System-Cargo 1.75 die Lockfile-Version 4 nicht liest):

```
cargo test -p dbrain-reasoner   -> EXIT=0
test result: ok. 7 passed; 0 failed; 5 ignored
```

TESTNACHWEIS[TW-2]: 7 passed, 5 ignored | Baseline: neues Crate, 0 rot

Die 5 ignorierten Tests sind DSN-gebunden (4 Scratch-Tests über
`REASONER_SCRATCH_DSN`, 1 Echtdaten-Test über `DEADLOCK_CENTRAL_DSN`). Ihre
Logik ist unten je Mangel gegen den Diff verifiziert; der Fixer meldet mit DSN
12 passed. Ohne beschaffbare Test-DSN in dieser Session nicht selbst gefahren.

| Mangel | behoben | Begründung (Datei:Zeile im Stand `fc74b71`) |
|---|---|---|
| 2 | ja | `data.rs:713` baut die Namensmenge aus `entities.canonical_name` UNION `entity_aliases.alias` mit `alias_kind IN (canonical, snapshot_name, class_name_short)`, Schlüssel `primary_external_id`, bindet `lower(...) = ANY($1)` für Event und Enrichment, Fallback auf `hero_name` bei leerer Menge. Test `patch_events_resolve_allowed_aliases_...` erwartet `{1,2,3,4,7}` und schließt Alias-Kind `external_id` (id 5) sowie einen Fremdhelden-Alias (id 6) aus. Deckt den Befund und die negativen Fälle. |
| 3 | ja | `data.rs:821` liest ausschließlich `tierlist.hero_build_sources`; fehlende Tabelle oder Spalte ergibt einen DB-Fehler statt leerer Rückgabe. Pseudo-Spalten `publish_ts`/`last_updated_ts` und der Integer 0 sind raus, Sortierung `COALESCE(last_updated_at, published_at) DESC NULLS LAST, version DESC NULLS LAST`, Zeitstempel als Epoch-Bigint. Test `author_builds_use_real_timestamps_...` prüft Ordnung `[3,2,1,4]`, Epoch-Werte, NULL-Verhalten und den Fehler nach `DROP COLUMN`. |
| 4 | ja | `data.rs:864` löst die Spiel-ID über `primary_external_id` auf die interne Entity auf, für `entity_id` und die `hero_stat_profiles`-Join-Kette. Spalten und Typen sind im Anhang der Fixrunde über `information_schema` am echten DSN belegt. Test `hero_stats_resolve_external_id_...` trennt korrekt Profil 175035 von der Fremd-Entity 25. |
| 5 | ja, dokumentarisch | Kein Code-Change (`ai_roles.rs:77` unverändert). Der laut Fix-Briefing zulässige Doku-Nachweis (Fireworks, `none` für DeepSeek V4) trägt die Entscheidung. Der Live-404 „Model not found" betrifft die Modellverfügbarkeit, nicht Paket A, und bleibt offener Betriebspunkt für den Orchestrator. |
| 6 | ja (Weitergabe) | In A nicht lösbar, korrekt an C delegiert. Kein Handlungsbedarf in A. |
| 7 | ja | `data.rs:448` sortiert `(lower(canonical_name)=lower($2)) DESC NULLS LAST, (lower(payload->>'name')=lower($2)) DESC NULLS LAST, fetched_at DESC, id DESC`. Exakttreffer schlägt jeden neueren Fuzzy-Treffer. Test `snapshot_prefers_canonical_exact_match_...` belegt beide Fälle. |

Secret-Prüfung (Auftragspunkt): Im committeten Code und in den Tests wird keine
DSN und kein Secret ausgegeben oder geloggt. Zugriff nur über Env-Variablen;
die `.expect(...)`-Meldung nennt den Variablennamen, nicht den Wert. Kein
DSN-Literal im Diff. Der Vorfall war ein einmaliger `psql`-Tool-Kanal außerhalb
des Codes, nicht persistiert. Rotation der DB-Zugangsdaten bleibt empfohlen
(Nutzer/Orchestrator), ist aber kein Nachbesserungsgrund an A.

Neue Befunde aus dem Fix (kein Blocker):

- Der Fix behandelt `hero_id` jetzt durchgängig als Spiel-ID
  (`primary_external_id`) und gleicht `hero_name`, `load_patch_events` und
  `load_hero_stat_values` an `load_meta_rows`, `load_synergies`,
  `load_author_builds` und `snapshot` an. Konsistente Korrektur, kein Defekt.
  Hinweis an C: alle Loader erwarten die Spiel-ID.
- `load_author_builds` gibt bei fehlender oder unsichtbarer `tierlist`-Tabelle
  jetzt einen Fehler statt einer leeren Liste zurück (bewusste Fail-loud-
  Entscheidung der Fixrunde). C muss diesen Fehler im Backtest abfangen oder
  die Sichtbarkeit von `tierlist` am Prod-Pool voraussetzen. Der Echtdaten-Test
  lädt 3 Autoren-Builds, am echten Pool ist die Tabelle also sichtbar.

## Urteil Runde 2

FREIGABE. Mängel 2, 3, 4 und 7 sind code-seitig behoben und durch
Regressionstests gedeckt, Mangel 5 per zulässigem Doku-Nachweis, Mangel 6
korrekt an C weitergegeben. Keine Signatur-Nebenwirkung auf `types.rs`. Offene
Punkte (Modellverfügbarkeit 404, DB-Zugangsdaten-Rotation, `tierlist`-
Sichtbarkeit am Prod-Pool) sind Betriebs- und Orchestrator-Sache, kein
Nachbesserungsgrund an Paket A.

## Fixrunde 2 (Merge-Kritiker)

Abgeschlossen am 12.09.2026 im Worktree
`/home/nathanael/.worktrees/deadlock-brain-a`, Branch `feat/build-reasoner-a`.
Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
Code-Commit: `a4d1375`, neu auf `fc74b71`, kein Amend. Ein Thread, keine
Unter-Threads oder Unter-Agenten. Nur der eigene Feature-Branch wurde gepusht.
Dieser Anhang ergänzt die bestehende zentrale Review-Akte; er ist nicht Teil
des Code-Commits und wird hier nicht auf `main` committet.
Die folgenden Zeilen beziehen sich auf `rust/crates/dbrain-reasoner/src/`.

| Fund | Datei:Zeile | Änderung in `a4d1375` | Regressionstest |
|---|---|---|---|
| BLOCKING: Verdict im Realpfad ungeprüft | `ai_roles.rs:240`, `ai_roles.rs:109` | `call` erhält den Rollenparser als Funktionsparameter. `run_critic` übergibt `parse_critic_response`; nur `pass` und `recompose` sind gültig, andere Urteile ergeben `ReasonerError::Ai`. Alle Rollen verwenden damit denselben Parser im direkten Parse und im HTTP-Pfad. | `ai_roles.rs:256`: echter `AiClient` gegen lokalen HTTP-Testserver. `pass` und `recompose` samt Issues werden akzeptiert; `maybe` muss einen KI-Parserfehler liefern. 1 neuer Test: vor Fix rot, nach Fix grün. |
| NIT: fehlende Zahlen werden stumm zu Null | `data.rs:26`, `data.rs:56`, `data.rs:72`, `data.rs:232`, `data.rs:349`, `data.rs:359` | `number` und `card_number` liefern `Option<f64>`. Fehlende, unparsbare und nicht endliche Werte bleiben `None`; echte Nullen bleiben `Some(0.0)`. Property-Maps überspringen ungültige Zahlen, optionales Spirit-Scaling bleibt unbekannt, Gesundheits- und Waffen-Aliase können auf gültige Ersatzwerte zurückfallen. | `data.rs:899`, `data.rs:920`, `data.rs:935`: 3 neue Tests für Property-Maps, optionales Scaling und Snapshot-Ersatzwerte. Alle 3 vor Fix rot, nach Fix grün; echte Nullen bleiben jeweils erhalten. |

Entscheidung zum Nit: `Option` statt Warnzustand je Feld vermeidet zusätzliche
Dependencies und prozessweite Log-Entprellung. Aufrufer mit optionalen Werten,
Property-Maps oder Ersatzquellen erhalten die Unterscheidung. An bestehenden
Pflichtfeldern vom Typ `f64`/`i64` bleibt der bisherige Null-Default ausdrücklich
am jeweiligen Aufrufer erhalten, nachdem vorhandene Ersatzquellen geprüft sind.
So bleiben die vereinbarten öffentlichen Typen unverändert.

Testnachweis im Worktree, jeweils unter `rust/` mit
`PATH=/home/nathanael/.cargo/bin:$PATH`:

| Lauf | Bestanden | Fehlgeschlagen | Ignoriert |
|---|---:|---:|---:|
| Baseline `fc74b71`, `cargo test -p dbrain-reasoner` | 7 | 0 | 5 |
| Rot-Gegenprobe: nur vier neue Tests, Produktionscode unverändert | 7 | 4 | 5 |
| Endstand `a4d1375`, `cargo test -p dbrain-reasoner` | 11 | 0 | 5 |

Die vier roten Tests scheiterten an den erwarteten Verhaltensassertionen;
kein Compilerfehler. Doc-Tests: 0. Die fünf unverändert ignorierten Tests
benötigen Postgres-DSNs und wurden in dieser Fixrunde nicht ausgeführt.
`cargo clippy -p dbrain-reasoner --all-targets -- -D warnings`: Exit 0.
Formatter nur auf den beiden eigenen Rust-Dateien:
`rustfmt --edition 2021 --check crates/dbrain-reasoner/src/ai_roles.rs crates/dbrain-reasoner/src/data.rs`:
Exit 0. `git diff --check`: Exit 0.

`types.rs`, `lib.rs`, andere Crates und Dependencies bleiben unverändert;
keine Code-Kommentare ergänzt. Der ungültige Kritiker liefert jetzt den
vertraglichen KI-Fehler für den aufrufenden Build-Fallback. Die übergeordnete
Build-Orchestrierung gehört nicht zu diesem Paket und wurde nicht verändert
oder als End-to-End-Fallback getestet.
