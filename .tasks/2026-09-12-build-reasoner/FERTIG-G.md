# FERTIG-G: Loader läuft für alle 38 Helden

Status: fertig
Branch: feat/build-reasoner-g
Commit: f10962c (fix(reasoner): load all hero snapshot abilities)
Push: origin/feat/build-reasoner-g

## Ursache mit Snapshot-Beleg

Der Loader hat in hero.rs ausschließlich ability.id gelesen. Bei den betroffenen
deadlock_assets_api/item_or_ability-Snapshots ist dieses Feld nicht vorhanden,
obwohl class_name die Ability eindeutig identifiziert:

| Held | Slot | Snapshot-class_name | id | ability_id | gefundene Top-Level-Schlüssel |
|---|---|---|---|---|---|
| Lady Geist | signature1 | ability_blood_bomb | <null> | <null> | ability_type, behaviour_bits, class_name, image, properties, start_trained, tooltip_details, type, update_time, upgrades, video, weapon_info |
| Infernus | signature1 | ability_incendiary_projectile | <null> | <null> | ability_type, behaviour_bits, class_name, image, properties, start_trained, tooltip_details, type, update_time, upgrades, video, weapon_info |
| Infernus | signature2 | ability_flame_dash | <null> | <null> | ability_type, behaviour_bits, class_name, image, properties, start_trained, tooltip_details, type, update_time, upgrades, video, weapon_info |

Damit wurde integer(ability.get("id")) zu 0, und position() erzeugte
"Ability 0 gehört nicht zum geladenen Helden". Die vollständigen Payloads tragen
in diesen Fällen class_name, properties und upgrades, aber keine numerische
Ability-ID.

Das Waffenproblem hatte dieselbe Snapshot-Ursache im zweiten Loaderpfad: zum
Beispiel Paradox/citadel_weapon_chrono_set liefert in weapon_info
bullet_damage=6.8, clip_size=40, cycle_time=0.294 und reload_duration=2.585,
aber kein shots_per_second beziehungsweise bullets_per_second. Der vorhandene
Parser ließ die Feuerrate deshalb bei 0 und meldete ein unvollständiges
Waffenprofil.

## Fix

- data.rs:45-66: numerische Ability-IDs werden aus id, ability_id, abilityId,
  external_id und item_id gelesen; ohne numerische ID wird class_name als stabile
  Ladeidentität verwendet.
- hero.rs:19-42: Enrichment matched zuerst die numerische ID und fällt bei
  fehlender ID auf class_name zurück. Ein echter Identitätsfehler nennt nun
  Held, Ability-Slot, Identität und alle gefundenen Schlüssel, ohne künstlich
  "Ability 0" zu melden.
- data.rs:451-469: fehlt die Feuerrate, wird sie aus 1 / cycle_time abgeleitet.
  Numerisch identifizierte Abilities werden in data.rs:481-486 vor class-only
  Einträgen priorisiert, damit kein imbue_target=0 nach außen gelangt.
  Nicht numerisch identifizierte Abilities werden nicht als falsche
  Patch-Snapshot-ID 0 veröffentlicht (data.rs:699-705).
- Fixtures in data.rs und hero.rs decken ID-Alias, class-only Payload,
  verständlichen Fehlertext, cycle_time und die Numeric-first-Priorisierung ab.

## 38-Helden-Nachweis

Vorher stammt aus dem Debug-Binary mit
reason build <Held> --no-ai --no-persist --json gegen den Central-Pool. Nachher
ist derselbe Lauf auf Commit f10962c.

| ID | Held | Vorher | Vorheriger Fehler | Nachher |
|---:|---|---:|---|---:|
| 1 | Infernus | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 2 | Seven | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 3 | Vindicta | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 4 | Lady Geist | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 6 | Abrams | 0 | OK | 0 |
| 7 | Wraith | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 8 | McGinnis | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 10 | Paradox | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 11 | Dynamo | 0 | OK | 0 |
| 12 | Kelvin | 0 | OK | 0 |
| 13 | Haze | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 14 | Holliday | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 15 | Bebop | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 16 | Calico | 0 | OK | 0 |
| 17 | Grey Talon | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 18 | Mo & Krill | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 19 | Shiv | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 20 | Ivy | 0 | OK | 0 |
| 25 | Warden | 0 | OK | 0 |
| 27 | Yamato | 0 | OK | 0 |
| 31 | Lash | 0 | OK | 0 |
| 35 | Viscous | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 50 | Pocket | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 52 | Mirage | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 58 | Vyper | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 60 | Sinclair | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 63 | Mina | 0 | OK | 0 |
| 64 | Drifter | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 65 | Venator | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 66 | Victor | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |
| 67 | Paige | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 69 | The Doorman | 0 | OK | 0 |
| 72 | Billy | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 76 | Graves | 0 | OK | 0 |
| 77 | Apollo | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 79 | Rem | 1 | Datenfehler: Waffenprofil unvollständig; Primärwaffen-Snapshot laden | 0 |
| 80 | Silver | 0 | OK | 0 |
| 81 | Celeste | 1 | Datenfehler: Ability 0 gehört nicht zum geladenen Helden | 0 |

Summe vorher: 12/38 Exit 0, 26/38 Exit 1.
Summe nachher: 38/38 Exit 0, 0/38 Exit 1.

## Tests

| Lauf | Ergebnis |
|---|---|
| Baseline vor Paket G | Workspace 273 bestanden, 58 ignoriert |
| cargo test -p dbrain-reasoner | 89 bestanden, 0 fehlgeschlagen, 16 ignoriert |
| cargo clippy -p dbrain-reasoner --all-targets -- -D warnings | Exit 0, keine Warnungen |
| cargo test --workspace | 277 bestanden, 0 fehlgeschlagen, 58 ignoriert |
| Reasoner --include-ignored mit Central read-only, ohne REASONER_SCRATCH_DSN | 96 bestanden, 9 erwartete Scratch-Fehler, Central-Tests grün |
| Debug-Binary, 38 Live-Helden | 38/38 Exit 0 |

Es gab keinen Schreibzugriff auf den Central-Pool.

