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

Status: umgesetzt, geprüft und auf `origin/feat/build-reasoner-b` gepusht.
Keine offenen Bump-ups. Unabhängige Review durch den Orchestrator ausstehend.

Worktree `/home/nathanael/.worktrees/deadlock-brain-b`, Intent-Thread
`33a32f58-476b-4a67-99cc-8f6c1e8f7001`. Keine Unter-Threads oder Unter-Agenten.
`main` wurde nicht verändert oder gepusht. Der Arbeitsbaum ist sauber.

Commits in verlangter Reihenfolge:

- A zuerst gemergt, anschließend auch dessen Gate-Fix `a4d1375` nachgezogen;
  zweiter Merge `f6f0f70`.
- `9b88e27`: eigener Gate-Commit für die zwei Blocker und sechs Nits,
  einschließlich der freigegebenen Ability-Felder und des Waffen-Loaders.
- `fc5bf2f`: separater Commit für die B-Review-Fixes und ihre Regressionen.

Beide Fix-Commits tragen `Co-authored-by: GPT-6 <gpt-6@local>`.
Die additive Typ-/Loader-Erweiterung wurde während dieser Fixrunde vom
Orchestrator ausdrücklich freigegeben. Geänderte Dateien insgesamt:
`Cargo.toml`, `rust/Cargo.lock`, `src/types.rs`, `src/data.rs`, `src/lib.rs`,
`src/hero.rs`, `src/item.rs`, `src/mechanics.rs`, jeweils im Reasoner-Crate,
abgesehen vom Workspace-Lockfile.

### Elf Review-Mängel

Dateizeilen beziehen sich auf `fc5bf2f`, Basis für `src/` ist
`rust/crates/dbrain-reasoner/`. Die Selbstprüfung prüft die Erfüllung des
Briefings und ersetzt keine unabhängige Review.

| Nr. | Datei:Zeile | Änderung | Commit |
|---|---|---|---|
| 1 | `src/item.rs:104`, `src/item.rs:365` | Benannte Kern-Schwelle als Median des mechanischen Slot-Werts derselben Kostenklasse, `CORE_SCORE_QUANTILE = 0.5`. Der DSN-Test prüft alle vier Referenzen, das Waffenprofil und Willpower T3. Alle vier liegen darüber. | `fc5bf2f` |
| 2 | `src/data.rs:599`, `src/lib.rs:13`, `src/hero.rs:57` | Öffentlicher `load_hero_abilities`, exportiert und durch `load_built_hero_model` mit Grundmodell, Stat-Zeilen und Enrichment verkettet. Skalierungsstufe hat damit einen ausführbaren Datenweg. | `fc5bf2f` |
| 3 | `src/types.rs:154`, `src/data.rs:267`, `src/mechanics.rs:443` | Basiswirkung, Tickrate und Dauer werden transportiert. Proc-Wert verwendet Schussabstand, Item-Tickrate oder Ability-Tickrate gegen Proc-Cooldown und MaxStacks. Direkter Test für einen 7-s-Kanal mit Tick 0,1 und Cooldown 0,7 ergibt 10 Procs. | `9b88e27`, `fc5bf2f` |
| 4 | `src/mechanics.rs:37`, `src/mechanics.rs:74`, `src/mechanics.rs:143` | Additives `condition_factor_for_hero` ersetzt die Konstanten im Scoring. Melee/Action aus Archetyp, Klassen und Rollen, ShotBound aus Magazin-/Reload-Anteil. Auch Melee-Shred in der passiven Map benötigt Melee; passives Leben bleibt erhalten. | `fc5bf2f` |
| 5 | `src/item.rs:51` | `total = per_slot_value + meta_support + patch_support`; keine Vermischung mit Soul-Effizienz. `per_soul_value` bleibt getrennt für frühe Käufe. | `fc5bf2f` |
| 6 | `src/mechanics.rs:10`, `src/mechanics.rs:343` | Kaufboni und Properties werden in Schaden oder effektivem Leben pro Sekunde bewertet. Spirit über Skalierung, Castzahl und gegebenenfalls Wirkungsdauer; Vitality über Basisleben/W. Keine ungefilterte Summe aller DB-Stats als Spirit-Scale. | `fc5bf2f` |
| 7 | `src/data.rs:354`, `src/mechanics.rs:263` | Gemeinsame Cast-Formel: min(charges + W/recharge, W*uptime/channel). Anfangsladungen und Kanalbudget sind separat getestet. | `9b88e27`, `fc5bf2f` |
| 8 | `src/mechanics.rs:161`, `src/mechanics.rs:177` | Imbue-Ziel maximiert Ability-DPS mal itemabhängigem Gewinn; Damage, Cooldown-Reduktion und Ladungen gehen ein. Test zeigt den Zielwechsel bei erschöpftem Kanalbudget. | `fc5bf2f` |
| 9 | `src/mechanics.rs:70`, `src/item.rs:68` | Eigene Größe `hit_rate`; Gleichverteilung des Restlebens explizit als ungemessene Annahme in Evidence. 65 Prozent Schwelle ergibt darunter 0,35. | `fc5bf2f` |
| 10 | `src/mechanics.rs:199`, `src/mechanics.rs:214` | Heldenspezifische Kaufphase anhand Soul-Kurve und frühester Skalierungsstufe, Verteidigung als Late. Fehlende Kurve/Stufe fällt sichtbar auf die Kostenheuristik zurück. | `fc5bf2f` |
| 11 | `src/hero.rs:19` | Zuordnung über die Position der passenden Ability-ID, kein Slot als Vektorindex und kein Enumerations-Fallback. Unbekannte IDs sind ein Datenfehler. Unsortierte Liste und echter Warden-Snapshot geprüft. | `fc5bf2f` |

