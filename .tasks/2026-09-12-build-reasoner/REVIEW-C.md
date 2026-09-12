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
