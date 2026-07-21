---
title: "Disengaging Sigil"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_fencer_throwblade"
canonical_name: "Disengaging Sigil"
snapshot_id: 39791
source_document_id: 7071
payload_hash: "68d9dd101a4fe0b445e84f0e1ad108ffaf8e61377cfcf02689ec9784d59fbab3"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.211246+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Disengaging Sigil

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_throwblade`
- Snapshot ID: `39791`
- Source-Dokument: `7071`
- Kurzinfo: Disengaging Sigil aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 12
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_fencer_throwblade_desc",
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Value": 4
      },
      {
        "Key": "SigilRadius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 6.5
      }
    ],
    "DescKey": "ability_fencer_throwblade_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 85
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Type": "slow",
          "Value": 30
        },
        {
          "Key": "FireRateSlow",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fencer_throwblade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Disengaging Sigil",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 2.0
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "JumpVelocityHidden": {
      "Name": null,
      "Value": 16
    },
    "TraceToGroundDistance": {
      "Name": null,
      "Value": 1000
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BonusBulletSpeedPercent": 25,
      "BonusFireRate": 25,
      "BuffDuration": 8,
      "DescKey": "ability_fencer_throwblade_t1_desc"
    },
    {
      "DescKey": "ability_fencer_throwblade_t2_desc",
      "ResetsAirLimit": 1,
      "StaminaToRestore": 1
    },
    {
      "DescKey": "ability_fencer_throwblade_t3_desc",
      "RecastTime": 4
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
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "disengaging sigil",
      "name": "Disengaging Sigil",
      "type": "ability"
    }
  ]
}
````
