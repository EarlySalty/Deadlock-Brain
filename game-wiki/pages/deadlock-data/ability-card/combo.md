---
title: "Combo"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_ult_combo"
canonical_name: "Combo"
snapshot_id: 39842
source_document_id: 7071
payload_hash: "97edb00806bf74ccdd6a8f7e92a76828d7f85d020a339db6c16253d072db1dc6"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.309914+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Combo

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_ult_combo`
- Snapshot ID: `39842`
- Source-Dokument: `7071`
- Kurzinfo: Combo aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

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
