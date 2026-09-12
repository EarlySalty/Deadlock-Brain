# Review-B (Runde 1): build-reasoner, Paket B (deterministischer Kern)

status: NACHBESSERN (2026-09-12)

Geprueft: Branch `feat/build-reasoner-b` im Worktree
`/home/nathanael/.worktrees/deadlock-brain-b`, Diff `3bef602..68dc58c`. Lesend,
kein Code geaendert, kein Branch angefasst. `lib.rs` wurde nur temporaer um die
drei `mod`-Zeilen ergaenzt, um Clippy und Tests zu fahren, und danach auf den
Commit-Stand zurueckgesetzt (Arbeitsbaum sauber). Vergleichsbasis: MECHANIK.md
(alle Regeln), ARCHITEKTUR.md (5, 7, 9, 13), BRIEFING-B.md, REVIEW-A.md
(Mangel 1), PAKETE.md, Skill `deadlock-build-bauen`.

Dateien: `rust/crates/dbrain-reasoner/src/mechanics.rs` (477),
`src/hero.rs` (317), `src/item.rs` (310). Keine anderen Dateien im Diff.

## Testnachweis

```
cargo fmt -p dbrain-reasoner -- --check                         -> exit 0
cargo clippy -p dbrain-reasoner --all-targets -- -D warnings    -> exit 0 nur mit pub mod
cargo test -p dbrain-reasoner                                   -> exit 0
```

TESTNACHWEIS[TW-1]: 20 passed, 2 ignored | Baseline: 0 rot

Die zwei ignorierten Tests (Warden-Loader aus A, Warden-Referenz-Scoring aus B)
brauchen `DEADLOCK_CENTRAL_DSN`. In dieser Review-Session war kein
Infisical-Zugang gesetzt (`INFISICAL_API_URL`/Token fehlen,
`scripts/export_infisical_env.py` bricht ab). Der Echtdaten-Lauf war hier also
nicht moeglich, siehe Mangel 1.

Clippy-Hinweis: Mit `lib.rs`-Einbindung nur ueber `mod hero; mod item; mod
mechanics;` schlaegt `clippy -D warnings` mit 33 `dead_code`-Fehlern fehl, weil
Paket C als Aufrufer noch fehlt. Erst `pub mod` bzw. `pub use` der drei Module
macht Clippy gruen. Die "clippy gruen"-Meldung in FERTIG-B haelt nur, wenn die
Delegator-Integration die Module re-exportiert. Weitergabe an D.

## Warden-Referenz-Zahlen

Nicht produzierbar: ohne DSN kein Echtdaten-Lauf. Damit fehlen die vier
Referenzscores (Veil Walker, Mercurial Magnum, Siphon Bullets, Quicksilver
Reload) und die fuenf besten/schlechtesten Items. Das ist zusammen mit Mangel 1
der Grund fuer NACHBESSERN. Der Lauf muss mit DSN nachgeholt werden, bevor Paket
B freigegeben wird.

## Maengel

### 1. Kern-Schwelle wird nicht geprueft, Warden-Pflicht unbelegt (blockierend)

`item.rs:307`: Der Integrationstest `scores_warden_reference_items_from_real_snapshot`
assertiert je Referenz-Item nur `score.score.total > 0.0`. BRIEFING-B Punkt 4 und
Review-Briefing Punkt 3 verlangen, dass die vier Items **ueber der Kern-Schwelle**
liegen. Eine Kern-Schwelle existiert im Code nirgends. `total > 0` ist fast immer
erfuellt und beweist die Pflichtaussage "die vier sind Kern-Items" nicht.
Vorschlag: Schwelle definieren (z.B. Rang unter den Top-N oder Score-Quantil aus
`score_items`) und den Test dagegen fahren, nicht gegen `> 0`.

### 2. scaling_step wird in Produktion nie gefuellt (blockierend fuer Integration)

