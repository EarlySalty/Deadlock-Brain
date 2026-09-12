# Review-C (Runde 1): build-reasoner, Paket C (Zusammensetzung und Belege)

status: NACHBESSERN (2026-09-12)

Geprueft: Branch `feat/build-reasoner-c` im Worktree
`/home/nathanael/.worktrees/deadlock-brain-c`, Diff `3bef602..HEAD` (Commit
1e23609). Lesend, kein Code geaendert, kein Branch angefasst. Vergleichsbasis:
ARCHITEKTUR.md (5, 8, 9, 10, 11, 13), MECHANIK.md (7, 13, 14, 16, 17),
BRIEFING-C.md, FERTIG-C.md, types.rs (Paket A), publish_original.rs im
Steam-Bot, tasks.rs/builds.rs im Steam-Bot.

## Testnachweis

Die fuenf C-Module temporaer in `lib.rs` deklariert (`mod` plus `pub use`,
nicht committet), Laeufe im Verzeichnis `rust/` mit
`export PATH=/home/nathanael/.cargo/bin:$PATH`:

```
cargo fmt -p dbrain-reasoner -- --check                                  -> exit 0
cargo clippy -p dbrain-reasoner -p dbrain-builds --all-targets -- -D warnings -> exit 0
cargo test -p dbrain-reasoner -p dbrain-builds                           -> exit 0
```

TESTNACHWEIS[TW-1]: dbrain-reasoner 15 passed, 1 ignored; dbrain-builds 7
passed, 5 ignored | Baseline (Paket A, `3bef602`): Reasoner 7 passed, 1
ignored, 0 rot

Die temporaere `lib.rs`-Aenderung wurde danach mit `git checkout` zurueckgenommen;
der Worktree steht wieder auf `1e23609`, `lib.rs` ist unveraendert. Die Zahlen
decken sich mit FERTIG-C.md. Der eine ignorierte Reasoner-Test und die fuenf
ignorierten dbrain-builds-Tests brauchen Scratch-Postgres.

DB-Echtlauf nicht moeglich: `DEADLOCK_CENTRAL_DSN` und `INFISICAL_TOKEN` sind in
dieser Session nicht gesetzt (gleiche Lage wie Review-A). Die vom Briefing
verlangte Pruefung an den echten Warden-Events (30.06.2026) und den
Spiritual-Overflow-Events (28.07.2026) konnte deshalb nicht gegen die Datenbank
laufen. Bewertung des Patch-Deltas daher statisch am Code plus Unit-Tests, die
den Fall auf Warden-Stil-Patchzeilen nachbilden (`patch.rs:377`). Das ist
Mangel 7.

## Spec-Treue der Signaturen

Gegen types.rs (Paket A) und ARCHITEKTUR Abschnitt 8 gepru:eft, keine
Typabweichung:

- `compute_patch_delta(&HeroModel, &[Value]) -> Vec<PatchDelta>` (patch.rs:212):
  passt.
- `build_meta_index(&[MetaRow], &[AuthorBuild], &[Value], &ReasonerConfig) ->
  MetaIndex` (meta.rs:50): passt. Zusatz `build_meta_index_with_seed_path` und
  `load_seed_builds` sind zulaessige Ergaenzungen fuer den Seed-Pfad.
- `compose_build(&HeroModel, &[ScoredItem], &[PatchDelta], &ReasonerConfig) ->
  BuildObject` (composer.rs:165): passt. `compose_build_with_blocklist`
  (composer.rs:174) traegt die eine Recompose-Runde, zulaessige Ergaenzung.
- `backtest_metrics(&BuildObject, &AuthorBuild) -> BacktestMetrics`
  (backtest.rs:35), `backtest_hero` async (backtest.rs:152): passt.
- `to_publish_payload(&BuildObject) -> BuildSpecPayload` (publish.rs:37): passt.

Die vier Entscheidungen aus FERTIG-C.md sind inhaltlich vertretbar: kein Raten
ohne numerische Item-ID (Entscheidung 1, korrekt umgesetzt, patch.rs:96 und
`filter_map` auf `target`), Jaccard separat als `core_jaccard` weil
`BacktestMetrics` kein Feld hat (Entscheidung 2), Seed-Loader mit explizitem
Pfad und Item-Modellen (Entscheidung 3), Publish nur ueber `steam.steam_tasks`
ohne Steam-Bot-Aenderung (Entscheidung 4).

## Maengel

### 1. apply_patch_delta: Waffen-Buff erreicht den Damage-Plan nie (blockierend)

