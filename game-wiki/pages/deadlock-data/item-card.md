---
title: "item card"
entries: 279
---

# item card

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_active_reload" title="Active Reload" -->

## Active Reload

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_active_reload`
- Snapshot ID: `40260`
- Source-Dokument: `7073`
- Kurzinfo: Active Reload aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Active Reload`
- Payload Hash: `05baebd7f6847dbf1f7b441789381ff56bf8c6826b606d1559d7ee2187bdfa21`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.283941+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "While reloading, pressing {g:citadel_binding:'Reload'} during the highlighted portion will <span class=\"highlight\">instantly finish your reload</span> and grant you <span class=\"highlight\">Fire Rate</span>, <span class=\"highlight\">Bullet Lifesteal</span> and <span class=\"highlight\">Move Speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": 12,
    "DescKey": "#upgrade_active_reload_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 16
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_active_reload",
  "Name": "Active Reload",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusClipSizePercent": 10,
    "BonusFireRate": 15,
    "BonusMoveSpeed": "3m",
    "BulletLifestealPercent": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "active reload",
      "name": "Active Reload",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aerial_supremacy" title="Aerial Supremacy" -->

## Aerial Supremacy

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aerial_supremacy`
- Snapshot ID: `40262`
- Source-Dokument: `7073`
- Kurzinfo: Aerial Supremacy aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Aerial Supremacy`
- Payload Hash: `4bdacc47838a046c10869da209f200c7e1b5d7db01a5aa2108c766d1fbcf6975`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.287304+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_superior_stamina"
  ],
  "Cost": 9999,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "Stamina",
        "Value": 7
      },
      {
        "Key": "AirMoveIncreasePercent",
        "Value": 70
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": 40
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_aerial_supremacy_passive",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aerial_supremacy",
  "Name": "Aerial Supremacy",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "AirMoveIncreasePercent": 30,
    "Stamina": 3,
    "StaminaCooldownReduction": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_thermal_detonator" title="Alchemical Fire" -->

## Alchemical Fire

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_thermal_detonator`
- Snapshot ID: `40484`
- Source-Dokument: `7073`
- Kurzinfo: Alchemical Fire aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Alchemical Fire`
- Payload Hash: `4e90bdeb4a0fd9fce29dd142b9062888b33b2975d91054e04b3c267eeac524ee`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.692030+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": "Throw a flask that explodes on contact, creating an area that does increasing {g:citadel_inline_attribute:'SpiritDamage'} <span class=\"highlight\">per second</span> and reduces enemy <span class=\"highlight\">Bullet Resist</span>.<br><br>50% less effective vs non-heroes.",
  "Info1": {
    "Alt": [
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DPSMax",
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Type": "tech_damage",
        "Value": 95.0
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "10m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30.0,
    "DescKey": "#upgrade_thermal_detonator_desc",
    "Main": [
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Type": "tech_damage",
        "Value": 45.0
      },
      {
        "Key": "BulletArmorReduction",
        "Scale": {
          "Type": "spirit",
          "Value": -0.055
        },
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -7.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_thermal_detonator",
  "Name": "Alchemical Fire",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DPSIncrease": {
      "Key": "DPSIncrease",
      "Scale": {
        "Type": "spirit",
        "Value": 0.04
      },
      "Type": "tech_damage",
      "Value": 7.0
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": 50
    },
    "NonHeroReductionPercent": {
      "Key": "NonHeroReductionPercent",
      "Value": 50
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BulletArmorReduction": -8,
    "DPS": 30,
    "DPSMax": 30,
    "SpiritPower": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "alchemical fire",
      "name": "Alchemical Fire",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ammo_scavenger" title="Ammo Scavenger" -->

## Ammo Scavenger

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ammo_scavenger`
- Snapshot ID: `40263`
- Source-Dokument: `7073`
- Kurzinfo: Ammo Scavenger aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ammo Scavenger`
- Payload Hash: `177e193836f81eefa04f661752dd2be163df5f4f96bb3841d32f27bcf93a8105`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.288909+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Any time you secure or deny a Soul from an entity you <span class=\"highlight\">get ammo back</span> and gain stacking <span class=\"highlight\">Spirit Power</span>.<br>At Max Stacks, gain bonus Sprint speed",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2m"
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "AmmoScavengerDuration",
        "Type": "duration",
        "Value": 45
      },
      {
        "Key": "MaxStacks",
        "Value": 18
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ammo_scavenger_desc",
    "Main": [
      {
        "Key": "AmmoPerSoul",
        "Value": 2
      },
      {
        "Key": "SpiritPowerPerSoul",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 1
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_ammo_scavenger",
  "Name": "Ammo Scavenger",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ancient_shield" title="Ancient Shield" -->

## Ancient Shield

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ancient_shield`
- Snapshot ID: `40264`
- Source-Dokument: `7073`
- Kurzinfo: Ancient Shield aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ancient Shield`
- Payload Hash: `4c1453a0e0ef03a255b8dfd833a0a946d2bd8aac8a77c689e7d6240f4f08d96a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.290474+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 40
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 40
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 40
      },
      {
        "Key": "DegenResistance",
        "Type": "healing",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 40
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 40
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_ancient_shield",
  "Name": "Ancient Shield",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BulletResist": 10,
    "DegenResistance": 10,
    "OutOfCombatHealthRegen": 10,
    "StatusResistancePercent": 10,
    "TechResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_apex_combat" title="Apex Combat" -->

## Apex Combat

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_apex_combat`
- Snapshot ID: `40269`
- Source-Dokument: `7073`
- Kurzinfo: Apex Combat aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Apex Combat`
- Payload Hash: `9446be826b3863dc9932bc9f8e0e749426c144c0d230faa0e7669c45cf4f13b5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.297945+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_ricochet"
  ],
  "Cost": 9999,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "RicochetTargetsTooltipOnly",
        "Value": 4
      },
      {
        "Key": "RicochetRadius",
        "Type": "distance",
        "Value": "13m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_apex_combat_desc",
    "Main": [
      {
        "Key": "RicochetDamagePercent",
        "Type": "bullet_damage",
        "Value": 65
      }
    ],
    "Type": null
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProcChance",
        "Value": 40
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_critshot_desc",
    "Main": [
      {
        "Key": "CritDamagePercent",
        "Type": "bullet_damage",
        "Value": 125
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_apex_combat",
  "Name": "Apex Combat",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 5,
  "Upgrades": {
    "CritDamagePercent": 30,
    "ProcChance": 5,
    "RicochetDamagePercent": 25
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_arcane_surge" title="Arcane Surge" -->

## Arcane Surge

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_surge`
- Snapshot ID: `40274`
- Source-Dokument: `7073`
- Kurzinfo: Arcane Surge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Arcane Surge`
- Payload Hash: `dc02a7ef1c33d0e1d4bb228c10b8f4447f3edc39b3d2e15055d0a982f2e196dc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.305798+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 1600,
  "Description": "After you <span class=\"highlight\">Dash-Jump</span>, the <span class=\"highlight\">next ability you use</span> within 7s will have bonus <span class=\"highlight\">Range, Duration,</span> and <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "Stamina",
        "Value": 1
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "ArcaneSurgeWindow",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_arcane_surge_desc",
    "Main": [
      {
        "Key": "TechRangeMultiplierBuff",
        "Type": "distance",
        "UsageFlags": "ConditionallyApplied",
        "Value": 12
      },
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "UsageFlags": "ConditionallyApplied",
        "Value": 15
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_arcane_surge",
  "Name": "Arcane Surge",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplierBuff": {
      "Key": "TechRadiusMultiplierBuff",
      "Type": "distance",
      "UsageFlags": "ConditionallyApplied",
      "Value": 12
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate",
    "ClipSize",
    "Movement",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusAbilityDurationPercent": 15,
    "SpiritPower": 25,
    "Stamina": 1,
    "StaminaCooldownReduction": 14,
    "TechRadiusMultiplierBuff": 15,
    "TechRangeMultiplierBuff": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "arcane surge",
      "name": "Arcane Surge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_arctic_blast" title="Arctic Blast" -->

## Arctic Blast

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arctic_blast`
- Snapshot ID: `40275`
- Source-Dokument: `7073`
- Kurzinfo: Arctic Blast aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Arctic Blast`
- Payload Hash: `35f19230b8fa7280074f2f84bcdbd78f7c21f624845a29389d127a7ce7317830`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.308076+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_cold_front"
  ],
  "Cost": 6400,
  "Description": "Release an expanding ice blast that deals {g:citadel_inline_attribute:'SpiritDamage'}, <span class=\"highlight\">Freezing</span> and then <span class=\"highlight\">Slowing</span> targets it hits.<br><br>Slowed targets have their <span class=\"highlight\">stamina regen frozen</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "EndRadius",
        "Type": "distance",
        "Value": "16m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 24.0,
    "DescKey": "#upgrade_arctic_blast_desc",
    "Main": [
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.6975
        },
        "Type": "tech_damage",
        "Value": 175.0
      },
      {
        "Key": "FreezeDuration",
        "Type": "duration",
        "Value": 1
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_arctic_blast",
  "Name": "Arctic Blast",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DamageHeight": {
      "Key": "DamageHeight",
      "Value": "7m"
    },
    "NPCDamageMult": {
      "Key": "NPCDamageMult",
      "Type": "tech_damage",
      "Value": 1
    },
    "SlowDuration": {
      "Key": "SlowDuration",
      "LocTokenOverride": "ArcticBlastSlowDuration",
      "Type": "duration",
      "Value": 4
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": 60
    },
    "SpreadDuration": {
      "Key": "SpreadDuration",
      "Type": "duration",
      "Value": 0.6
    },
    "StartRadius": {
      "Key": "StartRadius",
      "Type": "distance",
      "Value": "2m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -12,
    "Damage": 150,
    "FreezeDuration": 0.25,
    "TechResist": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "arctic blast",
      "name": "Arctic Blast",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aprounds" title="Armor Piercing Rounds" -->

## Armor Piercing Rounds

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aprounds`
- Snapshot ID: `40270`
- Source-Dokument: `7073`
- Kurzinfo: Armor Piercing Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Armor Piercing Rounds`
- Payload Hash: `20f507cef1b4f4bda3e56479a1bd878f82d3d39b0183cc562162623dc9f2f474`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.299686+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 6400,
  "Description": "Your Bullets have a chance to become unavoidable, <span class=\"highlight\">piercing through</span> enemies and <span class=\"highlight\">ignoring their Bullet Resistance</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Value": 60
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_aprounds_desc",
    "Main": [
      {
        "Key": "ProcChance",
        "Value": 55
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_aprounds",
  "Name": "Armor Piercing Rounds",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BaseAttackDamagePercent": 30,
    "BonusBulletSpeedPercent": 55,
    "ProcChance": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "armor piercing rounds",
      "name": "Armor Piercing Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_bulletshredimbue" title="Ballistic Enchantment" -->

## Ballistic Enchantment

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bulletshredimbue`
- Snapshot ID: `40292`
- Source-Dokument: `7073`
- Kurzinfo: Ballistic Enchantment aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ballistic Enchantment`
- Payload Hash: `3f6c9de048b8302d389efcf4cd41c612cf66142ccfa5b02f018e2b4e54002452`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.337826+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_reach"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with increased <span class=\"highlight\">range</span>. Dealing damage with that ability grants you increased <span class=\"highlight\">weapon damage</span> per <span class=\"highlight\">unique hero hit</span>. Has reduced effect on non-heroes.",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 14
      },
      {
        "Key": "WeaponPowerPerStackNonHero",
        "Value": 5
      },
      {
        "Key": "NonHeroStackLimit",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_bulletshredimbue_desc",
    "Main": [
      {
        "Key": "WeaponPowerPerStack",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 22
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_bulletshredimbue",
  "Name": "Ballistic Enchantment",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 22
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15,
    "WeaponPowerPerStack": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "ballistic enchantment",
      "name": "Ballistic Enchantment",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_regenerating_bullet_shield" title="Battle Vest" -->

## Battle Vest

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_regenerating_bullet_shield`
- Snapshot ID: `40432`
- Source-Dokument: `7073`
- Kurzinfo: Battle Vest aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Battle Vest`
- Payload Hash: `43e2d83db4d17fec71623c037f99686ab3eea0a84843b4168a9b4ab3bce00550`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.581905+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "While you are <span class=\"highlight\">above 65% health</span>, gain <span class=\"highlight\">{g:citadel_inline_attribute:'WeaponDamage'}</span> and <span class=\"highlight\">{g:citadel_inline_attribute:'BonusFireRate'}</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_regenerating_bullet_shield_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 18
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 7
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_regenerating_bullet_shield",
  "Name": "Battle Vest",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LifeThreshold": {
      "Key": "LifeThreshold",
      "Value": 65
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusFireRate": 8,
    "BulletResist": 12,
    "OutOfCombatHealthRegen": 3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "battle vest",
      "name": "Battle Vest",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_berserker" title="Berserker" -->

## Berserker

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_berserker`
- Snapshot ID: `40282`
- Source-Dokument: `7073`
- Kurzinfo: Berserker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Berserker`
- Payload Hash: `e5b5b3d25f5fd4dfa11ee44ff51a064b86910d796d592b055d5c74aa0514c1ff`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.320772+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Your <span class=\"highlight\">Weapon Damage</span> increases as you take sustained damage.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageToStack",
        "Type": "bullet_armor_up",
        "Value": 120
      },
      {
        "Key": "MaxStacks",
        "Value": 10
      },
      {
        "Key": "DamageDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_berserker_desc",
    "Main": [
      {
        "Key": "WeaponPowerPerStack",
        "Type": "bullet_damage",
        "Value": 7
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_berserker",
  "Name": "Berserker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BulletResist": 8,
    "MaxStacks": 8,
    "WeaponPowerPerStack": 3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "berserker",
      "name": "Berserker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_blood_tribute" title="Blood Tribute" -->

## Blood Tribute

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_blood_tribute`
- Snapshot ID: `40284`
- Source-Dokument: `7073`
- Kurzinfo: Blood Tribute aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blood Tribute`
- Payload Hash: `4aa5799a19f54ca0a87e1f249ff5e6f5c83678d13d7f3227493c7826066cc671`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.324804+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCastToggle",
  "Components": null,
  "Cost": 3200,
  "Description": "Toggle: Continually sacrifice Health to improve {g:citadel_inline_attribute:'FireRate'}, <span class=\"highlight\">Debuff Resistance</span> and <span class=\"highlight\">Move Speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "InnateStatusResistancePercent",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "TechResist",
        "Value": 8
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 35
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_blood_tribute_desc",
    "Main": [
      {
        "Key": "HealthDrainedPerSecond",
        "Type": "damage",
        "Value": 50
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_blood_tribute",
  "Name": "Blood Tribute",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusFireRate": 30,
    "HealthDrainedPerSecond": -20,
    "OutOfCombatHealthRegen": 8,
    "TechResist": 14
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "blood tribute",
      "name": "Blood Tribute",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_boundless_spirit" title="Boundless Spirit" -->

## Boundless Spirit

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_boundless_spirit`
- Snapshot ID: `40286`
- Source-Dokument: `7073`
- Kurzinfo: Boundless Spirit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Boundless Spirit`
- Payload Hash: `0ccb3d4e93c6940f7a233601198d9478379f11fd6b30ad95399a23d31bd2de27`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.327940+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_soaring_spirit"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechPower",
        "Value": 30
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechPowerPercent",
        "Value": 15
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_boundless_spirit",
  "Name": "Boundless Spirit",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusHealth": 100,
    "OutOfCombatHealthRegen": 4,
    "TechPower": 25,
    "TechPowerPercent": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "boundless spirit",
      "name": "Boundless Spirit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_bullet_armor" title="Bullet Armor" -->

## Bullet Armor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_armor`
- Snapshot ID: `40288`
- Source-Dokument: `7073`
- Kurzinfo: Bullet Armor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Armor`
- Payload Hash: `9a00ee08c87b42e5a49541a00d76cc1fff2e6eb9c7778c11e92bc786e14a421f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.331686+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_bullet_armor",
  "Name": "Bullet Armor",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_vampire" title="Bullet Lifesteal" -->

## Bullet Lifesteal

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_vampire`
- Snapshot ID: `40494`
- Source-Dokument: `7073`
- Kurzinfo: Bullet Lifesteal aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Lifesteal`
- Payload Hash: `2115a849172f33e3eb78ed28f58c11f9938afb556bb84616fe0c3c8439df8a8b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.710026+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 13
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_vampire",
  "Name": "Bullet Lifesteal",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusHealth": 120,
    "BulletLifestealPercent": 16
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "bullet lifesteal",
      "name": "Bullet Lifesteal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_improved_bullet_armor" title="Bullet Resilience" -->

## Bullet Resilience

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_bullet_armor`
- Snapshot ID: `40379`
- Source-Dokument: `7073`
- Kurzinfo: Bullet Resilience aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Resilience`
- Payload Hash: `57f82de372b3d706f9132caaf4e4d303b22295d2a351d9f923c2e989b91fc1d9`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.492622+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "When below <span class=\"highlight\">50% health</span>, gain additional <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_improved_bullet_armor_desc",
    "Main": [
      {
        "Key": "HealthThreshold",
        "Value": 50
      },
      {
        "Key": "BulletResistBelowThreshold",
        "LocTokenOverride": "#BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_bullet_armor",
  "Name": "Bullet Resilience",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BulletResist": 10,
    "BulletResistBelowThreshold": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "bullet resilience",
      "name": "Bullet Resilience",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_bullet_resist_shredder" title="Bullet Resist Shredder" -->

## Bullet Resist Shredder

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_resist_shredder`
- Snapshot ID: `40291`
- Source-Dokument: `7073`
- Kurzinfo: Bullet Resist Shredder aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Resist Shredder`
- Payload Hash: `849ac7ae40dfe0813e0883e5815bb359b6902a62ef464e1b13602e5447a8a7c1`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.335957+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Reduces <span class=\"highlight\">Bullet Resist</span> on enemies when you deal {g:citadel_inline_attribute:'SpiritDamage'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 9
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 9
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_bullet_resist_shredder_desc",
    "Main": [
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_bullet_resist_shredder",
  "Name": "Bullet Resist Shredder",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BulletArmorReduction": -11,
    "BulletResist": 7
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "bullet resist shredder",
      "name": "Bullet Resist Shredder",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_burst_fire" title="Burst Fire" -->

## Burst Fire

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_burst_fire`
- Snapshot ID: `40293`
- Source-Dokument: `7073`
- Kurzinfo: Burst Fire aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Burst Fire`
- Payload Hash: `726ffb34db24e0e6960dfdbc79d18c3eac7f556919172ec70cfa769036e3e9ab`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.339445+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_rapid_rounds"
  ],
  "Cost": 3200,
  "Description": "Briefly gain <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Move Speed</span> when one of your bullets hits an enemy hero.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SlideScale",
        "Type": "movement_speed",
        "Value": 50
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 10
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 9,
    "DescKey": "#upgrade_burst_fire_desc",
    "Main": [
      {
        "Key": "ActivatedFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 32
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.25m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_burst_fire",
  "Name": "Burst Fire",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -1,
    "ActivatedFireRate": 15,
    "BonusFireRate": 14,
    "BonusMoveSpeed": "1.5m",
    "SlideScale": 50
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "burst fire",
      "name": "Burst Fire",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_capacitor" title="Capacitor" -->

## Capacitor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_capacitor`
- Snapshot ID: `40296`
- Source-Dokument: `7073`
- Kurzinfo: Capacitor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Capacitor`
- Payload Hash: `31d987a0e117b37641ba8e7d252828a4b2b633d235fc758d02a1b5fa4e91b53d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.343452+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_chain_lightning"
  ],
  "Cost": 6400,
  "Description": "Launch a projectile that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span>, applies a strong slow that recovers over time, <span class=\"highlight\">prevents Stamina usage</span> and <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 5
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ChainCount",
        "Value": 6
      },
      {
        "Key": "ChainRadius",
        "Type": "distance",
        "Value": "10m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.2,
    "DescKey": "#upgrade_chain_lightning_desc",
    "Main": [
      {
        "Key": "DamagePerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.19
        },
        "Type": "tech_damage",
        "Value": 43.0
      },
      {
        "Key": "ProcChance",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 40,
    "DescKey": "#upgrade_capacitor_desc",
    "Main": [
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 100
      },
      {
        "Key": "MaxSlowPercent",
        "Type": "slow",
        "Value": 75
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_capacitor",
  "Name": "Capacitor",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusPerChain": {
      "Key": "BonusPerChain",
      "Scale": {
        "Type": "spirit",
        "Value": 0.19
      },
      "Type": "tech_damage",
      "Value": 43.0
    },
    "ChainTickRate": {
      "Key": "ChainTickRate",
      "Value": 0.4
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "BossEnemy",
    "MinionEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -32,
    "BonusFireRate": 15,
    "Damage": 25,
    "DamagePerChain": 25,
    "ProcChance": 5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "capacitor",
      "name": "Capacitor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_celestial_guidance" title="Celestial Blessing" -->

## Celestial Blessing

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_celestial_guidance`
- Snapshot ID: `40298`
- Source-Dokument: `7073`
- Kurzinfo: Celestial Blessing aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Celestial Blessing`
- Payload Hash: `0bdb82ffcbd4aa23e3892b120a2d143f651f0d8cd71ddc79b6a15cf0c19b5fab`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.347154+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "Applies an extremely powerful cleanse that replenishes your allies globally.",
  "Info1": {
    "Alt": [
      {
        "Key": "MinHeal",
        "Type": "healing",
        "Value": 400
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30,
    "DescKey": "#upgrade_celestial_guidance_desc",
    "Main": [
      {
        "Key": "HealPercentAmount",
        "Type": "healing",
        "Value": 60
      },
      {
        "Key": "StaminaCooldownReduction",
        "UsageFlags": "ConditionallyApplied",
        "Value": 100
      },
      {
        "Key": "BuffMoveSpeedBonus",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "5m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_celestial_guidance",
  "Name": "Celestial Blessing",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "999m"
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -20,
    "BuffDuration": 2,
    "MinHeal": 200
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "celestial blessing",
      "name": "Celestial Blessing",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cheat_death" title="Cheat Death" -->

## Cheat Death

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cheat_death`
- Snapshot ID: `40302`
- Source-Dokument: `7073`
- Kurzinfo: Cheat Death aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cheat Death`
- Payload Hash: `0a1d70b7f79e6e9c72800587b2751d2db656de2d998d068afc97123f431606c2`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.353554+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 200
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DeathImmunityDamageReduction",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -60
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "UsageFlags": "ConditionallyApplied",
        "Value": -60
      }
    ],
    "ChargeUp": null,
    "Cooldown": 90.0,
    "DescKey": "#upgrade_cheat_death_unkillable_passive",
    "Main": [
      {
        "Key": "DeathImmunityDuration",
        "Type": "duration",
        "Value": 4.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cheat_death",
  "Name": "Cheat Death",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusMoveSpeed": {
      "Key": "BonusMoveSpeed",
      "Type": "move_speed",
      "UsageFlags": "ConditionallyApplied",
      "Value": "0m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": -60
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -20,
    "DeathImmunityDamageReduction": 90,
    "DeathImmunityDuration": 0.5,
    "HealAmpReceivePenaltyPercent": 90,
    "HealAmpRegenPenaltyPercent": 90
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "cheat death",
      "name": "Cheat Death",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cloak_of_opportunity" title="Cloak of Opportunity" -->

## Cloak of Opportunity

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloak_of_opportunity`
- Snapshot ID: `40309`
- Source-Dokument: `7073`
- Kurzinfo: Cloak of Opportunity aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cloak of Opportunity`
- Payload Hash: `c6a5b20eaf95ae8a3e3f1c04cb3675a41339b6fd3fb195831c828d9c83937b68`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.364128+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Block the next debuff that would apply <span class=\"highlight\">movement lock, Stun, Chained, Immobilize, or Sleep</span> and become <span class=\"highlight\">Unstoppable</span>. Also gain a <span class=\"highlight\">Barrier</span> and bonus {g:citadel_inline_attribute:'MoveSpeed'}",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusImmuneDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 12.0,
    "DescKey": "#upgrade_cloak_of_opportunity_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 500
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cloak_of_opportunity",
  "Name": "Cloak of Opportunity",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -4,
    "BonusMoveSpeed": "4m",
    "CombatBarrier": 300
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "cloak of opportunity",
      "name": "Cloak of Opportunity",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_close_range" title="Close Quarters" -->

## Close Quarters

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_close_range`
- Snapshot ID: `40313`
- Source-Dokument: `7073`
- Kurzinfo: Close Quarters aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Close Quarters`
- Payload Hash: `a9ba7b8a0b4fc873b9e89ac50920a2959818bfd53b6624d5d594242855b4194f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.370912+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when in <span class=\"highlight\">close range</span> to your target.",
  "Info1": {
    "Alt": [
      {
        "Key": "MeleeResistPercent",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "CloseRangeBonusDamageRange",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_close_range_desc",
    "Main": [
      {
        "Key": "CloseRangeBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_close_range",
  "Name": "Close Quarters",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "CloseRangeBonusWeaponPower": 15,
    "MeleeResistPercent": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "close quarters",
      "name": "Close Quarters",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cold_front" title="Cold Front" -->

## Cold Front

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cold_front`
- Snapshot ID: `40314`
- Source-Dokument: `7073`
- Kurzinfo: Cold Front aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cold Front`
- Payload Hash: `f4dc4a9353b423dc090636574a4b7c5d4ab7269d5c75af76fc55712247eb29e0`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.372743+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": "Release an expanding ice blast that deals {g:citadel_inline_attribute:'SpiritDamage'} and <span class=\"highlight\">Slows</span> targets it hits.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "EndRadius",
        "Type": "distance",
        "Value": "10m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25.0,
    "DescKey": "#upgrade_cold_front_desc",
    "Main": [
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.465
        },
        "Type": "tech_damage",
        "Value": 95.0
      },
      {
        "Key": "MovementSpeedSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 60
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cold_front",
  "Name": "Cold Front",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DamageHeight": {
      "Key": "DamageHeight",
      "Value": "7m"
    },
    "NPCDamageMult": {
      "Key": "NPCDamageMult",
      "Type": "tech_damage",
      "Value": 1
    },
    "SpreadDuration": {
      "Key": "SpreadDuration",
      "Type": "duration",
      "Value": 0.6
    },
    "StartRadius": {
      "Key": "StartRadius",
      "Type": "distance",
      "Value": "2m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -13,
    "Damage": 60,
    "TechResist": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "cold front",
      "name": "Cold Front",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_colossus" title="Colossus" -->

## Colossus

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_colossus`
- Snapshot ID: `40315`
- Source-Dokument: `7073`
- Kurzinfo: Colossus aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Colossus`
- Payload Hash: `6a506dca26e2b44b2c13b634ceacd296d00d3961d3a53029222f11996f24453f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.375231+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_health"
  ],
  "Cost": 6400,
  "Description": "Grow <span class=\"highlight\">larger in size</span>, gaining {g:citadel_inline_attribute:'BulletResist'}, {g:citadel_inline_attribute:'SpiritResist'}, and {g:citadel_inline_attribute:'MeleeDamage'}. <br><br>Nearby enemies suffer from {g:citadel_inline_attribute:'Slow'} and have reduced <span class=\"highlight\">dash speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusBaseHealth",
        "Type": "health",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "14m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      },
      {
        "Key": "ModelScaleGrowthTooltip",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": 37.0,
    "DescKey": "#upgrade_colossus_desc",
    "Main": [
      {
        "Key": "BuffBulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "BuffTechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 30
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_colossus",
  "Name": "Colossus",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "GroundDashReductionPercent": {
      "Key": "GroundDashReductionPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": -25
    },
    "ModelScaleGrowth": {
      "Key": "ModelScaleGrowth",
      "Value": 1.2
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -7,
    "BonusBaseHealth": 15,
    "BuffBulletResist": 10,
    "BuffTechResist": 10,
    "ModelScaleGrowth": 0.2,
    "ModelScaleGrowthTooltip": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "colossus",
      "name": "Colossus",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_tempo" title="Compress Cooldown" -->

## Compress Cooldown

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_tempo`
- Snapshot ID: `40401`
- Source-Dokument: `7073`
- Kurzinfo: Compress Cooldown aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Compress Cooldown`
- Payload Hash: `aeb4e78661fc18d13e8940cf299ffe28ac810edae55e060185a64a5159dd1567`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.529191+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Imbue an ability to reduce its <span class=\"highlight\">Cooldown</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_tempo_desc",
    "Main": [
      {
        "Key": "CooldownReduction",
        "LocTokenOverride": "SingleAbilityCooldownReduction",
        "Type": "cooldown",
        "Value": 18
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_tempo",
  "Name": "Compress Cooldown",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "CooldownReduction": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "compress cooldown",
      "name": "Compress Cooldown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_missile" title="Conjure Missiles" -->

## Conjure Missiles

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_missile`
- Snapshot ID: `40395`
- Source-Dokument: `7073`
- Kurzinfo: Conjure Missiles aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Conjure Missiles`
- Payload Hash: `fadc347768b9c7f571e9468533c1c932f3db9d17a49c87152ae470d531657fed`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.518815+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 70
      },
      {
        "Key": "BonusFireRate",
        "Value": 10
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "1m"
      },
      {
        "Key": "MaxHealthLossPercent",
        "Type": "health",
        "Value": -15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_glass_cannon_desc",
    "Main": [
      {
        "Key": "WeaponPowerPerKill",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_magic_missile",
  "Name": "Conjure Missiles",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusClipPerKill": {
      "Key": "BonusClipPerKill",
      "Value": 2
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxClipBonus": {
      "Key": "MaxClipBonus",
      "Value": 14
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 5
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_counterspell" title="Counterspell" -->

## Counterspell

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_counterspell`
- Snapshot ID: `40319`
- Source-Dokument: `7073`
- Kurzinfo: Counterspell aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Counterspell`
- Payload Hash: `7d1f4f6c5656db2367c36c1b1be7936bbf53b0e6094557bf8660055c8332b803`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.383622+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Your next parry <span class=\"highlight\">protects you from the damage and effects of enemy abilities and items</span>. On a successful spell parry {g:citadel_inline_attribute:'Heal'} and gain {g:citadel_inline_attribute:'MoveSpeed'} and {g:citadel_inline_attribute:'Spirit'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      },
      {
        "Key": "SpiritPowerInnate",
        "Type": "tech_damage",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "SpellParryDuration",
        "Type": "duration",
        "Value": 0.8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 23.0,
    "DescKey": "#upgrade_counterspell_desc",
    "Main": [
      {
        "Key": "HealOnSuccess",
        "LocTokenOverride": "HealOnCounterSpell",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 150
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_power",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.75m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_counterspell",
  "Name": "Counterspell",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 150,
    "BonusMoveSpeed": "2m",
    "HealOnSuccess": 250,
    "SpiritPower": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "counterspell",
      "name": "Counterspell",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_banshee_slugs" title="Crippling Headshot" -->

## Crippling Headshot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_banshee_slugs`
- Snapshot ID: `40279`
- Source-Dokument: `7073`
- Kurzinfo: Crippling Headshot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crippling Headshot`
- Payload Hash: `f5671628d76c047b433aac8d38d4e426e44f6fa17f6db0ab8f343bf31217b6cf`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.315853+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_headshot_booster2"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_banshee_slugs_headshots_desc",
    "Main": [
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -16
      },
      {
        "Key": "MagicResistReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -16
      },
      {
        "Key": "HealAmpRegenPenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -35
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_banshee_slugs",
  "Name": "Crippling Headshot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DiminishingMultiplier": {
      "Key": "DiminishingMultiplier",
      "Value": 0.5
    },
    "HealAmpReceivePenaltyPercent": {
      "Key": "HealAmpReceivePenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -35
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusHealth": 150,
    "BulletResistReduction": -12,
    "HealAmpReceivePenaltyPercent": -25,
    "HealAmpRegenPenaltyPercent": -25,
    "MagicResistReduction": -12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "crippling headshot",
      "name": "Crippling Headshot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_crushing_fists" title="Crushing Fists" -->

## Crushing Fists

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_crushing_fists`
- Snapshot ID: `40322`
- Source-Dokument: `7073`
- Kurzinfo: Crushing Fists aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crushing Fists`
- Payload Hash: `a84320f44975b69dba26c04d82535ce946b8705ad451409a5a17d5815d30f830`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.390616+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_melee_charge"
  ],
  "Cost": 6400,
  "Description": "Your <span class=\"highlight\">{g:citadel_inline_attribute:'MeleeDamage'}</span> will <span class=\"highlight\">restore ammo</span> and apply a <span class=\"highlight\">stacking bullet resist debuff</span> on enemies. Heavy melee applies 2 stacks. <br><br>If the target reaches max stacks, they will be <span class=\"highlight\">stunned</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 22
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "MeleeDistanceScale",
        "Value": 60
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 5,
    "DescKey": "#upgrade_melee_charge_desc",
    "Main": [
      {
        "Key": "BonusHeavyMeleeDamage",
        "Type": "melee_damage",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "StunDuration",
        "Value": 0.5
      },
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_crushing_fists_desc",
    "Main": [
      {
        "Key": "LightMeleeAmmo",
        "Type": "clipsize",
        "Value": 15
      },
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -4
      },
      {
        "Key": "MaxStacks",
        "Value": 6
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crushing_fists",
  "Name": "Crushing Fists",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeavyMeleeMultiplier": {
      "Key": "HeavyMeleeMultiplier",
      "Value": 2
    },
    "LightMeleeStacks": {
      "Key": "LightMeleeStacks",
      "Value": 1
    }
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BonusMeleeDamagePercent": 15,
    "BulletResist": 12,
    "BulletResistReduction": -4,
    "MeleeDistanceScale": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "crushing fists",
      "name": "Crushing Fists",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_non_player_bonus_sacrifice" title="Cultist Sacrifice" -->

## Cultist Sacrifice

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_non_player_bonus_sacrifice`
- Snapshot ID: `40413`
- Source-Dokument: `7073`
- Kurzinfo: Cultist Sacrifice aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cultist Sacrifice`
- Payload Hash: `ef373bc5e9dbe7e86881384b718fc8143a8e2d1cf554659cc985cf71f1a2029f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.549725+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_non_player_bonus"
  ],
  "Cost": 3200,
  "Description": "Target an enemy NPC and consume it for <span class=\"highlight\">180% Bonus Souls</span> and grants a powerful long lasting buff.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      },
      {
        "Key": "NonPlayerBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "NonPlayerBulletResist",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 160
      }
    ],
    "ChargeUp": null,
    "Cooldown": 270,
    "DescKey": "#upgrade_non_player_bonus_sacrifice_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.8
        },
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10.0
      },
      {
        "Key": "BonusHealth",
        "Scale": {
          "Type": "power_increase",
          "Value": 4.0
        },
        "Type": "health",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50.0
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "UsageFlags": "ConditionallyApplied",
        "Value": 12
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_non_player_bonus_sacrifice",
  "Name": "Cultist Sacrifice",
  "Other": {
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "7m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusAbilityCharges": {
      "Key": "BonusAbilityCharges",
      "Type": "cast",
      "UsageFlags": "ConditionallyApplied",
      "Value": 1
    },
    "BonusSoulsPct": {
      "Key": "BonusSoulsPct",
      "Value": 180
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "UsageFlags": "ConditionallyApplied",
      "Value": 12
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "TrooperEnemy",
    "Neutral",
    "MinionEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 47,
    "BonusHealth": 300,
    "NonPlayerBonusWeaponPower": 30,
    "NonPlayerBulletResist": 30,
    "TechRadiusMultiplier": 40,
    "TechRangeMultiplier": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "cultist sacrifice",
      "name": "Cultist Sacrifice",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_glitch" title="Cursed Relic" -->

## Cursed Relic

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_glitch`
- Snapshot ID: `40353`
- Source-Dokument: `7073`
- Kurzinfo: Cursed Relic aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cursed Relic`
- Payload Hash: `ee8a912184cae44c59a8d225ecdafb1760222b69b950ac8aa17faccd350e12db`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.447341+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": "Curses an enemy - <span class=\"highlight\">interrupting, Silencing, Disarming</span>, and <span class=\"highlight\">preventing item usage</span>. <span class=\"highlight\">Removes all non-ultimate buffs</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -14
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 3.25
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 55.0,
    "DescKey": "#upgrade_glitch_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glitch",
  "Name": "Cursed Relic",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SkipFrames": {
      "Key": "SkipFrames",
      "UsageFlags": "ConditionallyApplied",
      "Value": 6
    }
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -40,
    "AbilityDuration": 0.25
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "cursed relic",
      "name": "Cursed Relic",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_debuff_reducer" title="Debuff Reducer" -->

## Debuff Reducer

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_debuff_reducer`
- Snapshot ID: `40324`
- Source-Dokument: `7073`
- Kurzinfo: Debuff Reducer aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Debuff Reducer`
- Payload Hash: `d4e41e1227b4d0591e0e154ee3fc1f432c69bb49d926ffe4f0402203030b0c16`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.394218+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Reduces the <span class=\"highlight\">duration</span> of all negative effects applied to you.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_debuff_reducer_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_debuff_reducer",
  "Name": "Debuff Reducer",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "StatusResistancePercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "debuff reducer",
      "name": "Debuff Reducer",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rupture" title="Decay" -->

## Decay

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rupture`
- Snapshot ID: `40445`
- Source-Dokument: `7073`
- Kurzinfo: Decay aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Decay`
- Payload Hash: `96d232248818a54bbb59f25b2a07b1aa394175095112a66d315bf71908662032`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.604127+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": "Inflict <span class=\"highlight\">damage over time</span> to a target, dealing damage based on their current health.<br>Decay's damage is non-lethal and does not apply item procs.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 8
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 65
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Type": "range",
        "Value": "20m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30.0,
    "DescKey": "#upgrade_rupture_desc",
    "Main": [
      {
        "Key": "DotHealthPercent",
        "Scale": {
          "Type": "spirit",
          "Value": 0.004
        },
        "Type": "tech_damage",
        "Value": 2.6
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -50
      },
      {
        "Key": "AbilityCastRange",
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Type": "range",
        "Value": "20m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rupture",
  "Name": "Decay",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -50
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 1.0
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 90,
    "DotHealthPercent": 0.5,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "TechPower": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "decay",
      "name": "Decay",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_greater_withering_whip" title="Disarming Hex" -->

## Disarming Hex

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_greater_withering_whip`
- Snapshot ID: `40355`
- Source-Dokument: `7073`
- Kurzinfo: Disarming Hex aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Disarming Hex`
- Payload Hash: `19d0b7c9fa92dcc9d2143e03e826f2845e3242e30cd42070f7ba68edf78c8ce3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.450737+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_withering_whip"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Disarms</span> enemy target and reduces their <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "32m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.25
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_greater_withering_whip_desc",
    "Main": [
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      },
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -13
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_greater_withering_whip",
  "Name": "Disarming Hex",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 175,
    "BulletArmorReduction": -7
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "disarming hex",
      "name": "Disarming Hex",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_reduce_debuff_duration" title="Dispel Magic" -->

## Dispel Magic

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_reduce_debuff_duration`
- Snapshot ID: `40431`
- Source-Dokument: `7073`
- Kurzinfo: Dispel Magic aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dispel Magic`
- Payload Hash: `c4e6ff053bb37730e1281bdef3710e4b5250be409bf3710bb4697633caaa857a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.580198+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ActiveBonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2m"
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45.0,
    "DescKey": "#upgrade_reduce_debuff_duration_active_desc",
    "Main": [
      {
        "Key": "HealOnActivate",
        "Type": "healing",
        "Value": 250
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_reduce_debuff_duration",
  "Name": "Dispel Magic",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -25,
    "HealOnActivate": 150,
    "TechResist": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "dispel magic",
      "name": "Dispel Magic",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_divine_barrier" title="Divine Barrier" -->

## Divine Barrier

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_divine_barrier`
- Snapshot ID: `40328`
- Source-Dokument: `7073`
- Kurzinfo: Divine Barrier aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Divine Barrier`
- Payload Hash: `a84009c2b47fef7a6ccbaa951df847b8c4e5bb6ae6adc3d8dfa89d2e86255a25`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.401669+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_guardian_ward"
  ],
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Remove all non-stun debuffs</span> from the target and provide them with a <span class=\"highlight\">Barrier</span> and <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast. Cooldown is reduced by half when cast on someone else.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 10
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_divine_barrier_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 600
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2.75m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_divine_barrier",
  "Name": "Divine Barrier",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownReductionPctOnOthers": {
      "Key": "CooldownReductionPctOnOthers",
      "Value": 50
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 10
    }
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -27,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "divine barrier",
      "name": "Divine Barrier",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_diviners_kevlar" title="Diviner's Kevlar" -->

## Diviner's Kevlar

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_diviners_kevlar`
- Snapshot ID: `40329`
- Source-Dokument: `7073`
- Kurzinfo: Diviner's Kevlar aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Diviner's Kevlar`
- Payload Hash: `209d8f4c011cf0b2589a8f85aafac0f7aba3c2c3ff353cea44c3c826d52e0a82`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.403662+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Upon casting an <span class=\"highlight\">ultimate ability</span> gain a <span class=\"highlight\">Barrier</span> and temporary <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": 40.0,
    "DescKey": "#upgrade_diviners_kevlar_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 1000
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_diviners_kevlar",
  "Name": "Diviner's Kevlar",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -14,
    "BonusAbilityDurationPercent": 15,
    "CombatBarrier": 500,
    "TechPower": 55
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "diviner's kevlar",
      "name": "Diviner's Kevlar",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_arcane_extension" title="Duration Extender" -->

## Duration Extender

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_extension`
- Snapshot ID: `40272`
- Source-Dokument: `7073`
- Kurzinfo: Duration Extender aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Duration Extender`
- Payload Hash: `f711b813be48387e96039324fbe2a5aa59f21e86db95ba04c86e22e6b7454287`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.302890+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Imbue an ability to increase its <span class=\"highlight\">Duration</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_arcane_extension_desc",
    "Main": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 22
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_arcane_extension",
  "Name": "Duration Extender",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusAbilityDurationPercent": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "duration extender",
      "name": "Duration Extender",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ability_power_shard" title="Echo Shard" -->

## Echo Shard

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ability_power_shard`
- Snapshot ID: `40255`
- Source-Dokument: `7073`
- Kurzinfo: Echo Shard aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Echo Shard`
- Payload Hash: `970794d6692760659a1d0812c34811c1f33ef3a81bf57f908112de99f106c69b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.275458+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 5
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 5
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_ability_power_shard_active",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_ability_power_shard",
  "Name": "Echo Shard",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Movement",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -10,
    "BonusFireRate": 5,
    "BulletResist": 5,
    "TechRadiusMultiplier": 5,
    "TechRangeMultiplier": 5,
    "TechResist": 5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "echo shard",
      "name": "Echo Shard",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_electric_slippers" title="Electric Slippers" -->

## Electric Slippers

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_electric_slippers`
- Snapshot ID: `40335`
- Source-Dokument: `7073`
- Kurzinfo: Electric Slippers aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Electric Slippers`
- Payload Hash: `1e6015df0d3cda2fa8fb885dd8f856b9198ae22a2b5fabd39a920645a2c7bbd8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.414465+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "When sliding, evade bullets, gain fire rate and deal damage to enemies around you. <span class=\"diminish\"><br><br>Cooldown is per target.</span>",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "Stamina",
        "Value": 2
      },
      {
        "Key": "SlideScale",
        "Type": "movement_speed",
        "Value": 80
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "12m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.3,
    "DescKey": "#upgrade_electric_slippers_desc",
    "Main": [
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 100
      },
      {
        "Key": "EvasionWhileSliding",
        "UsageFlags": "ConditionallyApplied",
        "Value": 60
      },
      {
        "Key": "FireRateWhileSliding",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 60
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_electric_slippers",
  "Name": "Electric Slippers",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SlideTurnScale": {
      "Key": "SlideTurnScale",
      "Type": "movement_speed",
      "Value": 100
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "Damage": 50,
    "EvasionWhileSliding": 15,
    "FireRateWhileSliding": 20,
    "Stamina": 2
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "electric slippers",
      "name": "Electric Slippers",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_shield" title="Enchanter's Emblem" -->

## Enchanter's Emblem

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_shield`
- Snapshot ID: `40397`
- Source-Dokument: `7073`
- Kurzinfo: Enchanter's Emblem aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enchanter's Emblem`
- Payload Hash: `e518530e2850e83f6311a42ddab7b8c172c6f03687224b6a2e7332cafc4904f2`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.522188+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "While you are <span class=\"highlight\">above 65% health</span>, gain bonus <span class=\"highlight\">{g:citadel_inline_attribute:'Spirit'}</span> and <span class=\"highlight\">Cooldown Reduction</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechResist",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_shield_desc",
    "Main": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 15
      },
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_shield",
  "Name": "Enchanter's Emblem",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LifeThreshold": {
      "Key": "LifeThreshold",
      "Value": 65
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "CooldownReduction": 7,
    "OutOfCombatHealthRegen": 3,
    "TechPower": 15,
    "TechResist": 13
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "enchanter's emblem",
      "name": "Enchanter's Emblem",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_infinitemagazine" title="Endless Magazine" -->

## Endless Magazine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_infinitemagazine`
- Snapshot ID: `40383`
- Source-Dokument: `7073`
- Kurzinfo: Endless Magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Endless Magazine`
- Payload Hash: `641e6b08b28fea4f133ffaf2f75173e3b3ba77db741743893969f44e022d4af1`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.499873+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "Gain <span class=\"highlight\">infinite ammo</span> and {g:citadel_inline_attribute:'BonusFireRate'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 100
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Value": 20
      },
      {
        "Key": "BonusHealth",
        "Value": 200
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 40,
    "DescKey": "#upgrade_infinitemagazine_desc",
    "Main": [
      {
        "Key": "StatusEffectInfiniteClip",
        "Value": null
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_infinitemagazine",
  "Name": "Endless Magazine",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "FireRate",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cardio_calibrator" title="Enduring Speed" -->

## Enduring Speed

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cardio_calibrator`
- Snapshot ID: `40297`
- Source-Dokument: `7073`
- Kurzinfo: Enduring Speed aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enduring Speed`
- Payload Hash: `f0b65334adba8684539e577a6e6f1cb69081fdda96dbc4ea692fcf0b88535cf4`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.345351+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 1600,
  "Description": "Reduces the effect of enemy <span class=\"highlight\">Move Speed</span> penalties.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_cardio_calibrator_desc",
    "Main": [
      {
        "Key": "SlowResistancePercent",
        "Type": "move_speed",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cardio_calibrator",
  "Name": "Enduring Speed",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Movement",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusMoveSpeed": "2m",
    "OutOfCombatHealthRegen": 8,
    "SlowResistancePercent": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "enduring speed",
      "name": "Enduring Speed",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_resilience" title="Enduring Spirit" -->

## Enduring Spirit

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_resilience`
- Snapshot ID: `40437`
- Source-Dokument: `7073`
- Kurzinfo: Enduring Spirit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enduring Spirit`
- Payload Hash: `e85d9ffdc29f9aa945fe2bfffd5ffdb84d646e9d1d76c083d33c96e7f0888c4d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.589391+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 7
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_resilience",
  "Name": "Enduring Spirit",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_escalating_exposure" title="Escalating Exposure" -->

## Escalating Exposure

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_escalating_exposure`
- Snapshot ID: `40338`
- Source-Dokument: `7073`
- Kurzinfo: Escalating Exposure aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Escalating Exposure`
- Payload Hash: `49a38ac1d29181ff1e906348e8e7e38ea1f80014b1edba53a9ffc461af76f603`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.420726+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_vulnerability"
  ],
  "Cost": 6400,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} applies a stacking <span class=\"highlight\">Spirit Amp</span> that increases your {g:citadel_inline_attribute:'SpiritDamage'} to the target.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechArmorDamageReduction",
        "LocTokenOverride": "EscalatingExposureTechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      },
      {
        "Key": "TechResist",
        "Value": 17
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MaxStacks",
        "Value": 12
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.7,
    "DescKey": "#upgrade_escalating_exposure_desc",
    "Main": [
      {
        "Key": "MagicIncreasePerStack",
        "Type": "tech_armor_down",
        "Value": 4.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_escalating_exposure",
  "Name": "Escalating Exposure",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "MagicIncreasePerStack": 1.5,
    "MaxStacks": 6,
    "TechArmorDamageReduction": -10,
    "TechResist": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "escalating exposure",
      "name": "Escalating Exposure",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_reinforcing_casings" title="Escalating Resilience" -->

## Escalating Resilience

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_reinforcing_casings`
- Snapshot ID: `40434`
- Source-Dokument: `7073`
- Kurzinfo: Escalating Resilience aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Escalating Resilience`
- Payload Hash: `26998d8191c515e289074043f7d53eb8b34363d09c3d57014578097ef6d528bc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.584778+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_clip_size"
  ],
  "Cost": 3200,
  "Description": "Grants <span class=\"highlight\">Bullet Resist</span> when your bullets hit an enemy hero. <span class=\"highlight\">Each shot can only grant one stack.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Value": 18
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 35
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BulletResistPerStack",
        "UsageFlags": "ConditionallyApplied",
        "Value": 2
      },
      {
        "Key": "BulletResistDuration",
        "Value": 24
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_reinforcing_casings_desc",
    "Main": [
      {
        "Key": "MaxArmorStacks",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_reinforcing_casings",
  "Name": "Escalating Resilience",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusClipSizePercent": 30,
    "BonusHealth": 125,
    "BulletResistPerStack": 2,
    "MaxArmorStacks": 20,
    "WeaponPower": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "escalating resilience",
      "name": "Escalating Resilience",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_eternal_gift" title="Eternal Gift" -->

## Eternal Gift

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_eternal_gift`
- Snapshot ID: `40339`
- Source-Dokument: `7073`
- Kurzinfo: Eternal Gift aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Eternal Gift`
- Payload Hash: `110ee854c75e16138a7d971ac02fdf7f985237fb2c6b18af2d2a63751618e854`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.422820+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Periodically gain a random permanent stat buff.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_eternal_gift_desc",
    "Main": [
      {
        "Key": "BuffFrequency",
        "Type": "duration",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 165,
    "DescKey": "#upgrade_eternal_gift_respawn_desc",
    "Main": [
      {
        "Key": "RespawnTime",
        "UsageFlags": "ConditionallyApplied",
        "Value": -70
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eternal_gift",
  "Name": "Eternal Gift",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BuffFrequency": -0.5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "eternal gift",
      "name": "Eternal Gift",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_self_bubble" title="Ethereal Shift" -->

## Ethereal Shift

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_self_bubble`
- Snapshot ID: `40446`
- Source-Dokument: `7073`
- Kurzinfo: Ethereal Shift aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ethereal Shift`
- Payload Hash: `22501bf537dc90fec9d53b7974af32689f86972d413c236fafd38c37f844f996`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.605833+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "You enter a void state and become <span class=\"highlight\">untargetable and invincible</span> for a short duration, during which you float slowly and cannot perform actions. Afterwards you gain <span class=\"highlight\">Spirit Power, Move Speed, and Spirit Resist</span>.<br>Can be canceled early.<br><span class=\"diminish\">Activation cancels any active ability.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSpirit",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3m"
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "FloatMoveSpeed",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_self_bubble_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "ShiftingVeilDuration",
        "Type": "duration",
        "Value": 4.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_self_bubble",
  "Name": "Ethereal Shift",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DampingFactor": {
      "Key": "DampingFactor",
      "Value": 3
    },
    "LiftHeight": {
      "Key": "LiftHeight",
      "Value": 200
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -10,
    "AbilityDuration": 2,
    "BonusMoveSpeed": "2m",
    "BonusSpirit": 30,
    "FloatMoveSpeed": "3.5m",
    "TechResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "ethereal shift",
      "name": "Ethereal Shift",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_express_shot" title="Express Shot" -->

## Express Shot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_express_shot`
- Snapshot ID: `40341`
- Source-Dokument: `7073`
- Kurzinfo: Express Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Express Shot`
- Payload Hash: `49bd8d17eee4d0baae6f785fde1e6c81426af3f6c90fc1d1850ca9e7aeddecff`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.426355+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 3200,
  "Description": "Your next attack will <span class=\"highlight\">fire twice</span> in quick succession with <span class=\"highlight\">increased damage</span> and velocity. This attack consumes extra ammo.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 60
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProcBulletVelocity",
        "LocTokenOverride": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 100
      },
      {
        "Key": "ProcAmmoConsumed",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_express_shot_desc",
    "Main": [
      {
        "Key": "ProcBaseAttackDamagePercent",
        "LocTokenOverride": "BaseAttackDamagePercent",
        "Scale": {
          "Type": "power_increase",
          "Value": 2.0
        },
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 125.0
      },
      {
        "Key": "ProcBaseAttackDamagePercentAltFire",
        "LocTokenOverride": "BaseAttackDamagePercentAltFire",
        "Scale": {
          "Type": "power_increase",
          "Value": 1.3
        },
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_express_shot",
  "Name": "Express Shot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusBulletSpeedPercent": 45,
    "ProcBaseAttackDamagePercent": 75,
    "ProcBaseAttackDamagePercentAltFire": 25
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "express shot",
      "name": "Express Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_clip_size" title="Extended Magazine" -->

## Extended Magazine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_clip_size`
- Snapshot ID: `40304`
- Source-Dokument: `7073`
- Kurzinfo: Extended Magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extended Magazine`
- Payload Hash: `8a05e185fee11219d6149036bbc37e6d8b34c586fa5aeba02e22d69d3ae5c5a2`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.356759+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_clip_size",
  "Name": "Extended Magazine",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusClipSizePercent": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "extended magazine",
      "name": "Extended Magazine",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_extra_charge" title="Extra Charge" -->

## Extra Charge

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_extra_charge`
- Snapshot ID: `40342`
- Source-Dokument: `7073`
- Kurzinfo: Extra Charge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Charge`
- Payload Hash: `31498c63dd606f080795f6dbab7099266357e24cd8dc32483dd2500cd6e5969c`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.428241+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Adds one to your ability max <span class=\"highlight\">charges</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSpiritForChargedAbilities",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_extra_charge_desc",
    "Main": [
      {
        "Key": "BonusAbilityCharges",
        "Type": "cast",
        "Value": 1
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_extra_charge",
  "Name": "Extra Charge",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusAbilityCharges": 1,
    "BonusSpiritForChargedAbilities": 7
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "extra charge",
      "name": "Extra Charge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health" title="Extra Health" -->

## Extra Health

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health`
- Snapshot ID: `40366`
- Source-Dokument: `7073`
- Kurzinfo: Extra Health aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Health`
- Payload Hash: `6b612155363e0bb023d20c3867ed8722eb05d9d4056afaa964456eff31531036`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.470780+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 210
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health",
  "Name": "Extra Health",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusHealth": 115
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "extra health",
      "name": "Extra Health",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_endurance" title="Extra Regen" -->

## Extra Regen

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_endurance`
- Snapshot ID: `40337`
- Source-Dokument: `7073`
- Kurzinfo: Extra Regen aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Regen`
- Payload Hash: `0cf37662cf05458f20dfa3a7ca1b7d7aa55965d8f6a8cfdc4aaadc499f6a07c3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.419067+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 2.5
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_endurance",
  "Name": "Extra Regen",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusHealthRegen": 9
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "extra regen",
      "name": "Extra Regen",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_improved_spirit" title="Extra Spirit" -->

## Extra Spirit

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_spirit`
- Snapshot ID: `40380`
- Source-Dokument: `7073`
- Kurzinfo: Extra Spirit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Spirit`
- Payload Hash: `a3f2f8299c84af19ac36fc23d7ffd7174ed95aa90b093bd22246fb6d38785329`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.494346+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechPower",
        "Value": 10
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_spirit",
  "Name": "Extra Spirit",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "TechPower": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "extra spirit",
      "name": "Extra Spirit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_improved_stamina" title="Extra Stamina" -->

## Extra Stamina

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_stamina`
- Snapshot ID: `40381`
- Source-Dokument: `7073`
- Kurzinfo: Extra Stamina aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Stamina`
- Payload Hash: `3bc22681007a2101f3b63e78e7c9a9924b116a869532abdfdd0ed120ad85fed4`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.496264+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaCooldownReduction",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "Stamina",
        "Value": 1
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_stamina",
  "Name": "Extra Stamina",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "Stamina": 1,
    "StaminaCooldownReduction": 14
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "extra stamina",
      "name": "Extra Stamina",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_fleetfoot_boots" title="Fleetfoot" -->

## Fleetfoot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_fleetfoot_boots`
- Snapshot ID: `40345`
- Source-Dokument: `7073`
- Kurzinfo: Fleetfoot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fleetfoot`
- Payload Hash: `c1daa00bfc9fcec3e69ca7561962b9c83773e706840fd37da6140830328fe225`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.434411+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 6
      },
      {
        "Key": "SlideScale",
        "Type": "movement_speed",
        "Value": 35
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_fleetfoot_boots_passive_desc",
    "Main": [],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_fleetfoot_boots_active_desc",
    "Main": [
      {
        "Key": "ActiveBonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3.0m"
      },
      {
        "Key": "SlowResistancePercent",
        "Type": "move_speed",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fleetfoot_boots",
  "Name": "Fleetfoot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileShootingSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileZoomedSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "Value": 100
    }
  },
  "ShopFilters": [
    "ClipSize",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "ActiveBonusMoveSpeed": "3m",
    "BulletResist": 12,
    "SlowResistancePercent": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "fleetfoot",
      "name": "Fleetfoot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_focus_lens" title="Focus Lens" -->

## Focus Lens

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_focus_lens`
- Snapshot ID: `40346`
- Source-Dokument: `7073`
- Kurzinfo: Focus Lens aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Focus Lens`
- Payload Hash: `32816c9a819f1802e1d6dda2572f53ca761adc4cedd6139a33d4b92e0e48763e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.436178+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_spirit_sap"
  ],
  "Cost": 6400,
  "Description": "Target an enemy to <span class=\"highlight\">Silence</span> them. A portion of <span class=\"highlight\">all damage dealt</span> during the silence gets applied to the target when the silence wears off.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Value": 10
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "20m"
      },
      {
        "Key": "ResistReductionDuration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_focus_lens_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.5
      },
      {
        "Key": "PercentDamage",
        "Type": "damage",
        "Value": 30
      },
      {
        "Key": "MagicResistReduction",
        "Type": "tech_armor_down",
        "Value": -9
      },
      {
        "Key": "TechPowerReduction",
        "Type": "spirit",
        "Value": -30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_focus_lens",
  "Name": "Focus Lens",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -12,
    "AbilityDuration": 0.25,
    "BonusFireRate": 20,
    "MagicResistReduction": -12,
    "PercentDamage": 20,
    "TechPowerReduction": -26
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "focus lens",
      "name": "Focus Lens",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_chonky" title="Fortitude" -->

## Fortitude

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_chonky`
- Snapshot ID: `40303`
- Source-Dokument: `7073`
- Kurzinfo: Fortitude aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fortitude`
- Payload Hash: `8a7ac6bc0a8174c31d3578d860bfe366d1418b899fec508daa86d7da9defe2ac`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.355285+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_health"
  ],
  "Cost": 3200,
  "Description": "After not taking damage for a period, gain health regen.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 375
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_chonky_desc",
    "Main": [
      {
        "Key": "RestoreDelay",
        "Value": 10
      },
      {
        "Key": "HealLifePercentOutOfCombat",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_chonky_high_health_passive_desc",
    "Main": [
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.5m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_chonky",
  "Name": "Fortitude",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealthThreshold": {
      "Key": "HealthThreshold",
      "Value": 75
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 375,
    "BonusMoveSpeed": "1m",
    "HealLifePercentOutOfCombat": 1,
    "RestoreDelay": -6
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "fortitude",
      "name": "Fortitude",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_fervor" title="Frenzy" -->

## Frenzy

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_fervor`
- Snapshot ID: `40343`
- Source-Dokument: `7073`
- Kurzinfo: Frenzy aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frenzy`
- Payload Hash: `a2b481e1908b8a938bfb52fccef73af422365fa24c7f8219049ba94814512da1`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.430713+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 160
      },
      {
        "Key": "BonusFireRate",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16,
    "DescKey": "#upgrade_fervor_passive",
    "Main": [
      {
        "Key": "FervorMovespeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "4m"
      },
      {
        "Key": "FervorFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      },
      {
        "Key": "FervorStatusResistancePercent",
        "Type": "duration",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fervor",
  "Name": "Frenzy",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LowHealthThreshold": {
      "Key": "LowHealthThreshold",
      "Value": 50
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "FireRate",
    "ClipSize",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusHealth": 125,
    "FervorFireRate": 20,
    "FervorMovespeed": "3m",
    "FervorStatusResistancePercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "frenzy",
      "name": "Frenzy",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_shivas_bracelet" title="Frostbite Charm" -->

## Frostbite Charm

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shivas_bracelet`
- Snapshot ID: `40450`
- Source-Dokument: `7073`
- Kurzinfo: Frostbite Charm aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frostbite Charm`
- Payload Hash: `09b680e61fdca190657e046baf54aa162f32873e997ac956924c38330c14aa7d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.613090+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Imbued ability has its <span class=\"highlight\">Cooldown reduced</span> and gains <span class=\"highlight\">Spirit Power</span>. When the ability deals damage, <span class=\"highlight\">freeze</span> the target and apply <span class=\"highlight\">bonus damage</span>. <span class=\"diminish\"><br><br>Cooldown is per target.</span>",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 10.0,
    "DescKey": "#upgrade_shivas_bracelet_desc",
    "Main": [
      {
        "Key": "ImbuedCooldownReduction",
        "Type": "cooldown",
        "Value": 50
      },
      {
        "Key": "ImbuedTechPower",
        "Type": "tech_damage",
        "Value": 70
      },
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 200
      },
      {
        "Key": "FreezeDuration",
        "Value": 1
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_shivas_bracelet",
  "Name": "Frostbite Charm",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -2,
    "Damage": 100,
    "FreezeDuration": 0.3,
    "ImbuedCooldownReduction": 15,
    "ImbuedTechPower": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "frostbite charm",
      "name": "Frostbite Charm",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_fury_trance" title="Fury Trance" -->

## Fury Trance

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_fury_trance`
- Snapshot ID: `40349`
- Source-Dokument: `7073`
- Kurzinfo: Fury Trance aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fury Trance`
- Payload Hash: `00da32b27e117c24885fcb18d2bc0b5f3c15418f3bdefb5a3edecff735f1c32f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.441092+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_vampire"
  ],
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 14
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 6.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 18.0,
    "DescKey": "#upgrade_fury_trance_active",
    "Main": [
      {
        "Key": "ActiveBonusFireRate",
        "Type": "fire_rate",
        "Value": 32
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fury_trance",
  "Name": "Fury Trance",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate",
    "WeaponDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "ActiveBonusFireRate": 25,
    "BonusHealth": 110,
    "BulletLifestealPercent": 28,
    "TechResist": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "fury trance",
      "name": "Fury Trance",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_glass_cannon" title="Glass Cannon" -->

## Glass Cannon

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_glass_cannon`
- Snapshot ID: `40351`
- Source-Dokument: `7073`
- Kurzinfo: Glass Cannon aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Glass Cannon`
- Payload Hash: `17e66f93970e754dfaac86b419bd5731901d54b709a5790d45ac1a5643a53db8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.444090+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Each hero kill grants <span class=\"highlight\">permanent Fire Rate</span> (up to a max of 8 times). Death results in the loss of 1 stack.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 80
      },
      {
        "Key": "MaxHealthLossPercent",
        "Type": "health",
        "Value": -13
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_glass_cannon_desc",
    "Main": [
      {
        "Key": "FireRatePerKill",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 7
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glass_cannon",
  "Name": "Glass Cannon",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusClipPerKill": {
      "Key": "BonusClipPerKill",
      "Value": 2
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 2
    },
    "BuildUpPerShot": {
      "Key": "BuildUpPerShot",
      "Value": 1.2
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 8
    },
    "SlowDuration": {
      "Key": "SlowDuration",
      "Type": "duration",
      "Value": 3
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "LocTokenOverride": "GlassCannon_SlowPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": 30
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BaseAttackDamagePercent": 60,
    "FireRatePerKill": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "glass cannon",
      "name": "Glass Cannon",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_glass_cannon2" title="Glass Cannon v2" -->

## Glass Cannon v2

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_glass_cannon2`
- Snapshot ID: `40352`
- Source-Dokument: `7073`
- Kurzinfo: Glass Cannon v2 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Glass Cannon v2`
- Payload Hash: `c1e8c8e9c2eca5594501424f6f3bc4443cc0de41633c50ad3ab41c07482bd8ab`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.445802+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Each hero kill grants <span class=\"highlight\">permanent Weapon Damage</span> (up to a max of {s:MaxStacks} times). Death results in the loss of 1 stack.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 100
      },
      {
        "Key": "BonusFireRate",
        "Value": 100
      },
      {
        "Key": "MaxHealthLossPercent",
        "Type": "health",
        "Value": -50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_glass_cannon2",
  "Name": "Glass Cannon v2",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "WeaponPowerPerKill": {
      "Key": "WeaponPowerPerKill",
      "UsageFlags": "ConditionallyApplied",
      "Value": 10
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_goose_egg" title="Golden Goose Egg" -->

## Golden Goose Egg

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_goose_egg`
- Snapshot ID: `40354`
- Source-Dokument: `7073`
- Kurzinfo: Golden Goose Egg aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Golden Goose Egg`
- Payload Hash: `fb82dde7987a1d579d27de9e842d21e9e3a095ed56c62ae5c08952e5614af8f4`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.448940+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": "Gain <span class=\"highlight\">souls over time</span>, as long as you are alive.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1m"
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Value": -10
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_goose_egg_active_desc",
    "Main": [
      {
        "Key": "BonusGoldPerMinute",
        "Type": "souls",
        "Value": 90
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_goose_egg",
  "Name": "Golden Goose Egg",
  "Other": {
    "AbilityChannelTime": {
      "Key": "AbilityChannelTime",
      "Type": "cast",
      "Value": 2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusBuffsPerGold": {
      "Key": "BonusBuffsPerGold",
      "Value": 80
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "StartingGold": {
      "Key": "StartingGold",
      "Value": 400
    },
    "ThinkRate": {
      "Key": "ThinkRate",
      "Value": 3
    }
  },
  "ShopFilters": [
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusBuffsPerGold": -50,
    "BonusSprintSpeed": 5,
    "OutOfCombatHealthRegen": 10,
    "OutgoingDamagePenaltyPercent": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "golden goose egg",
      "name": "Golden Goose Egg",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_range" title="Greater Expansion" -->

## Greater Expansion

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_range`
- Snapshot ID: `40483`
- Source-Dokument: `7073`
- Kurzinfo: Greater Expansion aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Greater Expansion`
- Payload Hash: `20f4301c50529f73047d07c18956d35e8e35a5f0d294faa504a9991525cb372e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.689861+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_reach"
  ],
  "Cost": 3200,
  "Description": "Increases the <span class=\"highlight\">range</span> and <span class=\"highlight\">effect radius</span> of your abilities and items.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_tech_range_desc",
    "Main": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_range",
  "Name": "Greater Expansion",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 30
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "TechRadiusMultiplier": 20,
    "TechRangeMultiplier": 20,
    "TechResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "greater expansion",
      "name": "Greater Expansion",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_grit" title="Grit" -->

## Grit

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_grit`
- Snapshot ID: `40356`
- Source-Dokument: `7073`
- Kurzinfo: Grit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grit`
- Payload Hash: `ba2d70d60fb1c22ab2e84beba7f5228b2f40224c924fd23625aa58884c908ad6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.452486+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> for a short duration.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60,
    "DescKey": "#upgrade_grit_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 200
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_grit",
  "Name": "Grit",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -25,
    "CombatBarrier": 250,
    "OutOfCombatHealthRegen": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "grit",
      "name": "Grit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_guardian_ward" title="Guardian Ward" -->

## Guardian Ward

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_guardian_ward`
- Snapshot ID: `40357`
- Source-Dokument: `7073`
- Kurzinfo: Guardian Ward aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Guardian Ward`
- Payload Hash: `c89c0b2151fcce563213cfb66d9b085f553494b2611d7eb1dc29fdc09176c775`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.454124+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Provide the target with a <span class=\"highlight\">Barrier</span> and temporary <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast.<br>Cooldown is reduced by half when cast on someone else.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 8
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60,
    "DescKey": "#upgrade_guardian_ward_desc",
    "Main": [
      {
        "Key": "GuardianWardCombatBarrier",
        "LocTokenOverride": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 250
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2.75m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_guardian_ward",
  "Name": "Guardian Ward",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownReductionPctOnOthers": {
      "Key": "CooldownReductionPctOnOthers",
      "Value": 50
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 8
    }
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -12,
    "ChannelMoveSpeed": 2,
    "GuardianWardCombatBarrier": 250,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "guardian ward",
      "name": "Guardian Ward",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_haunting_scream" title="Haunting Scream" -->

## Haunting Scream

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_haunting_scream`
- Snapshot ID: `40358`
- Source-Dokument: `7073`
- Kurzinfo: Haunting Scream aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Haunting Scream`
- Payload Hash: `cc28b8779a0140f35f35d6d9f0c8df6c14e8cb6cda0fc10ceb94bdf5c0b692c2`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.456081+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "Launch an expanding projectile which <span class=\"highlight\">Curses</span> enemies for a short duration.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechPowerPercent",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25.0,
    "DescKey": "#upgrade_haunting_scream_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_haunting_scream",
  "Name": "Haunting Scream",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Damage": {
      "Key": "Damage",
      "Value": 400
    },
    "GrowthPerMeter": {
      "Key": "GrowthPerMeter",
      "Value": "0.15m"
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "1m"
    },
    "InitialWidth": {
      "Key": "InitialWidth",
      "Value": "5.0m"
    },
    "SkipFrames": {
      "Key": "SkipFrames",
      "UsageFlags": "ConditionallyApplied",
      "Value": 6
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -8,
    "AbilityDuration": 0.25,
    "TechPowerPercent": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_eldritch_shot" title="Haunting Shot" -->

## Haunting Shot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_eldritch_shot`
- Snapshot ID: `40334`
- Source-Dokument: `7073`
- Kurzinfo: Haunting Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Haunting Shot`
- Payload Hash: `c7f4d522559709b2d2f6591281f5f66760395a31ee8aee7ce3d6dab3ad0a3820`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.412294+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Your next bullet applies a powerful debuff reducing the enemy's damage output, healing and movement speed. It also deals {g:citadel_inline_attribute:'BonusSpiritDamage'} based on the targets current Health. <br><br>The bullet is larger and penetrates through targets.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletRadius",
        "Value": "1.5m"
      },
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "MovementSpeedSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "ChargeUp": null,
    "Cooldown": 2.5,
    "DescKey": "#upgrade_eldritch_shot_desc",
    "Main": [
      {
        "Key": "HealthPctDamage",
        "Type": "tech_damage",
        "Value": 10
      },
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eldritch_shot",
  "Name": "Haunting Shot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -40
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 1
    },
    "Radius": {
      "Key": "Radius",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -1,
    "GroundDashReductionPercent": -10,
    "HealAmpReceivePenaltyPercent": -15,
    "HealAmpRegenPenaltyPercent": -15,
    "HealthPctDamage": 5,
    "MovementSpeedSlow": 10,
    "OutgoingDamagePenaltyPercent": -15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "haunting shot",
      "name": "Haunting Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_headhunter" title="Headhunter" -->

## Headhunter

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_headhunter`
- Snapshot ID: `40359`
- Source-Dokument: `7073`
- Kurzinfo: Headhunter aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Headhunter`
- Payload Hash: `36789912e15a766783e628704e3611fb88bd568e6c5e6d3d1c7bf295e8e9a534`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.458260+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_headshot_booster"
  ],
  "Cost": 3200,
  "Description": "Your next <span class=\"highlight\">headshot</span> against an enemy Hero deals {g:citadel_inline_attribute:'BonusWeaponDamage'}, {g:citadel_inline_attribute:'Heal'} you, and briefly grants {g:citadel_inline_attribute:'BonusMoveSpeed'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 5
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.75m"
      },
      {
        "Key": "MovementSpeedBonusDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_headhunter_desc",
    "Main": [
      {
        "Key": "HeadShotBonusDamage",
        "Scale": {
          "Type": "power_increase",
          "Value": 4.0
        },
        "Type": "bullet_damage",
        "Value": 75.0
      },
      {
        "Key": "HealPercentPerHeadshot",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.06
        },
        "Type": "healing",
        "Value": 4
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headhunter",
  "Name": "Headhunter",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -3,
    "HeadShotBonusDamage": 75,
    "HealPercentPerHeadshot": 4
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "headhunter",
      "name": "Headhunter",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_headshot_booster" title="Headshot Booster" -->

## Headshot Booster

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_headshot_booster`
- Snapshot ID: `40360`
- Source-Dokument: `7073`
- Kurzinfo: Headshot Booster aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Headshot Booster`
- Payload Hash: `84b8cb11adb4ffab6f95c845ba6805027938b652b2d33dffa05728b15a1d82d2`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.460561+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Your next <span class=\"highlight\">headshot</span> against an enemy Hero deals {g:citadel_inline_attribute:'BonusWeaponDamage'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 9,
    "DescKey": "#upgrade_headshot_booster_desc",
    "Main": [
      {
        "Key": "HeadShotBonusDamage",
        "Type": "bullet_damage",
        "Value": 45
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headshot_booster",
  "Name": "Headshot Booster",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    }
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "HeadShotBonusDamage": 55
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "headshot booster",
      "name": "Headshot Booster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_healbane" title="Healbane" -->

## Healbane

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_healbane`
- Snapshot ID: `40363`
- Source-Dokument: `7073`
- Kurzinfo: Healbane aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healbane`
- Payload Hash: `b5b6df5efbff632d62755ba9b5ed10a665a7137c20537918ed7193422e27f8df`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.465422+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your {g:citadel_inline_attribute:'SpiritDamage'} applies <span class=\"highlight\">Healing Reduction</span>. If an enemy hero dies under this effect, you receive a large heal.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 7
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_healbane_desc",
    "Main": [
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -35
      },
      {
        "Key": "HealOnKill",
        "Type": "healing",
        "Value": 275
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healbane",
  "Name": "Healbane",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -35
    }
  },
  "ShopFilters": [
    "Healing",
    "Disruption",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "HealOnKill": 125,
    "TechPower": 11
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "healbane",
      "name": "Healbane",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_healing_booster" title="Healing Booster" -->

## Healing Booster

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_healing_booster`
- Snapshot ID: `40365`
- Source-Dokument: `7073`
- Kurzinfo: Healing Booster aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Booster`
- Payload Hash: `86c24961e526ab823eeab883c076c98562248ea420098cdd6579eb2bcb45bbf8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.468997+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_endurance"
  ],
  "Cost": 1600,
  "Description": "Increases the effectiveness of your <span class=\"highlight\">healing</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 3
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_healing_booster_desc",
    "Main": [
      {
        "Key": "HealAmpCastPercent",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healing_booster",
  "Name": "Healing Booster",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPercent": {
      "Key": "HealAmpRegenPercent",
      "Value": 20
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusHealthRegen": 9,
    "HealAmpCastPercent": 15,
    "HealAmpRegenPercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "healing booster",
      "name": "Healing Booster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health_nova" title="Healing Nova" -->

## Healing Nova

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_nova`
- Snapshot ID: `40368`
- Source-Dokument: `7073`
- Kurzinfo: Healing Nova aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Nova`
- Payload Hash: `1bdd4e84c74818c4af0d39f421991ce2002e807a07b80f1c8303daa2ba0fb14f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.474139+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> yourself and nearby allies.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 5
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "RegenDuration",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "AuraRadius",
        "Type": "distance",
        "Value": "18m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60,
    "DescKey": "#upgrade_health_nova_desc",
    "Main": [
      {
        "Key": "TotalHealthRegen",
        "Scale": {
          "Type": "power_increase",
          "Value": 6
        },
        "Type": "healing",
        "Value": 325
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_nova",
  "Name": "Healing Nova",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 5
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "TechPower": 12,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12,
    "TotalHealthRegen": 425
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "healing nova",
      "name": "Healing Nova",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health_stimpak" title="Healing Rite" -->

## Healing Rite

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stimpak`
- Snapshot ID: `40372`
- Source-Dokument: `7073`
- Kurzinfo: Healing Rite aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Rite`
- Payload Hash: `c95dbdc9dcf8b014a56ab9a50a843719b119aa98e2c895ac3f33b6356348f957`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.480675+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 800,
  "Description": "Grant <span class=\"highlight\">Regen</span> and <span class=\"highlight\">Sprint Speed</span> to the target. Gets dispelled if you take damage from enemy players or objectives. Can be self-cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "RegenDuration",
        "Type": "duration",
        "Value": 20
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 70,
    "DescKey": "#upgrade_health_stimpak_desc",
    "Main": [
      {
        "Key": "TotalHealthRegen",
        "Scale": {
          "Type": "spirit",
          "Value": 1.1
        },
        "Type": "healing",
        "Value": 300
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stimpak",
  "Name": "Healing Rite",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -60,
    "BonusSprintSpeed": "6m",
    "TotalHealthRegen": 600
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "healing rite",
      "name": "Healing Rite",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_healbuff" title="Healing Tempo" -->

## Healing Tempo

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_healbuff`
- Snapshot ID: `40364`
- Source-Dokument: `7073`
- Kurzinfo: Healing Tempo aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Tempo`
- Payload Hash: `8c2cb965b3c14cfd7702183768f2a78530eddfe8d5312c835cf376e46fb3becb`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.467152+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_healing_booster"
  ],
  "Cost": 6400,
  "Description": "Applying {g:citadel_inline_attribute:'Heal'} to yourself or an ally grants the target {g:citadel_inline_attribute:'BonusFireRate'} and {g:citadel_inline_attribute:'BonusMoveSpeed'}.<br><br><span class=\"diminish\">Does not apply on innate Regen or passive Bullet/Spirit Lifesteals.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 10
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 6
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "HealAmpCastPercent",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_healbuff_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 35
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "1.25m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healbuff",
  "Name": "Healing Tempo",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPercent": {
      "Key": "HealAmpRegenPercent",
      "Value": 25
    },
    "MinimumHealAmount": {
      "Key": "MinimumHealAmount",
      "Value": 1
    }
  },
  "ShopFilters": [
    "FireRate",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusFireRate": 20,
    "BonusHealthRegen": 6,
    "BonusMoveSpeed": "2m",
    "HealAmpCastPercent": 10,
    "HealAmpRegenPercent": 10,
    "TechResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "healing tempo",
      "name": "Healing Tempo",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_dps_aura" title="Heroic Aura" -->

## Heroic Aura

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_dps_aura`
- Snapshot ID: `40331`
- Source-Dokument: `7073`
- Kurzinfo: Heroic Aura aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Heroic Aura`
- Payload Hash: `0b31be9f5041cbb765356c7530d6a8c3d7b6b155989af00e3dfb9c60970c2be5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.407107+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Provides <span class=\"highlight\">Bullet Resist</span> to nearby friendly units.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "35m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_dps_aura_desc",
    "Main": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 17
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "ActiveRadius",
        "Type": "distance",
        "Value": "35m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": 22.0,
    "DescKey": "#upgrade_dps_aura_active_desc",
    "Main": [
      {
        "Key": "ActiveBonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2.25m"
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 26
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_dps_aura",
  "Name": "Heroic Aura",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "NonHeroMult": {
      "Key": "NonHeroMult",
      "Value": 2
    }
  },
  "ShopFilters": [
    "FireRate",
    "Healing",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "ActiveBonusMoveSpeed": "3m",
    "ActiveRadius": "15m",
    "BonusFireRate": 34,
    "BulletResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "heroic aura",
      "name": "Heroic Aura",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_superacolytes_glove" title="Hex-Sealed Knuckles" -->

## Hex-Sealed Knuckles

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_superacolytes_glove`
- Snapshot ID: `40471`
- Source-Dokument: `7073`
- Kurzinfo: Hex-Sealed Knuckles aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hex-Sealed Knuckles`
- Payload Hash: `d7de6c069472b22bc0c7186245a42f64d8e8d9572f01aaf176b9aa3b1d65f426`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.666107+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "After dealing an accumulated <span class=\"highlight\">{s:StoredSpiritDamage}</span> spirit damage, your next <span class=\"highlight\">Heavy Melee Attack</span> deals an additional <span class=\"highlight\">{s:StoredSpiritDamage} spirit damage</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 1
      },
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 3,
    "DescKey": "#upgrade_superacolytes_glove_desc",
    "Main": [
      {
        "Key": "StoredSpiritDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.93
        },
        "Type": "tech_damage",
        "Value": 200.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_superacolytes_glove",
  "Name": "Hex-Sealed Knuckles",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 13
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spellshield" title="Hexafoil Ward" -->

## Hexafoil Ward

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellshield`
- Snapshot ID: `40460`
- Source-Dokument: `7073`
- Kurzinfo: Hexafoil Ward aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hexafoil Ward`
- Payload Hash: `981c24c6532ddcdef2d1ff2e37b35dff44a1a2794151be2a9e6343db245068a7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.630898+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Block the next {g:citadel_inline_attribute:'SpiritDamage'} or <span class=\"highlight\">Debuff</span>, preventing its effects. Only regenerates outside of combat.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SpellShieldFlavorText",
        "Type": "time",
        "Value": "asdasd"
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 12,
    "DescKey": "#upgrade_spellshield_desc",
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_spellshield",
  "Name": "Hexafoil Ward",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": "1.3m"
    },
    "SpellShieldLingerDuration": {
      "Key": "SpellShieldLingerDuration",
      "Value": 0.3
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_high_velocity_mag" title="High-Velocity Rounds" -->

## High-Velocity Rounds

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_high_velocity_mag`
- Snapshot ID: `40374`
- Source-Dokument: `7073`
- Kurzinfo: High-Velocity Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `High-Velocity Rounds`
- Payload Hash: `16a2a5c11b7131a7a1e6fb2fd5e47d326a66f680c4605f8a805422c304dd5992`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.483520+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 60
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_high_velocity_mag",
  "Name": "High-Velocity Rounds",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusBulletSpeedPercent": 45
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "high-velocity rounds",
      "name": "High-Velocity Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_hollow_point_rounds" title="Hollow Point" -->

## Hollow Point

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_hollow_point_rounds`
- Snapshot ID: `40375`
- Source-Dokument: `7073`
- Kurzinfo: Hollow Point aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hollow Point`
- Payload Hash: `c2dce8557ce3704e77c0ff71a8dfc5c6e263a1c0b195b4e9ddf42453efc6cc84`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.485272+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "When you are <span class=\"highlight\">above 65% health</span>, deal additional <span class=\"highlight\">Weapon Damage</span> and your bullets reduce enemy <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4.5
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_hollow_point_rounds_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -9
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_hollow_point_rounds",
  "Name": "Hollow Point",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LifeThreshold": {
      "Key": "LifeThreshold",
      "Value": 65
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 25,
    "BonusHealth": 150,
    "BulletArmorReduction": -12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "hollow point",
      "name": "Hollow Point",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_bullet_armor_reduction_aura" title="Hunter's Aura" -->

## Hunter's Aura

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_armor_reduction_aura`
- Snapshot ID: `40289`
- Source-Dokument: `7073`
- Kurzinfo: Hunter's Aura aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hunter's Aura`
- Payload Hash: `493a05ed7f44fcc36a804a1d665e487627912d64db1c743f6dbcde36eca8bbac`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.333080+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Reduces nearby enemies' <span class=\"highlight\">Bullet Resist and Fire Rate</span>. If there is only one enemy hero nearby, this <span class=\"highlight\">effect is doubled</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_bullet_armor_reduction_aura_desc",
    "Main": [
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -10
      },
      {
        "Key": "FireRateSlow",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyEnemyApplied",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_bullet_armor_reduction_aura",
  "Name": "Hunter's Aura",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SingleTargetPlayerMultiplier": {
      "Key": "SingleTargetPlayerMultiplier",
      "Value": 2
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 125,
    "BonusSprintSpeed": "3m",
    "BulletArmorReduction": -6,
    "FireRateSlow": 5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "hunter's aura",
      "name": "Hunter's Aura",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_soaring_spirit" title="Improved Spirit" -->

## Improved Spirit

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_soaring_spirit`
- Snapshot ID: `40458`
- Source-Dokument: `7073`
- Kurzinfo: Improved Spirit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Improved Spirit`
- Payload Hash: `8d4479c86f4a8b99fc35c866d623fc5cd564218c85454c570c3fdb93afd96577`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.626872+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_spirit"
  ],
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechPower",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_soaring_spirit",
  "Name": "Improved Spirit",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusHealth": {
      "Key": "BonusHealth",
      "Type": "health",
      "Value": 75
    },
    "BonusSprintSpeed": {
      "Key": "BonusSprintSpeed",
      "Type": "move_speed",
      "Value": "1m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Movement",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "OutOfCombatHealthRegen": 3,
    "TechPower": 22
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "improved spirit",
      "name": "Improved Spirit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_auto_cleanse" title="Indomitable" -->

## Indomitable

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_auto_cleanse`
- Snapshot ID: `40278`
- Source-Dokument: `7073`
- Kurzinfo: Indomitable aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Indomitable`
- Payload Hash: `518603d12ad9c56e2a6140ef6830199e3ae39fb10723833d6b060fcdaa16b860`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.313780+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_vex_barrier"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 10
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 10
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 55,
    "DescKey": "#upgrade_upgrade_auto_cleanse_desc",
    "Main": [
      {
        "Key": "VexBarrierCombatBarrier",
        "LocTokenOverride": "CombatBarrier",
        "Scale": {
          "Type": "spirit",
          "Value": 2.0
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 325.0
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_auto_cleanse",
  "Name": "Indomitable",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownReductionOnProc": {
      "Key": "CooldownReductionOnProc",
      "Type": "cooldown",
      "Value": 20
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -35,
    "BulletResist": 14,
    "TechResist": 14,
    "VexBarrierCombatBarrier": 450
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "indomitable",
      "name": "Indomitable",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_infinite_rounds" title="Infinite Rounds" -->

## Infinite Rounds

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_infinite_rounds`
- Snapshot ID: `40382`
- Source-Dokument: `7073`
- Kurzinfo: Infinite Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infinite Rounds`
- Payload Hash: `a31d0583b3878dc2164ac6498a14d27754a298d3ae0e5b7696f8d6e2e8ff5e8b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.498150+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "You have <span class=\"highlight\">infinite</span> ammo.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Value": 200
      },
      {
        "Key": "BonusFireRate",
        "Value": 35
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_aprounds_desc",
    "Main": [
      {
        "Key": "ProcChance",
        "Value": 65
      }
    ],
    "Type": null
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_infinite_rounds_desc",
    "Main": [],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_infinite_rounds",
  "Name": "Infinite Rounds",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BonusBulletSpeedPercent": 100,
    "BonusFireRate": 20,
    "ProcChance": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "infinite rounds",
      "name": "Infinite Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_infuser" title="Infuser" -->

## Infuser

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_infuser`
- Snapshot ID: `40384`
- Source-Dokument: `7073`
- Kurzinfo: Infuser aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infuser`
- Payload Hash: `8b7a7444c94aa4c55c3d01de7f8e9b2b6d9466851cd7daf5644a95032943d5f5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.501336+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Gain <span class=\"highlight\">Spirit Lifesteal</span> and <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityLifestealPercentHeroPassive",
        "Type": "healing",
        "Value": 13
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
      },
      {
        "Key": "BonusHealth",
        "Value": 100
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30.0,
    "DescKey": "#upgrade_infuser_desc",
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 70
      },
      {
        "Key": "BonusSpirit",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_infuser",
  "Name": "Infuser",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "NonHeroAbilityLifestealTooltipOnly": {
      "Key": "NonHeroAbilityLifestealTooltipOnly",
      "Value": 3
    }
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -10,
    "AbilityLifestealPercentHeroPassive": 16,
    "BonusHealth": 50,
    "BonusSpirit": 30,
    "TechResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "infuser",
      "name": "Infuser",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_inhibitor" title="Inhibitor" -->

## Inhibitor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_inhibitor`
- Snapshot ID: `40385`
- Source-Dokument: `7073`
- Kurzinfo: Inhibitor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Inhibitor`
- Payload Hash: `4c0a81eb1168b55572e6c5bfa90ffd5796a12de8a2867fa207844f9f2ecdd1d3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.502982+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets build up to reduce the target's <span class=\"highlight\">outgoing damage</span> and apply <span class=\"highlight\">healing reduction</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 10
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 150
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 0.77
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_inhibitor_desc",
    "Main": [
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -30
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_inhibitor",
  "Name": "Inhibitor",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -40
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BaseAttackDamagePercent": 20,
    "BonusHealth": 125,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "OutgoingDamagePenaltyPercent": -20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "inhibitor",
      "name": "Inhibitor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_intensifying_clip" title="Intensifying Magazine" -->

## Intensifying Magazine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_intensifying_clip`
- Snapshot ID: `40386`
- Source-Dokument: `7073`
- Kurzinfo: Intensifying Magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Intensifying Magazine`
- Payload Hash: `13a557480f119ef3daf8416f223ce86d26b060e6a35db15f976d2354fdff3820`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.504498+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Increases <span class=\"highlight\">Weapon Damage</span> as you continuously fire your weapon.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": ""
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_intensifying_clip_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercentAtMaxDuration",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 45
      },
      {
        "Key": "ShootDurationForMax",
        "Value": 2.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_intensifying_clip",
  "Name": "Intensifying Magazine",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercentAtMaxDuration": 55,
    "BonusClipSizePercent": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "intensifying magazine",
      "name": "Intensifying Magazine",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_juggernaut" title="Juggernaut" -->

## Juggernaut

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_juggernaut`
- Snapshot ID: `40387`
- Source-Dokument: `7073`
- Kurzinfo: Juggernaut aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Juggernaut`
- Payload Hash: `04974f1a0818f0e6374848431ef6d1e26a3d8df65d3e10b98c0b80a1af3f2296`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.506173+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_cardio_calibrator"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "2.5m"
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 25
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SlowResistancePercent",
        "Type": "move_speed",
        "Value": 50
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "FireRateSlowDuration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_juggernaut_slow_desc",
    "Main": [
      {
        "Key": "FireRateSlow",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyEnemyApplied",
        "Value": 40
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_juggernaut",
  "Name": "Juggernaut",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusHealthRegen": 8,
    "BonusMoveSpeed": "3.5m",
    "FireRateSlow": 20,
    "MeleeResistPercent": 15,
    "SlowResistancePercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "juggernaut",
      "name": "Juggernaut",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_kinetic_sash" title="Kinetic Dash" -->

## Kinetic Dash

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_kinetic_sash`
- Snapshot ID: `40388`
- Source-Dokument: `7073`
- Kurzinfo: Kinetic Dash aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kinetic Dash`
- Payload Hash: `e37d628b47bfb14ccda5443998009c9945f7ff874cc7c40ce6e7237be6e7e9a8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.507707+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 1600,
  "Description": "When you <span class=\"highlight\">Dash-Jump</span> you gain <span class=\"highlight\">Fire Rate</span> and bonus <span class=\"highlight\">Ammo</span> until your next reload. Lasts up to 7s.",
  "Info1": {
    "Alt": [
      {
        "Key": "Stamina",
        "Value": 1
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_kinetic_sash_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "BonusClipSize",
        "LocTokenOverride": "TemporaryBonusClipSize",
        "Type": "clipsize",
        "UsageFlags": "ConditionallyApplied",
        "Value": 6
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_kinetic_sash",
  "Name": "Kinetic Dash",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate",
    "ClipSize",
    "Movement",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusClipSize": 6,
    "BonusFireRate": 20,
    "Stamina": 1,
    "StaminaCooldownReduction": 14
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "kinetic dash",
      "name": "Kinetic Dash",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_target_stun" title="Knockdown" -->

## Knockdown

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_target_stun`
- Snapshot ID: `40475`
- Source-Dokument: `7073`
- Kurzinfo: Knockdown aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Knockdown`
- Payload Hash: `95c0053c6b3ef810eb3a1458cbe9d8a6fdf52d9b376d805eadedfa61b4b21eb6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.674645+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": "Apply a <span class=\"highlight\">Stun</span> after <span class=\"highlight\">2s</span>. Stun duration is increased against <span class=\"highlight\">airborne</span> targets.<br><br><span class=\"diminish\">Increases the target's gravity for the duration of the stun.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "45m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_target_stun_desc",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 0.5
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_target_stun",
  "Name": "Knockdown",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxBonusDuration": {
      "Key": "MaxBonusDuration",
      "Value": 1.5
    },
    "MaxHeightForBonus": {
      "Key": "MaxHeightForBonus",
      "Value": "30m"
    },
    "StunDelay": {
      "Key": "StunDelay",
      "Type": "duration",
      "Value": 2
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 5
    },
    "VisualContractRadius": {
      "Key": "VisualContractRadius",
      "Value": "3m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 75,
    "StunDuration": 0.75,
    "TechRadiusMultiplier": 6,
    "TechRangeMultiplier": 6
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "knockdown",
      "name": "Knockdown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_damage_recycler" title="Leech" -->

## Leech

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_damage_recycler`
- Snapshot ID: `40323`
- Source-Dokument: `7073`
- Kurzinfo: Leech aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Leech`
- Payload Hash: `03522cca81a281fe0fff58c6772e700091b11a1f4a7510d7873a776ac9cf10d7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.392409+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_vampire",
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Reduces the effect of enemy applied <span class=\"highlight\">healing reduction</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 180
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 12
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 25
      },
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_damage_recycler",
  "Name": "Leech",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityLifestealPercentHero": 15,
    "BaseAttackDamagePercent": 15,
    "BonusHealth": 200,
    "BulletLifestealPercent": 15,
    "TechPower": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "leech",
      "name": "Leech",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_boxing_glove" title="Lifestrike" -->

## Lifestrike

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_boxing_glove`
- Snapshot ID: `40287`
- Source-Dokument: `7073`
- Kurzinfo: Lifestrike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lifestrike`
- Payload Hash: `7ce3431ec098d9c3835ce50c21947a1f2c80c34e9ff8fc2f862e7c341343f442`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.330002+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_lifestrike_gauntlets"
  ],
  "Cost": 3200,
  "Description": "Your <span class=\"highlight\">Melee Attack</span> applies <span class=\"highlight\">Movement Slow</span> and <span class=\"highlight\">heals you</span> for a percentage of the <span class=\"highlight\">Melee Damage</span> dealt plus a fixed amount. <span class=\"diminish\"><br><br>This heal is 40% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 16
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 4,
    "DescKey": "#upgrade_boxing_glove_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 60
      },
      {
        "Key": "LifestealHeal",
        "Scale": {
          "Type": "power_increase",
          "Value": 1.5
        },
        "Type": "healing",
        "Value": 100
      },
      {
        "Key": "LifestealHealPercent",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.5
        },
        "Type": "healing",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_boxing_glove",
  "Name": "Lifestrike",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LightMeleeCooldownMult": {
      "Key": "LightMeleeCooldownMult",
      "Value": 1.5
    },
    "NonHeroHealPct": {
      "Key": "NonHeroHealPct",
      "Value": 40
    }
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -3,
    "BonusHealth": 125,
    "BonusMeleeDamagePercent": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "lifestrike",
      "name": "Lifestrike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ultimate_burst" title="Lightning Scroll" -->

## Lightning Scroll

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ultimate_burst`
- Snapshot ID: `40491`
- Source-Dokument: `7073`
- Kurzinfo: Lightning Scroll aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lightning Scroll`
- Payload Hash: `31ad2cfb213ad16774c9c380407aed48f0bd6d8832526ced0f7dd3d806716fcc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.704345+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_slow"
  ],
  "Cost": 6400,
  "Description": "Damage from your ultimate applies a {g:citadel_inline_attribute:'Stun'} and deals {g:citadel_inline_attribute:'BonusSpiritDamage'} after a short delay.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "MovementSpeedSlow",
        "LocTokenOverride": "LightningScrollMysticSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DelayBeforeStun",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ultimate_burst_desc",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 0.75
      },
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 150
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ultimate_burst_hint",
    "Main": [],
    "Type": "Passive"
  },
  "Info4": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ultimate_burst_upgrade",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ultimate_burst",
  "Name": "Lightning Scroll",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 2
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "GroundDashReductionPercent": {
      "Key": "GroundDashReductionPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": -12
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "Type": "slow",
      "Value": 80
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusHealth": 100,
    "BonusSprintSpeed": "5m",
    "Damage": 100,
    "StunDuration": 0.75
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "lightning scroll",
      "name": "Lightning Scroll",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_long_range" title="Long Range" -->

## Long Range

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_long_range`
- Snapshot ID: `40390`
- Source-Dokument: `7073`
- Kurzinfo: Long Range aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Long Range`
- Payload Hash: `8a8c7f4af8f16e7a9e39b4f3c773b5ef932e3a11fe3edae8d07c9cffbfaa4991`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.511012+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when <span class=\"highlight\">beyond a minimum distance</span> from your target.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAttackRangePercent",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "LongRangeBonusWeaponPowerMinRange",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_long_range_desc",
    "Main": [
      {
        "Key": "LongRangeBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_long_range",
  "Name": "Long Range",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusAttackRangePercent": 8,
    "LongRangeBonusWeaponPower": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "long range",
      "name": "Long Range",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_critshot" title="Lucky Shot" -->

## Lucky Shot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_critshot`
- Snapshot ID: `40321`
- Source-Dokument: `7073`
- Kurzinfo: Lucky Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lucky Shot`
- Payload Hash: `267ec7ada193b1e12c0254c127ce1fff72a4512a34f7b371d432e4d0a6b6bab3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.388470+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets have a chance to be empowered, causing them to deal <span class=\"highlight\">bonus weapon damage</span> on hit.<br><span class=\"diminish\">Bonus damage cannot Crit.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProcChance",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_critshot_desc",
    "Main": [
      {
        "Key": "CritDamagePercent",
        "Type": "bullet_damage",
        "Value": 100
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_critshot",
  "Name": "Lucky Shot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusClipSizePercent": 40,
    "CritDamagePercent": 30,
    "ProcChance": 5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "lucky shot",
      "name": "Lucky Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_carpet" title="Magic Carpet" -->

## Magic Carpet

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_carpet`
- Snapshot ID: `40393`
- Source-Dokument: `7073`
- Kurzinfo: Magic Carpet aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Magic Carpet`
- Payload Hash: `1f010ba8b42fb08f6a0add7459e02701a52541db292606942719300df0035c1b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.515680+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "Summon a Magic Carpet that will <span class=\"highlight\">fly</span> you away. While flying you are immune to slows and doing any action will dismiss the carpet. <span class=\"diminish\"><br>Cannot use abilities while the carpet is being summoned.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 15
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      },
      {
        "Key": "TechPower",
        "Value": 14
      },
      {
        "Key": "GravityScale",
        "Value": -15
      },
      {
        "Key": "AirControlPercent",
        "Type": "move_speed",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "FlyMoveSpeed",
        "LocTokenOverride": "MagicCarpetMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "7m"
      },
      {
        "Key": "SummonDuration",
        "Value": 1.3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 32.0,
    "DescKey": "#upgrade_magic_carpet_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_carpet",
  "Name": "Magic Carpet",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityDuration": 8,
    "BonusAbilityDurationPercent": 15,
    "FlyMoveSpeed": "6m",
    "SummonDuration": -0.3,
    "TechPower": 46
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "magic carpet",
      "name": "Magic Carpet",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rocket_booster" title="Majestic Leap" -->

## Majestic Leap

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rocket_booster`
- Snapshot ID: `40442`
- Source-Dokument: `7073`
- Kurzinfo: Majestic Leap aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Majestic Leap`
- Payload Hash: `e5a24be2e1bb0d606c36ee0abc6b7269361426355f638f16bbc5b1cce4f1ce18`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.598684+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Launch yourself</span> high into the air and grant yourself a <span class=\"highlight\">Barrier</span>. While in the air, you can use the active again to drop down faster.<br><br><span class=\"diminish\">Cannot be used for 5s if attacked by enemy Hero.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "AirControlPercentBarrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_rocket_booster_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Scale": {
          "Type": "power_increase",
          "Value": 12.0
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 200.0
      },
      {
        "Key": "InterruptCooldown",
        "Type": "cooldown",
        "Value": 5
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rocket_booster",
  "Name": "Majestic Leap",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AirControlPercent": {
      "Key": "AirControlPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DropDownSpeed": {
      "Key": "DropDownSpeed",
      "Value": "35m"
    },
    "ImpactHeight": {
      "Key": "ImpactHeight",
      "Value": "2m"
    },
    "JumpVelocityHidden": {
      "Key": "JumpVelocityHidden",
      "Value": "27m"
    },
    "MaxLandingSpeed": {
      "Key": "MaxLandingSpeed",
      "Value": "20m"
    },
    "MinAimAngle": {
      "Key": "MinAimAngle",
      "Value": 30
    },
    "SlamDownRadius": {
      "Key": "SlamDownRadius",
      "Type": "distance",
      "Value": "10m"
    },
    "SlowDuration": {
      "Key": "SlowDuration",
      "Type": "duration",
      "Value": 2.5
    },
    "SlowPercent": {
      "Key": "SlowPercent",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": 40
    },
    "TossSpeed": {
      "Key": "TossSpeed",
      "Value": 500
    },
    "VerticalDifferenceTolerance": {
      "Key": "VerticalDifferenceTolerance",
      "Value": "2m"
    }
  },
  "ShopFilters": [
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -35,
    "CombatBarrier": 275,
    "InterruptCooldown": -3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "majestic leap",
      "name": "Majestic Leap",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rocket_boots" title="Majestic Leap - Disabled" -->

## Majestic Leap - Disabled

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rocket_boots`
- Snapshot ID: `40443`
- Source-Dokument: `7073`
- Kurzinfo: Majestic Leap - Disabled aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Majestic Leap - Disabled`
- Payload Hash: `dcd4c8c9039ee34d362c202a08745cbac1616a7ed8cae838bda174cba89ac9b3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.600735+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 8
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_rocket_boots_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_rocket_boots",
  "Name": "Majestic Leap - Disabled",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AirControlPercent": {
      "Key": "AirControlPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "InterruptCooldown": {
      "Key": "InterruptCooldown",
      "Type": "cooldown",
      "Value": 4
    },
    "JumpVelocityHidden": {
      "Key": "JumpVelocityHidden",
      "Value": "22.5m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_melee_charge" title="Melee Charge" -->

## Melee Charge

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_melee_charge`
- Snapshot ID: `40404`
- Source-Dokument: `7073`
- Kurzinfo: Melee Charge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Melee Charge`
- Payload Hash: `e2148d200ac90065f374e3fa1b599a58c75d6c8081226761472380aabfa357c7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.534175+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your next <span class=\"highlight\">Heavy Melee</span> attack against an enemy <span class=\"highlight\">deals increased damage</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 10
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "MeleeDistanceScale",
        "Value": 50
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 5,
    "DescKey": "#upgrade_melee_charge_desc",
    "Main": [
      {
        "Key": "BonusHeavyMeleeDamage",
        "Type": "melee_damage",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_melee_charge",
  "Name": "Melee Charge",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 2,
  "Upgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BulletResist": 12,
    "MeleeDistanceScale": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "melee charge",
      "name": "Melee Charge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_lifestrike_gauntlets" title="Melee Lifesteal" -->

## Melee Lifesteal

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_lifestrike_gauntlets`
- Snapshot ID: `40389`
- Source-Dokument: `7073`
- Kurzinfo: Melee Lifesteal aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Melee Lifesteal`
- Payload Hash: `06bcc3c67c6bce6898a7e9092f121529538b5e8c469a4bce9ddf7576041da2c3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.509411+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Your next <span class=\"highlight\">Melee</span> attack <span class=\"highlight\">heals you</span>. <span class=\"diminish\"><br><br>This heal is 30% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 12
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_lifestrike_gauntlets_desc",
    "Main": [
      {
        "Key": "LifestrikeHeal",
        "Type": "healing",
        "Value": 100
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_lifestrike_gauntlets",
  "Name": "Melee Lifesteal",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LightMeleeCooldownMult": {
      "Key": "LightMeleeCooldownMult",
      "Value": 1.5
    },
    "NonHeroHealPct": {
      "Key": "NonHeroHealPct",
      "Value": 30
    }
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -6,
    "BonusMeleeDamagePercent": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "melee lifesteal",
      "name": "Melee Lifesteal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ethereal_bullets" title="Mercurial Magnum" -->

## Mercurial Magnum

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ethereal_bullets`
- Snapshot ID: `40340`
- Source-Dokument: `7073`
- Kurzinfo: Mercurial Magnum aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mercurial Magnum`
- Payload Hash: `824b31e0c01b68ae62a9f384e9404af3e127608c66fc8a5dbadea139a257e9e9`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.424586+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_quick_silver"
  ],
  "Cost": 6400,
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use. Until your next reload, your <span class=\"highlight\">bullets deal {g:citadel_inline_attribute:'BonusSpiritDamage'}</span> based on your Spirit Power.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
      },
      {
        "Key": "TechPower",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AmmoReloadPercent",
        "Value": 100
      }
    ],
    "ChargeUp": 14,
    "Cooldown": null,
    "DescKey": "#upgrade_ethereal_bullets_desc",
    "Main": [
      {
        "Key": "BulletsBonusMagicDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.49
        },
        "Type": "tech_damage",
        "Value": 25.0
      },
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Type": "tech_damage",
        "Value": 60.0
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 22
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_ethereal_bullets",
  "Name": "Mercurial Magnum",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 15
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuffDuration": {
      "Key": "BuffDuration",
      "Value": 12
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusClipSizePercent": 60,
    "BonusFireRate": 20,
    "BulletsBonusMagicDamage": 20,
    "Damage": 120,
    "TechPower": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mercurial magnum",
      "name": "Mercurial Magnum",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_metal_skin" title="Metal Skin" -->

## Metal Skin

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_metal_skin`
- Snapshot ID: `40406`
- Source-Dokument: `7073`
- Kurzinfo: Metal Skin aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Metal Skin`
- Payload Hash: `7c72a925f4888c8eede83c67421b4cc3892ce922a7f95afda6ebdccab1c947f0`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.537344+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Become <span class=\"highlight\">immune to bullets</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ActiveMoveSpeedPenalty",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "-1.5m"
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -20
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": 24.0,
    "DescKey": "#upgrade_metal_skin_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_metal_skin",
  "Name": "Metal Skin",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -2,
    "ActiveMoveSpeedPenalty": "6.5m",
    "BulletResist": 5,
    "GroundDashReductionPercent": 60
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "metal skin",
      "name": "Metal Skin",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_non_player_bonus" title="Monster Rounds" -->

## Monster Rounds

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_non_player_bonus`
- Snapshot ID: `40412`
- Source-Dokument: `7073`
- Kurzinfo: Monster Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Monster Rounds`
- Payload Hash: `de058e67f103d448c1b6ed3c48615975e7959072356d09aa7aa8cbbe1316cfbc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.547949+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "NonPlayerBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "NonPlayerBulletResist",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_non_player_bonus",
  "Name": "Monster Rounds",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "NonPlayerBonusWeaponPower": 35,
    "NonPlayerBulletResist": 35,
    "OutOfCombatHealthRegen": 1
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "monster rounds",
      "name": "Monster Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_burst" title="Mystic Burst" -->

## Mystic Burst

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_burst`
- Snapshot ID: `40392`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Burst aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Burst`
- Payload Hash: `968f1acf7f0577f39383d05fdced3011df0e10cc1596c45ed2cbfa4e17b42780`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.513951+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, causing abilities dealing more than <span class=\"highlight\">80</span> damage to deal additional damage.",
  "Info1": {
    "Alt": [],
    "ChargeUp": 14,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_burst_desc",
    "Main": [
      {
        "Key": "Damage",
        "LocTokenOverride": "MagicBurstDamage",
        "Type": "tech_damage",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_burst",
  "Name": "Mystic Burst",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 14
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MinimumDamage": {
      "Key": "MinimumDamage",
      "Value": 80
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "Damage": 60
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic burst",
      "name": "Mystic Burst",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_patrons_blessing" title="Mystic Conduit" -->

## Mystic Conduit

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_patrons_blessing`
- Snapshot ID: `40416`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Conduit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Conduit`
- Payload Hash: `0e3959b249dcb8293856880180818e979d24aa2d381420754bb2d509afd1ca7b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.554967+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Provides yourself and allies with a powerful spirit focused aura. Has reduced values on allies.",
  "Info1": {
    "Alt": [
      {
        "Key": "AllyPercentage",
        "Value": 50
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "25m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_patrons_blessing_desc",
    "Main": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 40
      },
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_armor_up",
        "Value": 300
      },
      {
        "Key": "DamageThresholdDuration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25,
    "DescKey": "#upgrade_patrons_blessing_proc_desc",
    "Main": [
      {
        "Key": "HealAmount",
        "Type": "healing",
        "Value": 700
      },
      {
        "Key": "HealRadius",
        "Type": "distance",
        "Value": "35m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_patrons_blessing",
  "Name": "Mystic Conduit",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechPower": {
      "Key": "TechPower",
      "Value": 40
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 40
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "CooldownReduction": 10,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic conduit",
      "name": "Mystic Conduit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_reach" title="Mystic Expansion" -->

## Mystic Expansion

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_reach`
- Snapshot ID: `40396`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Expansion aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Expansion`
- Payload Hash: `86dae919016ad86c20f011e8ca040fcb2dd9a58b89306bba23bdeb619217be79`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.520440+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Imbue an ability to increase its <span class=\"highlight\">range</span> and <span class=\"highlight\">effect radius</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_reach_desc",
    "Main": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_reach",
  "Name": "Mystic Expansion",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 20
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic expansion",
      "name": "Mystic Expansion",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_mystic_regeneration" title="Mystic Regeneration" -->

## Mystic Regeneration

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystic_regeneration`
- Snapshot ID: `40408`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Regeneration aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Regeneration`
- Payload Hash: `0a7e65bffbd8a1299e234406d07306b8190cf8f43e144fadecbb7e7d864557a4`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.540585+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} to enemy Heroes grants you Bonus {g:citadel_inline_attribute:'Regen'}. Stacks when dealing damage to different heroes.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "RegenerationDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_mystic_regeneration_desc",
    "Main": [
      {
        "Key": "Regeneration",
        "Type": "healing",
        "Value": 4
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_mystic_regeneration",
  "Name": "Mystic Regeneration",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusHealth": 150,
    "Regeneration": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic regeneration",
      "name": "Mystic Regeneration",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_mystic_reverb" title="Mystic Reverb" -->

## Mystic Reverb

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystic_reverb`
- Snapshot ID: `40409`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Reverb aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Reverb`
- Payload Hash: `de4e6f64ba2c082591a4d4444ca23452c77d0eaa2d809c296b6239d0216d090d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.542339+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "16m"
      },
      {
        "Key": "DelayDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 6.25,
    "DescKey": "#upgrade_tech_bleed_desc",
    "Main": [
      {
        "Key": "TechDamagePercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50
      },
      {
        "Key": "ImbueAbilityLifesteal",
        "LocTokenOverride": "ImbueAbilityLifesteal",
        "Type": "healing",
        "Value": 22
      },
      {
        "Key": "MovementSpeedSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_mystic_reverb",
  "Name": "Mystic Reverb",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxHealthDamage": {
      "Key": "MaxHealthDamage",
      "LocTokenOverride": "MagicShockDamage",
      "Type": "tech_damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": 10
    },
    "MinimumDamage": {
      "Key": "MinimumDamage",
      "Value": 100
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityLifestealPercentHero": 25,
    "TechDamagePercent": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic reverb",
      "name": "Mystic Reverb",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_crackshot" title="Mystic Shot" -->

## Mystic Shot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_crackshot`
- Snapshot ID: `40320`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Shot`
- Payload Hash: `b68c8b9d4a1743a6575dfb611ba76b1712c808a427c0a1b65f2544480ccceb9e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.385969+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your next bullet deals bonus {g:citadel_inline_attribute:'SpiritDamage'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 9,
    "DescKey": "#upgrade_crackshot_desc",
    "Main": [
      {
        "Key": "ProcBonusMagicDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 1.2
        },
        "Type": "tech_damage",
        "Value": 40.0
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crackshot",
  "Name": "Mystic Shot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 1
    },
    "Radius": {
      "Key": "Radius",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "ProcBonusMagicDamage": 109,
    "SpiritPower": 14
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic shot",
      "name": "Mystic Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_slow" title="Mystic Slow" -->

## Mystic Slow

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_slow`
- Snapshot ID: `40399`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Slow aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Slow`
- Payload Hash: `38f2e841f53a27c762dd131cb1a17594826e9862d8b94cfaa5ace69da2ec12ca`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.525406+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "When the target takes {g:citadel_inline_attribute:'SpiritDamage'}, they have their <span class=\"highlight\">Move Speed</span> reduced.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_slow_desc",
    "Main": [
      {
        "Key": "MovementSpeedSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -12
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_slow",
  "Name": "Mystic Slow",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BonusHealth": 100,
    "BonusSprintSpeed": 1,
    "GroundDashReductionPercent": -10,
    "MovementSpeedSlow": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic slow",
      "name": "Mystic Slow",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_vulnerability" title="Mystic Vulnerability" -->

## Mystic Vulnerability

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_vulnerability`
- Snapshot ID: `40402`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Vulnerability aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Vulnerability`
- Payload Hash: `43ebbe684054e2e6da5e33d9e46dbc99b73cede896841e16a5ecb367f70abf11`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.531055+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "When an enemy takes {g:citadel_inline_attribute:'SpiritDamage'}, they have their {g:citadel_inline_attribute:'SpiritResist'} reduced.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_vulnerability_desc",
    "Main": [
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_vulnerability",
  "Name": "Mystic Vulnerability",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "TechArmorDamageReduction": -10,
    "TechResist": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystic vulnerability",
      "name": "Mystic Vulnerability",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_mystical_piano" title="Mystical Piano" -->

## Mystical Piano

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystical_piano`
- Snapshot ID: `40410`
- Source-Dokument: `7073`
- Kurzinfo: Mystical Piano aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystical Piano`
- Payload Hash: `8a5b244d971c381ce3a56e1c5891124e3a88f472a7d7b9378a3636e4732ed230`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.544368+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 9999,
  "Description": "After a short delay, enemies in the target area will be stunned and have their stamina depleted. After the stun they will be temporarily dazed.",
  "Info1": {
    "Alt": [
      {
        "Key": "StunDelay",
        "Value": 1.7
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "12m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 23.0,
    "DescKey": "#upgrade_mystical_piano_desc",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "DazeDuration",
        "Type": "duration",
        "Value": 2.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_mystical_piano",
  "Name": "Mystical Piano",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1.7
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DazeMoveSpeed": {
      "Key": "DazeMoveSpeed",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": "2m"
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -15,
    "Radius": "3m"
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "mystical piano",
      "name": "Mystical Piano",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_nullification_aura" title="Nullification Burst" -->

## Nullification Burst

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_nullification_aura`
- Snapshot ID: `40414`
- Source-Dokument: `7073`
- Kurzinfo: Nullification Burst aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Nullification Burst`
- Payload Hash: `05c09a52abfc80844245af93ec189298fa06d6e8ecc55cd9237a3545c282a665`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.551799+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "Removes any positive buffs and prevents stamina usage and healing effects on enemies.",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 40
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 300
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "EndRadius",
        "Type": "distance",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 18.0,
    "DescKey": "#upgrade_nullification_aura_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      },
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.465
        },
        "Type": "tech_damage",
        "Value": 250.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_nullification_aura",
  "Name": "Nullification Burst",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DamageHeight": {
      "Key": "DamageHeight",
      "Value": "4m"
    },
    "SpreadDuration": {
      "Key": "SpreadDuration",
      "Type": "duration",
      "Value": 0.5
    },
    "StartRadius": {
      "Key": "StartRadius",
      "Type": "distance",
      "Value": "2m"
    }
  },
  "ShopFilters": [
    "Disruption"
  ],
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "BonusHealth": 300,
    "Damage": 200,
    "EndRadius": "6m",
    "StatusResistancePercent": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "nullification burst",
      "name": "Nullification Burst",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_omnicharge_pendant" title="Omnicharge Signet" -->

## Omnicharge Signet

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_omnicharge_pendant`
- Snapshot ID: `40415`
- Source-Dokument: `7073`
- Kurzinfo: Omnicharge Signet aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Omnicharge Signet`
- Payload Hash: `9572460ee55bab927f7cc08fffdbd66b2394831a6006f9c8046176e7de836207`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.553409+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Imbue any active non-ultimate ability with <span class=\"highlight\">Bonus Ability Charges</span>. Already charged abilities receive more bonus charges.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSpiritForChargedAbilities",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "CooldownBetweenChargeReduction",
        "Type": "cooldown",
        "Value": 70
      },
      {
        "Key": "CooldownReductionOnChargedAbilities",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_omnicharge_pendant_desc",
    "Main": [
      {
        "Key": "BonusAbilityCharges",
        "LocTokenOverride": "OmniPendantBonusAbilityCharges",
        "Type": "cast",
        "Value": 4
      },
      {
        "Key": "BonusAbilityChargesNonCharge",
        "Type": "cast",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_omnicharge_pendant",
  "Name": "Omnicharge Signet",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "EnableAbilityCharges": {
      "Key": "EnableAbilityCharges",
      "Value": 1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BonusAbilityCharges": 2,
    "BonusSpiritForChargedAbilities": 30,
    "CooldownBetweenChargeReduction": 5,
    "CooldownReductionOnChargedAbilities": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "omnicharge signet",
      "name": "Omnicharge Signet",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_pristine_emblem" title="Opening Rounds" -->

## Opening Rounds

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_pristine_emblem`
- Snapshot ID: `40421`
- Source-Dokument: `7073`
- Kurzinfo: Opening Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Opening Rounds`
- Payload Hash: `733a12e9388d6027a441bbdaf67e36ab8e5dc2d5881f2c2db2b305f8826891d0`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.563111+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 1600,
  "Description": "Your attacks have additional <span class=\"highlight\">Weapon Damage</span> against <span class=\"highlight\">enemies above 50% health</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 60
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 8
      },
      {
        "Key": "TechPower",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_pristine_emblem_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercentBonus",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_pristine_emblem",
  "Name": "Opening Rounds",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "EnemyLifeThreshold": {
      "Key": "EnemyLifeThreshold",
      "Value": 50
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BaseAttackDamagePercentBonus": 25,
    "BonusBulletSpeedPercent": 45,
    "TechPower": 18
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "opening rounds",
      "name": "Opening Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_heal_on_level" title="Patron's Healing" -->

## Patron's Healing

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_heal_on_level`
- Snapshot ID: `40362`
- Source-Dokument: `7073`
- Kurzinfo: Patron's Healing aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Patron's Healing`
- Payload Hash: `b1dc8670e00ee3bc8f7fdc52ff9932bb519646a335a4129b6dc4a7471744cabd`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.464064+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "When you receive a <span class=\"highlight\">Boon</span>, automatically <span class=\"highlight\">heal</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 2.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_heal_on_level_desc",
    "Main": [
      {
        "Key": "HealOnLevelHealAmount",
        "Type": "healing",
        "Value": 210
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_heal_on_level",
  "Name": "Patron's Healing",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AuraRadius": {
      "Key": "AuraRadius",
      "Type": "distance",
      "Value": "15m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": "1.3m"
    }
  },
  "ShopFilters": [
    "Durability",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_phantom_strike" title="Phantom Strike" -->

## Phantom Strike

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_phantom_strike`
- Snapshot ID: `40418`
- Source-Dokument: `7073`
- Kurzinfo: Phantom Strike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Phantom Strike`
- Payload Hash: `f1e312cf75061c51886131af1ca950551a09e138928f9bb64eb89d40d9e2311e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.558616+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Teleport</span> to an enemy target and pull them to the ground. Dealing <span class=\"highlight\">damage</span>, <span class=\"highlight\">Move speed</span> reduction and <span class=\"highlight\">Disarm</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 15
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "25m"
      },
      {
        "Key": "SlowDuration",
        "LocTokenOverride": "PhantomStrikeDebuffDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_phantom_strike_desc",
    "Main": [
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50
      },
      {
        "Key": "ImpactDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.93
        },
        "Type": "tech_damage",
        "Value": 75.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_phantom_strike",
  "Name": "Phantom Strike",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.35
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": "1.3m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Movement",
    "Disruption"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -20,
    "BaseAttackDamagePercent": 20,
    "ImpactDamage": 100,
    "TechPower": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "phantom strike",
      "name": "Phantom Strike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="cosmetic_item_voting_poster" title="Place Poster" -->

## Place Poster

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `cosmetic_item_voting_poster`
- Snapshot ID: `40242`
- Source-Dokument: `7073`
- Kurzinfo: Place Poster aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Place Poster`
- Payload Hash: `dcf7662c63e4408d958b2523e798cbad78031075b08aa1c3e2e357d2cfde80dc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.257904+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "OnRelease",
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "cosmetic_item_voting_poster",
  "Name": "Place Poster",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.15
    },
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": 550
    },
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 0.5
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Radius": {
      "Key": "Radius",
      "Value": 5
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": null,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "place poster",
      "name": "Place Poster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_deflecting_armor" title="Plated Armor" -->

## Plated Armor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_deflecting_armor`
- Snapshot ID: `40325`
- Source-Dokument: `7073`
- Kurzinfo: Plated Armor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Plated Armor`
- Payload Hash: `8718941bc6ea9595dc9f207aa4af24b6f8d64b7d59b4487333854fc78b585be5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.396188+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Gain a chance to either deflect incoming bullets, preventing all {g:citadel_inline_attribute:'WeaponDamage'} or prevent all <span class=\"highlight\">on-hit effects</span> from bullets.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 130
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_deflecting_armor_desc",
    "Main": [
      {
        "Key": "DeflectionPercent",
        "Type": "bullet_armor_up",
        "Value": 30
      },
      {
        "Key": "BulletProcDeflectionPercent",
        "Type": "bullet_armor_up",
        "Value": 50
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_deflecting_armor",
  "Name": "Plated Armor",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DeflectionRandomness": {
      "Key": "DeflectionRandomness",
      "Value": 1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BulletProcDeflectionPercent": 15,
    "DeflectionPercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "plated armor",
      "name": "Plated Armor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_close_quarter_combat" title="Point Blank" -->

## Point Blank

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_close_quarter_combat`
- Snapshot ID: `40312`
- Source-Dokument: `7073`
- Kurzinfo: Point Blank aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Point Blank`
- Payload Hash: `0ce6e0f1721046d8eb3450c4f22eb3312cee8dfb8b2e9aa0dca4509b3068046e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.368951+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_close_range"
  ],
  "Cost": 3200,
  "Description": "When in <span class=\"highlight\">close range</span> to your target, gain <span class=\"highlight\">Weapon Damage</span> and your bullets apply a <span class=\"highlight\">Movement Slow</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "CloseRangeBonusDamageRange",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_close_quarter_combat_desc",
    "Main": [
      {
        "Key": "CloseRangeBonusWeaponPower",
        "Type": "bullet_damage",
        "Value": 50
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_close_quarter_combat",
  "Name": "Point Blank",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 150,
    "CloseRangeBonusWeaponPower": 30,
    "MeleeResistPercent": 30,
    "SlowPercent": 5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "point blank",
      "name": "Point Blank",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_prism_blast" title="Prism Blast" -->

## Prism Blast

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_prism_blast`
- Snapshot ID: `40420`
- Source-Dokument: `7073`
- Kurzinfo: Prism Blast aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Prism Blast`
- Payload Hash: `e7c70cafb7ed65ec8202f095412f46124e218cb6528f762b4b335f6ae396d9c2`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.561291+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "You enter a void state and become <span class=\"highlight\">untargetable and invincible</span> for a short duration, during which lasers blast out and rotate around you.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 40.0,
    "DescKey": "#upgrade_prism_blast_desc",
    "Main": [
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 1.75
        },
        "Type": "tech_damage",
        "Value": 270.0
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "ShiftingVeilDuration",
        "Type": "duration",
        "Value": 6.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_prism_blast",
  "Name": "Prism Blast",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BeamLength": {
      "Key": "BeamLength",
      "Type": "distance",
      "Value": "30m"
    },
    "BeamWidth": {
      "Key": "BeamWidth",
      "Value": "2.9m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DampingFactor": {
      "Key": "DampingFactor",
      "Value": 3
    },
    "FloatMoveSpeed": {
      "Key": "FloatMoveSpeed",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": "2.5m"
    },
    "LiftHeight": {
      "Key": "LiftHeight",
      "Value": 100
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -10,
    "AbilityDuration": 2
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "prism blast",
      "name": "Prism Blast",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_quick_silver" title="Quicksilver Reload" -->

## Quicksilver Reload

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_quick_silver`
- Snapshot ID: `40426`
- Source-Dokument: `7073`
- Kurzinfo: Quicksilver Reload aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Quicksilver Reload`
- Payload Hash: `530ff391e994a19f64963989b62ade159ef711822547d873d52550440a234a7c`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.570645+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use.",
  "Info1": {
    "Alt": [],
    "ChargeUp": 18,
    "Cooldown": null,
    "DescKey": "#upgrade_quick_silver_desc",
    "Main": [
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Type": "tech_damage",
        "Value": 44.0
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 10
      },
      {
        "Key": "AmmoReloadPercent",
        "Type": "clipsize",
        "Value": 100
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_quick_silver",
  "Name": "Quicksilver Reload",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 18
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuffDuration": {
      "Key": "BuffDuration",
      "Value": 12
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityChargeUpTime": -4,
    "AbilityCooldown": -4,
    "BonusFireRate": 20,
    "Damage": 56
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "quicksilver reload",
      "name": "Quicksilver Reload",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_resonant_healing" title="Radiant Regeneration" -->

## Radiant Regeneration

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_resonant_healing`
- Snapshot ID: `40438`
- Source-Dokument: `7073`
- Kurzinfo: Radiant Regeneration aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Radiant Regeneration`
- Payload Hash: `1d877d8ee29361fc16c9c98623c956017cedf4a605d687205268adbe09a7a1ba`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.591070+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_mystic_regeneration"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> and gain bonus <span class=\"highlight\">Movement Speed</span> for a short duration when you cast an ability.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "RegenerationDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_mystic_regeneration_desc",
    "Main": [
      {
        "Key": "Regeneration",
        "Scale": {
          "Type": "spirit",
          "Value": 0.04
        },
        "Type": "healing",
        "Value": 4
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 6,
    "DescKey": "#upgrade_resonant_healing_desc",
    "Main": [
      {
        "Key": "HealingPerCast",
        "Scale": {
          "Type": "power_increase",
          "Value": 2.0
        },
        "Type": "healing",
        "Value": 70
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": "1.75m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_resonant_healing",
  "Name": "Radiant Regeneration",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 110,
    "BonusMoveSpeed": "1m",
    "HealingPerCast": 60,
    "Regeneration": 9
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "radiant regeneration",
      "name": "Radiant Regeneration",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rapid_recharge" title="Rapid Recharge" -->

## Rapid Recharge

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rapid_recharge`
- Snapshot ID: `40427`
- Source-Dokument: `7073`
- Kurzinfo: Rapid Recharge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rapid Recharge`
- Payload Hash: `e00e77fa6cf9afc58b74610bbad2eeae1b88df6107498ad732dec9052d683710`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.572632+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_extra_charge"
  ],
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReductionOnChargedAbilities",
        "Value": 14
      },
      {
        "Key": "BonusSpiritForChargedAbilities",
        "Value": 14
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusAbilityCharges",
        "Type": "cast",
        "Value": 2
      },
      {
        "Key": "CooldownBetweenChargeReduction",
        "Type": "cooldown",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rapid_recharge",
  "Name": "Rapid Recharge",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusAbilityCharges": 2,
    "BonusSpiritForChargedAbilities": 20,
    "CooldownBetweenChargeReduction": 5,
    "CooldownReductionOnChargedAbilities": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "rapid recharge",
      "name": "Rapid Recharge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rapid_rounds" title="Rapid Rounds" -->

## Rapid Rounds

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rapid_rounds`
- Snapshot ID: `40428`
- Source-Dokument: `7073`
- Kurzinfo: Rapid Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rapid Rounds`
- Payload Hash: `0fc3c4b5393dd5d97e648fae4241476040cf9ed82464f21c46bb2f7bf9663ed5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.574352+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Value": 9
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rapid_rounds",
  "Name": "Rapid Rounds",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusFireRate": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "rapid rounds",
      "name": "Rapid Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_vex_barrier" title="Reactive Barrier" -->

## Reactive Barrier

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_vex_barrier`
- Snapshot ID: `40496`
- Source-Dokument: `7073`
- Kurzinfo: Reactive Barrier aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Reactive Barrier`
- Payload Hash: `5586dab38e9349ed6963d2b982e78c49511565de120a2eb2fbe330a62c5eadca`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.713869+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> when you are <span class=\"highlight\">Stunned, Chained, Immobilized, Slept or Silenced</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 55,
    "DescKey": "#upgrade_vex_barrier_desc",
    "Main": [
      {
        "Key": "VexBarrierCombatBarrier",
        "LocTokenOverride": "CombatBarrier",
        "Scale": {
          "Type": "spirit",
          "Value": 1.8
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 325.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_vex_barrier",
  "Name": "Reactive Barrier",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -15,
    "VexBarrierCombatBarrier": 375
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "reactive barrier",
      "name": "Reactive Barrier",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_personal_rejuvenator" title="Rebirth" -->

## Rebirth

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_personal_rejuvenator`
- Snapshot ID: `40417`
- Source-Dokument: `7073`
- Kurzinfo: Rebirth aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rebirth`
- Payload Hash: `6593b1a9451ab31db913a065a9a99c2c4af24db3027fa10bc7a8fcf85609f482`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.556856+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Respawns the hero at the spot they died.",
  "Info1": {
    "Alt": [
      {
        "Key": "RespawnDelay",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 509.0,
    "DescKey": "#upgrade_personal_rejuvenator_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_personal_rejuvenator",
  "Name": "Rebirth",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_melee_rebuttal" title="Rebuttal" -->

## Rebuttal

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_melee_rebuttal`
- Snapshot ID: `40405`
- Source-Dokument: `7073`
- Kurzinfo: Rebuttal aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rebuttal`
- Payload Hash: `6721b88d9b1e0d1eb222262962d0c12bc2afa1940860343bca4673bd9f104d28`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.535786+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "On a successful <span class=\"highlight\">Parry</span> against an enemy Hero, <span class=\"highlight\">Heal</span> yourself for the damage parried and returns that damage to the target, and temporarily gain increased <span class=\"highlight\">damage.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "ParryCooldownReduction",
        "Type": "cooldown",
        "Value": 1.75
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 18
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_melee_rebuttal_desc",
    "Main": [
      {
        "Key": "BonusDamagePercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_melee_rebuttal",
  "Name": "Rebuttal",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ParrySuccessHealPercentage": {
      "Key": "ParrySuccessHealPercentage",
      "Type": "healing",
      "Value": 100
    }
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "BonusDamagePercent": 20,
    "BonusHealth": 150,
    "MeleeResistPercent": 22,
    "ParryCooldownReduction": 0.5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "rebuttal",
      "name": "Rebuttal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rechargingbullets" title="Recharging Rush" -->

## Recharging Rush

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rechargingbullets`
- Snapshot ID: `40430`
- Source-Dokument: `7073`
- Kurzinfo: Recharging Rush aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Recharging Rush`
- Payload Hash: `00fa9e1390fb60f4970d5ce0c49feb2bea10e51083016c1544be07a05034f2ec`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.578188+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Dealing significant {g:citadel_inline_attribute:'WeaponDamage'} replenishes a charge for <span class=\"highlight\">each of your charged abilities</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageWindow",
        "Value": 3.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25,
    "DescKey": "#upgrade_rechargingbullets_desc",
    "Main": [
      {
        "Key": "DamageThreshold",
        "Value": 200
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rechargingbullets",
  "Name": "Recharging Rush",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -12,
    "BaseAttackDamagePercent": 30,
    "BonusClipSizePercent": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "recharging rush",
      "name": "Recharging Rush",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ability_refresher" title="Refresher" -->

## Refresher

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ability_refresher`
- Snapshot ID: `40256`
- Source-Dokument: `7073`
- Kurzinfo: Refresher aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Refresher`
- Payload Hash: `a98d4b7aa2fb195f8ce044ef7c581e1ad0beef2a9162cde5a55115ec46e530df`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.277206+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Reset the cooldown</span> of all your abilities and <span class=\"highlight\">restore all your charges</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 300,
    "DescKey": "#upgrade_ability_refresher_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ability_refresher",
  "Name": "Refresher",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.6
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BulletResist": {
      "Key": "BulletResist",
      "Type": "bullet_armor_up",
      "Value": 15
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechResist": {
      "Key": "TechResist",
      "Type": "tech_armor_up",
      "Value": 14
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -210
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "refresher",
      "name": "Refresher",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rescue_beam" title="Rescue Beam" -->

## Rescue Beam

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rescue_beam`
- Snapshot ID: `40436`
- Source-Dokument: `7073`
- Kurzinfo: Rescue Beam aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rescue Beam`
- Payload Hash: `693d0b1aa42401afcac3a90b68ca4c9b635c38c526fac263b5674341ac758020`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.587637+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heals</span> a target allied hero and yourself for a percentage of <span class=\"highlight\">Max Health</span>. Once while healing, you can <span class=\"highlight\">Pull</span> the target towards you. Can be self-cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityChannelTime",
        "Type": "cast",
        "Value": 2.5
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "35m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60.0,
    "DescKey": "#upgrade_rescue_beam_desc",
    "Main": [
      {
        "Key": "HealPercentAmount",
        "Type": "healing",
        "Value": 20
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "0m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rescue_beam",
  "Name": "Rescue Beam",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealInterval": {
      "Key": "HealInterval",
      "Value": 0.2
    },
    "SelfModifier": {
      "Key": "SelfModifier",
      "Value": 100
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 6
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -45,
    "HealPercentAmount": 15,
    "TechRadiusMultiplier": 20,
    "TechRangeMultiplier": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "rescue beam",
      "name": "Rescue Beam",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_restorative_locket" title="Restorative Locket" -->

## Restorative Locket

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_restorative_locket`
- Snapshot ID: `40439`
- Source-Dokument: `7073`
- Kurzinfo: Restorative Locket aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Restorative Locket`
- Payload Hash: `25e63346800df150aed95d66abb02daaeccab395db9475e95018525ed496ff51`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.592943+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MaxStacks",
        "Value": 25
      },
      {
        "Key": "MaxStaminaRestore",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 20.0,
    "DescKey": "#upgrade_restorative_locket_active_desc",
    "Main": [
      {
        "Key": "HealPerStack",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.32
        },
        "Type": "healing",
        "Value": 16
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_restorative_locket",
  "Name": "Restorative Locket",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "35m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "35m"
    }
  },
  "ShopFilters": [
    "Healing",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -8,
    "HealPerStack": 30,
    "MaxStaminaRestore": 2,
    "MinStaminaRestore": 2,
    "TechResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "restorative locket",
      "name": "Restorative Locket",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_medic_bullets" title="Restorative Shot" -->

## Restorative Shot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_medic_bullets`
- Snapshot ID: `40403`
- Source-Dokument: `7073`
- Kurzinfo: Restorative Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Restorative Shot`
- Payload Hash: `69e80efb64ec629033563c1e4b05d3dc83e92e0323b8f1a35f0fb94f28616855`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.532551+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "Your next bullet will <span class=\"highlight\">heal</span> you based on what target you hit.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 6,
    "DescKey": "#upgrade_medic_bullets_desc",
    "Main": [
      {
        "Key": "HealFromHero",
        "Type": "healing",
        "Value": 50
      },
      {
        "Key": "HealFromNPC",
        "Type": "healing",
        "Value": 20
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_medic_bullets",
  "Name": "Restorative Shot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "HealFromHero": 100,
    "HealFromNPC": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "restorative shot",
      "name": "Restorative Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_return_fire" title="Return Fire" -->

## Return Fire

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_return_fire`
- Snapshot ID: `40440`
- Source-Dokument: `7073`
- Kurzinfo: Return Fire aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Return Fire`
- Payload Hash: `f01343147c80534e03ed9084e3d44a351792711e2af51c963efcbb54bc5d72f4`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.594762+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": "Automatically <span class=\"highlight\">fire a bullet</span> towards any attacker who damages you with their abilities or weapon.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 6.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 23.0,
    "DescKey": "#upgrade_return_fire_desc",
    "Main": [
      {
        "Key": "BulletDamageReflectedPct",
        "Type": "bullet_damage",
        "Value": 65
      },
      {
        "Key": "SpiritDamageReflectedPct",
        "Type": "tech_damage",
        "Value": 25
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_return_fire",
  "Name": "Return Fire",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability",
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -10,
    "BulletDamageReflectedPct": 25,
    "BulletResist": 16,
    "SpiritDamageReflectedPct": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "return fire",
      "name": "Return Fire",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_ricochet" title="Ricochet" -->

## Ricochet

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ricochet`
- Snapshot ID: `40441`
- Source-Dokument: `7073`
- Kurzinfo: Ricochet aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ricochet`
- Payload Hash: `e2be0653810dbd4ce1be5b9035e070aa3aa99c59f69cc1c3ab0c0332e926c8dd`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.596637+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "RicochetTargetsTooltipOnly",
        "Value": 2
      },
      {
        "Key": "RicochetRadius",
        "Type": "distance",
        "Value": "13m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_ricochet_desc",
    "Main": [
      {
        "Key": "RicochetDamagePercent",
        "Type": "bullet_damage",
        "Value": 65
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ricochet",
  "Name": "Ricochet",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusFireRate": 25,
    "RicochetDamagePercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "ricochet",
      "name": "Ricochet",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_runed_gauntlets" title="Runed Gauntlets" -->

## Runed Gauntlets

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_runed_gauntlets`
- Snapshot ID: `40444`
- Source-Dokument: `7073`
- Kurzinfo: Runed Gauntlets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Runed Gauntlets`
- Payload Hash: `47994bb6eae4301fa80654b86058ace5eabfde0c0641d05b4e2a4a2d097c9d0c`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.602350+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Everytime you land a heavy melee, your existing cooldowns get reduced.",
  "Info1": {
    "Alt": [
      {
        "Key": "MeleeResistPercent",
        "Value": 50
      },
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "MeleeDistanceScale",
        "Value": 150
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 10,
    "DescKey": "#upgrade_runed_gauntlets_parry_desc",
    "Main": [],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_runed_gauntlets_desc",
    "Main": [
      {
        "Key": "CooldownReductionOnHitPct",
        "Type": "cooldown",
        "Value": 16
      },
      {
        "Key": "CooldownReductionOnHitMin",
        "Type": "cooldown",
        "Value": 4
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_runed_gauntlets",
  "Name": "Runed Gauntlets",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "CooldownReductionOnHitPct": 6,
    "MeleeDistanceScale": 50,
    "MeleeResistPercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "runed gauntlets",
      "name": "Runed Gauntlets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_withering_whip" title="Rusted Barrel" -->

## Rusted Barrel

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_withering_whip`
- Snapshot ID: `40506`
- Source-Dokument: `7073`
- Kurzinfo: Rusted Barrel aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rusted Barrel`
- Payload Hash: `9b2385bf3a06deef212bccb40bb5fc299b14ac55d19552487d046f5091fa084e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.730362+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 800,
  "Description": "Target an enemy to reduce their <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Bullet Resistance</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 60
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "32m"
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "WitheringWhipDisarmDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_withering_whip_desc",
    "Main": [
      {
        "Key": "FireRateSlow",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyEnemyApplied",
        "Value": 32
      },
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_withering_whip",
  "Name": "Rusted Barrel",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 130,
    "BulletArmorReduction": -4,
    "FireRateSlow": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "rusted barrel",
      "name": "Rusted Barrel",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_discord" title="Scourge" -->

## Scourge

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_discord`
- Snapshot ID: `40327`
- Source-Dokument: `7073`
- Kurzinfo: Scourge aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Scourge`
- Payload Hash: `812f0b9a1e804150481512c00b767be11330970a8cc0d4df10fb30912a031ef6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.399613+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": "Apply <span class=\"highlight\">Spirit Resist</span> and an aura on a friendly target that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span> to enemies proportional to their max health. <br>Can be self cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 17
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "35m"
      },
      {
        "Key": "AuraRadius",
        "Type": "distance",
        "Value": "10m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_discord_desc",
    "Main": [
      {
        "Key": "MaxHealthPercentAsDPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0055
        },
        "Type": "tech_damage",
        "Value": 2.6
      },
      {
        "Key": "TechResist",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_discord",
  "Name": "Scourge",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.25
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 125,
    "CombatBarrier": 300,
    "MaxHealthPercentAsDPS": 2,
    "StatusResistancePercent": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "scourge",
      "name": "Scourge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_icarus_wings" title="Seraphim Wings" -->

## Seraphim Wings

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_icarus_wings`
- Snapshot ID: `40376`
- Source-Dokument: `7073`
- Kurzinfo: Seraphim Wings aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Seraphim Wings`
- Payload Hash: `4783927190797b29541985751a3c35f5c9947f8eea06cbc1f782cda2619cbb75`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.487326+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "When Airborne, You deal more damage and take reduced damage. Allows <span class=\"highlight\">unlimited</span> Air Dash and Jumping.",
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaCooldownReduction",
        "Value": 120
      },
      {
        "Key": "GravityScale",
        "Value": -70
      },
      {
        "Key": "AirControlPercent",
        "Type": "move_speed",
        "Value": 100
      },
      {
        "Key": "AirControlAccelPercent",
        "Type": "move_speed",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_icarus_wings_desc",
    "Main": [
      {
        "Key": "AirBonusDamageGiven",
        "Value": 40
      },
      {
        "Key": "AirBonusDamageTaken",
        "Value": -40
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_icarus_wings",
  "Name": "Seraphim Wings",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "AirBonusDamageGiven": 10,
    "AirBonusDamageTaken": -10,
    "StaminaCooldownReduction": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "seraphim wings",
      "name": "Seraphim Wings",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_shadow_step" title="Shadow Step" -->

## Shadow Step

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shadow_step`
- Snapshot ID: `40447`
- Source-Dokument: `7073`
- Kurzinfo: Shadow Step aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Step`
- Payload Hash: `f84c4a25f850b7da3026bcc3ad171a7499b94100e6ebdf4038fca28ab45fc433`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.607548+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "<span class=\"highlight\">Teleport</span> straight ahead.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 3.0,
    "DescKey": "#upgrade_shadow_step_desc",
    "Main": [
      {
        "Key": "AbilityCastRange",
        "LocTokenOverride": "WarpStoneRange",
        "Type": "range",
        "Value": "12m"
      }
    ],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamagePulseInterval",
        "Value": 0.5
      },
      {
        "Key": "DamagePulseRadius",
        "Type": "distance",
        "Value": "13m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_shadow_step_passive_desc",
    "Main": [
      {
        "Key": "DamagePulseAmount",
        "Type": "tech_damage",
        "Value": 50
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_shadow_step",
  "Name": "Shadow Step",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_shadow_strike" title="Shadow Strike" -->

## Shadow Strike

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shadow_strike`
- Snapshot ID: `40448`
- Source-Dokument: `7073`
- Kurzinfo: Shadow Strike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Strike`
- Payload Hash: `e9cf0fda828751fa3f7965b2d2a604aefe226f9b671388491ea7f5feb3695ca3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.609253+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Go <span class=\"highlight\">Invisible</span> on <span class=\"highlight\">Stamina use</span> with no detection range. Doing a <span class=\"highlight\">melee attack</span> while invisible will cause you to <span class=\"highlight\">steal bullet and spirit resistance</span> from them and deal <span class=\"highlight\">damage over time</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "Stamina",
        "Value": 3
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 350
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "InvisFadeToDuration",
        "Type": "duration",
        "Value": 0.2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_shadow_strike_desc",
    "Main": [
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Type": "tech_damage",
        "Value": 125.0
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "InvisDuration",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "StealDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "ResistStealAmount",
        "Type": "tech_armor_up",
        "Value": 40
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shadow_strike",
  "Name": "Shadow Strike",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 1
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "SpottedRadius": {
      "Key": "SpottedRadius",
      "Type": "distance",
      "Value": "0m"
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 250,
    "DPS": 125,
    "ResistStealAmount": 20,
    "Stamina": 1
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "shadow strike",
      "name": "Shadow Strike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cloaking_device_active" title="Shadow Weave" -->

## Shadow Weave

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloaking_device_active`
- Snapshot ID: `40311`
- Source-Dokument: `7073`
- Kurzinfo: Shadow Weave aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Weave`
- Payload Hash: `736169e4977c9832635c8437b0476dbd976be44f72c38430c9a7019ecdf6c852`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.367113+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Become <span class=\"highlight\">Stealthed</span>. Whenever you take damage while Stealthed you get briefly revealed.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 5
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SpottedRadius",
        "Value": "20m"
      },
      {
        "Key": "InvisMoveSpeedMod",
        "Type": "move_speed",
        "Value": "5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45.0,
    "DescKey": "#upgrade_cloaking_device_active_desc",
    "Main": [
      {
        "Key": "StatusEffectInvisible",
        "Value": null
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "StealthDuration",
        "Type": "duration",
        "Value": 13
      }
    ],
    "Type": "Active"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "AmbushDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_cloaking_device_active_ambush_desc",
    "Main": [
      {
        "Key": "AmbushBonusFireRate",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "AmbushBonusTechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "AmbushBonusMeleeDamage",
        "Type": "melee_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cloaking_device_active",
  "Name": "Shadow Weave",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "FullInvisDistance": {
      "Key": "FullInvisDistance",
      "Value": "30m"
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisCancelOnDamage": {
      "Key": "InvisCancelOnDamage",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 0.6
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 1.5
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -17,
    "AmbushBonusFireRate": 35,
    "AmbushBonusMeleeDamage": 30,
    "AmbushBonusTechPower": 35,
    "OutOfCombatHealthRegen": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "shadow weave",
      "name": "Shadow Weave",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_sharpshooter" title="Sharpshooter" -->

## Sharpshooter

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_sharpshooter`
- Snapshot ID: `40449`
- Source-Dokument: `7073`
- Kurzinfo: Sharpshooter aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sharpshooter`
- Payload Hash: `2471e40b2f6cf5a0bc92668b92f4d81c0479d76a5986b6e2548900fa70f4dc67`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.611143+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_long_range",
    "upgrade_high_velocity_mag"
  ],
  "Cost": 3200,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when <span class=\"highlight\">beyond a minimum distance</span> from your target.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Type": "bullet_damage",
        "Value": 60
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 10
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1.0m"
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "-0.7m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusAttackRangePercent",
        "Type": "distance",
        "Value": 20
      },
      {
        "Key": "BonusZoomPercent",
        "Type": "distance",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "LongRangeBonusWeaponPowerMinRange",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_sharpshooter_desc",
    "Main": [
      {
        "Key": "LongRangeBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 60
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_sharpshooter",
  "Name": "Sharpshooter",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusAttackRangePercent": 10,
    "BonusBulletSpeedPercent": 45,
    "LongRangeBonusWeaponPower": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "sharpshooter",
      "name": "Sharpshooter",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_shrink_ray" title="Shrink Ray" -->

## Shrink Ray

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shrink_ray`
- Snapshot ID: `40451`
- Source-Dokument: `7073`
- Kurzinfo: Shrink Ray aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shrink Ray`
- Payload Hash: `ff5851e300606dd0fef3efb6871c56737b5b442a515c97107cf7450e1d69baeb`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.614704+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 9999,
  "Description": "Reduces <span class=\"highlight\">Model Size</span> and grants <span class=\"highlight\">Move Speed</span> to the target. Allows <span class=\"highlight\">usage of tunnels</span> in this mode. Can be self-cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "ShrinkDuration",
        "Type": "duration",
        "Value": 60
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30,
    "DescKey": "#upgrade_shrink_ray_desc",
    "Main": [
      {
        "Key": "ModelScaleGrowthTooltip",
        "Value": -50
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "5.0m"
      },
      {
        "Key": "BonusFireRate",
        "Value": 20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shrink_ray",
  "Name": "Shrink Ray",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ModelScaleGrowth": {
      "Key": "ModelScaleGrowth",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -15,
    "BonusFireRate": 20,
    "BonusMoveSpeed": "2m",
    "ModelScaleGrowth": -0.15,
    "ModelScaleGrowthTooltip": -15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "shrink ray",
      "name": "Shrink Ray",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_targeted_silence" title="Silence Wave" -->

## Silence Wave

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_targeted_silence`
- Snapshot ID: `40476`
- Source-Dokument: `7073`
- Kurzinfo: Silence Wave aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silence Wave`
- Payload Hash: `8a46d7689a3ffc075a7d58267a3a07a9ece88df36a6554a594a87a6367e1b1ff`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.676688+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Launch an expanding projectile which <span class=\"highlight\">Silences</span> enemies for a short duration and deals impact damage. <br><br><span class=\"diminish\">Silence does not interrupt channeling abilities.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "#SilenceDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 42.0,
    "DescKey": "#upgrade_targeted_silence_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Type": "tech_damage",
        "Value": 75.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_targeted_silence",
  "Name": "Silence Wave",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownOnMiss": {
      "Key": "CooldownOnMiss",
      "Value": 30.0
    },
    "GrowthPerMeter": {
      "Key": "GrowthPerMeter",
      "Value": "0.15m"
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "1m"
    },
    "InitialWidth": {
      "Key": "InitialWidth",
      "Value": "5.0m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 75,
    "Damage": 125
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "silence wave",
      "name": "Silence Wave",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_proc_silence" title="Silencer" -->

## Silencer

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_proc_silence`
- Snapshot ID: `40423`
- Source-Dokument: `7073`
- Kurzinfo: Silencer aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silencer`
- Payload Hash: `09119130c0a53e725dc6ab56e8ec8f1fa4584121a742cc38df51f39ef31151c4`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.565986+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets build up to a <span class=\"highlight\">Silence</span>. Victims are immune to the build up for <span class=\"highlight\">10s</span> after silence expires.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_proc_silence_debuff_desc",
    "Main": [
      {
        "Key": "TechDamageReduction",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -25
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "SilenceDuration",
        "Value": 2.5
      },
      {
        "Key": "ImmunityDuration",
        "Value": 10
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 1.04
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_proc_silence_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_proc_silence",
  "Name": "Silencer",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "SilenceDuration": 1.25,
    "TechDamageReduction": -15,
    "TechResist": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "silencer",
      "name": "Silencer",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_silencer" title="Silencer" -->

## Silencer

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_silencer`
- Snapshot ID: `40452`
- Source-Dokument: `7073`
- Kurzinfo: Silencer aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silencer`
- Payload Hash: `6dd4b0fa785cc9ba515f6e50c77828a402bee2eb68ab124bf1c836d454f18915`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.616656+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "EMPDuration",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": 36.0,
    "DescKey": "#upgrade_silencer_active",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_silencer",
  "Name": "Silencer",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 4
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_siphon_bullets" title="Siphon Bullets" -->

## Siphon Bullets

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_siphon_bullets`
- Snapshot ID: `40453`
- Source-Dokument: `7073`
- Kurzinfo: Siphon Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Siphon Bullets`
- Payload Hash: `0b9d380c25e44b12b10f1e53e59f67b22d454cb7b3e6afd7dfafeed0985380d5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.618292+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 15
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StealDuration",
        "Type": "duration",
        "Value": 17
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.2,
    "DescKey": "#upgrade_siphon_bullets_desc_passive2",
    "Main": [
      {
        "Key": "HealthStealPctHero",
        "LocTokenOverride": "SiphonBullets_HealthStealPctHero",
        "Type": "health",
        "Value": 2.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_siphon_bullets",
  "Name": "Siphon Bullets",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 9999
    },
    "ParticleRadius": {
      "Key": "ParticleRadius",
      "Type": "distance",
      "Value": "1m"
    },
    "StackLostPerDeath": {
      "Key": "StackLostPerDeath",
      "Value": 2
    },
    "StealPerHit": {
      "Key": "StealPerHit",
      "Value": 1
    },
    "StealPerKill": {
      "Key": "StealPerKill",
      "Value": 1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BulletResist": 10,
    "HealthStealPctHero": 1.5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "siphon bullets",
      "name": "Siphon Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_slowing_bullets" title="Slowing Bullets" -->

## Slowing Bullets

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_slowing_bullets`
- Snapshot ID: `40455`
- Source-Dokument: `7073`
- Kurzinfo: Slowing Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slowing Bullets`
- Payload Hash: `6e6c14fb50cc7a10c2036695b061b3ad56e909c16137cd0f3ad605e7a7f36715`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.621928+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your bullets build up a <span class=\"highlight\">Movement Slow</span> on enemies.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -22
      },
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 0.7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_slowing_bullets_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_slowing_bullets",
  "Name": "Slowing Bullets",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 30,
    "GroundDashReductionPercent": -10,
    "SlowPercent": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "slowing bullets",
      "name": "Slowing Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_containment" title="Slowing Hex" -->

## Slowing Hex

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_containment`
- Snapshot ID: `40316`
- Source-Dokument: `7073`
- Kurzinfo: Slowing Hex aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slowing Hex`
- Payload Hash: `7a1dc5741d52d58891b6c829121ca6dabe38cbb883beba59271975fe5bf34773`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.377498+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 1600,
  "Description": "<span class=\"highlight\">Slows movement</span> of enemy target. Also <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.<br><span class=\"diminish\">Increases the target's gravity.<br>Does not affect target's stamina usage.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "25m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 3.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 27,
    "DescKey": "#upgrade_containment_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_containment",
  "Name": "Slowing Hex",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Movement",
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -18,
    "GroundDashReductionPercent": -6,
    "SlowPercent": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "slowing hex",
      "name": "Slowing Hex",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="cosmetic_item_snowball" title="Snowball" -->

## Snowball

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `cosmetic_item_snowball`
- Snapshot ID: `40241`
- Source-Dokument: `7073`
- Kurzinfo: Snowball aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Snowball`
- Payload Hash: `8936d1796a3f37db025f4842299c23ec8d9124318b9c0232565374da630de2fb`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.255887+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": null,
  "Description": "Throw snowballs at your friends and enemies. Distance and other properties improve as you make progress in the <span class=\"highlight\">2025 Holiday Challenge</span>.<br><br><span class=\"diminish\">Throwing a Snowball at an ally won't inflict damage and will reset its cooldown.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "SnowballSpeed",
        "Value": 630
      },
      {
        "Key": "SnowballCount",
        "Value": 1
      },
      {
        "Key": "Radius",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 18,
    "DescKey": "#cosmetic_item_snowball_desc",
    "Main": [
      {
        "Key": "Damage",
        "Value": 1
      },
      {
        "Key": "AbilityCharges",
        "Type": "cast",
        "Value": 1
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "cosmetic_item_snowball",
  "Name": "Snowball",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": 1.5
    },
    "AbilityPostCastDuration": {
      "Key": "AbilityPostCastDuration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MaxLevel": {
      "Key": "MaxLevel",
      "Value": 32
    },
    "Progression": {
      "Charges": {
        "Behavior": "UsePrevious",
        "Levels": {
          "0": 1.0,
          "16": 3.0,
          "24": 4.0,
          "32": 5.0,
          "8": 2.0
        }
      },
      "Cooldown": {
        "Levels": {
          "0": 18.0,
          "1": 16.0,
          "2": 14.0,
          "3": 12.0,
          "4": 10.0,
          "5": 8.0
        }
      },
      "Damage": {
        "Levels": {
          "0": 1.0,
          "1": 2.0,
          "2": 4.0,
          "3": 8.0,
          "32": 20.0,
          "4": 12.0
        }
      },
      "Key": "Progression",
      "Radius": {
        "Levels": {
          "0": 5.0,
          "32": 20.0
        }
      },
      "SnowballCount": {
        "Behavior": "UsePrevious",
        "Levels": {
          "0": 1.0,
          "16": 2.0,
          "32": 3.0
        }
      },
      "Speed": {
        "Levels": {
          "0": 630.0,
          "1": 787.0,
          "2": 944.0,
          "3": 1102.0,
          "32": 2756.0,
          "4": 1260.0,
          "5": 1417.0,
          "6": 1575.0
        }
      }
    },
    "Spread": {
      "Key": "Spread",
      "Value": 3
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": null,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly",
    "AllEnemy"
  ],
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "snowball",
      "name": "Snowball",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_corpse_explosion" title="Soul Explosion" -->

## Soul Explosion

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_corpse_explosion`
- Snapshot ID: `40318`
- Source-Dokument: `7073`
- Kurzinfo: Soul Explosion aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Soul Explosion`
- Payload Hash: `8cafc7ad44c33cddcde0eaa1a00a4c93a2f1eb1259bb34852fd06b179e654fb7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.381924+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Kills or assists cause an <span class=\"highlight\">explosion</span> where the victim dies. Kills against heroes have greater radius and damage.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 110
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 3
      },
      {
        "Key": "TechPower",
        "Value": 6
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "HeroMultiplier",
        "Value": 150
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_corpse_explosion_desc",
    "Main": [
      {
        "Key": "ExplosionDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.93
        },
        "Type": "tech_damage",
        "Value": 125
      },
      {
        "Key": "ExplosionRadius",
        "Value": "4m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_corpse_explosion",
  "Name": "Soul Explosion",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ArmingTime": {
      "Key": "ArmingTime",
      "Value": 0.1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_rebirth" title="Soul Rebirth" -->

## Soul Rebirth

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rebirth`
- Snapshot ID: `40429`
- Source-Dokument: `7073`
- Kurzinfo: Soul Rebirth aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Soul Rebirth`
- Payload Hash: `c3531c592b4ba2624508696ea1d9d40711b99b014c3c23b3769df540a6661f18`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.576374+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SpawnTimePenalty",
        "Type": "duration",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 240.0,
    "DescKey": "#upgrade_rebirth_passive",
    "Main": [
      {
        "Key": "RespawnHealthPercent",
        "Type": "healing",
        "Value": 50
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_rebirth",
  "Name": "Soul Rebirth",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "RespawnDelay": {
      "Key": "RespawnDelay",
      "Type": "duration",
      "Value": 4
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spellbreaker" title="Spellbreaker" -->

## Spellbreaker

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellbreaker`
- Snapshot ID: `40459`
- Source-Dokument: `7073`
- Kurzinfo: Spellbreaker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spellbreaker`
- Payload Hash: `58329bb5856513e6097f734a4e387449149db2357207d10f7162cdf7926b5788`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.629004+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "Description": "The next instance of high {g:citadel_inline_attribute:'SpiritDamage'} you take is significantly reduced.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 18
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 25
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_damage",
        "Value": 175
      }
    ],
    "ChargeUp": null,
    "Cooldown": 9,
    "DescKey": "#upgrade_spellbreaker_desc",
    "Main": [
      {
        "Key": "SpiritDamageReductionProc",
        "Type": "tech_armor_up",
        "Value": 65
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellbreaker",
  "Name": "Spellbreaker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -3,
    "StatusResistancePercent": 15,
    "TechResist": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spellbreaker",
      "name": "Spellbreaker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_enchanted_holsters" title="Spellslinger" -->

## Spellslinger

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_enchanted_holsters`
- Snapshot ID: `40336`
- Source-Dokument: `7073`
- Kurzinfo: Spellslinger aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spellslinger`
- Payload Hash: `dbaa85f13ad7c66f14120623f9b39d5cbadded59ea376dfb401138c35a7b763d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.416732+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "While in-combat whenever you cast an ability or item, gain a stacking buff that improves fire rate and reload speed. <br><span class=\"diminish\">Each stack refreshes the duration.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MaxStacks",
        "Value": 6
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 18
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_enchanted_holsters_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 11
      },
      {
        "Key": "ReloadSpeedMultipler",
        "Type": "reload_speed",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": -10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_enchanted_holsters",
  "Name": "Spellslinger",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusFireRate": 6,
    "CooldownReduction": 8,
    "ReloadSpeedMultipler": -3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spellslinger",
      "name": "Spellslinger",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_armor" title="Spirit Armor" -->

## Spirit Armor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_armor`
- Snapshot ID: `40477`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Armor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Armor`
- Payload Hash: `d8cbb6209cf7e5a814f30384b754d2ec30a2c06953d555fc2a4bb6ad5487b49f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.678754+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 15
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_tech_armor",
  "Name": "Spirit Armor",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spirit_burn" title="Spirit Burn" -->

## Spirit Burn

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_burn`
- Snapshot ID: `40463`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Burn aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Burn`
- Payload Hash: `95a01475532e31ba1252f908e178bf24a4f8235d692f4257183dbe3c9feaa784`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.636033+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Dealing significant {g:citadel_inline_attribute:'SpiritDamage'} to an enemy within 5s causes an explosion dealing damage and a burn to nearby enemies. While burning, enemies take damage over time and receive reduced healing.<br><span class=\"diminish\">Deals half-damage and has half-cooldown on non-heroes.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ExplosionRadius",
        "Value": "12m"
      },
      {
        "Key": "DebuffDuration",
        "Type": "tech_damage",
        "Value": 8
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "UsageFlags": "ConditionallyApplied",
        "Value": -70
      }
    ],
    "ChargeUp": null,
    "Cooldown": 20,
    "DescKey": "#upgrade_spirit_burn_desc",
    "Main": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_armor_up",
        "Value": 500
      },
      {
        "Key": "ExplosionDamage",
        "Type": "tech_damage",
        "Value": 110
      },
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Type": "tech_damage",
        "Value": 24.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_burn",
  "Name": "Spirit Burn",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownReductionPctOnNonHeroes": {
      "Key": "CooldownReductionPctOnNonHeroes",
      "Value": 50
    },
    "DamagePctVsNonHeroes": {
      "Key": "DamagePctVsNonHeroes",
      "Value": 50
    },
    "DamageThresholdDuration": {
      "Key": "DamageThresholdDuration",
      "Value": 5
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": -70
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 6
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -6,
    "DPS": 20,
    "ExplosionDamage": 160,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit burn",
      "name": "Spirit Burn",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health_stealing_magic" title="Spirit Lifesteal" -->

## Spirit Lifesteal

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stealing_magic`
- Snapshot ID: `40371`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Lifesteal aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Lifesteal`
- Payload Hash: `99ede903cb5f30b4264db8ab25a77cfafa02ee7d7d05c5020d6846c712363e20`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.478898+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 13
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stealing_magic",
  "Name": "Spirit Lifesteal",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "NonHeroAbilityLifestealTooltipOnly": {
      "Key": "NonHeroAbilityLifestealTooltipOnly",
      "Value": 3
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityLifestealPercentHero": 14,
    "BonusHealth": 80,
    "TechPower": 9
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit lifesteal",
      "name": "Spirit Lifesteal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spellslinger_headshots" title="Spirit Rend" -->

## Spirit Rend

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellslinger_headshots`
- Snapshot ID: `40461`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Rend aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Rend`
- Payload Hash: `bb22095afbcff6a65fc1f27334684c729e070aabc26309e575f9caa4a6b6183b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.632420+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_tech_defense_shredders"
  ],
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_spellslinger_headshots_part1_desc",
    "Main": [
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      },
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10
      }
    ],
    "Type": null
  },
  "Info3": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "MaxStacks",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": 2,
    "DescKey": "#upgrade_spellslinger_headshots_part2_desc",
    "Main": [
      {
        "Key": "MagicResistReduction",
        "LocTokenOverride": "SpellSlingerHeadshots_SpiritShredPerStack",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -7
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellslinger_headshots",
  "Name": "Spirit Rend",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityLifestealPercentHero": 10,
    "MagicResistReduction": -5,
    "TechArmorDamageReduction": -10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit rend",
      "name": "Spirit Rend",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_purge" title="Spirit Resilience" -->

## Spirit Resilience

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_purge`
- Snapshot ID: `40482`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Resilience aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Resilience`
- Payload Hash: `b8091de73e4619c7fb7e5cddc17d2bb4bf93d6ba1bb7ddc5d80814b5717287e8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.687923+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "When below <span class=\"highlight\">50% health</span>, gain additional Spirit Resist.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_tech_purge_desc",
    "Main": [
      {
        "Key": "HealthThreshold",
        "Value": 50
      },
      {
        "Key": "TechResistBelowThreshold",
        "LocTokenOverride": "#TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_purge",
  "Name": "Spirit Resilience",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "TechResist": 10,
    "TechResistBelowThreshold": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit resilience",
      "name": "Spirit Resilience",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spirit_sap" title="Spirit Sap" -->

## Spirit Sap

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_sap`
- Snapshot ID: `40464`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Sap aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Sap`
- Payload Hash: `d816db86a5ad07e82c3ff81aeeb5eac9875872cdc0a1b884e697dadc93d7009a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.637870+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 1600,
  "Description": "Target an enemy to <span class=\"highlight\">reduce their Spirit Resist</span> and <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 18,
    "DescKey": "#upgrade_spirit_sap_desc",
    "Main": [
      {
        "Key": "MagicResistReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -9
      },
      {
        "Key": "TechPowerReduction",
        "Type": "spirit",
        "Value": -30
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_sap",
  "Name": "Spirit Sap",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -12,
    "BonusHealth": 150,
    "MagicResistReduction": -12,
    "TechPowerReduction": -26
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit sap",
      "name": "Spirit Sap",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spirit_bubble" title="Spirit Shielding" -->

## Spirit Shielding

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_bubble`
- Snapshot ID: `40462`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Shielding aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Shielding`
- Payload Hash: `5aa245d24a7641e44405115fe050fbf312ad2ad7642c42f81008459e35bcc628`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.634401+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'SpiritDamage'} from enemy Heroes in a small time frame.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "bullet_armor_up",
        "Value": 225
      },
      {
        "Key": "DamageWindow",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_spirit_bubble_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Scale": {
          "Type": "power_increase",
          "Value": 5.0
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 300.0
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 18
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_bubble",
  "Name": "Spirit Shielding",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -20,
    "CombatBarrier": 175,
    "OutOfCombatHealthRegen": 3,
    "TechResist": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit shielding",
      "name": "Spirit Shielding",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_defense_shredders" title="Spirit Shredder Bullets" -->

## Spirit Shredder Bullets

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_defense_shredders`
- Snapshot ID: `40480`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Shredder Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Shredder Bullets`
- Payload Hash: `1c73a625e3b695a8f821067ca4c26a0e76fe165f49b8b60c34f18e940f0a0c91`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.683960+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your bullets apply a debuff that reduces the <span class=\"highlight\">Spirit Resist</span> of the target and grants you and your allies <span class=\"highlight\">Spirit Lifesteal</span> against them.",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_tech_defense_shredders_desc",
    "Main": [
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      },
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_defense_shredders",
  "Name": "Spirit Shredder Bullets",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "AbilityLifestealPercentHero": 10,
    "TechArmorDamageReduction": -10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit shredder bullets",
      "name": "Spirit Shredder Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_spirit_snatch" title="Spirit Snatch" -->

## Spirit Snatch

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_snatch`
- Snapshot ID: `40465`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Snatch aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Snatch`
- Payload Hash: `c9a2b9c3d9067b7b678848128ffc0d2c30d8c630e3e43a644eb852989bd404bd`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.650283+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_acolytes_glove"
  ],
  "Cost": 3200,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, the attack deals extra {g:citadel_inline_attribute:'SpiritDamage'} and steals <span class=\"highlight\">Spirit Resist</span> and <span class=\"highlight\">Spirit Power</span>.<span class=\"diminish\"><br><br>Effects are reduced by 30% for Light Melee hits.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 7
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 6,
    "DescKey": "#upgrade_spirit_snatch_desc",
    "Main": [
      {
        "Key": "SpiritDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.84
        },
        "Type": "tech_damage",
        "Value": 50.0
      },
      {
        "Key": "TechArmorGain",
        "LocTokenOverride": "SpiritSnatch_TechArmorSteal",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 12
      },
      {
        "Key": "TechPowerGain",
        "LocTokenOverride": "SpiritSnatch_TechPowerSteal",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_snatch",
  "Name": "Spirit Snatch",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LightMeleeReduction": {
      "Key": "LightMeleeReduction",
      "Value": 30
    },
    "TechArmorDamageReduction": {
      "Key": "TechArmorDamageReduction",
      "Type": "tech_armor_down",
      "UsageFlags": "ConditionallyApplied",
      "Value": -12
    },
    "TechPowerReduction": {
      "Key": "TechPowerReduction",
      "Type": "tech_damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -25
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "SpiritDamage": 50,
    "TechArmorDamageReduction": -5,
    "TechArmorGain": 5,
    "TechPowerGain": 35,
    "TechPowerReduction": -35
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit snatch",
      "name": "Spirit Snatch",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_acolytes_glove" title="Spirit Strike" -->

## Spirit Strike

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_acolytes_glove`
- Snapshot ID: `40258`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Strike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Strike`
- Payload Hash: `a16ca4b520f3589340c1dff0fcb22bdfa90d61546308206f1af056d82f681190`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.280662+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, deal extra {g:citadel_inline_attribute:'SpiritDamage'} with the attack and reduce the target's <span class=\"highlight\">Spirit Resist</span>.<span class=\"diminish\"><br><br>Cooldown is 2x longer for Light Melee hits.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_acolytes_glove_desc",
    "Main": [
      {
        "Key": "SpiritDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.372
        },
        "Type": "tech_damage",
        "Value": 40.0
      },
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -6
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_acolytes_glove",
  "Name": "Spirit Strike",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LightMeleeCooldownMult": {
      "Key": "LightMeleeCooldownMult",
      "Value": 2
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "SpiritDamage": 80,
    "TechArmorDamageReduction": -5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spirit strike",
      "name": "Spirit Strike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_overflow" title="Spiritual Overflow" -->

## Spiritual Overflow

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_overflow`
- Snapshot ID: `40481`
- Source-Dokument: `7073`
- Kurzinfo: Spiritual Overflow aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spiritual Overflow`
- Payload Hash: `4b71d951c1463a0a8275107545972de67f34e5f4b4418e5dda3d1666bdeff96a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.685909+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Gain bonus <span class=\"highlight\">Fire Rate</span>, <span class=\"highlight\">Spirit Power</span> and <span class=\"highlight\">Spirit Lifesteal</span> by <span class=\"highlight\">charging up</span> when shooting enemy heroes.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 15
      },
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 13
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuildUpPerShot",
        "Value": 0.75
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_tech_overflow_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 32
      },
      {
        "Key": "BonusSpirit",
        "Type": "tech_damage",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_overflow",
  "Name": "Spiritual Overflow",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "NonHeroAbilityLifestealTooltipOnly": {
      "Key": "NonHeroAbilityLifestealTooltipOnly",
      "Value": 3
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityLifestealPercentHero": 15,
    "BonusAbilityDurationPercent": 15,
    "BonusFireRate": 20,
    "BonusHealth": 80,
    "BonusSpirit": 30,
    "TechPower": 9
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "spiritual overflow",
      "name": "Spiritual Overflow",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_split_shot" title="Split Shot" -->

## Split Shot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_split_shot`
- Snapshot ID: `40466`
- Source-Dokument: `7073`
- Kurzinfo: Split Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Split Shot`
- Payload Hash: `c5e8200723f1862b7ae405ac26a28305eb796b84fc03751e2aaacd5e47019f8f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.655350+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": "Make your weapon fire <span class=\"highlight\">multishot</span>. <br><br> Hitting more than one Hero per attack will grant a <span class=\"highlight\">stacking weapon damage bonus</span>. <br><br><span class=\"diminish\">Targets can only be hit once per multishot.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusShotsDuration",
        "LocTokenOverride": "BuffDuration",
        "Value": 5
      },
      {
        "Key": "MaxStacks",
        "Value": 5
      },
      {
        "Key": "WeaponDamageBonusDuration",
        "LocTokenOverride": "SplitShotWeaponDuration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": 27,
    "DescKey": "#upgrade_split_shot_desc",
    "Main": [
      {
        "Key": "BulletSplitShot",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 5
      },
      {
        "Key": "WeaponDamagePerStack",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_split_shot",
  "Name": "Split Shot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SpreadAngleDegrees": {
      "Key": "SpreadAngleDegrees",
      "Value": 45
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BulletSplitShot": 4,
    "WeaponDamagePerStack": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "split shot",
      "name": "Split Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_sprint_booster" title="Sprint Boots" -->

## Sprint Boots

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_sprint_booster`
- Snapshot ID: `40467`
- Source-Dokument: `7073`
- Kurzinfo: Sprint Boots aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sprint Boots`
- Payload Hash: `ac5545a77e3868a7ffe9a425eee3ffe01a3b50acef871c7578800073d257b897`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.658565+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_sprint_booster",
  "Name": "Sprint Boots",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "Upgrades": {
    "BonusSprintSpeed": "12m",
    "OutOfCombatHealthRegen": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "sprint boots",
      "name": "Sprint Boots",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_backstabber" title="Stalker" -->

## Stalker

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_backstabber`
- Snapshot ID: `40498`
- Source-Dokument: `7073`
- Kurzinfo: Stalker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stalker`
- Payload Hash: `ef793a64cc9ed61a923a6607a68ad8e3fc54ae1b987e7db4c692483942dc9608`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.717882+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Dealing {g:citadel_inline_attribute:'WeaponDamage'} at close range opens a wound and grants you {g:citadel_inline_attribute:'BonusMoveSpeed'}. <br><br>Wounded enemies take {g:citadel_inline_attribute:'SpiritDPS'}, have reduced {g:citadel_inline_attribute:'BulletResist'}, and are revealed <span class=\"highlight\">through walls</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "ReduceFootstepSound",
        "Value": -50
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Value": 5
      },
      {
        "Key": "ProcRadius",
        "LocTokenOverride": "ProcRadius",
        "Type": "distance",
        "Value": "8m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 6,
    "DescKey": "#upgrade_weapon_backstabber_desc",
    "Main": [
      {
        "Key": "DPS",
        "Type": "tech_damage",
        "Value": 17
      },
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -6
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.5m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_backstabber",
  "Name": "Stalker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 5
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DebuffRadius": {
      "Key": "DebuffRadius",
      "LocTokenOverride": "BackstabberRadius",
      "Type": "distance",
      "Value": "25m"
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusMoveSpeed": "2m",
    "BulletResistReduction": -10,
    "DPS": 20,
    "ReduceFootstepSound": -50
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "stalker",
      "name": "Stalker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_superior_stamina" title="Stamina Mastery" -->

## Stamina Mastery

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_superior_stamina`
- Snapshot ID: `40472`
- Source-Dokument: `7073`
- Kurzinfo: Stamina Mastery aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stamina Mastery`
- Payload Hash: `b06d06818eec1d2fc2f33781f04f38874d7780841a89fbb7b3e7c8a63b768785`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.667822+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "Stamina",
        "Value": 2
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": 18
      },
      {
        "Key": "AirMoveIncreasePercent",
        "Value": 23
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_superior_stamina_passive",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_superior_stamina",
  "Name": "Stamina Mastery",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AirMoveIncreasePercent": 40,
    "Stamina": 2,
    "StaminaCooldownReduction": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "stamina mastery",
      "name": "Stamina Mastery",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cooldown_reduction" title="Superior Cooldown" -->

## Superior Cooldown

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cooldown_reduction`
- Snapshot ID: `40317`
- Source-Dokument: `7073`
- Kurzinfo: Superior Cooldown aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Superior Cooldown`
- Payload Hash: `c97d4469a75a15f6e2d68eadee009c27ebee9ef99da7208ce5142de4c4fe0d93`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.379549+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_tempo"
  ],
  "Cost": 3200,
  "Description": "Reduces the <span class=\"highlight\">Cooldown</span> of your abilities.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_cooldown_reduction_desc",
    "Main": [
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cooldown_reduction",
  "Name": "Superior Cooldown",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "CooldownReduction": 10,
    "OutOfCombatHealthRegen": 6
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "superior cooldown",
      "name": "Superior Cooldown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_imbued_duration_extender" title="Superior Duration" -->

## Superior Duration

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_imbued_duration_extender`
- Snapshot ID: `40378`
- Source-Dokument: `7073`
- Kurzinfo: Superior Duration aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Superior Duration`
- Payload Hash: `d9f6a52f93a1ebb18e2afac461e976b8eb6d4e34fd0182300f5d2428db8a38b1`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.490481+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_arcane_extension"
  ],
  "Cost": 3200,
  "Description": "Increases the <span class=\"highlight\">duration</span> of your abilities and items.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_imbued_duration_extender_desc",
    "Main": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 28
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_imbued_duration_extender",
  "Name": "Superior Duration",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusAbilityDurationPercent": 12,
    "BulletResist": 8
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "superior duration",
      "name": "Superior Duration",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_suppressor" title="Suppressor" -->

## Suppressor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_suppressor`
- Snapshot ID: `40473`
- Source-Dokument: `7073`
- Kurzinfo: Suppressor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Suppressor`
- Payload Hash: `f3a1e1ff41fb3a1e87426f873b655f32b48575da30e47822d461c2d010c34c88`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.670049+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "When you deal {g:citadel_inline_attribute:'SpiritDamage'} to enemies, you also reduce their <span class=\"highlight\">Fire Rate</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 6
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_suppressor_desc",
    "Main": [
      {
        "Key": "FireRateSlow",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyEnemyApplied",
        "Value": 28
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_suppressor",
  "Name": "Suppressor",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Healing",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "BulletResist": 16,
    "FireRateSlow": 20,
    "TechPower": 12
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "suppressor",
      "name": "Suppressor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_storm" title="Surge of Power" -->

## Surge of Power

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_storm`
- Snapshot ID: `40400`
- Source-Dokument: `7073`
- Kurzinfo: Surge of Power aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Surge of Power`
- Payload Hash: `018b55c919e8eddc8a0333f6134875371a2ab6d2f86b8ecfbdd9bbd84f48d758`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.527099+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_spirit"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with <span class=\"highlight\">permanent Spirit Power</span>. When that ability is used, gain bonus <span class=\"highlight\">Move Speed</span> and maintain full speed while attacking.",
  "Info1": {
    "Alt": [
      {
        "Key": "MovementSpeedBonusDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 14,
    "DescKey": "#upgrade_magic_storm_desc",
    "Main": [
      {
        "Key": "ImbuedTechPower",
        "Type": "tech_damage",
        "Value": 28
      },
      {
        "Key": "FireRateBonus",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.75m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_storm",
  "Name": "Surge of Power",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileShootingSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileZoomedSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusMoveSpeed": "2m",
    "FireRateBonus": 18,
    "ImbuedTechPower": 32
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "surge of power",
      "name": "Surge of Power",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_blitz_bullets" title="Swift Striker" -->

## Swift Striker

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_blitz_bullets`
- Snapshot ID: `40283`
- Source-Dokument: `7073`
- Kurzinfo: Swift Striker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Swift Striker`
- Payload Hash: `f48b06fd8ec244de7a2d6d9a95e65e0e62ebdf30d202a5f7a1b12492028f6e19`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.323166+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_rapid_rounds"
  ],
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Value": 20
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_blitz_bullets",
  "Name": "Swift Striker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusFireRate": 15,
    "BonusSprintSpeed": "4m"
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "swift striker",
      "name": "Swift Striker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_shock" title="Tankbuster" -->

## Tankbuster

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_shock`
- Snapshot ID: `40398`
- Source-Dokument: `7073`
- Kurzinfo: Tankbuster aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tankbuster`
- Payload Hash: `5bb7eda0a1db80a700ac4811d99f5ff51b88871f0c197f9c63c9305b3e275cbb`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.523743+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_burst"
  ],
  "Cost": 3200,
  "Description": "Charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, causing abilities dealing more than <span class=\"highlight\">165</span> damage to deal additional damage. <span class=\"highlight\">Ignores Spirit Resistance.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": 14,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_shock_desc",
    "Main": [
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 40
      },
      {
        "Key": "CurrentHealthDamage",
        "LocTokenOverride": "MagicShockDamage",
        "Type": "tech_damage",
        "Value": 8
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_shock",
  "Name": "Tankbuster",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 14
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MinimumDamage": {
      "Key": "MinimumDamage",
      "Value": 165
    },
    "ReProcLockoutTime": {
      "Key": "ReProcLockoutTime",
      "Value": 5
    },
    "WatcherMaxDuration": {
      "Key": "WatcherMaxDuration",
      "Value": 30
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 100,
    "CurrentHealthDamage": 5,
    "Damage": 60
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "tankbuster",
      "name": "Tankbuster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_chain_lightning" title="Tesla Bullets" -->

## Tesla Bullets

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_chain_lightning`
- Snapshot ID: `40299`
- Source-Dokument: `7073`
- Kurzinfo: Tesla Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tesla Bullets`
- Payload Hash: `9ec4584950238deeec6924b10dd28b92c073a44e48db68f7ed969cb95ce8066e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.349099+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Your bullets have a chance to <span class=\"highlight\">shock</span> your target. The <span class=\"highlight\">shock</span> will jump to a nearby enemy.",
  "Info1": {
    "Alt": [
      {
        "Key": "ChainCount",
        "Value": 4
      },
      {
        "Key": "ChainRadius",
        "Type": "distance",
        "Value": "8m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.2,
    "DescKey": "#upgrade_chain_lightning_desc",
    "Main": [
      {
        "Key": "DamagePerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.19
        },
        "Type": "tech_damage",
        "Value": 33.0
      },
      {
        "Key": "ProcChance",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_chain_lightning",
  "Name": "Tesla Bullets",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusPerChain": {
      "Key": "BonusPerChain",
      "Scale": {
        "Type": "spirit",
        "Value": 0.19
      },
      "Type": "tech_damage",
      "Value": 33.0
    },
    "ChainTickRate": {
      "Key": "ChainTickRate",
      "Value": 0.4
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusPerChain": 25,
    "DamagePerChain": 25
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "tesla bullets",
      "name": "Tesla Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_timeless_emblem" title="Timeless Emblem" -->

## Timeless Emblem

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_timeless_emblem`
- Snapshot ID: `40485`
- Source-Dokument: `7073`
- Kurzinfo: Timeless Emblem aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Timeless Emblem`
- Payload Hash: `014af0641a96f410949a11ea3cb31b3e403c47cd27f9f960cdc1bf8384d03314`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.693931+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_transcendent_cooldown"
  ],
  "Cost": 9999,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 50
      },
      {
        "Key": "ItemCooldownReduction",
        "Type": "cooldown",
        "Value": 50
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 50
      },
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_timeless_emblem",
  "Name": "Timeless Emblem",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 50
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BonusAbilityDurationPercent": 10,
    "CooldownReduction": 10,
    "ItemCooldownReduction": 10,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_titan_round" title="Titanic Magazine" -->

## Titanic Magazine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_titan_round`
- Snapshot ID: `40486`
- Source-Dokument: `7073`
- Kurzinfo: Titanic Magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Titanic Magazine`
- Payload Hash: `dc6e6d7c0d319f6d7c64ef72f86c955b266779af9b65431e3e72748619848cc7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.695621+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_clip_size"
  ],
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 14
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 100
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_titan_round",
  "Name": "Titanic Magazine",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BaseAttackDamagePercent": 18,
    "BonusClipSizePercent": 70
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "titanic magazine",
      "name": "Titanic Magazine",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_damage_pulse" title="Torment Pulse" -->

## Torment Pulse

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_damage_pulse`
- Snapshot ID: `40479`
- Source-Dokument: `7073`
- Kurzinfo: Torment Pulse aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Torment Pulse`
- Payload Hash: `62bcf83f658e3494177c7e68aa802e09d58c7ae52fd17a6703f51e1e0d861bcc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.682084+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Periodically deals {g:citadel_inline_attribute:'SpiritDamage'} to the closest two enemies nearby.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 18
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamagePulseRadius",
        "Type": "distance",
        "Value": "9m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.4,
    "DescKey": "#upgrade_tech_damage_pulse_desc",
    "Main": [
      {
        "Key": "DamagePulseAmount",
        "Scale": {
          "Type": "spirit",
          "Value": 0.23
        },
        "Type": "tech_damage",
        "Value": 25.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_damage_pulse",
  "Name": "Torment Pulse",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 75,
    "DamagePulseAmount": 30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "torment pulse",
      "name": "Torment Pulse",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health_2" title="Toughness" -->

## Toughness

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_2`
- Snapshot ID: `40367`
- Source-Dokument: `7073`
- Kurzinfo: Toughness aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Toughness`
- Payload Hash: `71efc908ee797c7d7ee198755bfe635e9b17436be809fd07efb1610eba95f6c8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.472505+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 200
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_health_2",
  "Name": "Toughness",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_toxic_bullets" title="Toxic Bullets" -->

## Toxic Bullets

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_toxic_bullets`
- Snapshot ID: `40488`
- Source-Dokument: `7073`
- Kurzinfo: Toxic Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Toxic Bullets`
- Payload Hash: `ff82e2d46946cb467c0c7a186222ec996b17c0348b3ce8f6c1bb8c95aad474c5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.698892+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Bleed</span> on enemies, causing them to lose a <span class=\"highlight\">percentage</span> of their <span class=\"highlight\">Max Health</span> over time. Also applies <span class=\"highlight\">Healing Reduction</span> on the bleeding target.",
  "Info1": {
    "Alt": [
      {
        "Key": "DotDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 1.28
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_toxic_bullets_desc",
    "Main": [
      {
        "Key": "DotHealthPercent",
        "Scale": {
          "Type": "spirit",
          "Value": 0.005
        },
        "Type": "tech_damage",
        "Value": 1.9
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -35
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_toxic_bullets",
  "Name": "Toxic Bullets",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DotMultiplerTroopers": {
      "Key": "DotMultiplerTroopers",
      "Value": 0.5
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -35
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "DotHealthPercent": 0.7,
    "HealAmpReceivePenaltyPercent": -30,
    "HealAmpRegenPenaltyPercent": -30
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "toxic bullets",
      "name": "Toxic Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_transcendent_cooldown" title="Transcendent Cooldown" -->

## Transcendent Cooldown

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_transcendent_cooldown`
- Snapshot ID: `40489`
- Source-Dokument: `7073`
- Kurzinfo: Transcendent Cooldown aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Transcendent Cooldown`
- Payload Hash: `7c7c81b398bde587d4d7c5ffb9a6589afc3e61c7ea4d7d98a15fba6f6004feba`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.700762+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_cooldown_reduction"
  ],
  "Cost": 6400,
  "Description": "Reduces the <span class=\"highlight\">Cooldown</span> of your abilities and items.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_transcendent_cooldown_desc",
    "Main": [
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 25
      },
      {
        "Key": "ItemCooldownReduction",
        "Type": "cooldown",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_transcendent_cooldown",
  "Name": "Transcendent Cooldown",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "CooldownReduction": 15,
    "ItemCooldownReduction": 10,
    "OutOfCombatHealthRegen": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "transcendent cooldown",
      "name": "Transcendent Cooldown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_trophy_collector" title="Trophy Collector" -->

## Trophy Collector

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_trophy_collector`
- Snapshot ID: `40490`
- Source-Dokument: `7073`
- Kurzinfo: Trophy Collector aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Trophy Collector`
- Payload Hash: `3cbe2c1d349c1aec245d94043475f6e2a8df9d15b6b1de4c48bdf1d3fddbb754`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.702441+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 1600,
  "Description": "Whenever you score an <span class=\"highlight\">assist or kill</span>, gain extra <span class=\"highlight\">sprint</span>, <span class=\"highlight\">ability range</span> and <span class=\"highlight\">passive soul generation</span>. This effect stacks and persists through death.",
  "Info1": {
    "Alt": [
      {
        "Key": "NonPlayerBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -15
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MaxStacks",
        "Value": 16
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_trophy_collector_desc",
    "Main": [
      {
        "Key": "StackingBonusSprintSpeed",
        "LocTokenOverride": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.15m"
      },
      {
        "Key": "StackingTechRangeMultiplier",
        "LocTokenOverride": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 0.75
      },
      {
        "Key": "StackingGoldPerMinute",
        "Type": "souls",
        "Value": 18
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_trophy_collector",
  "Name": "Trophy Collector",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "StackingTechRadiusMultiplier": {
      "Key": "StackingTechRadiusMultiplier",
      "Type": "distance",
      "Value": 0.75
    },
    "ThinkRate": {
      "Key": "ThinkRate",
      "Value": 3
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusSprintSpeed": "12m",
    "MaxStacks": 83,
    "OutOfCombatHealthRegen": 6,
    "StackingTechRadiusMultiplier": 3,
    "StackingTechRangeMultiplier": 3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "trophy collector",
      "name": "Trophy Collector",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_unstable_concoction" title="Unstable Concoction" -->

## Unstable Concoction

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstable_concoction`
- Snapshot ID: `40492`
- Source-Dokument: `7073`
- Kurzinfo: Unstable Concoction aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unstable Concoction`
- Payload Hash: `c60a18a16d78b86719974cc8b36f5a4e05a8290f14e68353feffbf61ca11eeb9`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.706189+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "Consume a concoction that grants you <span class=\"highlight\">Unstoppable</span> and increased <span class=\"highlight\">speed, health, spirit and weapon damage</span>. After a short duration <span class=\"highlight\">you die and explode</span>, stunning nearby enemies and dealing damage based on your maximum health. Dying this way reduces your respawn time by <span class=\"highlight\">50%</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.0
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 150
      },
      {
        "Key": "Radius",
        "Value": "22m"
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 150
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25.0,
    "DescKey": "#upgrade_unstable_concoction_desc",
    "Main": [
      {
        "Key": "MaxHPDamage",
        "Type": "tech_damage",
        "Value": 30
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 3.0
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "10m"
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "UsageFlags": "ConditionallyApplied",
        "Value": 3000
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstable_concoction",
  "Name": "Unstable Concoction",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "RespawnTimeMod": {
      "Key": "RespawnTimeMod",
      "Value": 50
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "BaseAttackDamagePercent": 50,
    "BonusHealth": 1300,
    "StunDuration": 0.5,
    "TechPower": 50
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "unstable concoction",
      "name": "Unstable Concoction",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_unstoppable" title="Unstoppable" -->

## Unstoppable

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstoppable`
- Snapshot ID: `40493`
- Source-Dokument: `7073`
- Kurzinfo: Unstoppable aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unstoppable`
- Payload Hash: `abdd2358d8ac7d7d1d97e09dc5038ce1ce957b85a7b61c6fe01d6665a76b02d9`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.708082+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "Description": "Temporarily suppress <span class=\"highlight\">negative status effects</span> and become <span class=\"highlight\">immune</span> to <span class=\"highlight\">Stun, Silence, Sleep, Root, and Disarm</span>. <br>Cannot be used while <span class=\"highlight\">Stunned</span> or <span class=\"highlight\">Slept</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 25
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 60.0,
    "DescKey": "#upgrade_unstoppable_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5.5
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstoppable",
  "Name": "Unstoppable",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -35,
    "AbilityDuration": 1.25,
    "BonusHealth": 75,
    "StatusResistancePercent": 15
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "unstoppable",
      "name": "Unstoppable",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_surging_power" title="Vampiric Burst" -->

## Vampiric Burst

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_surging_power`
- Snapshot ID: `40474`
- Source-Dokument: `7073`
- Kurzinfo: Vampiric Burst aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vampiric Burst`
- Payload Hash: `4de9e2418b10e5c0969dc743bc8f90d4470a34d70ae3bec0485ba0be5a1e3cc5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.672418+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_vampire"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 13
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
      },
      {
        "Key": "BonusHealth",
        "Value": 100
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ActiveReloadPercent",
        "Value": 75
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30.0,
    "DescKey": "#upgrade_surging_power_active",
    "Main": [
      {
        "Key": "ActiveBonusLifesteal",
        "Type": "healing",
        "Value": 70
      },
      {
        "Key": "ActiveBonusFireRate",
        "Type": "fire_rate",
        "Value": 34
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_surging_power",
  "Name": "Vampiric Burst",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate",
    "WeaponDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -10,
    "ActiveBonusFireRate": 25,
    "BonusHealth": 110,
    "BulletLifestealPercent": 16,
    "BulletResist": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "vampiric burst",
      "name": "Vampiric Burst",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_veil_walker" title="Veil Walker" -->

## Veil Walker

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_veil_walker`
- Snapshot ID: `40495`
- Source-Dokument: `7073`
- Kurzinfo: Veil Walker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Veil Walker`
- Payload Hash: `a38a761c3c13b45a91ecda1750c9a461db5028a72b2ecdc1a84c58ad59896069`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.711758+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 3200,
  "Description": "Walking through a <span class=\"highlight\">cosmic veil</span> grants you <span class=\"highlight\">Stealth</span>, <span class=\"highlight\">Heal</span> and increased <span class=\"highlight\">Move Speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "InvisDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 15.0,
    "DescKey": "#upgrade_veil_walker_desc",
    "Main": [
      {
        "Key": "StatusEffectInvisible",
        "Value": null
      },
      {
        "Key": "BonusMoveSpeed",
        "LocTokenOverride": "#VeilWalker_MoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3.5m"
      },
      {
        "Key": "HealOnVeil",
        "LocTokenOverride": "#VeilWalker_Heal",
        "Scale": {
          "Type": "power_increase",
          "Value": 8
        },
        "Type": "healing",
        "Value": 85
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_veil_walker",
  "Name": "Veil Walker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 16
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 0.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 1.25
    },
    "SpottedRadius": {
      "Key": "SpottedRadius",
      "Value": "20m"
    }
  },
  "ShopFilters": [
    "Durability",
    "ClipSize"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -9,
    "BonusMoveSpeed": "4m",
    "BonusSprintSpeed": "12m",
    "HealOnVeil": 300,
    "InvisDuration": 4,
    "InvisMoveSpeedMod": "6m",
    "OutOfCombatHealthRegen": 8,
    "SpiritPower": 25
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "veil walker",
      "name": "Veil Walker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aoe_root" title="Vortex Web" -->

## Vortex Web

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_root`
- Snapshot ID: `40265`
- Source-Dokument: `7073`
- Kurzinfo: Vortex Web aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vortex Web`
- Payload Hash: `6766c8932752c717f129a591939d2b1869eda62d9f45bf3c63cf8632f79521c5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.292028+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_containment"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.0
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "ChargeUp": null,
    "Cooldown": 42.0,
    "DescKey": "#upgrade_aoe_root_active",
    "Main": [
      {
        "Key": "CaptureRadius",
        "Type": "distance",
        "Value": "12m"
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_aoe_root",
  "Name": "Vortex Web",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "30m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 8
    },
    "TetherDuration": {
      "Key": "TetherDuration",
      "Value": 0.5
    },
    "TetherRadius": {
      "Key": "TetherRadius",
      "Type": "distance",
      "Value": "1m"
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -22,
    "BonusSprintSpeed": "9m",
    "SlowPercent": 15,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "vortex web",
      "name": "Vortex Web",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_warp_stone" title="Warp Stone" -->

## Warp Stone

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_warp_stone`
- Snapshot ID: `40497`
- Source-Dokument: `7073`
- Kurzinfo: Warp Stone aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Warp Stone`
- Payload Hash: `a5733dc5b916bf629e2a982343f37986951f1fc434710674935fcf6867afe9bc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.715928+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Teleport</span> straight ahead, gaining <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "CasterBuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_warp_stone_desc",
    "Main": [
      {
        "Key": "AbilityCastRange",
        "LocTokenOverride": "WarpStoneRange",
        "Type": "range",
        "Value": "11m"
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_warp_stone",
  "Name": "Warp Stone",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCastRange": "9m",
    "AbilityCooldown": -3,
    "BulletResist": 20
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "warp stone",
      "name": "Warp Stone",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_headshot_booster2" title="Weakening Headshot" -->

## Weakening Headshot

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_headshot_booster2`
- Snapshot ID: `40361`
- Source-Dokument: `7073`
- Kurzinfo: Weakening Headshot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Weakening Headshot`
- Payload Hash: `8d4ed718049b4b5aedc2ef335cd768290c542c98a7e6cba2dc932a529e897a93`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.462481+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Landing a <span class=\"highlight\">Headshot</span> reduces their <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 60
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_headshot_booster2_desc",
    "Main": [
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -13
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headshot_booster2",
  "Name": "Weakening Headshot",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DiminishingMultiplier": {
      "Key": "DiminishingMultiplier",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusHealth": 125,
    "BulletResistReduction": -7
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "weakening headshot",
      "name": "Weakening Headshot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_shielding" title="Weapon Shielding" -->

## Weapon Shielding

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_shielding`
- Snapshot ID: `40504`
- Source-Dokument: `7073`
- Kurzinfo: Weapon Shielding aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Weapon Shielding`
- Payload Hash: `bc702fa5c3f923ea026b5e0b27e261121986939c0c4e4155c299602be0e59d28`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.726747+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'WeaponDamage'} from enemy Heroes in a small time frame.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "bullet_armor_up",
        "Value": 250
      },
      {
        "Key": "DamageWindow",
        "Type": "duration",
        "Value": 4.0
      },
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35,
    "DescKey": "#upgrade_weapon_shielding_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Scale": {
          "Type": "power_increase",
          "Value": 5.0
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 300.0
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 18
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_shielding",
  "Name": "Weapon Shielding",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -20,
    "BulletResist": 15,
    "CombatBarrier": 225,
    "OutOfCombatHealthRegen": 3
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "weapon shielding",
      "name": "Weapon Shielding",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weighted_shots" title="Weighted Shots" -->

## Weighted Shots

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weighted_shots`
- Snapshot ID: `40505`
- Source-Dokument: `7073`
- Kurzinfo: Weighted Shots aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Weighted Shots`
- Payload Hash: `e25eb987bc4a171a0b7dfbaa81a3fc62ade854efc24eddb9fec0359d3a8bb967`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.728534+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_slowing_bullets"
  ],
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Movement Slow</span> on enemies.",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 22
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": -14
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "-0.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 40
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -22
      },
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 0.7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_weighted_shots_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weighted_shots",
  "Name": "Weighted Shots",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 35,
    "GroundDashReductionPercent": -10,
    "SlowPercent": 20,
    "StatusResistancePercent": 10
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "weighted shots",
      "name": "Weighted Shots",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_absorbing_armor" title="Witchmail" -->

## Witchmail

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_absorbing_armor`
- Snapshot ID: `40257`
- Source-Dokument: `7073`
- Kurzinfo: Witchmail aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Witchmail`
- Payload Hash: `c1455ac8a8cf068c44f6d5f601801d46049c96e2d37136d72b3e7b90efd1ff6e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.278958+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Taking heavy hits of {g:citadel_inline_attribute:'SpiritDamage'} from an enemy reduces a <span class=\"highlight\">random ability cooldown</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "Value": 22
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 14
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamageThreshold",
        "Type": "tech_damage",
        "Value": 75
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1,
    "DescKey": "#upgrade_absorbing_armor_desc",
    "Main": [
      {
        "Key": "CooldownReductionPerHit",
        "Type": "cooldown",
        "Value": 4
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_absorbing_armor",
  "Name": "Witchmail",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownReduction": {
      "Key": "CooldownReduction",
      "Value": 7
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "CooldownReductionPerHit": 2,
    "TechPower": 26,
    "TechResist": 5
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "witchmail",
      "name": "Witchmail",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="ability_item_pickup_effects" title="ability_item_pickup_effects" -->

## ability_item_pickup_effects

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `ability_item_pickup_effects`
- Snapshot ID: `40234`
- Source-Dokument: `7073`
- Kurzinfo: ability_item_pickup_effects aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `ability_item_pickup_effects`
- Payload Hash: `62581182cbdca4c50fe99b694e17db8abef8d05883e0e88418d0adb12615f53e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.246577+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "ability_item_pickup_effects",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 2.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": null,
  "StreetBrawl": false,
  "TargetTypes": [
    "Hero"
  ],
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="armor_upgrade_base" title="armor_upgrade_base" -->

## armor_upgrade_base

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `armor_upgrade_base`
- Snapshot ID: `40235`
- Source-Dokument: `7073`
- Kurzinfo: armor_upgrade_base aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `armor_upgrade_base`
- Payload Hash: `b054f8a7f060a3470f2afd254dc82d87b04c9e68ebb25466263da7fd7839f0ae`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.248084+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "armor_upgrade_base",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="armor_upgrade_t1" title="armor_upgrade_t1" -->

## armor_upgrade_t1

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `armor_upgrade_t1`
- Snapshot ID: `40236`
- Source-Dokument: `7073`
- Kurzinfo: armor_upgrade_t1 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `armor_upgrade_t1`
- Payload Hash: `87fe8ee3fbed8bcfdc22fc568ea54d06baa93c3bfb540efc30178436e285bfeb`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.249483+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "armor_upgrade_t1",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="armor_upgrade_t2" title="armor_upgrade_t2" -->

## armor_upgrade_t2

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `armor_upgrade_t2`
- Snapshot ID: `40237`
- Source-Dokument: `7073`
- Kurzinfo: armor_upgrade_t2 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `armor_upgrade_t2`
- Payload Hash: `961593ad8795a088fe6594c2f7f90b57a86c413075778092a8f99a163d8f3bd6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.250626+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "armor_upgrade_t2",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="armor_upgrade_t3" title="armor_upgrade_t3" -->

## armor_upgrade_t3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `armor_upgrade_t3`
- Snapshot ID: `40238`
- Source-Dokument: `7073`
- Kurzinfo: armor_upgrade_t3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `armor_upgrade_t3`
- Payload Hash: `1581136985e7aba5c47ebe63ff4392b129bde6a4b84c0c89450ccca0cbbff8d7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.251649+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "armor_upgrade_t3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="armor_upgrade_t4" title="armor_upgrade_t4" -->

## armor_upgrade_t4

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `armor_upgrade_t4`
- Snapshot ID: `40239`
- Source-Dokument: `7073`
- Kurzinfo: armor_upgrade_t4 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `armor_upgrade_t4`
- Payload Hash: `9d85ae9ea4c73bbeebcb81fb95f23e0606015a6cf53f461b07bd9a61c4575748`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.253006+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "armor_upgrade_t4",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="armor_upgrade_t5" title="armor_upgrade_t5" -->

## armor_upgrade_t5

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `armor_upgrade_t5`
- Snapshot ID: `40240`
- Source-Dokument: `7073`
- Kurzinfo: armor_upgrade_t5 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `armor_upgrade_t5`
- Payload Hash: `c8db18a8ed88c2a93f648fcfc91af9b6972bd89be17c1ed5813537d98a4e8ad0`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.254397+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "armor_upgrade_t5",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 5,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="item_projectile_test_01" title="item_projectile_test_01" -->

## item_projectile_test_01

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_01`
- Snapshot ID: `40243`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_01 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `item_projectile_test_01`
- Payload Hash: `74391fe7180f99ba761b40c870fecc3493aa0b41d5718f0e8f85c84cdf21785a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.259759+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_01_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_01",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="item_projectile_test_02" title="item_projectile_test_02" -->

## item_projectile_test_02

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_02`
- Snapshot ID: `40244`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_02 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `item_projectile_test_02`
- Payload Hash: `dd85ee9fe9573a7e6ffd6fae046d141828abb592f1aaedb03b306f20ba53a99e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.261353+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_02_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_02",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="item_projectile_test_03" title="item_projectile_test_03" -->

## item_projectile_test_03

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_03`
- Snapshot ID: `40245`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_03 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `item_projectile_test_03`
- Payload Hash: `6ece1ed0da00e93e179c57c273405c1f9de58118872fb05c767169263e5b5ff8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.262712+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_03_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_03",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="item_projectile_test_04" title="item_projectile_test_04" -->

## item_projectile_test_04

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_04`
- Snapshot ID: `40246`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_04 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `item_projectile_test_04`
- Payload Hash: `e1044ac47250b17878128f551960e4f1b2d2812561fefbd19b507f33baeaad8e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.263884+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_04_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_04",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="item_projectile_test_05" title="item_projectile_test_05" -->

## item_projectile_test_05

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_05`
- Snapshot ID: `40247`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_05 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `item_projectile_test_05`
- Payload Hash: `87e9f061a686f8fa72dae5fd72803f7ce57d65169acec40af17516426a88bda3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.265186+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_05_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_05",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="item_projectile_test_06" title="item_projectile_test_06" -->

## item_projectile_test_06

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_06`
- Snapshot ID: `40248`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_06 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `item_projectile_test_06`
- Payload Hash: `494494f140ee767dd7136d00af895e94c570dca35cd5ebb5ebf364afec54e92d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.266288+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_06_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_06",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="tech_upgrade_base" title="tech_upgrade_base" -->

## tech_upgrade_base

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `tech_upgrade_base`
- Snapshot ID: `40249`
- Source-Dokument: `7073`
- Kurzinfo: tech_upgrade_base aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `tech_upgrade_base`
- Payload Hash: `48e42b1845785a9fa181ee32aaa3d3e1d2aeda003d8c17e01daeb058a85ac226`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.267684+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "tech_upgrade_base",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="tech_upgrade_t1" title="tech_upgrade_t1" -->

## tech_upgrade_t1

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `tech_upgrade_t1`
- Snapshot ID: `40250`
- Source-Dokument: `7073`
- Kurzinfo: tech_upgrade_t1 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `tech_upgrade_t1`
- Payload Hash: `96bc369ccabab23f06f78bb9b70e6d380e9cf6437d0a52e3931a7732ed6a8227`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.268801+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "tech_upgrade_t1",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="tech_upgrade_t2" title="tech_upgrade_t2" -->

## tech_upgrade_t2

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `tech_upgrade_t2`
- Snapshot ID: `40251`
- Source-Dokument: `7073`
- Kurzinfo: tech_upgrade_t2 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `tech_upgrade_t2`
- Payload Hash: `05d13c8cd6823d0100c6fd44bb59bbf3a35d323eda58170aab564e548400f98b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.270151+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "tech_upgrade_t2",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="tech_upgrade_t3" title="tech_upgrade_t3" -->

## tech_upgrade_t3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `tech_upgrade_t3`
- Snapshot ID: `40252`
- Source-Dokument: `7073`
- Kurzinfo: tech_upgrade_t3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `tech_upgrade_t3`
- Payload Hash: `da2163ed5eb4a0968d3d7ee4547e5341f7298837a2f24b6a9531c6fbe038e6e7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.271501+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "tech_upgrade_t3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="tech_upgrade_t4" title="tech_upgrade_t4" -->

## tech_upgrade_t4

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `tech_upgrade_t4`
- Snapshot ID: `40253`
- Source-Dokument: `7073`
- Kurzinfo: tech_upgrade_t4 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `tech_upgrade_t4`
- Payload Hash: `fceae28aec42cf588bb757e1474c36f8892047c8de7d861187ea8753a384ba09`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.272829+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "tech_upgrade_t4",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="tech_upgrade_t5" title="tech_upgrade_t5" -->

## tech_upgrade_t5

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `tech_upgrade_t5`
- Snapshot ID: `40254`
- Source-Dokument: `7073`
- Kurzinfo: tech_upgrade_t5 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `tech_upgrade_t5`
- Payload Hash: `3531001067516b6c4428ac1775a12530177ff46d4a46a58e326b0a408eb8b6ab`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.274094+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "tech_upgrade_t5",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 5,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_active_bullet_shield" title="upgrade_active_bullet_shield" -->

## upgrade_active_bullet_shield

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_active_bullet_shield`
- Snapshot ID: `40259`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_active_bullet_shield aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_active_bullet_shield`
- Payload Hash: `3d680bbb0062d688e4c33bbe21f3e54066b675b8099d0267acad24888780e1ec`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.282516+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": 64.0,
    "DescKey": "#upgrade_active_bullet_shield_desc",
    "Main": [],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_generic_bullet_shield_desc",
    "Main": [],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_sprint_booster_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_active_bullet_shield",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
    },
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "25m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "30m"
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly",
    "MinionFriendly"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aerial_assault" title="upgrade_aerial_assault" -->

## upgrade_aerial_assault

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aerial_assault`
- Snapshot ID: `40261`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_aerial_assault aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_aerial_assault`
- Payload Hash: `2a03f3317e60a2074bf5458bd5046521fa8a10af16356eb13c07f639e1a4f72e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.286080+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 26.0,
    "DescKey": "#upgrade_aerial_assault_active",
    "Main": [],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 25
      },
      {
        "Key": "BonusFireRate",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aerial_assault",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ActivateTime": {
      "Key": "ActivateTime",
      "Value": 0.75
    },
    "AssaultDuration": {
      "Key": "AssaultDuration",
      "Type": "duration",
      "Value": 3.0
    },
    "AssaultFireRate": {
      "Key": "AssaultFireRate",
      "UsageFlags": "ConditionallyApplied",
      "Value": 60
    },
    "AssaultLifestealPercent": {
      "Key": "AssaultLifestealPercent",
      "Type": "healing",
      "UsageFlags": "ConditionallyApplied",
      "Value": 20
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ExplodeRadius": {
      "Key": "ExplodeRadius",
      "Type": "distance",
      "Value": "3m"
    },
    "JumpVelocityHidden": {
      "Key": "JumpVelocityHidden",
      "Value": "30.0m"
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aoe_silence" title="upgrade_aoe_silence" -->

## upgrade_aoe_silence

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_silence`
- Snapshot ID: `40266`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_aoe_silence aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_aoe_silence`
- Payload Hash: `0fff16e08b30bb4a7f7dd3db062b0411f13649dd7143b702ef46142b053b2d2f`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.293768+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 150
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "8m"
      },
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 32.0,
    "DescKey": "#upgrade_aoe_silence_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aoe_silence",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "StartRadius": {
      "Key": "StartRadius",
      "Type": "distance",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aoe_smoke_bomb" title="upgrade_aoe_smoke_bomb" -->

## upgrade_aoe_smoke_bomb

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_smoke_bomb`
- Snapshot ID: `40267`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_aoe_smoke_bomb aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_aoe_smoke_bomb`
- Payload Hash: `d4952aad8ab6dae3507efa323d8cdc879fd3c10db4a170e84225fc4dd41b5439`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.295323+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 25
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "30m"
      },
      {
        "Key": "InvisMoveSpeedMod",
        "Type": "move_speed",
        "Value": "1.5m"
      },
      {
        "Key": "SpottedRadius",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 64.0,
    "DescKey": "#upgrade_aoe_smoke_bomb_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aoe_smoke_bomb",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 1.5
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "FullInvisDistance": {
      "Key": "FullInvisDistance",
      "Value": "50m"
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisCancelOnDamage": {
      "Key": "InvisCancelOnDamage",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 0.3
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 0.25
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_aoe_tech_shield" title="upgrade_aoe_tech_shield" -->

## upgrade_aoe_tech_shield

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_tech_shield`
- Snapshot ID: `40268`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_aoe_tech_shield aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_aoe_tech_shield`
- Payload Hash: `18c34f7e56e691ee60ee7c73685030ae86586b27b2d13781b36ae2bea8f480da`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.296671+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aoe_tech_shield",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
    },
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 32.0
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 16
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "30m"
    },
    "TempTechShieldHealth": {
      "Key": "TempTechShieldHealth",
      "Type": "tech_armor_up",
      "Value": 400
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_arcane_eater" title="upgrade_arcane_eater" -->

## upgrade_arcane_eater

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_eater`
- Snapshot ID: `40271`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_arcane_eater aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_arcane_eater`
- Payload Hash: `35f86266b90aba2bc9f5ddfb6a1f0e728fd7939c5bacb6a3e3998c17de199ce7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.301577+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StealDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.5,
    "DescKey": "#upgrade_arcane_eater_desc",
    "Main": [
      {
        "Key": "SpiritStolePerHit",
        "Type": "tech_damage",
        "Value": 1
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_arcane_eater",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HeadshotBonusSteal": {
      "Key": "HeadshotBonusSteal",
      "Value": 1
    },
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 99
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_arcane_medallion" title="upgrade_arcane_medallion" -->

## upgrade_arcane_medallion

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_medallion`
- Snapshot ID: `40273`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_arcane_medallion aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_arcane_medallion`
- Payload Hash: `802aa97c2e5b6ec2becfe6b90fe55dd1e8d4aa3e451c8ad09e78bf4c4c2676f1`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.304709+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_arcane_medallion_desc",
    "Main": [
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_arcane_medallion",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MinimumDamage": {
      "Key": "MinimumDamage",
      "Value": 200
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_attack_speed_1" title="upgrade_attack_speed_1" -->

## upgrade_attack_speed_1

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_attack_speed_1`
- Snapshot ID: `40276`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_attack_speed_1 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_attack_speed_1`
- Payload Hash: `7b450847badd792e413ddd96cc1d4b583a172cbf4d384f0951b8db3d0dd6cfc7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.310604+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_attack_speed_1",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_attack_speed_2" title="upgrade_attack_speed_2" -->

## upgrade_attack_speed_2

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_attack_speed_2`
- Snapshot ID: `40277`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_attack_speed_2 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_attack_speed_2`
- Payload Hash: `bc701cc75a800b7b6d1522a4fd522fff651051cecf5f77b716595060a87766c8`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.312289+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_attack_speed_2",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_base" title="upgrade_base" -->

## upgrade_base

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_base`
- Snapshot ID: `40280`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_base aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_base`
- Payload Hash: `8d609d0f19f2a69c2de58877c091767aa044973e66c04e1cc7a0862945ce88b6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.318042+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_base",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": null,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_belt_fed_magazine" title="upgrade_belt_fed_magazine" -->

## upgrade_belt_fed_magazine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_belt_fed_magazine`
- Snapshot ID: `40281`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_belt_fed_magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_belt_fed_magazine`
- Payload Hash: `a618db9902e78f5c1f27fba36d6f4bd2877849960a72a01cf6892558973d205b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.319461+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "SpinUpTime",
        "Value": 3.5
      },
      {
        "Key": "InitialFireRateDecrease",
        "Value": 40
      },
      {
        "Key": "MaxFireRateIncrease",
        "Value": 60
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_belt_fed_magazine_desc",
    "Main": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 125
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_belt_fed_magazine",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SpinUpDecay": {
      "Key": "SpinUpDecay",
      "Value": 1.0
    }
  },
  "ShopFilters": [
    "FireRate",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_bonus_ability_charge_3" title="upgrade_bonus_ability_charge_3" -->

## upgrade_bonus_ability_charge_3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bonus_ability_charge_3`
- Snapshot ID: `40285`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_bonus_ability_charge_3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_bonus_ability_charge_3`
- Payload Hash: `ff0521a8988ea5258e959782daa4550c53fe128b90907eb26d7c7e9cf758c574`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.326710+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityCharges",
        "Type": "cast",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_bonus_ability_charge_3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_bullet_damage_reduction_aura" title="upgrade_bullet_damage_reduction_aura" -->

## upgrade_bullet_damage_reduction_aura

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_damage_reduction_aura`
- Snapshot ID: `40290`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_bullet_damage_reduction_aura aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_bullet_damage_reduction_aura`
- Payload Hash: `46b24c29b5e2b951c234a067ebd1b3e41ab2b942ecaa8a864419b4b8d7bab2cd`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.334727+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "FireRateSlow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "12m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_bullet_damage_reduction_aura_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_bullet_damage_reduction_aura",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_burst_fire_actuator" title="upgrade_burst_fire_actuator" -->

## upgrade_burst_fire_actuator

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_burst_fire_actuator`
- Snapshot ID: `40294`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_burst_fire_actuator aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_burst_fire_actuator`
- Payload Hash: `fce61841f0dd4318dd321de7db112f2dcd129dfefb71baa14f8abb42ac395d45`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.341099+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseBonusWeaponPower",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "PerfectBurstWeaponPower",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_burst_fire_actuator_desc",
    "Main": [
      {
        "Key": "BurstFireShotsFromClipPercent",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_burst_fire_actuator",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BurstFireShotPercent": {
      "Key": "BurstFireShotPercent",
      "Value": 100
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "NonBurstFireConversionFactor": {
      "Key": "NonBurstFireConversionFactor",
      "Value": 3
    },
    "NormalizedClipEmptySpeedIncrease": {
      "Key": "NormalizedClipEmptySpeedIncrease",
      "Value": 25
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_camouflage" title="upgrade_camouflage" -->

## upgrade_camouflage

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_camouflage`
- Snapshot ID: `40295`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_camouflage aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_camouflage`
- Payload Hash: `ac6dc928f025ca6ff6083c9700637dd72e9aa0911434509d3f1d3c0995fc3516`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.342303+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_camouflage",
  "Name": null,
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 10.5
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 1.0
    },
    "MovementThresholdSq": {
      "Key": "MovementThresholdSq",
      "Value": 200
    },
    "RegenWhileInvisible": {
      "Key": "RegenWhileInvisible",
      "Value": 5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "SpottedRadius": {
      "Key": "SpottedRadius",
      "Value": "20m"
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_charge_mastery" title="upgrade_charge_mastery" -->

## upgrade_charge_mastery

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_charge_mastery`
- Snapshot ID: `40300`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_charge_mastery aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_charge_mastery`
- Payload Hash: `74acf5a9a05aab27246c996890cd5089fc51969870ada3caa3159bc7bb1a26af`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.351088+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_charge_mastery_passive",
    "Main": [],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BonusAbilityCharges",
        "Type": "cast",
        "Value": 4
      },
      {
        "Key": "CooldownBetweenChargeReduction",
        "Type": "cooldown",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_charge_mastery",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusChargedAbilityDamage": {
      "Key": "BonusChargedAbilityDamage",
      "Value": 15
    },
    "BonusChargedCooldownReduction": {
      "Key": "BonusChargedCooldownReduction",
      "Type": "cooldown",
      "Value": 15
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_charmed_wraps" title="upgrade_charmed_wraps" -->

## upgrade_charmed_wraps

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_charmed_wraps`
- Snapshot ID: `40301`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_charmed_wraps aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_charmed_wraps`
- Payload Hash: `81638d13e6988487e2834d066118e1b4e8e10633ac527fc82152c5c595f1ab04`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.352372+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReductionFlat",
        "Value": 8.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": 26.0,
    "DescKey": "#upgrade_charmed_wraps_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_charmed_wraps",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "LightMeleeTimeWindow": {
      "Key": "LightMeleeTimeWindow",
      "Value": 4.0
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_clip_size_2" title="upgrade_clip_size_2" -->

## upgrade_clip_size_2

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_clip_size_2`
- Snapshot ID: `40305`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_clip_size_2 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_clip_size_2`
- Payload Hash: `d41b8f5d7fb1a81a04fde5b98ef93b61124da333e4cde2397219218836748f2c`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.358895+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_clip_size_2",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusClipSizePercent": {
      "Key": "BonusClipSizePercent",
      "Value": 30
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_clip_size_3" title="upgrade_clip_size_3" -->

## upgrade_clip_size_3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_clip_size_3`
- Snapshot ID: `40306`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_clip_size_3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_clip_size_3`
- Payload Hash: `354ced09046df475ef17909e3e52170d6f053715c1b163ee775cce374ff905d5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.360039+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 60
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_clip_size_3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_clip_size_fixed" title="upgrade_clip_size_fixed" -->

## upgrade_clip_size_fixed

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_clip_size_fixed`
- Snapshot ID: `40307`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_clip_size_fixed aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_clip_size_fixed`
- Payload Hash: `a513dc693116eaa39edcfbe12411d5b71cf40de7294659b71035a01e75e5f2cc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.361338+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSize",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_clip_size_fixed",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_clip_size_fixed_t3" title="upgrade_clip_size_fixed_t3" -->

## upgrade_clip_size_fixed_t3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_clip_size_fixed_t3`
- Snapshot ID: `40308`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_clip_size_fixed_t3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_clip_size_fixed_t3`
- Payload Hash: `a26a68da693b17ddf02f59882df414c084d2314ea93dd331f24b55df4242488a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.362778+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_clip_size_fixed_t3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusClipSize": {
      "Key": "BonusClipSize",
      "Value": 12
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_cloaking_device" title="upgrade_cloaking_device" -->

## upgrade_cloaking_device

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloaking_device`
- Snapshot ID: `40310`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_cloaking_device aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_cloaking_device`
- Payload Hash: `75d450b2753ac8abd9630d7614dc1666bee82b8c1c2cb957ca05a40b3f81e2fb`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.365754+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_cloaking_device",
  "Name": null,
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 10.5
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "FullInvisDistance": {
      "Key": "FullInvisDistance",
      "Value": "50m"
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisCancelOnDamage": {
      "Key": "InvisCancelOnDamage",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "InvisMoveSpeedMod": {
      "Key": "InvisMoveSpeedMod",
      "Type": "move_speed",
      "Value": "3m"
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "SpottedRadius": {
      "Key": "SpottedRadius",
      "Value": "20m"
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_disarm" title="upgrade_disarm" -->

## upgrade_disarm

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_disarm`
- Snapshot ID: `40326`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_disarm aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_disarm`
- Payload Hash: `779ef164cdd60794bb382464e96cca2f1460a9a07597025256580c10a78051cc`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.398332+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "17m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 26.0,
    "DescKey": "#upgrade_disarm_desc",
    "Main": [
      {
        "Key": "FireRateSlow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "SpiritSteal",
        "Value": 22
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_disarm",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SpiritStealDuration": {
      "Key": "SpiritStealDuration",
      "Type": "duration",
      "Value": 6
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_double_jump" title="upgrade_double_jump" -->

## upgrade_double_jump

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_double_jump`
- Snapshot ID: `40330`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_double_jump aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_double_jump`
- Payload Hash: `fb2c553ea20ba636d40b9ea2afa5463d9d3e2bf3af02e71fa6dbb70583071572`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.405660+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_double_jump",
  "Name": null,
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 8.5
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AirJumpVerticalSpeedPercent": {
      "Key": "AirJumpVerticalSpeedPercent",
      "Type": "move_speed",
      "Value": 75
    },
    "AirJumps": {
      "Key": "AirJumps",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "InterruptCooldown": {
      "Key": "InterruptCooldown",
      "Type": "cooldown",
      "Value": 4
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_drum_magazine" title="upgrade_drum_magazine" -->

## upgrade_drum_magazine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_drum_magazine`
- Snapshot ID: `40332`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_drum_magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_drum_magazine`
- Payload Hash: `5af4f0c52c0e5846fbce0692046a6b239d5317f1122d5c8d40731fb8c8b4be3d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.409096+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_drum_magazine",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusClipSize": {
      "Key": "BonusClipSize",
      "Value": 20
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ReloadSpeedMultipler": {
      "Key": "ReloadSpeedMultipler",
      "Value": -25
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_duration_extender" title="upgrade_duration_extender" -->

## upgrade_duration_extender

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_duration_extender`
- Snapshot ID: `40333`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_duration_extender aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_duration_extender`
- Payload Hash: `bb1090a76244e0a2f151d6230981859f2e59362a9b8124e7b284570ca9f54f80`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.410374+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_duration_extender_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": true,
  "Key": "upgrade_duration_extender",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusAbilityDurationPercent": {
      "Key": "BonusAbilityDurationPercent",
      "Type": "duration",
      "Value": 20
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_fire_rate_aura" title="upgrade_fire_rate_aura" -->

## upgrade_fire_rate_aura

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_fire_rate_aura`
- Snapshot ID: `40344`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_fire_rate_aura aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_fire_rate_aura`
- Payload Hash: `b32d05338a5746e30704c4293bd378fbb03d55490aebc44c87ad4622ed99eb1b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.432765+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Value": 10
      },
      {
        "Key": "BonusFireRateNPC",
        "Value": 35
      },
      {
        "Key": "AuraRadius",
        "Type": "distance",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_fire_rate_aura_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_fire_rate_aura",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_frenzy" title="upgrade_frenzy" -->

## upgrade_frenzy

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_frenzy`
- Snapshot ID: `40347`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_frenzy aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_frenzy`
- Payload Hash: `c11777483f9607bb8fbfeb91578afaf83069f4e2260a39fc8b46521bdbd2aa3b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.438208+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_frenzy_vampire_desc",
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "15m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_frenzy_aura_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_frenzy",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "KillBonusMoveSpeedPerStack": {
      "Key": "KillBonusMoveSpeedPerStack",
      "Type": "move_speed",
      "Value": "2m"
    },
    "LowHealthLifeStealPercent": {
      "Key": "LowHealthLifeStealPercent",
      "Type": "healing",
      "Value": 45
    },
    "LowHealthPercentThreshold": {
      "Key": "LowHealthPercentThreshold",
      "Value": 45
    },
    "MaxKillBonusMoveSpeedStack": {
      "Key": "MaxKillBonusMoveSpeedStack",
      "Value": 3
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_full_spectrum" title="upgrade_full_spectrum" -->

## upgrade_full_spectrum

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_full_spectrum`
- Snapshot ID: `40348`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_full_spectrum aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_full_spectrum`
- Payload Hash: `81df192fbaf11eeaebbc1c9aacbf17607c4d07ecbe1c3cb4bd4b0d84d0572a23`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.439644+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusDamagePerHero",
        "Type": "tech_damage",
        "Value": 5
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_full_spectrum_passive",
    "Main": [],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 60
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_full_spectrum",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 60
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_galvanic_storm" title="upgrade_galvanic_storm" -->

## upgrade_galvanic_storm

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_galvanic_storm`
- Snapshot ID: `40350`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_galvanic_storm aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_galvanic_storm`
- Payload Hash: `e09e894ef62e2fdfe001e7d47acbe0ca30cf3ee487c303887dc8d2f4bf7dd02b`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.442837+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusPerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.457002
        },
        "Type": "tech_damage",
        "Value": 75.0
      },
      {
        "Key": "ProcChance",
        "Value": 30
      },
      {
        "Key": "ChainRadius",
        "Type": "distance",
        "Value": "7m"
      },
      {
        "Key": "ChainCount",
        "Value": 7
      },
      {
        "Key": "ChainTickRate",
        "Value": 0.2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_galvanic_storm_passive1",
    "Main": [
      {
        "Key": "DamagePerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.304668
        },
        "Type": "tech_damage",
        "Value": 50.0
      }
    ],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffMoveSpeedBonus",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3m"
      },
      {
        "Key": "GalvanicBuffDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_galvanic_storm_passive2",
    "Main": [
      {
        "Key": "BuffDamageMult",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_regenerating_tech_shield",
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_galvanic_storm",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ExplodeRadius": {
      "Key": "ExplodeRadius",
      "Type": "distance",
      "Value": "10m"
    },
    "GalvanicDebuffDuration": {
      "Key": "GalvanicDebuffDuration",
      "Type": "duration",
      "Value": 5
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 0.4
    },
    "TechShieldMaxHealth": {
      "Key": "TechShieldMaxHealth",
      "Type": "tech_armor_up",
      "Value": 400
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health_regen_1" title="upgrade_health_regen_1" -->

## upgrade_health_regen_1

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_regen_1`
- Snapshot ID: `40369`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_health_regen_1 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_health_regen_1`
- Payload Hash: `6d3a26b6958cbb12011a341acc72821fa9801c61d5a2ec29db6c161b61c756b6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.475805+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 2.75
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_health_regen_1",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_health_regen_aura" title="upgrade_health_regen_aura" -->

## upgrade_health_regen_aura

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_regen_aura`
- Snapshot ID: `40370`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_health_regen_aura aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_health_regen_aura`
- Payload Hash: `e0c29db35970c68bd274e439dc62661bd9547c15501fd06eacffd98c0e44471d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.477556+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "HealRadius",
        "Type": "distance",
        "Value": "9m"
      },
      {
        "Key": "HealInterval",
        "Type": "cooldown",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_health_regen_aura_desc",
    "Main": [
      {
        "Key": "HealAmount",
        "Type": "healing",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_health_regen_aura",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_high_impact_armor" title="upgrade_high_impact_armor" -->

## upgrade_high_impact_armor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_high_impact_armor`
- Snapshot ID: `40373`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_high_impact_armor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_high_impact_armor`
- Payload Hash: `84a59bd317027996c4f8206d31e1b52cc1ac162fa1f55626e1555cabb517833d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.482423+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_high_impact_armor",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DamageReduction": {
      "Key": "DamageReduction",
      "Value": 60
    },
    "DamageThreshold": {
      "Key": "DamageThreshold",
      "Value": 40
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_imbued_ability" title="upgrade_imbued_ability" -->

## upgrade_imbued_ability

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_imbued_ability`
- Snapshot ID: `40377`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_imbued_ability aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_imbued_ability`
- Payload Hash: `1aa125965d8003da4f68312defb378f45c1a6ddbdff701881aa4d4e45e8e68aa`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.489138+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_imbued_ability_passive1",
    "Main": [],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "UsageFlags": "ConditionallyApplied",
        "Value": 15
      },
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_imbued_ability_passive2",
    "Main": [],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": true,
  "Key": "upgrade_imbued_ability",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ImbuedBonusDamage": {
      "Key": "ImbuedBonusDamage",
      "Type": "ETechPower",
      "Value": 10
    },
    "ImbuedBonusDuration": {
      "Key": "ImbuedBonusDuration",
      "Type": "duration",
      "Value": 25
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_long_range_slowing_tech" title="upgrade_long_range_slowing_tech" -->

## upgrade_long_range_slowing_tech

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_long_range_slowing_tech`
- Snapshot ID: `40391`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_long_range_slowing_tech aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_long_range_slowing_tech`
- Payload Hash: `18924d3cc5352c2df8942d685b60350f2bf9866852bf63d1ec21d3c96e5de199`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.512822+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_long_range_slowing_tech_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_long_range_slowing_tech",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 30
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_magic_clarity" title="upgrade_magic_clarity" -->

## upgrade_magic_clarity

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_clarity`
- Snapshot ID: `40394`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_magic_clarity aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_magic_clarity`
- Payload Hash: `be76a2e855bd3713ad79ce0dddfc5a3d4abff0c3c0605b5f17cf5f3fd133a18c`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.517496+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "On ability or item use, gain a <span class=\"highlight\">move speed</span> bonus for {s:AbilityDuration}s. Your next ability cast will remove the move speed bonus and apply a <span class=\"highlight\">Spirit</span> bonus to that ability.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_magic_clarity_desc",
    "Main": [
      {
        "Key": "BonusSpirit",
        "LocTokenOverride": "ClarityBonusSpirit",
        "Type": "tech_damage",
        "Value": 28
      },
      {
        "Key": "BonusMovespeed",
        "Type": "movement_speed",
        "Value": "2m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_magic_clarity",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 8
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusSpiritMaxTime": {
      "Key": "BonusSpiritMaxTime",
      "Value": 12
    },
    "BonusSpiritMin": {
      "Key": "BonusSpiritMin",
      "Value": 2
    },
    "BonusSpiritWindow": {
      "Key": "BonusSpiritWindow",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_mod_disruptor" title="upgrade_mod_disruptor" -->

## upgrade_mod_disruptor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_mod_disruptor`
- Snapshot ID: `40407`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_mod_disruptor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_mod_disruptor`
- Payload Hash: `8312a84fbdf80d7618451fedf66e82799f8aefae9372225c7e4de0dbaeb7ff5d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.539311+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "DPSIncrease",
        "Scale": {
          "Type": "spirit",
          "Value": 0.030467
        },
        "Type": "tech_damage",
        "Value": 10.0
      },
      {
        "Key": "DPSMax",
        "Scale": {
          "Type": "spirit",
          "Value": 0.304668
        },
        "Type": "tech_damage",
        "Value": 100.0
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "8m"
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -18
      }
    ],
    "ChargeUp": null,
    "Cooldown": 32.0,
    "DescKey": "#upgrade_mod_disruptor_desc",
    "Main": [
      {
        "Key": "DPS",
        "Scale": {
          "Type": "spirit",
          "Value": 0.152334
        },
        "Type": "tech_damage",
        "Value": 50.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_mod_disruptor",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_nearby_enemy_boost" title="upgrade_nearby_enemy_boost" -->

## upgrade_nearby_enemy_boost

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_nearby_enemy_boost`
- Snapshot ID: `40411`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_nearby_enemy_boost aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_nearby_enemy_boost`
- Payload Hash: `7084c37d1aebf6febea12e29d8e2acfd879ec365e76133d09f828a499c1cdbb7`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.546514+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "1m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_nearby_enemy_boost_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 48
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_nearby_enemy_boost",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MinTargetsRequired": {
      "Key": "MinTargetsRequired",
      "Value": 2
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_predator_precision" title="upgrade_predator_precision" -->

## upgrade_predator_precision

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_predator_precision`
- Snapshot ID: `40419`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_predator_precision aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_predator_precision`
- Payload Hash: `4416c63cb0e950f0fb872d5bc9d4202b3b2b9e2b139952c2f20bcb5318992b18`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.560213+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_predator_precision_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50
      },
      {
        "Key": "LifeThreshold",
        "Value": 50
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_predator_precision",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_proc_disarm" title="upgrade_proc_disarm" -->

## upgrade_proc_disarm

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_proc_disarm`
- Snapshot ID: `40422`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_proc_disarm aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_proc_disarm`
- Payload Hash: `e46c25d7dd6937c7cfc7cfb7e8bd4c0c8e0dba120e7db092293e8a0bde701155`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.564831+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 1.0
      },
      {
        "Key": "BuildUpDuration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "ImmunityDuration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_proc_disarm_desc",
    "Main": [
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_proc_disarm",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_proc_tech_damage" title="upgrade_proc_tech_damage" -->

## upgrade_proc_tech_damage

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_proc_tech_damage`
- Snapshot ID: `40424`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_proc_tech_damage aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_proc_tech_damage`
- Payload Hash: `ab3cd9626ab6b174618c11acea9bb24536bd733e89051fd51a1da7705a94823d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.568026+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseDamagePct",
        "Scale": {
          "Type": "spirit",
          "Value": 3.04668
        },
        "Type": "tech_damage",
        "Value": 0.0001
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1,
    "DescKey": "#upgrade_proc_tech_damage_desc",
    "Main": [
      {
        "Key": "NanoTechPerShot",
        "Scale": {
          "Type": "spirit",
          "Value": 3.04668
        },
        "Type": "tech_damage",
        "Value": 1.0
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_proc_tech_damage",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BaseDamagePerShot": {
      "Key": "BaseDamagePerShot",
      "Scale": {
        "Type": "spirit",
        "Value": 1.0
      },
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "SpellAmplificationMultiplier": {
      "Key": "SpellAmplificationMultiplier",
      "Value": 5
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_quarantine" title="upgrade_quarantine" -->

## upgrade_quarantine

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_quarantine`
- Snapshot ID: `40425`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_quarantine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_quarantine`
- Payload Hash: `6bb4a42922bbe61ad58eb895c75a7578b1130b00b9a4d5b0bd9707e6203348d5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.569310+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.5
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 42.0,
    "DescKey": "#upgrade_quarantine_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_quarantine",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_regenerative_armor" title="upgrade_regenerative_armor" -->

## upgrade_regenerative_armor

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_regenerative_armor`
- Snapshot ID: `40433`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_regenerative_armor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_regenerative_armor`
- Payload Hash: `6e693d9e6b80c6fdff40ef967e4c2e636151e738c668ce62fccbdaa6bcef8c3d`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.583566+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_regenerative_armor",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 5
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusHealthRegen": {
      "Key": "BonusHealthRegen",
      "Type": "healing",
      "UsageFlags": "ConditionallyApplied",
      "Value": 20
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_reload_speed" title="upgrade_reload_speed" -->

## upgrade_reload_speed

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_reload_speed`
- Snapshot ID: `40435`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_reload_speed aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_reload_speed`
- Payload Hash: `3cf1d47198e8e006fa2d35cdba520477e042d0ada3e314e5d696911cc996d943`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.586481+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_reload_speed",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ReloadSpeedMultipler": {
      "Key": "ReloadSpeedMultipler",
      "Value": -35
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_slow_immunity" title="upgrade_slow_immunity" -->

## upgrade_slow_immunity

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_slow_immunity`
- Snapshot ID: `40454`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_slow_immunity aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_slow_immunity`
- Payload Hash: `661d7fc2c3a38c295fdeb44da15d482adc3812018f3cd130ec772bc1801412c1`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.620102+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_slow_immunity",
  "Name": null,
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 32.0
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 6
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_slowing_tech" title="upgrade_slowing_tech" -->

## upgrade_slowing_tech

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_slowing_tech`
- Snapshot ID: `40456`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_slowing_tech aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_slowing_tech`
- Payload Hash: `34fd2f54ef49f8375b64f551b842167ea90644d7bad9c3459888e98f85512d17`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.623998+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_slowing_tech_desc",
    "Main": [
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 15
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_slowing_tech",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy"
  ],
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_small_attack_speed" title="upgrade_small_attack_speed" -->

## upgrade_small_attack_speed

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_small_attack_speed`
- Snapshot ID: `40457`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_small_attack_speed aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_small_attack_speed`
- Payload Hash: `a75e069ddc51cbd42b35944e73703405debd6ab164d84440dbe64e8ab07fa688`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.625381+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusFireRate",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_small_attack_speed",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_stabilizer" title="upgrade_stabilizer" -->

## upgrade_stabilizer

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_stabilizer`
- Snapshot ID: `40468`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_stabilizer aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_stabilizer`
- Payload Hash: `493aec1c60114e36ca71c3b84a78b88ad55ab2c24b3116388ad3cabc239015c0`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.661398+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_stabilizer_desc",
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_stabilizer",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileShootingSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileZoomedSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "Value": 100
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_stabilizing_tripod" title="upgrade_stabilizing_tripod" -->

## upgrade_stabilizing_tripod

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_stabilizing_tripod`
- Snapshot ID: `40469`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_stabilizing_tripod aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_stabilizing_tripod`
- Payload Hash: `7a1af5a7269c738ed89654dd8e15ecac79a79c4f4256e5ccee213a594c817346`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.662935+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "StationaryWeaponPower",
        "Value": 40
      },
      {
        "Key": "StationaryRecoilReduction",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "TurretAttackRange",
        "Type": "distance",
        "Value": "50m"
      },
      {
        "Key": "CasterHealthPercent",
        "Value": 100
      }
    ],
    "ChargeUp": null,
    "Cooldown": 64.0,
    "DescKey": "#upgrade_stabilizing_tripod_desc",
    "Main": [
      {
        "Key": "TurretLifetime",
        "Type": "duration",
        "Value": 20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_stabilizing_tripod",
  "Name": null,
  "Other": {
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "15m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 20
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AttackConeAngle": {
      "Key": "AttackConeAngle",
      "Value": 10
    },
    "BulletSpeedOverride": {
      "Key": "BulletSpeedOverride",
      "Value": 6500
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ModelScale": {
      "Key": "ModelScale",
      "Value": 1.0
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    },
    "TrackingSpeed": {
      "Key": "TrackingSpeed",
      "Value": 125
    },
    "TurretAttackDelay": {
      "Key": "TurretAttackDelay",
      "Value": 0.35
    },
    "TurretDeployTime": {
      "Key": "TurretDeployTime",
      "Type": "duration",
      "Value": 2.0
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_stasis_bomb" title="upgrade_stasis_bomb" -->

## upgrade_stasis_bomb

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_stasis_bomb`
- Snapshot ID: `40470`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_stasis_bomb aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_stasis_bomb`
- Payload Hash: `2e1c9051b724b3232f15b61944386b2f635ef90feef9115744c907c5e4292006`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.664605+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "StasisRadius",
        "Type": "distance",
        "Value": "7m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 37.0,
    "DescKey": "#upgrade_stasis_bomb_active1",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 1.5
      },
      {
        "Key": "ExplodeDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 1.52334
        },
        "Type": "tech_damage",
        "Value": 250.0
      },
      {
        "Key": "StasisRadius",
        "Type": "distance",
        "Value": "7m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_stasis_bomb_active2",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "EMPDuration",
        "Value": 3
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 1.6
      },
      {
        "Key": "BuildUpDuration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "ImmunityDuration",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_stasis_bomb_desc_passive",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_stasis_bomb",
  "Name": null,
  "Other": {
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "20m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 6
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "DampingFactor": {
      "Key": "DampingFactor",
      "Value": 0.5
    },
    "EMPProcChance": {
      "Key": "EMPProcChance",
      "Value": 100
    },
    "ExplodeRadius": {
      "Key": "ExplodeRadius",
      "Type": "distance",
      "Value": "7m"
    },
    "LiftHeight": {
      "Key": "LiftHeight",
      "Value": 120
    },
    "MoveSpeedMax": {
      "Key": "MoveSpeedMax",
      "Type": "slow",
      "Value": "4m"
    },
    "SlamdownSpeed": {
      "Key": "SlamdownSpeed",
      "Value": 500
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_tech_cleave" title="upgrade_tech_cleave" -->

## upgrade_tech_cleave

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_cleave`
- Snapshot ID: `40478`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_tech_cleave aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_tech_cleave`
- Payload Hash: `1f18c04ab58a9c67be8667506b7364dd5d53fa98a4f82c91e0885ed894ebd59e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.680482+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "TechCleaveRadius",
        "Type": "distance",
        "Value": "10m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_tech_cleave_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_tech_cleave",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 0.05
    },
    "TechCleaveDamagePercent": {
      "Key": "TechCleaveDamagePercent",
      "Type": "tech_damage",
      "Value": 75
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_toughness_3" title="upgrade_toughness_3" -->

## upgrade_toughness_3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_toughness_3`
- Snapshot ID: `40487`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_toughness_3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_toughness_3`
- Payload Hash: `fcb3cdec6a4cbd3aef39e965c2ed6905c3819670a5d61d3868c044426774cf0c`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.697491+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_toughness_3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_detention_ammo" title="upgrade_weapon_detention_ammo" -->

## upgrade_weapon_detention_ammo

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_detention_ammo`
- Snapshot ID: `40499`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_weapon_detention_ammo aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_weapon_detention_ammo`
- Payload Hash: `42c31378a6db0ef12b3a9876e38e9d331453408ec09257cc404683666166b5ea`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.720033+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "BonusFireRate",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuildUpPerShot",
        "Value": 1.2
      },
      {
        "Key": "TetherRadius",
        "Type": "distance",
        "Value": "7m"
      },
      {
        "Key": "ImmunityDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_weapon_detention_ammo_desc",
    "Main": [
      {
        "Key": "TetherDuration",
        "Type": "duration",
        "Value": 2.3
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_weapon_detention_ammo",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 10
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_eater" title="upgrade_weapon_eater" -->

## upgrade_weapon_eater

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_eater`
- Snapshot ID: `40500`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_weapon_eater aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_weapon_eater`
- Payload Hash: `815bd228066a41b15170f1f29fd450935eb76634de17011fa194e4845f77d0b5`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.721370+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_weapon_eater_desc",
    "Main": [],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_weapon_eater",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BaseBonusCounter": {
      "Key": "BaseBonusCounter",
      "Type": "bullet_damage",
      "Value": -5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "KillWindow": {
      "Key": "KillWindow",
      "Value": 1
    },
    "MaxWeaponPower": {
      "Key": "MaxWeaponPower",
      "Type": "bullet_damage",
      "Value": 30
    },
    "WeaponPowerPerDeath": {
      "Key": "WeaponPowerPerDeath",
      "Type": "bullet_damage",
      "Value": 6
    },
    "WeaponPowerPerKill": {
      "Key": "WeaponPowerPerKill",
      "Type": "bullet_damage",
      "Value": 3.5
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_instant_reload" title="upgrade_weapon_instant_reload" -->

## upgrade_weapon_instant_reload

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_instant_reload`
- Snapshot ID: `40501`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_weapon_instant_reload aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_weapon_instant_reload`
- Payload Hash: `98225bff69e380a21477fba76ee08c3e414fe653bc1d4dbaec38713a84584591`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.722827+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_weapon_instant_reload",
  "Name": null,
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 21.0
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusClipSizePercent": {
      "Key": "BonusClipSizePercent",
      "Value": 60
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_overdrive_clip" title="upgrade_weapon_overdrive_clip" -->

## upgrade_weapon_overdrive_clip

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_overdrive_clip`
- Snapshot ID: `40502`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_weapon_overdrive_clip aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_weapon_overdrive_clip`
- Payload Hash: `1aa81c7378a5c23a1ed45a99c7d6ea4d3db56c6d1f905e0dfeca0f16d06f4984`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.724122+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "HealthDamagePercent",
        "Type": "tech_damage",
        "Value": 20
      },
      {
        "Key": "BonusWeaponPower",
        "Type": "bullet_damage",
        "Value": 100
      },
      {
        "Key": "OverdriveClipDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 53.0,
    "DescKey": "#upgrade_weapon_overdrive_clip_desc",
    "Main": [
      {
        "Key": "BonusReloadSpeed",
        "Type": "time",
        "Value": -75
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_weapon_overdrive_clip",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="upgrade_weapon_power_and_health_drain" title="upgrade_weapon_power_and_health_drain" -->

## upgrade_weapon_power_and_health_drain

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_power_and_health_drain`
- Snapshot ID: `40503`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_weapon_power_and_health_drain aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `upgrade_weapon_power_and_health_drain`
- Payload Hash: `4bbf3064b2736b50455f73cbe4ec3cb6c1ea9f860cd6d3498e2190eaf40676fd`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.725578+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "HealthDrainedPerSecond",
        "Value": 35
      }
    ],
    "ChargeUp": null,
    "Cooldown": 11.5,
    "DescKey": "#upgrade_weapon_power_and_health_drain_desc",
    "Main": [
      {
        "Key": "WeaponPowerWhileActivated",
        "Type": "bullet_damage",
        "Value": 135
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_weapon_power_and_health_drain",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "Interval": {
      "Key": "Interval",
      "Value": 0.1
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="weapon_upgrade_base" title="weapon_upgrade_base" -->

## weapon_upgrade_base

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `weapon_upgrade_base`
- Snapshot ID: `40507`
- Source-Dokument: `7073`
- Kurzinfo: weapon_upgrade_base aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `weapon_upgrade_base`
- Payload Hash: `573a91602358dc42060f2c1e53178e9468c0d6c4987755666a67585437f15949`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.732282+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "weapon_upgrade_base",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="weapon_upgrade_t1" title="weapon_upgrade_t1" -->

## weapon_upgrade_t1

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `weapon_upgrade_t1`
- Snapshot ID: `40508`
- Source-Dokument: `7073`
- Kurzinfo: weapon_upgrade_t1 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `weapon_upgrade_t1`
- Payload Hash: `6592c52a949e4a6d42f0d2c87ee5ad061e4a9b82b089fe7d2a0d79b83b16f0d9`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.733807+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "weapon_upgrade_t1",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="weapon_upgrade_t2" title="weapon_upgrade_t2" -->

## weapon_upgrade_t2

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `weapon_upgrade_t2`
- Snapshot ID: `40509`
- Source-Dokument: `7073`
- Kurzinfo: weapon_upgrade_t2 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `weapon_upgrade_t2`
- Payload Hash: `99d5fb930abc5b3f61b77037d50947d506d31f3037784583a11e2807e2a92be3`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.735333+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "weapon_upgrade_t2",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="weapon_upgrade_t3" title="weapon_upgrade_t3" -->

## weapon_upgrade_t3

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `weapon_upgrade_t3`
- Snapshot ID: `40510`
- Source-Dokument: `7073`
- Kurzinfo: weapon_upgrade_t3 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `weapon_upgrade_t3`
- Payload Hash: `ba73fe1c3296fb2a629cd4e2e7dc6c4f2431af98e2c28f6c7df69b87e139125e`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.737040+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "weapon_upgrade_t3",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="weapon_upgrade_t4" title="weapon_upgrade_t4" -->

## weapon_upgrade_t4

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `weapon_upgrade_t4`
- Snapshot ID: `40511`
- Source-Dokument: `7073`
- Kurzinfo: weapon_upgrade_t4 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `weapon_upgrade_t4`
- Payload Hash: `6216327925efcd40be4b2dfdee985a96145d101a44cfc399f64ad1d63402e8e6`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.738571+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "weapon_upgrade_t4",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item_card" external_id="weapon_upgrade_t5" title="weapon_upgrade_t5" -->

## weapon_upgrade_t5

### Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `weapon_upgrade_t5`
- Snapshot ID: `40512`
- Source-Dokument: `7073`
- Kurzinfo: weapon_upgrade_t5 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `weapon_upgrade_t5`
- Payload Hash: `7027bcb15659a9d91f8ba803cd296c170c749c90def36a59b8970c5f5d1a622a`
- Source Content Hash: `9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json`
- Fetched At: `2026-07-09T19:36:25.739838+00:00`

### Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "weapon_upgrade_t5",
  "Name": null,
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 5,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
      "ProductName": "citadel",
      "ServerAppID": "1422460",
      "ServerVersion": "6624",
      "SourceRevision": "10806932",
      "ToolsAppID": "211",
      "VersionDate": "Jul 08 2026",
      "VersionTime": "08:23:51",
      "appID": "1422450"
    }
  }
}
````