Skalierungsstufe am echten Snapshot: Willpower, Ability-ID `2751689917`,
Upgrade-Index 2, `CombatBarrier`, `scale_function.stat_scale = 0.8`,
`EAddToScale`-Bonus 2.7, Ergebnis 3.5. Gewöhnlicher WeaponPowerDebuff ist
keine Skalierungsstufe. Der Test mit den Spec-Zahlen für Life Drain prüft
0.3225 + 0.3 = 0.6225.

### Gate-Funde

Alle acht Funde wurden im separaten Commit `9b88e27` behoben. Zeilen unten
beziehen sich für die Navigation auf den finalen Stand `fc5bf2f`.

| Gate | Datei:Zeile | Änderung und Nachweis |
|---|---|---|
| 1, blockierend | `src/data.rs:267`, `src/data.rs:362`, `src/data.rs:441`, `src/data.rs:574` | Spirit-DPS aus Basiswirkung mal Castzahl/W, nicht Cooldown/Kanal. 60 Schaden bei 30 s Cooldown ergibt 3,5 DPS im 40-s-Fenster; längerer Cooldown senkt DPS. Waffensnapshot über `items.weapon_primary`, einschließlich Nachladezeit. Echter Warden hat Waffenanteil 0,604215. |
| 2, blockierend | `src/data.rs:242` | Objektschlüssel bleiben Zielstats, EFireRate und ERoundsPerSecond getrennt. Scale wird nur bei Spirit-Bezug als per_spirit verwendet, per_level nur aus ausdrücklichem per_level-Feld. Test korrigiert und erweitert. B verwendet denselben Parser statt einer zweiten Auswertung. |
| 3 | `src/data.rs:946` | Tatsächliche Stat-Schlüssel geprüft. Numerische Zeilen werden dedupliziert, Null bleibt unbekannt. Nur `spirit_scaling.*` ist per_spirit; `_per_level`/`_per_boon` ist Level-/Boon-Zuwachs. Basis-HP, DPS, Ratings usw. werden nicht als Skalierungsstats ausgegeben. Echtdaten: sechs passende Stat-Zeilen. |
| 4 | `src/data.rs:794` | Patch-Events nach ID dedupliziert, Enrichment-Auswahl durch sortierte JSON-Werte deterministisch. Scratch-Test enthält doppelte Enrichments und prüft neben der ID-Menge ausdrücklich die Zeilenzahl. |
| 5 | `src/data.rs:158` | Fehlendes, null oder false Imbue-Flag ergibt false. Direkter Test mit diesen Fällen und true. |
| 6 | `src/data.rs:521` | Ein benannter, fehlender Signature-Snapshot führt zu MissingSnapshot. Es wird kein verkürzter Vektor mit verschobenen Slots zurückgegeben. Scratch-Regression bestanden. |
| 7 | `src/data.rs:1158` | Zeitstempel in Fixture mit explizitem UTC-Offset; Test setzt die Verbindung auf Pacific/Honolulu und prüft dieselben Unix-Zeitwerte. |
| 8 | `Cargo.toml:11`, `rust/Cargo.lock` | Die vier ungenutzten direkten Abhängigkeiten anyhow, dbrain-builds, dbrain-learn und dbrain-retrieval entfernt; Lockfile entsprechend aktualisiert. |

AbilityModel hat drei additive, mit Serde-Defaults kompatible Felder:
`base_effect` als Schaden je vollständiger Aktivierung, `tick_rate` in Sekunden,
`duration` in Sekunden. Für PulseDPS wird die Basiswirkung mit der Dauer
multipliziert, DamagePerTick berücksichtigt Tickzahl. Das bereits vorhandene
`HeroModel.weapon.reload_duration` wird vollständig gefüllt; ein zweites,
widersprüchliches Reload-Feld auf HeroModel-Ebene wurde nicht eingeführt.

### Tests und Rot-Gegenproben

