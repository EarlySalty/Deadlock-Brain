---
title: "Nox Nostra"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_vampirebat_batswarm"
canonical_name: "Nox Nostra"
snapshot_id: 39918
source_document_id: 7071
payload_hash: "ecb58b6bf2effa02cb55e78d39cb37415f839279e77f4ea753ef740c54f24a03"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.457433+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Nox Nostra

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batswarm`
- Snapshot ID: `39918`
- Source-Dokument: `7071`
- Kurzinfo: Nox Nostra aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.5
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 40
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
  "Cast": {
    "BonusBatsMax": {
      "Name": "Max Additional Bats",
      "Type": "cast",
      "Value": 50
    }
  },
  "Cooldown": {
    "TimeToGainLockonStack": {
      "Name": null,
      "Type": "cooldown",
      "Value": 0.01
    }
  },
  "DescKey": "ability_vampirebat_batswarm_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBatsPerProc",
        "Name": "Additional Bats Per Love Bite",
        "Type": "cast",
        "Value": 2
      },
      {
        "Key": "BatCount",
        "Name": "Total Bats Released",
        "Type": "cast",
        "Value": 75
      }
    ],
    "DescKey": "ability_vampirebat_batswarm_desc",
    "Main": {
      "Props": [
        {
          "Key": "BatPerSecond",
          "Name": "Bats per Second",
          "Value": 30
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.094
          },
          "Type": "tech_damage",
          "Value": 4.6
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "StatusEffect": "Silence",
          "Title": "On Hit:",
          "Value": 1.25
        },
        {
          "Key": "CurrentHealthPercent",
          "Name": "Current Health",
          "Title": "On Hit:",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_vampirebat_batswarm",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Nox Nostra",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 12
    },
    "AirDrag": {
      "Name": null,
      "Value": 12
    },
    "BatCountPerWave": {
      "Name": null,
      "Value": 1
    },
    "BatEffectiveness": {
      "Name": null,
      "Value": 0.2
    },
    "BatSpawnRadius": {
      "Name": null,
      "Value": 1.5
    },
    "BatSpawnRandomAngle": {
      "Name": null,
      "Value": 0.15
    },
    "BatSpawnRandomVelocity": {
      "Name": null,
      "Value": 300
    },
    "CurrentHealthDamageCapToBosses": {
      "Name": null,
      "Value": 20
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 1
    },
    "GroundAccelerationPercentage": {
      "Name": null,
      "Value": -80
    },
    "GroundFrictioNpercentage": {
      "Name": null,
      "Value": -80
    },
    "JumpCeilingCheckDistance": {
      "Name": null,
      "Value": 11
    },
    "JumpPitch": {
      "Name": null,
      "Value": -60
    },
    "JumpSpeed": {
      "Name": null,
      "Value": 17
    },
    "MaxBatTargets": {
      "Name": null,
      "Value": 2
    },
    "MaxLockonStacks": {
      "Name": null,
      "Value": 1
    },
    "NotInConeLosesLock": {
      "Name": null,
      "Value": 1
    },
    "StacksCanDecay": {
      "Name": null,
      "Value": 1
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 20
    },
    "TimeToLoseLockonStack": {
      "Name": null,
      "Value": 0.3
    },
    "VerticalDrag": {
      "Name": null,
      "Value": 1
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
      "Damage": 1.9
    },
    {
      "AbilityCooldown": -45
    },
    {
      "CurrentHealthPercent": 0.5,
      "DescKey": "ability_vampirebat_batswarm_t3_desc"
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "nox nostra",
      "name": "Nox Nostra",
      "type": "ability"
    }
  ]
}
````