`hero.rs:8` `build_hero_model(loaded, abilities: &[Value], stats)` fuellt
`scaling_step` nur aus dem Argument `abilities`. Es gibt aber keinen oeffentlichen
Loader, der rohe Ability-Payloads liefert: `data.rs:463 ability_snapshots` ist
privat, `data.rs:291` setzt `scaling_step: None`, und keiner der `pub`-Loader gibt
`Vec<Value>` der Abilities zurueck (`lib.rs` exportiert nur `load_hero_model`,
`load_item_models`, `load_hero_stat_values` usw.). Folge: Die Fassade und Paket C
koennen `build_hero_model` nicht mit Ability-Values speisen, `scaling_step` bleibt
`None`. Damit fehlt die Skalierungsstufe (MECHANIK 12), laut Spec der Angelpunkt
des ganzen Kaufplans. Vorschlag: `ability_snapshots` als `pub load_hero_abilities`
freigeben (Datei gehoert zu A, also A-Bump-up oder Delegator) und die Fassade
`load_hero_model` + `build_hero_model` + `load_hero_abilities` verketten lassen.

### 3. Tickrate gegen Proc-Cooldown nicht verdrahtet (wichtig)

`mechanics.rs:57 proc_stacks` hat einen Unit-Test (0.1/0.7/7.0 -> 10), aber keinen
Aufrufer im Scoring. `property_value` gibt fuer ProcCooldown-Felder pauschal 0.0
zurueck (`mechanics.rs:203`) und multipliziert nie mit Stapelzahl;
`item.proc_cooldown` wird in `score_item` nie gelesen; eine TickRate fehlt im
`ItemModel` ganz. MECHANIK 3 und MECHANIK 17 Punkt 4 machen genau diese Rechnung
zum Kern des Werts von Schuss-Proc-Items (Siphon Bullets, Blood Tribute) auf
Warden. So bleiben diese Items unterbewertet. Vorschlag: Proc-Wert ueber
`proc_stacks` in `combat_window_value` einrechnen; fehlt die TickRate im Modell,
als Bump-up an A/C melden statt die Regel tot zu lassen.

### 4. Helden-abhaengige Bedingungen auf Konstanten verkuerzt (wichtig)

`mechanics.rs:37 condition_factor(item, cfg)` hat kein `hero`-Argument
(eingefrorene Signatur, ARCHITEKTUR 7). Deshalb: `MeleeBound -> 0.5`
(MECHANIK 4: 1.0 wenn der Held zuschlaegt, sonst 0.0), `ShotBound -> 0.75`
(Spec: `weapon_uptime(hero)`), `ActionBound -> 1.0` immer (Spec: 1.0 wenn die
Aktion in der Rotation ist, sonst 0.5). Fuer Warden (Waffen-Kern) verzerrt der
feste ShotBound-Wert die Schuss-Proc-Items, und Spirit Strike/Snatch (MeleeBound)
bekommen 0.5 statt 0.0 fuer einen Helden, der nicht zuschlaegt. Vorschlag:
Signatur um `hero` erweitern (Bump-up, da `types`/ARCHITEKTUR fix) oder die
helden-abhaengige Korrektur in Paket C nachziehen und hier als Schuld vermerken.

### 5. total mischt per_soul und per_slot in einer Zahl (wichtig)

`item.rs:51`: `total = per_slot_value + per_soul_value*100.0 + meta + patch`.
MECHANIK 7 haelt beide Sichten bewusst getrennt (fruehe Kaufwellen nach
`per_soul_value`, Endstand nach `per_slot_value`). Die Summe mit dem willkuerlichen
Faktor 100 vermengt beide und beguenstigt billige Items doppelt, ohne dass eine
der beiden Sichten sauber herauskommt. Der Faktor 100 ist nirgends begruendet.
Vorschlag: beide Werte getrennt fuehren (der Composer in C waehlt je Phase die
passende Sicht) statt sie in `total` zu addieren; wenn ein Gesamtscore noetig ist,
den Mischfaktor aus der Spec ableiten und dokumentieren.

### 6. Kaufboni je Slot auf verschiedenen Einheiten (wichtig)

