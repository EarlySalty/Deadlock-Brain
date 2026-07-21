---
title: "Enchanter's Satchel"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "synth_pulse"
canonical_name: "Enchanter's Satchel"
snapshot_id: 39901
source_document_id: 7071
payload_hash: "d8b607dd773a9bbefbd226ae6c45ffdd29516bc3c20e368feb6749af2a269879"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.424728+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Enchanter's Satchel

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_pulse`
- Snapshot ID: `39901`
- Source-Dokument: `7071`
- Kurzinfo: Enchanter's Satchel aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 1.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 17.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_pulse_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "synth_pulse_desc",
    "Main": {
      "Props": [
        {
          "Key": "AbilityChannelTime",
          "Name": "Channel Duration",
          "Type": "cast",
          "Value": 1.5
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.1
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "MoveSlowPercent",
          "Name": "Movement Slow",
          "Type": "slow",
          "Value": 0
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "bullet_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "synth_pulse",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Enchanter's Satchel",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.0254
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 12
  },
  "Slot": "3",
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 90
    },
    {
      "AbilityChannelTime": 1.5,
      "DebuffDuration": 4.0,
      "DescKey": "synth_pulse_t3_desc",
      "FireRateSlow": 40,
      "MoveSlowPercent": 40,
      "Radius": 4
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "enchanter's satchel",
      "name": "Enchanter's Satchel",
      "type": "ability"
    }
  ]
}
````