`patch.rs:317`:

```
hero.weapon.sustained_dps = if hero.weapon.sustained_dps > 0.0 {
    hero.weapon.sustained_dps
} else {
    hero.weapon.bullet_damage * hero.weapon.shots_per_second
};
hero.damage_plan.weapon_dps = hero.weapon.sustained_dps;
```

Die Bedingung ist verkehrt herum. Paket B liefert fuer jeden echten Helden ein
`sustained_dps > 0`, also greift immer der obere Zweig und behaelt den alten
Wert. Die zuvor auf das Waffenprofil angewandten Deltas (`bullet_damage`,
`fire_rate`, `reload`, patch.rs:280) fliessen damit nicht in `weapon_dps`,
`weapon_share` und `primary_axis` zurueck. Der Damage-Plan bleibt stehen.

Folge: genau der Pflichtfall faellt aus. MECHANIK.md Abschnitt 13 und 17 und
ARCHITEKTUR Abschnitt 13 verlangen, dass ein Waffen-Buff aus dem Nightshift-Patch
den Waffen-DPS im Damage-Plan hebt, damit Wardens Waffen-Kern ueber die Schwelle
steigt und der Patch-Wechsel-Backtest den Sprung von "tot" auf "Meta" zeigt. Mit
dem invertierten Guard bewegt ein Helden-Waffen-Buff den Score nicht. Nur
Item-gezielte Deltas wirken (ueber `patched_score`, composer.rs:78), Helden- und
Ability-Buffs versanden.

Vorschlag: `sustained_dps` nach dem Anwenden der Deltas immer aus dem
aktualisierten Waffenprofil neu rechnen (die Formel aus MECHANIK Abschnitt 8,
`weapon_dps`), nicht nur wenn es vorher 0 war. Danach `weapon_dps`, den Anteil
und `primary_axis` ableiten wie schon vorhanden.

### 2. Backtest: nicht messbare Reihenfolge wird als 1,0 gemeldet (wichtig)

`backtest.rs:49`: wenn keine gemeinsamen Items existieren oder eine der beiden
Seiten weniger als zwei Items hat, gibt `order_proximity` den Wert `1.0` zurueck.
`1.0` heisst laut ARCHITEKTUR Abschnitt 11 "maximal verdreht", also der
schlechteste messbare Fall. Briefing-C Punkt 5 verlangt aber ausdruecklich
ehrlich "nicht messbar" statt einer Zahl, wenn Reihenfolge oder Vergleichsbasis
fehlt. Ein fehlender Vergleich wird hier als schlechtestes Ergebnis ausgegeben,
und der Aggregatwert (backtest.rs:130) mittelt diese `1.0` mit ein und
verschlechtert den Held-Schnitt. Dasselbe beim Leer-Autoren-Fall
(backtest.rs:123): `order_proximity = 1.0`, `core_coverage = 0.0` sehen aus wie
gemessen.

Ursache liegt auch am Typ aus Paket A: `BacktestMetrics.order_proximity` ist
`f64` ohne `Option`, kann "nicht messbar" also nicht tragen. Vorschlag fuer diese
Runde: nicht messbare Autoren aus dem Aggregat ausschliessen statt sie als `1.0`
zu mitteln, und den Zustand im Report-Detail sichtbar machen. Die saubere Loesung
(Feld optional oder ein Messbar-Flag) ist eine Abstimmung mit Paket D, weil sie
`types.rs` beruehrt. In REVIEW-C fuer D vermerkt.

### 3. Composer setzt sell_priority nie (wichtig)

`composer.rs:137`: `build_item` setzt `sell_priority: None` fest. MECHANIK.md
Abschnitt 7 verlangt, dass billige T1-Items, die spaeter verkauft werden, im
Build eine `sell_priority` bekommen. Der Composer ist die einzige Stelle, die das
setzen kann. Damit ist die durchgereichte Kette bis in den Payload
(`publish.rs:33`, Steam-Bot Proto-Feld 4) fuer `sell_priority` tot: das Feld
kommt nie mit einem Wert an. `imbue` dagegen wird korrekt gesetzt
(composer.rs:118).

Zusaetzlich bleibt `ability_order` im BuildObject immer leer
(composer.rs:253); ein veroeffentlichtes Build traegt damit keine Skill-Order.
Ob die Skill-Order in C oder erst in der Fassade (Paket D) gefuellt wird, ist
nicht festgelegt. Vorschlag: `sell_priority` im Composer aus Kosten/Tier und der
Kaufphase ableiten (MECHANIK 7), und mit D klaeren, wer `ability_order` fuellt.

