---
title: "Lycan Curse"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_transformation"
canonical_name: "Lycan Curse"
snapshot_id: 39942
source_document_id: 7071
payload_hash: "100aadd601476c31eee64c67b32d93859877e0043402cedbea1c0081b2086df8"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.503156+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Lycan Curse

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_transformation`
- Snapshot ID: `39942`
- Source-Dokument: `7071`
- Kurzinfo: Lycan Curse aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 80
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 15
  },
  "DescKey": "ability_werewolf_transformation_desc",
  "HeroKey": "hero_werewolf_transformed",
  "HeroName": "Silver (Transformed)",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletResist",
        "Name": "Bullet Resist",
        "Type": "bullet_armor_up",
        "Value": 0
      },
      {
        "Key": "TechResist",
        "Name": "Spirit Resist",
        "Type": "tech_armor_up",
        "Value": 0
      },
      {
        "Key": "MissingHealthPercentHeal",
        "Name": "Missing Health as Healing",
        "Type": "healing",
        "Value": 30
      },
      {
        "Key": "KillDurationBonus",
        "Name": "Kill Duration Bonus",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "HealAmount",
        "Name": "Heal Amount",
        "Scale": {
          "Type": "power_increase",
          "Value": 1.0
        },
        "Type": "healing",
        "Value": 0
      },
      {
        "Key": "Stamina",
        "Name": "Stamina",
        "Type": "move_speed",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_transformation_desc",
    "Main": {
      "Props": [
        {
          "Key": "BonusHealth",
          "Name": "Bonus Health",
          "Scale": {
            "Type": "power_increase",
            "Value": 15.0
          },
          "Type": "health",
          "Value": 125
        },
        {
          "Key": "BonusMoveSpeed",
          "Name": "Move Speed",
          "Type": "move_speed",
          "Value": 1.5
        },
        {
          "Key": "BonusSprintSpeed",
          "Name": "Sprint Speed",
          "Type": "move_speed",
          "Value": 0
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": 0.45
          },
          "Type": "fire_rate",
          "Value": 60
        }
      ]
    }
  },
  "Key": "ability_werewolf_transformation",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 7.62
    }
  },
  "Name": "Lycan Curse",
  "Other": {
    "AbilityChargesConditionally": {
      "Name": null,
      "Value": 1
    },
    "AbilityPostCastDuration": {
      "Name": null,
      "Value": 0.5
    },
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "AutoActivateHealthThreshold": {
      "Name": null,
      "Value": 20
    },
    "BonusDurationOnBullet": {
      "Name": null,
      "Value": 0.15
    },
    "BonusDurationOnHeavyMelee": {
      "Name": null,
      "Value": 1.5
    },
    "BonusDurationOnLightMelee": {
      "Name": null,
      "Value": 0.5
    },
    "BonusDurationPerHealthPercentLost": {
      "Name": null,
      "Value": 0.1
    },
    "CameraTurnRateMax": {
      "Name": null,
      "Value": 188
    },
    "EndingWarningSoundDuration": {
      "Name": null,
      "Value": 3.0
    },
    "HeadshotResist": {
      "Name": null,
      "Value": -20
    },
    "LowHealthFraction": {
      "Name": null,
      "Value": 30
    },
    "LowHealthRageBonus": {
      "Name": null,
      "Scale": {
        "Type": "power_increase",
        "Value": 1.8
      },
      "Value": 40
    },
    "MaxRage": {
      "Name": null,
      "Scale": {
        "Type": "power_increase",
        "Value": 9.4
      },
      "Value": 100
    },
    "MaxStacks": {
      "Name": "Max Stacks",
      "Value": 15
    },
    "RagePerDamage": {
      "Name": null,
      "Value": 0.255
    },
    "RagePercentagePerSecondInCombat": {
      "Name": null,
      "Value": 1
    },
    "RagePercentagePerSecondOutOfCombat": {
      "Name": null,
      "Value": -3
    },
    "ReadyDuration": {
      "Name": null,
      "Value": 3
    },
    "StackDuration": {
      "Name": "Stack Duration",
      "Value": 5
    }
  },
  "Slot": "4",
  "Upgrades": [
    {
      "BulletResist": 20,
      "DescKey": "ability_werewolf_transformation_t1_desc",
      "TechResist": 20
    },
    {
      "BonusHealth": 200,
      "BonusMoveSpeed": 4,
      "DescKey": "ability_werewolf_transformation_t2_desc"
    },
    {
      "DescKey": "ability_werewolf_transformation_t3_desc",
      "KillCreditWindow": 1.5,
      "KillDurationBonus": 15
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
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "lycan curse",
      "name": "Lycan Curse",
      "type": "ability"
    }
  ]
}
````
