---
title: "Ground Strike"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_lash_down_strike"
canonical_name: "Ground Strike"
snapshot_id: 39843
source_document_id: 7071
payload_hash: "fb31a734210cda669192c11f916f50b1f122f57fdef4f121086e32af6f147ecc"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.312204+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Ground Strike

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_down_strike`
- Snapshot ID: `39843`
- Source-Dokument: `7071`
- Kurzinfo: Ground Strike aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 18.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "StompDamagePerMeterSecondary": {
      "Name": null,
      "Scale": {
        "Type": "spirit",
        "Value": 0.008137
      },
      "Type": "tech_damage",
      "Value": 4.2
    }
  },
  "DescKey": "citadel_ability_lash_down_strike_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "EnemySlowPct",
        "Name": "Enemy Move Speed",
        "Type": "slow",
        "Value": 0
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_lash_down_strike_desc",
    "Main": {
      "Props": [
        {
          "Key": "StompDamage",
          "Name": "Stomp Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.7905
          },
          "Type": "tech_damage",
          "Value": 60.0
        },
        {
          "Key": "StompDamagePerMeterPrimary",
          "Name": "Damage Per Meter",
          "Scale": {
            "Type": "spirit",
            "Value": 0.04
          },
          "Type": "tech_damage",
          "Value": 5.5
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0
        }
      ]
    }
  },
  "Key": "citadel_ability_lash_down_strike",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Ground Strike",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "MinAimAngle": {
      "Name": null,
      "Value": 60
    },
    "StompDamagePrimaryRange": {
      "Name": null,
      "Value": 25
    },
    "StompVerticalThreshold": {
      "Name": null,
      "Value": 118
    },
    "StrikeVelocity": {
      "Name": null,
      "Value": 50
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 10
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -10.0
    },
    {
      "DescKey": "citadel_ability_lash_down_strike_t2_desc",
      "EnemySlowPct": 50,
      "SlowDuration": 3,
      "StompBounceHeight": 400,
      "TossDuration": 1
    },
    {
      "DescKey": "citadel_ability_lash_down_strike_t3_desc",
      "StompDamagePerMeterPrimary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.03255
        },
        "Value": 2.13
      },
      "StompDamagePerMeterSecondary": {
        "Multiply": true,
        "Scale": {
          "Type": "spirit",
          "Value": 0.008137
        },
        "Value": 2.13
      }
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
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "ground strike",
      "name": "Ground Strike",
      "type": "ability"
    }
  ]
}
````
