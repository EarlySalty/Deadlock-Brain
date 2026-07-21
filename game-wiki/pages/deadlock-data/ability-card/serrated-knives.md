---
title: "Serrated Knives"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_shiv_dagger"
canonical_name: "Serrated Knives"
snapshot_id: 39883
source_document_id: 7071
payload_hash: "6932fb318260d979b11d62eac86abef99e0c8ec01a2395e60d553ff5c2ab7245"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.389741+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Serrated Knives

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dagger`
- Snapshot ID: `39883`
- Source-Dokument: `7071`
- Kurzinfo: Serrated Knives aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
