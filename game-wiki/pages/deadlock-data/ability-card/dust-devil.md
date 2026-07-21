---
title: "Dust Devil"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "mirage_tornado"
canonical_name: "Dust Devil"
snapshot_id: 39852
source_document_id: 7071
payload_hash: "bb88ae552dd126f6e19b6e492924364aa0e03638bb058d67cf484e77df77475f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.330087+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Dust Devil

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `mirage_tornado`
- Snapshot ID: `39852`
- Source-Dokument: `7071`
- Kurzinfo: Dust Devil aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 36.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "mirage_tornado_desc",
  "Duration": {
    "EnemyLiftDuration": {
      "Name": "Lift Up Time",
      "Type": "duration",
      "Value": 0.2
    }
  },
  "HeroKey": "hero_mirage",
  "HeroName": "Mirage",
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Name": "Radius",
        "Type": "distance",
        "Value": 4
      },
      {
        "Key": "WhirlwindDuration",
        "Name": "Bullet Evasion Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 30
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "DescKey": "mirage_tornado_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "HoldInPlaceDuration",
          "Name": "Lift Duration",
          "Type": "duration",
          "Value": 0.3
        },
        {
          "Key": "WhirlwindEvasionChance",
          "Name": "Bullet Evasion Chance",
          "Value": 30
        }
      ]
    }
  },
  "Key": "mirage_tornado",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Dust Devil",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "ClimbHeight": {
      "Name": null,
      "Value": 1
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.1
    },
    "DistanceAboveGround": {
      "Name": null,
      "Value": 0.5
    },
    "DropDownRate": {
      "Name": null,
      "Value": 10
    },
    "LiftHeight": {
      "Name": null,
      "Value": 3
    },
    "MaxDeltaMovementControl": {
      "Name": null,
      "Value": 2
    },
    "ProjectileThinkInterval": {
      "Name": null,
      "Value": 0.01
    },
    "TickRate": {
      "Name": null,
      "Value": 0.25
    },
    "TornadoSpeed": {
      "Name": null,
      "Value": 24
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 4
  },
  "Range": {
    "OpenHeight": {
      "Name": null,
      "Type": "distance",
      "Value": 8
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 60,
      "DescKey": "mirage_tornado_t1_desc"
    },
    {
      "AbilityCooldown": -12,
      "DescKey": "mirage_tornado_t2_desc",
      "WhirlwindEvasionChance": 30
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 0
      },
      "DescKey": "mirage_tornado_t3_desc",
      "HoldInPlaceDuration": 0.3,
      "RecastWindow": 6
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
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "dust devil",
      "name": "Dust Devil",
      "type": "ability"
    }
  ]
}
````
