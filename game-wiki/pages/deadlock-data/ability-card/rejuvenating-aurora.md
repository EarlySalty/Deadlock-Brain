---
title: "Rejuvenating Aurora"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_nikuman"
canonical_name: "Rejuvenating Aurora"
snapshot_id: 39785
source_document_id: 7071
payload_hash: "37808580b7054a75c3af8501120fdcb362b7fe9e7f01ade23571af2b6c4b6fb8"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.199907+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Rejuvenating Aurora

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_nikuman`
- Snapshot ID: `39785`
- Source-Dokument: `7071`
- Kurzinfo: Rejuvenating Aurora aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
