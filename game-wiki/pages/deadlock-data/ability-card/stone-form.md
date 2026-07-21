---
title: "Stone Form"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_tengu_stone_form"
canonical_name: "Stone Form"
snapshot_id: 39905
source_document_id: 7071
payload_hash: "d534f10907af2dbb139357eb1707d715d41b1d58a19846283be1384dfda97b56"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.431292+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Stone Form

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tengu_stone_form`
- Snapshot ID: `39905`
- Source-Dokument: `7071`
- Kurzinfo: Stone Form aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
