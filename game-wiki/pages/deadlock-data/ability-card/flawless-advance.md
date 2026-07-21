---
title: "Flawless Advance"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_fencer_lunge"
canonical_name: "Flawless Advance"
snapshot_id: 39793
source_document_id: 7071
payload_hash: "4f393a8092f8bedf41fecea75f5193bee12e0f4e69c535f810a3b965c2aa1eaa"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.215280+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flawless Advance

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_lunge`
- Snapshot ID: `39793`
- Source-Dokument: `7071`
- Kurzinfo: Flawless Advance aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 26
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
  "Damage": {
    "MaxProcBleedDamagePercent": {
      "Name": null,
      "Type": "tech_damage",
      "Value": 50
    }
  },
  "DescKey": "ability_fencer_lunge_desc",
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxStabs",
        "Name": "Max Lunges",
        "Value": 3
      },
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Value": 0
      }
    ],
    "DescKey": "ability_fencer_lunge_desc",
    "Main": {
      "Props": [
        {
          "Key": "BaseDamage",
          "Name": "Base Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.55
          },
          "Type": "tech_damage",
          "Value": 25
        },
        {
          "Key": "MaxDamageBeforePerfect",
          "Name": "Max Hold Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.9
          },
          "Type": "tech_damage",
          "Value": 40
        }
      ]
    }
  },
  "Info2": {
    "Alt": [],
    "Main": {
      "Props": [
        {
          "Key": "PerfectDamage",
          "Name": "Perfect Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.55
          },
          "Title": "On Perfect Hold:",
          "Type": "tech_damage",
          "Value": 65
        },
        {
          "Key": "HealFixedHealth",
          "Name": "Heal on hero hit",
          "Scale": {
            "Type": "spirit",
            "Value": 0.0
          },
          "Title": "On Perfect Hold:",
          "Type": "healing",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_fencer_lunge",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flawless Advance",
  "Other": {
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.2
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AttackingDashSpeed": {
      "Name": null,
      "Value": 55.88
    },
    "DashAngleThreshold": {
      "Name": null,
      "Value": 89
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 1.85
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 27.94
    },
    "HoldDurationMax": {
      "Name": null,
      "Value": 1.1
    },
    "HoldDurationMin": {
      "Name": null,
      "Value": 0.25
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 2
    },
    "ParryCooldownReduction": {
      "Name": "Parry Cooldown",
      "Value": 5
    },
    "PctTravelDistanceToDamageIn": {
      "Name": null,
      "Value": 80
    },
    "PerfectHoldTimeStart": {
      "Name": "Perfect Window Start",
      "Value": 0.525
    },
    "PerfectWindowDuration": {
      "Name": "Perfect Window Duration",
      "Value": 0.25
    },
    "RecastTime": {
      "Name": null,
      "Value": 5
    },
    "SlashCollisionRadius": {
      "Name": null,
      "Value": 4.05
    }
  },
  "Range": {
    "AttackDashRange": {
      "Name": "Attacking Lunge Distance",
      "Type": "distance",
      "Value": 3.0
    },
    "DashRange": {
      "Name": "Time Window",
      "Type": "distance",
      "Value": 5.0
    },
    "SlashLength": {
      "Name": "Slash Length",
      "Type": "distance",
      "Value": 13
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Type": "distance",
      "Value": 1.6
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_fencer_lunge_t1_desc",
      "HealFixedHealth": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.3
        },
        "Value": 35
      }
    },
    {
      "AbilityCooldown": -12,
      "BulletResist": 60,
      "DashBuffDuration": 1.5,
      "DescKey": "ability_fencer_lunge_t2_desc"
    },
    {
      "AttackDashRange": 3.0,
      "BaseDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 30
      },
      "DashSpeed": 13.97,
      "DescKey": "ability_fencer_lunge_t3_desc",
      "MaxDamageBeforePerfect": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 45
      },
      "PerfectDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 65
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
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "flawless advance",
      "name": "Flawless Advance",
      "type": "ability"
    }
  ]
}
````
