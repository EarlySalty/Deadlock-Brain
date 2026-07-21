---
title: "Traveler"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "mirage_teleport"
canonical_name: "Traveler"
snapshot_id: 39854
source_document_id: 7071
payload_hash: "e163a4bac2b91cb3254aeb1d93bf44a041e280429cfd357105dbec7e8da0ad48"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.334099+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Traveler

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_teleport`
- Snapshot ID: `39854`
- Source-Dokument: `7071`
- Kurzinfo: Traveler aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
