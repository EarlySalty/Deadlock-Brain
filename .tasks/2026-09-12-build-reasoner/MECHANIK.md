# MECHANIK: build-reasoner

status: Entwurf (2026-09-12), Design-Paket P0, lesend

Jede Regel ist eine Formel mit Datenquelle. Feldnamen sind gegen die echten
Rohdaten geprueft (`data/raw/deadlock_assets_api/heroes.113bc12ea286f2a2.json`,
`items.f16f8c8dd04ab5d9.json`) und gegen `dbrain-builds/src/engine.rs`
(brain-Schema). Zahlen erzeugt ausschliesslich der Rust-Kern, nie die KI.

Grundeinheiten (Default-Annahmen, im Report sichtbar, per `ReasonerConfig`
einstellbar): Kampffenster `W = 40 s`, Kanalanteil `channel_uptime = 0.55`. Diese
zwei Zahlen sind gesetzte Annahmen aus der Skill-Datei, keine gemessenen Werte.

## 1. Kaufbonus je Slot und Tier

Quelle: Hero-Payload `purchase_bonuses`, verifiziertes Format:
`purchase_bonuses.spirit[] = {tier, value, value_type}` (analog `weapon`,
`vitality`), z.B. spirit T1..T5 = 4, 7, 10, 13, 16 mit
`value_type = MODIFIER_VALUE_TECH_POWER`.

```
purchase_bonus_value(item, hero):
  slot   = item.slot                         # Weapon | Vitality | Spirit
  tier   = item.tier                          # item_tier 1..5
  bonus  = hero.purchase_bonuses[slot][tier].value
  return worth_of_stat(bonus, item.slot, hero)
```

`worth_of_stat` rechnet den Bonus in dieselbe Kampffenster-Wirkungseinheit um wie
`combat_window_value`, damit Slot-Boni und Item-Wirkung addierbar sind: ein
Spirit-Bonus geht ueber die Spirit-Skalierung der Kern-Faehigkeit in DPS, ein
Weapon-Bonus ueber das Waffenprofil. Folge (Skill-Regel): ein Spirit-Slot schlaegt
fast immer den gleich teuren Vitality-Slot, weil der Tier-Bonus mitzaehlt; billige
T1-Spirit-Items werden dadurch nicht mehr unterschaetzt.

## 2. Wirkung im Kampffenster (ersetzt Winrate-Ranking)

Quelle: Item `properties` (flache Zahlen, z.B. `BaseAttackDamagePercentBonus`,
`BulletShieldMaxHealth`, `SpiritPower`), Ability `properties`
(`AbilityChannelTime`, `AbilityCharges`, `AbilityCooldown`,
`AbilityCooldownBetweenCharge`, `AbilityDuration`), Hero `scaling_stats`.

Die tragfaehige Metrik ist Wirkung im Kampffenster, nicht pro Cast und nicht im
Dauerbetrieb (Skill, Abschnitt Metrikfalle):

```
casts = min(charges + W / recharge_time,
            W * channel_uptime / channel_time)
combat_window_value(item, hero, cfg):
  raw   = property_effect(item, hero)         # DPS-Beitrag der Item-Properties
  return raw * condition_factor(item, cfg)
```

`property_effect` gewichtet Properties nach ihrer Wirkung pro Sekunde Kampf: Spirit,
Resistenz-Shred, Heilverstaerkung, Lifesteal, Waffenschaden zaehlen voll; Dauer-,
Ladungs- und reine Cooldown-Properties zaehlen nur ueber ihren Spirit-/Waffenanteil,
weil Kanaldauer und Ladungen um dieselbe Kampfzeit konkurrieren.

## 3. Tickrate gegen Proc-Cooldown

Quelle: Ability `properties` mit `*TickRate` (verifiziert: `BleedTickRate`,
`BarbedWireTickRate`), Item `properties` mit `*ProcCooldown` bzw. Proc-Feldern
(verifiziert: `BoloProcDamage`, `*PerStack` wie `AmpPercentPerStack`).

```
proc_stacks(tick_rate, proc_cooldown, window):
  by_ticks = window / tick_rate
  by_proc  = window / proc_cooldown
  return min(by_ticks, by_proc)
```

