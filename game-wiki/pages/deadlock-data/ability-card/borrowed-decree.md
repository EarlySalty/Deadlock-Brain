---
title: "Borrowed Decree"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_necro_gravestone"
canonical_name: "Borrowed Decree"
snapshot_id: 39862
source_document_id: 7071
payload_hash: "8fadc6c4628aade2933b04872ebd3544b7d8b71d42f6dfb2b2608cd3a2193982"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.349797+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Borrowed Decree

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_gravestone`
- Snapshot ID: `39862`
- Source-Dokument: `7071`
- Kurzinfo: Borrowed Decree aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