### 4. Meta-Verbreitung kann ueber 1,0 laufen (nit)

`meta.rs:72`: `prevalence_raw` ist die Summe ueber alle gruppierten Zeilen eines
Items, `max_prevalence` (meta.rs:64) ist das Maximum einer einzelnen Zeile. Die
Normierung `prevalence_raw / max_prevalence` (meta.rs:83) kann darum groesser als
1,0 werden, sobald ein Item nach dem Patch-Filter mehr als eine Zeile hat (etwa
mehrere Brackets). Heute geringe Wirkung, weil je Patch meist eine Zeile bleibt,
aber inkonsistent. Vorschlag: Summe gegen Summe oder Maximum gegen Maximum
normieren.

### 5. Situationsblock-Heuristik trifft den Referenzbuild nur teilweise (nit)

`composer.rs:40`: `is_can_buy_one` verlangt `item.is_active`. Passive
Resist-Items (etwa Spirit Resilience oder Bullet Resilience, falls in den Daten
passiv) fallen damit nicht in "Can buy 1", sondern in den Kern. `is_can_buy_one`
wird vor `is_shield` geprueft (composer.rs:189), also landet ein aktives Item mit
Defense (etwa Reactive Barrier) in "Can buy 1" statt in "Shields", obwohl der
Referenzbuild es unter "Shields" fuehrt. Die Namenslisten sind plausibel, aber
die Zuordnung braucht einen Abgleich an echten Item-Daten, sobald Paket B echte
`ScoredItem` liefert. Vorschlag: gegen den Warden-Referenzbuild (MECHANIK 17)
verifizieren und Reihenfolge der Klassifikatoren pruefen.

### 6. Item-Analyst-Filter aus Review-A Mangel 6 ist in C nicht umgesetzt (nit, Koordination)

Review-A Mangel 6 gab an Paket C weiter, dass ein Warum-Text des Item-Analysten
verworfen werden muss, wenn er ein Item nennt, das nicht im Build ist. In den
C-Dateien gibt es keinen KI-Zusammenfuehrungs-Code: `compose_build` setzt
`why: String::new()` (composer.rs:137), die Verdrahtung mit `ai_roles`
liegt in der Fassade (`lib.rs`, Paket A/D). Der Filter gehoert also in die
Fassade und ist dort noch offen. Kein Handlungsbedarf in den C-Dateien, aber als
Weitergabe an D festgehalten, damit die Auflage nicht verloren geht.

### 7. Echt-Event-Pruefung nicht gefahren (nit, Umgebung)

Ohne DSN und Infisical-Token in dieser Session (siehe Testnachweis). Das
Patch-Delta ist statisch und ueber die Unit-Tests geprueft (Vorzeichen aus
`change_type` und Wert, Groesse aus `old_value`/`new_value` oder aus
`numeric_tokens` der Patchzeile, patch.rs:183; kein Raten ohne Item-ID). Der
echte Lauf gegen die Warden- und Spiritual-Overflow-Events steht aus und gehoert
in den Integrationslauf von Paket D.

## Zur Kenntnis (keine Maengel)

- Hygiene sauber: keine Secrets, keine Code-Kommentare, keine neuen externen
  Crates, keine Modellnamen. Der Seed-Loader liest `referenz/*.json` nur lesend
  mit sortierter Reihenfolge (meta.rs:175), nichts wird in `tierlist`
  geschrieben.
- spec.rs-Erweiterung ist rueckwaertskompatibel: `description`, `width`,
  `height` an `BuildSpecCategory` und `imbue`, `sell_priority` an `BuildSpecMod`,
  alle mit `skip_serializing_if = "Option::is_none"`; der bestehende
  `assemble_payload` setzt sie auf `None` (spec.rs:327, 337). Keine
  compile-gepruefte Query dazugekommen, kein sqlx-Cache-Eintrag noetig.
- Publish-Payload trifft den Handler: `publish_original.rs` liest `imbue` (Alias),
  `sell_priority`, pro Kategorie `description`/`width`/`height`/`optional` und
  `ability_order` (`ability_id`/`currency_type`/`delta`) genau so, wie
  `to_publish_payload` sie erzeugt. `width` 780 und `height` 260 liegen ueber den
  Client-Mindestwerten (ARCHITEKTUR Abschnitt 10).
