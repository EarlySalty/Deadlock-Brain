---
title: "Rabbit Hex"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_magician_animalhexarea"
canonical_name: "Rabbit Hex"
snapshot_id: 39849
source_document_id: 7071
payload_hash: "72177554d14fc09c1d5e795e0defcc50073c70b2ae3e4baf90cd1c7b565da230"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.324438+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Rabbit Hex

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_animalhexarea`
- Snapshot ID: `39849`
- Source-Dokument: `7071`
- Kurzinfo: Rabbit Hex aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 24
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_magician_animalhexarea_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [
      {
        "Key": "MoveSpeedBonusPct",
        "Name": "Move Speed bonus",
        "Type": "move_speed",
        "Value": 36
      },
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 6.5
      }
    ],
    "DescKey": "ability_magician_animalhexarea_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 0
        },
        {
          "Key": "HexDuration",
          "Name": "Hex Duration",
          "Value": 2
        },
        {
          "Key": "DamageAmpPercentage",
          "Name": "Damage Amp",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0558
          },
          "Type": "damage",
          "Value": 15
        }
      ]
    }
  },
  "Key": "ability_magician_animalhexarea",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "HexMoveSpeedLimit": {
      "Name": "Movement Speed Limit",
      "Type": "move_speed",
      "Value": 6
    }
  },
  "Name": "Rabbit Hex",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDampingDuration": {
      "Name": null,
      "Value": 1
    },
    "DetonationDelay": {
      "Name": null,
      "Value": 0.9
    },
    "SelfBumpImpulse": {
      "Name": null,
      "Value": 500
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "HexDuration": 1
    },
    {
      "DamageAmpPercentage": 7,
      "DescKey": "ability_magician_animalhexarea_t3_desc",
      "Radius": 3
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
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "rabbit hex",
      "name": "Rabbit Hex",
      "type": "ability"
    }
  ]
}
````
