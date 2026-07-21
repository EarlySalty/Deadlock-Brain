---
title: "Captivating Read"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_bookworm_aoemagic"
canonical_name: "Captivating Read"
snapshot_id: 39769
source_document_id: 7071
payload_hash: "d5f2a542ea7f8b1b6ca7e257a0a23cdc9d39d615d72300e040449166fb083ad2"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.169264+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Captivating Read

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_aoemagic`
- Snapshot ID: `39769`
- Source-Dokument: `7071`
- Kurzinfo: Captivating Read aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_bookworm_aoemagic_desc",
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 45
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0.5
      }
    ],
    "DescKey": "ability_bookworm_aoemagic_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 90
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "StatusEffect": "Immobilize",
          "Title": "On Hit:",
          "Value": 1.0
        },
        {
          "Key": "TechArmorDamageReduction",
          "Name": "Spirit Resist",
          "Title": "On Hit:",
          "Type": "tech_armor_down",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_bookworm_aoemagic",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Captivating Read",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "DetonationDelay": {
      "Name": null,
      "Value": 1.25
    },
    "Height": {
      "Name": null,
      "Value": 8
    }
  },
  "Radius": {
    "Name": "Radius",
    "Value": 7.5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1
    },
    {
      "DebuffDuration": 6,
      "DescKey": "ability_bookworm_aoemagic_t3_desc",
      "Radius": 1,
      "TechArmorDamageReduction": -18
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "captivating read",
      "name": "Captivating Read",
      "type": "ability"
    }
  ]
}
````