Beispiel aus der Skill-Datei: `TickRate 0.1` gegen `ProcCooldown 0.7` ergibt in
einem 7-Sekunden-Kanal rund neun Stapel. Ein Stapel-Item auf einem Kanal ist damit
weit mehr wert als auf einem Einzelschlag. Immer beide Zahlen gegeneinander rechnen,
nie den Itemtext glauben.

## 4. Bedingte Items abwerten

Quelle: `is_active_item` (bool, verifiziert), `activation` (verifiziert),
`description.desc` (Bedingung steht nur im Text, nie im Property-Block). ConditionKind
wird aus `description.desc` klassifiziert (`item::classify_condition`), bei
Uneindeutigkeit hilft der Item-Analyst mit einem Text-Label, aendert aber keine Zahl.

```
condition_factor(item, cfg):
  match item.condition:
    None                        -> 1.0
    ActiveCooldown{uptime, cd}  -> clamp(uptime, 0.0, 1.0)     # voll nur wenn Fenster den Cast deckt
    ActionBound{action}         -> if action in hero_rotation(hero) then 1.0 else 0.5
    RampUp{ramp_seconds}        -> max(0.0, 1.0 - ramp_seconds / cfg.combat_window_seconds)   # etwa halb
    StateBound{threshold}       -> hit_rate(threshold)          # realistische Trefferquote
    MeleeBound                  -> if hero_melees(hero) then 1.0 else 0.0
    ShotBound                   -> weapon_uptime(hero)
```

Beispiele (Skill): Infuser (aktiv, 7 s alle 30) nur anteilig; Arcane Surge
(dash-jump-gebunden) voll, wenn der Dash zum Ablauf gehoert; Spiritual Overflow
(erst 5 s Aufladung) etwa halb; Enchanter's Emblem (ueber 65 Prozent Leben) nach
Trefferquote. Spirit Strike und Spirit Snatch geben Shred nur `MeleeBound` und sind
fuer einen Helden, der nicht zuschlaegt, tot; Mystic Vulnerability gibt Shred, sobald
das Ziel Spirit-Schaden nimmt, also selbst ausgeloest durch einen Kanal.

## 5. Aktiv- gegen Passivwert

Quelle: `is_active_item = true` plus eine zweite Property mit dem Zusatz `Passive`
im Namen (Skill). Beide getrennt modellieren:

```
active_value  = property_effect(item.properties)         # grosse Zahl, nur im Fenster des Casts
passive_value = property_effect(item.passive_properties) # Dauerwert
combat_contribution = passive_value + active_value * condition_factor(item, cfg)
```

## 6. disabled und shopable

Quelle: `shopable` (bool, verifiziert), `disabled` (Hero-Payload bool; auf Item-
Ebene meist null). Regel: `shopable = false` oder `disabled = true` schliesst das
Item ganz aus dem Ranking aus, auch wenn es in den Daten noch steht.

## 7. Slot-Knappheit gegen Soul-Budget

Frueh limitieren Souls, spaet limitieren Slots (Skill 5b). Zwei Sichten:

```
per_soul_value(item) = combat_window_value(item, hero, cfg) / item.cost
per_slot_value(combat, purchase) = combat + purchase
```

Der Composer waehlt fruehe Kaufwellen nach `per_soul_value`, den Endstand nach
`per_slot_value`. Ein Greedy nach Souls stapelt sonst billige T1-Items, die spaeter
verkauft werden muessen; solche Items bekommen im Build eine `sell_priority` gesetzt.
Praxisregel: erste Kaufwelle lieber drei T2 als zwei T1 plus ein T2.

## 8. Nebenwaffe

Quelle: `weapon_info` (verifizierte Felder: `bullet_damage`, `shots_per_second`,
`bullets_per_second`, `clip_size`, `cycle_time`, `reload_duration`,
`damage_per_second`, `damage_per_second_with_reload`, `range`,
`damage_falloff_start_range`, `damage_falloff_end_range`, `crit_bonus_start`).

```
weapon_dps(weapon, window):
  cycle = clip_size / shots_per_second
  return damage_per_magazine / (cycle + reload_duration)      # entspricht damage_per_second_with_reload
```

