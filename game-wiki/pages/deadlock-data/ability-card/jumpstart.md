---
title: "Jumpstart"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_frank_selfzap"
canonical_name: "Jumpstart"
snapshot_id: 39804
source_document_id: 7071
payload_hash: "ec2506c95117edaab7cd9c7696bc9f32ffb75f20d2f9b41525856e0a0224322c"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.236830+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Jumpstart

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_frank_selfzap`
- Snapshot ID: `39804`
- Source-Dokument: `7071`
- Kurzinfo: Jumpstart aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.35
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 30
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4.5
  },
  "DescKey": "ability_frank_selfzap_desc",
  "HeroKey": "hero_frank",
  "HeroName": "Victor",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_frank_selfzap_desc",
    "Main": {
      "Props": [
        {
          "Key": "CurrentHealthPercentDamage",
          "Name": "Current Health",
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "TotalHealthRegen",
          "Name": "Total HP Regen",
          "Scale": {
            "Type": "spirit",
            "Value": 1.2
          },
          "Title": "On Buff:",
          "Type": "healing",
          "Value": 100
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "On Buff:",
          "Type": "move_speed",
          "Value": 3
        },
        {
          "Key": "StatusResistancePercent",
          "Name": "Debuff Resist",
          "Title": "On Buff:",
          "Type": "duration",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_frank_selfzap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Jumpstart",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusMoveSpeed": 3
    },
    {
      "AbilityCooldown": -8,
      "DescKey": "ability_frank_selfzap_t2_desc",
      "TotalHealthRegen": 70
    },
    {
      "AbilityCharges": 1,
      "DescKey": "ability_frank_selfzap_t3_desc",
      "StatusResistancePercent": 50,
      "TotalHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.9
        },
        "Value": 0
      }
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
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "jumpstart",
      "name": "Jumpstart",
      "type": "ability"
    }
  ]
}
````
