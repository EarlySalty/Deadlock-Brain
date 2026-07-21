---
title: "Riposte"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_fencer_riposte"
canonical_name: "Riposte"
snapshot_id: 39792
source_document_id: 7071
payload_hash: "a473e0615a9aeb49ad560e95dc07626678bd8d4daa5637492bb1ea5e1eda8b73"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.213101+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Riposte

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_riposte`
- Snapshot ID: `39792`
- Source-Dokument: `7071`
- Kurzinfo: Riposte aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": {
    "Name": "Channel Duration",
    "Type": "cast",
    "Value": 0.8
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 22
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "Debuff": {
    "MoveSpeedMax": {
      "Name": "Move Speed",
      "Type": "slow",
      "Value": 4
    }
  },
  "DescKey": "ability_fencer_riposte_desc",
  "Health": {
    "AbilityLifestealPercentHero": {
      "Name": "Spirit Lifesteal",
      "Type": "healing",
      "Value": 50
    }
  },
  "HeroKey": "hero_fencer",
  "HeroName": "Apollo",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_fencer_riposte_desc",
    "Main": {
      "Props": [
        {
          "Key": "ParryWindow",
          "Name": "Invulnerability Duration",
          "Value": 0.3
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "MeleeResistReductionDuration",
        "Name": "Melee Resist Reduction Duration",
        "Type": "duration",
        "Value": 3.0
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "Title": "On Pommel Hit:",
          "Type": "duration",
          "Value": 0.6
        },
        {
          "Key": "MeleeResistReduction",
          "Name": "Melee Resist",
          "Title": "On Pommel Hit:",
          "Type": "bullet_armor_down",
          "Value": -22
        }
      ]
    }
  },
  "Key": "ability_fencer_riposte",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Riposte",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CounterattackAntiMashDelay": {
      "Name": null,
      "Value": 0.2
    },
    "DamageThreshold": {
      "Name": "Damage Threshold",
      "Scale": {
        "Type": "power_increase",
        "Value": 4
      },
      "Value": 60
    },
    "DampingFactor": {
      "Name": null,
      "Value": 0.5
    },
    "DashGraceWindow": {
      "Name": null,
      "Value": 1.3
    },
    "DashRadius": {
      "Name": "Radius",
      "Value": 2.2
    },
    "DashSpeed": {
      "Name": "Dash Speed",
      "Value": 76.2
    },
    "LiftHeight": {
      "Name": null,
      "Value": 240
    },
    "SideMoveSpeed": {
      "Name": null,
      "Value": -100
    },
    "SlashConeAngle": {
      "Name": null,
      "Value": 90
    },
    "SlashHalfWidth": {
      "Name": null,
      "Value": 1
    },
    "SlashRadius": {
      "Name": "Slash Radius",
      "Value": 6
    },
    "SlowDuration": {
      "Name": "Slow Duration",
      "Value": 4
    },
    "SlowPercent": {
      "Name": "Move Speed",
      "Value": 40
    },
    "TurnRateMax": {
      "Name": null,
      "Value": 10
    }
  },
  "Range": {
    "DashRange": {
      "Name": "Time Window",
      "Type": "distance",
      "Value": 35
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "MeleeResistReduction": -30
    },
    {
      "StunDuration": 1.6
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
      "lookup": "riposte",
      "name": "Riposte",
      "type": "ability"
    }
  ]
}
````