`mechanics.rs:17-28`: Weapon rechnet `bonus/100 * weapon_dps`, Spirit
`bonus * (spirit_scale + spirit_dps/100)` ohne `/100`, Vitality
`bonus * base_health/100`. MECHANIK 1 verlangt ausdruecklich, dass `worth_of_stat`
alle Slots in dieselbe Kampffenster-Einheit umrechnet, damit Slot-Boni vergleichbar
und mit der Item-Wirkung addierbar sind. Die drei Zweige liegen auf
unterschiedlichen Skalen, damit ist die Kernaussage der Regel ("Spirit-Slot
schlaegt fast immer den gleich teuren Vitality-Slot") nicht auf gemeinsamer Basis
belegt. Vorschlag: alle drei Zweige auf dieselbe DPS-nahe Einheit wie
`combat_window_value` bringen.

### 7. ability_dps-Casts weichen von der Formel ab (nit)

`mechanics.rs:138`: MECHANIK 2 gibt `casts = min(charges + W/recharge,
W*uptime/channel_time)`. Der Code laesst `charges +` im ersten Term weg und
multipliziert `charges` stattdessen in den zweiten Term. Nur heuristisch relevant,
aber weicht woertlich von der Spec ab.

### 8. imbue_target ohne imbue_gain (nit)

`mechanics.rs:90`: reines `argmax ability_dps`. MECHANIK 9 gewichtet mit
`imbue_gain(item, ability)`. Fuer die meisten Faelle (staerkste
Schadensfaehigkeit) gleich, aber nicht die Formel.

### 9. StateBound als 1 - threshold statt hit_rate (nit)

`mechanics.rs:51`: `(1.0 - threshold)`. MECHANIK 4 sieht `hit_rate(threshold)`
(realistische Trefferquote) vor. Als Annahme vertretbar, aber nicht als solche
benannt und nicht die Spec-Groesse.

### 10. buy_phase an festen Kostenschwellen statt Soul-/Skalierungskurve (nit)

`mechanics.rs:101`: 1600/6400 als harte Grenzen. MECHANIK 10/12 haengt die Phase
an Soul-Kurve und Skalierungsstufe. Der Composer in C kann das ueberschreiben,
daher nur Hinweis.

### 11. Ability-Zuordnung in build_hero_model fragil (nit)

`hero.rs:24`: Zielindex ueber `slot.checked_sub(1)` als Vektorindex mit Fallback
auf die Aufzaehlung. Wenn die geladenen Abilities nicht slot-sortiert sind, trifft
das die falsche Ability. Ohne echten Payload nicht verifizierbar; am
Warden-Snapshot pruefen, wenn der DSN-Lauf laeuft.

## Formeln ohne Unit-Test mit Spec-Zahlen (Briefing Punkt 1)

Getestet mit Spec-Zahlen: `purchase_bonus_value`, `proc_stacks`, `weapon_dps`,
`per_slot_value`, `per_soul_value`, `buy_phase`, `imbue_target`, `damage_plan`,
`condition_factor` (nur RampUp und ActiveCooldown).

Ohne direkten Unit-Test mit Spec-Zahlen:
- `property_effect`/`property_value` (`mechanics.rs:188,199`): die zentrale
  Kampffenster-Wirkungsrechnung (MECHANIK 2), nur indirekt ueber den
  Score-Test beruehrt.
- `ability_dps` (`mechanics.rs:127`): kein direkter Zahlentest.
- `active_value`/`passive_value` (`mechanics.rs:76,83`): nur indirekt.
- `condition_factor`-Zweige MeleeBound, ShotBound, ActionBound, StateBound: nicht
  getestet (siehe Mangel 4).

## Paketschnitt (Review-Briefing Punkt 2)

Umgesetzt wie in REVIEW-A Mangel 1 entschieden: `hero::build_hero_model` nimmt das
geladene `HeroModel` und ergaenzt nur Skalierung, Scaling-Step und Waffenprofil;
`item::build_item_model` validiert das geladene `ItemModel` und klont es. Keine
Doppelung der Payload-Auswertung aus `data.rs`. Die Signaturaenderung gegenueber
ARCHITEKTUR 7 (geladenes Modell statt rohem Payload) ist sauber. Einzige Luecke ist
die fehlende Ability-Value-Quelle fuer `scaling_step` (Mangel 2).

## Scoring (Review-Briefing Punkt 3)

Deterministisch, endgueltige Zahlen, kein starrer Pfad spirit/weapon/tank: der
Damage-Plan (`mechanics.rs:164`) leitet die Achse aus `weapon_share` ab und laesst
Hybrid zu, der Warden-Unit-Test zeigt Waffen-Kern. Meta ist echtes Nebensignal
(`item.rs:117`, Deckel 0.5 gegen Score-Groessen im zweistelligen Bereich). Die
Waffen-Kern-Erkennung fuer Warden ist damit im Ansatz vorhanden; belastbar ist sie
erst mit dem DSN-Lauf (Mangel 1) und mit verdrahteter Proc-Rechnung (Mangel 3).

## Hygiene

Keine Secrets, keine Code-Kommentare in den drei Dateien, keine neuen Crates, keine
KI-Aufrufe. Echte Umlaute vorhanden, keine Gedankenstriche. `cargo fmt` sauber.

## Urteil

NACHBESSERN.

Blocker: Mangel 1 (Kern-Schwelle und Warden-Pflicht unbelegt, DSN-Lauf fehlt),
Mangel 2 (scaling_step ohne Datenquelle, Angelpunkt tot). Wichtig: Mangel 3, 4, 5,
6. Rest Nits.

## Was Paket D bei der Integration wissen muss

- `lib.rs` muss `hero`, `item`, `mechanics` als `pub mod` bzw. mit `pub use`
  einbinden, sonst schlaegt `clippy -D warnings` mit `dead_code` fehl.
- Es fehlt ein oeffentlicher Loader fuer rohe Ability-Values. Ohne ihn bleibt
  `scaling_step` in Produktion `None`. `data.rs::ability_snapshots` (Paket A)
  freigeben oder ein `load_hero_abilities` ergaenzen und die Fassade verketten.
- Der Echtdaten-Warden-Lauf mit DSN muss nachgeholt werden: vier Referenzscores
  und fuenf beste/schlechteste Items protokollieren, gegen eine echte Kern-Schwelle
  pruefen. Vorher keine Freigabe von B.

## Fixrunde 1

Status: Bump-up offen, noch keine Fertigfreigabe. Stand 2026-09-12.
Worktree `/home/nathanael/.worktrees/deadlock-brain-b`, Branch
`feat/build-reasoner-b`, Intent-Thread `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
Keine Unter-Agenten oder Unter-Threads gestartet.

A wurde wie beauftragt zuerst gemergt. Auch der anschließend eingetroffene
Gate-Fix `a4d1375` ist enthalten; aktueller Merge-Commit `f6f0f70`.
Die B-Fixes liegen im Arbeitsbaum, noch ohne Fix-Commit und ohne Push,
weil der verstärkte Echtdaten-Pflichttest weiterhin einen echten Fehler zeigt.
`types.rs` ist unverändert. In `data.rs` steht nur die freigegebene additive
Funktion `load_hero_abilities`, in `lib.rs` deren Export sowie die ausdrücklich
nur temporären drei `pub mod`-Zeilen für die Prüfung.

### Selbstprüfung je Review-Mangel

Zeilen beziehen sich auf den aktuellen Arbeitsbaum. Die Prüfung ersetzt nicht
die nachgelagerte unabhängige Review-Runde.

| Mangel | Datei:Zeile | Änderung und verbleibender Befund |
|---|---|---|
| 1 | `src/item.rs:104`, `src/item.rs:362` | Benannte Kern-Schwelle `CORE_SCORE_QUANTILE = 0.5`, Median des mechanischen Slot-Werts derselben Kostenklasse. Meta beeinflusst die Schwelle nicht. Test prüft alle vier Items und protokolliert alle Ränge. Quicksilver Reload bleibt unter der Schwelle, siehe Daten-Bump-up. |
| 2 | `src/data.rs:516`, `src/lib.rs:18`, `src/hero.rs:83` | Öffentlicher Ability-Loader und verketteter `load_built_hero_model`: Grundmodell, Ability-Payloads, Stat-Zeilen, Enrichment und konfigurierter Damage-Plan. Echtdaten belegen Willpower T3 mit 0,8 + 2,7 = 3,5. |
| 3 | `src/mechanics.rs:461` | Scoring ruft `proc_stacks` auf, liest `item.proc_cooldown`, nutzt Schussabstand für ShotBound und vorhandene Item-TickRate für Tick-Procs. MaxStacks begrenzt die Anzahl. Ability-Tickraten können mit den eingefrorenen Typen weiterhin nicht bis ins Scoring transportiert werden. Bump-up offen. |
| 4 | `src/mechanics.rs:37`, `src/mechanics.rs:74` | Additive Funktion `condition_factor_for_hero` im Scoring. ShotBound aus Magazin, Feuerrate und Nachladen; Melee aus Archetyp/Klasse; ActionBound aus Ability-Rollen und Klassen. Alter API-Einstieg bleibt kompatibel. Die heuristische Rotation wird als Annahme in Evidence benannt. Warden fehlt noch die echte Nachladezeit im geladenen Modell. |
| 5 | `src/item.rs:51` | `total = per_slot_value + meta_support + patch_support`. Kein Summieren von Soul- und Slot-Effizienz. `per_soul_value` bleibt für frühe Composer-Käufe verfügbar. Regression prüft identische Slot-Scores bei achtfachem Preis. |
| 6 | `src/mechanics.rs:10`, `src/mechanics.rs:361` | Kaufboni und Properties auf Schaden oder effektivem Leben pro Sekunde normiert. Spirit über Ability-Skalierung und Castzahl, Vitality über Basisleben und Kampffenster. Keine Summe sämtlicher numerischer DB-Stats als Spirit-Skalierung mehr. Basiswirkung der Abilities fehlt weiterhin im Typ, siehe Bump-up. |
| 7 | `src/mechanics.rs:258` | Casts exakt als Minimum aus Anfangsladungen plus W/recharge und W*uptime/channel. Tests mit W=40, uptime=0,55, Kanal=7 und recharge=30, einschließlich Sättigung. |
| 8 | `src/mechanics.rs:150`, `src/mechanics.rs:166` | Ziel nach Ability-Wert mal itemabhängigem Imbue-Gewinn; Cooldown und Ladungen konkurrieren mit dem Kanalbudget. Test zeigt Zielwechsel bei 50 Prozent Cooldown-Reduktion. Absolute Basiswirkung bleibt Teil des Bump-ups. |
| 9 | `src/mechanics.rs:70`, `src/item.rs:68` | Eigene Größe `hit_rate`; Gleichverteilung des Restlebens ausdrücklich als ungemessene Annahme in Evidence. 65 Prozent Schwelle ergibt unter dieser Annahme 0,35. |
| 10 | `src/mechanics.rs:194`, `src/mechanics.rs:209` | Additive heldenabhängige Kaufphase aus Soul-Kurve und frühester Skalierungsstufe. Annahme: Upgrade-Kosten 1/2/5, Unlock-Level 1/3/5/8; fehlende Kurve/Stufe fällt sichtbar auf Kostenheuristik zurück. Verteidigung wird Late. Dies ist eine früheste Phase, keine vollständige kumulative Kaufplanung. |
| 11 | `src/hero.rs:19` | Zuordnung durch Position der passenden Ability-ID, kein Slot als Vektorindex und kein Enumerations-Fallback. Unbekannte IDs erzeugen einen Datenfehler. Test mit unsortierter Ability-Liste und Echtdaten bestanden. |

Weitere aus dem Echtdatenlauf abgeleitete Korrekturen: EAddToScale addiert den
Bonus auf `properties.<name>.scale_function.stat_scale`; gewöhnlicher
WeaponPowerDebuff wird nicht mehr als Skalierungsstufe erkannt. Doppelte
Properties in aktiver und passiver Map zählen einmal. Bullet-Speed, MaxStacks
und rohe AbilityCooldown-Werte erzeugen keinen direkten Schaden. Instant Reload
zählt gesparte Nachladezeit statt pauschal 100 Prozent mehr Waffen-DPS.

### Testnachweise

Alle Läufe nutzen die Debug-Toolchain unter `/home/nathanael/.cargo/bin`.
Die drei B-Module waren für sämtliche Paket-Prüfungen temporär öffentlich
in `lib.rs` eingebunden.

| Stand | Ohne DSN | Mit Echtdaten-DSN und isoliertem Scratch-DSN |
|---|---|---|
| Anfang nach erstem A-Merge | 20 bestanden, 0 fehlgeschlagen, 6 ignoriert | Zwei echte Warden-Tests bestanden; zunächst vier Fehler wegen fehlender Scratch-Konfiguration |
| Vergleichsbaseline nach A-Gate-Fix, unveränderter B-Code aus 68dc58c | 24 bestanden, 0 fehlgeschlagen, 6 ignoriert | 30 bestanden, 0 fehlgeschlagen, 0 ignoriert |
| Aktueller Fixstand | 44 bestanden, 0 fehlgeschlagen, 6 ignoriert | 49 bestanden, 1 fehlgeschlagen, 0 ignoriert |

Der eine echte Fehler ist ausschließlich Quicksilver Reload unter der
Kern-Schwelle. Kein Skip ohne DSN im verstärkten B-Pflichttest.
Die Scratch-Instanz wurde separat unter `/tmp/reasoner-b-pg` auf Port 55439
mit Datenbank `reasoner_a_fix` gestartet. Nur dort liefen die schreibenden
A-Tests. Der Echtdatenzugang lief mit `default_transaction_read_only=on`;
der B-Pflichttest erzwingt dies zusätzlich beim Verbindungsaufbau.

Rot-Gegenproben für alle 20 neuen Tests:

- Erste elf Regressionen vor den Implementierungsänderungen: 20 bestanden,
  11 fehlgeschlagen, 6 ignoriert.
- Gegner-Debuff/Selbstkosten-Test vor der Korrektur: 38 bestanden,
  1 fehlgeschlagen, 6 ignoriert.
- Reload-Regression und erweiterter Property-Test vor der Korrektur:
  42 bestanden, 2 fehlgeschlagen, 6 ignoriert.
- Temporäre Rücknahmen von Rotations-, Imbue-, Phasen-, Doppelzählungs-,
  Tick-, Total- und Skalierungsfix: 36 bestanden, 8 fehlgeschlagen,
  6 ignoriert. Diese Mutationen wurden anschließend vollständig entfernt.

Formatter auf den drei eigenen B-Dateien und
`cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` sind grün.
Es gibt keine Code-Kommentare in den drei B-Dateien. Kein externer KI-Aufruf;
A enthält einen lokalen HTTP-Fixture-Test für den Kritiker.

Prüflogs liegen unter `/tmp/reasoner-b-red.log`,
`/tmp/reasoner-b-red2.log`, `/tmp/reasoner-b-red3.log`,
`/tmp/reasoner-b-mutants.log`, `/tmp/reasoner-b-baseline-final-a.log`,
`/tmp/reasoner-b-baseline-final-a-dsn.log`, `/tmp/reasoner-b-live2.log`.

### Warden-Zahlen des aktuellen, noch unvollständigen Modells

W=40 s, Kanalanteil=0,55, Meta leer, Patch-Deltas leer.
Score ist `per_slot_value`; in diesem Lauf ist `total` identisch.
Diese Zahlen sind ein Fehlernachweis, noch keine fachliche Freigabe des Builds.

| Referenz-Item | Score | Kern-Schwelle gleicher Kostenklasse | Ergebnis |
|---|---:|---:|---|
| Veil Walker | 11,867174 | 10,577337 | darüber |
| Mercurial Magnum | 53,035390 | 15,218273 | darüber |
| Siphon Bullets | 35,918983 | 15,218273 | darüber |
| Quicksilver Reload | 9,918407 | 10,735000 | darunter |

| Gruppe | Item | Score |
|---|---|---:|
| Top 1 | Express Shot | 128,860088 |
| Top 2 | Spirit Burn | 103,856031 |
| Top 3 | Lucky Shot | 101,467107 |
| Top 4 | Frenzy | 89,569428 |
| Top 5 | Juggernaut | 88,854534 |
| Bottom 5 | Extra Stamina | 1,411637 |
| Bottom 4 | Extra Charge | 1,124294 |
| Bottom 3 | Mystic Expansion | 1,122370 |
| Bottom 2 | Golden Goose Egg | -4,734296 |
| Bottom 1 | Trophy Collector | -6,671390 |

### Bump-up und konkrete Fortsetzung

[Bump-up] Paket B: Grund: Der erlaubte Loader-Zugriff liefert nur die vier
Signature-Abilities. Wardens Hero-Payload enthält kein `weapon_info`; seine
Stat-Zeilen enthalten Basiswaffenschaden, Feuerrate und Magazin, aber keine
Nachladezeit. Die echte Waffenquelle ist der Snapshot `item_or_ability` mit
`class_name = citadel_weapon_warden_set`, referenziert durch
`hero.items.weapon_primary`. Er enthält `reload_duration = 2.914`,
`bullet_damage = 17.34`, `shots_per_second = 3.8095238095238098`,
`clip_size = 17`, `damage_per_second_with_reload = 38.6520684455517`.
Aktuell ist `reload_duration` im HeroModel weiterhin 0. Ohne diese Daten
bekommt Quicksilver Reload keinen rechnerischen Reload-Gewinn.

Zusätzlich fehlen in AbilityModel Basiswirkung, Tickrate und Wirkungsdauer.
Die vorhandene Ability-Skalierung allein ist kein absoluter Schadenswert.
Damit sind der derzeitige Spirit-DPS-Wert und der Waffenanteil nur ein
unzureichendes Modell; die grünen ersten Referenzscores sind kein Ersatz
für den vollständigen Datenweg.

Erledigt: freigegebener Loader, mechanische Fixes und Regressionen, DSN-Zugang,
komplette Testläufe samt echtem negativem Pflichtnachweis, Schwelle unverändert.
Worktree: `/home/nathanael/.worktrees/deadlock-brain-b`.
Offen: Freigabe zur additiven Erweiterung von `types.rs` und der dazugehörigen
Loader-Pfade in `data.rs`, alternativ verbindliche Übergabe dieser
Datenkorrekturen an A/D. Konkrete Änderung: Waffensnapshot über
`hero.items.weapon_primary` auflösen, vorhandenes WeaponProfile vollständig
füllen; AbilityModel um transportierte Basiswirkung/Tickrate/Dauer ergänzen,
vorhandene `scale_function`-Werte übernehmen und damit die bereits angebundene
Proc-/Kampffensterrechnung speisen. Danach Schwellenlauf unverändert wiederholen,
Formatter/Clippy/Tests abschließen, temporäre Modulzeilen entfernen, nur B-Dateien
und freigegebene Ergänzungen mit Modell-Trailer committen und ausschließlich
`feat/build-reasoner-b` pushen. Der aktuelle rote Pflichtlauf wird nicht gepusht.

Secret-Hinweis: Ein früher fehlgeschlagener psql-Aufruf hat einen Teil des DSN
in seiner Tool-Fehlerausgabe gezeigt. Folgende Verbindungsfehler wurden
abgeschirmt. Keine DSN-Werte stehen in diesem Bericht oder im Diff.

Vor der Übergabe wurden die drei temporären Modulzeilen wieder entfernt und
die eigene Scratch-Postgres gestoppt. Für weitere B-Paket-Tests erneut
`pub mod hero; pub mod item; pub mod mechanics;` temporär ergänzen und die
bestehende Scratch-Instanz mit `pg_ctl -D /tmp/reasoner-b-pg -l
/tmp/reasoner-b-pg.log -o '-k /tmp/reasoner-b-pg-socket -p 55439 -h 127.0.0.1'
start` starten. Der Quellstand einschließlich aller Fixes bleibt erhalten.