| Stand | Ohne DSN | Mit DSN und Scratch-DB |
|---|---|---|
| B-Baseline nach erstem A-Merge | 20 bestanden, 0 fehlgeschlagen, 6 ignoriert | Zwei Warden-Tests grün, zunächst vier fehlende Scratch-Konfigurationen |
| Vergleichsbaseline nach A-Gate-Fix a4d1375, alter B-Code | 24 bestanden, 0 fehlgeschlagen, 6 ignoriert | 30 bestanden, 0 fehlgeschlagen, 0 ignoriert |
| Separater Gate-Auftrag, Baseline ohne temporäre B-Module | 11 bestanden, 0 fehlgeschlagen, 5 ignoriert | Vorbestehende integrierte Baseline siehe vorige Zeile |
| Gate-Commit 9b88e27 ohne temporäre B-Module | 14 bestanden, 0 fehlgeschlagen, 6 ignoriert | 20 bestanden, 0 fehlgeschlagen, 0 ignoriert |
| Endstand mit temporär eingebundenen B-Modulen | 53 bestanden, 0 fehlgeschlagen, 7 ignoriert | 60 bestanden, 0 fehlgeschlagen, 0 ignoriert |

`cargo fmt -p dbrain-reasoner -- --check` und
`cargo clippy -p dbrain-reasoner --all-targets -- -D warnings` sind grün.
Toolchain über `/home/nathanael/.cargo/bin`, Debug-Build, kein Release.
Die sieben ignorierten Tests sind fünf Scratch-Tests und zwei Echtdaten-Tests;
beim DSN-Lauf wurden alle ausgeführt. Kein externer KI-Aufruf, A enthält nur
einen lokalen HTTP-Fixture-Test für den Kritiker.

Die drei `pub mod`-Zeilen für B waren entsprechend dem Briefing nur zur
Paket-Prüfung eingebunden und sind nicht im Commit. Der einzige B-Diff in
`lib.rs` ist der freigegebene Loader-Export. Formatter, Git-Diff-Check und
Arbeitsbaum sind auch nach Entfernen der temporären Zeilen sauber.

Rot-Nachweise:

- Erste elf B-Regressionen gegen die alte Implementierung: 20 grün, 11 rot,
  6 ignoriert.
- Gezielte Rücknahme von Rotation, Imbue-Gewinn, Phasen, Doppelzählung,
  Tick-Verdrahtung, Total-Mischung und Skalierungs-Erkennung: 36 grün,
  8 rot, 6 ignoriert. Mutationen danach vollständig entfernt.
- Gegner-Debuff/Selbstkosten: 38 grün, 1 rot; Reload/Property-Einheiten:
  42 grün, 2 rot.
- Gate-Funde vor Korrektur: 12 grün, 5 rot, 1 gefilterter Echtdaten-Test.
- Gate-Gegenprobe für Basiswirkung, Imbue-null und nicht-UTC-Fixture:
  16 grün, 4 rot, 1 gefilterter Echtdaten-Test. Originaldatei anschließend
  wiederhergestellt, Git-Diff leer.
- Neue Ability-Tick-Verdrahtung und nicht-zirkulärer Damage-Plan: 47 grün,
  2 rot, 7 ignoriert vor Implementierung.
- PulseDPS-Dauer: gezielter Test 0 grün, 1 rot vor Korrektur.
- Aktive Dauer/Cooldown und passiver Melee-Shred: 50 grün, 2 rot vor Korrektur.
- Max-HP-Entzug: gezielter Test 0 grün, 1 rot vor Korrektur.

Alle zuvor ungetesteten Formeln haben direkte Zahlentests: property_effect/
property_value, ability_dps/Casts, aktive und passive Werte sowie sämtliche
heldenabhängigen Condition-Zweige. Die weiteren Regressionen prüfen auch
Kern-Schwelle, Reload-Zeitgewinn, Identitätszuordnung und Enrichment.

Echtdaten nur lesend: `default_transaction_read_only=on` über Verbindungsoptionen
und zusätzlich im B-Pflichttest erzwungen. Schreibende A-Tests liefen nur in der
eigenen lokalen Instanz `/tmp/reasoner-b-pg`, Port 55439, Datenbank
`reasoner_a_fix`. Keine Migration und kein Schreibzugriff auf Produktionsdaten.

### Warden-Echtdatenlauf

251 geladene Items, vier Referenz-Items, sechs Skalierungsstat-Zeilen,
416 deduplizierte Patch-Zeilen, drei Autoren-Builds. Scoring hier ohne Meta
und ohne Patch-Deltas, W=40 s, Kanalanteil=0,55. `total` entspricht deshalb
`per_slot_value`. Schwelle: Median der gleichen Kostenklasse, vor dem ersten
Scoring-Lauf festgelegt und danach unverändert.

