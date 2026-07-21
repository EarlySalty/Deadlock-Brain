---
title: "Rallying Charge"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_bookworm_knightcharge"
canonical_name: "Rallying Charge"
snapshot_id: 39770
source_document_id: 7071
payload_hash: "1b548175df0873d562c10aec3b378e5132ef17f82eba5fd18c53b098fd0ce42c"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.171103+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Rallying Charge

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightcharge`
- Snapshot ID: `39770`
- Source-Dokument: `7071`
- Kurzinfo: Rallying Charge aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 600
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.7
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 220
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 13
  },
  "DescKey": "ability_bookworm_knightcharge_desc",
  "Duration": {
    "BuffDuration": {
      "Name": "Buff Duration",
      "Type": "duration",
      "Value": 9
    }
  },
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_bookworm_knightcharge_desc",
    "Main": {
      "Props": []
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.0
          },
          "Title": "On Enemy Hit:",
          "Type": "tech_damage",
          "Value": 125
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Enemy Hit:",
          "Value": 1.0
        }
      ]
    }
  },
  "Info3": {
    "Alt": [
      {
        "Key": "MaxAmp",
        "Type": "damage",
        "Value": 100
      },
      {
        "Key": "MaxAmpDistance",
        "Type": "distance",
        "Value": 250
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "HealAmount",
          "Name": "Heal Amount",
          "Scale": {
            "Type": "spirit",
            "Value": 1.6
          },
          "Title": "On Friendly Hit:",
          "Type": "healing",
          "Value": 125
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Title": "On Friendly Hit:",
          "Type": "move_speed",
          "Value": 5
        }
      ]
    }
  },
  "Key": "ability_bookworm_knightcharge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 1.3
    }
  },
  "Name": "Rallying Charge",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.8
    },
    "AllyHeight": {
      "Name": null,
      "Value": 20
    },
    "AllyRadius": {
      "Name": null,
      "Value": 4
    },
    "CancelCooldownRefundPercentage": {
      "Name": null,
      "Value": 50
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 20
    },
    "GravityAcceleration": {
      "Name": null,
      "Value": -1900
    },
    "GroundStickHeight": {
      "Name": null,
      "Value": 0.05
    },
    "KnightBonusPerWave": {
      "Name": null,
      "Value": -99
    },
    "KnightChargeHeight": {
      "Name": null,
      "Value": 3.5
    },
    "KnightChargeWidth": {
      "Name": null,
      "Value": 1.7
    },
    "KnightCount": {
      "Name": null,
      "Value": 5
    },
    "KnightCountInFirstWave": {
      "Name": null,
      "Value": 5
    },
    "KnightJumpSpeed": {
      "Name": null,
      "Value": 900
    },
    "KnightMaxFallHeight": {
      "Name": null,
      "Value": -35
    },
    "KnightMaxJumpHeight": {
      "Name": null,
      "Value": 30
    },
    "KnightNavForwardDistance": {
      "Name": null,
      "Value": 8
    },
    "KnightNavSearchDistance": {
      "Name": null,
      "Value": 10
    },
    "KnightPositionSpread": {
      "Name": "Knight Spread",
      "Value": 1.8
    },
    "KnightPositionStagger": {
      "Name": null,
      "Value": -4
    },
    "KnightWhiskerLength": {
      "Name": null,
      "Value": 300
    },
    "KnightWhiskerSide": {
      "Name": null,
      "Value": 50
    },
    "KnightWhiskerStrength": {
      "Name": null,
      "Value": 0.2
    },
    "TargetFindingDelay": {
      "Name": null,
      "Value": 0.04
    },
    "TossBackSpeed": {
      "Name": null,
      "Value": 100
    },
    "TossUpSpeed": {
      "Name": null,
      "Value": 600
    },
    "WaveCount": {
      "Name": null,
      "Value": 2
    },
    "WavePositionStagger": {
      "Name": null,
      "Value": -15
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "HealAmount": 150
    },
    {
      "AbilityCooldown": -45,
      "DescKey": "ability_bookworm_knightcharge_t2_desc",
      "KnightCount": 4,
      "KnightCountInFirstWave": 4
    },
    {
      "Damage": 160.0,
      "DescKey": "ability_bookworm_knightcharge_t3_desc",
      "MaxAmp": 70,
      "StunDuration": 0.5
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
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "rallying charge",
      "name": "Rallying Charge",
      "type": "ability"
    }
  ]
}
````
