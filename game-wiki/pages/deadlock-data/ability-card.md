---
title: "ability card"
generated_at: "2026-07-21T18:03:55.468852924+00:00"
entries: 203
---

# ability card

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_swan_acrobat" title="Acrobat" -->

## Acrobat

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_swan_acrobat`
- Snapshot ID: `39897`
- Source-Dokument: `7071`
- Kurzinfo: Acrobat aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Acrobat`
- Payload Hash: `837f8a8e6f5e20b170f54ce7695909d6713a41ee857d471ac57b488b5cdd34fd`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.417725+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 7
  },
  "DescKey": "ability_swan_acrobat_desc",
  "HeroKey": "hero_swan",
  "HeroName": "Swan",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_swan_acrobat_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxStacks",
          "Name": "Max Stacks",
          "Type": "cast",
          "Value": 4
        },
        {
          "Key": "BurstBonusPerStack",
          "Name": "Weapon Burst Bonus",
          "Title": "On Buff:",
          "Type": "bullet_damage",
          "Value": 1
        },
        {
          "Key": "FireRatePerStack",
          "Name": "Fire Rate per Stack",
          "Title": "On Buff:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_swan_acrobat",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Acrobat",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "FireRatePerStack": 4
    },
    {
      "MaxStacks": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_swan",
      "hero_name": "Swan",
      "lookup": "acrobat",
      "name": "Acrobat",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="synth_affliction" title="Affliction" -->

## Affliction

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_affliction`
- Snapshot ID: `39902`
- Source-Dokument: `7071`
- Kurzinfo: Affliction aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Affliction`
- Payload Hash: `d196d65e09c654ce779488fdcc853792aae12319913030339832ad16565be1f8`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.426318+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.6
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 170.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_affliction_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Name": "Healing Reduction",
        "Value": 0
      }
    ],
    "DescKey": "synth_affliction_desc",
    "Main": {
      "Props": [
        {
          "Key": "CurrentHealthDamage",
          "Name": "Current Health Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.22
          },
          "Type": "tech_damage",
          "Value": 34
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Type": "duration",
          "Value": 11
        }
      ]
    }
  },
  "Key": "synth_affliction",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Affliction",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CanBePurged": {
      "Name": null,
      "Value": 1
    },
    "DamageInterval": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 9
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -35
    },
    {
      "DebuffDuration": 3,
      "DescKey": "synth_affliction_t2_desc",
      "Radius": 4
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.11
        },
        "Value": 14
      },
      "DescKey": "synth_affliction_t3_desc",
      "DisableHealing": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "affliction",
      "name": "Affliction",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_afterburn" title="Afterburn" -->

## Afterburn

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_afterburn`
- Snapshot ID: `39833`
- Source-Dokument: `7071`
- Kurzinfo: Afterburn aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Afterburn`
- Payload Hash: `f1283b2ac32ec372c8aba104078c4833f0aed2fc00dbd86a5e0caa397c741a0a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.290826+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_afterburn_desc",
  "Duration": {
    "BuildUpDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 17
    }
  },
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [
      {
        "Key": "BuildUpBulletPercentPerHit",
        "Name": "Buildup Per Bullet",
        "Type": "cast",
        "Value": 8.1
      },
      {
        "Key": "CritBuildup",
        "Name": "Buildup Per Headshot",
        "Type": "cast",
        "Value": 15.4
      },
      {
        "Key": "RefillDuration",
        "Type": "duration",
        "Value": 0.5
      },
      {
        "Key": "RefillDurationCrit",
        "Name": "Extend Per Headshot",
        "Type": "duration",
        "Value": 1.0
      }
    ],
    "DescKey": "ability_afterburn_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.66
          },
          "Title": "Burn Effect:",
          "Type": "tech_damage",
          "Value": 14.0
        },
        {
          "Key": "BurnDurationBase",
          "Name": "Unknown(BurnDurationBase)",
          "Title": "Burn Effect:",
          "Type": "duration",
          "Value": 3
        },
        {
          "Key": "BurnDuration",
          "Name": "Burn Duration",
          "Title": "Burn Effect:",
          "Type": "duration",
          "Value": 3
        },
        {
          "Key": "OutgoingTechDamagePercent",
          "Name": "Spirit Damage",
          "Title": "Burn Effect:",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_afterburn",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Afterburn",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DPS": 16
    },
    {
      "DescKey": "ability_afterburn_t2_desc",
      "OutgoingTechDamagePercent": -35
    },
    {
      "BurnDuration": 3,
      "DescKey": "ability_afterburn_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "afterburn",
      "name": "Afterburn",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_tengu_airlift" title="Air Drop" -->

## Air Drop

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_airlift`
- Snapshot ID: `39906`
- Source-Dokument: `7071`
- Kurzinfo: Air Drop aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Air Drop`
- Payload Hash: `194cc896f8bf075b799f8474fc31b7f52c1cb17bfb148e10cee29e9896c82659`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.433333+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 22
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 100.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 21.0
  },
  "Damage": {
    "AllyOutgoingDamagePercent": {
      "Name": null,
      "Type": "damage",
      "Value": -20
    }
  },
  "DescKey": "citadel_ability_tengu_airlift_desc",
  "Duration": {
    "AllyCastDelay": {
      "Name": null,
      "Type": "duration",
      "Value": 0.1
    }
  },
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [
      {
        "Key": "OnLandDamageRadius",
        "Name": "Landing Radius",
        "Type": "distance",
        "Value": 20
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BulletArmorReduction",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_down",
        "Value": 0
      },
      {
        "Key": "BulletArmorReductionDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "SilenceDuration",
        "Name": "Silence Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_tengu_airlift_desc",
    "Main": {
      "Props": [
        {
          "Key": "ExplodeDamage",
          "Name": "Explode Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7
          },
          "Type": "tech_damage",
          "Value": 115.0
        },
        {
          "Key": "AirDropOutgoingDamagePercent",
          "Name": "Outgoing Damage Bonus",
          "Type": "damage",
          "Value": 20
        },
        {
          "Key": "AirDropBulletShield",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0
          },
          "Type": "combat_barrier",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_tengu_airlift",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Air Drop",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CooldownReductionPctOnOthers": {
      "Name": null,
      "Value": 30
    },
    "InterruptCooldown": {
      "Name": "Interrupt Cooldown",
      "Value": 3.5
    },
    "OnLandDamageRadiusStart": {
      "Name": null,
      "Value": 16
    },
    "SilenceBombSpeed": {
      "Name": null,
      "Value": 12
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 300
      },
      "DescKey": "citadel_ability_tengu_airlift_t1_desc"
    },
    {
      "DebuffDuration": 3,
      "DescKey": "citadel_ability_tengu_airlift_t2_desc",
      "SlowPercent": 40
    },
    {
      "AirDropBulletShield": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_tengu_airlift_t3_desc",
      "ExplodeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 0
      },
      "SilenceDuration": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "air drop",
      "name": "Air Drop",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_warden_crowd_control" title="Alchemical Flask" -->

## Alchemical Flask

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_crowd_control`
- Snapshot ID: `39931`
- Source-Dokument: `7071`
- Kurzinfo: Alchemical Flask aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Alchemical Flask`
- Payload Hash: `5bd06508781ba8d1dea2a16cad3bfe67b83560db4725dbe173d585c8b0a237a1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.482986+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_warden_crowd_control_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 7
      },
      {
        "Key": "StaminaReduction",
        "Name": "Stamina Reduction",
        "Value": 0
      }
    ],
    "DescKey": "ability_warden_crowd_control_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.63
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "MoveSpeedSlowPct",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "WeaponPowerDebuff",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": -25
        }
      ]
    }
  },
  "Key": "ability_warden_crowd_control",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Alchemical Flask",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ForwardVelocity": {
      "Name": null,
      "Value": 800
    },
    "ProjectileLifetime": {
      "Name": null,
      "Value": 60
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "ability_warden_crowd_control_t1_desc",
      "StaminaReduction": 1
    },
    {
      "Damage": 35,
      "DescKey": "ability_warden_crowd_control_t2_desc",
      "WeaponPowerDebuff": -25
    },
    {
      "AbilityCooldown": -7,
      "DescKey": "ability_warden_crowd_control_t3_desc",
      "FireRateSlow": 30,
      "Radius": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "alchemical flask",
      "name": "Alchemical Flask",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_icebeam" title="Arctic Beam" -->

## Arctic Beam

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_icebeam`
- Snapshot ID: `39837`
- Source-Dokument: `7071`
- Kurzinfo: Arctic Beam aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Arctic Beam`
- Payload Hash: `20290afdbad19961382d7c6eed6796c1ff398d166469fdc81ffa8d5584c8bb15`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.298630+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5.0
  },
  "Debuff": {
    "ChannelSlowPercent": {
      "Name": null,
      "Type": "slow",
      "Value": 8
    }
  },
  "DescKey": "ability_icebeam_desc",
  "Duration": {
    "SlowDuration": {
      "Name": "Slow Duration",
      "Type": "duration",
      "Value": 0.6
    }
  },
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxSlowTime",
        "Name": "Time To Max Debuff",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "PathLength",
        "Name": "Beam Length",
        "Type": "distance",
        "Value": 25
      },
      {
        "Key": "IceBeamBuildupProcDuration",
        "Name": "Debuff Linger Duration",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "BeamSplit",
        "Name": "Extra Beam Range",
        "Scale": {
          "Type": "range",
          "Value": 0.0
        },
        "Type": "radius",
        "Value": 0
      },
      {
        "Key": "MaxGroundDashReductionPercent",
        "Name": "Max Dash Slow",
        "Value": -20
      }
    ],
    "DescKey": "ability_icebeam_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.38
          },
          "Type": "tech_damage",
          "Value": 45.0
        },
        {
          "Key": "MaxSlowPercent",
          "Name": "Max Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "MaxFireRateSlowPercent",
          "Name": "Max Fire Rate",
          "Type": "fire_rate",
          "Value": 20
        }
      ]
    }
  },
  "Key": "ability_icebeam",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Arctic Beam",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "MinSlowPercent": {
      "Name": null,
      "Value": 30
    },
    "PathWidth": {
      "Name": null,
      "Value": 1.1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_icebeam_t1_desc",
      "MaxFireRateSlowPercent": 25,
      "MaxSlowPercent": 25
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 20
      },
      "DescKey": "ability_icebeam_t2_desc"
    },
    {
      "AbilityCooldown": -13,
      "BeamSplit": {
        "Scale": {
          "Type": "range",
          "Value": 0.93
        },
        "Value": 10
      },
      "BeamSplitCount": 2,
      "DescKey": "ability_icebeam_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "arctic beam",
      "name": "Arctic Beam",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_hornet_snipe" title="Assassinate" -->

## Assassinate

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_snipe`
- Snapshot ID: `39830`
- Source-Dokument: `7071`
- Kurzinfo: Assassinate aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Assassinate`
- Payload Hash: `4bd3007b3c023a3843acc266c4ee9560cd6428f646c8490efed0ce34e5df59ce`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.285455+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 55.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2.5
  },
  "DescKey": "citadel_ability_hornet_snipe_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [
      {
        "Key": "TimeToFullCharge",
        "Name": "Full Charge Time",
        "Type": "duration",
        "Value": 1.0
      },
      {
        "Key": "MinChargeDamagePercent",
        "Name": "No Charge Damage",
        "Value": 50
      },
      {
        "Key": "HeadshotBonus",
        "Name": "Headshot Damage",
        "Type": "bullet_damage",
        "Value": 20
      }
    ],
    "DescKey": "citadel_ability_hornet_snipe_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "LowHealthEnemyDamageBonus",
          "Name": "Max Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.3
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "WeaponDamageBonusPerKill",
          "Name": "Weapon Damage Per Kill",
          "Type": "bullet_damage",
          "Value": 6
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_snipe",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Assassinate",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusGoldOnKill": {
      "Name": "Bonus Souls Per Assassination",
      "Value": 250
    },
    "LowHealthEnemyThresholdPct": {
      "Name": "Low Health Threshold",
      "Value": 50
    },
    "MaxSoundDistance": {
      "Name": null,
      "Value": 2000
    },
    "MoveSpeed": {
      "Name": null,
      "Value": 4
    },
    "ShotRadius": {
      "Name": null,
      "Value": 4.0
    },
    "ViewPunch": {
      "Name": null,
      "Value": 2.5
    }
  },
  "Range": {
    "Range": {
      "Name": "Range",
      "Type": "distance",
      "Value": 1000
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "LowHealthEnemyDamageBonus": 80
    },
    {
      "WeaponDamageBonusPerKill": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "assassinate",
      "name": "Assassinate",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_magician_copyult" title="Audience Participation" -->

## Audience Participation

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_copyult`
- Snapshot ID: `39850`
- Source-Dokument: `7071`
- Kurzinfo: Audience Participation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Audience Participation`
- Payload Hash: `742dd4c48efe3f4b8216c6ac86aa351196d57b4ebeeb7750094e3a4dc2357c9a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.326244+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 85
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_magician_copyult_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_magician_copyult_desc",
    "Main": {
      "Props": [
        {
          "Key": "CopiedUltWindow",
          "Name": "Copy Duration",
          "Type": "time",
          "Value": 12
        },
        {
          "Key": "CopyCooldownPercentage",
          "Name": "Copied Cooldown",
          "Type": "cooldown",
          "Value": 40
        }
      ]
    }
  },
  "Key": "ability_magician_copyult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Audience Participation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CopyInternalCooldown": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DescKey": "ability_magician_copyult_t1_desc"
    },
    {
      "DescKey": "ability_magician_copyult_t2_desc"
    },
    {
      "DescKey": "ability_magician_copyult_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "audience participation",
      "name": "Audience Participation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_frank_painaura" title="Aura of Suffering" -->

## Aura of Suffering

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_painaura`
- Snapshot ID: `39805`
- Source-Dokument: `7071`
- Kurzinfo: Aura of Suffering aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Aura of Suffering`
- Payload Hash: `66eb744406fa898e7f5a0074bcc736185b8c0f256c12471dba988722ccbf61ee`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.238617+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 2.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "Damage": {
    "Damage": {
      "Name": "Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 1.1
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "DescKey": "ability_frank_painaura_desc",
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [
      {
        "Key": "SelfDamagePercentage",
        "Name": "Self Damage",
        "Type": "damage",
        "Value": 70
      },
      {
        "Key": "StatusResistancePercent",
        "Name": "Debuff Resist",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_frank_painaura_desc",
    "Main": {
      "Props": [
        {
          "Key": "MinDPS",
          "Name": "Minimum DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 0.15
          },
          "Type": "tech_damage",
          "Value": 13
        },
        {
          "Key": "MaxDPS",
          "Name": "Max DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 0.72
          },
          "Type": "tech_damage",
          "Value": 58
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Enemy Hit:",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "OutgoingDamagePercent",
          "Name": "Damage",
          "Title": "On Enemy Hit:",
          "Type": "damage",
          "Value": 0
        },
        {
          "Key": "IncomingDamagePercent",
          "Name": "Damage Taken",
          "Title": "On Enemy Hit:",
          "Type": "health",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_frank_painaura",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Aura of Suffering",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 0.5
    },
    "SelfDPS": {
      "Name": null,
      "Value": 15
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    },
    "ToggleOffDelay": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 8
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DebuffDuration": 0.5,
      "DescKey": "ability_frank_painaura_t1_desc",
      "EnemyDashSlowPercent": -25,
      "SlowPercent": 25
    },
    {
      "DescKey": "ability_frank_painaura_t2_desc",
      "MaxDPS": 34.0,
      "MinDps": 6
    },
    {
      "DescKey": "ability_frank_painaura_t3_desc",
      "IncomingDamagePercent": 15,
      "Radius": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "aura of suffering",
      "name": "Aura of Suffering",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_nano_catform" title="Ava" -->

## Ava

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_catform`
- Snapshot ID: `39857`
- Source-Dokument: `7071`
- Kurzinfo: Ava aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ava`
- Payload Hash: `e0c620a4ea3e44aeedd8b0f79fda60541715d3441d6b6c1172caae7ccb20872f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.340242+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_catform_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [
      {
        "Key": "SpeedBuildDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "InterruptCooldown",
        "Name": "Interrupt Cooldown",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "HealthRegen",
        "Name": "Health Regen",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "ability_nano_catform_desc",
    "Main": {
      "Props": [
        {
          "Key": "MinBonusMoveSpeedPercent",
          "Name": "Min Move Speed",
          "Type": "move_speed",
          "Value": 30
        },
        {
          "Key": "MaxBonusMoveSpeedPercent",
          "Name": "Max Move Speed",
          "Type": "move_speed",
          "Value": 65
        },
        {
          "Key": "BuffDuration",
          "Name": "Buff Duration",
          "Type": "duration",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_nano_catform",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Ava",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CatFormDamageDealtReduction": {
      "Name": null,
      "Value": -100
    },
    "EnemyDamageSpeedPenalty": {
      "Name": null,
      "Value": 65
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BuffDuration": 15
    },
    {
      "DescKey": "ability_nano_catform_t2_desc",
      "HealthRegen": 15,
      "MaxBonusMoveSpeedPercent": 45
    },
    {
      "DamageAmpBuildDuration": 10,
      "DamageAmpDuration": 6,
      "DescKey": "ability_nano_catform_t3_desc",
      "OutgoingDamagePercent": 20
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "ava",
      "name": "Ava",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="synth_barrage" title="Barrage" -->

## Barrage

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_barrage`
- Snapshot ID: `39899`
- Source-Dokument: `7071`
- Kurzinfo: Barrage aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Barrage`
- Payload Hash: `a0c42b0cc63850b498d14499316a3266b5c768a726a5d530035daf9253a23ef5`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.421468+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_barrage_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "ProjectileAmount",
        "Name": "Projectile Amount",
        "Value": 4
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 1.5
      },
      {
        "Key": "AmpDuration",
        "Name": "Amp Duration",
        "Type": "duration",
        "Value": 15
      }
    ],
    "DescKey": "synth_barrage_desc",
    "Main": {
      "Props": [
        {
          "Key": "AmpPercentPerStack",
          "Name": "Amp Per Stack",
          "Type": "damage",
          "Value": 6
        },
        {
          "Key": "DamagePerProjectile",
          "Name": "Damage Per Projectile",
          "Scale": {
            "Type": "spirit",
            "Value": 0.465
          },
          "Type": "tech_damage",
          "Value": 32
        },
        {
          "Key": "MoveSlowPercent",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 30
        }
      ]
    }
  },
  "Key": "synth_barrage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Barrage",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.3
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 2.54
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 10
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DamagePerProjectile": 16
    },
    {
      "AbilityCooldown": -16.0
    },
    {
      "AmpPercentPerStack": 4,
      "DescKey": "synth_barrage_t3_desc",
      "Radius": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "barrage",
      "name": "Barrage",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_punkgoat_ult" title="Bashdown" -->

## Bashdown

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_ult`
- Snapshot ID: `39875`
- Source-Dokument: `7071`
- Kurzinfo: Bashdown aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bashdown`
- Payload Hash: `013bfd052d58141afce49665d30f14de55dc64aa6b7a3fcb0713293d67f99277`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.374515+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 4
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 35
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "DescKey": "ability_punkgoat_ult_desc",
  "Duration": {
    "ExplodeDelay": {
      "Name": "Explosion Delay",
      "Type": "duration",
      "Value": 0.5
    },
    "PullDownDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.75
    }
  },
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [
      {
        "Key": "FireRateSlowDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "bullet_damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_punkgoat_ult_desc",
    "Main": {
      "Props": [
        {
          "Key": "MeleeDamage",
          "Name": "Melee Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.9
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "HeavyMeleeDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "heavy_melee",
            "Value": 0
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.1
          },
          "Type": "tech_damage",
          "Value": 35
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 0.4
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_punkgoat_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 4.826
    }
  },
  "Name": "Bashdown",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 2000
    },
    "CountsAsLightMelee": {
      "Name": null,
      "Value": 1
    },
    "TossForce": {
      "Name": null,
      "Value": 350
    }
  },
  "Range": {
    "PlaceDistanceInFrontOfCaster": {
      "Name": null,
      "Type": "distance",
      "Value": 6.2
    },
    "PullDownRange": {
      "Name": null,
      "Type": "distance",
      "Value": 3
    },
    "WaveEndRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 8.0
    },
    "WaveStartRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 0.5
    },
    "WaveThickness": {
      "Name": null,
      "Type": "distance",
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_punkgoat_ult_t1_desc"
    },
    {
      "AbilityCastRange": 2,
      "AbilityCharges": 1,
      "DescKey": "ability_punkgoat_ult_t2_desc"
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "CountsAsHeavyMelee": 1,
      "CountsAsLightMelee": -1,
      "DescKey": "ability_punkgoat_ult_t3_desc",
      "HeavyMeleeDamage": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.5
        },
        "Value": 0.0
      },
      "MeleeDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0.0
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "bashdown",
      "name": "Bashdown",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_warden_lock_down" title="Binding Word" -->

## Binding Word

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_lock_down`
- Snapshot ID: `39933`
- Source-Dokument: `7071`
- Kurzinfo: Binding Word aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Binding Word`
- Payload Hash: `24d05faafd297a2fe8350d07bb5347788490b9eeeed3298f40a35c78091f1250`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.487027+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34
  },
  "DescKey": "ability_warden_lock_down_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [
      {
        "Key": "EscapeTime",
        "Name": "Escape Time",
        "Type": "duration",
        "Value": 2.8
      },
      {
        "Key": "EscapeRange",
        "Name": "Escape Range",
        "Type": "distance",
        "Value": 20
      }
    ],
    "DescKey": "ability_warden_lock_down_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.437344
          },
          "Type": "tech_damage",
          "Value": 110
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "Type": "duration",
          "Value": 1.75
        }
      ]
    }
  },
  "Key": "ability_warden_lock_down",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Binding Word",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AdditionalTargetRadius": {
      "Name": null,
      "Value": 20
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BulletArmorReduction": 20,
      "BulletArmorReductionDuration": 5,
      "DescKey": "ability_warden_lock_down_t1_desc"
    },
    {
      "DescKey": "ability_warden_lock_down_t2_desc",
      "ImmobilizeDuration": 0.75
    },
    {
      "AbilityCooldown": -14,
      "DescKey": "ability_warden_lock_down_t3_desc",
      "SilenceDebuff": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "binding word",
      "name": "Binding Word",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_scrap_blast" title="Bio Blast" -->

## Bio Blast

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_scrap_blast`
- Snapshot ID: `39949`
- Source-Dokument: `7071`
- Kurzinfo: Bio Blast aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bio Blast`
- Payload Hash: `818a2d5c295d8bc769e8b825b470dd437722bf0f5269bd8ab452d4b7e75406d7`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.516755+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 64.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3
  },
  "DescKey": "ability_scrap_blast_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "EnemyMoveSlowDuration",
        "Name": "Enemy Slow Duration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "DescKey": "ability_scrap_blast_desc",
    "Main": {
      "Props": [
        {
          "Key": "ScrapDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.731203
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "EnemyMoveSlow",
          "Name": "Enemy Slow per hit",
          "Type": "slow",
          "Value": 10
        }
      ]
    }
  },
  "Key": "ability_scrap_blast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bio Blast",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "BlastRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 10
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "ScrapDamage": 55
    },
    {
      "EnemyMoveSlow": 20
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "bio blast",
      "name": "Bio Blast",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_punkgoat_blasted" title="Blasted" -->

## Blasted

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_blasted`
- Snapshot ID: `39877`
- Source-Dokument: `7071`
- Kurzinfo: Blasted aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blasted`
- Payload Hash: `a46a3081d352adb759a64f61945c1af10e4c8bf0cb8b373e071993ed2c1142da`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.378936+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.28
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 27
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8.0
  },
  "DescKey": "ability_punkgoat_blasted_desc",
  "Duration": {
    "BulletDamageAmpDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 7.0
    },
    "DurationPerHeavyMelee": {
      "Name": null,
      "Type": "duration",
      "Value": 4.5
    },
    "DurationPerLightMelee": {
      "Name": null,
      "Type": "duration",
      "Value": 2.8
    }
  },
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_punkgoat_blasted_desc",
    "Main": {}
  },
  "Info2": {
    "Alt": [
      {
        "Key": "HealthBoostDuration",
        "Type": "duration",
        "Value": 11
      }
    ],
    "DescKey": "ability_punkgoat_blastedactive_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxHealthMelee",
          "Name": "Melee Bonus Health",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Title": "While Blasted:",
          "Type": "healing",
          "Value": 70
        },
        {
          "Key": "BulletDamageAmp",
          "Name": "Wrecked Bullet Amp",
          "Title": "While Blasted:",
          "Type": "bullet_damage",
          "Value": 10
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "While Blasted:",
          "Type": "move_speed",
          "Value": 0.0
        }
      ]
    }
  },
  "Key": "ability_punkgoat_blasted",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Blasted",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlastedRateOnBulletPct": {
      "Name": null,
      "Value": 50
    },
    "BulletsReloadedPerHeavyMeleePct": {
      "Name": null,
      "Value": 100
    },
    "BulletsReloadedPerLightMeleePct": {
      "Name": null,
      "Value": 35
    },
    "LightMeleeScalePct": {
      "Name": null,
      "Value": 40
    },
    "MaxDuration": {
      "Name": null,
      "Value": 35.0
    },
    "NonPlayerResourceScalePct": {
      "Name": null,
      "Value": 25
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.25,
      "DescKey": "ability_punkgoat_blasted_t1_desc"
    },
    {
      "BulletDamageAmp": 7,
      "DescKey": "ability_punkgoat_blasted_t2_desc",
      "GainSlamOnUse": 1
    },
    {
      "DescKey": "ability_punkgoat_blasted_t3_desc",
      "MaxHealthMelee": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 50
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "blasted",
      "name": "Blasted",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="operative_blindside" title="Blindside" -->

## Blindside

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `operative_blindside`
- Snapshot ID: `39863`
- Source-Dokument: `7071`
- Kurzinfo: Blindside aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blindside`
- Payload Hash: `a464190ebf4dad34889b95254cbe594208dfdf986902dcc12f304aa80740b901`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.351651+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_operative",
  "HeroName": "Raven",
  "Info1": {
    "Alt": [],
    "DescKey": "operative_blindside_active_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.279
          },
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "TurnRateSlowDuration",
          "Name": "Slowed Turn Rate Duration",
          "Type": "duration",
          "Value": 2.0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "operative_blindside_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "BackstabBonusDamagePct",
          "Name": "Blindside Damage",
          "Value": 40
        }
      ]
    }
  },
  "Key": "operative_blindside",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Blindside",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 100
    },
    "MaxCameraAngleForSeeing": {
      "Name": null,
      "Value": 180
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "TurnRateSlowDuration": 1
    },
    {
      "AbilityCooldown": -12
    },
    {
      "BackstabBonusDamagePct": 30
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "blindside",
      "name": "Blindside",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_shiv_defer_damage" title="Bloodletting" -->

## Bloodletting

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_defer_damage`
- Snapshot ID: `39885`
- Source-Dokument: `7071`
- Kurzinfo: Bloodletting aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bloodletting`
- Payload Hash: `59ee33ce0a960674b747834ef3e8547f823e498c05c0d9f55ca5a3081f2cd74a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.393677+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_shiv_defer_damage_desc",
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_defer_damage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePctDeferred",
          "Name": "Incoming Damage Deferred",
          "Type": "damage",
          "Value": 25
        },
        {
          "Key": "DeferredDamageDuration",
          "Name": "Deferred Damage Duration",
          "Type": "duration",
          "Value": 6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "DeferClearPct",
          "Name": "Deferred Damage Cleared",
          "Title": "On Activate",
          "Type": "healing",
          "Value": 35
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_defer_damage_max_rage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePctDeferredMaxRage",
          "Name": "Incoming Damage Deferred",
          "Type": "damage",
          "Value": 15
        }
      ]
    }
  },
  "Key": "citadel_ability_shiv_defer_damage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bloodletting",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "DeferClearPct": 35
    },
    {
      "DamagePctDeferred": 15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "bloodletting",
      "name": "Bloodletting",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_drifter_hunger" title="Bloodscent" -->

## Bloodscent

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_drifter_hunger`
- Snapshot ID: `39781`
- Source-Dokument: `7071`
- Kurzinfo: Bloodscent aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bloodscent`
- Payload Hash: `fcbce6ff00335e4a3889ae088bb7bce3a433c0e23fb08e20f9d18d017abaa142`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.192545+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 80
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_drifter_hunger_desc",
  "Duration": {
    "InvisDuration": {
      "Name": "Invisibility Duration",
      "Scale": {
        "Type": "spirit",
        "Value": 0.0
      },
      "Type": "duration",
      "Value": 0
    },
    "KillDuration": {
      "Name": "Kill Duration",
      "Type": "duration",
      "Value": 300
    },
    "TrailDuration": {
      "Name": "Trail Duration",
      "Type": "duration",
      "Value": 10
    }
  },
  "Health": {
    "LowHealthThreshold": {
      "Name": "Health Threshold",
      "Type": "health",
      "Value": 30
    }
  },
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMoveSpeed",
        "Name": "Move Speed",
        "Type": "move_speed",
        "Value": 0
      }
    ],
    "DescKey": "ability_drifter_hunger_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "AmpDamagePercent",
          "Name": "Amplified Damage",
          "Type": "damage",
          "Value": 15
        },
        {
          "Key": "IsolationRange",
          "Name": "Isolation Range",
          "Type": "distance",
          "Value": 20
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "WeaponDmgPerIsolationKill",
          "Name": "Permanent Weapon Damage",
          "Title": "On Isolated Hero Death:",
          "Type": "bullet_damage",
          "Value": 3
        },
        {
          "Key": "HealOnKillPct",
          "Name": "Missing Health as Healing",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Isolated Hero Death:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_drifter_hunger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bloodscent",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DelayBeforeInvisStarts": {
      "Name": null,
      "Value": 0.6
    },
    "InvisFadeToDuration": {
      "Name": "Fade Time",
      "Value": 0.3
    },
    "IsolationAssistPercentValue": {
      "Name": null,
      "Value": 100
    },
    "MaxTrailTargets": {
      "Name": "Max Trail Targets",
      "Value": 2
    },
    "RevealOnDamageDuration": {
      "Name": null,
      "Value": 0.25
    },
    "RevealOnSpottedDuration": {
      "Name": null,
      "Value": 1.5
    },
    "SpottedRadius": {
      "Name": "Spot Radius",
      "Value": 15
    },
    "TargetLingerDuration": {
      "Name": null,
      "Value": 3
    },
    "TickRate": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3,
      "DescKey": "ability_drifter_hunger_t1_desc"
    },
    {
      "DescKey": "ability_drifter_hunger_t2_desc",
      "HealOnKillPct": 24,
      "StaminaToRestore": 2
    },
    {
      "AmpDamagePercent": 12.0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "bloodscent",
      "name": "Bloodscent",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bookworm_dragonfire" title="Bookwyrm" -->

## Bookwyrm

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_dragonfire`
- Snapshot ID: `39767`
- Source-Dokument: `7071`
- Kurzinfo: Bookwyrm aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bookwyrm`
- Payload Hash: `bc15016afacddd07ea60420a6a23487d7da9fc2c847739a26fa20397a0f26f42`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.164985+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_bookworm_dragonfire_desc",
  "Duration": {
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Type": "duration",
      "Value": 1.5
    }
  },
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [
      {
        "Key": "DragonTravelRange",
        "Name": "Dragon Travel Range",
        "Type": "distance",
        "Value": 20
      }
    ],
    "DescKey": "ability_bookworm_dragonfire_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "GroundFlameDuration",
          "Name": "Trail Duration",
          "Type": "duration",
          "Value": 3.0
        }
      ]
    }
  },
  "Key": "ability_bookworm_dragonfire",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bookwyrm",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 0.1
    },
    "DragonConeRange": {
      "Name": null,
      "Value": 5
    },
    "DragonRangePerSecond": {
      "Name": null,
      "Value": 500
    },
    "DragonSearchRadius": {
      "Name": null,
      "Value": 8.5
    },
    "DragonSearchTickRate": {
      "Name": null,
      "Value": 0.1
    },
    "DragonUpwardSpeed": {
      "Name": null,
      "Value": 400
    },
    "GroundAuraSpacing": {
      "Name": null,
      "Value": 1
    },
    "StartupDelay": {
      "Name": null,
      "Value": 0.3
    },
    "TickRate": {
      "Name": null,
      "Value": 0.3
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -12,
      "DescKey": "ability_bookworm_dragonfire_t1_desc"
    },
    {
      "AbilityCharges": 1,
      "DescKey": "ability_bookworm_dragonfire_t2_desc",
      "GroundFlameDuration": 2,
      "Radius": 1
    },
    {
      "DPS": 30.0,
      "Damage": 100,
      "DescKey": "ability_bookworm_dragonfire_t3_desc",
      "DragonTravelRange": 12
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "bookwyrm",
      "name": "Bookwyrm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_kickflip" title="Boot Kick" -->

## Boot Kick

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_kickflip`
- Snapshot ID: `39936`
- Source-Dokument: `7071`
- Kurzinfo: Boot Kick aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Boot Kick`
- Payload Hash: `a758ea2258f477835d8d5ee67d87f4b4a28f6ba6c03817aa4c02b80dabd9c2fb`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.492725+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10.6
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 21
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_werewolf",
  "HeroName": "Silver",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_kickflip_desc_1",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.9
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "DisarmDuration",
          "Name": "Disarm Duration",
          "StatusEffect": "Disarmed",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MarkDuration",
        "Name": "Mark Duration",
        "Scale": {
          "Type": "duration",
          "Value": 1.0
        },
        "Type": "duration",
        "Value": 3
      }
    ],
    "DescKey": "ability_werewolf_kickflip_desc_2",
    "Main": {
      "Props": [
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.0
          },
          "Type": "tech_damage",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_werewolf_kickflip",
  "Name": "Boot Kick",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.25
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.8
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "EnemyPushForceAway": {
      "Name": null,
      "Value": 300
    },
    "EnemyPushForceUp": {
      "Name": null,
      "Value": 300
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 20
    },
    "LeapForwardOffset": {
      "Name": null,
      "Value": 2.5
    },
    "SelfPushForceCameraAway": {
      "Name": null,
      "Value": 600
    },
    "SelfPushForceUp": {
      "Name": null,
      "Value": 200
    },
    "SlowDuration": {
      "Name": "Slow Duration",
      "Value": 0.1
    },
    "SuccessInputWindow": {
      "Name": null,
      "Value": 0.3
    },
    "TimeScaleDebuff": {
      "Name": null,
      "Value": 95
    }
  },
  "Range": {
    "LeapRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 1.7
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "DescKey": "ability_werewolf_kickflip_t2_desc",
      "StaminaRestore": 2
    },
    {
      "BonusDamage": 80,
      "DebuffDuration": 5,
      "DescKey": "ability_werewolf_kickflip_t3_desc",
      "OutgoingDamagePercent": -35
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "boot kick",
      "name": "Boot Kick",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_necro_gravestone" title="Borrowed Decree" -->

## Borrowed Decree

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_gravestone`
- Snapshot ID: `39862`
- Source-Dokument: `7071`
- Kurzinfo: Borrowed Decree aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Borrowed Decree`
- Payload Hash: `8fadc6c4628aade2933b04872ebd3544b7d8b71d42f6dfb2b2608cd3a2193982`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.349797+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.66
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 140
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Type": "duration",
    "Value": 16
  },
  "Damage": {
    "BonusSpiritDamagePercentage": {
      "Name": "Bonus Spirit Damage",
      "Type": "tech_damage",
      "Value": 15
    },
    "TechPower": {
      "Name": "Spirit Power",
      "Scale": {
        "Type": "power_increase",
        "Value": 1.0
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "Health": {
    "BonusHealthRegen": {
      "Name": "Health Regen",
      "Scale": {
        "Type": "spirit",
        "Value": 0.0
      },
      "Type": "healing",
      "Value": 0
    },
    "GravestoneHealth": {
      "Name": "Gravestone Health",
      "Type": "health",
      "Value": 100
    }
  },
  "HeroKey": "hero_necro",
  "HeroName": "Graves",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_necro_gravestone_desc_1",
    "Main": {
      "Props": []
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "SummonHealth",
        "Name": "Summon Health",
        "Scale": {
          "Type": "power_increase",
          "Value": 8.0
        },
        "Type": "health",
        "Value": 180
      },
      {
        "Key": "SummonMeleeDamage",
        "Name": "Melee Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Type": "bullet_damage",
        "Value": 40
      },
      {
        "Key": "SummonLifetime",
        "Name": "Summon Lifetime",
        "Type": "duration",
        "Value": 20
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 1.25
      },
      {
        "Key": "ExplosionRadius",
        "Name": "Explosion Radius",
        "Value": 6.5
      },
      {
        "Key": "SummonFrequency",
        "Name": "Time to Summon",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_necro_gravestone_desc_2",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.27
          },
          "Type": "tech_damage",
          "Value": 115
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 80
        }
      ]
    }
  },
  "Key": "ability_necro_gravestone",
  "Name": "Borrowed Decree",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlockerScaleFactor": {
      "Name": null,
      "Value": 1
    },
    "BuffDuration": {
      "Name": "Buff Duration",
      "Value": -1
    },
    "BulletResist": {
      "Name": "Bullet Resist",
      "Value": 12.5
    },
    "DamageSlowDuration": {
      "Name": null,
      "Value": 0.5
    },
    "DamageSlowPercent": {
      "Name": null,
      "Value": 20
    },
    "DecayDuration": {
      "Name": null,
      "Value": 1
    },
    "DecayTickRate": {
      "Name": null,
      "Value": 0.1
    },
    "ExplodeDelay": {
      "Name": "Explosion Delay",
      "Value": 0.23
    },
    "GravestoneTakesDamage": {
      "Name": null,
      "Value": 1
    },
    "GrowTime": {
      "Name": null,
      "Value": 0.1
    },
    "KnockupRadius": {
      "Name": null,
      "Value": 4
    },
    "KnockupSideRatio": {
      "Name": null,
      "Value": 1
    },
    "KnockupSpeed": {
      "Name": null,
      "Value": 240
    },
    "MaxGravestones": {
      "Name": "Max Gravestones",
      "Value": 3
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 40
    },
    "PushForce": {
      "Name": null,
      "Value": 300
    },
    "ReplicateZombieCast": {
      "Name": null,
      "Value": 1
    },
    "SlowPercentPerStack": {
      "Name": null,
      "Value": 0.5
    },
    "SpawnDuration": {
      "Name": null,
      "Value": 1.5
    },
    "StackDuration": {
      "Name": "Stack Duration",
      "Value": 5
    },
    "StackingDebuffTickRate": {
      "Name": null,
      "Value": 0.25
    },
    "SummonBurstCount": {
      "Name": null,
      "Value": 2
    },
    "SummonBurstFrequency": {
      "Name": null,
      "Value": 0.1
    },
    "SummonInitialDelay": {
      "Name": null,
      "Value": 0.3
    },
    "SummonMaxCount": {
      "Name": null,
      "Value": 32
    },
    "SummonSearchRadius": {
      "Name": null,
      "Value": 4
    },
    "TechArmorDamageReductionPerStack": {
      "Name": "Spirit Resistance per Stack",
      "Value": -0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.4
    }
  },
  "Range": {
    "AuraRadius": {
      "Name": "Aura Radius",
      "Type": "distance",
      "Value": 8
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "AbilityDuration": 10,
      "DescKey": "ability_necro_gravestone_t2_desc",
      "MoveSpeedPercent": 25
    },
    {
      "CurrentHealthDamagePercentage": 5,
      "DescKey": "ability_necro_gravestone_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "borrowed decree",
      "name": "Borrowed Decree",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_trapper_poisonjar" title="Bottled Phantasmicide" -->

## Bottled Phantasmicide

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_poisonjar`
- Snapshot ID: `39907`
- Source-Dokument: `7071`
- Kurzinfo: Bottled Phantasmicide aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bottled Phantasmicide`
- Payload Hash: `3d505bcc78707cdbcc76a2bd25e1819b2054f566d4b4d726cf1f6dfeade43a24`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.435053+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_trapper_poisonjar_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "InitialRadius",
        "Name": "Initial Radius",
        "Type": "distance",
        "Value": 6
      },
      {
        "Key": "RadiusPerSecond",
        "Name": "Radius Growth",
        "Type": "distance",
        "Value": 0.25
      }
    ],
    "DescKey": "ability_trapper_poisonjar_desc",
    "Main": {
      "Props": [
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Scale": {
            "Type": "spirit",
            "Value": 0.279
          },
          "Type": "slow",
          "Value": 25
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 8
        }
      ]
    }
  },
  "Key": "ability_trapper_poisonjar",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bottled Phantasmicide",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Height": {
      "Name": null,
      "Value": 2
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "SlowPercent": 20
    },
    {
      "AbilityDuration": 4
    },
    {
      "DescKey": "ability_trapper_poisonjar_t3_desc",
      "TechArmorDamageReduction": -25
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "bottled phantasmicide",
      "name": "Bottled Phantasmicide",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bounce_pad" title="Bounce Pad" -->

## Bounce Pad

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bounce_pad`
- Snapshot ID: `39749`
- Source-Dokument: `7071`
- Kurzinfo: Bounce Pad aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bounce Pad`
- Payload Hash: `2cf0328941b17be353ba7e64fa60797585209708760309b55578fb2d75fb4ff3`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.127450+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.08
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 41
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3.5
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 22
  },
  "DescKey": "ability_bounce_pad_desc",
  "Duration": {
    "MinAirTimeForStomp": {
      "Name": "Min Air Time for Stomp",
      "Type": "duration",
      "Value": 0.2
    }
  },
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [
      {
        "Key": "AirControlPercent",
        "Name": "Air Control",
        "Type": "move_speed",
        "Value": 100
      },
      {
        "Key": "SpeedOnLandDuration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_bounce_pad_desc",
    "Main": {
      "Props": [
        {
          "Key": "StompDamage",
          "Name": "Stomp Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.372
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "Radius",
          "Name": "Radius",
          "Type": "distance",
          "Value": 9
        }
      ]
    }
  },
  "Key": "ability_bounce_pad",
  "Move": {
    "AirControlAccelPercent": {
      "Name": "Air Acceleration",
      "Type": "move_speed",
      "Value": 50
    },
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Bounce Pad",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BarrelBounceVelocity": {
      "Name": null,
      "Value": 800
    },
    "BarrelUpFactor": {
      "Name": null,
      "Value": 1
    },
    "BounceVelocity": {
      "Name": null,
      "Value": 750
    },
    "PlaceDistance": {
      "Name": null,
      "Value": 200
    },
    "Scale": {
      "Name": null,
      "Value": 1
    },
    "TossSpeed": {
      "Name": null,
      "Value": 12.7
    },
    "UpFactor": {
      "Name": null,
      "Value": 1.2
    },
    "VerticalDifferenceTolerance": {
      "Name": null,
      "Value": 60
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 9
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_bounce_pad_t1_desc"
    },
    {
      "DescKey": "ability_bounce_pad_t2_desc",
      "SpeedOnLand": 4,
      "SpeedOnLandDuration": 4
    },
    {
      "DescKey": "ability_bounce_pad_t3_desc",
      "StompStunDuration": 0.7
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "bounce pad",
      "name": "Bounce Pad",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="fathom_breach" title="Breach" -->

## Breach

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_breach`
- Snapshot ID: `39892`
- Source-Dokument: `7071`
- Kurzinfo: Breach aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Breach`
- Payload Hash: `b0a290c1e1ba1fbbd40c9a60437ae4af66c0c06884c074c4d4721362a7c07c01`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.406570+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 22.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "fathom_breach_desc",
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_breach_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Key": "fathom_breach",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Breach",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GravityScale": {
      "Name": "Gravity Scale",
      "Value": 1.4
    },
    "TossSpeed": {
      "Name": null,
      "Value": 350
    },
    "WallImpactLookAheadDistance": {
      "Name": null,
      "Value": 100
    }
  },
  "Range": {
    "ExplosionRadius": {
      "Name": "Explosion Radius",
      "Type": "distance",
      "Value": 6
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "ExplosionRadius": 3
    },
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 120
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "breach",
      "name": "Breach",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bullet_flurry" title="Bullet Dance" -->

## Bullet Dance

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bullet_flurry`
- Snapshot ID: `39826`
- Source-Dokument: `7071`
- Kurzinfo: Bullet Dance aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Dance`
- Payload Hash: `df50e1d97557798befc28964c734aca7d87d187e66ffa0b8e46cd15c127b46c4`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.278435+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.4
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 165.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Type": "duration",
    "Value": 3.5
  },
  "DescKey": "ability_bullet_flurry_desc",
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [
      {
        "Key": "EvasionPercent",
        "Name": "Bullet Evasion",
        "Value": 30
      },
      {
        "Key": "ChannelMoveSpeed",
        "Name": "Channel Move Speed",
        "Type": "move_speed",
        "Value": 4
      }
    ],
    "DescKey": "ability_bullet_flurry_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 25
        },
        {
          "Key": "WeaponDamageBonus",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": 7
        },
        {
          "Key": "TargetsPerTick",
          "Name": "Targets Hit Per Shot",
          "Type": "radius",
          "Value": 1
        }
      ]
    }
  },
  "Key": "ability_bullet_flurry",
  "Name": "Bullet Dance",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "OverrideBulletRadius": {
      "Name": null,
      "Value": 10
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 16
  },
  "Range": {
    "RadiusMin": {
      "Name": null,
      "Type": "distance",
      "Value": 0.75
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "BonusFireRate": 10,
      "ChannelMoveSpeed": 3,
      "DescKey": "ability_bullet_flurry_t2_desc"
    },
    {
      "AbilityCooldown": -65,
      "DescKey": "ability_bullet_flurry_t3_desc",
      "EvasionPercent": 40.0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "bullet dance",
      "name": "Bullet Dance",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_burrow" title="Burrow" -->

## Burrow

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_burrow`
- Snapshot ID: `39840`
- Source-Dokument: `7071`
- Kurzinfo: Burrow aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Burrow`
- Payload Hash: `8a4daf837a851189a7ae673f913643c1ca95a09bb60d2f8f53f504ad941b1444`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.305770+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_burrow_desc",
  "Duration": {
    "SpeedLostDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 1
    },
    "SpinSlowDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.3
    }
  },
  "HeroKey": "hero_krill",
  "HeroName": "Mo & Krill",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 60
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 30
      }
    ],
    "DescKey": "ability_burrow_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 5
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 1.488
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "SpinDuration",
          "Name": "Spin Duration",
          "Type": "duration",
          "Value": 1.5
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 1
        }
      ]
    }
  },
  "Key": "ability_burrow",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Burrow",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EnemyDamageSpeedPenalty": {
      "Name": null,
      "Value": 0.5
    },
    "SpinSlowPercent": {
      "Name": null,
      "Value": 10
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    },
    "UpForce": {
      "Name": null,
      "Value": 250
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DPS": 50,
      "DescKey": "ability_burrow_t1_desc"
    },
    {
      "AbilityChannelTime": 4,
      "DescKey": "ability_burrow_t2_desc",
      "Radius": 2
    },
    {
      "AbilityCooldown": -20.0,
      "BonusMoveSpeed": 4,
      "DescKey": "ability_burrow_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "burrow",
      "name": "Burrow",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_doorman_bomb" title="Call Bell" -->

## Call Bell

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_bomb`
- Snapshot ID: `39775`
- Source-Dokument: `7071`
- Kurzinfo: Call Bell aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Call Bell`
- Payload Hash: `43fccb2eb7488ed27b938722d0e6b54c7125079eb857ccb6feb73d26ebd67969`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.180549+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 6
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "ability_doorman_bomb_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7
          },
          "Title": "On Impact",
          "Type": "tech_damage",
          "Value": 40
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProjectileFuse",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "AbilityDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "ExplosionDamage",
          "Name": "Explosion Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.2
          },
          "Title": "Explosion",
          "Type": "tech_damage",
          "Value": 55
        },
        {
          "Key": "DebuffAccuracy",
          "Name": "Weapon Accuracy",
          "Title": "Explosion",
          "Type": "fire_rate",
          "Value": -40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "Explosion",
          "Type": "slow",
          "Value": 35
        }
      ]
    }
  },
  "Key": "ability_doorman_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Call Bell",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.5
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AccuracyDebuffFalloffBias": {
      "Name": null,
      "Value": 0.3
    },
    "EnableAura": {
      "Name": null,
      "Value": 1
    },
    "ProjectileDrag": {
      "Name": null,
      "Value": 0.975
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "DescKey": "ability_doorman_bomb_t1_desc"
    },
    {
      "DescKey": "ability_doorman_bomb_t2_desc",
      "ExplosionDamage": 40,
      "ImpactDamage": 30
    },
    {
      "DescKey": "ability_doorman_bomb_t3_desc",
      "ExplosionDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.35
        },
        "Value": 0
      },
      "ImpactDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.35
        },
        "Value": 0
      },
      "ProjectileFuse": 26,
      "Radius": 4.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "call bell",
      "name": "Call Bell",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bookworm_aoemagic" title="Captivating Read" -->

## Captivating Read

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_aoemagic`
- Snapshot ID: `39769`
- Source-Dokument: `7071`
- Kurzinfo: Captivating Read aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Captivating Read`
- Payload Hash: `d5f2a542ea7f8b1b6ca7e257a0a23cdc9d39d615d72300e040449166fb083ad2`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.169264+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_bookworm_aoemagic_desc",
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 45
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0.5
      }
    ],
    "DescKey": "ability_bookworm_aoemagic_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "StatusEffect": "Immobilize",
          "Title": "On Hit:",
          "Value": 1.0
        },
        {
          "Key": "TechArmorDamageReduction",
          "Name": "Spirit Resist",
          "Title": "On Hit:",
          "Type": "tech_armor_down",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_bookworm_aoemagic",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Captivating Read",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DetonationDelay": {
      "Name": null,
      "Value": 1.25
    },
    "Height": {
      "Name": null,
      "Value": 8
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 7.5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1
    },
    {
      "DebuffDuration": 6,
      "DescKey": "ability_bookworm_aoemagic_t3_desc",
      "Radius": 1,
      "TechArmorDamageReduction": -18
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "captivating read",
      "name": "Captivating Read",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_card_toss" title="Card Trick" -->

## Card Trick

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_card_toss`
- Snapshot ID: `39943`
- Source-Dokument: `7071`
- Kurzinfo: Card Trick aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Card Trick`
- Payload Hash: `88f0f6fff0355ee83e6bb38a7468f6cdcf3c1dac289e4d0ebf7fc3576a96271d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.505156+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Type": "range",
    "Value": 500
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Scale": {
      "Type": "cooldown",
      "Value": 0.0
    },
    "Type": "cooldown",
    "Value": 0.6
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_card_toss_desc",
  "Duration": {
    "ClubSlowDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 3
    }
  },
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_card_toss_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.55
          },
          "Type": "tech_damage",
          "Value": 45
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "ability_header_card_toss_suits_desc",
    "Main": {
      "Props": [
        {
          "Key": "SpadeDamageBonus",
          "Name": "Bonus Damage",
          "Title": "Spade",
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "DiamondResistShred",
          "Name": "Bullet and Spirit Resist Reduction",
          "Title": "Diamond",
          "Type": "bullet_armor_down",
          "Value": -8.0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "ClubSlowPercent",
          "Name": "Movement Slow",
          "Title": "Club",
          "Type": "slow",
          "Value": -30
        },
        {
          "Key": "HeartHeal",
          "Name": "Heal",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Title": "Heart",
          "Type": "healing",
          "Value": 75
        }
      ]
    }
  },
  "Key": "citadel_ability_card_toss",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Card Trick",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusAbilityResource": {
      "Name": null,
      "Value": 100
    },
    "CardResourceGenPctScale": {
      "Name": "Card Summon Rate",
      "Scale": {
        "Type": "cooldown",
        "Value": -1.0
      },
      "Value": 85
    },
    "CardResourcePerBulletCrit": {
      "Name": null,
      "Value": 6
    },
    "CardResourcePerBulletHit": {
      "Name": null,
      "Value": 4
    },
    "CardResourcePerHeavyMelee": {
      "Name": null,
      "Value": 25
    },
    "CardResourcePerLightMelee": {
      "Name": null,
      "Value": 10
    },
    "CooldownBetweenCards": {
      "Name": null,
      "Value": 0.5
    },
    "DiamondResistShredDuration": {
      "Name": null,
      "Value": 5
    },
    "HeartHealNonHeroRatio": {
      "Name": null,
      "Value": 0.5
    },
    "NonPlayerCardResourceScale": {
      "Name": null,
      "Value": 0.35
    },
    "ProjectileOriginHeightOffset": {
      "Name": null,
      "Value": 50
    },
    "ResourcePerCard": {
      "Name": null,
      "Value": 100
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Range": {
    "JokerExtraCardSearchRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 20
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
      },
      "DescKey": "citadel_ability_card_toss_t2_desc"
    },
    {
      "ClubSlowPercent": -20,
      "DescKey": "citadel_ability_card_toss_t3_desc",
      "DiamondResistShred": -5,
      "HeartHeal": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 75
      },
      "ImprovedJokerChance": 1,
      "SpadeDamageBonus": 40
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "card trick",
      "name": "Card Trick",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_punkgoat_tether" title="Chain Gang" -->

## Chain Gang

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_tether`
- Snapshot ID: `39878`
- Source-Dokument: `7071`
- Kurzinfo: Chain Gang aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Chain Gang`
- Payload Hash: `07e2895f705d870ba0b5a27a83470b0c30ecaa00b6eab7739e1dc1935ebb7ac6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.380763+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 12.0
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 175
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.8
  },
  "Damage": {
    "DPS": {
      "Name": "Damage Per Second",
      "Scale": {
        "Type": "spirit",
        "Value": 0.9
      },
      "Type": "tech_damage",
      "Value": 45
    },
    "DamageIncreasePct": {
      "Name": null,
      "Scale": {
        "Type": "spirit",
        "Value": 0.0
      },
      "Type": "damage",
      "Value": 0
    }
  },
  "DescKey": "ability_punkgoat_tether_desc",
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [
      {
        "Key": "MoveSpeedSlowMaxPct",
        "Type": "move_speed",
        "Value": 35
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 0
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "bullet_damage",
        "Value": 0
      },
      {
        "Key": "FireRateSlowDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_punkgoat_tether_desc",
    "Main": {
      "Props": [
        {
          "Key": "UnstoppablePerHero",
          "Name": "Per Hero",
          "StatusEffect": "Unstoppable",
          "Type": "duration",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 120
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 2.8
        }
      ]
    }
  },
  "Key": "ability_punkgoat_tether",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveSpeedSlowMinPct": {
      "Name": null,
      "Type": "move_speed",
      "Value": 25
    }
  },
  "Name": "Chain Gang",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "PullDistance": {
      "Name": null,
      "Value": 4
    },
    "PullDuration": {
      "Name": null,
      "Value": 0.8
    },
    "PullForceMax": {
      "Name": null,
      "Value": 2000
    },
    "PullTrackCasterDuration": {
      "Name": null,
      "Value": 0.5
    },
    "RopeSnapNoLOSDuration": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Range": {
    "RopeLength": {
      "Name": null,
      "Type": "distance",
      "Value": 2.0
    },
    "RopeSnapDistance": {
      "Name": null,
      "Type": "distance",
      "Value": 45.0
    },
    "RopeSoftEdgeLength": {
      "Name": null,
      "Type": "distance",
      "Value": 4.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BulletResist": 40,
      "DescKey": "ability_punkgoat_tether_t1_desc",
      "TechResist": 40
    },
    {
      "AbilityCooldown": -40
    },
    {
      "AbilityCastRange": 5,
      "DescKey": "ability_punkgoat_tether_t3_desc",
      "UnstoppablePerHero": 1.3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "chain gang",
      "name": "Chain Gang",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_charged_shot" title="Charged Shot" -->

## Charged Shot

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_charged_shot`
- Snapshot ID: `39867`
- Source-Dokument: `7071`
- Kurzinfo: Charged Shot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Charged Shot`
- Payload Hash: `e06a29f6c28b00e339b7834f842af6d6353f35ae4af9990ab6a601b88f8f2b99`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.359350+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 17.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 4
  },
  "DescKey": "ability_charged_shot_desc",
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_charged_shot_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Key": "ability_charged_shot",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.5
    }
  },
  "Name": "Charged Shot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 4.1
    },
    "CameraHeightOffset": {
      "Name": null,
      "Value": 20
    },
    "CameraHorizontalOffset": {
      "Name": null,
      "Value": 15
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1.524
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "Damage": 54.0,
      "DescKey": "ability_charged_shot_t2_desc"
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "ability_charged_shot_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "charged shot",
      "name": "Charged Shot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_ult_combo" title="Combo" -->

## Combo

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_ult_combo`
- Snapshot ID: `39842`
- Source-Dokument: `7071`
- Kurzinfo: Combo aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Combo`
- Payload Hash: `97edb00806bf74ccdd6a8f7e92a76828d7f85d020a339db6c16253d072db1dc6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.309914+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 4.0
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 2.4
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_ult_combo_desc",
  "HeroKey": "hero_krill",
  "HeroName": "Mo & Krill",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_ult_combo_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "LifeStealPercentOnHit",
          "Name": "Unknown(LifeStealPercentOnHit)",
          "Type": "health",
          "Value": 0
        },
        {
          "Key": "BonusHealthOnKill",
          "Name": "Bonus Max Health Per Kill",
          "Scale": {
            "Type": "power_increase",
            "Value": 2
          },
          "Type": "health",
          "Value": 40
        }
      ]
    }
  },
  "Key": "ability_ult_combo",
  "Name": "Combo",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DescKey": "ability_ult_combo_t1_desc",
      "LifeStealPercentOnHit": 100
    },
    {
      "AbilityCooldown": -30,
      "BulletResist": 50,
      "DescKey": "ability_ult_combo_t2_desc"
    },
    {
      "AbilityChannelTime": 0.7,
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 40
      },
      "DescKey": "ability_ult_combo_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "combo",
      "name": "Combo",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fire_bomb" title="Concussive Combustion" -->

## Concussive Combustion

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fire_bomb`
- Snapshot ID: `39834`
- Source-Dokument: `7071`
- Kurzinfo: Concussive Combustion aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Concussive Combustion`
- Payload Hash: `ff64ff7c0c1d3b3425c7a1d3606d4182a527af30c6d82cffdc4a39c9b683f8de`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.292466+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 190.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_fire_bomb_desc",
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 12
      }
    ],
    "DescKey": "ability_fire_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "ExplodeDelay",
          "Name": "Explosion Delay",
          "Type": "duration",
          "Value": 3.25
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 125
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 1.25
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "FireRatePerHero",
          "Name": "Unknown(FireRatePerHero)",
          "Title": "On Hero Hit:",
          "Type": "fire_rate",
          "Value": 0
        },
        {
          "Key": "BuffDuration",
          "Name": "Buff Duration",
          "Title": "On Hero Hit:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fire_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Concussive Combustion",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -65.0,
      "DescKey": "ability_fire_bomb_t2_desc",
      "LifeStealPercentOnHit": 100
    },
    {
      "DescKey": "ability_fire_bomb_t3_desc",
      "Radius": 10,
      "StunDuration": 0.9
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "concussive combustion",
      "name": "Concussive Combustion",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_priest_flashbang" title="Consecrating Grenade" -->

## Consecrating Grenade

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_flashbang`
- Snapshot ID: `39871`
- Source-Dokument: `7071`
- Kurzinfo: Consecrating Grenade aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Consecrating Grenade`
- Payload Hash: `b566d0642d8867294682a5d73e406ebe27853274e75a028d4b86e4b5acd98c48`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.366730+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.03
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25
  },
  "Damage": {
    "HealAmpRegenPenaltyPercent": {
      "Name": "Healing Reduction",
      "Type": "damage",
      "Value": -30
    }
  },
  "DescKey": "ability_priest_flashbang_desc",
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "BurnDuration",
        "Name": "Burn Duration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "BurnRadius",
        "Name": "Burn Radius",
        "Type": "distance",
        "Value": 4.5
      }
    ],
    "DescKey": "ability_priest_flashbang_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 1.0
          },
          "Type": "bullet_damage",
          "Value": 35
        },
        {
          "Key": "HealAmpReceivePenaltyPercent",
          "Name": "Healing Reduction",
          "Type": "damage",
          "Value": -30
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "power_increase",
            "Value": 1.6
          },
          "Title": "On Hit:",
          "Type": "damage",
          "Value": 10
        },
        {
          "Key": "DPSPercentHealth",
          "Name": "Max Health as Damage",
          "Title": "On Hit:",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Value": 0
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Title": "On Hit:",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_priest_flashbang",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Consecrating Grenade",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.15
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BounceGrenadeSpeed": {
      "Name": null,
      "Value": 1100
    },
    "BounceLifetime": {
      "Name": null,
      "Value": 0.5
    },
    "BurnLingerDuration": {
      "Name": null,
      "Value": 0.15
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 15
    },
    "PreBounceLifetime": {
      "Name": null,
      "Value": 15
    },
    "TickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "BurnDuration": 1.0,
      "BurnRadius": 1.5,
      "DescKey": "ability_priest_flashbang_t2_desc",
      "Radius": 1.5
    },
    {
      "AbilityCharges": 1,
      "AbilityCooldownBetweenCharge": 3,
      "DescKey": "ability_priest_flashbang_t3_desc",
      "HealAmpReceivePenaltyPercent": -20,
      "HealAmpRegenPenaltyPercent": -20
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "consecrating grenade",
      "name": "Consecrating Grenade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_wrecker_salvage" title="Consume" -->

## Consume

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_salvage`
- Snapshot ID: `39948`
- Source-Dokument: `7071`
- Kurzinfo: Consume aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Consume`
- Payload Hash: `8988cd08724d5afd855e541a02d6234509dc1bc1398dfcd12c8c29100a6b0e5b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.514757+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12.5
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_wrecker_salvage_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "SalvageDuration",
        "Name": "Max Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "MaxRange",
        "Name": "Max Tether Range",
        "Type": "distance",
        "Value": 20
      }
    ],
    "DescKey": "ability_wrecker_salvage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "SalvageBonus_FireRate",
          "Name": "Fire Rate per bonus",
          "Type": "bullet_damage",
          "Value": 0
        },
        {
          "Key": "ConsumeHealPercentage",
          "Name": "Consume Heal Percentage",
          "Type": "healing",
          "Value": 50
        }
      ]
    }
  },
  "Key": "ability_wrecker_salvage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 3.8
    }
  },
  "Name": "Consume",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickInterval": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "ConsumeHealPercentage": 25
    },
    {
      "DPS": 40
    },
    {
      "AbilityUnitTargetLimit": 2,
      "DescKey": "ability_wrecker_salvage_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "consume",
      "name": "Consume",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_crackshot" title="Crackshot" -->

## Crackshot

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_crackshot`
- Snapshot ID: `39750`
- Source-Dokument: `7071`
- Kurzinfo: Crackshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crackshot`
- Payload Hash: `aeee41f188da30c16a3b2e190651919510dd6180b81cfcb8f4d485050a9c295b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.129825+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_crackshot_desc",
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_crackshot_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.116
          },
          "Type": "bullet_damage",
          "Value": 55
        },
        {
          "Key": "FadingSlowPercent",
          "Name": "Fading Move Speed",
          "Type": "slow",
          "Value": 50
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Type": "duration",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_crackshot",
  "Name": "Crackshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CrackshotNPCCDReduction": {
      "Name": null,
      "Value": 50
    }
  },
  "Range": {
    "ExplosionRadius": {
      "Name": "Explosion Radius",
      "Type": "distance",
      "Value": 2
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_crackshot_t1_desc",
      "FadingSlowPercent": 25
    },
    {
      "Damage": 49.5
    },
    {
      "AbilityCooldownPerHeadshot": -6,
      "AbilityCooldownPerHeadshotNPC": -3,
      "DescKey": "ability_crackshot_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "crackshot",
      "name": "Crackshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_trapper_spiderwave" title="Crawling Plague" -->

## Crawling Plague

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spiderwave`
- Snapshot ID: `39910`
- Source-Dokument: `7071`
- Kurzinfo: Crawling Plague aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crawling Plague`
- Payload Hash: `d45de10000b86a32725eac856aa2836f6b52bb67dd2cf9575999a031f6b00c61`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.441627+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.6
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 160
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_trapper_spiderwave_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "SpiderLifetime",
        "Name": "Spider Life Time",
        "Value": 25
      },
      {
        "Key": "SpiderSearchRadius",
        "Name": "Spider Chase Distance",
        "Type": "distance",
        "Value": 2
      },
      {
        "Key": "SpiritStealDuration",
        "Name": "Spirit Debuff Duration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "SpiritReducedPerStack",
        "Name": "Spirit Reduction",
        "Type": "tech_damage",
        "Value": 5
      },
      {
        "Key": "SpiritResReducedPerStack",
        "Name": "Spirit Resist Reduction",
        "Type": "tech_armor_up",
        "Value": 5
      }
    ],
    "DescKey": "ability_trapper_spiderwave_desc",
    "Main": {
      "Props": [
        {
          "Key": "SpiderDamage",
          "Name": "Spider Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 140
        },
        {
          "Key": "SpiderCount",
          "Name": "Spiders Released",
          "Value": 5
        }
      ]
    }
  },
  "Key": "ability_trapper_spiderwave",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Crawling Plague",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SpiderArmingTime": {
      "Name": null,
      "Value": 0.5
    },
    "SpiderChaseVelocity": {
      "Name": null,
      "Value": 400
    },
    "SpiderClimbHeight": {
      "Name": null,
      "Value": 0.3
    },
    "SpiderDistAboveGround": {
      "Name": null,
      "Value": 0.1
    },
    "SpiderExplodeRadius": {
      "Name": null,
      "Value": 3
    },
    "SpiderFloatDownRate": {
      "Name": null,
      "Value": 8
    },
    "SpiderGravity": {
      "Name": null,
      "Value": 1
    },
    "SpiderRandomPositionRadius": {
      "Name": null,
      "Value": 4
    },
    "SpiderTickRate": {
      "Name": null,
      "Value": 0.3
    },
    "SpreadAngle": {
      "Name": null,
      "Value": 30
    },
    "SpreadDistance": {
      "Name": null,
      "Value": 900
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 3.5
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -45
    },
    {
      "DescKey": "ability_trapper_spiderwave_t2_desc",
      "SpiritReducedPerStack": 3,
      "SpiritResReducedPerStack": 3
    },
    {
      "DescKey": "ability_trapper_spiderwave_t3_desc",
      "SpiderCount": 5,
      "SpreadDistance": 900
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "crawling plague",
      "name": "Crawling Plague",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_healing_slash" title="Crimson Slash" -->

## Crimson Slash

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_healing_slash`
- Snapshot ID: `39953`
- Source-Dokument: `7071`
- Kurzinfo: Crimson Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crimson Slash`
- Payload Hash: `9abf96cb3b87752760d29c0294744ceb44995522caeaa936b93a7a616090968e`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.525050+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_healing_slash_desc",
  "Duration": {
    "BuffDuration": {
      "Name": "Buff Duration",
      "Scale": {
        "Type": "duration",
        "Value": 1.0
      },
      "Type": "duration",
      "Value": 0
    }
  },
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "citadel_ability_healing_slash_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.37
          },
          "Type": "tech_damage",
          "Value": 55
        },
        {
          "Key": "HealFixedHealth",
          "Name": "Heal on hero hit",
          "Scale": {
            "Type": "spirit",
            "Value": 1.035871
          },
          "Type": "healing",
          "Value": 55
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "bullet_damage",
          "Value": 30
        }
      ]
    }
  },
  "Key": "citadel_ability_healing_slash",
  "Name": "Crimson Slash",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 13
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BuffDuration": 4,
      "BuffMeleeDamage": 30,
      "DescKey": "citadel_ability_healing_slash_t1_desc"
    },
    {
      "DescKey": "citadel_ability_healing_slash_t2_desc",
      "HealMaxHealth": 6
    },
    {
      "AbilityCooldown": -10,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_healing_slash_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "crimson slash",
      "name": "Crimson Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_hornet_sting" title="Crow Familiar" -->

## Crow Familiar

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_sting`
- Snapshot ID: `39829`
- Source-Dokument: `7071`
- Kurzinfo: Crow Familiar aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crow Familiar`
- Payload Hash: `ea774a1ab9b94ff86a61f5b89ddc9b600981f5f7d16e8371ff70466b7bae8460`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.283665+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_hornet_sting_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Name": "Healing Reduction",
        "Value": 0
      },
      {
        "Key": "BulletResistReduction",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_down",
        "Value": -6
      },
      {
        "Key": "TechArmorDamageReduction",
        "Type": "tech_armor_down",
        "Value": -6
      }
    ],
    "DescKey": "citadel_ability_hornet_sting_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "DotHealthPercent",
          "Name": "Bleed Damage",
          "Type": "tech_damage",
          "Value": 2.2
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Type": "duration",
          "Value": 5
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_sting",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Crow Familiar",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 1.0
    },
    "VisualSplashRadius": {
      "Name": null,
      "Value": 4
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_hornet_sting_t1_desc",
      "HealAmpReceivePenaltyPercent": -35,
      "HealAmpRegenPenaltyPercent": -35
    },
    {
      "AbilityCooldown": -16.0,
      "DescKey": "citadel_ability_hornet_sting_t2_desc",
      "DotHealthPercent": 0.5
    },
    {
      "BulletResistReduction": -8,
      "DebuffDuration": 2,
      "DescKey": "citadel_ability_hornet_sting_t3_desc",
      "TechArmorDamageReduction": -8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "crow familiar",
      "name": "Crow Familiar",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_unicorn_prismaticguard" title="Dazzling Trick" -->

## Dazzling Trick

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_prismaticguard`
- Snapshot ID: `39912`
- Source-Dokument: `7071`
- Kurzinfo: Dazzling Trick aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dazzling Trick`
- Payload Hash: `400311946c6188dbc52a372e64d6e48ea0b9b7495a5689def619404663179f6b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.446110+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_unicorn_prismaticguard_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_unicorn_prismaticguard_desc",
    "Main": {
      "Props": [
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "bullet_armor_up",
          "Value": 100
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ExplodeRadius",
        "Name": "Explosion Radius",
        "Type": "distance",
        "Value": 14
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Title": "On Explode",
          "Type": "duration",
          "Value": 1.75
        },
        {
          "Key": "BarrierDamagePercentage",
          "Name": "Barrier Damage",
          "Title": "On Explode",
          "Type": "tech_damage",
          "Value": 50
        }
      ]
    }
  },
  "Key": "ability_unicorn_prismaticguard",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Dazzling Trick",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MaxLifetime": {
      "Name": "Lifetime",
      "Value": 4
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3.5
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Value": 80
      },
      "DescKey": "ability_unicorn_prismaticguard_t2_desc"
    },
    {
      "AbilityCooldown": -18,
      "DebuffDuration": 1.5,
      "DescKey": "ability_unicorn_prismaticguard_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "dazzling trick",
      "name": "Dazzling Trick",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_lash_ultimate" title="Death Slam" -->

## Death Slam

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_ultimate`
- Snapshot ID: `39846`
- Source-Dokument: `7071`
- Kurzinfo: Death Slam aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Death Slam`
- Payload Hash: `3e9b50e18a0eefe270858bb171a4d30644e1258dc4fd961a64db8978eb5c7eb1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.318581+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 20
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 2.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 170.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "TimeToGainLockonStack": {
      "Name": null,
      "Type": "cooldown",
      "Value": 0.7
    }
  },
  "DescKey": "citadel_ability_lash_ultimate_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 50
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "AbilityCastDelay",
        "Name": "Cast Delay",
        "Type": "cast",
        "Value": 0.3
      }
    ],
    "DescKey": "citadel_ability_lash_ultimate_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 105
        },
        {
          "Key": "ThrowDistance",
          "Name": "Max Throw Distance",
          "Scale": {
            "Type": "spirit",
            "Value": 0.14
          },
          "Value": 14
        }
      ]
    }
  },
  "Key": "citadel_ability_lash_ultimate",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Death Slam",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 6
    },
    "BoostTime": {
      "Name": null,
      "Value": 1.0
    },
    "HangTime": {
      "Name": null,
      "Value": 0.6
    },
    "ImpactRadius": {
      "Name": "Impact Radius",
      "Value": 6
    },
    "LiftHeight": {
      "Name": null,
      "Value": 6
    },
    "LosingLockGraceTime": {
      "Name": null,
      "Value": 0.4
    },
    "MaxLockonStacks": {
      "Name": null,
      "Value": 1
    },
    "NotInConeLosesLock": {
      "Name": null,
      "Value": 1
    },
    "SlamSpeed": {
      "Name": null,
      "Value": 1600
    },
    "ThrowStraightDuration": {
      "Name": null,
      "Value": 1.5
    },
    "TimeToLoseLockonStack": {
      "Name": null,
      "Value": 2
    },
    "UpBoostSpeed": {
      "Name": null,
      "Value": 400
    }
  },
  "Range": {
    "LockonConeAngle": {
      "Name": null,
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "ThrowDistance": 12
    },
    {
      "AbilityCooldown": -35
    },
    {
      "AbilityCastRange": 6,
      "DescKey": "citadel_ability_lash_ultimate_t3_desc",
      "StunDuration": 1.2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "death slam",
      "name": "Death Slam",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_gunslinger_salvo" title="Demontrigger Blitz" -->

## Demontrigger Blitz

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_salvo`
- Snapshot ID: `39890`
- Source-Dokument: `7071`
- Kurzinfo: Demontrigger Blitz aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Demontrigger Blitz`
- Payload Hash: `6c5c7afabf298c4039c0660caed4f8b0cbf4444860ca53f7dfbd6b1ba73d6bea`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.402686+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 60
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 90
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_gunslinger_salvo_desc",
  "HeroKey": "hero_skyrunner",
  "HeroName": "Skyrunner",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityCastDelay",
        "Name": "Cast Delay",
        "Type": "cast",
        "Value": 1
      },
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 1
      }
    ],
    "DescKey": "ability_gunslinger_salvo_desc",
    "Main": {
      "Props": [
        {
          "Key": "TotalShots",
          "Name": "Total Shots Fired",
          "Type": "bullet_damage",
          "Value": 4
        },
        {
          "Key": "ProcDamagePercentage",
          "Name": "Bonus Bullet Damage",
          "Type": "tech_damage",
          "Value": 220
        }
      ]
    }
  },
  "Key": "ability_gunslinger_salvo",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 3.8
    }
  },
  "Name": "Demontrigger Blitz",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Damage": {
      "Name": "Damage",
      "Value": null
    },
    "OverrideBulletRadius": {
      "Name": null,
      "Value": 0.3
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "DebuffDuration": 6
    },
    {
      "TotalShots": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "demontrigger blitz",
      "name": "Demontrigger Blitz",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fencer_throwblade" title="Disengaging Sigil" -->

## Disengaging Sigil

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_throwblade`
- Snapshot ID: `39791`
- Source-Dokument: `7071`
- Kurzinfo: Disengaging Sigil aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Disengaging Sigil`
- Payload Hash: `68d9dd101a4fe0b445e84f0e1ad108ffaf8e61377cfcf02689ec9784d59fbab3`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.211246+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_fencer_throwblade_desc",
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 4
      },
      {
        "Key": "SigilRadius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 6.5
      }
    ],
    "DescKey": "ability_fencer_throwblade_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 85
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 30
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fencer_throwblade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Disengaging Sigil",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 2.0
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "JumpVelocityHidden": {
      "Name": null,
      "Value": 16
    },
    "TraceToGroundDistance": {
      "Name": null,
      "Value": 1000
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BonusBulletSpeedPercent": 25,
      "BonusFireRate": 25,
      "BuffDuration": 8,
      "DescKey": "ability_fencer_throwblade_t1_desc"
    },
    {
      "DescKey": "ability_fencer_throwblade_t2_desc",
      "ResetsAirLimit": 1,
      "StaminaToRestore": 1
    },
    {
      "DescKey": "ability_fencer_throwblade_t3_desc",
      "RecastTime": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "disengaging sigil",
      "name": "Disengaging Sigil",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="mirage_sand_phantom" title="Djinn's Mark" -->

## Djinn's Mark

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_sand_phantom`
- Snapshot ID: `39853`
- Source-Dokument: `7071`
- Kurzinfo: Djinn's Mark aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Djinn's Mark`
- Payload Hash: `a7e32bfa092367ead96e524be61f71df2288b9a9a57b4b8310a8f4ab43027eeb`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.332264+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 3
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "mirage_sand_phantom_desc",
  "HeroKey": "hero_mirage",
  "HeroName": "Mirage",
  "Info1": {
    "Alt": [
      {
        "Key": "ProcCooldown",
        "Name": "Max Frequency",
        "Type": "cooldown",
        "Value": 3
      },
      {
        "Key": "VictimStackDuration",
        "Name": "Djinn's Mark Duration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "RevealDuration",
        "Name": "Reveal Duration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "ProcMaxRange",
        "Name": "Proc Max Range",
        "Type": "range",
        "Value": 40
      },
      {
        "Key": "StunDuration",
        "Name": "Stun Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "mirage_sand_phantom_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "ProcDamageBase",
          "Name": "Base Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.35
          },
          "Type": "tech_damage",
          "Value": 11
        },
        {
          "Key": "DMarkMultiplierPerStack",
          "Name": "Damage Per Mark",
          "Type": "damage",
          "Value": 2
        },
        {
          "Key": "MaxStacks",
          "Name": "Max Stacks",
          "Type": "distance",
          "Value": 4
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "mirage_sand_phantom_desc",
    "Main": {
      "Props": []
    }
  },
  "Key": "mirage_sand_phantom",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Djinn's Mark",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "mirage_sand_phantom_t1_desc",
      "MovementSpeedSlow": 60,
      "SlowDuration": 0.8
    },
    {
      "DescKey": "mirage_sand_phantom_t2_desc",
      "ProcDamageBase": 20,
      "VictimStackDuration": 3
    },
    {
      "AbilityCooldown": -1,
      "DescKey": "mirage_sand_phantom_t3_desc",
      "MaxStacks": 1,
      "ProcCooldown": -1,
      "StunDuration": 0.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "djinn's mark",
      "name": "Djinn's Mark",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_doorman_doorway" title="Doorway" -->

## Doorway

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_doorway`
- Snapshot ID: `39776`
- Source-Dokument: `7071`
- Kurzinfo: Doorway aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Doorway`
- Payload Hash: `e9c909c9c736b8799c2fe87dd27065123239bfc822766323b0decf5556d593e5`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.182388+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 50
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 20
  },
  "DescKey": "ability_doorman_doorway_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_doorway_desc",
    "Main": {
      "Props": [
        {
          "Key": "DoorwayDistance",
          "Name": "Doorway Distance",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "distance",
          "Value": 70
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Portal",
          "Type": "bullet_armor_up",
          "Value": 0
        },
        {
          "Key": "BarrierDuration",
          "Name": "Barrier Duration",
          "Title": "On Portal",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_doorman_doorway",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Doorway",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DoorwayCloseCooldown": {
      "Name": null,
      "Value": 8
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityDuration": 15
    },
    {
      "BarrierDuration": 12,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 250
      },
      "DescKey": "ability_doorman_doorway_t2_desc"
    },
    {
      "AbilityCastRange": 30,
      "DescKey": "ability_doorman_doorway_t3_desc",
      "DoorwayDistance": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.15
        },
        "Value": 45
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "doorway",
      "name": "Doorway",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="mirage_tornado" title="Dust Devil" -->

## Dust Devil

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_tornado`
- Snapshot ID: `39852`
- Source-Dokument: `7071`
- Kurzinfo: Dust Devil aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dust Devil`
- Payload Hash: `bb88ae552dd126f6e19b6e492924364aa0e03638bb058d67cf484e77df77475f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.330087+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 36.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "mirage_tornado_desc",
  "Duration": {
    "EnemyLiftDuration": {
      "Name": "Lift Up Time",
      "Type": "duration",
      "Value": 0.2
    }
  },
  "HeroKey": "hero_mirage",
  "HeroName": "Mirage",
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 4
      },
      {
        "Key": "WhirlwindDuration",
        "Name": "Bullet Evasion Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 30
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "DescKey": "mirage_tornado_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "HoldInPlaceDuration",
          "Name": "Lift Duration",
          "Type": "duration",
          "Value": 0.3
        },
        {
          "Key": "WhirlwindEvasionChance",
          "Name": "Bullet Evasion Chance",
          "Value": 30
        }
      ]
    }
  },
  "Key": "mirage_tornado",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Dust Devil",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ClimbHeight": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.1
    },
    "DistanceAboveGround": {
      "Name": null,
      "Value": 0.5
    },
    "DropDownRate": {
      "Name": null,
      "Value": 10
    },
    "LiftHeight": {
      "Name": null,
      "Value": 3
    },
    "MaxDeltaMovementControl": {
      "Name": null,
      "Value": 2
    },
    "ProjectileThinkInterval": {
      "Name": null,
      "Value": 0.01
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    },
    "TornadoSpeed": {
      "Name": null,
      "Value": 24
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Range": {
    "OpenHeight": {
      "Name": null,
      "Type": "distance",
      "Value": 8
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 60,
      "DescKey": "mirage_tornado_t1_desc"
    },
    {
      "AbilityCooldown": -12,
      "DescKey": "mirage_tornado_t2_desc",
      "WhirlwindEvasionChance": 30
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "mirage_tornado_t3_desc",
      "HoldInPlaceDuration": 0.3,
      "RecastWindow": 6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "dust devil",
      "name": "Dust Devil",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="synth_pulse" title="Enchanter's Satchel" -->

## Enchanter's Satchel

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_pulse`
- Snapshot ID: `39901`
- Source-Dokument: `7071`
- Kurzinfo: Enchanter's Satchel aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enchanter's Satchel`
- Payload Hash: `d8b607dd773a9bbefbd226ae6c45ffdd29516bc3c20e368feb6749af2a269879`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.424728+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 17.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_pulse_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "synth_pulse_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "Type": "cast",
          "Value": 1.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.1
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "MoveSlowPercent",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "bullet_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "synth_pulse",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Enchanter's Satchel",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.0254
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 90
    },
    {
      "AbilityChannelTime": 1.5,
      "DebuffDuration": 4.0,
      "DescKey": "synth_pulse_t3_desc",
      "FireRateSlow": 40,
      "MoveSlowPercent": 40,
      "Radius": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "enchanter's satchel",
      "name": "Enchanter's Satchel",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_netshot" title="Entangling Bola" -->

## Entangling Bola

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_netshot`
- Snapshot ID: `39937`
- Source-Dokument: `7071`
- Kurzinfo: Entangling Bola aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Entangling Bola`
- Payload Hash: `3d7318b4fe05b8d131936beb96ee175cc15f91389715bbc762c1012572d5e89b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.494433+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.24
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 23
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_netshot_desc",
  "HeroKey": "hero_werewolf",
  "HeroName": "Silver",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_werewolf_netshot_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 1.5
        }
      ]
    }
  },
  "Key": "ability_werewolf_netshot",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Entangling Bola",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "SlowPercent": 25
    },
    {
      "AbilityCooldown": -8
    },
    {
      "DebuffDuration": 0.75,
      "DescKey": "ability_werewolf_netshot_t3_desc",
      "RicochetCount": 2,
      "RicochetRange": 15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "entangling bola",
      "name": "Entangling Bola",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_tengu_urn" title="Entangling Thorns" -->

## Entangling Thorns

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_urn`
- Snapshot ID: `39903`
- Source-Dokument: `7071`
- Kurzinfo: Entangling Thorns aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Entangling Thorns`
- Payload Hash: `33df161c3965430ae130acf8d833433240750f8fbf82c8e2642a7cc9ad37730d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.427983+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 5
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "citadel_ability_tengu_urn_desc",
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_tengu_urn_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.55
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_tengu_urn",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Entangling Thorns",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Height": {
      "Name": null,
      "Value": 2
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_tengu_urn_t2_desc",
      "Radius": 2
    },
    {
      "DescKey": "citadel_ability_tengu_urn_t3_desc",
      "EntangleDuration": 1.6,
      "TimeToEntangle": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "entangling thorns",
      "name": "Entangling Thorns",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_blood_bomb" title="Essence Bomb" -->

## Essence Bomb

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_blood_bomb`
- Snapshot ID: `39807`
- Source-Dokument: `7071`
- Kurzinfo: Essence Bomb aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Essence Bomb`
- Payload Hash: `9a06288f1e050cf6c6301570a9c91b93ef19d098b8ee16e2f0dc45a4857bd361`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.242236+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_blood_bomb_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "ArmingDuration",
        "Name": "Arming Duration",
        "Type": "duration",
        "Value": 0.65
      }
    ],
    "DescKey": "ability_blood_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.22
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "SelfDamagePct",
          "Name": "Health Cost",
          "Value": 30
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "ability_blood_bomb_t3_desc",
    "Main": {
      "Props": [
        {
          "Key": "BloodSpillDuration",
          "Name": "Toxic Mess Duration",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 2
  },
  "Key": "ability_blood_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Essence Bomb",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BeepSoundBuildupCount": {
      "Name": null,
      "Value": 4
    },
    "BeepSoundIntervalBias": {
      "Name": null,
      "Value": 0.55
    },
    "BeepSoundMaxFrequency": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 7
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 50,
      "DescKey": "ability_blood_bomb_t2_desc",
      "Radius": 2
    },
    {
      "BloodSpillDPSPercent": 26,
      "BloodSpillDuration": 6,
      "DescKey": "ability_blood_bomb_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "essence bomb",
      "name": "Essence Bomb",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_necro_fear" title="Essence Theft" -->

## Essence Theft

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_fear`
- Snapshot ID: `39861`
- Source-Dokument: `7071`
- Kurzinfo: Essence Theft aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Essence Theft`
- Payload Hash: `157ba1ae66a6c696498d85998c44a9003f4ffb5bc9543d843cf29f145e0af142`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.347934+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "ability_necro_fear_desc",
  "HeroKey": "hero_necro",
  "HeroName": "Graves",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxStolenTargets",
        "Name": "Max Steal Targets",
        "Value": 3
      },
      {
        "Key": "ShootDurationForMax",
        "Name": "Time for Max Damage",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_necro_fear_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxStolenAttackDamage",
          "Name": "Max Weapon Damage Stolen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.25
          },
          "Type": "bullet_damage",
          "Value": 25
        },
        {
          "Key": "MaxStolenSpiritResist",
          "Name": "Max Spirit Resist Stolen",
          "Type": "tech_armor_down",
          "Value": 10
        },
        {
          "Key": "MaxStolenFireRate",
          "Name": "Max Fire Rate Stolen",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_necro_fear",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Essence Theft",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DelayBeforeLoss": {
      "Name": null,
      "Value": 0.5
    },
    "ProgressLossMultiplier": {
      "Name": null,
      "Value": 2.3
    },
    "ProgressLossPerSecond": {
      "Name": null,
      "Value": 1
    },
    "TickInterval": {
      "Name": null,
      "Value": 0.15
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "MaxStolenSpiritResist": 5
    },
    {
      "MaxStolenAttackDamage": 20
    },
    {
      "DescKey": "ability_necro_fear_t3_desc",
      "SkullBuildUp": 0.15,
      "ZombieExplosionBuildUp": 1.0,
      "ZombieMeleeBuildUp": 0.15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "essence theft",
      "name": "Essence Theft",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="drifter_darkness" title="Eternal Night" -->

## Eternal Night

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `drifter_darkness`
- Snapshot ID: `39782`
- Source-Dokument: `7071`
- Kurzinfo: Eternal Night aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Eternal Night`
- Payload Hash: `d2041af4ef1e0e7049b4918f1c1c1afb138effd7d0da507d154714371600d291`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.194432+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.0
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 100
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 145.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.5
  },
  "Damage": {
    "BonusFireRate": {
      "Name": "Fire Rate",
      "Scale": {
        "Type": "spirit",
        "Value": 0
      },
      "Type": "fire_rate",
      "Value": 0
    }
  },
  "DescKey": "drifter_darkness_desc",
  "Duration": {
    "RevealDuration": {
      "Name": "Reveal Duration",
      "Type": "duration",
      "Value": 3
    }
  },
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxTargets",
        "Name": "Max Targets",
        "Value": 2
      }
    ],
    "DescKey": "drifter_darkness_desc",
    "Main": {
      "Props": [
        {
          "Key": "SmallVisionDistance",
          "Name": "Reduced Vision",
          "Type": "distance",
          "Value": 15
        },
        {
          "Key": "BonusSprintSpeed",
          "Name": "Sprint Speed",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "drifter_darkness",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Eternal Night",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 0.001
    },
    "BonusSprintAcceleration": {
      "Name": null,
      "Value": 12
    },
    "DarkFactor": {
      "Name": null,
      "Value": 1.0
    },
    "DistanceForMaxProjSpeed": {
      "Name": null,
      "Value": 200
    },
    "MaxProjectileSpeed": {
      "Name": null,
      "Value": 3000
    },
    "MinProjectileSpeed": {
      "Name": null,
      "Value": 3000
    },
    "PostProcessFadeInTime": {
      "Name": null,
      "Value": 0.2
    },
    "PostProcessFadeOutTime": {
      "Name": null,
      "Value": 1.0
    }
  },
  "Range": {
    "DrifterNearbyRangeCheck": {
      "Name": "Drifter Nearby",
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusSprintSpeed": 10
    },
    {
      "AbilityCooldown": -40
    },
    {
      "AbilityDuration": 2.5,
      "DescKey": "drifter_darkness_t3_desc",
      "MaxTargets": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "eternal night",
      "name": "Eternal Night",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_uppercut" title="Exploding Uppercut" -->

## Exploding Uppercut

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_uppercut`
- Snapshot ID: `39756`
- Source-Dokument: `7071`
- Kurzinfo: Exploding Uppercut aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Exploding Uppercut`
- Payload Hash: `19267aa737398bb542fe1d3f1b1d413c7bd1bb88f0b0d5f33ad6ce6564a2daf6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.142117+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 22.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_uppercut_desc",
  "Duration": {
    "ExplodeDebuffDuration": {
      "Name": "Fire Rate Slow Duration",
      "Type": "duration",
      "Value": 5
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_uppercut_desc",
    "Main": {
      "Props": [
        {
          "Key": "UppercutDamage",
          "Name": "Uppercut Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.0
          },
          "Type": "melee_damage",
          "Value": 0.01
        },
        {
          "Key": "MissingHPHeal",
          "Name": "Missing HP Heal",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "OnLandDamageRadius",
        "Name": "Landing Radius",
        "Type": "distance",
        "Value": 14
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "LandingDamage",
          "Name": "Area Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Title": "On Landing:",
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": -0.186
          },
          "Title": "On Landing:",
          "Type": "slow",
          "Value": -14
        }
      ]
    }
  },
  "Key": "citadel_ability_uppercut",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Exploding Uppercut",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuffGunRangePercent": {
      "Name": null,
      "Value": 100
    },
    "EnemyHeroTossVelocity": {
      "Name": null,
      "Value": 20
    },
    "ForceReductionOnAngleDown": {
      "Name": null,
      "Value": 0.75
    },
    "MeleeHalfAngle": {
      "Name": null,
      "Value": 60
    },
    "TossDurationFriendly": {
      "Name": null,
      "Value": 0.3
    },
    "TossVelocity": {
      "Name": null,
      "Value": 25
    }
  },
  "Range": {
    "MeleeAttackLength": {
      "Name": "Melee Range",
      "Type": "distance",
      "Value": 6
    },
    "MeleeRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 2.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "BuffBaseWeaponPct": 30,
      "DescKey": "citadel_ability_uppercut_t2_desc",
      "UppercutBuffOnHit": 9
    },
    {
      "DescKey": "citadel_ability_uppercut_t3_desc",
      "MissingHPHeal": 18,
      "RestoreHookCooldown": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "exploding uppercut",
      "name": "Exploding Uppercut",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="mirage_fire_beetles" title="Fire Scarabs" -->

## Fire Scarabs

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_fire_beetles`
- Snapshot ID: `39851`
- Source-Dokument: `7071`
- Kurzinfo: Fire Scarabs aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fire Scarabs`
- Payload Hash: `2b0631a478b4a26de107836d0a38b6f0b781ef50a427973edfcdc7452bf5e90c`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.328192+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.05
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 35
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 1
  },
  "DescKey": "mirage_fire_beetles_desc",
  "HeroKey": "hero_mirage",
  "HeroName": "Mirage",
  "Info1": {
    "Alt": [
      {
        "Key": "StealDuration",
        "Name": "Steal Duration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "DescKey": "mirage_fire_beetles_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.1
          },
          "Type": "healing",
          "Value": 8
        },
        {
          "Key": "OutgoingDamagePenaltyPercent",
          "Name": "Damage Penalty",
          "Type": "damage",
          "Value": -20
        }
      ]
    }
  },
  "Key": "mirage_fire_beetles",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Fire Scarabs",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 100
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DPS": 7
    },
    {
      "AbilityCharges": 2,
      "DescKey": "mirage_fire_beetles_t2_desc"
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.17
        },
        "Value": 0
      },
      "OutgoingDamagePenaltyPercent": -15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "fire scarabs",
      "name": "Fire Scarabs",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_stacking_damage" title="Fixation" -->

## Fixation

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_stacking_damage`
- Snapshot ID: `39825`
- Source-Dokument: `7071`
- Kurzinfo: Fixation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fixation`
- Payload Hash: `c11fd00fef1e5a6e2d5348c9af0079902ff06eec5c3378f1c1dcf9d535a3ab30`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.276717+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_stacking_damage_desc",
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_stacking_damage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamageBonusFixedPerStack",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": 0.2
        },
        {
          "Key": "MaxStacks",
          "Name": "Max Stacks",
          "Value": 40
        },
        {
          "Key": "ProcDamage",
          "Name": "Spirit Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_stacking_damage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Fixation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_stacking_damage_t1_desc",
      "ProcDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 40
      },
      "ProcDamageStackCount": 20,
      "SlowDuration": 2,
      "SlowPercent": 15
    },
    {
      "AbilityDuration": 5,
      "DescKey": "ability_stacking_damage_t2_desc",
      "MaxStacks": 40
    },
    {
      "DamageBonusFixedPerStack": 0.14
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "fixation",
      "name": "Fixation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_skyrunner_flakshot" title="Flakshot" -->

## Flakshot

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_flakshot`
- Snapshot ID: `39889`
- Source-Dokument: `7071`
- Kurzinfo: Flakshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flakshot`
- Payload Hash: `d3282ffb056a982754f0fe488b7591cd116ed81198198fb46646c2f26f572e35`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.400826+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_skyrunner_flakshot_desc",
  "HeroKey": "hero_skyrunner",
  "HeroName": "Skyrunner",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_skyrunner_flakshot_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.093
          },
          "Type": "bullet_damage",
          "Value": 3
        },
        {
          "Key": "RicochetChance",
          "Name": "Ricochet Chance",
          "Type": "cast",
          "Value": 50
        },
        {
          "Key": "RicochetDamagePercent",
          "Name": "Ricochet Damage",
          "Type": "bullet_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "ability_skyrunner_flakshot",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flakshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DirectionVariance": {
      "Name": null,
      "Value": 0.02
    },
    "MinEffectiveness": {
      "Name": null,
      "Value": -1
    },
    "RicochetAssistRatio": {
      "Name": null,
      "Value": 0.5
    },
    "RicochetRadius": {
      "Name": "Ricochet Range",
      "Value": 20
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "RicochetChance": 50
    },
    {
      "BonusDamage": 5
    },
    {
      "RicochetDamagePercent": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_skyrunner",
      "hero_name": "Skyrunner",
      "lookup": "flakshot",
      "name": "Flakshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_flame_dash" title="Flame Dash" -->

## Flame Dash

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_flame_dash`
- Snapshot ID: `39832`
- Source-Dokument: `7071`
- Kurzinfo: Flame Dash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flame Dash`
- Payload Hash: `9a0d27eee7158f1bbde1f56d8006cb91eb9feee31f375c1e5c8a006d2e50a22f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.289042+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 38.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3.0
  },
  "DescKey": "ability_flame_dash_desc",
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [
      {
        "Key": "GroundFlameDuration",
        "Name": "Trail Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "FlameAuraRadius",
        "Name": "Trail Width",
        "Type": "distance",
        "Value": 4.5
      },
      {
        "Key": "SlowResistance",
        "Name": "Slow Resistance",
        "Type": "move_speed",
        "Value": 50
      }
    ],
    "DescKey": "ability_flame_dash_desc",
    "Main": {
      "Props": [
        {
          "Key": "SpeedBurstSpeed",
          "Name": "Max Dash Speed",
          "Type": "move_speed",
          "Value": 20
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7
          },
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Title": "On Hit:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_flame_dash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 18
    },
    "DashAirSpeed": {
      "Name": null,
      "Type": "move_speed",
      "Value": 8
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Type": "move_speed",
      "Value": 12
    }
  },
  "Name": "Flame Dash",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 1.0
    },
    "FlameDashJumpBonus": {
      "Name": null,
      "Value": 50
    },
    "GroundAuraSpacing": {
      "Name": null,
      "Value": 1
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -65
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -12.0
    },
    {
      "DPS": 20.0,
      "DescKey": "ability_flame_dash_t2_desc",
      "GroundFlameDuration": 1
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 14,
      "DescKey": "ability_flame_dash_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "flame dash",
      "name": "Flame Dash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fencer_lunge" title="Flawless Advance" -->

## Flawless Advance

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_lunge`
- Snapshot ID: `39793`
- Source-Dokument: `7071`
- Kurzinfo: Flawless Advance aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flawless Advance`
- Payload Hash: `4f393a8092f8bedf41fecea75f5193bee12e0f4e69c535f810a3b965c2aa1eaa`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.215280+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "Damage": {
    "MaxProcBleedDamagePercent": {
      "Name": null,
      "Type": "tech_damage",
      "Value": 50
    }
  },
  "DescKey": "ability_fencer_lunge_desc",
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxStabs",
        "Name": "Max Lunges",
        "Value": 3
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Value": 0
      }
    ],
    "DescKey": "ability_fencer_lunge_desc",
    "Main": {
      "Props": [
        {
          "Key": "BaseDamage",
          "Name": "Base Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.55
          },
          "Type": "tech_damage",
          "Value": 25
        },
        {
          "Key": "MaxDamageBeforePerfect",
          "Name": "Max Hold Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.9
          },
          "Type": "tech_damage",
          "Value": 40
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "PerfectDamage",
          "Name": "Perfect Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.55
          },
          "Title": "On Perfect Hold:",
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "HealFixedHealth",
          "Name": "Heal on hero hit",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Perfect Hold:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fencer_lunge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flawless Advance",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AttackingDashSpeed": {
      "Name": null,
      "Value": 55.88
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 1.85
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 27.94
    },
    "HoldDurationMax": {
      "Name": null,
      "Value": 1.1
    },
    "HoldDurationMin": {
      "Name": null,
      "Value": 0.25
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 2
    },
    "ParryCooldownReduction": {
      "Name": "Parry Cooldown",
      "Value": 5
    },
    "PctTravelDistanceToDamageIn": {
      "Name": null,
      "Value": 80
    },
    "PerfectHoldTimeStart": {
      "Name": "Perfect Window Start",
      "Value": 0.525
    },
    "PerfectWindowDuration": {
      "Name": "Perfect Window Duration",
      "Value": 0.25
    },
    "RecastTime": {
      "Name": null,
      "Value": 5
    },
    "SlashCollisionRadius": {
      "Name": null,
      "Value": 4.05
    }
  },
  "Range": {
    "AttackDashRange": {
      "Name": "Attacking Lunge Distance",
      "Type": "distance",
      "Value": 3.0
    },
    "DashRange": {
      "Name": "Time Window",
      "Type": "distance",
      "Value": 5.0
    },
    "SlashLength": {
      "Name": "Slash Length",
      "Type": "distance",
      "Value": 13
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Type": "distance",
      "Value": 1.6
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_fencer_lunge_t1_desc",
      "HealFixedHealth": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.3
        },
        "Value": 35
      }
    },
    {
      "AbilityCooldown": -12,
      "BulletResist": 60,
      "DashBuffDuration": 1.5,
      "DescKey": "ability_fencer_lunge_t2_desc"
    },
    {
      "AttackDashRange": 3.0,
      "BaseDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 30
      },
      "DashSpeed": 13.97,
      "DescKey": "ability_fencer_lunge_t3_desc",
      "MaxDamageBeforePerfect": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 45
      },
      "PerfectDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 65
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "flawless advance",
      "name": "Flawless Advance",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_hornet_leap" title="Flight" -->

## Flight

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_leap`
- Snapshot ID: `39828`
- Source-Dokument: `7071`
- Kurzinfo: Flight aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flight`
- Payload Hash: `b74cb95c20694d42601f7bbccdc4f3f8b661b5295640eb01495ef1189088c248`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.281817+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 13
  },
  "DescKey": "citadel_ability_hornet_leap_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_hornet_leap_desc",
    "Main": {
      "Props": [
        {
          "Key": "MagicDamagePerBullet",
          "Name": "Spirit Damage Per Bullet",
          "Scale": {
            "Type": "spirit",
            "Value": 0.18
          },
          "Type": "tech_damage",
          "Value": 10
        },
        {
          "Key": "FlyingItemCastRange",
          "Name": "Item Range",
          "Type": "distance",
          "Value": 50
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_leap",
  "Name": "Flight",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSideMoveSpeedPercentage": {
      "Name": null,
      "Value": -35
    },
    "JumpVelocity": {
      "Name": "Jump Velocity",
      "Value": 1000
    },
    "MaxFlyHeight": {
      "Name": null,
      "Value": 1720
    },
    "MinVelocityZ": {
      "Name": null,
      "Value": -20.0
    },
    "WeaponRecoilReduction": {
      "Name": "Recoil Reduction",
      "Value": 40
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusClipSizePercent": 50,
      "DescKey": "citadel_ability_hornet_leap_t1_desc"
    },
    {
      "AbilityDuration": 10
    },
    {
      "DescKey": "citadel_ability_hornet_leap_t3_desc",
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 10
      },
      "RefreshOnKill": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "flight",
      "name": "Flight",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_lash_flog" title="Flog" -->

## Flog

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_lash_flog`
- Snapshot ID: `39845`
- Source-Dokument: `7071`
- Kurzinfo: Flog aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flog`
- Payload Hash: `8dd1a42d2b9df3a058c4dbadbc4922d39980032e42316d601194028b40d41bfb`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.316539+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_lash_flog_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "HealPctVsNonHeroes",
        "Name": "Heal vs non-heroes",
        "Type": "healing",
        "Value": 16
      },
      {
        "Key": "EnemySlowPct",
        "Name": "Enemy Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "EnemySlowDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      }
    ],
    "DescKey": "ability_lash_flog_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.85
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "HealPctVsHeroes",
          "Name": "Heal vs heroes",
          "Type": "healing",
          "Value": 50
        },
        {
          "Key": "TargetingConeAngle",
          "Name": "Attack Angle",
          "Type": "distance",
          "Value": 38
        }
      ]
    }
  },
  "Key": "ability_lash_flog",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flog",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 30
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_lash_flog_t1_desc",
      "EnemySlowDuration": 3,
      "EnemySlowPct": 35
    },
    {
      "AbilityCooldown": -16.0,
      "DescKey": "ability_lash_flog_t2_desc",
      "FireRateSlow": 30
    },
    {
      "Damage": 80,
      "DescKey": "ability_lash_flog_t3_desc",
      "HealPctVsHeroes": 20,
      "HealPctVsNonHeroes": 6,
      "TargetingConeAngle": 40
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "flog",
      "name": "Flog",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="synth_plasma_flux" title="Flying Cloak" -->

## Flying Cloak

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_plasma_flux`
- Snapshot ID: `39900`
- Source-Dokument: `7071`
- Kurzinfo: Flying Cloak aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flying Cloak`
- Payload Hash: `891f2a70199b5f3c737e0dd1630237dcc5836dee3037be1260c17c9f6da2f51f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.423117+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_plasma_flux_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxLifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 3.8
      }
    ],
    "DescKey": "synth_plasma_flux_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 60
        }
      ]
    }
  },
  "Key": "synth_plasma_flux",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flying Cloak",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 70
    },
    {
      "DescKey": "synth_plasma_flux_t2_desc",
      "WeaponDamageBonus": 5,
      "WeaponDamageBonusDuration": 6
    },
    {
      "AbilityCooldown": -11,
      "DescKey": "synth_plasma_flux_t3_desc",
      "MaxLifetime": 1.6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "flying cloak",
      "name": "Flying Cloak",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_flying_strike" title="Flying Slash" -->

## Flying Slash

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_flying_strike`
- Snapshot ID: `39952`
- Source-Dokument: `7071`
- Kurzinfo: Flying Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Flying Slash`
- Payload Hash: `cb8b13aa80949f69b54fb2aa7c55bafc7a25b89e05b806a3d9dda35d287a93da`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.522938+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 28
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 36.0
  },
  "DescKey": "citadel_ability_flying_strike_desc",
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 2.5
      }
    ],
    "DescKey": "citadel_ability_flying_strike_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.0
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 50
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "SpiritBonus",
          "Name": "Bonus Spirit",
          "Title": "Upon reaching target:",
          "Type": "tech_damage",
          "Value": 0.0
        },
        {
          "Key": "BuffDuration",
          "Name": "Buff Duration",
          "Title": "Upon reaching target:",
          "Type": "duration",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 1
  },
  "Key": "citadel_ability_flying_strike",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flying Slash",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -18
    },
    {
      "BuffDuration": 6,
      "DescKey": "citadel_ability_flying_strike_t2_desc",
      "SpiritBonus": 40
    },
    {
      "AbilityCastRange": 15,
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 3,
      "CanGrappleAllyHeroes": 1,
      "DescKey": "citadel_ability_flying_strike_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "flying slash",
      "name": "Flying Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_ice_grenade" title="Frost Grenade" -->

## Frost Grenade

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_ice_grenade`
- Snapshot ID: `39835`
- Source-Dokument: `7071`
- Kurzinfo: Frost Grenade aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frost Grenade`
- Payload Hash: `6986e3e8e84d259b72285442cc00c42c330c76632ea199e6055fead8d518d179`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.294863+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7
  },
  "DescKey": "ability_ice_grenade_desc",
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "ability_ice_grenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "HealAmount",
          "Name": "Heal Amount",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "healing",
          "Value": 60
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 40
        }
      ]
    }
  },
  "Key": "ability_ice_grenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Frost Grenade",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 30,
      "DescKey": "ability_ice_grenade_t1_desc",
      "HealAmount": 30
    },
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_ice_grenade_t2_desc",
      "PauseStaminaRegen": 1
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 0
      },
      "DescKey": "ability_ice_grenade_t3_desc",
      "HealAmount": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.9
        },
        "Value": 0
      },
      "Radius": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frost grenade",
      "name": "Frost Grenade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_ice_dome" title="Frozen Shelter" -->

## Frozen Shelter

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_ice_dome`
- Snapshot ID: `39838`
- Source-Dokument: `7071`
- Kurzinfo: Frozen Shelter aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frozen Shelter`
- Payload Hash: `9aa6c83f1af6a45375d9fac361345e82f6de9c4d1eca49b8787106ffbf17f96b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.301188+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 8
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 195
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_ice_dome_desc",
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 35
      },
      {
        "Key": "MaxHealthRegen",
        "Name": "Max Health Heal",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "ability_ice_dome_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusHealthRegen",
          "Name": "Health Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "healing",
          "Value": 90
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_ice_dome",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Frozen Shelter",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlockerScaleFactor": {
      "Name": null,
      "Value": 115
    },
    "EnemyDragSpeed": {
      "Name": null,
      "Value": 25.4
    },
    "GrowTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 10
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 1.5
    },
    {
      "BonusHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 65
      },
      "DescKey": "ability_ice_dome_t3_desc",
      "PurgeOnCast": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frozen shelter",
      "name": "Frozen Shelter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_wraith_rapidfire" title="Full Auto" -->

## Full Auto

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wraith_rapidfire`
- Snapshot ID: `39945`
- Source-Dokument: `7071`
- Kurzinfo: Full Auto aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Full Auto`
- Payload Hash: `bbeb64f93171247bca540720c6724a81980a97e0d74af03185bef26f403ee8f5`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.508766+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "citadel_ability_wraith_rapidfire_desc",
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityLifestealPercent",
        "Name": "Spirit Lifesteal",
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "BulletLifestealPercent",
        "Name": "Bullet Lifesteal",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_wraith_rapidfire_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 20
        },
        {
          "Key": "MagicDamagePerBullet",
          "Name": "Spirit Damage Per Bullet",
          "Scale": {
            "Type": "spirit",
            "Value": 0.03
          },
          "Type": "tech_damage",
          "Value": 2
        }
      ]
    }
  },
  "Key": "citadel_ability_wraith_rapidfire",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Full Auto",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 3,
      "BonusFireRate": 10,
      "DescKey": "citadel_ability_wraith_rapidfire_t2_desc"
    },
    {
      "DescKey": "citadel_ability_wraith_rapidfire_t3_desc",
      "MagicDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      },
      "UnlimitedAmmo": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "full auto",
      "name": "Full Auto",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_nano_clustergrenade" title="Gloom Bombs" -->

## Gloom Bombs

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_clustergrenade`
- Snapshot ID: `39855`
- Source-Dokument: `7071`
- Kurzinfo: Gloom Bombs aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Gloom Bombs`
- Payload Hash: `a7030fbb41e4830b1997e7d7436aac7765e303c2890337a7dd3be7a786309fb2`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.335818+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_clustergrenade_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_nano_clustergrenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "GrenadeCount",
          "Name": "Bomb Count",
          "Type": "cast",
          "Value": 4
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.644
          },
          "Type": "tech_damage",
          "Value": 45
        },
        {
          "Key": "BonusDamageVsBarrier",
          "Name": "Bonus Damage vs Barriers",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_nano_clustergrenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Gloom Bombs",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GrenadeAngleVariance": {
      "Name": null,
      "Value": 0.08
    },
    "Lifetime": {
      "Name": "Lifetime",
      "Value": 0.75
    },
    "MultiHitPenaltyPercentage": {
      "Name": null,
      "Value": 65
    },
    "TimeBetweenGrenades": {
      "Name": null,
      "Value": 0.05
    },
    "TossSpeed": {
      "Name": null,
      "Value": 400
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 3.0
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "DescKey": "ability_nano_clustergrenade_t2_desc",
      "MeleeResistReduction": -6,
      "MeleeResistReductionDuration": 6
    },
    {
      "GrenadeCount": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "gloom bombs",
      "name": "Gloom Bombs",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_frenzy" title="Go For The Throat" -->

## Go For The Throat

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_frenzy`
- Snapshot ID: `39939`
- Source-Dokument: `7071`
- Kurzinfo: Go For The Throat aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Go For The Throat`
- Payload Hash: `546167dde00a1c206890f51cf4360270fec356d257930bb3a9fda7998c23172a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.498+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 7.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 6.5
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_frenzy_desc",
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_werewolf_frenzy_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.5
          },
          "Type": "melee_damage",
          "Value": 0.0
        },
        {
          "Key": "MissingHealthDamagePercentage",
          "Name": "Missing Health Damage",
          "Type": "melee_damage",
          "Value": 6
        },
        {
          "Key": "LifeStealPercentOnHit",
          "Name": "Unknown(LifeStealPercentOnHit)",
          "Type": "healing",
          "Value": 40
        }
      ]
    }
  },
  "Key": "ability_werewolf_frenzy",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Go For The Throat",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.5
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 16
    }
  },
  "Range": {
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 30
    },
    {
      "LifeStealPercentOnHit": 25
    },
    {
      "MissingHealthDamagePercentage": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "go for the throat",
      "name": "Go For The Throat",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="viscous_goo_bowling_ball" title="Goo Ball" -->

## Goo Ball

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_bowling_ball`
- Snapshot ID: `39930`
- Source-Dokument: `7071`
- Kurzinfo: Goo Ball aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Goo Ball`
- Payload Hash: `dac87e15b7989235d52a4384154a155aaf7e862c8af58055bad05370543af6bd`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.481021+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.55
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 11
  },
  "DescKey": "viscous_goo_bowling_ball_desc",
  "HeroKey": "hero_viscous",
  "HeroName": "Viscous",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Value": 35
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Value": 35
      }
    ],
    "DescKey": "viscous_goo_bowling_ball_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1
          },
          "Type": "tech_damage",
          "Value": 110
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "Value": 0.5
        },
        {
          "Key": "BallRadius",
          "Name": "Ball Radius",
          "Type": "radius",
          "Value": 1.4
        }
      ]
    }
  },
  "Key": "viscous_goo_bowling_ball",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 7
    },
    "MoveSpeedMax": {
      "Name": "Move Speed",
      "Type": "move_speed",
      "Value": 7
    }
  },
  "Name": "Goo Ball",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AccelerationPercentage": {
      "Name": null,
      "Value": -60
    },
    "AirJumpForce": {
      "Name": null,
      "Value": 500
    },
    "BallOffset": {
      "Name": null,
      "Value": 50
    },
    "BreakablePropDamageRadius": {
      "Name": null,
      "Value": 75
    },
    "CastWhileRolling": {
      "Name": null,
      "Value": 1
    },
    "FrictionPercentage": {
      "Name": null,
      "Value": -85
    },
    "JumpForce": {
      "Name": null,
      "Value": 500
    },
    "KnockForce": {
      "Name": null,
      "Value": 400
    },
    "ParticleRadiusMultiplier": {
      "Name": null,
      "Value": 1.2
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Range": {
    "BallHitRadius": {
      "Name": null,
      "Type": "radius",
      "Value": 1.8
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -25
    },
    {
      "BulletResist": 10,
      "Damage": 70,
      "DescKey": "viscous_goo_bowling_ball_t2_desc",
      "TechResist": 10
    },
    {
      "AbilityDuration": 7,
      "DescKey": "viscous_goo_bowling_ball_t3_desc",
      "StunDuration": 0.3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "goo ball",
      "name": "Goo Ball",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_lash" title="Grapple" -->

## Grapple

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash`
- Snapshot ID: `39844`
- Source-Dokument: `7071`
- Kurzinfo: Grapple aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grapple`
- Payload Hash: `626e9bd832af66139f97b9848f609d5bf411cc73ef6e94557e0b33f4fe17f3d9`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.314438+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 35.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "DescKey": "citadel_ability_lash_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "WeaponDamageBonus",
        "Name": "Weapon Damage",
        "Type": "bullet_damage",
        "Value": 0
      },
      {
        "Key": "WeaponDamageBonusDuration",
        "Name": "Bonus Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "WeaponFireRateBonus",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_lash_desc",
    "Main": {
      "Props": [
        {
          "Key": "JumpVelocity",
          "Name": "Jump Velocity",
          "Type": "move_speed",
          "Value": 20
        }
      ]
    }
  },
  "Key": "citadel_ability_lash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Grapple",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "JumpSlowResistance": {
      "Name": null,
      "Value": 0.667
    },
    "LashFriendlies": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -17.0
    },
    {
      "AbilityCastRange": 20,
      "DescKey": "citadel_ability_lash_t2_desc",
      "WeaponDamageBonus": 7.0,
      "WeaponDamageBonusDuration": 10
    },
    {
      "AbilityCharges": 1,
      "AirControlPercent": 60,
      "DescKey": "citadel_ability_lash_t3_desc",
      "RestoreStaminaOnUse": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "grapple",
      "name": "Grapple",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_hook" title="Grapple Arm" -->

## Grapple Arm

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hook`
- Snapshot ID: `39758`
- Source-Dokument: `7071`
- Kurzinfo: Grapple Arm aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grapple Arm`
- Payload Hash: `415d6af8a07f9b04d50f58d89fb0eb240ff8e1f7424d3f54061832885caf7311`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.146553+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 23.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_hook_desc",
  "Duration": {
    "RestrictionDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_hook_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Type": "range",
          "Value": 30
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.7
          },
          "Type": "melee_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_hook",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Grapple Arm",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CancelHookDuration": {
      "Name": null,
      "Value": 0.2
    },
    "FriendlyHookIgnoreRange": {
      "Name": null,
      "Value": 8
    },
    "HookImpactDelay": {
      "Name": null,
      "Value": 0.5
    },
    "HookingSlowSpeedLimit": {
      "Name": null,
      "Value": 5
    },
    "SlowPercent": {
      "Name": "Move Speed",
      "Value": 90
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BulletAmp": 20,
      "BulletAmpDuration": 6,
      "DescKey": "citadel_ability_hook_t1_desc"
    },
    {
      "AbilityCastRange": 30
    },
    {
      "AbilityCooldown": -11.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "grapple arm",
      "name": "Grapple Arm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_necro_zombiewall" title="Grasping Hands" -->

## Grasping Hands

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_zombiewall`
- Snapshot ID: `39860`
- Source-Dokument: `7071`
- Kurzinfo: Grasping Hands aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grasping Hands`
- Payload Hash: `5388cfdba9b2e7fa60688a34347066c32486cbfdf16c40f9d73c4169cd9220c6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.346134+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 24
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "Debuff": {
    "SlowPercent": {
      "Name": "Move Speed",
      "Type": "slow",
      "Value": 40
    }
  },
  "DescKey": "ability_necro_zombiewall_desc",
  "HeroKey": "hero_necro",
  "HeroName": "Graves",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_necro_zombiewall_desc",
    "Main": {
      "Props": [
        {
          "Key": "SummonCount",
          "Name": "Gangsters Summoned",
          "Value": 1
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "DamagePctPerWallHit",
          "Name": "Max Health Damage",
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "StatusEffect": "Immobilize",
          "Value": 1.25
        }
      ]
    }
  },
  "Key": "ability_necro_zombiewall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Grasping Hands",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraRadius": {
      "Name": "Aura Radius",
      "Value": 0.75
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 0.5
    },
    "GroundAuraPopDelay": {
      "Name": null,
      "Value": 1.1
    },
    "GroundAuraSpacing": {
      "Name": null,
      "Value": 1
    },
    "TetherDuration": {
      "Name": "Tether Duration",
      "Value": 1
    },
    "TetherRadius": {
      "Name": "Tether Radius",
      "Value": 0.1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    },
    "ZombieWallDeployTime": {
      "Name": null,
      "Value": 0.6
    },
    "ZombieWallHeight": {
      "Name": null,
      "Value": 2.5
    },
    "ZombieWallLength": {
      "Name": "Wall Length",
      "Scale": {
        "Type": "spirit",
        "Value": 0.05
      },
      "Value": 14
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "Damage": 90,
      "DescKey": "ability_necro_zombiewall_t2_desc",
      "ZombieWallLength": 10
    },
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_necro_zombiewall_t3_desc",
      "ImmobilizeDuration": 0.75,
      "SummonCount": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "grasping hands",
      "name": "Grasping Hands",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_lash_down_strike" title="Ground Strike" -->

## Ground Strike

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_down_strike`
- Snapshot ID: `39843`
- Source-Dokument: `7071`
- Kurzinfo: Ground Strike aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ground Strike`
- Payload Hash: `fb31a734210cda669192c11f916f50b1f122f57fdef4f121086e32af6f147ecc`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.312204+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "StompDamagePerMeterSecondary": {
      "Name": null,
      "Scale": {
        "Type": "spirit",
        "Value": 0.008137
      },
      "Type": "tech_damage",
      "Value": 4.2
    }
  },
  "DescKey": "citadel_ability_lash_down_strike_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "EnemySlowPct",
        "Name": "Enemy Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_lash_down_strike_desc",
    "Main": {
      "Props": [
        {
          "Key": "StompDamage",
          "Name": "Stomp Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7905
          },
          "Type": "tech_damage",
          "Value": 60.0
        },
        {
          "Key": "StompDamagePerMeterPrimary",
          "Name": "Damage Per Meter",
          "Scale": {
            "Type": "spirit",
            "Value": 0.04
          },
          "Type": "tech_damage",
          "Value": 5.5
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_lash_down_strike",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Ground Strike",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MinAimAngle": {
      "Name": null,
      "Value": 60
    },
    "StompDamagePrimaryRange": {
      "Name": null,
      "Value": 25
    },
    "StompVerticalThreshold": {
      "Name": null,
      "Value": 118
    },
    "StrikeVelocity": {
      "Name": null,
      "Value": 50
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 10
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -10.0
    },
    {
      "DescKey": "citadel_ability_lash_down_strike_t2_desc",
      "EnemySlowPct": 50,
      "SlowDuration": 3,
      "StompBounceHeight": 400,
      "TossDuration": 1
    },
    {
      "DescKey": "citadel_ability_lash_down_strike_t3_desc",
      "StompDamagePerMeterPrimary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.03255
        },
        "Value": 2.13
      },
      "StompDamagePerMeterSecondary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.008137
        },
        "Value": 2.13
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "ground strike",
      "name": "Ground Strike",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_guided_arrow" title="Guided Owl" -->

## Guided Owl

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_guided_arrow`
- Snapshot ID: `39870`
- Source-Dokument: `7071`
- Kurzinfo: Guided Owl aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Guided Owl`
- Payload Hash: `4daa41a4aa0816c81bb3fdcb6d39454dbcf16c9cb10828a82dc25ee68b5dfa8c`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.364824+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.5
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 125.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_guided_arrow_desc",
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusTechPowerPerKill",
        "Name": "Spirit Power Per Kill",
        "Type": "tech_damage",
        "Value": 8
      }
    ],
    "DescKey": "ability_guided_arrow_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93744
          },
          "Type": "tech_damage",
          "Value": 230.0
        },
        {
          "Key": "ExplosionRadius",
          "Name": "Explosion Radius",
          "Type": "distance",
          "Value": 12
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "Type": "duration",
          "Value": 0.75
        }
      ]
    }
  },
  "Key": "ability_guided_arrow",
  "Name": "Guided Owl",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 85.0
    },
    {
      "AbilityCooldown": -40.0
    },
    {
      "DescKey": "ability_guided_arrow_t3_desc",
      "LowHealthEnemyThresholdPct": 22
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "guided owl",
      "name": "Guided Owl",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_priest_knockback" title="Gutshot" -->

## Gutshot

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_knockback`
- Snapshot ID: `39872`
- Source-Dokument: `7071`
- Kurzinfo: Gutshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Gutshot`
- Payload Hash: `6c0a41169bae4897edb97df0bc90a6d8476d58a343100ec3ce5d54a00a074fd0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.368502+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 23
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_priest_knockback_desc",
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Value": 0
      },
      {
        "Key": "WallStunDistance",
        "Name": "Wall Stun Range",
        "Type": "distance",
        "Value": 7
      }
    ],
    "DescKey": "ability_priest_knockback_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 0.7
          },
          "Type": "bullet_damage",
          "Value": 60
        },
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 0.8
          },
          "Title": "On Wall Hit:",
          "Type": "bullet_damage",
          "Value": 30
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Wall Hit:",
          "Value": 0.6
        }
      ]
    }
  },
  "Key": "ability_priest_knockback",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Gutshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 99
    },
    "BonusKnockbackDistance": {
      "Name": null,
      "Value": 3.5
    },
    "KnockbackSpeed": {
      "Name": null,
      "Value": 1200
    },
    "MaxPushForceHorizontal": {
      "Name": null,
      "Value": 1400
    },
    "MaxPushForceVertical": {
      "Name": null,
      "Value": 500
    },
    "PushForce": {
      "Name": null,
      "Value": 6
    },
    "SelfPushForce": {
      "Name": null,
      "Value": 500
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 60
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_priest_knockback_t2_desc",
      "StunDuration": 0.4
    },
    {
      "BuffDuration": 5,
      "DescKey": "ability_priest_knockback_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "gutshot",
      "name": "Gutshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_rocket_barrage" title="Heavy Barrage" -->

## Heavy Barrage

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_rocket_barrage`
- Snapshot ID: `39798`
- Source-Dokument: `7071`
- Kurzinfo: Heavy Barrage aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Heavy Barrage`
- Payload Hash: `fd28234f7bd1d9f9f67181bb45796a6a5192a78a92a71f2ab9cf4c843f14497a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.225755+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 36
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 200.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "Debuff": {
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Type": "slow",
      "Value": -35
    }
  },
  "DescKey": "citadel_ability_rocket_barrage_desc",
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "MinDistance",
        "Name": "Min Range",
        "Type": "distance",
        "Value": 8.5
      },
      {
        "Key": "AbilityDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "MoveSlowDuration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_rocket_barrage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePerRocket",
          "Name": "Damage Per Rocket",
          "Scale": {
            "Type": "spirit",
            "Value": 0.2
          },
          "Type": "tech_damage",
          "Value": 21
        },
        {
          "Key": "GrenadesPerSecond",
          "Name": "Rockets per second",
          "Value": 6
        },
        {
          "Key": "ExplosionRadius",
          "Name": "Explosion Radius",
          "Type": "distance",
          "Value": 4.5
        }
      ]
    }
  },
  "Key": "citadel_ability_rocket_barrage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Heavy Barrage",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 100
    },
    "DetonateTimer": {
      "Name": null,
      "Value": 5
    },
    "ExplosionFalloffDisabled": {
      "Name": null,
      "Value": 1
    },
    "IntervalRampUpStart": {
      "Name": null,
      "Value": 0.35
    },
    "IntervalRampUpTime": {
      "Name": null,
      "Value": 0.3
    },
    "MaxSpread": {
      "Name": null,
      "Value": 5
    },
    "ProjectileIgnoreCollisionTime": {
      "Name": null,
      "Value": 0.2
    },
    "TrackSpeedFar": {
      "Name": null,
      "Value": 100
    },
    "TrackSpeedNear": {
      "Name": null,
      "Value": 150
    },
    "TrackingTime": {
      "Name": null,
      "Value": 0.4
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_rocket_barrage_t1_desc",
      "EnemyDashSlowPercent": -18,
      "MoveSlowDuration": 1,
      "MoveSlowPercent": 30
    },
    {
      "AbilityCooldown": -45.0,
      "AbilityDuration": 6,
      "DescKey": "citadel_ability_rocket_barrage_t2_desc"
    },
    {
      "DamagePerRocket": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 15
      },
      "DescKey": "citadel_ability_rocket_barrage_t3_desc",
      "ExplosionRadius": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "heavy barrage",
      "name": "Heavy Barrage",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_priest_beartrap" title="Hex-Lined Snap Trap" -->

## Hex-Lined Snap Trap

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_beartrap`
- Snapshot ID: `39873`
- Source-Dokument: `7071`
- Kurzinfo: Hex-Lined Snap Trap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hex-Lined Snap Trap`
- Payload Hash: `de48707d95305f8fe4eae3cc5d275ae8dc20ac933ce267e40147e80cd9b9ba73`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.370890+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "DescKey": "ability_priest_beartrap_desc",
  "Duration": {
    "ArmTime": {
      "Name": "Arm Time",
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "IncomingDamagePercentFromCaster",
        "Name": "Damage Taken",
        "Type": "damage",
        "Value": 0
      },
      {
        "Key": "RevealDuration",
        "Name": "Reveal Duration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "Lifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 30
      }
    ],
    "DescKey": "ability_priest_beartrap_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.2
          },
          "Type": "tech_damage",
          "Value": 80
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "StatusEffect": "Immobilize",
          "Type": "duration",
          "Value": 1.25
        }
      ]
    }
  },
  "Key": "ability_priest_beartrap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Hex-Lined Snap Trap",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TetherDuration": {
      "Name": "Tether Duration",
      "Value": 0.6
    },
    "TetherRadius": {
      "Name": "Tether Radius",
      "Value": 0.3
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    },
    "TrapHeight": {
      "Name": null,
      "Value": 2
    },
    "TripUpSpeed": {
      "Name": null,
      "Value": 6.35
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 2
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1.0
    },
    {
      "AbilityCharges": 1,
      "DescKey": "ability_priest_beartrap_t3_desc",
      "IncomingDamagePercentFromCaster": 30
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "hex-lined snap trap",
      "name": "Hex-Lined Snap Trap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_doorman_hotel" title="Hotel Guest" -->

## Hotel Guest

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_hotel`
- Snapshot ID: `39778`
- Source-Dokument: `7071`
- Kurzinfo: Hotel Guest aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hotel Guest`
- Payload Hash: `f110b18964cba0e2f2cc0b449836e48dc2b509633b5ce3dbed3f3e40f28bf930`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.186447+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 7
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 140
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.5
  },
  "DescKey": "ability_doorman_hotel_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_hotel_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Title": "Cost of Stay",
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "LateCheckoutDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Title": "Failure to Check-Out",
          "Type": "tech_damage",
          "Value": 125
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Title": "While a Guest",
          "Type": "duration",
          "Value": 6.5
        },
        {
          "Key": "HotelTimeScale",
          "Name": "Slow",
          "Title": "While a Guest",
          "Type": "slow",
          "Value": null
        }
      ]
    }
  },
  "Key": "ability_doorman_hotel",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Hotel Guest",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.7
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TimeSlowDuration": {
      "Name": null,
      "Value": 1.0
    },
    "TimeSlowPercentage": {
      "Name": null,
      "Value": 100
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20,
      "DescKey": "ability_doorman_hotel_t1_desc",
      "StaminaDrain": 1
    },
    {
      "Damage": 150,
      "DescKey": "ability_doorman_hotel_t2_desc",
      "LateCheckoutDamage": 150,
      "LateCheckoutStun": 1.5
    },
    {
      "DescKey": "ability_doorman_hotel_t3_desc",
      "LateCheckoutCooldown": 13,
      "UnstoppableWhileHotelOccupied": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "hotel guest",
      "name": "Hotel Guest",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_bebop_laser_beam" title="Hyper Beam" -->

## Hyper Beam

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bebop_laser_beam`
- Snapshot ID: `39759`
- Source-Dokument: `7071`
- Kurzinfo: Hyper Beam aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hyper Beam`
- Payload Hash: `2c1d78d8258c9afd30e60913bb38183532a1c77b43c9ddb04e6c4cb0823c3ce1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.148459+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.0
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 11
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 120.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_bebop_laser_beam_desc",
  "Duration": {
    "SlowTargetDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [
      {
        "Key": "BeamLength",
        "Name": "Beam Length",
        "Type": "distance",
        "Value": 70
      },
      {
        "Key": "BeamWidth",
        "Name": "Beam Width",
        "Value": 2.9
      }
    ],
    "DescKey": "citadel_ability_bebop_laser_beam_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 2.511
          },
          "Type": "tech_damage",
          "Value": 160
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 25
        }
      ]
    }
  },
  "Key": "citadel_ability_bebop_laser_beam",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.8
    }
  },
  "Name": "Hyper Beam",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "BeamCloseDamagePercent": {
      "Name": null,
      "Value": 75
    },
    "BeamCloseRadius": {
      "Name": null,
      "Value": 5.0
    },
    "BeamEndRadius": {
      "Name": null,
      "Value": 4.0
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Value": -40
    },
    "Interval": {
      "Name": null,
      "Value": 0.1
    },
    "TrackingSpeed": {
      "Name": null,
      "Value": 55
    },
    "ZoomBias": {
      "Name": null,
      "Value": 0.5
    },
    "ZoomTime": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "DPS": 108.0
    },
    {
      "BeamLifesteal": 65,
      "BeamLifestealNonHeroPercent": 20,
      "DescKey": "citadel_ability_bebop_laser_beam_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "hyper beam",
      "name": "Hyper Beam",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_icepath" title="Ice Path" -->

## Ice Path

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_icepath`
- Snapshot ID: `39836`
- Source-Dokument: `7071`
- Kurzinfo: Ice Path aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ice Path`
- Payload Hash: `55b2183aa6d09a316df2799c76f6f97c8478ac7d08b7bda20f6a34fe40de21c9`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.296574+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_icepath_desc",
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "IcePathAuraDuration",
        "Name": "Ice Trail Duration",
        "Type": "duration",
        "Value": 18
      }
    ],
    "DescKey": "ability_icepath_desc",
    "Main": {
      "Props": [
        {
          "Key": "MoveSpeedBonus",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 2
        },
        {
          "Key": "SprintSpeedBonus",
          "Name": "Sprint Speed",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_icepath",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Name": "move speed penalty while shooting reduction",
      "Type": "move_speed",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Name": null,
      "Type": "move_speed",
      "Value": 100
    },
    "SlideScale": {
      "Name": "Slide Distance",
      "Type": "move_speed",
      "Value": 50
    },
    "SlowResistancePercent": {
      "Name": "Slow Resist",
      "Type": "move_speed",
      "Value": 60
    }
  },
  "Name": "Ice Path",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "IcePathEdgeWidth": {
      "Name": null,
      "Value": 0.7
    },
    "IcePathInterval": {
      "Name": null,
      "Value": 0.5
    },
    "IcePathPullInStrength": {
      "Name": null,
      "Value": 20
    },
    "MinHeight": {
      "Name": null,
      "Value": 20
    },
    "PopupForce": {
      "Name": null,
      "Value": 30
    }
  },
  "Range": {
    "IcePathShardRadius": {
      "Name": "Path Width",
      "Type": "distance",
      "Value": 1.2
    },
    "ModifierRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BulletResist": 35,
      "DescKey": "ability_icepath_t1_desc",
      "MoveSpeedBonus": 2
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "BonusSpirit": 20,
      "BonusSpiritPct": 35,
      "DescKey": "ability_icepath_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "ice path",
      "name": "Ice Path",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="gunslinger_demonMark" title="Infernal Brand" -->

## Infernal Brand

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `gunslinger_demonMark`
- Snapshot ID: `39821`
- Source-Dokument: `7071`
- Kurzinfo: Infernal Brand aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infernal Brand`
- Payload Hash: `438e4823524049ec804912d57977bbd12b62ea1c3b938897b635d7735a8f4916`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.268410+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "Damage": {
      "Name": "Damage",
      "Scale": {
        "Type": "weapon_damage",
        "Value": 0.93
      },
      "Type": "bullet_damage",
      "Value": 100
    }
  },
  "DescKey": "gunslinger_demonMark_desc",
  "HeroKey": "hero_gunslinger",
  "HeroName": "Gunslinger",
  "Info1": {
    "Alt": [
      {
        "Key": "SearchRadius",
        "Name": "Search Radius",
        "Value": 20
      },
      {
        "Key": "MarkDuration",
        "Name": "Mark Duration",
        "Value": 5
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Value": 3
      }
    ],
    "DescKey": "gunslinger_demonMark_desc",
    "Main": {
      "Props": [
        {
          "Key": "ProcDamage",
          "Name": "Spirit Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 25
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "gunslinger_demonMark",
  "Name": "Infernal Brand",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SearchAngle": {
      "Name": null,
      "Value": 20
    },
    "SearchRate": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "ProcDamage": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "infernal brand",
      "name": "Infernal Brand",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_passive_beefy" title="Infernal Resilience" -->

## Infernal Resilience

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_passive_beefy`
- Snapshot ID: `39754`
- Source-Dokument: `7071`
- Kurzinfo: Infernal Resilience aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infernal Resilience`
- Payload Hash: `c3594cb9eaab5768b0931ce634996242acd0954b9eaf191ca2eb1c4aafcb33cd`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.137750+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_passive_beefy_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealthRegen",
        "Name": "Health Regen",
        "Type": "health",
        "Value": 1
      },
      {
        "Key": "BonusMaxHealth",
        "Name": "Max Health",
        "Type": "health",
        "Value": 0
      },
      {
        "Key": "MeleeLifesteal",
        "Name": "Melee Lifesteal",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_passive_beefy_desc",
    "Main": {
      "Props": [
        {
          "Key": "RegenIncomingDamagePercent",
          "Name": "Damage Regenerated",
          "Title": "Passive:",
          "Type": "healing",
          "Value": 13
        },
        {
          "Key": "RegenIncomingDamageDuration",
          "Name": "Regeneration Time",
          "Title": "Passive:",
          "Type": "duration",
          "Value": 20
        }
      ]
    }
  },
  "Key": "citadel_ability_passive_beefy",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Infernal Resilience",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "NonHeroHealPct": {
      "Name": null,
      "Value": 40
    },
    "RegenDamageInterval": {
      "Name": null,
      "Value": 1.0
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMaxHealth": 200
    },
    {
      "MeleeLifesteal": 18
    },
    {
      "DescKey": "citadel_ability_passive_beefy_t3_desc",
      "RegenIncomingDamagePercent": 8,
      "StatusResistancePercent": 20
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "infernal resilience",
      "name": "Infernal Resilience",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_boho_damageshare" title="Intertwine" -->

## Intertwine

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_boho_damageshare`
- Snapshot ID: `39762`
- Source-Dokument: `7071`
- Kurzinfo: Intertwine aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Intertwine`
- Payload Hash: `ccb83b345702810eba89ffb23a06b11033d666108eb2163cdade32933776bab7`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.155586+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_boho_damageshare_desc",
  "HeroKey": "hero_boho",
  "HeroName": "Boho",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxLinks",
        "Name": "Max Links",
        "Type": "cast",
        "Value": 6
      }
    ],
    "DescKey": "ability_boho_damageshare_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamageShareRadius",
          "Name": "Link Distance",
          "Type": "distance",
          "Value": 8
        },
        {
          "Key": "DamageSharePercentage",
          "Name": "Linked Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.186
          },
          "Type": "tech_damage",
          "Value": 30
        }
      ]
    }
  },
  "Key": "ability_boho_damageshare",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Intertwine",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 6
    },
    "LinkDuration": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DamageShareRadius": 3
    },
    {
      "AbilityDuration": 3
    },
    {
      "DamageSharePercentage": 22.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "intertwine",
      "name": "Intertwine",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_priest_weaponswap" title="Ira Domini" -->

## Ira Domini

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_weaponswap`
- Snapshot ID: `39874`
- Source-Dokument: `7071`
- Kurzinfo: Ira Domini aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ira Domini`
- Payload Hash: `f05cc4adbae10b24691410ab67f86158581cbd1bef79a724ff8665c933c48aa0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.372611+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 160
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 15
  },
  "DescKey": "ability_priest_weaponswap_desc",
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_priest_weaponswap_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 1.5
          },
          "Type": "bullet_damage",
          "Value": 120
        },
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "power_increase",
            "Value": 3.0
          },
          "Title": "While Blessed:",
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "ExecuteThreshold",
          "Name": "Execute Threshold",
          "Title": "While Blessed:",
          "Type": "damage",
          "Value": 8
        }
      ]
    }
  },
  "Key": "ability_priest_weaponswap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Ira Domini",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusAmpToVampire": {
      "Name": null,
      "Value": 5
    },
    "ExplodeRadius": {
      "Name": "Explosion Radius",
      "Value": 0.2
    },
    "PushForce": {
      "Name": null,
      "Value": 500
    },
    "StakeCount": {
      "Name": "Total Stakes",
      "Value": 3
    },
    "SwapEndDelay": {
      "Name": null,
      "Value": 0.6
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.2,
      "DescKey": "ability_priest_weaponswap_t1_desc"
    },
    {
      "AbilityCooldown": -15,
      "BonusDamage": 65,
      "DescKey": "ability_priest_weaponswap_t2_desc"
    },
    {
      "AllStakesBlessed": 1,
      "DescKey": "ability_priest_weaponswap_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "ira domini",
      "name": "Ira Domini",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fencer_ultimate" title="Itani Lo Sahn" -->

## Itani Lo Sahn

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_ultimate`
- Snapshot ID: `39794`
- Source-Dokument: `7071`
- Kurzinfo: Itani Lo Sahn aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Itani Lo Sahn`
- Payload Hash: `401b6933788b97133795460a222f356480025a1d3f08446296313f7b7bb0deff`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.217416+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.5
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 145
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "ImpactDamage": {
      "Name": "Impact Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 0.77
      },
      "Type": "tech_damage",
      "Value": 70
    }
  },
  "Debuff": {
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Type": "slow",
      "Value": -30
    }
  },
  "DescKey": "ability_fencer_ultimate_desc",
  "Duration": {
    "CasterLockDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 1.8
    }
  },
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [
      {
        "Key": "IncomingDamageReductionPercent",
        "Name": "Time Slow Damage Reduction",
        "Value": 70
      },
      {
        "Key": "LowHealthEnemyThresholdPct",
        "Name": "Low Health Threshold",
        "Value": 50
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 1.8
      }
    ],
    "DescKey": "ability_fencer_ultimate_desc",
    "Main": {
      "Props": [
        {
          "Key": "DelayedDamage",
          "Name": "Delayed Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.6
          },
          "Type": "tech_damage",
          "Value": 200
        },
        {
          "Key": "BonusDamagePercent",
          "Name": "Bonus Damage",
          "Value": 60
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "CooldownReductionOnHit",
          "Name": "Cooldown Reduced on Hero Hit",
          "Title": "On hit",
          "Type": "cooldown",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fencer_ultimate",
  "Name": "Itani Lo Sahn",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 254.0
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "GapDistanceToWall": {
      "Name": null,
      "Value": 180
    },
    "MoveSpeedPenaltyMaxSpeed": {
      "Name": null,
      "Value": 200
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -100
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.35
    },
    "TimeScaleDebuff": {
      "Name": null,
      "Value": 70
    },
    "TimerSoundDuration": {
      "Name": null,
      "Value": 1
    },
    "TravelDistPctBeforeWallGapCheck": {
      "Name": null,
      "Value": 70
    },
    "TurnRateMaxDuringCast": {
      "Name": null,
      "Value": 999
    },
    "VacuumSpeed": {
      "Name": null,
      "Value": 10.16
    }
  },
  "Range": {
    "DashRadius": {
      "Name": "Radius",
      "Type": "distance",
      "Value": 7
    },
    "DashRange": {
      "Name": "Time Window",
      "Type": "distance",
      "Value": 27
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DashRange": 8
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BonusDamagePercent": 50,
      "DescKey": "ability_fencer_ultimate_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "itani lo sahn",
      "name": "Itani Lo Sahn",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_necro_hauntingskull" title="Jar of Dead" -->

## Jar of Dead

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_hauntingskull`
- Snapshot ID: `39859`
- Source-Dokument: `7071`
- Kurzinfo: Jar of Dead aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jar of Dead`
- Payload Hash: `585ce929f63beccbad7ee438ab211a9daa0d19216fba7e9a6fa5ec388a7bd559`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.344003+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 4
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 13
  },
  "Cast": {
    "ResourceCost": {
      "Name": null,
      "Scale": {
        "Type": "cooldown",
        "Value": 1.0
      },
      "Type": "cast",
      "Value": 120
    }
  },
  "DescKey": "ability_necro_hauntingskull_desc",
  "Duration": {
    "SkullLifetime": {
      "Name": "Deadhead Lifetime",
      "Type": "duration",
      "Value": 10
    }
  },
  "Health": {
    "SummonHealth": {
      "Name": "Summon Health",
      "Scale": {
        "Type": "power_increase",
        "Value": 1.3
      },
      "Type": "health",
      "Value": 20
    }
  },
  "HeroKey": "hero_necro",
  "HeroName": "Graves",
  "Info1": {
    "Alt": [
      {
        "Key": "HealOnHit",
        "Value": 0
      },
      {
        "Key": "TargetSearchRadius",
        "Name": "Search Range",
        "Type": "distance",
        "Value": 7
      },
      {
        "Key": "TargetDashRadius",
        "Name": "Dash Range",
        "Scale": {
          "Type": "range",
          "Value": 1.0
        },
        "Type": "distance",
        "Value": 15
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 0
      },
      {
        "Key": "HealPerPickup",
        "Name": "Heal On Pickup",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0
        },
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "ability_necro_hauntingskull_desc",
    "Main": {
      "Props": [
        {
          "Key": "SkullCount",
          "Name": "Deadheads",
          "Type": "cast",
          "Value": 4
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.25
          },
          "Type": "tech_damage",
          "Value": 16
        }
      ]
    }
  },
  "Key": "ability_necro_hauntingskull",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Jar of Dead",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DelayBeforeRespawning": {
      "Name": null,
      "Value": 1
    },
    "KillTime": {
      "Name": null,
      "Value": 0.2
    },
    "MaxHits": {
      "Name": "Max Hits",
      "Value": -1
    },
    "PickupsPerBossDeath": {
      "Name": null,
      "Value": 5
    },
    "PickupsPerDeath": {
      "Name": null,
      "Value": 1
    },
    "PickupsPerHeroDeath": {
      "Name": null,
      "Value": 5
    },
    "PickupsPerNeutralTrooperDeath": {
      "Name": null,
      "Value": 2
    },
    "ResourceGenerationPercent": {
      "Name": null,
      "Value": 100
    },
    "ResourcePerPickup": {
      "Name": null,
      "Value": 10
    },
    "ResourceRadius": {
      "Name": null,
      "Value": 40
    },
    "SkullImmuneDuration": {
      "Name": null,
      "Value": 0.15
    },
    "SkullKillGold": {
      "Name": null,
      "Scale": {
        "Type": "power_increase",
        "Value": 0.5
      },
      "Value": 7
    },
    "SpawnRadius": {
      "Name": null,
      "Value": 2
    },
    "SummonTakesDamage": {
      "Name": null,
      "Value": 1
    },
    "TargetSearchDelayMax": {
      "Name": null,
      "Value": 1.25
    },
    "TargetSearchDelayMin": {
      "Name": null,
      "Value": 1.5
    },
    "TargetSearchInitialDelayMax": {
      "Name": null,
      "Value": 0.2
    },
    "TargetSearchInitialDelayMin": {
      "Name": null,
      "Value": 0.15
    },
    "TargetSearchInitialStagger": {
      "Name": null,
      "Value": 0.125
    },
    "TickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "ability_necro_hauntingskull_t1_desc",
      "HealPerPickup": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.1
        },
        "Value": 5
      }
    },
    {
      "DescKey": "ability_necro_hauntingskull_t2_desc",
      "SlowDuration": 1,
      "SlowPercent": 30
    },
    {
      "DescKey": "ability_necro_hauntingskull_t3_desc",
      "SkullCount": 2,
      "SkullLifetime": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "jar of dead",
      "name": "Jar of Dead",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_swan_leap" title="Jeté" -->

## Jeté

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_swan_leap`
- Snapshot ID: `39896`
- Source-Dokument: `7071`
- Kurzinfo: Jeté aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jeté`
- Payload Hash: `b2a7edb0339160b913d70b716426a6ebf6a8c0b96b97a295e9df35d5fde432fe`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.415699+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_swan_leap_desc",
  "HeroKey": "hero_swan",
  "HeroName": "Swan",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_swan_leap_desc",
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_swan_leap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Jeté",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuffDuration": {
      "Name": "Buff Duration",
      "Value": 7
    },
    "JumpPitch": {
      "Name": null,
      "Value": -30
    },
    "JumpSpeed": {
      "Name": null,
      "Value": 20
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_swan",
      "hero_name": "Swan",
      "lookup": "jeté",
      "name": "Jeté",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_frank_selfzap" title="Jumpstart" -->

## Jumpstart

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_selfzap`
- Snapshot ID: `39804`
- Source-Dokument: `7071`
- Kurzinfo: Jumpstart aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Jumpstart`
- Payload Hash: `ec2506c95117edaab7cd9c7696bc9f32ffb75f20d2f9b41525856e0a0224322c`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.236830+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4.5
  },
  "DescKey": "ability_frank_selfzap_desc",
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_frank_selfzap_desc",
    "Main": {
      "Props": [
        {
          "Key": "CurrentHealthPercentDamage",
          "Name": "Current Health",
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "TotalHealthRegen",
          "Name": "Total HP Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 1.2
          },
          "Title": "On Buff:",
          "Type": "healing",
          "Value": 100
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "On Buff:",
          "Type": "move_speed",
          "Value": 3
        },
        {
          "Key": "StatusResistancePercent",
          "Name": "Debuff Resist",
          "Title": "On Buff:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_frank_selfzap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Jumpstart",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3
    },
    {
      "AbilityCooldown": -8,
      "DescKey": "ability_frank_selfzap_t2_desc",
      "TotalHealthRegen": 70
    },
    {
      "AbilityCharges": 1,
      "DescKey": "ability_frank_selfzap_t3_desc",
      "StatusResistancePercent": 50,
      "TotalHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.9
        },
        "Value": 0
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "jumpstart",
      "name": "Jumpstart",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_shiv_killing_blow" title="Killing Blow" -->

## Killing Blow

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_killing_blow`
- Snapshot ID: `39886`
- Source-Dokument: `7071`
- Kurzinfo: Killing Blow aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Killing Blow`
- Payload Hash: `2a9f99933ac381c094426557f5bd455f8e19bd9a1b78d1d42902b25ce587b90e`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.395289+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.05
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 12
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 145.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "RageDrainDelayDuration": {
      "Name": null,
      "Type": "cooldown",
      "Value": 12
    },
    "RecastWindow": {
      "Name": "Recast Window",
      "Type": "cooldown",
      "Value": 20
    }
  },
  "Duration": {
    "RageDrainRate": {
      "Name": null,
      "Type": "duration",
      "Value": 0.25
    }
  },
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_killing_blow_active_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 200
        },
        {
          "Key": "EnemyHealthPercent",
          "Name": "Enemy health threshold",
          "Type": "health",
          "Value": 20
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_killing_blow_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "BuffDamage",
          "Name": "Full Rage Damage Bonus",
          "Type": "damage",
          "Value": 8
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_killing_blow_passive_desc_footer",
    "Main": {}
  },
  "Key": "citadel_ability_shiv_killing_blow",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Killing Blow",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.15
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusAbilityResource": {
      "Name": null,
      "Value": 10
    },
    "CameraDistance": {
      "Name": null,
      "Value": 400
    },
    "EnemyHealthPercentBuffer": {
      "Name": null,
      "Value": 3
    },
    "FailedExecuteCooldownPenalty": {
      "Name": null,
      "Value": 30
    },
    "MinTimeToTarget": {
      "Name": null,
      "Value": 0.5
    },
    "MoveSpeedToTarget": {
      "Name": null,
      "Value": 30
    },
    "RagePerHeavyMelee": {
      "Name": null,
      "Value": 2.85384
    },
    "RagePerLightMelee": {
      "Name": null,
      "Value": 1.55664
    },
    "RagePerSpiritDamage": {
      "Name": null,
      "Value": 0.01452864
    },
    "RagePerWeaponDamage": {
      "Name": null,
      "Value": 0.0158766
    },
    "SlashRange": {
      "Name": null,
      "Value": 90
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCastRange": 6,
      "BonusMoveSpeed": 2,
      "DescKey": "citadel_ability_shiv_killing_blow_t1_desc"
    },
    {
      "AbilityCooldown": -25,
      "BuffDamage": 16,
      "DescKey": "citadel_ability_shiv_killing_blow_t2_desc"
    },
    {
      "DescKey": "citadel_ability_shiv_killing_blow_t3_desc",
      "EnemyHealthPercent": 8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "killing blow",
      "name": "Killing Blow",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_chrono_kinetic_carbine" title="Kinetic Carbine" -->

## Kinetic Carbine

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_kinetic_carbine`
- Snapshot ID: `39773`
- Source-Dokument: `7071`
- Kurzinfo: Kinetic Carbine aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kinetic Carbine`
- Payload Hash: `efd92e90b6a78f4094d0861307b2065a312703e2b1adf6690bfe60396eaa3a5f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.176769+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_chrono_kinetic_carbine_desc",
  "Duration": {
    "MaxChargeDuration": {
      "Name": "Full Charge Time",
      "Type": "duration",
      "Value": 2.5
    },
    "MinSlowDuration": {
      "Name": "Min Time-Stop",
      "Type": "duration",
      "Value": 0.25
    }
  },
  "HeroKey": "hero_chrono",
  "HeroName": "Paradox",
  "Info1": {
    "Alt": [
      {
        "Key": "MinBonusBulletDamage",
        "Name": "Min Damage",
        "Scale": {
          "Type": "weapon_power",
          "Value": 25
        },
        "Type": "tech_damage",
        "Value": 5
      },
      {
        "Key": "SpeedBoostDuration",
        "Name": "Charge Hold Duration",
        "Type": "duration",
        "Value": 3.5
      },
      {
        "Key": "HeadshotBonus",
        "Name": "Headshot Damage",
        "Type": "tech_damage",
        "Value": 14
      }
    ],
    "DescKey": "citadel_ability_chrono_kinetic_carbine_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxBonusBulletDamage",
          "Name": "Max Damage",
          "Scale": {
            "Type": "weapon_power",
            "Value": 125
          },
          "Type": "tech_damage",
          "Value": 5
        },
        {
          "Key": "MaxSlowDuration",
          "Name": "Max Time-Stop",
          "Type": "duration",
          "Value": 0.4
        },
        {
          "Key": "SpeedChange",
          "Name": "Bonus Speed",
          "Scale": {
            "Type": "spirit",
            "Value": 0.13
          },
          "Type": "move_speed",
          "Value": 25
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_kinetic_carbine",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Kinetic Carbine",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirMoveIncreasePercent": {
      "Name": "Air Jump/Dash Distance",
      "Value": 20
    },
    "BonusBulletSpeed": {
      "Name": null,
      "Value": 100
    },
    "BulletRadiusOverride": {
      "Name": null,
      "Value": 16.0
    },
    "BulletTimeScale": {
      "Name": null,
      "Value": 0.01
    },
    "MoveSpeedWhileShootingPenaltyReduction": {
      "Name": null,
      "Value": 100
    },
    "ProjectileTimeScale": {
      "Name": null,
      "Value": 0.01
    },
    "ShotCount": {
      "Name": null,
      "Value": 1
    },
    "TimeScaleDebuff": {
      "Name": null,
      "Value": 90
    },
    "TimeWarpRadius": {
      "Name": null,
      "Value": 5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "MaxSlowDuration": 0.4
    },
    {
      "AbilityCooldown": -12,
      "DescKey": "citadel_ability_chrono_kinetic_carbine_t2_desc",
      "SpeedChange": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 0
      }
    },
    {
      "DescKey": "citadel_ability_chrono_kinetic_carbine_t3_desc",
      "MaxBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "MinBonusBulletDamage": {
        "Scale": {
          "Type": "weapon_power",
          "Value": 55
        },
        "Value": 0
      },
      "SpeedBoostDuration": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "kinetic carbine",
      "name": "Kinetic Carbine",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_stomp" title="Kinetic Pulse" -->

## Kinetic Pulse

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_stomp`
- Snapshot ID: `39783`
- Source-Dokument: `7071`
- Kurzinfo: Kinetic Pulse aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kinetic Pulse`
- Payload Hash: `4f22e646e03f54b62c11d3ecc6ae1b44b4fc4d66ed9fda7f21b2fbee60fee48d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.196279+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.42
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 5
  },
  "DescKey": "citadel_ability_stomp_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "StompRange",
        "Name": "Pulse Range",
        "Type": "distance",
        "Value": 16
      },
      {
        "Key": "StompWidth",
        "Name": "Pulse Width",
        "Type": "distance",
        "Value": 5.5
      }
    ],
    "DescKey": "citadel_ability_stomp_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.55
          },
          "Type": "tech_damage",
          "Value": 115.0
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 1
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BulletResistReduction",
          "Name": "Bullet Resist",
          "Title": "On Hit:",
          "Type": "bullet_armor_down",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "SlowDuration",
          "Name": "Slow Duration",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_stomp",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Kinetic Pulse",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ClimbHeight": {
      "Name": null,
      "Value": 1.0
    },
    "DistanceAboveGround": {
      "Name": null,
      "Value": 1.0
    },
    "DropDownRate": {
      "Name": null,
      "Value": 20
    },
    "ImpactInterval": {
      "Name": null,
      "Value": 0.1
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.2
    },
    "TossSpeed": {
      "Name": null,
      "Value": 450
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "DescKey": "citadel_ability_stomp_t1_desc"
    },
    {
      "BulletResistReduction": -15,
      "DescKey": "citadel_ability_stomp_t2_desc",
      "SlowDuration": 4,
      "SlowPercent": 30
    },
    {
      "Damage": 135,
      "DescKey": "citadel_ability_stomp_t3_desc",
      "StompRange": 20
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "kinetic pulse",
      "name": "Kinetic Pulse",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_tangotether" title="Kudzu Connection" -->

## Kudzu Connection

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tangotether`
- Snapshot ID: `39904`
- Source-Dokument: `7071`
- Kurzinfo: Kudzu Connection aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kudzu Connection`
- Payload Hash: `68b655a0b7e1787a9f9b27584a75e12bceda2fcef81304dc2e5f717f6edeb6ad`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.429643+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 16
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 37.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 12
  },
  "DescKey": "citadel_ability_tangotether_desc",
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [
      {
        "Key": "MoveSpeedBonus",
        "Name": "Move Speed",
        "Type": "move_speed",
        "Value": 0
      },
      {
        "Key": "TotalTetherTargets",
        "Name": "Tether Count",
        "Value": 1
      }
    ],
    "DescKey": "citadel_ability_tangotether_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": 0.18
          },
          "Type": "fire_rate",
          "Value": 10
        },
        {
          "Key": "BulletLifestealPercent",
          "Name": "Bullet Lifesteal",
          "Scale": {
            "Type": "spirit",
            "Value": 0.15
          },
          "Type": "healing",
          "Value": 15
        },
        {
          "Key": "TetherSharedHealPct",
          "Name": "Replicated Healing",
          "Scale": {
            "Type": "power_increase",
            "Value": 0.85
          },
          "Type": "healing",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_tangotether",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Name": "move speed penalty while shooting reduction",
      "Type": "move_speed",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Name": null,
      "Type": "move_speed",
      "Value": 100
    }
  },
  "Name": "Kudzu Connection",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HealingPerGlub": {
      "Name": null,
      "Value": 20
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "MoveSpeedBonus": 2
    },
    {
      "BonusFireRate": 8,
      "BulletLifestealPercent": 8,
      "DescKey": "citadel_ability_tangotether_t2_desc"
    },
    {
      "AbilityCooldown": -37,
      "AbilityDuration": -13,
      "DescKey": "citadel_ability_tangotether_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "kudzu connection",
      "name": "Kudzu Connection",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_warden_riot_protocol" title="Last Stand" -->

## Last Stand

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_riot_protocol`
- Snapshot ID: `39934`
- Source-Dokument: `7071`
- Kurzinfo: Last Stand aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Last Stand`
- Payload Hash: `84d55d965c47872872af9e579f8805c2c53ab1b8b4ab701cd18acbb009857759`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.488781+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 180.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_warden_riot_protocol_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [
      {
        "Key": "PulseInterval",
        "Name": "Pulse Interval",
        "Type": "duration",
        "Value": 0.5
      },
      {
        "Key": "HealthStealPct",
        "Name": "Non-Hero Lifesteal",
        "Type": "healing",
        "Value": 10
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 50
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 50
      }
    ],
    "DescKey": "ability_warden_riot_protocol_desc",
    "Main": {
      "Props": [
        {
          "Key": "PulseDPS",
          "Name": "DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 70
        },
        {
          "Key": "HealthStealPctHero",
          "Name": "Hero Lifesteal",
          "Type": "healing",
          "Value": 75
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_warden_riot_protocol",
  "Name": "Last Stand",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ConeAngle": {
      "Name": "Cone Angle",
      "Value": 115
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Radius": 4
    },
    {
      "AbilityCooldown": -30.0,
      "DescKey": "ability_warden_riot_protocol_t2_desc",
      "PulseDPS": 40.5
    },
    {
      "AbilityDuration": 4,
      "BulletResist": 30,
      "DescKey": "ability_warden_riot_protocol_t3_desc",
      "TechResist": 30,
      "UnstoppableCastDelay": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "last stand",
      "name": "Last Stand",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_nano_dash" title="Leaping Slash" -->

## Leaping Slash

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_dash`
- Snapshot ID: `39856`
- Source-Dokument: `7071`
- Kurzinfo: Leaping Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Leaping Slash`
- Payload Hash: `eb2e548fdfeffdd99cda4a34061d6faf6d5f39ad73cd76a0a27fe43dcb0fc94f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.337963+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 9
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 13
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_dash_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [
      {
        "Key": "BountyDuration",
        "Name": "Bounty Duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_nano_dash_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.8
          },
          "Type": "melee_damage",
          "Value": 10.0
        },
        {
          "Key": "HealAmount",
          "Name": "Heal Amount",
          "Scale": {
            "Type": "spirit",
            "Value": 1.4
          },
          "Title": "On Hero Hit:",
          "Type": "healing",
          "Value": 40
        },
        {
          "Key": "CooldownRefundPercent",
          "Name": "Cooldown Refund",
          "Title": "On Hero Hit:",
          "Type": "cooldown",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_nano_dash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Leaping Slash",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 550
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 2.0
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 60.96
    },
    "MoveSpeedPenaltyMaxSpeed": {
      "Name": null,
      "Value": 200
    },
    "PostDashMaintainedVelocityRatio": {
      "Name": null,
      "Value": 0.15
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -90
    },
    "SlashForwardOffset": {
      "Name": null,
      "Value": 1.5
    },
    "SlashHeight": {
      "Name": null,
      "Value": 2.5
    }
  },
  "Range": {
    "SlashRadius": {
      "Name": "Slash Radius",
      "Type": "distance",
      "Value": 4
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "HealAmount": 25
    },
    {
      "BonusGoldOnKill": 200,
      "BountyDuration": 3,
      "DescKey": "ability_nano_dash_t2_desc"
    },
    {
      "CooldownRefundPercent": 50,
      "DescKey": "ability_nano_dash_t3_desc",
      "ImpactDamage": 60
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "leaping slash",
      "name": "Leaping Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_viper_venom" title="Lethal Venom" -->

## Lethal Venom

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_venom`
- Snapshot ID: `39924`
- Source-Dokument: `7071`
- Kurzinfo: Lethal Venom aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lethal Venom`
- Payload Hash: `4e7793b3d15dbe40016d5d2f32c7e59646c88f313efed0dd3bb66acb5c114766`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.469020+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "power_increase",
      "Value": 0.2
    },
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_viper_venom_desc",
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [
      {
        "Key": "VenomMaxDamageHealthPercentage",
        "Name": "Health for Max Damage",
        "Value": 30
      },
      {
        "Key": "VenomDuration",
        "Name": "Venom Buildup Duration",
        "Value": 3
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Name": "Healing Reduction",
        "Value": 0
      }
    ],
    "DescKey": "ability_viper_venom_desc",
    "Main": {
      "Props": [
        {
          "Key": "VenomMinDamage",
          "Name": "Minimum Venom Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 20
        },
        {
          "Key": "VenomMaxDamage",
          "Name": "Max Venom Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.79
          },
          "Type": "tech_damage",
          "Value": 140
        },
        {
          "Key": "VenomMissingHealthDamagePercentage",
          "Name": "Missing Health Damage",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_viper_venom",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Lethal Venom",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuildUpDuration": {
      "Name": null,
      "Value": 5
    },
    "VenomBuildupPerShot": {
      "Name": null,
      "Value": 1
    },
    "VenomMinDamageHealthPercentage": {
      "Name": null,
      "Value": 100
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "VenomMaxDamage": 31.5
    },
    {
      "AbilityCooldown": -12,
      "DescKey": "ability_viper_venom_t2_desc",
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
    },
    {
      "BuildUpPerShot": 4.5,
      "DescKey": "ability_viper_venom_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "lethal venom",
      "name": "Lethal Venom",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_life_drain" title="Life Drain" -->

## Life Drain

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_life_drain`
- Snapshot ID: `39808`
- Source-Dokument: `7071`
- Kurzinfo: Life Drain aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Life Drain`
- Payload Hash: `85a5ea40e828a019c969a50fd60102fb62a1d81c33812dd2b3810fbc61dcecdc`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.244084+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 18
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.5
  },
  "Debuff": {
    "MoveSpeedReduction": {
      "Name": null,
      "Type": "slow",
      "Value": 40
    }
  },
  "DescKey": "ability_life_drain_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "LifeDrainHealthMult",
        "Name": "Damage to Heal",
        "Type": "healing",
        "Value": 75
      },
      {
        "Key": "MaxRange",
        "Name": "Max Tether Range",
        "Type": "distance",
        "Value": 28
      }
    ],
    "DescKey": "ability_life_drain_desc",
    "Main": {
      "Props": [
        {
          "Key": "LifeDrainPerSecond",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.43
          },
          "Type": "tech_damage",
          "Value": 32
        }
      ]
    }
  },
  "Key": "ability_life_drain",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Life Drain",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 10
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "LifeDrainPerSecond": 18
    },
    {
      "AbilityDuration": 2.5
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 0.1,
      "DescKey": "ability_life_drain_t3_desc",
      "LifeDrainPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
        },
        "Value": 0
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "life drain",
      "name": "Life Drain",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_skyrunner_swingline" title="Lifethread" -->

## Lifethread

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_swingline`
- Snapshot ID: `39887`
- Source-Dokument: `7071`
- Kurzinfo: Lifethread aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lifethread`
- Payload Hash: `0bce603281c37209bc97bb35fecfe2f4afda2c099c2e1f5e505f99dba9a77e7c`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.397182+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 0.1
  },
  "DescKey": "ability_skyrunner_swingline_desc",
  "HeroKey": "hero_skyrunner",
  "HeroName": "Skyrunner",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_skyrunner_swingline_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxMoveSpeed",
          "Name": "Max Movement Speed",
          "Type": "move_speed",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_skyrunner_swingline",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Lifethread",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SwingLineBulletDistance": {
      "Name": null,
      "Value": 40
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_skyrunner",
      "hero_name": "Skyrunner",
      "lookup": "lifethread",
      "name": "Lifethread",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_unicorn_radiantblast" title="Light Eater" -->

## Light Eater

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_radiantblast`
- Snapshot ID: `39911`
- Source-Dokument: `7071`
- Kurzinfo: Light Eater aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Light Eater`
- Payload Hash: `6f5290a646fb0ee7f9af1a3ac57291b0b10e3be7722b4c6fdedbeac00eff19bd`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.444003+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.05
    },
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_unicorn_radiantblast_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Name": "Duration",
        "Scale": {
          "Type": "spirit",
          "Value": 0.05
        },
        "Type": "duration",
        "Value": 8
      }
    ],
    "DescKey": "ability_unicorn_radiantblast_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityLifestealPercentHero",
          "Name": "Spirit Lifesteal",
          "Type": "healing",
          "Value": 20
        },
        {
          "Key": "FlareDamage",
          "Name": "Flare Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.47
          },
          "Type": "tech_damage",
          "Value": 40
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.34
          },
          "Title": "On Bullet Hit",
          "Type": "tech_damage",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_unicorn_radiantblast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Light Eater",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 100
    },
    "ExtraSweepRadius": {
      "Name": null,
      "Value": 2
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 70
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityLifestealPercentHero": 15
    },
    {
      "AbilityCastRange": 3,
      "AbilityCooldown": -10,
      "DescKey": "ability_unicorn_radiantblast_t2_desc"
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Value": 25
      },
      "DescKey": "ability_unicorn_radiantblast_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "light eater",
      "name": "Light Eater",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_lightning_ball" title="Lightning Ball" -->

## Lightning Ball

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lightning_ball`
- Snapshot ID: `39811`
- Source-Dokument: `7071`
- Kurzinfo: Lightning Ball aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lightning Ball`
- Payload Hash: `8a1fbf90ce1eee56c10d714cf8249eb5aa7e552a5ee5bc4202a709c82d46e486`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.250050+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 6
  },
  "DescKey": "citadel_ability_lightning_ball_desc",
  "HeroKey": "hero_gigawatt",
  "HeroName": "Seven",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxLifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 5
      }
    ],
    "DescKey": "citadel_ability_lightning_ball_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.5
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "ShockRadius",
          "Name": "Radius",
          "Type": "distance",
          "Value": 4.25
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_lightning_ball",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Lightning Ball",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MinShockDuration": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "DescKey": "citadel_ability_lightning_ball_t2_desc",
      "MaxLifetime": 1,
      "SlowPercent": 35
    },
    {
      "DPS": 58.5,
      "DescKey": "citadel_ability_lightning_ball_t3_desc",
      "ShockRadius": 1.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "lightning ball",
      "name": "Lightning Ball",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_familiar_helpinghands" title="Lil Helpers" -->

## Lil Helpers

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_helpinghands`
- Snapshot ID: `39789`
- Source-Dokument: `7071`
- Kurzinfo: Lil Helpers aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lil Helpers`
- Payload Hash: `18727015c4d26c26fd90908373ee5c5e60e73d6234630100672f4180430e5f64`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.207077+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Type": "range",
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "DPSPerSprite": {
      "Name": null,
      "Type": "tech_damage",
      "Value": 1
    },
    "Damage": {
      "Name": "Damage",
      "Type": "tech_damage",
      "Value": 20
    }
  },
  "DescKey": "ability_familiar_helpinghands_desc",
  "Duration": {
    "ArmTime": {
      "Name": "Arm Time",
      "Type": "duration",
      "Value": 0.1
    },
    "HelperChoreCooldownDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 5
    },
    "HelperDowntimeDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 15.1
    },
    "InfestHealInterval": {
      "Name": null,
      "Type": "duration",
      "Value": 2.0
    },
    "PatrolDamageCooldown": {
      "Name": null,
      "Type": "duration",
      "Value": 10
    }
  },
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_familiar_helpinghands_desc",
    "Main": {
      "Props": []
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "TechArmorGain",
          "Name": "Spirit Resist",
          "Title": "Following Hero:",
          "Type": "tech_armor_up",
          "Value": 12
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "Following Hero:",
          "Type": "move_speed",
          "Value": 3.0
        },
        {
          "Key": "PlayerInfestDuration",
          "Name": "Duration",
          "Title": "Following Hero:",
          "Value": 8
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "InfestDamageTakenPercent",
          "Name": "Damage / Resists",
          "Title": "Following Trooper:",
          "Value": 30
        },
        {
          "Key": "InfestHeal",
          "Name": "Trooper Heal",
          "Scale": {
            "Type": "spirit",
            "Value": 0.14
          },
          "Title": "Following Trooper:",
          "Type": "healing",
          "Value": 8
        },
        {
          "Key": "NPCInfestDuration",
          "Name": "Duration",
          "Title": "Following Trooper:",
          "Value": 50
        }
      ]
    }
  },
  "Key": "ability_familiar_helpinghands",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Lil Helpers",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HelperCount": {
      "Name": null,
      "Value": 1
    },
    "HelpersPerPatrol": {
      "Name": null,
      "Value": 4
    },
    "InfestBurstHealthPercent": {
      "Name": null,
      "Value": 75
    },
    "TickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Range": {
    "AuraAttackHeight": {
      "Name": null,
      "Type": "distance",
      "Value": 10
    },
    "AuraRadius": {
      "Name": "Aura Radius",
      "Type": "distance",
      "Value": 10
    },
    "AuraSoftRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 10
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.5,
      "DescKey": "ability_familiar_helpinghands_t1_desc",
      "HelperCount": 1
    },
    {
      "DescKey": "ability_familiar_helpinghands_t2_desc",
      "HelperCount": 1,
      "InfestDamageTakenPercent": 15
    },
    {
      "DescKey": "ability_familiar_helpinghands_t3_desc",
      "HelperCount": 1,
      "TechArmorGain": 15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "lil helpers",
      "name": "Lil Helpers",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_vampirebat_lovebites" title="Love Bites" -->

## Love Bites

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_lovebites`
- Snapshot ID: `39917`
- Source-Dokument: `7071`
- Kurzinfo: Love Bites aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Love Bites`
- Payload Hash: `56aaa3b0696b8cfecee98388e2c93d63d646c75101ae18f5888dc1a957f7228f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.455414+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "PerTargetCooldown": {
      "Name": null,
      "Type": "cooldown",
      "Value": 10
    }
  },
  "DescKey": "ability_vampirebat_lovebites_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_vampirebat_lovebites_desc",
    "Main": {
      "Props": [
        {
          "Key": "MagicDamagePerBullet",
          "Name": "Spirit Damage Per Bullet",
          "Scale": {
            "Type": "spirit",
            "Value": 0.09
          },
          "Type": "tech_damage",
          "Value": 4
        },
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.85
          },
          "Title": "On Proc:",
          "Type": "tech_damage",
          "Value": 45
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Title": "On Proc:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_vampirebat_lovebites",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Love Bites",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuildUpDuration": {
      "Name": null,
      "Value": 5
    },
    "BuildUpHeadshotBonus": {
      "Name": null,
      "Value": 1.5
    },
    "BuildUpPerBat": {
      "Name": null,
      "Value": 20
    },
    "BuildUpPerDagger": {
      "Name": null,
      "Value": 30
    },
    "BuildUpPerShot": {
      "Name": null,
      "Value": 18.4
    },
    "EffectivenessVolumeScaleMax": {
      "Name": null,
      "Value": 1.0
    },
    "EffectivenessVolumeScaleMin": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_vampirebat_lovebites_t1_desc",
      "SlowDuration": 3,
      "SlowPercent": 30
    },
    {
      "BonusDamage": 45,
      "DescKey": "ability_vampirebat_lovebites_t2_desc",
      "MagicDamagePerBullet": 3.0
    },
    {
      "BonusFireRate": 25,
      "BuffDuration": 5,
      "DescKey": "ability_vampirebat_lovebites_t3_desc",
      "PerTargetCooldown": -5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "love bites",
      "name": "Love Bites",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_doorman_luggage_cart" title="Luggage Cart" -->

## Luggage Cart

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_luggage_cart`
- Snapshot ID: `39777`
- Source-Dokument: `7071`
- Kurzinfo: Luggage Cart aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Luggage Cart`
- Payload Hash: `a9196095b2546352c02c926dc1491a7f86c99faa3d90fda1f54f5984505e0477`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.184274+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_doorman_luggage_cart_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_luggage_cart_desc",
    "Main": {
      "Props": [
        {
          "Key": "CartDamage",
          "Name": "Cart Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.75
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "ability_doorman_luggage_cart_t3_note",
    "Main": {
      "Props": [
        {
          "Key": "WallImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0
          },
          "Title": "On Wall Hit",
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Wall Hit",
          "Type": "duration",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 1
  },
  "Key": "ability_doorman_luggage_cart",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Luggage Cart",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "CartDamage": 60
    },
    {
      "AbilityCastRange": 20,
      "DescKey": "ability_doorman_luggage_cart_t2_desc",
      "WallImpactDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 75
      }
    },
    {
      "AbilityCooldown": -15,
      "DescKey": "ability_doorman_luggage_cart_t3_desc",
      "StunDuration": 1.25
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "luggage cart",
      "name": "Luggage Cart",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="fathom_lurkers_ambush" title="Lurker's Ambush" -->

## Lurker's Ambush

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_lurkers_ambush`
- Snapshot ID: `39894`
- Source-Dokument: `7071`
- Kurzinfo: Lurker's Ambush aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lurker's Ambush`
- Payload Hash: `6f2ebce9967c1da67b64639db3d1fb99c644c67e39e1bb5061417721ffa1b8e1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.411601+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Duration": {
    "NonLatchedDurationPct": {
      "Name": null,
      "Type": "duration",
      "Value": 50
    }
  },
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_lurkers_ambush_passive_desc",
    "Main": {
      "Props": [
        {
          "Key": "NotSeenByEnemiesRegen",
          "Name": "Max Health Regen",
          "Type": "healing",
          "Value": 3
        },
        {
          "Key": "InvisFadeToDuration",
          "Name": "Fade Time",
          "Value": 1.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "fathom_lurkers_ambush_active_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "EnemySlowPct",
          "Name": "Enemy Move Speed",
          "Type": "slow",
          "Value": 60
        },
        {
          "Key": "DebuffMaxDuration",
          "Name": "Max Slow Duration",
          "Type": "duration",
          "Value": 3.5
        }
      ]
    }
  },
  "Key": "fathom_lurkers_ambush",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5
    }
  },
  "Name": "Lurker's Ambush",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ChannelTimeForMaxDebuff": {
      "Name": null,
      "Value": 1.5
    },
    "DebuffMinDuration": {
      "Name": null,
      "Value": 1.0
    },
    "InitialHeight": {
      "Name": null,
      "Value": 350
    },
    "RevealOnDamageDuration": {
      "Name": null,
      "Value": 0.5
    },
    "RevealOnSpottedDuration": {
      "Name": null,
      "Value": 3
    },
    "StandStillMinTime": {
      "Name": null,
      "Value": 0.5
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Range": {
    "SpottedRadius": {
      "Name": "Spot Radius",
      "Type": "distance",
      "Value": 999
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "DebuffMaxDuration": 1
    },
    {
      "NotSeenByEnemiesRegen": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "lurker's ambush",
      "name": "Lurker's Ambush",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_transformation" title="Lycan Curse" -->

## Lycan Curse

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_transformation`
- Snapshot ID: `39942`
- Source-Dokument: `7071`
- Kurzinfo: Lycan Curse aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lycan Curse`
- Payload Hash: `100aadd601476c31eee64c67b32d93859877e0043402cedbea1c0081b2086df8`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.503156+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 80
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 15
  },
  "DescKey": "ability_werewolf_transformation_desc",
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 0
      },
      {
        "Key": "MissingHealthPercentHeal",
        "Name": "Missing Health as Healing",
        "Type": "healing",
        "Value": 30
      },
      {
        "Key": "KillDurationBonus",
        "Name": "Kill Duration Bonus",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "HealAmount",
        "Name": "Heal Amount",
        "Scale": {
          "Type": "power_increase",
          "Value": 1.0
        },
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "Stamina",
        "Name": "Stamina",
        "Type": "move_speed",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_transformation_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusHealth",
          "Name": "Bonus Health",
          "Scale": {
            "Type": "power_increase",
            "Value": 15.0
          },
          "Type": "health",
          "Value": 125
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 1.5
        },
        {
          "Key": "BonusSprintSpeed",
          "Name": "Sprint Speed",
          "Type": "move_speed",
          "Value": 0
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": 0.45
          },
          "Type": "fire_rate",
          "Value": 60
        }
      ]
    }
  },
  "Key": "ability_werewolf_transformation",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 7.62
    }
  },
  "Name": "Lycan Curse",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.5
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AutoActivateHealthThreshold": {
      "Name": null,
      "Value": 20
    },
    "BonusDurationOnBullet": {
      "Name": null,
      "Value": 0.15
    },
    "BonusDurationOnHeavyMelee": {
      "Name": null,
      "Value": 1.5
    },
    "BonusDurationOnLightMelee": {
      "Name": null,
      "Value": 0.5
    },
    "BonusDurationPerHealthPercentLost": {
      "Name": null,
      "Value": 0.1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "EndingWarningSoundDuration": {
      "Name": null,
      "Value": 3.0
    },
    "HeadshotResist": {
      "Name": null,
      "Value": -20
    },
    "LowHealthFraction": {
      "Name": null,
      "Value": 30
    },
    "LowHealthRageBonus": {
      "Name": null,
      "Scale": {
        "Type": "power_increase",
        "Value": 1.8
      },
      "Value": 40
    },
    "MaxRage": {
      "Name": null,
      "Scale": {
        "Type": "power_increase",
        "Value": 9.4
      },
      "Value": 100
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 15
    },
    "RagePerDamage": {
      "Name": null,
      "Value": 0.255
    },
    "RagePercentagePerSecondInCombat": {
      "Name": null,
      "Value": 1
    },
    "RagePercentagePerSecondOutOfCombat": {
      "Name": null,
      "Value": -3
    },
    "ReadyDuration": {
      "Name": null,
      "Value": 3
    },
    "StackDuration": {
      "Name": "Stack Duration",
      "Value": 5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BulletResist": 20,
      "DescKey": "ability_werewolf_transformation_t1_desc",
      "TechResist": 20
    },
    {
      "BonusHealth": 200,
      "BonusMoveSpeed": 4,
      "DescKey": "ability_werewolf_transformation_t2_desc"
    },
    {
      "DescKey": "ability_werewolf_transformation_t3_desc",
      "KillCreditWindow": 1.5,
      "KillDurationBonus": 15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "lycan curse",
      "name": "Lycan Curse",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_blood_shards" title="Malice" -->

## Malice

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_blood_shards`
- Snapshot ID: `39809`
- Source-Dokument: `7071`
- Kurzinfo: Malice aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Malice`
- Payload Hash: `5a47dc55404d6e99177ddaea5ec118821032840c20d53f553aed5b912813a029`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.245779+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.12
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 6
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_blood_shards_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "NumBloodShards",
        "Name": "Blood Shards",
        "Value": 3
      },
      {
        "Key": "MaxStacks",
        "Name": "Max Stacks",
        "Value": 5
      }
    ],
    "DescKey": "ability_blood_shards_desc",
    "Main": {
      "Props": [
        {
          "Key": "HealthToDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.558
          },
          "Type": "tech_damage",
          "Value": 23.0
        },
        {
          "Key": "SelfDamagePct",
          "Name": "Health Cost",
          "Type": "tech_damage",
          "Value": 9
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 9
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "VulnerabilityPerStack",
          "Name": "Damage Amplification",
          "Title": "Effect Per Stack:",
          "Type": "damage",
          "Value": 8
        },
        {
          "Key": "MoveSpeedPenaltyPerStack",
          "Name": "Movement Slow",
          "Title": "Effect Per Stack:",
          "Type": "slow",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_blood_shards",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Malice",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SpreadAngleDegrees": {
      "Name": null,
      "Value": 6
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "DescKey": "ability_blood_shards_t2_desc",
      "HealthToDamage": 25.2,
      "NumBloodShards": 4,
      "SpreadAngleDegrees": 22
    },
    {
      "DescKey": "ability_blood_shards_t3_desc",
      "VulnerabilityPerStack": 7
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "malice",
      "name": "Malice",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_maulingleap" title="Mauling Leap" -->

## Mauling Leap

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_maulingleap`
- Snapshot ID: `39940`
- Source-Dokument: `7071`
- Kurzinfo: Mauling Leap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mauling Leap`
- Payload Hash: `d5a7cc3e5420dfb6564ed9d30897fd84c80e9185442870cfe7f7a9748f128e5d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.499712+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 18.1
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.55
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_maulingleap_desc",
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "DescKey": "ability_werewolf_maulingleap_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.5
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.15
          },
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "BulletArmorReduction",
          "Name": "Bullet Resist",
          "Title": "On Hit:",
          "Type": "bullet_armor_down",
          "Value": -8
        }
      ]
    }
  },
  "Key": "ability_werewolf_maulingleap",
  "Name": "Mauling Leap",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AllowRamMultiple": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "LeapForwardOffset": {
      "Name": null,
      "Value": 0.3
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "WorldImpactRadius": {
      "Name": null,
      "Value": 25
    }
  },
  "Range": {
    "LeapMultiHitRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 1.5
    },
    "LeapRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 2.8
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DPS": 10
    },
    {
      "AbilityCooldown": -9
    },
    {
      "BulletArmorReduction": -12
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "mauling leap",
      "name": "Mauling Leap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_mobile_resupply" title="Medicinal Specter" -->

## Medicinal Specter

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_mobile_resupply`
- Snapshot ID: `39796`
- Source-Dokument: `7071`
- Kurzinfo: Medicinal Specter aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Medicinal Specter`
- Payload Hash: `43266a106dfc574ea3fe406b96c165aa66bf63eb96448df03242bf85faa0c024`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.221935+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.5
  },
  "DescKey": "citadel_ability_mobile_resupply_desc",
  "Health": {
    "TurretHealMult": {
      "Name": null,
      "Type": "healing",
      "Value": 1.0
    }
  },
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaCooldownReduction",
        "Name": "Stamina Recovery",
        "Type": "stamina_recovery",
        "Value": 0
      },
      {
        "Key": "MaxHealthRegenPct",
        "Name": "Max Health Regen",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_mobile_resupply_desc",
    "Main": {
      "Props": [
        {
          "Key": "ExternalBonusHealthRegen",
          "Name": "Health Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "healing",
          "Value": 25
        },
        {
          "Key": "HealRadius",
          "Name": "Heal Radius",
          "Type": "distance",
          "Value": 6
        },
        {
          "Key": "AuraFireRateBonus",
          "Name": "Unknown(AuraFireRateBonus)",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_mobile_resupply",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Medicinal Specter",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HealInterval": {
      "Name": "Heal Interval",
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_mobile_resupply_t1_desc",
      "SpiritResist": 40
    },
    {
      "AbilityCooldown": -20.0,
      "DescKey": "citadel_ability_mobile_resupply_t2_desc",
      "StaminaCooldownReduction": 100.0
    },
    {
      "AbilityDuration": 1.5,
      "DescKey": "citadel_ability_mobile_resupply_t3_desc",
      "HealRadius": 3,
      "MaxHealthRegenPct": 2.0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "medicinal specter",
      "name": "Medicinal Specter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_shieldedsentry" title="Mini Turret" -->

## Mini Turret

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shieldedsentry`
- Snapshot ID: `39795`
- Source-Dokument: `7071`
- Kurzinfo: Mini Turret aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mini Turret`
- Payload Hash: `026c4ce50fdd33fd99dd1f55c7cfe4ebf79a6ad7b96ee15004eeb77d2ebc8e11`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.219551+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3
  },
  "DescKey": "citadel_ability_shieldedsentry_desc",
  "Duration": {
    "TurretDeployTime": {
      "Name": "Deploy Time",
      "Type": "duration",
      "Value": 0.25
    }
  },
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "TurretLifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 35
      }
    ],
    "DescKey": "citadel_ability_shieldedsentry_desc",
    "Main": {
      "Props": [
        {
          "Key": "TurretDPS",
          "Name": "Turret DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 0.42
          },
          "Type": "tech_damage",
          "Value": 24
        },
        {
          "Key": "TurretBaseHealth",
          "Name": "Turret Health",
          "Scale": {
            "Type": "power_increase",
            "Value": 7.8
          },
          "Type": "health",
          "Value": 90
        },
        {
          "Key": "TurretAttackRange",
          "Name": "Attack Range",
          "Type": "distance",
          "Value": 30
        }
      ]
    }
  },
  "Key": "citadel_ability_shieldedsentry",
  "Name": "Mini Turret",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AttackConeAngle": {
      "Name": null,
      "Value": 10
    },
    "AttackSpeedMult": {
      "Name": null,
      "Value": 100
    },
    "BossDamagePercentIncoming": {
      "Name": null,
      "Value": 50
    },
    "BossDamagePercentOutgoing": {
      "Name": null,
      "Value": 30
    },
    "DecayingResist": {
      "Name": null,
      "Value": 80
    },
    "DecayingResistDuration": {
      "Name": null,
      "Value": 6
    },
    "MeleeResist": {
      "Name": null,
      "Value": 35
    },
    "ModelScale": {
      "Name": null,
      "Value": 0.8
    },
    "NonHeroDamagePercentOutgoing": {
      "Name": null,
      "Value": 50
    },
    "TechResist": {
      "Name": "Spirit Resist",
      "Value": 35
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "TrackingSpeed": {
      "Name": null,
      "Value": 430
    },
    "TurretAttackDelay": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Range": {
    "TurretAttackFalloffEnd": {
      "Name": null,
      "Type": "distance",
      "Value": 30
    },
    "TurretAttackFalloffStart": {
      "Name": null,
      "Type": "distance",
      "Value": 20
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_shieldedsentry_t1_desc",
      "TurretAttackRange": 10,
      "TurretDPS": 10
    },
    {
      "AbilityCharges": 2
    },
    {
      "AttackSpeedMult": 25,
      "DescKey": "citadel_ability_shieldedsentry_t3_desc",
      "TurretLifetime": 12
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "mini turret",
      "name": "Mini Turret",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_incendiary_projectile" title="Napalm" -->

## Napalm

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_incendiary_projectile`
- Snapshot ID: `39831`
- Source-Dokument: `7071`
- Kurzinfo: Napalm aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Napalm`
- Payload Hash: `8504105b4a29829b0d3030ef27a9b9b484f902e98c8e7830a702de014233ba44`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.287255+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 6
  },
  "DescKey": "ability_incendiary_projectile_desc",
  "HeroKey": "hero_inferno",
  "HeroName": "Infernus",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_incendiary_projectile_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 40.0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 35
        },
        {
          "Key": "SlowDuration",
          "Name": "Slow Duration",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 4
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "IncomingDamagePercentFromCaster",
          "Name": "Damage Taken",
          "Title": "Napalm Effects:",
          "Type": "damage",
          "Value": 16
        },
        {
          "Key": "LifestealPercentHero",
          "Name": "Lifesteal",
          "Title": "Napalm Effects:",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "HealAmpReceivePenaltyPercent",
          "Name": "Healing Reduction",
          "Title": "Napalm Effects:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_incendiary_projectile",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 18
    }
  },
  "Name": "Napalm",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GrowthPerMeter": {
      "Name": null,
      "Value": 0.5
    },
    "HeightOffGround": {
      "Name": null,
      "Value": 50
    },
    "InitialWidth": {
      "Name": null,
      "Value": 1
    },
    "ParticleRadiusMultiplier": {
      "Name": null,
      "Value": 1.15
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "DescKey": "ability_incendiary_projectile_t2_desc",
      "LifestealPercentHero": 15
    },
    {
      "DescKey": "ability_incendiary_projectile_t3_desc",
      "HealAmpReceivePenaltyPercent": -33,
      "HealAmpRegenPenaltyPercent": -33,
      "IncomingDamagePercentFromCaster": 17
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "napalm",
      "name": "Napalm",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_familiar_ability01" title="Naptime" -->

## Naptime

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability01`
- Snapshot ID: `39790`
- Source-Dokument: `7071`
- Kurzinfo: Naptime aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Naptime`
- Payload Hash: `c91589175bbf076f6eaf528cda20189c04667c4a993b430c383261f075e405c7`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.209228+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.18
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 24
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Scale": {
      "Type": "duration",
      "Value": 0.0
    },
    "Type": "cast",
    "Value": 1.9
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 200.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_familiar_ability01_desc",
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 19.0
      },
      {
        "Key": "SleepDuration",
        "Name": "Sleep Duration",
        "Type": "duration",
        "Value": 4.0
      },
      {
        "Key": "MinSleepTime",
        "Type": "duration",
        "Value": 0.5
      },
      {
        "Key": "DamageResistPctWhileChanneling",
        "Value": 30
      }
    ],
    "DescKey": "ability_familiar_ability01_desc",
    "Main": {
      "Props": [
        {
          "Key": "AwakeDamage",
          "Name": "Wake Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 120
        },
        {
          "Key": "SleepDamageThreshold",
          "Name": "Sleep Damage Threshold",
          "Scale": {
            "Type": "power_increase",
            "Value": 3.1
          },
          "Type": "damage",
          "Value": 100
        },
        {
          "Key": "MoveSpeedAndDashSlowPct",
          "Name": "Move/Dash Slow",
          "Type": "slow",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_familiar_ability01",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "SleepMoveSpeed": {
      "Name": "Sleep Movespeed",
      "Type": "move_speed",
      "Value": 1.5
    }
  },
  "Name": "Naptime",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "Height": {
      "Name": null,
      "Value": 20
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 19.0
  },
  "Slot": "4",
  "Upgrades": [
    {
      "ConsumeStaminaOnWake": 1,
      "DescKey": "ability_familiar_ability01_t1_desc",
      "NoStaminaRegenDuringSleep": 1
    },
    {
      "DescKey": "ability_familiar_ability01_t2_desc",
      "Radius": 3,
      "SleepDuration": 0.75
    },
    {
      "AbilityCooldown": -55,
      "DamageResistPctWhileChanneling": 50,
      "DescKey": "ability_familiar_ability01_t3_desc",
      "UnstoppableWhileChanneling": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "naptime",
      "name": "Naptime",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_vampirebat_batswarm" title="Nox Nostra" -->

## Nox Nostra

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batswarm`
- Snapshot ID: `39918`
- Source-Dokument: `7071`
- Kurzinfo: Nox Nostra aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Nox Nostra`
- Payload Hash: `ecb58b6bf2effa02cb55e78d39cb37415f839279e77f4ea753ef740c54f24a03`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.457433+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cast": {
    "BonusBatsMax": {
      "Name": "Max Additional Bats",
      "Type": "cast",
      "Value": 50
    }
  },
  "Cooldown": {
    "TimeToGainLockonStack": {
      "Name": null,
      "Type": "cooldown",
      "Value": 0.01
    }
  },
  "DescKey": "ability_vampirebat_batswarm_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBatsPerProc",
        "Name": "Additional Bats Per Love Bite",
        "Type": "cast",
        "Value": 2
      },
      {
        "Key": "BatCount",
        "Name": "Total Bats Released",
        "Type": "cast",
        "Value": 75
      }
    ],
    "DescKey": "ability_vampirebat_batswarm_desc",
    "Main": {
      "Props": [
        {
          "Key": "BatPerSecond",
          "Name": "Bats per Second",
          "Value": 30
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.094
          },
          "Type": "tech_damage",
          "Value": 4.6
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "StatusEffect": "Silence",
          "Title": "On Hit:",
          "Value": 1.25
        },
        {
          "Key": "CurrentHealthPercent",
          "Name": "Current Health",
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_vampirebat_batswarm",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Nox Nostra",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 12
    },
    "AirDrag": {
      "Name": null,
      "Value": 12
    },
    "BatCountPerWave": {
      "Name": null,
      "Value": 1
    },
    "BatEffectiveness": {
      "Name": null,
      "Value": 0.2
    },
    "BatSpawnRadius": {
      "Name": null,
      "Value": 1.5
    },
    "BatSpawnRandomAngle": {
      "Name": null,
      "Value": 0.15
    },
    "BatSpawnRandomVelocity": {
      "Name": null,
      "Value": 300
    },
    "CurrentHealthDamageCapToBosses": {
      "Name": null,
      "Value": 20
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "GroundAccelerationPercentage": {
      "Name": null,
      "Value": -80
    },
    "GroundFrictioNpercentage": {
      "Name": null,
      "Value": -80
    },
    "JumpCeilingCheckDistance": {
      "Name": null,
      "Value": 11
    },
    "JumpPitch": {
      "Name": null,
      "Value": -60
    },
    "JumpSpeed": {
      "Name": null,
      "Value": 17
    },
    "MaxBatTargets": {
      "Name": null,
      "Value": 2
    },
    "MaxLockonStacks": {
      "Name": null,
      "Value": 1
    },
    "NotInConeLosesLock": {
      "Name": null,
      "Value": 1
    },
    "StacksCanDecay": {
      "Name": null,
      "Value": 1
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 20
    },
    "TimeToLoseLockonStack": {
      "Name": null,
      "Value": 0.3
    },
    "VerticalDrag": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "LockonConeAngle": {
      "Name": null,
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 1.9
    },
    {
      "AbilityCooldown": -45
    },
    {
      "CurrentHealthPercent": 0.5,
      "DescKey": "ability_vampirebat_batswarm_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "nox nostra",
      "name": "Nox Nostra",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_wrecker_garbage_suck" title="Overload" -->

## Overload

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_garbage_suck`
- Snapshot ID: `39950`
- Source-Dokument: `7071`
- Kurzinfo: Overload aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Overload`
- Payload Hash: `5def247d3a3ce91e974080b6050c61977a21c6d91f836986b1f1befc8ecde4af`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.518366+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 130
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "DPS": {
      "Name": "Damage Per Second",
      "Scale": {
        "Type": "spirit",
        "Value": 0.194988
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "DescKey": "citadel_ability_wrecker_garbage_suck_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "GarbageRadius",
        "Name": "Overload Radius",
        "Type": "distance",
        "Value": 12
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 50
      }
    ],
    "DescKey": "citadel_ability_wrecker_garbage_suck_desc",
    "Main": {
      "Props": [
        {
          "Key": "BaseDamage",
          "Name": "Base Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "DamagePerSecond",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 2.604
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "Type": "cast",
          "Value": 3
        }
      ]
    }
  },
  "Key": "citadel_ability_wrecker_garbage_suck",
  "Name": "Overload",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 800
    },
    "Speed": {
      "Name": null,
      "Value": 5.08
    },
    "TickRate": {
      "Name": null,
      "Value": 1
    },
    "TossAngle": {
      "Name": null,
      "Value": 45
    },
    "TossSpeed": {
      "Name": null,
      "Value": 8.89
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "GarbageRadius": 2
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BaseDamage": 100,
      "DamagePerSecond": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "overload",
      "name": "Overload",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_frank_shocktarget2" title="Pain Battery" -->

## Pain Battery

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_shocktarget2`
- Snapshot ID: `39803`
- Source-Dokument: `7071`
- Kurzinfo: Pain Battery aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pain Battery`
- Payload Hash: `b316de43cc4b7efc2beedfe064034f4710ccf403b25b7c0c65c0da6611fbe790`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.234923+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 28
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 2
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_frank_shocktarget2_desc",
  "Health": {
    "HealOnHit": {
      "Name": null,
      "Scale": {
        "Type": "healing",
        "Value": 1.0
      },
      "Type": "healing",
      "Value": 0
    }
  },
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_frank_shocktarget2_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hero Hit:",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "MissingHealthPercentHeal",
          "Name": "Missing Health as Healing",
          "Scale": {
            "Type": "healing",
            "Value": 1.0
          },
          "Title": "On Hero Hit:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_frank_shocktarget2",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pain Battery",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BatteryGenerationPercent": {
      "Name": "Battery Generation",
      "Scale": {
        "Type": "cooldown",
        "Value": -1.0
      },
      "Value": 100
    },
    "BoltCount": {
      "Name": null,
      "Value": 7
    },
    "BonusShocksDelay": {
      "Name": null,
      "Value": 0.2
    },
    "SpreadAngle": {
      "Name": null,
      "Value": 40
    },
    "SpreadRandomness": {
      "Name": null,
      "Value": 0.005
    },
    "StoredDamageHealthPercentRequired": {
      "Name": null,
      "Value": 40
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "ability_frank_shocktarget2_t1_desc",
      "SlowDuration": "2s",
      "SlowPercent": 40
    },
    {
      "Damage": 50.0
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
      },
      "DescKey": "ability_frank_shocktarget2_t3_desc",
      "MissingHealthPercentHeal": 15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "pain battery",
      "name": "Pain Battery",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_chrono_swap" title="Paradoxical Swap" -->

## Paradoxical Swap

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_swap`
- Snapshot ID: `39774`
- Source-Dokument: `7071`
- Kurzinfo: Paradoxical Swap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Paradoxical Swap`
- Payload Hash: `9427adcf561265e11b8773382fc265348118f59e27f03b7e7aa0fecb1ac49e8f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.178805+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 110.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_chrono_swap_desc",
  "HeroKey": "hero_chrono",
  "HeroName": "Paradox",
  "Info1": {
    "Alt": [
      {
        "Key": "CombatBarrier",
        "Name": "Barrier",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0
        },
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "BarrierDuration",
        "Name": "Barrier Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "MultiSwap",
        "Name": "Multi Target Radius",
        "Type": "distance",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_chrono_swap_desc",
    "Main": {
      "Props": [
        {
          "Key": "SwapDamage",
          "Name": "Swap Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.1
          },
          "Type": "tech_damage",
          "Value": 150.0
        },
        {
          "Key": "MaxHealthDamage",
          "Name": "Max Health Damage",
          "Type": "tech_damage",
          "Value": 0.0
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_swap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Paradoxical Swap",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DistanceToMaxTime": {
      "Name": null,
      "Value": 30
    },
    "InitialFreezeTime": {
      "Name": null,
      "Value": 0.25
    },
    "InitialHeight": {
      "Name": null,
      "Value": 350
    },
    "MinSwapTime": {
      "Name": null,
      "Value": 0.6
    },
    "SwapTime": {
      "Name": null,
      "Value": 1.0
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BarrierDuration": 8,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 200
      },
      "DescKey": "citadel_ability_chrono_swap_t1_desc"
    },
    {
      "AbilityCastRange": 13,
      "AbilityCooldown": -30,
      "DescKey": "citadel_ability_chrono_swap_t2_desc"
    },
    {
      "DescKey": "citadel_ability_chrono_swap_t3_desc",
      "MaxHealthDamage": 10,
      "MultiSwap": 7
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "paradoxical swap",
      "name": "Paradoxical Swap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_trapper_spidershield" title="Pest Barrier" -->

## Pest Barrier

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spidershield`
- Snapshot ID: `39909`
- Source-Dokument: `7071`
- Kurzinfo: Pest Barrier aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pest Barrier`
- Payload Hash: `a35b93f9c4788b796f388e4ada505add922192c7120a178ac4e7dabe1e7c8822`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.439481+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 45
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_trapper_spidershield_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Value": 30
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Value": 0.5
      }
    ],
    "DescKey": "ability_trapper_spidershield_desc",
    "Main": {
      "Props": [
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 2.046
          },
          "Type": "combat_barrier",
          "Value": 200
        },
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 5
        }
      ]
    }
  },
  "Key": "ability_trapper_spidershield",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pest Barrier",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "CombatBarrier": 200
    },
    {
      "Radius": 5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "pest barrier",
      "name": "Pest Barrier",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_viper_petrifybola" title="Petrifying Bola" -->

## Petrifying Bola

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_petrifybola`
- Snapshot ID: `39926`
- Source-Dokument: `7071`
- Kurzinfo: Petrifying Bola aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Petrifying Bola`
- Payload Hash: `4b5ba015b33cb7226fee5cfcc7a5accf2b049b242495230c89dc71fc4c3c21e6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.473181+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 105
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_viper_petrifybola_desc",
  "Health": {
    "PetrifyDamageBreakThreshold": {
      "Name": "Petrify Damage Block",
      "Type": "health",
      "Value": 200
    }
  },
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 50
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 1.5
      }
    ],
    "DescKey": "ability_viper_petrifybola_desc",
    "Main": {
      "Props": [
        {
          "Key": "PetrifyDuration",
          "Name": "Petrify Duration",
          "StatusEffect": "Petrify",
          "Value": 2.0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Type": "tech_damage",
          "Value": 50.0
        },
        {
          "Key": "PetrifyDamage",
          "Name": "Petrify Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.046
          },
          "Type": "tech_damage",
          "Value": 180
        }
      ]
    }
  },
  "Key": "ability_viper_petrifybola",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Petrifying Bola",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 8
  },
  "Slot": "4",
  "Upgrades": [
    {
      "PetrifyDamage": 49.5
    },
    {
      "AbilityCooldown": -20
    },
    {
      "DescKey": "ability_viper_petrifybola_t3_desc",
      "PetrifyDuration": 1.0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "petrifying bola",
      "name": "Petrifying Bola",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_familiar_ability02" title="Pillow Toss" -->

## Pillow Toss

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability02`
- Snapshot ID: `39787`
- Source-Dokument: `7071`
- Kurzinfo: Pillow Toss aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pillow Toss`
- Payload Hash: `daf5e34b951570fcaa9443c1704c9180a18386a2e6b7a3f54414f84a88b71116`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.203374+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "DescKey": "ability_familiar_ability02_desc",
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [
      {
        "Key": "EffectDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 3.0
      },
      {
        "Key": "CDReduceOnPillowHit",
        "Value": 5
      }
    ],
    "DescKey": "ability_familiar_ability02_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "FadingSlowPercent",
          "Name": "Fading Move Speed",
          "Type": "slow",
          "Value": 45
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Type": "duration",
          "Value": 0.4
        }
      ]
    }
  },
  "Key": "ability_familiar_ability02",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pillow Toss",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "OrbsToFire": {
      "Name": null,
      "Value": 1
    },
    "TossForce": {
      "Name": null,
      "Value": 300
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -7
    },
    {
      "DescKey": "ability_familiar_ability02_t2_desc",
      "FireRateSlow": 35,
      "Radius": 2
    },
    {
      "AbilityCharges": 1,
      "Damage": 100,
      "DescKey": "ability_familiar_ability02_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "pillow toss",
      "name": "Pillow Toss",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bookworm_knightbarrier" title="Plot Armor" -->

## Plot Armor

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightbarrier`
- Snapshot ID: `39768`
- Source-Dokument: `7071`
- Kurzinfo: Plot Armor aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Plot Armor`
- Payload Hash: `1ad0c5b609ed0f2fd309f075eb68f77fdffb04563821e3a6e553e2fd4976ab89`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.167215+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 35
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_bookworm_knightbarrier_desc",
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_bookworm_knightbarrier_desc",
    "Main": {
      "Props": [
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Type": "bullet_armor_up",
          "Value": 125
        },
        {
          "Key": "BaseAttackDamagePercent",
          "Name": "Weapon Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.2
          },
          "Type": "bullet_damage",
          "Value": 25
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_bookworm_knightbarrier",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Plot Armor",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusSpiritDamagePercent": {
      "Name": null,
      "Value": 15
    },
    "BonusTargetRadius": {
      "Name": null,
      "Value": 30
    },
    "PushForce": {
      "Name": null,
      "Value": 900
    }
  },
  "Range": {
    "ShoveRadius": {
      "Name": "Shove Radius",
      "Type": "distance",
      "Value": 6
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusFireRate": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 14
      },
      "DescKey": "ability_bookworm_knightbarrier_t1_desc"
    },
    {
      "AbilityDuration": 2,
      "CombatBarrier": 100,
      "DescKey": "ability_bookworm_knightbarrier_t2_desc"
    },
    {
      "BonusTargets": 2,
      "BonusTargetsBarrierPercentage": 100,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      },
      "DescKey": "ability_bookworm_knightbarrier_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "plot armor",
      "name": "Plot Armor",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_explosive_barrel" title="Powder Keg" -->

## Powder Keg

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_explosive_barrel`
- Snapshot ID: `39748`
- Source-Dokument: `7071`
- Kurzinfo: Powder Keg aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Powder Keg`
- Payload Hash: `55815cbf7e96b8ff8f8839f061ddbd02fc47f7c3592377432e790d00bd0a105e`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.124644+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.125
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7.5
  },
  "DescKey": "ability_explosive_barrel_desc",
  "Duration": {
    "BarrelLifetime": {
      "Name": "Barrel Life Time",
      "Type": "duration",
      "Value": 8
    }
  },
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_explosive_barrel_desc",
    "Main": {
      "Props": [
        {
          "Key": "ArmTime",
          "Name": "Arm Time",
          "Type": "duration",
          "Value": 0.1
        },
        {
          "Key": "BarrelDamage",
          "Name": "Explosion Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.2
          },
          "Type": "tech_damage",
          "Value": 80
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0.4
        }
      ]
    }
  },
  "Key": "ability_explosive_barrel",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Powder Keg",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BarrelHeavyMeleeForceForward": {
      "Name": null,
      "Value": 1800
    },
    "BarrelHeavyMeleeForceUp": {
      "Name": null,
      "Value": 300
    },
    "BarrelLightMeleeForceForward": {
      "Name": null,
      "Value": 1400
    },
    "BarrelLightMeleeForceUp": {
      "Name": null,
      "Value": 300
    },
    "BarrelPitchMax": {
      "Name": null,
      "Value": 90
    },
    "BarrelPitchMin": {
      "Name": null,
      "Value": 2
    },
    "BarrelRollSpeedMoveAir": {
      "Name": null,
      "Value": 10
    },
    "BarrelRollSpeedMoveMin": {
      "Name": null,
      "Value": 20
    },
    "BarrelScale": {
      "Name": null,
      "Value": 1.3
    },
    "MinTimeBeforeDestroy": {
      "Name": null,
      "Value": 0.1
    },
    "TossSpeed": {
      "Name": null,
      "Value": 3.556
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCharges": 1
    },
    {
      "AbilityCooldownBetweenCharge": -5,
      "BarrelDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 80
      },
      "DescKey": "ability_explosive_barrel_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "powder keg",
      "name": "Powder Keg",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_power_slash" title="Power Slash" -->

## Power Slash

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_power_slash`
- Snapshot ID: `39951`
- Source-Dokument: `7071`
- Kurzinfo: Power Slash aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Power Slash`
- Payload Hash: `2f380b4f9986e0e356e72abecc183134f2dc6d528be3bde61b311e3ba4fd7945`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.520432+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.4
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "MediumChargeDamagePct": {
      "Name": "Medium Charge Dmg",
      "Type": "tech_damage",
      "Value": 50
    },
    "ShortChargeDamagePct": {
      "Name": "Short Charge Dmg",
      "Type": "tech_damage",
      "Value": 30
    }
  },
  "DescKey": "citadel_ability_power_slash_desc",
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 60
      }
    ],
    "DescKey": "citadel_ability_power_slash_desc",
    "Main": {
      "Props": [
        {
          "Key": "FullChargeDamage",
          "Name": "Full Charge Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.85
          },
          "Type": "tech_damage",
          "Value": 145
        },
        {
          "Key": "SlashLength",
          "Name": "Slash Length",
          "Type": "distance",
          "Value": 22
        }
      ]
    }
  },
  "Key": "citadel_ability_power_slash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Power Slash",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 5
    },
    "PowerUpStages": {
      "Name": null,
      "Value": 3
    },
    "SlashCollisionRadius": {
      "Name": null,
      "Value": 4
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Type": "",
      "Value": 41
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_power_slash_t1_desc",
      "SlowDuration": 3,
      "SlowPercent": 40
    },
    {
      "AbilityCooldown": -4,
      "DescKey": "citadel_ability_power_slash_t2_desc"
    },
    {
      "DescKey": "citadel_ability_power_slash_t3_desc",
      "FullChargeDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 150
      },
      "SlashLength": 8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "power slash",
      "name": "Power Slash",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_power_surge" title="Power Surge" -->

## Power Surge

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_power_surge`
- Snapshot ID: `39813`
- Source-Dokument: `7071`
- Kurzinfo: Power Surge aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Power Surge`
- Payload Hash: `d3d5c217e67902ef638c1fec46f5be644c17461db11fc3d7ea982417b9fde8e4`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.253821+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 10
  },
  "DescKey": "ability_power_surge_desc",
  "HeroKey": "hero_gigawatt",
  "HeroName": "Seven",
  "Info1": {
    "Alt": [
      {
        "Key": "ChainRadius",
        "Name": "Jump Radius",
        "Type": "distance",
        "Value": 10
      }
    ],
    "DescKey": "ability_power_surge_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePerChain",
          "Name": "Shock Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.14
          },
          "Type": "tech_damage",
          "Value": 10
        },
        {
          "Key": "ChainCount",
          "Name": "Max Jumps",
          "Value": 4
        }
      ]
    }
  },
  "Key": "ability_power_surge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Power Surge",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusPerChain": {
      "Name": "Damage on Jump",
      "Scale": {
        "Type": "spirit",
        "Value": 0.32
      },
      "Value": 10
    },
    "ChainTickRate": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -18.0
    },
    {
      "BonusMoveSpeed": 3,
      "BonusPerChain": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.23
        },
        "Value": 8
      },
      "DamagePerChain": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.23
        },
        "Value": 8
      },
      "DescKey": "ability_power_surge_t2_desc"
    },
    {
      "AbilityDuration": 10,
      "DescKey": "ability_power_surge_t3_desc",
      "TechResistDebuff": -15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "power surge",
      "name": "Power Surge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_projectmind" title="Project Mind" -->

## Project Mind

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_projectmind`
- Snapshot ID: `39944`
- Source-Dokument: `7071`
- Kurzinfo: Project Mind aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Project Mind`
- Payload Hash: `755e247a335ae3271d1e8cb8054bda93d0133421154c04501ec65a731aa482d1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.506967+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.75
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 46.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_projectmind_desc",
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_projectmind_desc",
    "Main": {}
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BarrierDuration",
        "Name": "Barrier Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Title": "Barrier:",
          "Type": "range",
          "Value": 25
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "Barrier:",
          "Type": "bullet_armor_up",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_projectmind",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5.1
    }
  },
  "Name": "Project Mind",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "TrailInterval": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCastRange": 15
    },
    {
      "BarrierDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.7
        },
        "Value": 300
      },
      "DescKey": "citadel_ability_projectmind_t2_desc"
    },
    {
      "AbilityCooldown": -32.0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "project mind",
      "name": "Project Mind",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="viscous_telepunch" title="Puddle Punch" -->

## Puddle Punch

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_telepunch`
- Snapshot ID: `39929`
- Source-Dokument: `7071`
- Kurzinfo: Puddle Punch aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Puddle Punch`
- Payload Hash: `657e79658e57fc26483a22206a25023dc1c460155a4ab4a4a1014f470cdd7a39`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.479030+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 24.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 1.7
  },
  "DescKey": "viscous_telepunch_desc",
  "HeroKey": "hero_viscous",
  "HeroName": "Viscous",
  "Info1": {
    "Alt": [
      {
        "Key": "ImpactDuration",
        "Name": "Slow Duration",
        "Value": 4
      },
      {
        "Key": "LifeStealPercentOnHit",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "viscous_telepunch_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.6
          },
          "Type": "melee_damage",
          "Value": 20.0
        },
        {
          "Key": "DamageHeavyMelee",
          "Name": "Damage",
          "Scale": {
            "Type": "heavy_melee",
            "Value": 0
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0.6
        }
      ]
    }
  },
  "Key": "viscous_telepunch",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Puddle Punch",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FriendlyImpactDuration": {
      "Name": null,
      "Value": 2
    },
    "PunchFriendlyAirControl": {
      "Name": null,
      "Value": 30
    },
    "PunchHalfHeight": {
      "Name": null,
      "Value": 5.5
    },
    "PunchRollSlow": {
      "Name": null,
      "Value": -40
    },
    "PunchRollSlowDuration": {
      "Name": null,
      "Value": 1
    },
    "TossGroundSideRatio": {
      "Name": null,
      "Value": 0.7
    },
    "TossSpeed": {
      "Name": null,
      "Value": 625
    },
    "TossSpeedUpWall": {
      "Name": null,
      "Value": 500
    },
    "TossSpeedWall": {
      "Name": null,
      "Value": 750
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "Damage": 20,
      "DescKey": "viscous_telepunch_t1_desc"
    },
    {
      "DescKey": "viscous_telepunch_t2_desc",
      "LifeStealPercentOnHit": 60,
      "Radius": 1.5
    },
    {
      "AbilityCooldown": -14,
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": -40
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.6
        },
        "Value": 40
      },
      "DescKey": "viscous_telepunch_t3_desc",
      "UseHeavyMelee": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "puddle punch",
      "name": "Puddle Punch",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_chrono_pulse_grenade" title="Pulse Grenade" -->

## Pulse Grenade

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_pulse_grenade`
- Snapshot ID: `39819`
- Source-Dokument: `7071`
- Kurzinfo: Pulse Grenade aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Pulse Grenade`
- Payload Hash: `0044e221cef0b7739db5f3b47d3221498f1f6bb85088900f9f4367b73025a1cd`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.264772+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3.2
  },
  "DescKey": "citadel_ability_chrono_pulse_grenade_desc",
  "HeroKey": "hero_gunslinger",
  "HeroName": "Gunslinger",
  "Info1": {
    "Alt": [
      {
        "Key": "RadiusIncreasePerPulse",
        "Name": "Radius Per Pulse",
        "Type": "distance",
        "Value": 1
      },
      {
        "Key": "PulseInterval",
        "Name": "Pulse Interval",
        "Type": "duration",
        "Value": 0.8
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 8.0
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 20
      },
      {
        "Key": "MovementSlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0.2
      }
    ],
    "DescKey": "citadel_ability_chrono_pulse_grenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "PulseDamage",
          "Name": "Pulse Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "tech_damage",
          "Value": 35
        },
        {
          "Key": "DamageAmplificationPerStack",
          "Name": "Bonus Damage per Stack",
          "Type": "damage",
          "Value": 4
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_pulse_grenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Pulse Grenade",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -12
    },
    {
      "DescKey": "citadel_ability_chrono_pulse_grenade_t2_desc",
      "PulseDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 20
      }
    },
    {
      "AbilityDuration": 1.6,
      "DamageAmplificationPerStack": 4,
      "DescKey": "citadel_ability_chrono_pulse_grenade_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "pulse grenade",
      "name": "Pulse Grenade",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_void_sphere" title="Quantum Entanglement" -->

## Quantum Entanglement

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_void_sphere`
- Snapshot ID: `39784`
- Source-Dokument: `7071`
- Kurzinfo: Quantum Entanglement aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Quantum Entanglement`
- Payload Hash: `5100d21ac6cd51f32598301575cfeb5449882459d36710ad4843b194e484eb75`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.198150+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 1.4
  },
  "DescKey": "citadel_ability_void_sphere_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaRestore",
        "Name": "Stamina Restored",
        "Value": 1
      },
      {
        "Key": "AllyDistance",
        "Name": "Ally Distance",
        "Type": "distance",
        "Value": 13
      }
    ],
    "DescKey": "citadel_ability_void_sphere_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Type": "range",
          "Value": 10
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 1.4
        }
      ]
    }
  },
  "Key": "citadel_ability_void_sphere",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Quantum Entanglement",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TrailInterval": {
      "Name": null,
      "Value": 0.01
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCastRange": 6
    },
    {
      "AbilityCooldown": -6
    },
    {
      "ChargeReplenish": 1,
      "DescKey": "citadel_ability_void_sphere_t3_desc",
      "ReduceDebuffs": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "quantum entanglement",
      "name": "Quantum Entanglement",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_magician_animalhexarea" title="Rabbit Hex" -->

## Rabbit Hex

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_animalhexarea`
- Snapshot ID: `39849`
- Source-Dokument: `7071`
- Kurzinfo: Rabbit Hex aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rabbit Hex`
- Payload Hash: `72177554d14fc09c1d5e795e0defcc50073c70b2ae3e4baf90cd1c7b565da230`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.324438+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 24
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_magician_animalhexarea_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [
      {
        "Key": "MoveSpeedBonusPct",
        "Name": "Move Speed bonus",
        "Type": "move_speed",
        "Value": 36
      },
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 6.5
      }
    ],
    "DescKey": "ability_magician_animalhexarea_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "HexDuration",
          "Name": "Hex Duration",
          "Value": 2
        },
        {
          "Key": "DamageAmpPercentage",
          "Name": "Damage Amp",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0558
          },
          "Type": "damage",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_magician_animalhexarea",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "HexMoveSpeedLimit": {
      "Name": "Movement Speed Limit",
      "Type": "move_speed",
      "Value": 6
    }
  },
  "Name": "Rabbit Hex",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDampingDuration": {
      "Name": null,
      "Value": 1
    },
    "DetonationDelay": {
      "Name": null,
      "Value": 0.9
    },
    "SelfBumpImpulse": {
      "Name": null,
      "Value": 500
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "HexDuration": 1
    },
    {
      "DamageAmpPercentage": 7,
      "DescKey": "ability_magician_animalhexarea_t3_desc",
      "Radius": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "rabbit hex",
      "name": "Rabbit Hex",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_unicorn_luminousstrike" title="Radiant Daggers" -->

## Radiant Daggers

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_luminousstrike`
- Snapshot ID: `39913`
- Source-Dokument: `7071`
- Kurzinfo: Radiant Daggers aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Radiant Daggers`
- Payload Hash: `e96919b0a17adadf5eb37f926bd082b7d2be2fa28e218095c41b55c4a4270e44`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.447779+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "DescKey": "ability_unicorn_luminousstrike_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "ExplosionRadius",
        "Name": "Explosion Radius",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "PreExplosionDuration",
        "Type": "duration",
        "Value": 1.4
      },
      {
        "Key": "BuffMaxStacks",
        "Type": "cast",
        "Value": 6
      },
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 30
      }
    ],
    "DescKey": "ability_unicorn_luminousstrike_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.63
          },
          "Type": "tech_damage",
          "Value": 55
        },
        {
          "Key": "MagicIncreasePerStack",
          "Name": "Spirit Amp per Stack",
          "Type": "tech_armor_down",
          "Value": 8
        },
        {
          "Key": "FireRatePerStack",
          "Name": "Fire Rate per Stack",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_unicorn_luminousstrike",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Radiant Daggers",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuffDelay": {
      "Name": null,
      "Value": 0.75
    },
    "ClimbHeight": {
      "Name": null,
      "Value": 50.0
    },
    "ExplosionInterval": {
      "Name": "Beam Interval",
      "Value": 0.7
    },
    "PostExplosionDuration": {
      "Name": null,
      "Value": 0.8
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "AbilityCooldown": -22,
      "DescKey": "ability_unicorn_luminousstrike_t2_desc",
      "ImpactDamage": 80
    },
    {
      "FireRatePerStack": 9,
      "MagicIncreasePerStack": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "radiant daggers",
      "name": "Radiant Daggers",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_power_jump" title="Rain of Arrows" -->

## Rain of Arrows

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_power_jump`
- Snapshot ID: `39868`
- Source-Dokument: `7071`
- Kurzinfo: Rain of Arrows aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rain of Arrows`
- Payload Hash: `e440082d4920cec2baa78a43afeb3cc3d891482f4162ada45b392ba9e7f6eb6b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.361280+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "ability_power_jump_desc",
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BulletLifestealPercent",
        "Name": "Bullet Lifesteal",
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "TechLifestealPercent",
        "Name": "Spirit Lifesteal",
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "EvasionPercent",
        "Name": "Bullet Evasion",
        "Value": 0
      }
    ],
    "DescKey": "ability_power_jump_desc",
    "Main": {
      "Props": [
        {
          "Key": "BulletSplitShot",
          "Name": "Weapon Multishot",
          "Type": "fire_rate",
          "Value": 5
        },
        {
          "Key": "WeaponDamageBonus",
          "Name": "Weapon Damage",
          "Type": "bullet_damage",
          "Value": 3
        }
      ]
    }
  },
  "Key": "ability_power_jump",
  "Name": "Rain of Arrows",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirMoveIncreasePercent": {
      "Name": "Air Jump/Dash Distance",
      "Value": 25
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 6.64
    },
    "AltJumpSpeed": {
      "Name": null,
      "Value": 12
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.635
    },
    "FxRadius": {
      "Name": null,
      "Value": 4
    },
    "JumpPitch": {
      "Name": null,
      "Value": -60
    },
    "JumpSpeed": {
      "Name": null,
      "Value": 27.5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DescKey": "ability_power_jump_t1_desc",
      "SlowDuration": 1.5,
      "SlowPercent": 30,
      "WeaponDamageBonus": 3
    },
    {
      "AbilityCooldown": -12.0
    },
    {
      "BulletLifestealPercent": 30,
      "DescKey": "ability_power_jump_t3_desc",
      "EvasionPercent": 30,
      "TechLifestealPercent": 30
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "rain of arrows",
      "name": "Rain of Arrows",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_vampirebat_steallife" title="Rake" -->

## Rake

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_steallife`
- Snapshot ID: `39915`
- Source-Dokument: `7071`
- Kurzinfo: Rake aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rake`
- Payload Hash: `61c6dc819c4eb11a235d40f58ad98e03299dba18101955e1f16ab3c7b2f50cf0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.451567+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_vampirebat_steallife_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_vampirebat_steallife_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "stats_count",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "MissingHealthDamagePercentage",
          "Name": "Missing Health Damage",
          "Title": "On Hero Hit:",
          "Type": "tech_damage",
          "Value": 6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "RakeHealPerKill",
          "Name": "Heal Per Kill",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Title": "On Kill:",
          "Type": "healing",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_vampirebat_steallife",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Rake",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 6
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.2
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 3
    },
    "FallingDrag": {
      "Name": null,
      "Value": 20
    },
    "MaxFloatTime": {
      "Name": null,
      "Value": 4.0
    },
    "MiniJumpVelocity": {
      "Name": null,
      "Value": 200
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 60
    },
    "TimeBetweenAttacks": {
      "Name": null,
      "Value": 0.04
    },
    "TrooperExecuteThreshold": {
      "Name": null,
      "Value": 60
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -8,
      "DescKey": "ability_vampirebat_steallife_t2_desc",
      "RakeHealPerKill": 30
    },
    {
      "DescKey": "ability_vampirebat_steallife_t3_desc",
      "MissingHealthDamagePercentage": 7.0,
      "RakeHealPerKill": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.2
        },
        "Value": 0
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "rake",
      "name": "Rake",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bookworm_knightcharge" title="Rallying Charge" -->

## Rallying Charge

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightcharge`
- Snapshot ID: `39770`
- Source-Dokument: `7071`
- Kurzinfo: Rallying Charge aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rallying Charge`
- Payload Hash: `1b548175df0873d562c10aec3b378e5132ef17f82eba5fd18c53b098fd0ce42c`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.171103+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 600
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.7
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 220
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 13
  },
  "DescKey": "ability_bookworm_knightcharge_desc",
  "Duration": {
    "BuffDuration": {
      "Name": "Buff Duration",
      "Type": "duration",
      "Value": 9
    }
  },
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_bookworm_knightcharge_desc",
    "Main": {
      "Props": []
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Title": "On Enemy Hit:",
          "Type": "tech_damage",
          "Value": 125
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Enemy Hit:",
          "Value": 1.0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [
      {
        "Key": "MaxAmp",
        "Type": "damage",
        "Value": 100
      },
      {
        "Key": "MaxAmpDistance",
        "Type": "distance",
        "Value": 250
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "HealAmount",
          "Name": "Heal Amount",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Title": "On Friendly Hit:",
          "Type": "healing",
          "Value": 125
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "On Friendly Hit:",
          "Type": "move_speed",
          "Value": 5
        }
      ]
    }
  },
  "Key": "ability_bookworm_knightcharge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Rallying Charge",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.8
    },
    "AllyHeight": {
      "Name": null,
      "Value": 20
    },
    "AllyRadius": {
      "Name": null,
      "Value": 4
    },
    "CancelCooldownRefundPercentage": {
      "Name": null,
      "Value": 50
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 20
    },
    "GravityAcceleration": {
      "Name": null,
      "Value": -1900
    },
    "GroundStickHeight": {
      "Name": null,
      "Value": 0.05
    },
    "KnightBonusPerWave": {
      "Name": null,
      "Value": -99
    },
    "KnightChargeHeight": {
      "Name": null,
      "Value": 3.5
    },
    "KnightChargeWidth": {
      "Name": null,
      "Value": 1.7
    },
    "KnightCount": {
      "Name": null,
      "Value": 5
    },
    "KnightCountInFirstWave": {
      "Name": null,
      "Value": 5
    },
    "KnightJumpSpeed": {
      "Name": null,
      "Value": 900
    },
    "KnightMaxFallHeight": {
      "Name": null,
      "Value": -35
    },
    "KnightMaxJumpHeight": {
      "Name": null,
      "Value": 30
    },
    "KnightNavForwardDistance": {
      "Name": null,
      "Value": 8
    },
    "KnightNavSearchDistance": {
      "Name": null,
      "Value": 10
    },
    "KnightPositionSpread": {
      "Name": "Knight Spread",
      "Value": 1.8
    },
    "KnightPositionStagger": {
      "Name": null,
      "Value": -4
    },
    "KnightWhiskerLength": {
      "Name": null,
      "Value": 300
    },
    "KnightWhiskerSide": {
      "Name": null,
      "Value": 50
    },
    "KnightWhiskerStrength": {
      "Name": null,
      "Value": 0.2
    },
    "TargetFindingDelay": {
      "Name": null,
      "Value": 0.04
    },
    "TossBackSpeed": {
      "Name": null,
      "Value": 100
    },
    "TossUpSpeed": {
      "Name": null,
      "Value": 600
    },
    "WaveCount": {
      "Name": null,
      "Value": 2
    },
    "WavePositionStagger": {
      "Name": null,
      "Value": -15
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "HealAmount": 150
    },
    {
      "AbilityCooldown": -45,
      "DescKey": "ability_bookworm_knightcharge_t2_desc",
      "KnightCount": 4,
      "KnightCountInFirstWave": 4
    },
    {
      "Damage": 160.0,
      "DescKey": "ability_bookworm_knightcharge_t3_desc",
      "MaxAmp": 70,
      "StunDuration": 0.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "rallying charge",
      "name": "Rallying Charge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="fathom_reefdweller_harpoon" title="Reefdweller Harpoon" -->

## Reefdweller Harpoon

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_reefdweller_harpoon`
- Snapshot ID: `39893`
- Source-Dokument: `7071`
- Kurzinfo: Reefdweller Harpoon aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Reefdweller Harpoon`
- Payload Hash: `cdbe4a52f81925731127d46230d7230c793629447394f095d238d80f2598dec6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.409259+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "DescKey": "fathom_reefdweller_harpoon_desc",
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_reefdweller_harpoon_desc",
    "Main": {
      "Props": []
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {}
  },
  "Key": "fathom_reefdweller_harpoon",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Reefdweller Harpoon",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ReelSpeed": {
      "Name": null,
      "Value": 1500
    },
    "WallLatchIdealDist": {
      "Name": null,
      "Value": 5
    },
    "WallLatchSettleDist": {
      "Name": null,
      "Value": 40
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "AbilityCastRange": 5
    },
    {
      "BonusFireRate": 30,
      "DescKey": "fathom_reefdweller_harpoon_t3_desc",
      "DetachBuffDuration": 7
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "reefdweller harpoon",
      "name": "Reefdweller Harpoon",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_nikuman" title="Rejuvenating Aurora" -->

## Rejuvenating Aurora

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_nikuman`
- Snapshot ID: `39785`
- Source-Dokument: `7071`
- Kurzinfo: Rejuvenating Aurora aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rejuvenating Aurora`
- Payload Hash: `37808580b7054a75c3af8501120fdcb362b7fe9e7f01ade23571af2b6c4b6fb8`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.199907+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 48.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_nikuman_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "ShareWithFriendsRadius",
        "Name": "Friendly Heal Radius",
        "Type": "distance",
        "Value": 8
      }
    ],
    "DescKey": "citadel_ability_nikuman_desc",
    "Main": {
      "Props": [
        {
          "Key": "HealingPerSecond",
          "Name": "Health Restored",
          "Scale": {
            "Type": "spirit",
            "Value": 0.4
          },
          "Type": "healing",
          "Value": 30
        },
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "Type": "cast",
          "Value": 5
        }
      ]
    }
  },
  "Key": "citadel_ability_nikuman",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Rejuvenating Aurora",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraLingerDuration": {
      "Name": null,
      "Value": 1.0
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_nikuman_t1_desc",
      "MovementSpeedBonus": 4,
      "MovementSpeedBonusDuration": 8
    },
    {
      "AbilityChannelTime": 1.0,
      "AbilityCooldown": -20.0,
      "DescKey": "citadel_ability_nikuman_t2_desc"
    },
    {
      "DescKey": "citadel_ability_nikuman_t3_desc",
      "HealMaxHealthPercent": 2.5,
      "NoChannel": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "rejuvenating aurora",
      "name": "Rejuvenating Aurora",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="drifter_blood_blast" title="Rend" -->

## Rend

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `drifter_blood_blast`
- Snapshot ID: `39779`
- Source-Dokument: `7071`
- Kurzinfo: Rend aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rend`
- Payload Hash: `90c0cb64ed24190f46cfd31b84b062db02c72fdb4ee666e3ef5e10a9e0825c73`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.188583+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.4
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 16
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "drifter_blood_blast_desc",
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "RangeForBonusDamage",
        "Name": "Close Range",
        "Type": "distance",
        "Value": 8
      },
      {
        "Key": "LifestealDuration",
        "Name": "Lifesteal Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "drifter_blood_blast_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.8
          },
          "Type": "tech_damage",
          "Value": 40.0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.2
          },
          "Title": "On Close Range",
          "Type": "melee_damage",
          "Value": 0.0
        },
        {
          "Key": "DamageHeavyMelee",
          "Name": "Damage",
          "Scale": {
            "Type": "heavy_melee",
            "Value": 0
          },
          "Title": "On Close Range",
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "BulletLifestealPercentHero",
          "Name": "Bullet Lifesteal",
          "Title": "On Close Range",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "StatusEffect": "Silence",
          "Title": "On Close Range",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "drifter_blood_blast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Rend",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 30
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "ExtraSweepConeAngle": {
      "Name": null,
      "Value": 60
    },
    "ExtraSweepOffsetBehindCaster": {
      "Name": null,
      "Value": 80
    },
    "ExtraSweepRange": {
      "Name": null,
      "Value": 3
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Type": "distance",
      "Value": 50
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BonusDamage": 40
    },
    {
      "AbilityCooldown": -8.0,
      "DescKey": "drifter_blood_blast_t2_desc"
    },
    {
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0
      },
      "DamageHeavyMelee": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.55
        },
        "Value": 0
      },
      "DebuffDuration": 2.3,
      "DescKey": "drifter_blood_blast_t3_desc",
      "UseHeavyMelee": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "rend",
      "name": "Rend",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_nano_shadow_pulse" title="Return to Shadows" -->

## Return to Shadows

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_shadow_pulse`
- Snapshot ID: `39858`
- Source-Dokument: `7071`
- Kurzinfo: Return to Shadows aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Return to Shadows`
- Payload Hash: `8ea0e69c3156e327ba9ceb1c5aa0e99c7473f7d2f13232f63d8900d9b606a432`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.342062+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 115
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_shadow_pulse_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 3
      },
      {
        "Key": "OutgoingDamagePercent",
        "Name": "Damage",
        "Type": "damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_nano_shadow_pulse_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.609336
          },
          "Type": "tech_damage",
          "Value": 150.0
        },
        {
          "Key": "BonusMoveSpeedPercent",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 20
        }
      ]
    }
  },
  "Key": "ability_nano_shadow_pulse",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Return to Shadows",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 4
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 100
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.254
    },
    "ZAcceleration": {
      "Name": null,
      "Value": 800
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 7.5
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "BonusMoveSpeedPercent": 20,
      "Damage": 75,
      "DescKey": "ability_nano_shadow_pulse_t2_desc"
    },
    {
      "DescKey": "ability_nano_shadow_pulse_t3_desc",
      "HealAmount": 450,
      "RefundCooldowns": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "return to shadows",
      "name": "Return to Shadows",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="operative_revelation" title="Revelation" -->

## Revelation

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `operative_revelation`
- Snapshot ID: `39866`
- Source-Dokument: `7071`
- Kurzinfo: Revelation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Revelation`
- Payload Hash: `01cd76b71e9367c0aa7031725732a35a6204eb3ad6c212094f9838e7035158a7`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.357234+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 90.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "operative_revelation_desc",
  "HeroKey": "hero_operative",
  "HeroName": "Raven",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 25
      },
      {
        "Key": "TimeBeforeCursed",
        "Name": "Time Until Cursed",
        "Type": "duration",
        "Value": 2
      }
    ],
    "DescKey": "operative_revelation_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "Radius",
          "Name": "Radius",
          "Type": "distance",
          "Value": 15
        },
        {
          "Key": "CurseDuration",
          "Name": "Curse Duration",
          "Type": "duration",
          "Value": 3
        }
      ]
    }
  },
  "Key": "operative_revelation",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 4.2
    }
  },
  "Name": "Revelation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Value": -20
    },
    "MaxCameraAngleForSeeing": {
      "Name": null,
      "Value": 180
    },
    "MoveSpeedReduction": {
      "Name": null,
      "Value": 20
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 15
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Radius": 5
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "DPS": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "revelation",
      "name": "Revelation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fencer_riposte" title="Riposte" -->

## Riposte

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_riposte`
- Snapshot ID: `39792`
- Source-Dokument: `7071`
- Kurzinfo: Riposte aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Riposte`
- Payload Hash: `a473e0615a9aeb49ad560e95dc07626678bd8d4daa5637492bb1ea5e1eda8b73`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.213101+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.8
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 22
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Debuff": {
    "MoveSpeedMax": {
      "Name": "Move Speed",
      "Type": "slow",
      "Value": 4
    }
  },
  "DescKey": "ability_fencer_riposte_desc",
  "Health": {
    "AbilityLifestealPercentHero": {
      "Name": "Spirit Lifesteal",
      "Type": "healing",
      "Value": 50
    }
  },
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_fencer_riposte_desc",
    "Main": {
      "Props": [
        {
          "Key": "ParryWindow",
          "Name": "Invulnerability Duration",
          "Value": 0.3
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MeleeResistReductionDuration",
        "Name": "Melee Resist Reduction Duration",
        "Type": "duration",
        "Value": 3.0
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "Title": "On Pommel Hit:",
          "Type": "duration",
          "Value": 0.6
        },
        {
          "Key": "MeleeResistReduction",
          "Name": "Melee Resist",
          "Title": "On Pommel Hit:",
          "Type": "bullet_armor_down",
          "Value": -22
        }
      ]
    }
  },
  "Key": "ability_fencer_riposte",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Riposte",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CounterattackAntiMashDelay": {
      "Name": null,
      "Value": 0.2
    },
    "DamageThreshold": {
      "Name": "Damage Threshold",
      "Scale": {
        "Type": "power_increase",
        "Value": 4
      },
      "Value": 60
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.5
    },
    "DashGraceWindow": {
      "Name": null,
      "Value": 1.3
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 2.2
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 76.2
    },
    "LiftHeight": {
      "Name": null,
      "Value": 240
    },
    "SideMoveSpeed": {
      "Name": null,
      "Value": -100
    },
    "SlashConeAngle": {
      "Name": null,
      "Value": 90
    },
    "SlashHalfWidth": {
      "Name": null,
      "Value": 1
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Value": 6
    },
    "SlowDuration": {
      "Name": "Slow Duration",
      "Value": 4
    },
    "SlowPercent": {
      "Name": "Move Speed",
      "Value": 40
    },
    "TurnRateMax": {
      "Name": null,
      "Value": 10
    }
  },
  "Range": {
    "DashRange": {
      "Name": "Time Window",
      "Type": "distance",
      "Value": 35
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "MeleeResistReduction": -30
    },
    {
      "StunDuration": 1.6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "riposte",
      "name": "Riposte",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_punkgoat_goatflip" title="Rising Ram" -->

## Rising Ram

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_goatflip`
- Snapshot ID: `39876`
- Source-Dokument: `7071`
- Kurzinfo: Rising Ram aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rising Ram`
- Payload Hash: `fa3234061cf009623d220fa510963884d32d55582fca7dcc141845563bb0cf23`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.376930+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 0.3
  },
  "DescKey": "ability_punkgoat_goatflip_desc",
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [
      {
        "Key": "WeaponDamageBurst",
        "Type": "damage",
        "Value": 0
      },
      {
        "Key": "WeaponDamageBurstDuration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_punkgoat_goatflip_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.9
          },
          "Title": "On Impact:",
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "DealMaxHealthDamagePct",
          "Name": "Max Health Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Impact:",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_punkgoat_goatflip",
  "Name": "Rising Ram",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirControlAccelPercent": {
      "Name": "Air Acceleration",
      "Value": 50.0
    },
    "AirControlDashReductionPct": {
      "Name": null,
      "Value": -70.0
    },
    "AirControlDebuffDuration": {
      "Name": null,
      "Value": 1.5
    },
    "AirControlPercent": {
      "Name": "Air Control",
      "Value": 50.0
    },
    "AllowRamMultiple": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "ChargeSpeed": {
      "Name": "Charge Speed",
      "Value": 1200
    },
    "ChargeStrikeDistance": {
      "Name": null,
      "Value": 165
    },
    "GoingBackAwaySpeed": {
      "Name": null,
      "Value": -100
    },
    "GoingUpDistance": {
      "Name": null,
      "Value": 3.1
    },
    "GoingUpEnemyDistancePercent": {
      "Name": null,
      "Value": 95
    },
    "GoingUpSpeed": {
      "Name": null,
      "Value": 430
    },
    "HoverGravityScale": {
      "Name": null,
      "Value": 0.75
    },
    "KnockAwaySpeed": {
      "Name": null,
      "Value": 170
    },
    "NearbyHeroKillDistance": {
      "Name": null,
      "Value": 10
    },
    "ReduceCooldownOnHitPct": {
      "Name": null,
      "Value": 50
    },
    "TimeBeforeGoUpForLagComp": {
      "Name": null,
      "Value": 0.1
    },
    "TimeGoingUpEnemy": {
      "Name": null,
      "Value": 0.2
    },
    "WorldImpactRadius": {
      "Name": null,
      "Value": 25
    }
  },
  "Range": {
    "ChargeMultiHitRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 1.5
    },
    "ChargeRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 2.54
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DescKey": "ability_punkgoat_goatflip_t1_desc",
      "WeaponDamageBurst": 25,
      "WeaponDamageBurstDuration": 5
    },
    {
      "AbilityDuration": 0.4,
      "DescKey": "ability_punkgoat_goatflip_t2_desc"
    },
    {
      "AbilityCooldown": -13,
      "DealMaxHealthDamagePct": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.035
        },
        "Value": 8
      }
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "rising ram",
      "name": "Rising Ram",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_throw_sand" title="Sand Blast" -->

## Sand Blast

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_throw_sand`
- Snapshot ID: `39841`
- Source-Dokument: `7071`
- Kurzinfo: Sand Blast aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sand Blast`
- Payload Hash: `87b0bacb0dd1e83ae922d2bc7834446568eee63677fbeb2a8ce4654e7d67859f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.307716+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.5
  },
  "DescKey": "ability_throw_sand_desc",
  "HeroKey": "hero_krill",
  "HeroName": "Mo & Krill",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_throw_sand_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "StatusEffect": "Disarmed",
          "Type": "duration",
          "Value": 2.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_throw_sand",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Sand Blast",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GrowthPerMeter": {
      "Name": null,
      "Value": 0.5
    },
    "HeightOffGround": {
      "Name": null,
      "Value": 20
    },
    "InitialWidth": {
      "Name": null,
      "Value": 5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCastRange": 5,
      "Damage": 50,
      "DescKey": "ability_throw_sand_t1_desc"
    },
    {
      "DescKey": "ability_throw_sand_t2_desc",
      "GroundDashReductionPercent": -30,
      "SlowPercent": 30
    },
    {
      "AbilityCooldown": -25.0,
      "AbilityDuration": 1.5,
      "DescKey": "ability_throw_sand_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "sand blast",
      "name": "Sand Blast",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_vampirebat_batblink" title="Sanguine Retreat" -->

## Sanguine Retreat

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batblink`
- Snapshot ID: `39916`
- Source-Dokument: `7071`
- Kurzinfo: Sanguine Retreat aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sanguine Retreat`
- Payload Hash: `c3795f5d7bd28110484fb4a0c7c280d37b1afda238ec2525b9ce00b74a5a7106`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.453426+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "spirit",
      "Value": 0.02
    },
    "Type": "range",
    "Value": 9
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 0.65
  },
  "Cast": {
    "MaxRecasts": {
      "Name": "Max Recasts",
      "Type": "cast",
      "Value": 1
    }
  },
  "DescKey": "ability_vampirebat_batblink_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_vampirebat_batblink_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Scale": {
            "Type": "spirit",
            "Value": 0.02
          },
          "Type": "range",
          "Value": 9
        },
        {
          "Key": "RecastWindow",
          "Name": "Recast Window",
          "Type": "cooldown",
          "Value": 3.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Title": "On Cast:",
          "Type": "fire_rate",
          "Value": 0
        },
        {
          "Key": "BonusBullets",
          "Name": "Bonus Bullets",
          "Title": "On Cast:",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_vampirebat_batblink",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Sanguine Retreat",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EndJumpVelocity": {
      "Name": null,
      "Value": 200
    },
    "ExitVelocity": {
      "Name": null,
      "Value": 5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusBullets": 8,
      "BonusFireRate": 25,
      "BuffDuration": 8,
      "DescKey": "ability_vampirebat_batblink_t1_desc"
    },
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 3,
      "DescKey": "ability_vampirebat_batblink_t3_desc",
      "MaxRecasts": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "sanguine retreat",
      "name": "Sanguine Retreat",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="fathom_scalding_spray" title="Scalding Spray" -->

## Scalding Spray

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_scalding_spray`
- Snapshot ID: `39891`
- Source-Dokument: `7071`
- Kurzinfo: Scalding Spray aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Scalding Spray`
- Payload Hash: `2eb3f19b2dc06f16306b005b829ea1eb00b57a66459044e3c7c8c49f06090a39`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.404578+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3
  },
  "DescKey": "fathom_scalding_spray_desc",
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_scalding_spray_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.372
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "WeaponDamageBonusPerSec",
          "Name": "Weapon Damage Gain Per Sec",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0372
          },
          "Type": "bullet_damage",
          "Value": 5
        },
        {
          "Key": "WeaponDamageBonusDuration",
          "Name": "Bonus Duration",
          "Type": "duration",
          "Value": 12
        }
      ]
    }
  },
  "Key": "fathom_scalding_spray",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Scalding Spray",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": 55
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "scalding spray",
      "name": "Scalding Spray",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_intimidate" title="Scorn" -->

## Scorn

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_intimidate`
- Snapshot ID: `39839`
- Source-Dokument: `7071`
- Kurzinfo: Scorn aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Scorn`
- Payload Hash: `02fad13e846853307bde01333e47e4709e2bea38c5560e677a7e9394cdb0fc9d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.303355+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 13
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_intimidate_desc",
  "HeroKey": "hero_krill",
  "HeroName": "Mo & Krill",
  "Info1": {
    "Alt": [
      {
        "Key": "DamageHealMultNonHero",
        "Name": "Damage to Heal",
        "Type": "healing",
        "Value": 0.35
      }
    ],
    "DescKey": "ability_intimidate_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.75
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "DamageHealMult",
          "Name": "Hero Damage to Heal",
          "Type": "healing",
          "Value": 1.2
        }
      ]
    }
  },
  "Key": "ability_intimidate",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Scorn",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 9
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 35
    },
    {
      "AbilityCooldown": -5,
      "DescKey": "ability_intimidate_t2_desc",
      "Radius": 1
    },
    {
      "DamageBonus": 15,
      "DebuffDuration": 16,
      "DescKey": "ability_intimidate_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "scorn",
      "name": "Scorn",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_viper_debuffdagger" title="Screwjab Dagger" -->

## Screwjab Dagger

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_debuffdagger`
- Snapshot ID: `39923`
- Source-Dokument: `7071`
- Kurzinfo: Screwjab Dagger aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Screwjab Dagger`
- Payload Hash: `fa6cd4ae72204e1d4381479beb339c0596bd8c1fd278d96fb4baccccb158d3e1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.466975+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 10
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 4.0
  },
  "DescKey": "ability_viper_debuffdagger_desc",
  "Duration": {
    "SlowDuration": {
      "Name": "Slow Duration",
      "Type": "duration",
      "Value": 2
    }
  },
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_viper_debuffdagger_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 35
        },
        {
          "Key": "BulletResistReduction",
          "Name": "Bullet Resist",
          "Title": "On Hit:",
          "Type": "tech_armor_down",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StackDuration",
        "Name": "Stack Duration",
        "Value": 10
      },
      {
        "Key": "MaxStacks",
        "Name": "Max Stacks",
        "Value": 3
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "DamagePerStack",
          "Name": "Damage per Stack",
          "Scale": {
            "Type": "spirit",
            "Value": 0.4
          },
          "Title": "On Stack:",
          "Type": "tech_damage",
          "Value": 25
        },
        {
          "Key": "SlowPercentPerStack",
          "Name": "Move Speed per Stack",
          "Title": "On Stack:",
          "Type": "slow",
          "Value": 15
        },
        {
          "Key": "BulletResistReductionPerStack",
          "Name": "Bullet Resist per Stack",
          "Title": "On Stack:",
          "Type": "tech_armor_down",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_viper_debuffdagger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Screwjab Dagger",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "BulletResistReduction": -8,
      "BulletResistReductionPerStack": -6,
      "DescKey": "ability_viper_debuffdagger_t2_desc"
    },
    {
      "AbilityCooldownBetweenCharge": -2,
      "CooldownRefundPercent": 55,
      "DescKey": "ability_viper_debuffdagger_t3_desc",
      "MaxStacks": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "screwjab dagger",
      "name": "Screwjab Dagger",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_bull_leap" title="Seismic Impact" -->

## Seismic Impact

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_leap`
- Snapshot ID: `39755`
- Source-Dokument: `7071`
- Kurzinfo: Seismic Impact aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Seismic Impact`
- Payload Hash: `ad6457ec54c18328a696a761566530fc99becef361e13c3994b3f384e71f60e1`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.140085+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 215.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_bull_leap_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_bull_leap_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImmunityDuration",
          "Name": "Immunity Duration",
          "StatusEffect": "Unstoppable",
          "Type": "duration",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.325
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 1.6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusMaxHealthPerHero",
          "Name": "Max Health",
          "Title": "On Hero Hit:",
          "Type": "health",
          "Value": 0
        },
        {
          "Key": "BonusFireRatePerHero",
          "Name": "Fire Rate",
          "Title": "On Hero Hit:",
          "Type": "bullet_damage",
          "Value": 0
        },
        {
          "Key": "LandingBonusesDuration",
          "Name": "Buff Duration",
          "Title": "On Hero Hit:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_bull_leap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Seismic Impact",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ImpactHeight": {
      "Name": null,
      "Value": 6
    },
    "TossSpeed": {
      "Name": null,
      "Value": 450
    }
  },
  "Range": {
    "ImpactRadius": {
      "Name": "Impact Radius",
      "Type": "distance",
      "Value": 9
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -30.0
    },
    {
      "StunDuration": 0.8
    },
    {
      "DescKey": "citadel_ability_bull_leap_t3_desc",
      "ImmunityDuration": 6,
      "ImpactRadius": 6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "seismic impact",
      "name": "Seismic Impact",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_shiv_dagger" title="Serrated Knives" -->

## Serrated Knives

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dagger`
- Snapshot ID: `39883`
- Source-Dokument: `7071`
- Kurzinfo: Serrated Knives aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Serrated Knives`
- Payload Hash: `6932fb318260d979b11d62eac86abef99e0c8ec01a2395e60d553ff5c2ab7245`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.389741+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "DescKey": "citadel_ability_shiv_dagger_desc",
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_dagger_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "BleedDPSPerStack",
          "Name": "Bleed DPS Per Knife",
          "Scale": {
            "Type": "spirit",
            "Value": 0.13
          },
          "Type": "tech_damage",
          "Value": 10.0
        },
        {
          "Key": "BleedDuration",
          "Name": "Bleed Duration",
          "Type": "duration",
          "Value": 5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_dagger_max_rage_desc",
    "Main": {
      "Props": [
        {
          "Key": "MovementSlow",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_shiv_dagger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Serrated Knives",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BleedTickRate": {
      "Name": null,
      "Value": 1
    },
    "RicochetCount": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "AOERadius": {
      "Name": "Impact Radius",
      "Type": "distance",
      "Value": 10
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BleedDuration": 2
    },
    {
      "AbilityCharges": 2,
      "DescKey": "citadel_ability_shiv_dagger_t2_desc"
    },
    {
      "BleedDPSPerStack": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.09
        },
        "Value": 12
      },
      "DescKey": "citadel_ability_shiv_dagger_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "serrated knives",
      "name": "Serrated Knives",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_infinity_slash" title="Shadow Transformation" -->

## Shadow Transformation

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_infinity_slash`
- Snapshot ID: `39954`
- Source-Dokument: `7071`
- Kurzinfo: Shadow Transformation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Transformation`
- Payload Hash: `4b1a275560e9aeb09cca3964585858cbf3bea8b28f0646c226e5c2bfe23d809a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.526857+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "duration",
    "Value": 1.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "citadel_ability_infinity_slash_desc",
  "HeroKey": "hero_yamato",
  "HeroName": "Yamato",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 30
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 30
      },
      {
        "Key": "ShadowFormDurationOnKill",
        "Name": "Duration On Kill",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "MaxHealthHealOnCast",
        "Type": "healing",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_infinity_slash_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 5
        },
        {
          "Key": "AbilitySpeedPct",
          "Name": "Ability Speed",
          "Type": "cast",
          "Value": 60
        },
        {
          "Key": "MaxHealthRegen",
          "Name": "Max Health Heal",
          "Type": "healing",
          "Value": 15
        }
      ]
    }
  },
  "Key": "citadel_ability_infinity_slash",
  "Name": "Shadow Transformation",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "WeaponDamageBonus": 7
    },
    {
      "AbilityCooldown": -20,
      "BonusMoveSpeed": 4,
      "DescKey": "citadel_ability_infinity_slash_t2_desc"
    },
    {
      "AbilityDuration": 3.0,
      "BulletResist": 30,
      "DescKey": "citadel_ability_infinity_slash_t3_desc",
      "TechResist": 30
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "shadow transformation",
      "name": "Shadow Transformation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_unicorn_dazzlingorb" title="Shining Wonder" -->

## Shining Wonder

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_dazzlingorb`
- Snapshot ID: `39914`
- Source-Dokument: `7071`
- Kurzinfo: Shining Wonder aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shining Wonder`
- Payload Hash: `7469a13ca355289ea7fd699a0ebd5727f63a12e5b46220304523e338c85b6895`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.449645+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.75
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 160
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_unicorn_dazzlingorb_desc",
  "HeroKey": "hero_unicorn",
  "HeroName": "Celeste",
  "Info1": {
    "Alt": [
      {
        "Key": "GroundDashReductionPercent",
        "Name": "Dash Distance",
        "Type": "slow",
        "Value": -25
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 1.5
      },
      {
        "Key": "BounceRadius",
        "Name": "Bounce Range",
        "Type": "distance",
        "Value": 16.5
      },
      {
        "Key": "BounceGrace",
        "Value": 3
      }
    ],
    "DescKey": "ability_unicorn_dazzlingorb_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "tech_damage",
          "Value": 150
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 40
        },
        {
          "Key": "MaxBounces",
          "Name": "Bounces",
          "Value": 8
        }
      ]
    }
  },
  "Key": "ability_unicorn_dazzlingorb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Shining Wonder",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "NextTargetDuration": {
      "Name": null,
      "Value": 4
    },
    "PriorityBounceRadius": {
      "Name": null,
      "Value": 12.5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DescKey": "ability_unicorn_dazzlingorb_t1_desc",
      "GroundDashReductionPercent": -15,
      "SlowPercent": 20
    },
    {
      "Damage": 80
    },
    {
      "AbilityCooldown": -30,
      "DescKey": "ability_unicorn_dazzlingorb_t3_desc",
      "MaxBounces": 8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "shining wonder",
      "name": "Shining Wonder",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_frank_revive" title="Shocking Reanimation" -->

## Shocking Reanimation

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_revive`
- Snapshot ID: `39806`
- Source-Dokument: `7071`
- Kurzinfo: Shocking Reanimation aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shocking Reanimation`
- Payload Hash: `c03c8337a59de7c0b903fa37f0436ee7e81c5244eef397f05c7da51488fdd979`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.240494+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 275
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Debuff": {
    "EnemyDashSlowPercent": {
      "Name": null,
      "Type": "slow",
      "Value": -30
    }
  },
  "DescKey": "ability_frank_revive_desc",
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 0
      },
      {
        "Key": "BonusDamagePerBullet",
        "Name": "Damage per Bullet",
        "Scale": {
          "Type": "spirit",
          "Value": 0.0
        },
        "Type": "tech_damage",
        "Value": 0
      },
      {
        "Key": "BonusFireRate",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 120
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 3
      }
    ],
    "DescKey": "ability_frank_revive_desc",
    "Main": {
      "Props": [
        {
          "Key": "RespawnHealthPercent",
          "Name": "Rebirth Health",
          "Type": "healing",
          "Value": 50
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Title": "On Revive:",
          "Type": "tech_armor_up",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.0
          },
          "Title": "On Revive:",
          "Type": "tech_damage",
          "Value": 200
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Revive:",
          "Value": 1.5
        }
      ]
    }
  },
  "Key": "ability_frank_revive",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Shocking Reanimation",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.66
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HalfHeight": {
      "Name": null,
      "Value": 15
    },
    "InitialDelay": {
      "Name": null,
      "Value": 0.5
    },
    "RespawnDelay": {
      "Name": null,
      "Value": 3
    },
    "ZombieTickRate": {
      "Name": null,
      "Value": 0.02
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 18
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusDamagePerBullet": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.06
        },
        "Value": 6.0
      },
      "BonusFireRate": 15,
      "DescKey": "ability_frank_revive_t1_desc"
    },
    {
      "RespawnHealthPercent": 50
    },
    {
      "AbilityCooldown": -95,
      "Damage": 175,
      "DescKey": "ability_frank_revive_t3_desc",
      "StunDuration": 1.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "shocking reanimation",
      "name": "Shocking Reanimation",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_bull_charge" title="Shoulder Charge" -->

## Shoulder Charge

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_charge`
- Snapshot ID: `39753`
- Source-Dokument: `7071`
- Kurzinfo: Shoulder Charge aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shoulder Charge`
- Payload Hash: `76685ed092b38bf02367899af9c4ad5d6d578c6e4d6bee40e85ea934b6d73ef8`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.135922+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 1.4
  },
  "DescKey": "citadel_ability_bull_charge_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_bull_charge_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.4
          },
          "Title": "On Hero Collide:",
          "Type": "tech_damage",
          "Value": 30
        },
        {
          "Key": "WeaponDamageBonus",
          "Name": "Weapon Damage",
          "Title": "On Hero Collide:",
          "Type": "bullet_damage",
          "Value": 0
        },
        {
          "Key": "WeaponPowerIncreaseDuration",
          "Name": "Weapon Damage Duration",
          "Title": "On Hero Collide:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Wall Hit:",
          "Type": "duration",
          "Value": 0.3
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Wall Hit:",
          "Type": "slow",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_bull_charge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Shoulder Charge",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 200
    },
    "ChargeDragVerticalOffset": {
      "Name": null,
      "Value": 30
    },
    "ChargeRadius": {
      "Name": null,
      "Value": 2.2
    },
    "ChargeSpeedMax": {
      "Name": null,
      "Value": 30
    },
    "CollidePlayersStopTime": {
      "Name": null,
      "Value": 0.3
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -65
    },
    "SpeedInitial": {
      "Name": null,
      "Value": 18.75
    },
    "TossUpMagnitude": {
      "Name": null,
      "Value": 0.5
    },
    "TurnRateMax": {
      "Name": null,
      "Value": 140
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_bull_charge_t1_desc",
      "SlowDuration": 3,
      "SlowPercent": 40
    },
    {
      "DescKey": "citadel_ability_bull_charge_t2_desc",
      "StunDuration": 0.8
    },
    {
      "AbilityCooldown": -18,
      "DescKey": "citadel_ability_bull_charge_t3_desc",
      "WeaponDamageBonus": 1.5,
      "WeaponPowerIncreaseDuration": 6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "shoulder charge",
      "name": "Shoulder Charge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_trapper_webwall" title="Silktrap" -->

## Silktrap

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_webwall`
- Snapshot ID: `39908`
- Source-Dokument: `7071`
- Kurzinfo: Silktrap aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silktrap`
- Payload Hash: `50fa1e4518f6ad067b98b8f72aee29ebce637348eff55f19bc12fffdf2197f2f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.436890+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.22
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 1
  },
  "DescKey": "ability_trapper_webwall_desc",
  "HeroKey": "hero_trapper",
  "HeroName": "Trapper",
  "Info1": {
    "Alt": [
      {
        "Key": "WebDuration",
        "Name": "Web Duration",
        "Type": "duration",
        "Value": 120
      },
      {
        "Key": "MinWallToWallDistance",
        "Name": "Minimum Web Distance",
        "Type": "distance",
        "Value": 3
      },
      {
        "Key": "MaxWallToWallDistance",
        "Name": "Maximum Web Distance",
        "Type": "distance",
        "Value": 100
      }
    ],
    "DescKey": "ability_trapper_webwall_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.279
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 99
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Value": 3
        },
        {
          "Key": "SilenceDuration",
          "Name": "Silence Duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_trapper_webwall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Silktrap",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "WebArmTime": {
      "Name": null,
      "Value": 0.5
    },
    "WebWallTickRate": {
      "Name": null,
      "Value": 0.15
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 0.6
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "WebDuration": 120
    },
    {
      "DebuffDuration": 2,
      "DescKey": "ability_trapper_webwall_t3_desc",
      "SilenceDuration": 2.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "silktrap",
      "name": "Silktrap",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_self_vacuum" title="Singularity" -->

## Singularity

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_self_vacuum`
- Snapshot ID: `39786`
- Source-Dokument: `7071`
- Kurzinfo: Singularity aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Singularity`
- Payload Hash: `b82601e49b920f5ae519aa65e183f57aa9fe68021bb28d0f53542e3adde6fbd8`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.201540+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 2.75
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 265.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_self_vacuum_desc",
  "HeroKey": "hero_dynamo",
  "HeroName": "Dynamo",
  "Info1": {
    "Alt": [
      {
        "Key": "VacuumRadius",
        "Name": "Singularity Radius",
        "Type": "distance",
        "Value": 7
      },
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 2.75
      }
    ],
    "DescKey": "citadel_ability_self_vacuum_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.28
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "DPSPercentHealth",
          "Name": "Max Health as Damage",
          "Type": "tech_damage",
          "Value": 0.0
        }
      ]
    }
  },
  "Key": "citadel_ability_self_vacuum",
  "Name": "Singularity",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 400
    },
    "Speed": {
      "Name": null,
      "Value": 5.08
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    },
    "TossAngle": {
      "Name": null,
      "Value": 45
    },
    "TossSpeed": {
      "Name": null,
      "Value": 8.89
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "VacuumRadius": 2
    },
    {
      "AbilityChannelTime": 0.75
    },
    {
      "DPSPercentHealth": 6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "singularity",
      "name": "Singularity",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_bull_heal" title="Siphon Life" -->

## Siphon Life

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_heal`
- Snapshot ID: `39752`
- Source-Dokument: `7071`
- Kurzinfo: Siphon Life aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Siphon Life`
- Payload Hash: `25718fc4eb6e24cca97da6d9a3e277fbecdf57ec635fab6157f2b09bcbc5be97`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.133920+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "citadel_ability_bull_heal_desc",
  "HeroKey": "hero_atlas",
  "HeroName": "Abrams",
  "Info1": {
    "Alt": [
      {
        "Key": "NonHeroHealingFactor",
        "Name": "Lifesteal vs Non-Heroes",
        "Type": "healing",
        "Value": 35
      }
    ],
    "DescKey": "citadel_ability_bull_heal_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 22
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 4
        },
        {
          "Key": "HealingFactor",
          "Name": "Lifesteal",
          "Title": "On Hit:",
          "Type": "healing",
          "Value": 70
        }
      ]
    }
  },
  "Key": "citadel_ability_bull_heal",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Siphon Life",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 8
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.12
        },
        "Value": 18
      },
      "DescKey": "citadel_ability_bull_heal_t3_desc",
      "Radius": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "siphon life",
      "name": "Siphon Life",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_boho_bouncyprojectile" title="Skipshot" -->

## Skipshot

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_boho_bouncyprojectile`
- Snapshot ID: `39760`
- Source-Dokument: `7071`
- Kurzinfo: Skipshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Skipshot`
- Payload Hash: `5914cb4cb5df4b20f3bf0f69d9525db2f4ea52b5b50531a908869d59bbe8abc5`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.151303+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 14
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 15
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7
  },
  "DescKey": "ability_boho_bouncyprojectile_desc",
  "HeroKey": "hero_boho",
  "HeroName": "Boho",
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReductionPercentagePerHit",
        "Type": "cooldown",
        "Value": 15
      }
    ],
    "DescKey": "ability_boho_bouncyprojectile_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.4
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "BounceCount",
          "Name": "Shield Bounces",
          "Value": 3
        },
        {
          "Key": "BounceRadius",
          "Name": "Bounce Range",
          "Type": "distance",
          "Value": 18
        }
      ]
    }
  },
  "Key": "ability_boho_bouncyprojectile",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Skipshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -2
    },
    {
      "Damage": 18.0
    },
    {
      "BounceCount": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "skipshot",
      "name": "Skipshot",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_unloadgun" title="Slam Fire" -->

## Slam Fire

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_unloadgun`
- Snapshot ID: `39935`
- Source-Dokument: `7071`
- Kurzinfo: Slam Fire aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slam Fire`
- Payload Hash: `7dd936b4d0d9efe78158d1069bdae4f444f7100b1288abe5634dcb07c97a9476`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.490750+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.45
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Type": "range",
    "Value": 0.0254
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_werewolf_unloadgun_desc",
  "HeroKey": "hero_werewolf",
  "HeroName": "Silver",
  "Info1": {
    "Alt": [
      {
        "Key": "AccuracyPercentage",
        "Name": "Weapon Accuracy",
        "Type": "distance",
        "Value": -30
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Name": "Weapon Damage",
        "Type": "bullet_damage",
        "Value": 0
      },
      {
        "Key": "BonusCurrentHealthDamagePercentage",
        "Name": "Bonus Current Health as Damage",
        "Type": "bullet_damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_unloadgun_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxShots",
          "Name": "Max Shots",
          "Type": "cast",
          "Value": 3
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 300
        },
        {
          "Key": "CurrentHealthDamagePercentage",
          "Name": "Current Health as Damage",
          "Type": "bullet_damage",
          "Value": 2.5
        }
      ]
    }
  },
  "Key": "ability_werewolf_unloadgun",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5.08
    }
  },
  "Name": "Slam Fire",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BulletEffectiveness": {
      "Name": null,
      "Value": 0.1
    },
    "BulletRadiusOverride": {
      "Name": null,
      "Value": 7
    },
    "BulletSpread": {
      "Name": "Gun Spread",
      "Value": -1
    },
    "Damage": {
      "Name": "Damage",
      "Value": 40
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 3
    },
    "LingerDuration": {
      "Name": "Linger Duration",
      "Value": 0.1
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    },
    "RecoilDelayFactor": {
      "Name": null,
      "Value": 0.05
    },
    "RecoilRecoverySpeed": {
      "Name": null,
      "Value": 0.1
    },
    "RecoilSpeed": {
      "Name": null,
      "Value": 12
    },
    "RecoilStrength": {
      "Name": null,
      "Value": 12
    },
    "SpreadPenaltyPerShot": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BaseAttackDamagePercent": 15
    },
    {
      "AbilityCooldown": -10
    },
    {
      "BonusCurrentHealthDamagePercentage": 7,
      "DescKey": "ability_werewolf_unloadgun_t3_desc",
      "MaxStacks": 3,
      "StackDuration": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "slam fire",
      "name": "Slam Fire",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_sleep_dagger" title="Sleep Dagger" -->

## Sleep Dagger

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_sleep_dagger`
- Snapshot ID: `39823`
- Source-Dokument: `7071`
- Kurzinfo: Sleep Dagger aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sleep Dagger`
- Payload Hash: `bef24b601eaf5a964fb4b7a47817ed73b5585a02c5892926732d2df46ffaee2a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.272910+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_sleep_dagger_desc",
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [
      {
        "Key": "MinimumSleepTime",
        "Name": "Min Sleep Time",
        "Type": "duration",
        "Value": 0.2
      },
      {
        "Key": "SleepWakeUpDelay",
        "Name": "Wake Up Delay",
        "Scale": {
          "Type": "spirit",
          "Value": 0.003
        },
        "Type": "duration",
        "Value": 0.1
      },
      {
        "Key": "SleepMoveSpeed",
        "Name": "Sleep Movespeed",
        "Type": "move_speed",
        "Value": 1.5
      },
      {
        "Key": "RicochetRadius",
        "Name": "Ricochet Range",
        "Value": 0
      }
    ],
    "DescKey": "ability_sleep_dagger_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.8
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "SleepDuration",
          "Name": "Sleep Duration",
          "Type": "duration",
          "Value": 2.75
        }
      ]
    }
  },
  "Key": "ability_sleep_dagger",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Sleep Dagger",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DoesNotBreakInvis": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BulletResistReduction": -10.0,
      "BulletResistReductionDuration": 6,
      "DescKey": "ability_sleep_dagger_t1_desc"
    },
    {
      "AbilityCooldown": -18.0,
      "DescKey": "ability_sleep_dagger_t2_desc"
    },
    {
      "DebuffDuration": 3,
      "DescKey": "ability_sleep_dagger_t3_desc",
      "GroundDashReductionPercent": -50,
      "SleepDuration": 1,
      "SlowPercent": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "sleep dagger",
      "name": "Sleep Dagger",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_shiv_dash" title="Slice and Dice" -->

## Slice and Dice

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dash`
- Snapshot ID: `39884`
- Source-Dokument: `7071`
- Kurzinfo: Slice and Dice aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slice and Dice`
- Payload Hash: `67786f6be63a18b5b356f64e3073dfd1c5a6e424aace45d9c29c5951f067c074`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.391759+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_shiv_dash_desc",
  "HeroKey": "hero_shiv",
  "HeroName": "Shiv",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 14
      }
    ],
    "DescKey": "citadel_ability_shiv_dash_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.4415
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "DashRange",
          "Name": "Time Window",
          "Type": "distance",
          "Value": 12
        },
        {
          "Key": "TechArmorDamageReduction",
          "Name": "Spirit Resist",
          "Type": "tech_armor_down",
          "Value": -6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "CooldownReductionOnHitNonHero",
        "Name": "Cooldown Reduced on Non-Hero Hit",
        "Type": "cooldown",
        "Value": 0
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "CooldownReductionOnHit",
          "Name": "Cooldown Reduced on Hero Hit",
          "Title": "On hit",
          "Type": "cooldown",
          "Value": 0
        },
        {
          "Key": "MaxCooldownReductionsFromHits",
          "Name": "Max Cooldown Reduction",
          "Title": "On hit",
          "Value": 0
        }
      ]
    },
    "RequiresUpgradeIndex": 2
  },
  "Info3": {
    "Alt": [],
    "DescKey": "citadel_ability_shiv_dash_ult_desc",
    "Main": {}
  },
  "Key": "citadel_ability_shiv_dash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Slice and Dice",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 2.5
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 60.96
    },
    "MoveSpeedPenaltyMaxSpeed": {
      "Name": null,
      "Value": 200
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -100
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.35
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "DashRange": 2,
      "DescKey": "citadel_ability_shiv_dash_t2_desc",
      "TechArmorDamageReduction": -6
    },
    {
      "CooldownReductionOnHit": 2,
      "CooldownReductionOnHitNonHero": 1,
      "DescKey": "citadel_ability_shiv_dash_t3_desc",
      "ImpactDamage": 50,
      "MaxCooldownReductionsFromHits": 8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "slice and dice",
      "name": "Slice and Dice",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_viper_snakedash" title="Slither" -->

## Slither

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_snakedash`
- Snapshot ID: `39925`
- Source-Dokument: `7071`
- Kurzinfo: Slither aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slither`
- Payload Hash: `8f17ba425c389faf6af42ce20425ff13a2521057ca29fca9b04f96c74a3b1faa`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.471125+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_viper_snakedash_desc",
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_viper_snakedash_desc",
    "Main": {
      "Props": [
        {
          "Key": "SlideScale",
          "Name": "Slide Distance",
          "Type": "move_speed",
          "Value": 15
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Slide:",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_viper_snakedash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Slither",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "SlideScale": 20
    },
    {
      "Stamina": 2
    },
    {
      "AbilityCooldown": 8,
      "BuffDuration": 5,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.8
        },
        "Value": 180
      },
      "DescKey": "ability_viper_snakedash_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "slither",
      "name": "Slither",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_smoke_bomb" title="Smoke Bomb" -->

## Smoke Bomb

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_smoke_bomb`
- Snapshot ID: `39824`
- Source-Dokument: `7071`
- Kurzinfo: Smoke Bomb aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Smoke Bomb`
- Payload Hash: `92910b2c0169677ed73adebb3463add2f1964c2e5c48e7a6b24da5b1c4e9e08c`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.274874+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 33.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Type": "duration",
    "Value": 8
  },
  "DescKey": "ability_smoke_bomb_desc",
  "Duration": {
    "RevealOnDamageDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_haze",
  "HeroName": "Haze",
  "Info1": {
    "Alt": [
      {
        "Key": "SpottedRadius",
        "Name": "Spot Radius",
        "Type": "distance",
        "Value": 18
      },
      {
        "Key": "InvisMoveSpeedMod",
        "Name": "Invis Sprint Speed",
        "Type": "move_speed",
        "Value": 0
      },
      {
        "Key": "PostInvisBuffDuration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "PhaseOutDuration",
        "Name": "Invincible Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_smoke_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "InvisFadeToDuration",
          "Name": "Fade Time",
          "Type": "duration",
          "Value": 1.5
        }
      ]
    }
  },
  "Key": "ability_smoke_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Smoke Bomb",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FullInvisDistance": {
      "Name": null,
      "Value": 50
    },
    "InvisAlertWhenFading": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "InvisMoveSpeedMod": 7
    },
    {
      "AbilityCharges": 2,
      "AbilityCooldownBetweenCharge": 7,
      "DescKey": "ability_smoke_bomb_t2_desc"
    },
    {
      "BulletLifesteal": 50,
      "DescKey": "ability_smoke_bomb_t3_desc",
      "DispelOnUse": 1,
      "PostInvisBuffDuration": 5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_haze",
      "hero_name": "Haze",
      "lookup": "smoke bomb",
      "name": "Smoke Bomb",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_health_swap" title="Soul Exchange" -->

## Soul Exchange

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_health_swap`
- Snapshot ID: `39810`
- Source-Dokument: `7071`
- Kurzinfo: Soul Exchange aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Soul Exchange`
- Payload Hash: `d610d0155ba65e45e6e0202eac3c2f1e4fbb334a7955bcdd9d85edf625f21845`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.247917+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 5.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 220.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 0.25
  },
  "DescKey": "ability_health_swap_desc",
  "HeroKey": "hero_ghost",
  "HeroName": "Lady Geist",
  "Info1": {
    "Alt": [
      {
        "Key": "SelfBuffDuration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "SilenceDuration",
        "Name": "Silence Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "SilenceRadius",
        "Name": "Silence Radius",
        "Type": "distance",
        "Value": 0
      }
    ],
    "DescKey": "ability_health_swap_desc",
    "Main": {
      "Props": [
        {
          "Key": "EnemyMinHealthPct",
          "Name": "Enemy Min Health",
          "Type": "health",
          "Value": 30
        },
        {
          "Key": "MinHealthTakenPct",
          "Name": "Min Health Received",
          "Type": "health",
          "Value": 30
        }
      ]
    }
  },
  "Key": "ability_health_swap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 2
    }
  },
  "Name": "Soul Exchange",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EnemySlowPct": {
      "Name": "Enemy Move Speed",
      "Value": 70
    },
    "InitialUpSpeed": {
      "Name": null,
      "Value": 150
    },
    "MinDiffToCast": {
      "Name": null,
      "Value": 0.1
    },
    "PostCastHoldTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -50.0
    },
    {
      "DescKey": "ability_health_swap_t2_desc",
      "SilenceDuration": 3,
      "SilenceRadius": 25
    },
    {
      "BonusFireRate": 40,
      "BonusSpirit": 60,
      "DescKey": "ability_health_swap_t3_desc",
      "SelfBuffDuration": 8,
      "TechResist": 50
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "soul exchange",
      "name": "Soul Exchange",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_magician_cloneturret" title="Spectral Assistant" -->

## Spectral Assistant

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_cloneturret`
- Snapshot ID: `39848`
- Source-Dokument: `7071`
- Kurzinfo: Spectral Assistant aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spectral Assistant`
- Payload Hash: `3f6f32df04153e5cc9d5d0d9193a828cbea943e4451a01f19e1f2eaeb33d533d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.322511+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_magician_cloneturret_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_magician_cloneturret_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.36
          },
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 6
        },
        {
          "Key": "TotalSwaps",
          "Name": "Max Swaps",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_magician_cloneturret",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spectral Assistant",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TurretBulletTargetAngle": {
      "Name": null,
      "Type": "",
      "Value": 20
    },
    "TurretBulletTargetRadius": {
      "Name": null,
      "Value": 500
    },
    "TurretBulletVerticalOffset": {
      "Name": null,
      "Value": 2
    }
  },
  "Range": {
    "LeashRadius": {
      "Name": "Max Leash Radius",
      "Type": "time",
      "Value": 20
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 5,
      "AbilityDuration": 7,
      "DescKey": "ability_magician_cloneturret_t2_desc",
      "LeashRadius": 5
    },
    {
      "BonusFireRate": 60,
      "Damage": 12.6,
      "DescKey": "ability_magician_cloneturret_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "spectral assistant",
      "name": "Spectral Assistant",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_fissure_wall" title="Spectral Wall" -->

## Spectral Wall

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_fissure_wall`
- Snapshot ID: `39797`
- Source-Dokument: `7071`
- Kurzinfo: Spectral Wall aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spectral Wall`
- Payload Hash: `2f88a7ed36c6e449f372f3828d7b957e3cb58b04d711f8ccf7f4c899699c8aff`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.223692+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 50
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6.0
  },
  "DescKey": "citadel_ability_fissure_wall_desc",
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "MinRange",
        "Name": "Minimum Range",
        "Type": "distance",
        "Value": 5
      },
      {
        "Key": "WallImpactRange",
        "Name": "Impact Range",
        "Type": "distance",
        "Value": 5
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 2.5
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_fissure_wall_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.731203
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "WallStunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_fissure_wall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spectral Wall",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "NumWallSegments": {
      "Name": null,
      "Value": 8
    },
    "PushForce": {
      "Name": null,
      "Value": 175
    },
    "SegmentEmitTime": {
      "Name": null,
      "Value": 0.1
    },
    "TimeBetweenSegments": {
      "Name": null,
      "Value": 0.035
    },
    "TimeToMaxDistance": {
      "Name": null,
      "Value": 1.8
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BonusDamagePercent": 20,
      "DebuffDuration": 7,
      "DescKey": "citadel_ability_fissure_wall_t1_desc"
    },
    {
      "AbilityCooldown": -20.0,
      "AbilityDuration": 2,
      "DescKey": "citadel_ability_fissure_wall_t2_desc"
    },
    {
      "CreateTurrets": 2,
      "DescKey": "citadel_ability_fissure_wall_t3_desc",
      "SlowPercent": 30,
      "TurretLifeTime": 8
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "spectral wall",
      "name": "Spectral Wall",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_gravity_lasso" title="Spirit Lasso" -->

## Spirit Lasso

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_gravity_lasso`
- Snapshot ID: `39751`
- Source-Dokument: `7071`
- Kurzinfo: Spirit Lasso aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Lasso`
- Payload Hash: `964ab08484c34870c7a161c50003e3f178801769cff57640c83c729c6581ad76`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.131870+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 130
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.25
  },
  "DescKey": "ability_gravity_lasso_desc",
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [
      {
        "Key": "BouncePadExtendDuration",
        "Value": 1.0
      }
    ],
    "DescKey": "ability_gravity_lasso_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 2.25
        },
        {
          "Key": "AbilityCastRange",
          "Name": "Cast Range",
          "Type": "range",
          "Value": 20
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Key": "ability_gravity_lasso",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spirit Lasso",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraPreviewDistance": {
      "Name": null,
      "Value": 200
    },
    "CameraPreviewOffset": {
      "Name": null,
      "Value": 25
    },
    "CameraPreviewSpeed": {
      "Name": null,
      "Value": 0.6
    },
    "ExtraTargetConeAngle": {
      "Name": "Targetting Cone Angle",
      "Value": 60
    },
    "ExtraTargetHorizontalOffset": {
      "Name": null,
      "Value": 30
    },
    "FollowDampingFactor": {
      "Name": null,
      "Value": 8
    },
    "FollowDistance": {
      "Name": null,
      "Value": 60
    },
    "GrabExtraTargetsRadiusMult": {
      "Name": null,
      "Value": 2
    },
    "LassoTargetMaxSpeed": {
      "Name": null,
      "Value": 55
    },
    "LiftHeight": {
      "Name": null,
      "Value": 7
    },
    "LiftHorizontal": {
      "Name": null,
      "Value": -30
    },
    "LiftInitialDelay": {
      "Name": null,
      "Value": 0.5
    },
    "LiftInitialRisingSpeed": {
      "Name": null,
      "Value": 100
    },
    "LiftInitialVelocityStart": {
      "Name": null,
      "Value": 500
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 80
    },
    {
      "AbilityDuration": 0.75
    },
    {
      "AbilityCooldown": -40
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "spirit lasso",
      "name": "Spirit Lasso",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_immobilize_trap" title="Spirit Snare" -->

## Spirit Snare

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_immobilize_trap`
- Snapshot ID: `39869`
- Source-Dokument: `7071`
- Kurzinfo: Spirit Snare aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Snare`
- Payload Hash: `830a4defb8d3a54b19af73176cddd7a5b7cec99abff4b491e882ffe37c4b00e0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.363049+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_immobilize_trap_desc",
  "Duration": {
    "TripTime": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [
      {
        "Key": "ArmTime",
        "Name": "Arm Time",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 30
      },
      {
        "Key": "ChargedShotHitRadiusScale",
        "Name": "Charged Shot Radius",
        "Type": "radius",
        "Value": 30
      },
      {
        "Key": "BulletVulnerbility",
        "Name": "Bullet Damage Amp",
        "Type": "bullet_armor_down",
        "Value": 0
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_immobilize_trap_desc",
    "Main": {
      "Props": [
        {
          "Key": "TetherDuration",
          "Name": "Tether Duration",
          "Type": "duration",
          "Value": 2.25
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 25
        },
        {
          "Key": "Lifetime",
          "Name": "Lifetime",
          "Type": "duration",
          "Value": 22
        }
      ]
    }
  },
  "Key": "ability_immobilize_trap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spirit Snare",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SkipFrames": {
      "Name": null,
      "Value": 6
    },
    "TrapHeight": {
      "Name": null,
      "Value": 2
    },
    "TripGravity": {
      "Name": null,
      "Value": 0.4
    },
    "TripUpSpeed": {
      "Name": null,
      "Value": 6.35
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Range": {
    "TetherRadius": {
      "Name": "Tether Radius",
      "Type": "distance",
      "Value": 6
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "BulletArmorReduction": -15,
      "DebuffDuration": 10,
      "DescKey": "ability_immobilize_trap_t2_desc"
    },
    {
      "DescKey": "ability_immobilize_trap_t3_desc",
      "Radius": 1.5,
      "TetherDuration": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "spirit snare",
      "name": "Spirit Snare",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="viscous_goo_grenade" title="Splatter" -->

## Splatter

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_grenade`
- Snapshot ID: `39927`
- Source-Dokument: `7071`
- Kurzinfo: Splatter aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Splatter`
- Payload Hash: `866d099ff8d113e8e7ec748ac1fa2a554f757c2470b53caff67252ddcdd0d252`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.475269+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.001
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "viscous_goo_grenade_desc",
  "HeroKey": "hero_viscous",
  "HeroName": "Viscous",
  "Info1": {
    "Alt": [],
    "DescKey": "viscous_goo_grenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "tech_damage",
          "Value": 70
        },
        {
          "Key": "MaxBounces",
          "Name": "Bounces",
          "Value": 1
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "viscous_goo_grenade_bounce_desc",
    "Main": {},
    "RequiresUpgradeIndex": 2
  },
  "Info3": {
    "Alt": [],
    "DescKey": "viscous_goo_grenade_puddle_desc",
    "Main": {
      "Props": [
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 35
        },
        {
          "Key": "PuddleDuration",
          "Name": "Puddle Duration",
          "Scale": {
            "Type": "duration",
            "Value": 1.1
          },
          "Type": "duration",
          "Value": 10
        }
      ]
    }
  },
  "Key": "viscous_goo_grenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Splatter",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DetonateCooldown": {
      "Name": null,
      "Value": 0.15
    },
    "FourthHitDamagePercentage": {
      "Name": null,
      "Value": 0.26
    },
    "PuddleSlideBuff": {
      "Name": null,
      "Value": 60
    },
    "SecondHitDamagePercentage": {
      "Name": null,
      "Value": 0.5
    },
    "ThirdHitDamagePercentage": {
      "Name": null,
      "Value": 0.38
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 36,
      "DescKey": "viscous_goo_grenade_t1_desc",
      "Radius": 2
    },
    {
      "AbilityCooldown": -14
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "viscous_goo_grenade_t3_desc",
      "MaxBounces": 2
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "splatter",
      "name": "Splatter",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_hornet_chain" title="Stake" -->

## Stake

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_chain`
- Snapshot ID: `39827`
- Source-Dokument: `7071`
- Kurzinfo: Stake aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stake`
- Payload Hash: `dc64e4020936224245e2de5b5cb0ef55bf2501fe8e51e924eb58881329c70b55`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.280246+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_hornet_chain_desc",
  "HeroKey": "hero_hornet",
  "HeroName": "Vindicta",
  "Info1": {
    "Alt": [
      {
        "Key": "ChainLength",
        "Name": "Tether Length",
        "Type": "distance",
        "Value": 9
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 40
      }
    ],
    "DescKey": "citadel_ability_hornet_chain_desc",
    "Main": {
      "Props": [
        {
          "Key": "ChainDuration",
          "Name": "Tether Duration",
          "Type": "duration",
          "Value": 1.75
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.5
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "CaptureRadius",
          "Name": "Capture Radius",
          "Type": "distance",
          "Value": 9
        }
      ]
    }
  },
  "Key": "citadel_ability_hornet_chain",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Stake",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "EnemyDragSpeed": {
      "Name": null,
      "Value": 25.4
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 65
    },
    {
      "AbilityCooldown": -22.0
    },
    {
      "CaptureRadius": 2,
      "ChainDuration": 0.75,
      "DescKey": "citadel_ability_hornet_chain_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "stake",
      "name": "Stake",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="drifter_shadow_mark" title="Stalker's Mark" -->

## Stalker's Mark

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `drifter_shadow_mark`
- Snapshot ID: `39780`
- Source-Dokument: `7071`
- Kurzinfo: Stalker's Mark aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stalker's Mark`
- Payload Hash: `4cd9278c21d4a1aa83bebd6fea98ee1c15d3e4c7a4d6a31da059e99889c1c44f`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.190732+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "Damage": {
    "TeleportDamage": {
      "Name": "Blink Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 0.5
      },
      "Type": "tech_damage",
      "Value": 0
    }
  },
  "DescKey": "drifter_shadow_mark_desc",
  "HeroKey": "hero_drifter",
  "HeroName": "Drifter",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityCharges",
        "Name": "Charges",
        "Type": "cast",
        "Value": 0
      },
      {
        "Key": "BulletResistReduction",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_down",
        "Value": 0
      },
      {
        "Key": "AbilityCooldownBetweenCharge",
        "Name": "Charge Delay",
        "Type": "charge_cooldown",
        "Value": -1.0
      }
    ],
    "DescKey": "drifter_shadow_mark_desc",
    "Main": {
      "Props": [
        {
          "Key": "DotHealthPercent",
          "Name": "Bleed Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.015
          },
          "Type": "tech_damage",
          "Value": 2.0
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 5
        },
        {
          "Key": "HealAmpReceivePenaltyPercent",
          "Name": "Healing Reduction",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Title": "After ambushing:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "drifter_shadow_mark",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Stalker's Mark",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 3
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.3
    },
    "TeleportBackOffsetFromTarget": {
      "Name": null,
      "Value": 135
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    },
    "VerticalDrag": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BulletResistReduction": -8,
      "DescKey": "drifter_shadow_mark_t1_desc"
    },
    {
      "AbilityCooldown": -8,
      "AbilityDuration": 3,
      "DescKey": "drifter_shadow_mark_t2_desc"
    },
    {
      "DescKey": "drifter_shadow_mark_t3_desc",
      "DotHealthPercent": 2.0,
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "stalker's mark",
      "name": "Stalker's Mark",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_static_charge" title="Static Charge" -->

## Static Charge

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_static_charge`
- Snapshot ID: `39812`
- Source-Dokument: `7071`
- Kurzinfo: Static Charge aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Static Charge`
- Payload Hash: `eacd72c3fcd62e18bb432489f84d6d972498eb959618fe1bc2a078324db442b9`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.251791+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_static_charge_desc",
  "HeroKey": "hero_gigawatt",
  "HeroName": "Seven",
  "Info1": {
    "Alt": [
      {
        "Key": "ShockRadius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 5
      }
    ],
    "DescKey": "citadel_ability_static_charge_desc",
    "Main": {
      "Props": [
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 0.9
        },
        {
          "Key": "ShockDelay",
          "Name": "Delay Before Stun",
          "Type": "cast",
          "Value": 3.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.792137
          },
          "Type": "tech_damage",
          "Value": 35
        }
      ]
    }
  },
  "Key": "citadel_ability_static_charge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Static Charge",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityCastRange": 5,
      "DescKey": "citadel_ability_static_charge_t2_desc",
      "ShockRadius": 7
    },
    {
      "Damage": 160,
      "DescKey": "citadel_ability_static_charge_t3_desc",
      "StunDuration": 0.9
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "static charge",
      "name": "Static Charge",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_sticky_bomb" title="Sticky Bomb" -->

## Sticky Bomb

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_sticky_bomb`
- Snapshot ID: `39757`
- Source-Dokument: `7071`
- Kurzinfo: Sticky Bomb aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sticky Bomb`
- Payload Hash: `5e528be3401089906f9918034a0fbadb5cb97c4418b65c9f287a227d3336a0ef`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.144334+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 6
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3.5
  },
  "DescKey": "citadel_ability_sticky_bomb_desc",
  "Duration": {
    "KillCheckWindow": {
      "Name": null,
      "Type": "duration",
      "Value": 10.0
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_sticky_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "FuseTime",
          "Name": "Fuse Time",
          "Type": "duration",
          "Value": 3.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 85
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Title": "On Attach:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "BonusDamagePctPerPlayerHit",
          "Name": "Sticky Bomb Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0015
          },
          "Title": "On Hero Hit:",
          "Type": "damage",
          "Value": 1.0
        },
        {
          "Key": "BonusDamagePctPerPlayerKilled",
          "Name": "Sticky Bomb Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.01
          },
          "Title": "On Hero Kill:",
          "Type": "damage",
          "Value": 2.5
        }
      ]
    }
  },
  "Key": "citadel_ability_sticky_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Sticky Bomb",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "OnHitDiminish": {
      "Name": null,
      "Value": 60
    },
    "OnKillDiminish": {
      "Name": null,
      "Value": 7
    },
    "SelfDamagePercent": {
      "Name": null,
      "Value": 20
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 8
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 85
    },
    {
      "DescKey": "citadel_ability_sticky_bomb_t3_desc",
      "MovementSpeedBonus": 5,
      "MovementSpeedBonusDuration": 6,
      "StatusResistancePercent": 25
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "sticky bomb",
      "name": "Sticky Bomb",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_tengu_stone_form" title="Stone Form" -->

## Stone Form

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_stone_form`
- Snapshot ID: `39905`
- Source-Dokument: `7071`
- Kurzinfo: Stone Form aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stone Form`
- Payload Hash: `d534f10907af2dbb139357eb1707d715d41b1d58a19846283be1384dfda97b56`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.431292+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3
  },
  "Debuff": {
    "MoveSpeedMax": {
      "Name": "Move Speed",
      "Type": "slow",
      "Value": 8
    }
  },
  "DescKey": "citadel_ability_tengu_stone_form_desc",
  "HeroKey": "hero_tengu",
  "HeroName": "Ivy",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_tengu_stone_form_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 0.75
        },
        {
          "Key": "MaxHealthRegen",
          "Name": "Max Health Heal",
          "Type": "healing",
          "Value": 6
        }
      ]
    }
  },
  "Key": "citadel_ability_tengu_stone_form",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Stone Form",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.25
    },
    "LiftHeight": {
      "Name": null,
      "Value": 180
    },
    "LiftTime": {
      "Name": null,
      "Value": 1.0
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6
  },
  "Slot": "3",
  "Upgrades": [
    {
      "MaxHealthRegen": 7.0
    },
    {
      "AbilityCooldown": -25
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.7
        },
        "Value": 0
      },
      "DescKey": "citadel_ability_tengu_stone_form_t3_desc",
      "StunDuration": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "stone form",
      "name": "Stone Form",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_storm_cloud" title="Storm Cloud" -->

## Storm Cloud

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_storm_cloud`
- Snapshot ID: `39814`
- Source-Dokument: `7071`
- Kurzinfo: Storm Cloud aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Storm Cloud`
- Payload Hash: `d5b73b35696e558a94e0f9e75bb9182cd47ba6c51375fad50a8861d4cff156a9`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.255830+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 7
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 205.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "ExpandTime": {
      "Name": "Expand time",
      "Type": "cooldown",
      "Value": 3.5
    }
  },
  "DescKey": "citadel_ability_storm_cloud_desc",
  "HeroKey": "hero_gigawatt",
  "HeroName": "Seven",
  "Info1": {
    "Alt": [
      {
        "Key": "InitialRadius",
        "Name": "Initial Radius",
        "Type": "distance",
        "Value": 10
      },
      {
        "Key": "BulletResistOnActive",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_storm_cloud_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 95
        },
        {
          "Key": "FlightControlEnabled",
          "Name": "Flight Speed",
          "Value": 1.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "LightningStrikeDamage",
        "Name": "Strike Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Type": "tech_damage",
        "Value": 75.0
      },
      {
        "Key": "LightningStrikeRadius",
        "Name": "Strike Radius",
        "Type": "distance",
        "Value": 7
      }
    ],
    "DescKey": "citadel_ability_storm_cloud_lightning_strike_desc",
    "Main": {
      "Props": []
    }
  },
  "Key": "citadel_ability_storm_cloud",
  "Name": "Storm Cloud",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 600
    },
    "CloudHeight": {
      "Name": null,
      "Value": 120
    },
    "DamageInterval": {
      "Name": null,
      "Value": 0.3
    },
    "EndingSoonTime": {
      "Name": null,
      "Value": 2
    },
    "LightningStrikeDelay": {
      "Name": null,
      "Value": 0.25
    },
    "LightningStrikeKnockBackForce": {
      "Name": null,
      "Value": 500
    },
    "LightningStrikes": {
      "Name": null,
      "Value": 1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 30
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BulletResistOnActive": 55,
      "DescKey": "citadel_ability_storm_cloud_t1_desc"
    },
    {
      "AbilityChannelTime": 7,
      "DescKey": "citadel_ability_storm_cloud_t2_desc",
      "InitialRadius": 5,
      "Radius": 10
    },
    {
      "DPS": 65.0,
      "DescKey": "citadel_ability_storm_cloud_t3_desc",
      "FlightControlEnabled": 4
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "storm cloud",
      "name": "Storm Cloud",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_boho_doublehit" title="Swish" -->

## Swish

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_boho_doublehit`
- Snapshot ID: `39761`
- Source-Dokument: `7071`
- Kurzinfo: Swish aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Swish`
- Payload Hash: `8348ae4ae062db60d182c0f296f81720aaa50c304a093ac22b404db75ec3ba53`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.153040+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 11
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_boho_doublehit_desc",
  "HeroKey": "hero_boho",
  "HeroName": "Boho",
  "Info1": {
    "Alt": [
      {
        "Key": "BuffDuration",
        "Name": "Buff Duration",
        "Type": "move_speed",
        "Value": 4
      },
      {
        "Key": "CombatBarrierPerStack",
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Type": "health",
        "Value": 20
      }
    ],
    "DescKey": "ability_boho_doublehit_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Type": "tech_damage",
          "Value": 50
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Title": "On Hero Hit:",
          "Type": "health",
          "Value": 80
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "On Hero Hit:",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_boho_doublehit",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Swish",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.7
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 16
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 6
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 100
    },
    "TimeBetweenAttacks": {
      "Name": null,
      "Value": 0.35
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 18.0
    },
    {
      "BonusMoveSpeed": 2
    },
    {
      "CombatBarrier": 80
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "swish",
      "name": "Swish",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_familiar_attach" title="Tag Along" -->

## Tag Along

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_attach`
- Snapshot ID: `39788`
- Source-Dokument: `7071`
- Kurzinfo: Tag Along aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tag Along`
- Payload Hash: `da9435dfb494c9153e26c053497ef4224ad34844f6a1dd1f458283bfb9c1d74d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.205251+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 23
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5.5
  },
  "DescKey": "ability_familiar_attach_desc",
  "HeroKey": "hero_familiar",
  "HeroName": "Rem",
  "Info1": {
    "Alt": [
      {
        "Key": "HealthRegenDuration",
        "Name": "Regen Duration",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "TechPowerPercent",
        "Name": "Spirit Power",
        "Type": "tech_damage",
        "Value": 0
      },
      {
        "Key": "HopOffEffecDuration",
        "Name": "Buff Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "BonusSpiritPower",
        "Name": "Bonus Spirit Power",
        "Type": "tech_damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_familiar_attach_desc",
    "Main": {
      "Props": [
        {
          "Key": "MissingHealthBurstPct",
          "Name": "Missing Health Burst",
          "Scale": {
            "Type": "spirit",
            "Value": 0.03
          },
          "Type": "health",
          "Value": 15
        },
        {
          "Key": "HealingPerSecond",
          "Name": "Health Restored",
          "Scale": {
            "Type": "spirit",
            "Value": 0.4
          },
          "Type": "healing",
          "Value": 42
        },
        {
          "Key": "BonusBarrierAmpPercent",
          "Name": "Barrier Effectiveness",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "BonusItemDurationPercent",
          "Name": "Bonus Item Duration/Range",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_familiar_attach",
  "Move": {
    "BonusMoveSpeed": {
      "Name": "Move Speed",
      "Type": "move_speed",
      "Value": 4
    },
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5
    }
  },
  "Name": "Tag Along",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HopOutLockoutDuration": {
      "Name": null,
      "Value": 0.3
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "BonusBarrierAmpPercent": 35,
      "BonusItemDurationPercent": 35,
      "BonusItemRangePercent": 35,
      "DescKey": "ability_familiar_attach_t2_desc"
    },
    {
      "BonusSpiritPower": 35,
      "DescKey": "ability_familiar_attach_t3_desc",
      "HealingPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.34
        },
        "Value": 0
      },
      "HopOffEffecDuration": 10,
      "MissingHealthBurstPct": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.016
        },
        "Value": 0
      },
      "TechPowerPercent": 15
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "tag along",
      "name": "Tag Along",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_werewolf_cripplingslash" title="Tail Whack" -->

## Tail Whack

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_cripplingslash`
- Snapshot ID: `39941`
- Source-Dokument: `7071`
- Kurzinfo: Tail Whack aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tail Whack`
- Payload Hash: `653b20f8a47cdbdedcbd153e98de19c32b9f40f7ba343eac3c384a76f2189e83`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.501511+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_cripplingslash_desc",
  "Duration": {
    "SlowDuration": {
      "Name": "Slow Duration",
      "Type": "duration",
      "Value": 2.0
    }
  },
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_cripplingslash_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Type": "tech_damage",
          "Value": 45
        },
        {
          "Key": "DisarmDuration",
          "Name": "Disarm Duration",
          "StatusEffect": "Disarmed",
          "Title": "On Hit:",
          "Value": 2.0
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 30
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Title": "On Hit:",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_werewolf_cripplingslash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Tail Whack",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "LeftForce": {
      "Name": null,
      "Value": 400
    },
    "PushForce": {
      "Name": null,
      "Value": 300
    },
    "SlashHeight": {
      "Name": null,
      "Value": 3
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Value": 10
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "SlowPercent": 40
    },
    {
      "DescKey": "ability_werewolf_cripplingslash_t3_desc",
      "DisarmDuration": 1.5,
      "SlowDuration": 1.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "tail whack",
      "name": "Tail Whack",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_psychic_lift" title="Telekinesis" -->

## Telekinesis

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_psychic_lift`
- Snapshot ID: `39946`
- Source-Dokument: `7071`
- Kurzinfo: Telekinesis aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Telekinesis`
- Payload Hash: `b977e1b28611e42dd1cd47a29248f6b8f4687a8c4ef06f52b4bd33419b1822d0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.510659+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.45
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.65
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 2.25
  },
  "DescKey": "citadel_ability_psychic_lift_desc",
  "HeroKey": "hero_wraith",
  "HeroName": "Wraith",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 100
        },
        {
          "Key": "TossDistance",
          "Name": "Throw Range",
          "Value": 13
        },
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "StatusEffect": "Stun",
          "Type": "cast",
          "Value": 0.65
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_slam_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 2.25
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 40
        }
      ]
    }
  },
  "Key": "citadel_ability_psychic_lift",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Telekinesis",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.3
    },
    "LiftChainRadius": {
      "Name": null,
      "Value": 20
    },
    "LiftDuration": {
      "Name": null,
      "Value": 2
    },
    "LiftHeight": {
      "Name": null,
      "Value": 80
    },
    "TossUpStrength": {
      "Name": null,
      "Value": 220
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -45
    },
    {
      "AbilityCastRange": 6,
      "AbilityDuration": 1.5,
      "DescKey": "citadel_ability_psychic_lift_t3_desc",
      "TossDistance": 6
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "telekinesis",
      "name": "Telekinesis",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="viscous_restorative_goo" title="The Cube" -->

## The Cube

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_restorative_goo`
- Snapshot ID: `39928`
- Source-Dokument: `7071`
- Kurzinfo: The Cube aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `The Cube`
- Payload Hash: `6fdf9c8812b3cf53b737cb7fd3b06bb4d720c4322a91a536d6bfa636485e4cd0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.477163+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 26
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 42.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 3
  },
  "DescKey": "viscous_restorative_goo_desc",
  "HeroKey": "hero_viscous",
  "HeroName": "Viscous",
  "Info1": {
    "Alt": [],
    "DescKey": "viscous_restorative_goo_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusHealthRegen",
          "Name": "Health Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "healing",
          "Value": 40
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 3
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "DescKey": "viscous_restorative_goo_buff_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Value": 0
        },
        {
          "Key": "StaminaCooldownReduction",
          "Name": "Stamina Recovery",
          "Value": 0
        },
        {
          "Key": "PostCubeBuffDuration",
          "Name": "Buff Duration",
          "Value": 8
        }
      ]
    },
    "RequiresUpgradeIndex": 0
  },
  "Key": "viscous_restorative_goo",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "The Cube",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BreakoutTime": {
      "Name": null,
      "Value": 1
    },
    "BulletForce": {
      "Name": null,
      "Value": 600
    },
    "CubeScale": {
      "Name": null,
      "Value": 1.5
    },
    "Friction": {
      "Name": null,
      "Value": -80
    },
    "HeavyMeleeForce": {
      "Name": null,
      "Value": 700
    },
    "LightMeleeForce": {
      "Name": null,
      "Value": 300
    },
    "PushBackForce": {
      "Name": null,
      "Value": 250
    },
    "PushBackRadius": {
      "Name": null,
      "Value": 50
    },
    "SlideForce": {
      "Name": null,
      "Value": 70
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.5,
      "DescKey": "viscous_restorative_goo_t1_desc",
      "PostCubeBuff": 1,
      "StaminaCooldownReduction": 30
    },
    {
      "AbilityDuration": 1,
      "BonusHealthRegen": 25,
      "DescKey": "viscous_restorative_goo_t2_desc"
    },
    {
      "AbilityCooldown": -25.0,
      "DescKey": "viscous_restorative_goo_t3_desc",
      "PurgeDebuffs": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "the cube",
      "name": "The Cube",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_chrono_time_wall" title="Time Wall" -->

## Time Wall

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_time_wall`
- Snapshot ID: `39772`
- Source-Dokument: `7071`
- Kurzinfo: Time Wall aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Time Wall`
- Payload Hash: `5740d073a735ce6aad71e444ef5c87d5ff0c864b342189ef182134f128ab9bb6`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.175008+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 5.08
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5.5
  },
  "DescKey": "citadel_ability_chrono_time_wall_desc",
  "HeroKey": "hero_chrono",
  "HeroName": "Paradox",
  "Info1": {
    "Alt": [
      {
        "Key": "TimeWallWidth",
        "Name": "Wall Width",
        "Type": "range",
        "Value": 8
      },
      {
        "Key": "TimeWallHeight",
        "Name": "Wall Height",
        "Value": 4
      },
      {
        "Key": "TimeWallTimeScaleFriendly",
        "Name": "Ally Bullet Speed",
        "Value": 2
      }
    ],
    "DescKey": "citadel_ability_chrono_time_wall_desc",
    "Main": {
      "Props": [
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Value": 0
        },
        {
          "Key": "MovementSlowPct",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 80
        },
        {
          "Key": "TimeScaleDuration",
          "Name": "Time Stop Duration",
          "Type": "duration",
          "Value": 0.5
        },
        {
          "Key": "FriendlyBulletDamageBonus",
          "Name": "Ally Weapon Damage",
          "Type": "bullet_damage",
          "Value": 30
        }
      ]
    }
  },
  "Key": "citadel_ability_chrono_time_wall",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Time Wall",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AuraEffectDuration": {
      "Name": null,
      "Value": 2
    },
    "TimeWallDepth": {
      "Name": null,
      "Value": 0.5
    },
    "TimeWallDepthVisualScale": {
      "Name": null,
      "Value": 0.16
    },
    "TimeWallFormationTime": {
      "Name": null,
      "Value": 0.5
    },
    "TimeWallTimeScale": {
      "Name": null,
      "Value": 0.0001
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityDuration": 3.5,
      "DescKey": "citadel_ability_chrono_time_wall_t1_desc",
      "TimeWallHeight": 1,
      "TimeWallWidth": 3
    },
    {
      "DebuffDuration": 2.3,
      "DescKey": "citadel_ability_chrono_time_wall_t2_desc",
      "FriendlyBulletDamageBonus": 35
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 2,
      "DescKey": "citadel_ability_chrono_time_wall_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "time wall",
      "name": "Time Wall",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="mirage_teleport" title="Traveler" -->

## Traveler

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_teleport`
- Snapshot ID: `39854`
- Source-Dokument: `7071`
- Kurzinfo: Traveler aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Traveler`
- Payload Hash: `e163a4bac2b91cb3254aeb1d93bf44a041e280429cfd357105dbec7e8da0ad48`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.334099+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 140.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "mirage_teleport_desc",
  "HeroKey": "hero_mirage",
  "HeroName": "Mirage",
  "Info1": {
    "Alt": [
      {
        "Key": "MovementSpeedBonusDuration",
        "Name": "Move Speed Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "InterruptCooldown",
        "Name": "Interrupt Cooldown",
        "Type": "duration",
        "Value": 4
      }
    ],
    "DescKey": "mirage_teleport_desc",
    "Main": {
      "Props": [
        {
          "Key": "TeleportCompletedTime",
          "Name": "Wait Time",
          "Type": "duration",
          "Value": 2
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Value": 0
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        },
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "bullet_armor_up",
          "Value": 0
        }
      ]
    }
  },
  "Key": "mirage_teleport",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Traveler",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Range": {
    "SearchRadius": {
      "Name": "Search Radius",
      "Type": "distance",
      "Value": 30
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BonusFireRate": 20,
      "BonusMoveSpeed": 3,
      "DescKey": "mirage_teleport_t1_desc",
      "MovementSpeedBonusDuration": 12
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 400
      },
      "DescKey": "mirage_teleport_t2_desc"
    },
    {
      "AbilityCooldown": -70
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "traveler",
      "name": "Traveler",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="operative_umbrella_maneuver" title="Umbrella Maneuver" -->

## Umbrella Maneuver

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `operative_umbrella_maneuver`
- Snapshot ID: `39864`
- Source-Dokument: `7071`
- Kurzinfo: Umbrella Maneuver aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Umbrella Maneuver`
- Payload Hash: `919cacd3d243924eb765e0ca40277a09f94eaede904e05fbac3842123fb5c84b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.353234+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 32.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "operative_umbrella_maneuver_desc",
  "HeroKey": "hero_operative",
  "HeroName": "Raven",
  "Info1": {
    "Alt": [],
    "DescKey": "operative_umbrella_maneuver_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.395
          },
          "Type": "tech_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "operative_umbrella_maneuver",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Umbrella Maneuver",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ActivateTime": {
      "Name": null,
      "Value": 0.2
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 3.81
    },
    "BackwardsVelocity": {
      "Name": null,
      "Value": 13.0
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1.524
    },
    "TimeBeforeProjectileLaunch": {
      "Name": null,
      "Value": 1.25
    },
    "UpImpulse": {
      "Name": null,
      "Value": 15.0
    }
  },
  "Range": {
    "ExplodeRadius": {
      "Name": "Explosion Radius",
      "Type": "distance",
      "Value": 5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -14
    },
    {
      "Damage": 50
    },
    {
      "AbilityCooldown": 0,
      "DescKey": "operative_umbrella_maneuver_t3_desc"
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "umbrella maneuver",
      "name": "Umbrella Maneuver",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bomber_ability02" title="Unknown(ability_bomber_ability02)" -->

## Unknown(ability_bomber_ability02)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bomber_ability02`
- Snapshot ID: `39764`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_bomber_ability02) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_bomber_ability02)`
- Payload Hash: `be99bc8f7b2d1c278456a270103632b484c0def1441a38ffcf9367f6959dc996`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.159951+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_bomber",
  "HeroName": "Bomber",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_bomber_ability02",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_bomber_ability02)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bomber_ability03" title="Unknown(ability_bomber_ability03)" -->

## Unknown(ability_bomber_ability03)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bomber_ability03`
- Snapshot ID: `39765`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_bomber_ability03) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_bomber_ability03)`
- Payload Hash: `8b91565580dd0935a2af45207956af8c38eb849dfdcc48d1ff110d4738d891c4`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.161636+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_bomber",
  "HeroName": "Bomber",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_bomber_ability03",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_bomber_ability03)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_bomber_ult" title="Unknown(ability_bomber_ult)" -->

## Unknown(ability_bomber_ult)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bomber_ult`
- Snapshot ID: `39766`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_bomber_ult) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_bomber_ult)`
- Payload Hash: `af589389296320f03c9ff3846634fee6f4d4aef53adea6c0a0a454f42772d37b`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.163378+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 127.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_bomber",
  "HeroName": "Bomber",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCooldown",
          "Name": "Cooldown",
          "Type": "cooldown",
          "Value": 127.0
        }
      ]
    }
  },
  "Key": "ability_bomber_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_bomber_ult)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_charged_bomb" title="Unknown(ability_charged_bomb)" -->

## Unknown(ability_charged_bomb)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_charged_bomb`
- Snapshot ID: `39763`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_charged_bomb) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_charged_bomb)`
- Payload Hash: `a7986be0ec495cc2eba9ddf3bb69bec23bc6c2547d213784f02a354939d40368`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.157884+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 10.5
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "MaxDamage": {
      "Name": "Max Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 1.16064
      },
      "Type": "tech_damage",
      "Value": 100
    }
  },
  "HeroKey": "hero_bomber",
  "HeroName": "Bomber",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_charged_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_charged_bomb)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlastJumpVelocity": {
      "Name": null,
      "Value": 25
    },
    "BlastJumpVelocityCrouch": {
      "Name": null,
      "Value": 30
    },
    "BlastJumpVelocityGround": {
      "Name": null,
      "Value": 20
    },
    "MaxChargeTime": {
      "Name": null,
      "Value": 2.0
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 7
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fortuna_ability01" title="Unknown(ability_fortuna_ability01)" -->

## Unknown(ability_fortuna_ability01)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ability01`
- Snapshot ID: `39799`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_fortuna_ability01) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_fortuna_ability01)`
- Payload Hash: `88d74530defefbb0d1bb431eb530eec689b8540fd44f8ec3f504242215d07029`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.227756+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_fortuna",
  "HeroName": "Fortuna",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_fortuna_ability01",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_fortuna_ability01)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fortuna_ability02" title="Unknown(ability_fortuna_ability02)" -->

## Unknown(ability_fortuna_ability02)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ability02`
- Snapshot ID: `39800`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_fortuna_ability02) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_fortuna_ability02)`
- Payload Hash: `4b677a332df39763253a1cc00446e4f53e4e9af4366edc00f07c6f52d44b8200`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.229523+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_fortuna",
  "HeroName": "Fortuna",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_fortuna_ability02",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_fortuna_ability02)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fortuna_ability03" title="Unknown(ability_fortuna_ability03)" -->

## Unknown(ability_fortuna_ability03)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ability03`
- Snapshot ID: `39801`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_fortuna_ability03) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_fortuna_ability03)`
- Payload Hash: `f798582c2c00a181bba785a7475dcae63774be59556202087ca188b2a3a64903`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.231347+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_fortuna",
  "HeroName": "Fortuna",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_fortuna_ability03",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_fortuna_ability03)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_fortuna_ult" title="Unknown(ability_fortuna_ult)" -->

## Unknown(ability_fortuna_ult)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fortuna_ult`
- Snapshot ID: `39802`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_fortuna_ult) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_fortuna_ult)`
- Payload Hash: `cd8d98edf9fb2de35b2d5d21cb51d73c560b4e3bafba2c40133f6e79289f5ee5`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.233199+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 127.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_fortuna",
  "HeroName": "Fortuna",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCooldown",
          "Name": "Cooldown",
          "Type": "cooldown",
          "Value": 127.0
        }
      ]
    }
  },
  "Key": "ability_fortuna_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_fortuna_ult)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_graf_ability01" title="Unknown(ability_graf_ability01)" -->

## Unknown(ability_graf_ability01)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ability01`
- Snapshot ID: `39815`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_graf_ability01) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_graf_ability01)`
- Payload Hash: `a3b08ecd878e68e9571d6a77a0a08a7e8988b86ddf6fb8e4f77e2389509e64be`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.257909+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_graf",
  "HeroName": "Graf",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_graf_ability01",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_graf_ability01)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_graf_ability02" title="Unknown(ability_graf_ability02)" -->

## Unknown(ability_graf_ability02)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ability02`
- Snapshot ID: `39816`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_graf_ability02) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_graf_ability02)`
- Payload Hash: `0e137f8e257757a5b830d47bec9a01b701e4d93be87e13d90ddf3139ea259c32`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.259755+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_graf",
  "HeroName": "Graf",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_graf_ability02",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_graf_ability02)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_graf_ability03" title="Unknown(ability_graf_ability03)" -->

## Unknown(ability_graf_ability03)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ability03`
- Snapshot ID: `39817`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_graf_ability03) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_graf_ability03)`
- Payload Hash: `be7c29bc8af4e285afc99509a22c13005079e45267ff1ec5282eccaf956e42f0`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.261481+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_graf",
  "HeroName": "Graf",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_graf_ability03",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_graf_ability03)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_graf_ult" title="Unknown(ability_graf_ult)" -->

## Unknown(ability_graf_ult)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_graf_ult`
- Snapshot ID: `39818`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_graf_ult) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_graf_ult)`
- Payload Hash: `abd6b2fd2f60dc66927f6c7a3c7bca2ca2854282b5b32e82a5dfeea0662c1049`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.263174+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 127.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_graf",
  "HeroName": "Graf",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCooldown",
          "Name": "Cooldown",
          "Type": "cooldown",
          "Value": 127.0
        }
      ]
    }
  },
  "Key": "ability_graf_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_graf_ult)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_haunt" title="Unknown(ability_haunt)" -->

## Unknown(ability_haunt)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_haunt`
- Snapshot ID: `39921`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_haunt) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_haunt)`
- Payload Hash: `fef8fea87652fc76b03dbbed9e3425504ae16b5f5657556e85e6ac9faff61b21`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.463291+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_vandal",
  "HeroName": "Vandal",
  "Info1": {
    "Alt": [
      {
        "Key": "BuildUpBulletPercentPerHit",
        "Name": "Buildup Per Bullet",
        "Value": 8.33
      },
      {
        "Key": "CritBuildup",
        "Name": "Buildup Per Headshot",
        "Value": 16
      },
      {
        "Key": "BuildUpDuration",
        "Type": "duration",
        "Value": 0.1
      }
    ],
    "DescKey": "ability_afterburn_desc",
    "Main": {
      "Props": [
        {
          "Key": "DPS",
          "Name": "Damage Per Second",
          "Scale": {
            "Type": "spirit",
            "Value": 0.465
          },
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "BurnDuration",
          "Name": "Burn Duration",
          "Type": "duration",
          "Value": 0.3
        }
      ]
    }
  },
  "Key": "ability_haunt",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_haunt)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AfterburnSpiritDamageReduction": -30
    },
    {
      "BurnDuration": 1
    },
    {
      "DPS": 30
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_shieldguy_ability01" title="Unknown(ability_shieldguy_ability01)" -->

## Unknown(ability_shieldguy_ability01)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ability01`
- Snapshot ID: `39879`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_shieldguy_ability01) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_shieldguy_ability01)`
- Payload Hash: `953cc3cfa7712aed9569683109ac2bb10453cbcfdc504b9e92a11c2b5fae7721`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.382731+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_shieldguy",
  "HeroName": "Shield Guy",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_shieldguy_ability01",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_shieldguy_ability01)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_shieldguy_ability02" title="Unknown(ability_shieldguy_ability02)" -->

## Unknown(ability_shieldguy_ability02)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ability02`
- Snapshot ID: `39880`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_shieldguy_ability02) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_shieldguy_ability02)`
- Payload Hash: `7c2bbfd0da068b8688835b981b432df2abc3faa46bfed7482a26c3bb64a6c5f4`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.384451+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_shieldguy",
  "HeroName": "Shield Guy",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_shieldguy_ability02",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_shieldguy_ability02)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_shieldguy_ability03" title="Unknown(ability_shieldguy_ability03)" -->

## Unknown(ability_shieldguy_ability03)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ability03`
- Snapshot ID: `39881`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_shieldguy_ability03) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_shieldguy_ability03)`
- Payload Hash: `97f2620e0f05dc7541c9449ad793cfab36fb3bab2ac77295afe6af4e9dccfada`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.386402+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_shieldguy",
  "HeroName": "Shield Guy",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_shieldguy_ability03",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_shieldguy_ability03)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_shieldguy_ult" title="Unknown(ability_shieldguy_ult)" -->

## Unknown(ability_shieldguy_ult)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_shieldguy_ult`
- Snapshot ID: `39882`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_shieldguy_ult) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_shieldguy_ult)`
- Payload Hash: `6c2b1de379bbc77ebe4f7b1a443ee3bd5b68ef09c886005cd13c51df63467874`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.388033+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 127.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_shieldguy",
  "HeroName": "Shield Guy",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCooldown",
          "Name": "Cooldown",
          "Type": "cooldown",
          "Value": 127.0
        }
      ]
    }
  },
  "Key": "ability_shieldguy_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_shieldguy_ult)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_skyrunner_magic_beam" title="Unknown(ability_skyrunner_magic_beam)" -->

## Unknown(ability_skyrunner_magic_beam)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_magic_beam`
- Snapshot ID: `39888`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_skyrunner_magic_beam) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_skyrunner_magic_beam)`
- Payload Hash: `2283c710f911fee0f20a6f289d089dafa481d15f97edfc7833cf0fba67fd47b9`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.399084+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "HeroKey": "hero_skyrunner",
  "HeroName": "Skyrunner",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_skyrunner_magic_beam",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_skyrunner_magic_beam)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BlockerScaleFactor": {
      "Name": null,
      "Value": 4
    },
    "Damage": {
      "Name": "Damage",
      "Value": 120
    },
    "GrowTime": {
      "Name": null,
      "Value": 0.2
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 4
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_swan_featherboomerang" title="Unknown(ability_swan_featherboomerang)" -->

## Unknown(ability_swan_featherboomerang)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_swan_featherboomerang`
- Snapshot ID: `39895`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_swan_featherboomerang) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_swan_featherboomerang)`
- Payload Hash: `3cad25972cb819dc6ff46d339944e429ee5f1af0ca291b1248b08d30140a95a2`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.413918+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.05
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 8
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_swan",
  "HeroName": "Swan",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_swan_featherboomerang",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_swan_featherboomerang)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusDamage": {
      "Name": "Bonus Damage",
      "Value": 170
    },
    "Damage": {
      "Name": "Damage",
      "Value": 30
    },
    "ProjectileArrivalTime": {
      "Name": null,
      "Value": 1
    },
    "ProjectileForwardSpeed": {
      "Name": null,
      "Value": 800
    },
    "ProjectileSideFrequency": {
      "Name": null,
      "Value": 2
    },
    "ProjectileSideSpeed": {
      "Name": null,
      "Value": 300
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 4
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_swan_ult" title="Unknown(ability_swan_ult)" -->

## Unknown(ability_swan_ult)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_swan_ult`
- Snapshot ID: `39898`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_swan_ult) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_swan_ult)`
- Payload Hash: `7c4907106a8828f4256434a39a52e7e901dcc96bd2a28905c52b9a7181eb5f4a`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.419636+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 127.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_swan",
  "HeroName": "Swan",
  "Info1": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "AbilityCooldown",
          "Name": "Cooldown",
          "Type": "cooldown",
          "Value": 127.0
        }
      ]
    }
  },
  "Key": "ability_swan_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_swan_ult)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_vandal_pillar" title="Unknown(ability_vandal_pillar)" -->

## Unknown(ability_vandal_pillar)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vandal_pillar`
- Snapshot ID: `39920`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(ability_vandal_pillar) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(ability_vandal_pillar)`
- Payload Hash: `34d9fc4afd619dc0fd47c4403be5a3e27c8bb7c8acc7932b52c1e8123743fa74`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.461423+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 60
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_vandal",
  "HeroName": "Vandal",
  "Info1": {
    "Alt": [
      {
        "Key": "PetrifyDuration",
        "Name": "Petrify Duration",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "PetrifyDamageBreakThreshold",
        "Name": "Petrify Damage Block",
        "Type": "health",
        "Value": 200
      }
    ],
    "DescKey": "ability_viper_ult_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0695
          },
          "Type": "tech_damage",
          "Value": 200
        }
      ]
    }
  },
  "Key": "ability_vandal_pillar",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(ability_vandal_pillar)",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.15
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "HalfHeight": {
      "Name": null,
      "Value": 6
    },
    "PreDetonateDuration": {
      "Name": null,
      "Value": 0.6
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 4
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "PetrifyDuration": 1.5
    },
    {
      "Radius": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_vandal_overflow" title="Unknown(citadel_ability_vandal_overflow)" -->

## Unknown(citadel_ability_vandal_overflow)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_vandal_overflow`
- Snapshot ID: `39922`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(citadel_ability_vandal_overflow) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(citadel_ability_vandal_overflow)`
- Payload Hash: `bf3c0a10ce6a8ec2870da2c9749a747a9546dcc5ee2149dd20f592fa91978ad3`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.465082+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.6
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 1.25
  },
  "HeroKey": "hero_vandal",
  "HeroName": "Vandal",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "citadel_ability_vandal_overflow",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Unknown(citadel_ability_vandal_overflow)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.5
    },
    "LiftHeight": {
      "Name": null,
      "Value": 120
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -28.0
    },
    {
      "AbilityDuration": 0.5
    },
    {
      "AbilityUnitTargetLimit": 5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_vandal_surge" title="Unknown(citadel_ability_vandal_surge)" -->

## Unknown(citadel_ability_vandal_surge)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_vandal_surge`
- Snapshot ID: `39919`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(citadel_ability_vandal_surge) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(citadel_ability_vandal_surge)`
- Payload Hash: `dc982be415d7f2caf1d9260a60923c36d44f1c9a85f56c184e05a4aca39495e2`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.459605+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.6
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 1.25
  },
  "HeroKey": "hero_vandal",
  "HeroName": "Vandal",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_psychic_lift_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 100
        }
      ]
    }
  },
  "Key": "citadel_ability_vandal_surge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Unknown(citadel_ability_vandal_surge)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.5
    },
    "LiftHeight": {
      "Name": null,
      "Value": 120
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -28.0
    },
    {
      "AbilityDuration": 0.5
    },
    {
      "AbilityUnitTargetLimit": 5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="gunslinger_knockbackblast" title="Unknown(gunslinger_knockbackblast)" -->

## Unknown(gunslinger_knockbackblast)

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `gunslinger_knockbackblast`
- Snapshot ID: `39820`
- Source-Dokument: `7071`
- Kurzinfo: Unknown(gunslinger_knockbackblast) aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unknown(gunslinger_knockbackblast)`
- Payload Hash: `cba0ed27c8c63038d42c78d86199811ed47bcacf0d351ad8d0558eeb44faef7d`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.266587+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 8
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "HeroKey": "hero_gunslinger",
  "HeroName": "Gunslinger",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityCastDelay",
        "Name": "Cast Delay",
        "Type": "cast",
        "Value": 0.1
      },
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 0
      }
    ],
    "DescKey": "ability_gunslinger_salvo_desc",
    "Main": {
      "Props": []
    }
  },
  "Key": "gunslinger_knockbackblast",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Unknown(gunslinger_knockbackblast)",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ConeAngle": {
      "Name": "Cone Angle",
      "Value": 45
    },
    "Damage": {
      "Name": "Damage",
      "Value": 120
    },
    "EnemyShoveForce": {
      "Name": null,
      "Value": 800
    },
    "SelfShoveForce": {
      "Name": null,
      "Value": 200
    },
    "SlowDuration": {
      "Name": "Slow Duration",
      "Value": 3
    },
    "SlowPercent": {
      "Name": "Move Speed",
      "Value": 30
    },
    "StunDuration": {
      "Name": "Stun Duration",
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_magician_magicbolt" title="Vexing Bolt" -->

## Vexing Bolt

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_magicbolt`
- Snapshot ID: `39847`
- Source-Dokument: `7071`
- Kurzinfo: Vexing Bolt aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vexing Bolt`
- Payload Hash: `0117744ec23603c2dd61657ce4d14a6b0b1f7c5598fac5496d742090aaba4c18`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.320636+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 500
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 24
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 3
  },
  "DescKey": "ability_magician_magicbolt_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxDamageTime",
        "Name": "Time for Max Damage",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_magician_magicbolt_desc",
    "Main": {
      "Props": [
        {
          "Key": "MinDamage",
          "Name": "Minimum Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.93
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "MaxDamage",
          "Name": "Max Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.86
          },
          "Type": "tech_damage",
          "Value": 120
        },
        {
          "Key": "CloneDamagePercentage",
          "Name": "Assistant Damage",
          "Type": "tech_damage",
          "Value": 50.0
        }
      ]
    }
  },
  "Key": "ability_magician_magicbolt",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Vexing Bolt",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CloneBoltDelay": {
      "Name": null,
      "Value": 0.1
    },
    "InitialProjectileVelocity": {
      "Name": null,
      "Value": 800
    },
    "ProjectileLifetime": {
      "Name": null,
      "Value": 4
    },
    "ProjectileRedirectCount": {
      "Name": "Max Redirects",
      "Value": 1
    },
    "RedirectVelocity": {
      "Name": null,
      "Value": 1500
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 3.25
  },
  "Slot": "1",
  "Upgrades": [
    {
      "DebuffDuration": 5,
      "DescKey": "ability_magician_magicbolt_t1_desc",
      "FireRateSlow": 25
    },
    {
      "AbilityCooldown": -13
    },
    {
      "CloneDamagePercentage": 50.0,
      "DescKey": "ability_magician_magicbolt_t3_desc",
      "MaxDamage": 126.0
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "vexing bolt",
      "name": "Vexing Bolt",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="ability_warden_high_alert" title="Willpower" -->

## Willpower

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_high_alert`
- Snapshot ID: `39932`
- Source-Dokument: `7071`
- Kurzinfo: Willpower aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Willpower`
- Payload Hash: `e372968dc298032842d827f77753aaf7202d67f226d5c059900ef4885f00dd26`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.484867+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 40
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_warden_high_alert_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_warden_high_alert_desc",
    "Main": {
      "Props": [
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 0.8
          },
          "Type": "combat_barrier",
          "Value": 125
        },
        {
          "Key": "MoveSpeedBonusPct",
          "Name": "Move Speed bonus",
          "Type": "move_speed",
          "Value": 15
        },
        {
          "Key": "StatusResistancePercent",
          "Name": "Debuff Resist",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_warden_high_alert",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Willpower",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "MoveSpeedBonusPct": 20
    },
    {
      "AbilityCooldown": -24,
      "AbilityDuration": 2,
      "DescKey": "ability_warden_high_alert_t2_desc"
    },
    {
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 2.7
        },
        "Value": 0
      },
      "DescKey": "ability_warden_high_alert_t3_desc",
      "StatusResistancePercent": 40
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "willpower",
      "name": "Willpower",
      "type": "ability"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="ability_card" external_id="citadel_ability_wrecker_bouldergrenade" title="Wrecking Ball" -->

## Wrecking Ball

### Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_wrecker_bouldergrenade`
- Snapshot ID: `39947`
- Source-Dokument: `7071`
- Kurzinfo: Wrecking Ball aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Wrecking Ball`
- Payload Hash: `32ee8207f4dee34a5009ba71703f6b34422d1ae0519ec8fd9c9b728d3ac278b7`
- Source Content Hash: `1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json`
- Fetched At: `2026-07-09T19:36:24.512708+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 50
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 31
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "citadel_ability_wrecker_bouldergrenade_desc",
  "HeroKey": "hero_wrecker",
  "HeroName": "Wrecker",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 1.2
      }
    ],
    "DescKey": "citadel_ability_wrecker_bouldergrenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.302
          },
          "Type": "tech_damage",
          "Value": 150
        },
        {
          "Key": "ExplosionDamage",
          "Name": "Explosion Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.744
          },
          "Value": 80
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Type": "duration",
          "Value": 1
        }
      ]
    }
  },
  "Key": "citadel_ability_wrecker_bouldergrenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 2.5
    }
  },
  "Name": "Wrecking Ball",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ExplosionPushForce": {
      "Name": null,
      "Value": 1200
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 7
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -7.5
    },
    {
      "Damage": 50
    },
    {
      "StunDuration": 0.5
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
    "generated_by": "deadbot",
    "repo": "deadlock-wiki/deadlock-data",
    "repo_url": "https://github.com/deadlock-wiki/deadlock-data.git",
    "source_origin": "deadlock-wiki/deadlock-data",
    "source_trust": "trusted",
    "version": {
      "ClientVersion": "6624",
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
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "wrecking ball",
      "name": "Wrecking Ball",
      "type": "ability"
    }
  ]
}
````

