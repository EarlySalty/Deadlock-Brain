---
title: "Grasping Hands"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_necro_zombiewall"
canonical_name: "Grasping Hands"
snapshot_id: 39860
source_document_id: 7071
payload_hash: "5388cfdba9b2e7fa60688a34347066c32486cbfdf16c40f9d73c4169cd9220c6"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.346134+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Grasping Hands

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_necro_zombiewall`
- Snapshot ID: `39860`
- Source-Dokument: `7071`
- Kurzinfo: Grasping Hands aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
