---
title: "Heavy Barrage"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_rocket_barrage"
canonical_name: "Heavy Barrage"
snapshot_id: 39798
source_document_id: 7071
payload_hash: "fd28234f7bd1d9f9f67181bb45796a6a5192a78a92a71f2ab9cf4c843f14497a"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.225755+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Heavy Barrage

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_rocket_barrage`
- Snapshot ID: `39798`
- Source-Dokument: `7071`
- Kurzinfo: Heavy Barrage aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 36
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 200.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 8
  },
  "Debuff": {
    "GroundDashReductionPercent": {
      "Name": "Dash Distance",
      "Type": "slow",
      "Value": -35
    }
  },
  "DescKey": "citadel_ability_rocket_barrage_desc",
  "HeroKey": "hero_forge",
  "HeroName": "McGinnis",
  "Info1": {
    "Alt": [
      {
        "Key": "MinDistance",
        "Name": "Min Range",
        "Type": "distance",
        "Value": 8.5
      },
      {
        "Key": "AbilityDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "MoveSlowDuration",
        "Type": "duration",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_rocket_barrage_desc",
    "Main": {
      "Props": [
        {
          "Key": "DamagePerRocket",
          "Name": "Damage Per Rocket",
          "Scale": {
            "Type": "spirit",
            "Value": 0.2
          },
          "Type": "tech_damage",
          "Value": 21
        },
        {
          "Key": "GrenadesPerSecond",
          "Name": "Rockets per second",
          "Value": 6
        },
        {
          "Key": "ExplosionRadius",
          "Name": "Explosion Radius",
          "Type": "distance",
          "Value": 4.5
        }
      ]
    }
  },
  "Key": "citadel_ability_rocket_barrage",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Heavy Barrage",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 100
    },
    "DetonateTimer": {
      "Name": null,
      "Value": 5
    },
    "ExplosionFalloffDisabled": {
      "Name": null,
      "Value": 1
    },
    "IntervalRampUpStart": {
      "Name": null,
      "Value": 0.35
    },
    "IntervalRampUpTime": {
      "Name": null,
      "Value": 0.3
    },
    "MaxSpread": {
      "Name": null,
      "Value": 5
    },
    "ProjectileIgnoreCollisionTime": {
      "Name": null,
      "Value": 0.2
    },
    "TrackSpeedFar": {
      "Name": null,
      "Value": 100
    },
    "TrackSpeedNear": {
      "Name": null,
      "Value": 150
    },
    "TrackingTime": {
      "Name": null,
      "Value": 0.4
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "DescKey": "citadel_ability_rocket_barrage_t1_desc",
      "EnemyDashSlowPercent": -18,
      "MoveSlowDuration": 1,
      "MoveSlowPercent": 30
    },
    {
      "AbilityCooldown": -45.0,
      "AbilityDuration": 6,
      "DescKey": "citadel_ability_rocket_barrage_t2_desc"
    },
    {
      "DamagePerRocket": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 15
      },
      "DescKey": "citadel_ability_rocket_barrage_t3_desc",
      "ExplosionRadius": 2
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
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "heavy barrage",
      "name": "Heavy Barrage",
      "type": "ability"
    }
  ]
}
````