Waffen-Slots gehoeren gefuellt, aber mit Hybriden: ein Waffen-Slot mit reinem
Waffenschaden ist verlorener Platz, ein Waffen-Slot mit Shred oder Spirit ist
Pflicht. Die Waffe zaehlt zusaetzlich als Ausloeser fuer `ShotBound`-Items und als
Einkommensquelle.

## 9. Imbue-Zuordnung

Quelle: Item-Flag imbueable (Items mit Imbue-Marker, im Screenshot Quicksilver
Reload und Mercurial Magnum). Ein Imbue-Item wirkt auf eine gewaehlte Faehigkeit.

```
imbue_target(item, hero):
  return argmax over hero.abilities of
         ability_dps(ability, hero, cfg) * imbue_gain(item, ability)
```

Das Imbue geht auf die Faehigkeit mit dem groessten Wirkungszuwachs, meist die
Kern-Schadensfaehigkeit. Der gewaehlte `ability_id` wird `BuildItem.imbue_target`
und im Payload das Feld `imbue`.

## 10. Soul-Kurve und Kaufzeitpunkte

Quelle: Hero `level_info` (Soul-Staende je Level), Preisstufen 800, 1600, 3200,
6400; Meta `avg_buy_time_relative` aus `brain.hero_item_stats` als
Realitaetsabgleich, wann Items in echten Spielen fallen.

Drei Kaufphasen:
- **Lane** (`BuyPhase::Lane`): billige Multiplikatoren, die sofort wirken; Shred ist
  meist das Beste pro Soul. 4 bis 6 Items.
- **Kern** (`BuyPhase::Core`): nach der Skalierungsstufe (Abschnitt 12) Spirit oder
  Waffe stapeln, je nach Damage-Plan.
- **Spaetspiel** (`BuyPhase::Late`): Verteidigung (bei Kanaelen Pflicht, nicht Kuer)
  und situative Items mit sichtbarer Bedingung.

Die Buy-Order richtet sich an der Skalierungsstufe aus, nicht umgekehrt: alles
sparen, was noetig ist, um sie so frueh wie moeglich zu bekommen.

## 11. Helden-Modell

Quelle je Feld:

| Modellteil | Payload-Feld |
|---|---|
| Damage-Plan | `weapon_info` gegen Ability-`properties`, gewichtet mit Zeitanteil im Kampf |
| Skalierungsstats | Hero `scaling_stats`, `starting_stats`, `brain.hero_stat_values` |
| Faehigkeitsrollen | Ability `class_name`, `properties`, `behaviours`; Rollen-Label ueber `infer_ability_role_tags` (bestehend) |
| Waffenprofil | `weapon_info` (siehe Abschnitt 8) |
| Level-Kurve | Hero `level_info` |
| Kaufboni | Hero `purchase_bonuses` |

```
damage_plan(hero, cfg):
  wdps = weapon_dps(hero.weapon, cfg.combat_window_seconds)
  sdps = sum over core abilities of ability_dps(ability, hero, cfg)
  share = wdps / (wdps + sdps)
  primary = if share > 0.6 then Weapon else if share < 0.4 then Spirit else Hybrid
```

Der Damage-Plan entscheidet, ob der Kern Waffe, Spirit oder Hybrid ist. Das ersetzt
die drei starren Pfade des alten `engine::fits_path`; ein Held wie Warden kann so als
Waffen-Kern mit Spirit-Nebenwirkung erkannt werden statt in einen festen Pfad
gezwungen zu werden.

## 12. Skalierungsstufe als Angelpunkt

Quelle: Ability `upgrades[].property_upgrades[] = {bonus, name}` (verifiziertes
Format), Skill-Marker `upgrade_type: EAddToScale`.

```
scaling_step(ability):
  find upgrade i where property_upgrades[].name is a scaling stat
  return ScalingStep{ upgrade_index: i, stat, from, to }
```

Ein Upgrade, das die Skalierung aendert, ist der Angelpunkt: vor diesem Punkt ist
jedes Spirit-Item nur halb so viel wert, danach doppelt (Skill: Life Drain von
0.3225 auf 0.6225). Der Kaufplan richtet sich daran aus. Siehe Offene Frage 1 in
ARCHITEKTUR.md zur genauen Herkunft von `scale_function`/`EAddToScale` im aktuellen
Snapshot.

