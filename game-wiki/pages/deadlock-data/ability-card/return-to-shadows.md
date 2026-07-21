---
title: "Return to Shadows"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_nano_shadow_pulse"
canonical_name: "Return to Shadows"
snapshot_id: 39858
source_document_id: 7071
payload_hash: "8ea0e69c3156e327ba9ceb1c5aa0e99c7473f7d2f13232f63d8900d9b606a432"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.342062+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Return to Shadows

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_shadow_pulse`
- Snapshot ID: `39858`
- Source-Dokument: `7071`
- Kurzinfo: Return to Shadows aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 115
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_shadow_pulse_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityChannelTime",
        "Name": "Channel Duration",
        "Type": "cast",
        "Value": 3
      },
      {
        "Key": "OutgoingDamagePercent",
        "Name": "Damage",
        "Type": "damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_nano_shadow_pulse_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.609336
          },
          "Type": "tech_damage",
          "Value": 150.0
        },
        {
          "Key": "BonusMoveSpeedPercent",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 20
        }
      ]
    }
  },
  "Key": "ability_nano_shadow_pulse",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Return to Shadows",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 4
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 100
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 0.254
    },
    "ZAcceleration": {
      "Name": null,
      "Value": 800
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 7.5
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "BonusMoveSpeedPercent": 20,
      "Damage": 75,
      "DescKey": "ability_nano_shadow_pulse_t2_desc"
    },
    {
      "DescKey": "ability_nano_shadow_pulse_t3_desc",
      "HealAmount": 450,
      "RefundCooldowns": 1
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
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "return to shadows",
      "name": "Return to Shadows",
      "type": "ability"
    }
  ]
}
````
