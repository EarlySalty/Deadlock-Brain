---
title: "Bashdown"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_punkgoat_ult"
canonical_name: "Bashdown"
snapshot_id: 39875
source_document_id: 7071
payload_hash: "013bfd052d58141afce49665d30f14de55dc64aa6b7a3fcb0713293d67f99277"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.374515+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Bashdown

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_ult`
- Snapshot ID: `39875`
- Source-Dokument: `7071`
- Kurzinfo: Bashdown aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "distance",
    "Value": 4
  },
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.3
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 35
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 8
  },
  "DescKey": "ability_punkgoat_ult_desc",
  "Duration": {
    "ExplodeDelay": {
      "Name": "Explosion Delay",
      "Type": "duration",
      "Value": 0.5
    },
    "PullDownDuration": {
      "Name": null,
      "Type": "duration",
      "Value": 0.75
    }
  },
  "HeroKey": "hero_punkgoat",
  "HeroName": "Billy",
  "Info1": {
    "Alt": [
      {
        "Key": "FireRateSlowDuration",
        "Name": "Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "FireRateSlow",
        "Name": "Fire Rate",
        "Type": "bullet_damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_punkgoat_ult_desc",
    "Main": {
      "Props": [
        {
          "Key": "MeleeDamage",
          "Name": "Melee Damage",
          "Scale": {
            "Type": "melee",
            "Value": 0.9
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "HeavyMeleeDamage",
          "Name": "Damage",
          "Scale": {
            "Type": "heavy_melee",
            "Value": 0
          },
          "Type": "melee_damage",
          "Value": 0
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.1
          },
          "Type": "tech_damage",
          "Value": 35
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Title": "On Hit:",
          "Type": "duration",
          "Value": 0.4
        },
        {
          "Key": "SlowPercent",
          "Name": "Move Speed",
          "Title": "On Hit:",
          "Type": "move_speed",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_punkgoat_ult",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 4.826
    }
  },
  "Name": "Bashdown",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.3
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 2000
    },
    "CountsAsLightMelee": {
      "Name": null,
      "Value": 1
    },
    "TossForce": {
      "Name": null,
      "Value": 350
    }
  },
  "Range": {
    "PlaceDistanceInFrontOfCaster": {
      "Name": null,
      "Type": "distance",
      "Value": 6.2
    },
    "PullDownRange": {
      "Name": null,
      "Type": "distance",
      "Value": 3
    },
    "WaveEndRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 8.0
    },
    "WaveStartRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 0.5
    },
    "WaveThickness": {
      "Name": null,
      "Type": "distance",
      "Value": 1
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_punkgoat_ult_t1_desc"
    },
    {
      "AbilityCastRange": 2,
      "AbilityCharges": 1,
      "DescKey": "ability_punkgoat_ult_t2_desc"
    },
    {
      "AbilityCooldownBetweenCharge": -3,
      "CountsAsHeavyMelee": 1,
      "CountsAsLightMelee": -1,
      "DescKey": "ability_punkgoat_ult_t3_desc",
      "HeavyMeleeDamage": {
        "Scale": {
          "Type": "heavy_melee",
          "Value": 0.5
        },
        "Value": 0.0
      },
      "MeleeDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "melee",
          "Value": 0.0
        },
        "Value": 0.0
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
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "bashdown",
      "name": "Bashdown",
      "type": "ability"
    }
  ]
}
````
