---
title: "Spectral Assistant"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_magician_cloneturret"
canonical_name: "Spectral Assistant"
snapshot_id: 39848
source_document_id: 7071
payload_hash: "3f6f32df04153e5cc9d5d0d9193a828cbea943e4451a01f19e1f2eaeb33d533d"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.322511+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Spectral Assistant

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_magician_cloneturret`
- Snapshot ID: `39848`
- Source-Dokument: `7071`
- Kurzinfo: Spectral Assistant aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 40
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 6
  },
  "DescKey": "ability_magician_cloneturret_desc",
  "HeroKey": "hero_magician",
  "HeroName": "Sinclair",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_magician_cloneturret_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.36
          },
          "Type": "tech_damage",
          "Value": 15
        },
        {
          "Key": "AbilityDuration",
          "Name": "Duration",
          "Type": "duration",
          "Value": 6
        },
        {
          "Key": "TotalSwaps",
          "Name": "Max Swaps",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_magician_cloneturret",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spectral Assistant",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TurretBulletTargetAngle": {
      "Name": null,
      "Type": "",
      "Value": 20
    },
    "TurretBulletTargetRadius": {
      "Name": null,
      "Value": 500
    },
    "TurretBulletVerticalOffset": {
      "Name": null,
      "Value": 2
    }
  },
  "Range": {
    "LeashRadius": {
      "Name": "Max Leash Radius",
      "Type": "time",
      "Value": 20
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 5,
      "AbilityDuration": 7,
      "DescKey": "ability_magician_cloneturret_t2_desc",
      "LeashRadius": 5
    },
    {
      "BonusFireRate": 60,
      "Damage": 12.6,
      "DescKey": "ability_magician_cloneturret_t3_desc"
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
      "lookup": "spectral assistant",
      "name": "Spectral Assistant",
      "type": "ability"
    }
  ]
}
````
