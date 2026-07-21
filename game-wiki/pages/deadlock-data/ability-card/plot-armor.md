---
title: "Plot Armor"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_bookworm_knightbarrier"
canonical_name: "Plot Armor"
snapshot_id: 39768
source_document_id: 7071
payload_hash: "1ad0c5b609ed0f2fd309f075eb68f77fdffb04563821e3a6e553e2fd4976ab89"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.167215+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Plot Armor

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightbarrier`
- Snapshot ID: `39768`
- Source-Dokument: `7071`
- Kurzinfo: Plot Armor aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Type": "range",
    "Value": 35
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 28.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_bookworm_knightbarrier_desc",
  "HeroKey": "hero_bookworm",
  "HeroName": "Paige",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_bookworm_knightbarrier_desc",
    "Main": {
      "Props": [
        {
          "Key": "CombatBarrier",
          "Name": "Barrier",
          "Scale": {
            "Type": "spirit",
            "Value": 1.5
          },
          "Type": "bullet_armor_up",
          "Value": 125
        },
        {
          "Key": "BaseAttackDamagePercent",
          "Name": "Weapon Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.2
          },
          "Type": "bullet_damage",
          "Value": 25
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Type": "fire_rate",
          "Value": 0
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": []
    }
  },
  "Key": "ability_bookworm_knightbarrier",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Plot Armor",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BonusSpiritDamagePercent": {
      "Name": null,
      "Value": 15
    },
    "BonusTargetRadius": {
      "Name": null,
      "Value": 30
    },
    "PushForce": {
      "Name": null,
      "Value": 900
    }
  },
  "Range": {
    "ShoveRadius": {
      "Name": "Shove Radius",
      "Type": "distance",
      "Value": 6
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "BonusFireRate": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 14
      },
      "DescKey": "ability_bookworm_knightbarrier_t1_desc"
    },
    {
      "AbilityDuration": 2,
      "CombatBarrier": 100,
      "DescKey": "ability_bookworm_knightbarrier_t2_desc"
    },
    {
      "BonusTargets": 2,
      "BonusTargetsBarrierPercentage": 100,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      },
      "DescKey": "ability_bookworm_knightbarrier_t3_desc"
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
      "lookup": "plot armor",
      "name": "Plot Armor",
      "type": "ability"
    }
  ]
}
````
