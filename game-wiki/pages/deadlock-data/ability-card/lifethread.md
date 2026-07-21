---
title: "Lifethread"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_skyrunner_swingline"
canonical_name: "Lifethread"
snapshot_id: 39887
source_document_id: 7071
payload_hash: "0bce603281c37209bc97bb35fecfe2f4afda2c099c2e1f5e505f99dba9a77e7c"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.397182+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Lifethread

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_skyrunner_swingline`
- Snapshot ID: `39887`
- Source-Dokument: `7071`
- Kurzinfo: Lifethread aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