## 13. Patch-Delta

Quelle: `brain.patch_events` (32k) und `brain.patch_event_enrichments` (32k), je
Event Ziel (Held/Item/Ability) und Richtung. Ein Patch-Event wird zu einer
Verschiebung im Helden- oder Item-Modell:

```
compute_patch_delta(hero, events):
  for event in events:
    target    = classify_target(event)          # Hero | Item | Ability
    mechanic  = classify_mechanic(event)         # z.B. weapon_damage, cooldown, spirit_scaling
    sign      = direction(event)                 # +1 Buff, -1 Nerf
    magnitude = parsed_number(event)             # aus der Patchzeile, nicht geschaetzt
    yield PatchDelta{ target, mechanic, sign, magnitude, note }
```

`apply_patch_delta` verschiebt danach die betroffenen Item- und Ability-Werte vor dem
Scoring. Vorzeichen und Groesse kommen aus der geparsten Patchzeile; der Patch-Analyst
liefert nur den Klartext dazu, nie die Zahl. So wird "Warden ist wieder Meta"
ableitbar: ein Buff auf das Waffenprofil oder die Kern-Faehigkeit hebt den Waffen-DPS
im Damage-Plan, der Waffen-Kern steigt im Score ueber die Schwelle, und der
Patch-Wechsel-Backtest bestaetigt den Sprung gegenueber dem Vor-Patch-Stand.

## 14. Meta-Signale

Quelle und Gewichtung (Meta ist Nebensignal, nicht Ranking-Grundlage):

| Signal | Quelle | Gewicht | Mindeststichprobe |
|---|---|---|---|
| Verbreitung | `hero_item_stats.prevalence_builds` | mittel | `prevalence_builds >= min_prevalence_builds` |
| Winrate | `hero_item_stats.wins/losses/matches` | niedrig | `matches >= min_matches` |
| Lift | `hero_item_stats.lift_pp` | niedrig | vorhanden |
| Top-Autoren | `tierlist.hero_build_sources` (details) | hoch | Autor im watched-Set |
| Creator-Claims | Claims aus YouTube/Forum/Reddit | niedrig, nur Bestaetigung | genannt |
| Kaufzeitpunkt | `hero_item_stats.avg_buy_time_relative` | Phasenzuordnung | vorhanden |

```
meta_support(item):
  base = w_prev * norm(prevalence) + w_wr * norm(winrate) + w_lift * norm(lift)
  if author_hits > 0: base += w_author
  if claim_hits  > 0: base += w_claim
  return base if sample_ok(item) else damp(base)
```

Unter Mindeststichprobe wird das Signal gedaempft, nicht genullt, und die Confidence
des Items faellt auf `Low`. Meta hebt oder senkt einen mechanisch bereits plausiblen
Kandidaten, macht aber aus einem mechanisch schwachen Item keinen Kern.

## 15. Gleicher Feldname, verschiedene Bedeutung

Quelle: `description.desc` als einzige Wahrheit ueber die Bezugsgroesse (Skill 5d).
`SelfDamagePct` meint bei einem Item Prozent des Schadens, bei einem anderen Prozent
des aktuellen Lebens; `HealthToDamage`, `CurrentHealthDamage`, `MaxHealthDamage` sind
drei verschiedene Kurven. Regel: bei jedem Prozentfeld, dessen Bezugsgroesse nicht im
Namen steht, wird die Bezugsgroesse aus `description.desc` bestimmt; ist sie daraus
nicht entscheidbar, wird das Item mit sichtbarer Annahme und gesenkter Confidence
gefuehrt, nicht mit geratener Zahl.

## 16. Grenzen (was das Modell bewusst nicht rechnet)

- **Reichweite:** erhoeht keinen Schaden, verhindert nur Kanalabriss; haengt an der
  eigenen Abrissquote, fuer die es keine Daten gibt. Als offene Annahme benennen.
- **Lifesteal auf selbstheilende Faehigkeit:** ob der Effekt greift, steht nicht in
  den Daten; Annahme, kein Fakt, mit Messweg im Text.
- **Kanalanteil, Abrissquote, Fensterlaenge:** gesetzte Annahmen (`W = 40 s`,
  `channel_uptime = 0.55`), sichtbar im Report.