- Enqueue spiegelt den bestehenden Produktivweg: `enqueue_publish_task`
  (publish.rs:85) benutzt dasselbe Muster wie
  `steam-persistence::builds::insert_task_with_payload`
  (`INSERT INTO steam.steam_tasks(type, payload, status) VALUES(..., 'PENDING')`,
  builds.rs:598). Das fehlende `bot_account_id` ist damit die etablierte
  Konvention fuer BUILD-Tasks, kein Fehler. Laufzeitgepruefte Query, kein Cache.
- Kein Schreibpfad in den reinen Funktionen: `to_publish_payload` und
  `publish_task_payload` sind seiteneffektfrei; der einzige Insert steckt in
  `enqueue_publish_task`. Bei `--no-publish` bleibt er unerreicht, solange D ihn
  nur bei `--publish` aufruft (Auflage an D).
- Meta-Daempfung korrekt: unter Mindeststichprobe Faktor 0,5 statt Nullung
  (meta.rs:82), Autoren- und Claim-Treffer je Build/Claim genau einmal gezaehlt
  (meta.rs:41, 30).
- `backtest_hero` (backtest.rs:152) liefert bei vorhandenen Autoren ehrlich einen
  Fehler, dass der ScoredItem-Bestand aus Paket B fehlt, statt eine Scheinzahl.
  `detect_patch_switch` (backtest.rs:80) ist gebaut, aber noch nirgends im Report
  verdrahtet; `switch_detected` bleibt `None` (deckt sich mit FERTIG-C.md).

## Urteil

NACHBESSERN.

- Blocker: Mangel 1 (apply_patch_delta hebt den Waffen-DPS nicht, der Warden-
  Pflichtfall faellt aus).
- Wichtig, in dieser Runde zu fixen: Mangel 2 (nicht messbare Reihenfolge als
  1,0), Mangel 3 (sell_priority wird nie gesetzt).
- Nits und Koordination: Mangel 4, 5, 6, 7.

Fixes gehen an ein guenstiges Modell; Mangel 2 (Typ) und Mangel 3
(ability_order) brauchen eine kurze Abstimmung mit Paket D.

## Was Paket D bei der Integration wissen muss

- Die Orchestrierungs-Fassade (`reason_build`, `reason_patch_impact`,
  `reason_backtest`) und die `mod`-Zeilen in `lib.rs` fehlen noch und sind D.
  Beim Verdrahten `apply_patch_delta` (nach Fix) vor dem Scoring aufrufen, sonst
  wirkt kein Helden- oder Ability-Patch-Delta.
- Der Item-Analyst-Filter (Mangel 6) gehoert in die Fassade: KI-Warum-Texte, die
  ein Item nennen, das nicht im Build ist, verwerfen.
- `order_proximity` kann "nicht messbar" nicht ausdruecken (Mangel 2). Entweder
  `BacktestMetrics` um ein Messbar-Flag erweitern (`types.rs`, Paket A) oder im
  Report-Detail fuehren; bis dahin unmessbare Autoren nicht ins Aggregat mitteln.
- Ein echter Reasoner-vs-Seed-Backtest braucht den ScoredItem-Bestand aus Paket B
  und den reaktivierten Autoren-Scan (Paket S); bis dahin ist `backtest_hero` bei
  vorhandenen Autoren ein bewusster Fehler, nicht ein gruener Lauf.
- `sell_priority` und `ability_order` im Build fuellen (Mangel 3), sonst kommen
  beide Felder leer im Payload an.
- Bei `--publish` `enqueue_publish_task` aufrufen, bei `--no-publish` nicht; der
  Insert ist der einzige Schreibpfad.
- Real-Event-Pruefung (Warden 30.06.2026, Spiritual Overflow 28.07.2026) im
  Integrationslauf mit echtem DSN nachholen (Mangel 7).

## Fixrunde 1

