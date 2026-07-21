---
title: "Last Stand"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_warden_riot_protocol"
canonical_name: "Last Stand"
snapshot_id: 39934
source_document_id: 7071
payload_hash: "84d55d965c47872872af9e579f8805c2c53ab1b8b4ab701cd18acbb009857759"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.488781+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Last Stand

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_warden_riot_protocol`
- Snapshot ID: `39934`
- Source-Dokument: `7071`
- Kurzinfo: Last Stand aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 180.0
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
  "DescKey": "ability_warden_riot_protocol_desc",
  "HeroKey": "hero_warden",
  "HeroName": "Warden",
  "Info1": {
    "Alt": [
      {
        "Key": "PulseInterval",
        "Name": "Pulse Interval",
        "Type": "duration",
        "Value": 0.5
      },
      {
        "Key": "HealthStealPct",
        "Name": "Non-Hero Lifesteal",
        "Type": "healing",
        "Value": 10
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 50
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 50
      }
    ],
    "DescKey": "ability_warden_riot_protocol_desc",
    "Main": {
      "Props": [
        {
          "Key": "PulseDPS",
          "Name": "DPS",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 70
        },
        {
          "Key": "HealthStealPctHero",
          "Name": "Hero Lifesteal",
          "Type": "healing",
          "Value": 75
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_warden_riot_protocol",
  "Name": "Last Stand",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ConeAngle": {
      "Name": "Cone Angle",
      "Value": 115
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "4",
  "Upgrades": [
    {
      "Radius": 4
    },
    {
      "AbilityCooldown": -30.0,
      "DescKey": "ability_warden_riot_protocol_t2_desc",
      "PulseDPS": 40.5
    },
    {
      "AbilityDuration": 4,
      "BulletResist": 30,
      "DescKey": "ability_warden_riot_protocol_t3_desc",
      "TechResist": 30,
      "UnstoppableCastDelay": 1
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
      "lookup": "last stand",
      "name": "Last Stand",
      "type": "ability"
    }
  ]
}
````
