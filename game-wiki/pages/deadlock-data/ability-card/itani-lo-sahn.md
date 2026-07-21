---
title: "Itani Lo Sahn"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_fencer_ultimate"
canonical_name: "Itani Lo Sahn"
snapshot_id: 39794
source_document_id: 7071
payload_hash: "401b6933788b97133795460a222f356480025a1d3f08446296313f7b7bb0deff"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.217416+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Itani Lo Sahn

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_ultimate`
- Snapshot ID: `39794`
- Source-Dokument: `7071`
- Kurzinfo: Itani Lo Sahn aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 1.5
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 9999
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 145
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Damage": {
    "ImpactDamage": {
      "Name": "Impact Damage",
      "Scale": {
        "Type": "spirit",
        "Value": 0.77
      },
      "Type": "tech_damage",
      "Value": 70
    }
  },
  "Debuff": {
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Type": "slow",
      "Value": -30
    }
  },
  "DescKey": "ability_fencer_ultimate_desc",
  "Duration": {
    "CasterLockDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 1.8
    }
  },
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [
      {
        "Key": "IncomingDamageReductionPercent",
        "Name": "Time Slow Damage Reduction",
        "Value": 70
      },
      {
        "Key": "LowHealthEnemyThresholdPct",
        "Name": "Low Health Threshold",
        "Value": 50
      },
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Type": "duration",
        "Value": 1.8
      }
    ],
    "DescKey": "ability_fencer_ultimate_desc",
    "Main": {
      "Props": [
        {
          "Key": "DelayedDamage",
          "Name": "Delayed Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 2.6
          },
          "Type": "tech_damage",
          "Value": 200
        },
        {
          "Key": "BonusDamagePercent",
          "Name": "Bonus Damage",
          "Value": 60
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "CooldownReductionOnHit",
          "Name": "Cooldown Reduced on Hero Hit",
          "Title": "On hit",
          "Type": "cooldown",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fencer_ultimate",
  "Name": "Itani Lo Sahn",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AirSpeedMax": {
      "Name": null,
      "Value": 70
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 254.0
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "GapDistanceToWall": {
      "Name": null,
      "Value": 180
    },
    "MoveSpeedPenaltyMaxSpeed": {
      "Name": null,
      "Value": 200
    },
    "SideMoveSpeedReduction": {
      "Name": null,
      "Value": -100
    },
    "TechCleaveExpireTime": {
      "Name": null,
      "Value": 0.35
    },
    "TimeScaleDebuff": {
      "Name": null,
      "Value": 70
    },
    "TimerSoundDuration": {
      "Name": null,
      "Value": 1
    },
    "TravelDistPctBeforeWallGapCheck": {
      "Name": null,
      "Value": 70
    },
    "TurnRateMaxDuringCast": {
      "Name": null,
      "Value": 999
    },
    "VacuumSpeed": {
      "Name": null,
      "Value": 10.16
    }
  },
  "Range": {
    "DashRadius": {
      "Name": "Radius",
      "Type": "distance",
      "Value": 7
    },
    "DashRange": {
      "Name": "Time Window",
      "Type": "distance",
      "Value": 27
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DashRange": 8
    },
    {
      "AbilityCooldown": -35
    },
    {
      "BonusDamagePercent": 50,
      "DescKey": "ability_fencer_ultimate_t3_desc"
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
      "lookup": "itani lo sahn",
      "name": "Itani Lo Sahn",
      "type": "ability"
    }
  ]
}
````