Waffenprofil aus `citadel_weapon_warden_set`: Bullet damage 17,34,
shots_per_second 3,809524, Magazin 17, Reload 2,914 s. Damage-Plan:
38,652068 Waffen-DPS, 25,318627 Spirit-DPS, Waffenanteil 60,421523 Prozent,
Primärachse Weapon.

| Referenz-Item | Score | Kern-Schwelle | Ergebnis |
|---|---:|---:|---|
| Veil Walker | 13,853285 | 13,433479 | darüber |
| Mercurial Magnum | 50,246984 | 18,928813 | darüber |
| Siphon Bullets | 28,149679 | 18,928813 | darüber |
| Quicksilver Reload | 23,825812 | 12,280908 | darüber |

| Ranggruppe | Item | Score |
|---|---|---:|
| Top 1 | Spirit Burn | 104,139994 |
| Top 2 | Juggernaut | 73,451349 |
| Top 3 | Express Shot | 72,665889 |
| Top 4 | Frenzy | 65,451358 |
| Top 5 | Mystic Conduit | 61,962848 |
| Bottom 5 | Healing Rite | 1,408750 |
| Bottom 4 | Inhibitor | 0,348141 |
| Bottom 3 | Spirit Sap | -1,558071 |
| Bottom 2 | Trophy Collector | -2,187810 |
| Bottom 1 | Golden Goose Egg | -3,480256 |

Zwischenzeitliche Pflichtfehler wurden nicht durch Schwellenänderung beseitigt:
Quicksilver Reload lag ohne Nachladezeit bei 9,918 unter 10,735. Behoben durch
das echte Waffenprofil und Bewertung gesparter Reload-Zeit statt pauschal
100 Prozent zusätzlicher Waffen-DPS. Nach korrekter Dauerbewertung der
Spirit-Alternativen lag Siphon Bullets bei 18,004 unter 18,455. Dessen
verifizierter Effekttext beschreibt Max-HP-Entzug, bisher zählte nur eine
Seite. Nun zählen Zielverlust und eigener Lebensgewinn in derselben Einheit.
Veil Walker und Mercurial Magnum lagen in den Pflichtläufen über der Schwelle.
Keine Gewichte wurden auf das Referenzergebnis eingestellt.

### Annahmen und Integration durch C/D

- `hit_rate` bleibt ohne Telemetrie eine ausdrücklich genannte Gleichverteilung
  des Restlebens. Die Rotation wird aus Archetyp, Ability-Klasse und Rollen
  abgeleitet, nicht als beobachtete Spielrotation ausgegeben.
- Kaufphase ist die früheste modellierte Phase. Annahmen: Upgrade-Kosten 1/2/5,
  Ability-Unlocks auf Level 1/3/5/8; die vollständige kumulative Kaufplanung
  bleibt Aufgabe des Composers. Soul- und Slot-Effizienz sind getrennt.
- Max-HP-Entzug ist Bruttowirkung der Procs im Fenster: angenommene Ziel-HP gleich
  Hero-Basisleben, Zielverlust plus eigener Gewinn, ohne Rückgabe bei Debuff-Ende.
  Diese Grenze steht explizit in der Item-Evidence. Die Tabelle ist mechanische
  Einzelitem-Bewertung, kein bereits getesteter vollständiger Composer-Build.
- C/D bindet `hero`, `item`, `mechanics` öffentlich ein und verwendet
  `hero::load_built_hero_model(ctx, name)` oder dieselbe Loader-Verkettung.
  Bestehende Rust-Struct-Literale für AbilityModel müssen die drei neuen Felder
  setzen; ältere JSONs bleiben durch Serde-Defaults lesbar.
- C hat `order_proximity` zu Option<f64> geändert. Dieses Feld wurde hier nicht
  angefasst; C ist nicht eigenmächtig in B gemergt worden. Die durch C tatsächlich
  benötigte Abhängigkeit dbrain-builds muss bei dessen Modul-Integration wieder
  als verwendete Abhängigkeit aufgenommen werden. Der Gate-Commit entfernt nur
  die im A-Stand ungenutzten direkten Abhängigkeiten.
- Bericht und genaue Score-Ausgaben: `/tmp/reasoner-b-final-unit.log`,
  `/tmp/reasoner-b-final-dsn.log`, Gate-Nachweise unter
  `/tmp/reasoner-b-gate-baseline.log`, `/tmp/reasoner-b-gate-dsn.log`,
  `/tmp/reasoner-b-gate-red.log`, `/tmp/reasoner-b-gate-counterprobe.log`.

Secret-Hinweis: Ein früher fehlgeschlagener psql-Aufruf hat einen Teil des DSN
in seiner Tool-Fehlerausgabe gezeigt. Folgende Verbindungsfehler wurden
abgeschirmt. Kein DSN-Wert steht in diesem Bericht oder im Code-Diff.
