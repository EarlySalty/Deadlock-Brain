---
title: "Death Slam"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_lash_ultimate"
canonical_name: "Death Slam"
snapshot_id: 39846
source_document_id: 7071
payload_hash: "3e9b50e18a0eefe270858bb171a4d30644e1258dc4fd961a64db8978eb5c7eb1"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.318581+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Death Slam

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash_ultimate`
- Snapshot ID: `39846`
- Source-Dokument: `7071`
- Kurzinfo: Death Slam aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 20
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 2.3
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 170.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Cooldown": {
    "TimeToGainLockonStack": {
      "Name": null,
      "Type": "cooldown",
      "Value": 0.7
    }
  },
  "DescKey": "citadel_ability_lash_ultimate_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "SlowPercent",
        "Name": "Move Speed",
        "Type": "slow",
        "Value": 50
      },
      {
        "Key": "SlowDuration",
        "Name": "Slow Duration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "AbilityCastDelay",
        "Name": "Cast Delay",
        "Type": "cast",
        "Value": 0.3
      }
    ],
    "DescKey": "citadel_ability_lash_ultimate_desc",
    "Main": {
      "Props": [
        {
          "Key": "ImpactDamage",
          "Name": "Impact Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.974938
          },
          "Type": "tech_damage",
          "Value": 105
        },
        {
          "Key": "ThrowDistance",
          "Name": "Max Throw Distance",
          "Scale": {
            "Type": "spirit",
            "Value": 0.14
          },
          "Value": 14
        }
      ]
    }
  },
  "Key": "citadel_ability_lash_ultimate",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Death Slam",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 6
    },
    "BoostTime": {
      "Name": null,
      "Value": 1.0
    },
    "HangTime": {
      "Name": null,
      "Value": 0.6
    },
    "ImpactRadius": {
      "Name": "Impact Radius",
      "Value": 6
    },
    "LiftHeight": {
      "Name": null,
      "Value": 6
    },
    "LosingLockGraceTime": {
      "Name": null,
      "Value": 0.4
    },
    "MaxLockonStacks": {
      "Name": null,
      "Value": 1
    },
    "NotInConeLosesLock": {
      "Name": null,
      "Value": 1
    },
    "SlamSpeed": {
      "Name": null,
      "Value": 1600
    },
    "ThrowStraightDuration": {
      "Name": null,
      "Value": 1.5
    },
    "TimeToLoseLockonStack": {
      "Name": null,
      "Value": 2
    },
    "UpBoostSpeed": {
      "Name": null,
      "Value": 400
    }
  },
  "Range": {
    "LockonConeAngle": {
      "Name": null,
      "Type": "distance",
      "Value": 40
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "ThrowDistance": 12
    },
    {
      "AbilityCooldown": -35
    },
    {
      "AbilityCastRange": 6,
      "DescKey": "citadel_ability_lash_ultimate_t3_desc",
      "StunDuration": 1.2
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
      "lookup": "death slam",
      "name": "Death Slam",
      "type": "ability"
    }
  ]
}
````
