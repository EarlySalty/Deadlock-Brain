---
title: "Goo Ball"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "viscous_goo_bowling_ball"
canonical_name: "Goo Ball"
snapshot_id: 39930
source_document_id: 7071
payload_hash: "dac87e15b7989235d52a4384154a155aaf7e862c8af58055bad05370543af6bd"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.481021+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Goo Ball

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `viscous_goo_bowling_ball`
- Snapshot ID: `39930`
- Source-Dokument: `7071`
- Kurzinfo: Goo Ball aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.55
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 150
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 11
  },
  "DescKey": "viscous_goo_bowling_ball_desc",
  "HeroKey": "hero_viscous",
  "HeroName": "Viscous",
  "Info1": {
    "Alt": [
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Value": 35
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Value": 35
      }
    ],
    "DescKey": "viscous_goo_bowling_ball_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1
          },
          "Type": "tech_damage",
          "Value": 110
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "Value": 0.5
        },
        {
          "Key": "BallRadius",
          "Name": "Ball Radius",
          "Type": "radius",
          "Value": 1.4
        }
      ]
    }
  },
  "Key": "viscous_goo_bowling_ball",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 7
    },
    "MoveSpeedMax": {
      "Name": "Move Speed",
      "Type": "move_speed",
      "Value": 7
    }
  },
  "Name": "Goo Ball",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AccelerationPercentage": {
      "Name": null,
      "Value": -60
    },
    "AirJumpForce": {
      "Name": null,
      "Value": 500
    },
    "BallOffset": {
      "Name": null,
      "Value": 50
    },
    "BreakablePropDamageRadius": {
      "Name": null,
      "Value": 75
    },
    "CastWhileRolling": {
      "Name": null,
      "Value": 1
    },
    "FrictionPercentage": {
      "Name": null,
      "Value": -85
    },
    "JumpForce": {
      "Name": null,
      "Value": 500
    },
    "KnockForce": {
      "Name": null,
      "Value": 400
    },
    "ParticleRadiusMultiplier": {
      "Name": null,
      "Value": 1.2
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    }
  },
  "Range": {
    "BallHitRadius": {
      "Name": null,
      "Type": "radius",
      "Value": 1.8
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "AbilityCooldown": -25
    },
    {
      "BulletResist": 10,
      "Damage": 70,
      "DescKey": "viscous_goo_bowling_ball_t2_desc",
      "TechResist": 10
    },
    {
      "AbilityDuration": 7,
      "DescKey": "viscous_goo_bowling_ball_t3_desc",
      "StunDuration": 0.3
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
      "hero_key": "hero_viscous",
      "hero_name": "Viscous",
      "lookup": "goo ball",
      "name": "Goo Ball",
      "type": "ability"
    }
  ]
}
````
