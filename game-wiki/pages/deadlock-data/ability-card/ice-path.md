---
title: "Ice Path"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_icepath"
canonical_name: "Ice Path"
snapshot_id: 39836
source_document_id: 7071
payload_hash: "55b2183aa6d09a316df2799c76f6f97c8478ac7d08b7bda20f6a34fe40de21c9"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.296574+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Ice Path

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_icepath`
- Snapshot ID: `39836`
- Source-Dokument: `7071`
- Kurzinfo: Ice Path aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 50.0
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
  "DescKey": "ability_icepath_desc",
  "HeroKey": "hero_kelvin",
  "HeroName": "Kelvin",
  "Info1": {
    "Alt": [
      {
        "Key": "IcePathAuraDuration",
        "Name": "Ice Trail Duration",
        "Type": "duration",
        "Value": 18
      }
    ],
    "DescKey": "ability_icepath_desc",
    "Main": {
      "Props": [
        {
          "Key": "MoveSpeedBonus",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 2
        },
        {
          "Key": "SprintSpeedBonus",
          "Name": "Sprint Speed",
          "Type": "move_speed",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_icepath",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    },
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Name": "move speed penalty while shooting reduction",
      "Type": "move_speed",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Name": null,
      "Type": "move_speed",
      "Value": 100
    },
    "SlideScale": {
      "Name": "Slide Distance",
      "Type": "move_speed",
      "Value": 50
    },
    "SlowResistancePercent": {
      "Name": "Slow Resist",
      "Type": "move_speed",
      "Value": 60
    }
  },
  "Name": "Ice Path",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraDistance": {
      "Name": null,
      "Value": 250
    },
    "IcePathEdgeWidth": {
      "Name": null,
      "Value": 0.7
    },
    "IcePathInterval": {
      "Name": null,
      "Value": 0.5
    },
    "IcePathPullInStrength": {
      "Name": null,
      "Value": 20
    },
    "MinHeight": {
      "Name": null,
      "Value": 20
    },
    "PopupForce": {
      "Name": null,
      "Value": 30
    }
  },
  "Range": {
    "IcePathShardRadius": {
      "Name": "Path Width",
      "Type": "distance",
      "Value": 1.2
    },
    "ModifierRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 5
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BulletResist": 35,
      "DescKey": "ability_icepath_t1_desc",
      "MoveSpeedBonus": 2
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "BonusSpirit": 20,
      "BonusSpiritPct": 35,
      "DescKey": "ability_icepath_t3_desc"
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
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "ice path",
      "name": "Ice Path",
      "type": "ability"
    }
  ]
}
````
