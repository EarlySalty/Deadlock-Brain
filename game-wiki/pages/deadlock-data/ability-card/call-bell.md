---
title: "Call Bell"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_doorman_bomb"
canonical_name: "Call Bell"
snapshot_id: 39775
source_document_id: 7071
payload_hash: "43fccb2eb7488ed27b938722d0e6b54c7125079eb857ccb6feb73d26ebd67969"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.180549+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Call Bell

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_bomb`
- Snapshot ID: `39775`
- Source-Dokument: `7071`
- Kurzinfo: Call Bell aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 6
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 4
  },
  "DescKey": "ability_doorman_bomb_desc",
  "HeroKey": "hero_doorman",
  "HeroName": "The Doorman",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_doorman_bomb_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7
          },
          "Title": "On Impact",
          "Type": "tech_damage",
          "Value": 40
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ProjectileFuse",
        "Type": "duration",
        "Value": 3
      },
      {
        "Key": "AbilityDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "ExplosionDamage",
          "Name": "Explosion Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.2
          },
          "Title": "Explosion",
          "Type": "tech_damage",
          "Value": 55
        },
        {
          "Key": "DebuffAccuracy",
          "Name": "Weapon Accuracy",
          "Title": "Explosion",
          "Type": "fire_rate",
          "Value": -40
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "Explosion",
          "Type": "slow",
          "Value": 35
        }
      ]
    }
  },
  "Key": "ability_doorman_bomb",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Call Bell",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.5
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AccuracyDebuffFalloffBias": {
      "Name": null,
      "Value": 0.3
    },
    "EnableAura": {
      "Name": null,
      "Value": 1
    },
    "ProjectileDrag": {
      "Name": null,
      "Value": 0.975
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5.5
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCharges": 1,
      "DescKey": "ability_doorman_bomb_t1_desc"
    },
    {
      "DescKey": "ability_doorman_bomb_t2_desc",
      "ExplosionDamage": 40,
      "ImpactDamage": 30
    },
    {
      "DescKey": "ability_doorman_bomb_t3_desc",
      "ExplosionDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.35
        },
        "Value": 0
      },
      "ImpactDamage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.35
        },
        "Value": 0
      },
      "ProjectileFuse": 26,
      "Radius": 4.5
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
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "call bell",
      "name": "Call Bell",
      "type": "ability"
    }
  ]
}
````
