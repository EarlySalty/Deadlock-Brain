---
title: "The Cube"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "viscous_restorative_goo"
canonical_name: "The Cube"
snapshot_id: 39928
source_document_id: 7071
payload_hash: "6fdf9c8812b3cf53b737cb7fd3b06bb4d720c4322a91a536d6bfa636485e4cd0"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.477163+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# The Cube

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_restorative_goo`
- Snapshot ID: `39928`
- Source-Dokument: `7071`
- Kurzinfo: The Cube aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
