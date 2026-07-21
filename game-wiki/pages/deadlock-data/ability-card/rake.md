---
title: "Rake"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_vampirebat_steallife"
canonical_name: "Rake"
snapshot_id: 39915
source_document_id: 7071
payload_hash: "61c6dc819c4eb11a235d40f58ad98e03299dba18101955e1f16ab3c7b2f50cf0"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.451567+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Rake

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_steallife`
- Snapshot ID: `39915`
- Source-Dokument: `7071`
- Kurzinfo: Rake aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 16
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_vampirebat_steallife_desc",
  "HeroKey": "hero_vampirebat",
  "HeroName": "Mina",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_vampirebat_steallife_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "stats_count",
            "Value": 1.0
          },
          "Type": "tech_damage",
          "Value": 60
        },
        {
          "Key": "MissingHealthDamagePercentage",
          "Name": "Missing Health Damage",
          "Title": "On Hero Hit:",
          "Type": "tech_damage",
          "Value": 6
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "RakeHealPerKill",
          "Name": "Heal Per Kill",
          "Scale": {
            "Type": "spirit",
            "Value": 0.3
          },
          "Title": "On Kill:",
          "Type": "healing",
          "Value": 25
        }
      ]
    }
  },
  "Key": "ability_vampirebat_steallife",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Rake",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.4
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 6
    },
    "AirDrag": {
      "Name": null,
      "Value": 0.2
    },
    "FallSpeedMax": {
      "Name": null,
      "Value": 3
    },
    "FallingDrag": {
      "Name": null,
      "Value": 20
    },
    "MaxFloatTime": {
      "Name": null,
      "Value": 4.0
    },
    "MiniJumpVelocity": {
      "Name": null,
      "Value": 200
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 60
    },
    "TimeBetweenAttacks": {
      "Name": null,
      "Value": 0.04
    },
    "TrooperExecuteThreshold": {
      "Name": null,
      "Value": 60
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "Damage": 60
    },
    {
      "AbilityCooldown": -8,
      "DescKey": "ability_vampirebat_steallife_t2_desc",
      "RakeHealPerKill": 30
    },
    {
      "DescKey": "ability_vampirebat_steallife_t3_desc",
      "MissingHealthDamagePercentage": 7.0,
      "RakeHealPerKill": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.2
        },
        "Value": 0
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
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "rake",
      "name": "Rake",
      "type": "ability"
    }
  ]
}
````
