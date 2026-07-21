---
title: "Sanguine Retreat"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_vampirebat_batblink"
canonical_name: "Sanguine Retreat"
snapshot_id: 39916
source_document_id: 7071
payload_hash: "c3795f5d7bd28110484fb4a0c7c280d37b1afda238ec2525b9ce00b74a5a7106"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.453426+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Sanguine Retreat

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batblink`
- Snapshot ID: `39916`
- Source-Dokument: `7071`
- Kurzinfo: Sanguine Retreat aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
