---
title: "Skipshot"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_boho_bouncyprojectile"
canonical_name: "Skipshot"
snapshot_id: 39760
source_document_id: 7071
payload_hash: "5914cb4cb5df4b20f3bf0f69d9525db2f4ea52b5b50531a908869d59bbe8abc5"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.151303+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Skipshot

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_boho_bouncyprojectile`
- Snapshot ID: `39760`
- Source-Dokument: `7071`
- Kurzinfo: Skipshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 14
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 15
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 7
  },
  "DescKey": "ability_boho_bouncyprojectile_desc",
  "HeroKey": "hero_boho",
  "HeroName": "Boho",
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReductionPercentagePerHit",
        "Type": "cooldown",
        "Value": 15
      }
    ],
    "DescKey": "ability_boho_bouncyprojectile_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.4
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "BounceCount",
          "Name": "Shield Bounces",
          "Value": 3
        },
        {
          "Key": "BounceRadius",
          "Name": "Bounce Range",
          "Type": "distance",
          "Value": 18
        }
      ]
    }
  },
  "Key": "ability_boho_bouncyprojectile",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Skipshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -2
    },
    {
      "Damage": 18.0
    },
    {
      "BounceCount": 2
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
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "skipshot",
      "name": "Skipshot",
      "type": "ability"
    }
  ]
}
````
