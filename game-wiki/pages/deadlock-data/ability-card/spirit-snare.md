---
title: "Spirit Snare"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_immobilize_trap"
canonical_name: "Spirit Snare"
snapshot_id: 39869
source_document_id: 7071
payload_hash: "830a4defb8d3a54b19af73176cddd7a5b7cec99abff4b491e882ffe37c4b00e0"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.363049+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Spirit Snare

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_immobilize_trap`
- Snapshot ID: `39869`
- Source-Dokument: `7071`
- Kurzinfo: Spirit Snare aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 34
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_immobilize_trap_desc",
  "Duration": {
    "TripTime": {
      "Name": null,
      "Type": "duration",
      "Value": 0.5
    }
  },
  "HeroKey": "hero_orion",
  "HeroName": "Grey Talon",
  "Info1": {
    "Alt": [
      {
        "Key": "ArmTime",
        "Name": "Arm Time",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 30
      },
      {
        "Key": "ChargedShotHitRadiusScale",
        "Name": "Charged Shot Radius",
        "Type": "radius",
        "Value": 30
      },
      {
        "Key": "BulletVulnerbility",
        "Name": "Bullet Damage Amp",
        "Type": "bullet_armor_down",
        "Value": 0
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "ability_immobilize_trap_desc",
    "Main": {
      "Props": [
        {
          "Key": "TetherDuration",
          "Name": "Tether Duration",
          "Type": "duration",
          "Value": 2.25
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Type": "tech_damage",
          "Value": 25
        },
        {
          "Key": "Lifetime",
          "Name": "Lifetime",
          "Type": "duration",
          "Value": 22
        }
      ]
    }
  },
  "Key": "ability_immobilize_trap",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Spirit Snare",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "SkipFrames": {
      "Name": null,
      "Value": 6
    },
    "TrapHeight": {
      "Name": null,
      "Value": 2
    },
    "TripGravity": {
      "Name": null,
      "Value": 0.4
    },
    "TripUpSpeed": {
      "Name": null,
      "Value": 6.35
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 6.5
  },
  "Range": {
    "TetherRadius": {
      "Name": "Tether Radius",
      "Type": "distance",
      "Value": 6
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "BulletArmorReduction": -15,
      "DebuffDuration": 10,
      "DescKey": "ability_immobilize_trap_t2_desc"
    },
    {
      "DescKey": "ability_immobilize_trap_t3_desc",
      "Radius": 1.5,
      "TetherDuration": 1
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
      "hero_key": "hero_orion",
      "hero_name": "Grey Talon",
      "lookup": "spirit snare",
      "name": "Spirit Snare",
      "type": "ability"
    }
  ]
}
````
