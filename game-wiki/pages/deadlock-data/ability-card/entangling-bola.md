---
title: "Entangling Bola"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_netshot"
canonical_name: "Entangling Bola"
snapshot_id: 39937
source_document_id: 7071
payload_hash: "3d7318b4fe05b8d131936beb96ee175cc15f91389715bbc762c1012572d5e89b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.494433+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Entangling Bola

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_netshot`
- Snapshot ID: `39937`
- Source-Dokument: `7071`
- Kurzinfo: Entangling Bola aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.24
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 23
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_werewolf_netshot_desc",
  "HeroKey": "hero_werewolf",
  "HeroName": "Silver",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_werewolf_netshot_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Type": "tech_damage",
          "Value": 40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "slow",
          "Value": 20
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 1.5
        }
      ]
    }
  },
  "Key": "ability_werewolf_netshot",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Entangling Bola",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "SlowPercent": 25
    },
    {
      "AbilityCooldown": -8
    },
    {
      "DebuffDuration": 0.75,
      "DescKey": "ability_werewolf_netshot_t3_desc",
      "RicochetCount": 2,
      "RicochetRange": 15
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
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "entangling bola",
      "name": "Entangling Bola",
      "type": "ability"
    }
  ]
}
````
