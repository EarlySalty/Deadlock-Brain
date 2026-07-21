---
title: "Binding Word"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_warden_lock_down"
canonical_name: "Binding Word"
snapshot_id: 39933
source_document_id: 7071
payload_hash: "24d05faafd297a2fe8350d07bb5347788490b9eeeed3298f40a35c78091f1250"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.487027+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Binding Word

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_lock_down`
- Snapshot ID: `39933`
- Source-Dokument: `7071`
- Kurzinfo: Binding Word aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34
  },
  "DescKey": "ability_warden_lock_down_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [
      {
        "Key": "EscapeTime",
        "Name": "Escape Time",
        "Type": "duration",
        "Value": 2.8
      },
      {
        "Key": "EscapeRange",
        "Name": "Escape Range",
        "Type": "distance",
        "Value": 20
      }
    ],
    "DescKey": "ability_warden_lock_down_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.437344
          },
          "Type": "tech_damage",
          "Value": 110
        },
        {
          "Key": "ImmobilizeDuration",
          "Name": "Immobilize Duration",
          "Type": "duration",
          "Value": 1.75
        }
      ]
    }
  },
  "Key": "ability_warden_lock_down",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Binding Word",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AdditionalTargetRadius": {
      "Name": null,
      "Value": 20
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "BulletArmorReduction": 20,
      "BulletArmorReductionDuration": 5,
      "DescKey": "ability_warden_lock_down_t1_desc"
    },
    {
      "DescKey": "ability_warden_lock_down_t2_desc",
      "ImmobilizeDuration": 0.75
    },
    {
      "AbilityCooldown": -14,
      "DescKey": "ability_warden_lock_down_t3_desc",
      "SilenceDebuff": 1
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
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "binding word",
      "name": "Binding Word",
      "type": "ability"
    }
  ]
}
````