- **Kaufzeitpunkt der Autoren:** kein Zeitstempel je Item in `hero_build_sources`;
  Reihenfolge nur aus der Array-Position, Annahme im Backtest sichtbar.

Der Backtest muss dann zeigen: trifft der Reasoner-Kern trotz dieser Luecken den
Autoren-Kern (Kern-Ueberdeckung), liegt die Reihenfolge nah, und erkennt er den
patchbedingten Wechsel. Wo eine Grenze die Vorhersage traegt, macht der Backtest sie
sichtbar statt sie zu kaschieren.

## 17. Referenzfall Lightbringer x Situation Warden

Screenshot `referenz/lightbringer-warden.png`, ausgewertet: der Build ist ein
**Waffen-Kern** fuer Warden (id 25), kein Spirit-Build. Kern-Zeile:
High-Velocity Rounds, Opening Rounds, Extra Regen, Quicksilver Reload (Imbue),
Monster Rounds, Swift Striker, Titanic Magazine, Veil Walker, Enduring Speed,
Fleetfoot, Mercurial Magnum (Imbue), dann Spiritual Overflow, Siphon Bullets,
Boundless Spirit, Blood Tribute (Active), Unstoppable, Witchmail, Transcendent
Cooldown, Juggernaut. Situationsbloecke: "Can buy 1" (Metal Skin, Dispel Magic,
Spirit Resilience, Counterspell, Bullet Resilience, Vampiric Burst), "Tryhard"
(Slowing Hex), "Shields" (Spirit/Weapon Shielding, Reactive Barrier), "Optional".

Damit der Algo diesen Kern findet, braucht es:

1. **Damage-Plan mit Waffen-Erkennung (Abschnitt 11):** Wardens `weapon_info` plus
   die Waffen-Skalierung muss `weapon_share > 0.6` liefern, sonst zwingt der alte
   Spirit/weapon/tank-Pfad den Build in die falsche Richtung. Das ist der Kern-
   Grund, warum `engine::composite_score` ersetzt wird.
2. **Patch-Delta (Abschnitt 13):** der Nightshift-Patch muss als Buff auf Wardens
   Waffe oder Kern-Faehigkeit erkannt werden (`sign = +1`), damit der Waffen-Kern
   ueber die Schwelle steigt. Ohne Patch-Delta bleibt Warden "tot".
3. **Kaufbonus je Slot (Abschnitt 1):** die Weapon-Slot-Boni erklaeren, warum
   Monster Rounds, Swift Striker und Titanic Magazine trotz niedriger Winrate frueh
   kommen.
4. **Tickrate gegen Proc (Abschnitt 3):** Siphon Bullets und Blood Tribute leben von
   Schuss-Procs; ihr Wert steht und faellt mit `shots_per_second` gegen den
   Proc-Cooldown.
5. **Imbue-Zuordnung (Abschnitt 9):** Quicksilver Reload und Mercurial Magnum sind
   Imbue-Items; ihr Ziel muss auf Wardens Waffen- oder Kern-Faehigkeit fallen.
6. **Bedingte Abwertung (Abschnitt 4):** der "Can buy 1"-Block sind Resist-Actives
   mit Cooldown; sie gehoeren nicht in den Kern, sondern in einen Situationsblock,
   weil `condition_factor` sie unter Dauerlast abwertet.
7. **Spirit-Nebenwirkung als Hybrid (Abschnitt 11):** Spiritual Overflow, Boundless
   Spirit und Siphon Bullets zeigen, dass der Kern hybrid ist; der Damage-Plan muss
   Spirit als Nebenachse fuehren, nicht ausschliessen.

Backtest-Pflicht: der Reasoner-Kern fuer Warden muss die Waffen-Kernitems dieses
Builds mehrheitlich treffen, und der Patch-Wechsel-Test muss den Sprung von
"tot vor Nightshift" auf "Meta danach" als Vorzeichenwechsel im Waffen-Score zeigen.
Solange der Autoren-Scan tot ist, laeuft der Pflichtfall gegen den manuell
eingepflegten Referenz-Build (ARCHITEKTUR.md, Offene Frage 4).
