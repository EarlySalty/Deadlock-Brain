---
title: "Lethal Venom"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_viper_venom"
canonical_name: "Lethal Venom"
snapshot_id: 39924
source_document_id: 7071
payload_hash: "4e7793b3d15dbe40016d5d2f32c7e59646c88f313efed0dd3bb66acb5c114766"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.469020+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Lethal Venom

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_viper_venom`
- Snapshot ID: `39924`
- Source-Dokument: `7071`
- Kurzinfo: Lethal Venom aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "power_increase",
      "Value": 0.2
    },
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_viper_venom_desc",
  "HeroKey": "hero_viper",
  "HeroName": "Vyper",
  "Info1": {
    "Alt": [
      {
        "Key": "VenomMaxDamageHealthPercentage",
        "Name": "Health for Max Damage",
        "Value": 30
      },
      {
        "Key": "VenomDuration",
        "Name": "Venom Buildup Duration",
        "Value": 3
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Name": "Healing Reduction",
        "Value": 0
      }
    ],
    "DescKey": "ability_viper_venom_desc",
    "Main": {
      "Props": [
        {
          "Key": "VenomMinDamage",
          "Name": "Minimum Venom Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 20
        },
        {
          "Key": "VenomMaxDamage",
          "Name": "Max Venom Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.79
          },
          "Type": "tech_damage",
          "Value": 140
        },
        {
          "Key": "VenomMissingHealthDamagePercentage",
          "Name": "Missing Health Damage",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_viper_venom",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Lethal Venom",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuildUpDuration": {
      "Name": null,
      "Value": 5
    },
    "VenomBuildupPerShot": {
      "Name": null,
      "Value": 1
    },
    "VenomMinDamageHealthPercentage": {
      "Name": null,
      "Value": 100
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "VenomMaxDamage": 31.5
    },
    {
      "AbilityCooldown": -12,
      "DescKey": "ability_viper_venom_t2_desc",
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
    },
    {
      "BuildUpPerShot": 4.5,
      "DescKey": "ability_viper_venom_t3_desc"
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
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "lethal venom",
      "name": "Lethal Venom",
      "type": "ability"
    }
  ]
}
````