Fertigmeldung vom 12.09.2026, Paket C. Branch `feat/build-reasoner-c`,
Worktree `/home/nathanael/.worktrees/deadlock-brain-c`,
Intent-Thread `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
Ein Thread, keine Unter-Agenten. Code-Commit
`631182abd35294d7aef6074ceb3913461d4f5eaf`, auf
`origin/feat/build-reasoner-c` gepusht. Kein Merge oder Push nach main.
Die Angaben unten ersetzen die offenen Hinweise zu Mängeln 1 bis 5 aus Runde 1;
der ursprüngliche Reviewtext bleibt als Verlauf erhalten.

### Mängelabgleich gegen den eigenen Diff

Alle Codepfade unten liegen in `rust/crates/dbrain-reasoner/src/` und gehören
zum Commit `631182abd35294d7aef6074ceb3913461d4f5eaf`.

1. **Waffen-Buff:** `patch.rs:317` berechnet den nachhaltigen Waffen-DPS nach
   Anwendung der Deltas aus Magazininhalt, Feuerrate und Nachladedauer neu.
   Der alte positive Cachewert blockiert die Neuberechnung nicht mehr.
   `weapon_dps`, `weapon_share` und `primary_axis` erhalten den neuen Wert.
   Regression `patch.rs:405` prüft Bullet Damage, Fire Rate und Reload einzeln
   an einem Warden-ähnlichen Modell mit bereits positivem `sustained_dps`.
2. **Nicht messbare Reihenfolge:** `types.rs:342` ändert ausschließlich
   `BacktestMetrics.order_proximity` von `f64` auf `Option<f64>`.
   `backtest.rs:75` liefert bei fehlender Vergleichbarkeit `None`,
   `backtest.rs:162` mittelt ausschließlich `Some`; keine Autoren oder nur
   unmessbare Autoren ergeben auch im Aggregat `None`. Die neue
   `Display`-Implementierung für `BacktestReport` in `backtest.rs:8` zeigt
   je Autor und im Heldenschnitt „nicht messbar“. JSON trägt `null`.
   Regressionen: `backtest.rs:273` und `backtest.rs:306`.
3. **Verkauf und Skill-Order:** `composer.rs:137` setzt für Lane-Items mit
   Tier 1 Priorität 1, für andere Lane-Items Priorität 2. Items der Phasen
   Core und Late erhalten keine Verkaufspriorität, auch Core-T1 nicht.
   `meta.rs:27` wählt die Skill-Order des höchstgewichteten gültigen
   Autoren-Builds des angefragten Helden aus `details.abilityOrder`, mit
   Unterstützung der Snake-Case-Form. Reihenfolge, Currency und Delta bleiben
   erhalten. Ungültige oder unvollständige Orders werden vollständig
   verworfen, fremde Helden und nicht endliche Gewichte ausgeschlossen.
   Danach folgt die bereitgestellte Order aus `brain.hero_ability_orders`,
   sonst eine leere Order mit dem Beleg „Skill-Order: keine Quelle“.
   `composer.rs:176` übernimmt Order und Quellenbeleg in das BuildObject.
   Regressionen `composer.rs:361` und `composer.rs:450` prüfen den Weg über
   BuildObject und JSON bis in den bestehenden Publish-Payload samt
   `sell_priority`, `ability_order` und Quellenbeschreibung. `publish.rs`
   benötigte dafür keine Änderung.
4. **Verbreitung:** `meta.rs:179` klemmt den normierten Wert vor der
   Stichprobendämpfung auf höchstens 1,0. Zwei gleich große Zeilen für ein
   Item ergeben jetzt 1,0 statt 2,0; unter Mindeststichprobe 0,5 statt 1,0.
   Regression: `meta.rs:365`.
5. **Warden-Blöcke:** `composer.rs:206` prüft Shields vor „Can buy 1“.
   Die Namenszuordnung für Resilience und Counterspell setzt keinen aktiven
   Item-Status voraus. Allgemeine Defense- und Spirit-Eigenschaften ziehen
   Kernitems wie Veil Walker und Blood Tribute nicht mehr aus dem Kern.
   Die Optional-Liste enthält auch die im Seed dort geführten Counter-Items.
   Regression `composer.rs:384` lädt den echten Seed per `include_str!`:
   sämtliche 40 Items stehen in den fünf erwarteten Blöcken und in der
   Seed-Reihenfolge: Core 19, Can buy 1 6, Tryhard 1, Shields 3, Optional 11.
   Die ScoredItem-Werte sind Fixtures, keine Ausgabe von Paket B oder ein
   Datenbanknachweis. Reactive Barrier wird zusätzlich als aktiv modelliert,
   um den im Review genannten Klassifikator-Konflikt abzudecken.

### Schnittstellenhinweise für D

**Verbindliche Typänderung:** `BacktestMetrics.order_proximity: Option<f64>`.
Das ist die einzige Änderung in `types.rs`. D muss Zahl oder `null` beim
Speichern behandeln und darf fehlende Messwerte nicht als 1,0 einsetzen.
Für Textausgabe ist `BacktestReport::to_string()` verfügbar. Eine nullable
SQL-Spalte beziehungsweise eine entsprechende Anpassung des D-Speicherpfads
ist bei der Integration erforderlich; der ursprüngliche Architekturentwurf
hatte noch `order_proximity double precision NOT NULL`.

**Additiver Quellenadapter für die vorhandene Schnittstellenlücke:** Der
unveränderte A-Typ `MetaIndex` enthält nur Item-Signale, `AuthorBuild` enthält
keine Rohdetails, Helden-ID oder Gewichte. Die bisherige Composer-Signatur
nimmt keine Quellen entgegen. Deshalb ergänzt C ausschließlich in `meta.rs`
`AuthorBuildSource` und `MetaIndexWithSources` statt weitere A-Typen zu ändern.
Der Wrapper hält den bisherigen Index, gewichtete Autoren-Details und eine
nach Helden-ID indizierte Fallback-Order als `Vec<AbilityStep>`.

D muss die gelesenen Autoren-Details mit Helden-ID und Gewicht sowie die
Order aus `brain.hero_ability_orders` in diesen Wrapper einspeisen und
`compose_build_with_sources(hero, scored, deltas, cfg, blocked, meta)`
verwenden. Die Übersetzung der DB-Order in `AbilityStep` erfolgt an dieser
Integrationsgrenze; C hat keinen zusätzlichen Datenbank-Lader gebaut.
Die bisherigen Composer-Einstiege bleiben kompatibel und melden ohne
bereitgestellte Quelle ehrlich „keine Quelle“. D soll den Quellenbeleg bei
späterer KI-Anreicherung der Rationale erhalten. Dies ist eine additive
Anpassung gegenüber dem wörtlichen „aus MetaIndex“ im Fix-Briefing; die
Beschränkung auf genau eine Feldänderung in `types.rs` bleibt eingehalten.

### Testnachweis mit Baseline und Rot-Gegenproben

Baseline auf `1e23609`, C-Module temporär über `mod` und `pub use` in `lib.rs`
eingebunden: Reasoner **15 bestanden, 0 fehlgeschlagen, 1 ignoriert**;
dbrain-builds **7 bestanden, 0 fehlgeschlagen, 5 ignoriert**.

Fünf neue Regressionen vor den Produktionsfixes: Reasoner **15 bestanden,
5 fehlgeschlagen, 1 ignoriert**. Rot waren Waffen-DPS, unmessbare Reihenfolge,
Verkaufsprioritäten, Verbreitung und Warden-Blockzuordnung.
Zusätzliche Skill-Order-Gegenprobe gegen den noch leeren Quellenpfad:
**0 bestanden, 1 fehlgeschlagen, 22 ausgefiltert**, `ability_id` war `null`
statt 101. Zusätzliche Report-Gegenprobe mit temporärer numerischer
Ersatzanzeige für `None`: **0 bestanden, 1 fehlgeschlagen, 22 ausgefiltert**.
Beide Gegenproben wurden vor dem Abschlusslauf zurückgenommen.

Endstand mit eingebundenen C-Modulen: Reasoner **22 bestanden,
0 fehlgeschlagen, 1 ignoriert**; dbrain-builds **7 bestanden,
0 fehlgeschlagen, 5 ignoriert**. Alle sieben neuen Regressionen sind grün.
Die ignorierten Tests benötigen PostgreSQL. Mängel 6 und 7 sowie der echte
Event- und End-to-End-Lauf bleiben wie beauftragt bei D.

Warden-Modell: Bullet Damage 10, Fire Rate 2, Magazin 20, Reload 2,
Spirit-DPS 12. Ausgangs-DPS 16,6667, Waffenanteil 0,5814, Achse Hybrid.
Mit altem Guard blieb der DPS trotz Buff bei 16,6667, der Regressionstest
scheiterte bereits am ersten Bullet-Damage-Fall. Nach dem Fix:

| Einzelner Buff | Waffen-DPS | Waffenanteil | Achse |
|---|---:|---:|---|
| Bullet Damage +2 | 20,0000 | 0,6250 | Weapon |
| Fire Rate +1 | 23,0769 | 0,6579 | Weapon |
| Reload -1 | 18,1818 | 0,6024 | Weapon |

Der gemischte Reihenfolge-Test mittelt 0,0 und 1,0 zu 0,5 und schließt den
unmessbaren dritten Autor aus. Die vorhandene Metrik-Fixture bleibt bei
Kern-Überdeckung 0,6667, Jaccard 0,5 und Reihenfolge `Some(0,0)`.
Das sind Formel- und Fixture-Nachweise, kein echter Warden-Meta-Backtest.

Im Verzeichnis `rust/`, mit `PATH=/home/nathanael/.cargo/bin:$PATH`:

```text
cargo fmt -p dbrain-reasoner -- --config skip_children=true crates/dbrain-reasoner/src/patch.rs crates/dbrain-reasoner/src/meta.rs crates/dbrain-reasoner/src/composer.rs crates/dbrain-reasoner/src/backtest.rs crates/dbrain-reasoner/src/types.rs
cargo fmt -p dbrain-reasoner -- --check --config skip_children=true crates/dbrain-reasoner/src/patch.rs crates/dbrain-reasoner/src/meta.rs crates/dbrain-reasoner/src/composer.rs crates/dbrain-reasoner/src/backtest.rs crates/dbrain-reasoner/src/types.rs
cargo clippy -p dbrain-reasoner -p dbrain-builds --all-targets -- -D warnings
cargo test -p dbrain-reasoner -p dbrain-builds
```

Alle Abschlussbefehle erfolgreich, Clippy ohne Warnungen.
Die explizite Dateiliste beschränkt den Formatter auf die eigenen Dateien
und die nur temporär eingebundene `lib.rs`. `lib.rs` wurde anschließend
bytegenau auf den Ausgangsstand zurückgesetzt und ist nicht committet.
`git diff --check` ist sauber. Kein Release-Build, keine Code-Kommentare,
keine Änderung an fremden Implementierungsdateien oder am Steam-Bot.

## Review Runde 2

status: FREIGABE (2026-09-12)

Geprueft: Worktree `/home/nathanael/.worktrees/deadlock-brain-c`, Code-Commit
`631182a` (Fixrunde 1), Basis `1e23609`, Diff `git diff 1e23609..631182a`. Der
Worktree steht auf `04305ee` (Doku-Commit ueber dem Fix). Lesend, kein Code
geaendert, kein Branch angefasst. Nur die Maengel 1 bis 5 gegen die Maengelliste,
Maengel 6 und 7 liegen bei D. Vergleichsbasis: die Fixrunde-1-Angaben oben,
`referenz/lightbringer-warden.json`, MECHANIK-Abschnitte 7, 8, 13, 17.

### Maengelabgleich

1. **Waffen-Buff (Blocker): behoben.** `patch.rs:317` rechnet `sustained_dps`
   jetzt bedingungslos aus dem gepatchten Waffenprofil neu (Magazininhalt,
   Feuerrate, Nachladedauer, mit reload-beruecksichtigender Formel). Der alte
   positive Cachewert blockiert die Neuberechnung nicht mehr. `weapon_dps`,
   `weapon_share` und `primary_axis` leiten sich davon ab (`patch.rs:326` bis
   `339`). Die Hero-Deltas fuer `bullet_damage`, `fire_rate` und `reload` treffen
   die passenden Waffenfelder (`patch.rs:280` bis `284`); ein reiner Spirit-Buff
   (`spirit_scaling`, `patch.rs:286`) fasst die Waffe nicht an und bleibt ohne
   Waffen-Effekt. Regression `patch.rs:404` prueft die drei Mechaniken einzeln an
   einem Warden-Modell mit bereits positivem `sustained_dps` und assertiert
   `weapon_dps`, `weapon_share` und `primary_axis = Weapon`. Genau der
   Pflichtfall greift jetzt.

2. **Nicht messbare Reihenfolge (wichtig): behoben.** `types.rs:342` aendert
   ausschliesslich `order_proximity` von `f64` auf `Option<f64>`; keine andere
   Signatur ist angefasst (Diff an `types.rs` sind zwei Zeilen). `backtest.rs:77`
   gibt bei fehlender Vergleichbarkeit `None`, sonst `Some(...)`; das Aggregat
   mittelt per `filter_map` nur `Some` und liefert `None`, wenn kein messbarer
   Autor bleibt. Die neue `Display`-Implementierung zeigt „nicht messbar“, JSON
   traegt `null`. Regressionen in `backtest.rs` decken Some/None und den
   Report-Text ab. Die D-Uebernahme steht sauber im Anhang „Schnittstellenhinweise
   fuer D“: nullable SQL-Spalte, fehlende Messwerte nicht als 1,0 einsetzen. D kann
   die Aenderung uebernehmen.

3. **Verkauf und Skill-Order (wichtig): behoben.** `sell_priority`: `build_item`
   (`composer.rs:137`) setzt Lane-Items auf `Some(1)` bei Tier 1, sonst `Some(2)`,
   Core und Late auf `None`. Der Wert kommt bis in den Payload (Test
   `composer.rs:361` prueft `publish_task_payload` mit `sell_priority` 1, 2 und
   fehlend). `ability_order`: der neue Einstieg `compose_build_with_sources`
   (`composer.rs:176`) fuellt Order und Quellenbeleg aus
   `meta.ability_order(hero_id)` (`meta.rs:27`). Quellenreihenfolge wie gefordert:
   hoechstgewichteter gueltiger Autoren-Build des angefragten Helden (Filter auf
   `hero_id` und endliches Gewicht, deterministische Sortierung), dann
   `brain.hero_ability_orders`, sonst leer mit Beleg „Skill-Order: keine Quelle“.
   Unvollstaendige oder fremde Orders werden komplett verworfen
   (`collect::<Option<Vec<_>>>`, `ability_id > 0`). Regression
   `composer.rs:449` prueft den Weg bis in den Publish-Payload.

4. **Verbreitung (nit): behoben.** `meta.rs:179` klemmt den normierten Wert vor
   der Stichprobendaempfung mit `.min(1.0)`. Regression `caps_grouped_prevalence`
   (zwei gleiche Zeilen ergeben 1,0 statt 2,0, unter Mindeststichprobe 0,5).

5. **Warden-Bloecke (nit): behoben.** `is_shield` steht vor `is_can_buy_one`
   (`composer.rs:206`); die Klassifikatoren sind namensbasiert ohne
   `is_active`-Vorbedingung (Spirit- und Bullet Resilience sowie Counterspell in
   „Can buy 1“, Reactive Barrier in „Shields“). Regression
   `assigns_all_warden_seed_items_to_five_reference_blocks` (`composer.rs:383`)
   laedt den echten Seed per `include_str!` und prueft je Block Namen, Reihenfolge
   und Vollzaehligkeit. Referenz nachgezaehlt: 5 Bloecke, Core 19, Can buy 1 6,
   Tryhard 1, Shields 3, Optional 11, Summe 40, kein Anti-Heal-Item, daher bleibt
   der Counters-Block leer und es entstehen vier Situationsbloecke plus Core.

### Neuer Befund aus dem Fix (kein Blocker, Auflage an D)

Der Default-Pfad `compose_build` und `compose_build_with_blocklist` laesst
`ability_order` weiterhin leer; nur `compose_build_with_sources` fuellt sie. D
muss diesen Einstieg mit dem additiven Wrapper `MetaIndexWithSources`
(`meta.rs:20`) nutzen und die Autoren-Details, Gewichte und
`brain.hero_ability_orders` einspeisen, sonst kommt `ability_order` trotz Fix
leer im Payload an. Die neuen Typen `AuthorBuildSource` und `MetaIndexWithSources`
liegen in `meta.rs`, nicht in `types.rs`; das weicht vom woertlichen „aus
MetaIndex“ des Fix-Briefings ab, haelt aber die Beschraenkung auf eine einzige
`types.rs`-Feldaenderung ein und ist im Anhang „Schnittstellenhinweise fuer D“
dokumentiert. Vertretbar. Kein weiterer Regressionsschaden gefunden.

### Testnachweis

C-Module temporaer per `mod` in `lib.rs` deklariert (nicht committet), im
Verzeichnis `rust/` mit `PATH=/home/nathanael/.cargo/bin:$PATH`:

```
cargo test -p dbrain-reasoner -p dbrain-builds   -> exit 0
```

TESTNACHWEIS[TW-1]: dbrain-reasoner 22 passed, 1 ignored; dbrain-builds 7
passed, 5 ignored | Baseline (Fixrunde 1, `631182a`): identische Zahlen, 0 rot.
Die sechs ignorierten Tests brauchen ein Scratch-Postgres (`DEADLOCK_CENTRAL_DSN`
in dieser Session nicht gesetzt, gleiche Lage wie Runde 1). Die temporaere
`lib.rs`-Aenderung wurde per `git checkout` zurueckgenommen; `git status` ist
sauber, der Worktree steht wieder auf `04305ee`.

### Urteil

FREIGABE. Alle fuenf Maengel sind behoben und durch Regressionen abgedeckt, keine
neuen Blocker. Offene Auflagen fuer die Integration (Paket D): den
`compose_build_with_sources`-Pfad verdrahten, die `Option<f64>`-Spalte nullable
fuehren, sowie Maengel 6 und 7 (Item-Analyst-Filter und Echt-Event-Lauf).
