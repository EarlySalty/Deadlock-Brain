---
title: "Slam Fire"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_werewolf_unloadgun"
canonical_name: "Slam Fire"
snapshot_id: 39935
source_document_id: 7071
payload_hash: "7dd936b4d0d9efe78158d1069bdae4f444f7100b1288abe5634dcb07c97a9476"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.490750+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Slam Fire

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_unloadgun`
- Snapshot ID: `39935`
- Source-Dokument: `7071`
- Kurzinfo: Slam Fire aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.45
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Type": "range",
    "Value": 0.0254
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "AbilityDuration": {
    "Name": "Duration",
    "Type": "duration",
    "Value": 5
  },
  "DescKey": "ability_werewolf_unloadgun_desc",
  "HeroKey": "hero_werewolf",
  "HeroName": "Silver",
  "Info1": {
    "Alt": [
      {
        "Key": "AccuracyPercentage",
        "Name": "Weapon Accuracy",
        "Type": "distance",
        "Value": -30
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Name": "Weapon Damage",
        "Type": "bullet_damage",
        "Value": 0
      },
      {
        "Key": "BonusCurrentHealthDamagePercentage",
        "Name": "Bonus Current Health as Damage",
        "Type": "bullet_damage",
        "Value": 0
      }
    ],
    "DescKey": "ability_werewolf_unloadgun_desc",
    "Main": {
      "Props": [
        {
          "Key": "MaxShots",
          "Name": "Max Shots",
          "Type": "cast",
          "Value": 3
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Type": "fire_rate",
          "Value": 300
        },
        {
          "Key": "CurrentHealthDamagePercentage",
          "Name": "Current Health as Damage",
          "Type": "bullet_damage",
          "Value": 2.5
        }
      ]
    }
  },
  "Key": "ability_werewolf_unloadgun",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": 5.08
    }
  },
  "Name": "Slam Fire",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BulletEffectiveness": {
      "Name": null,
      "Value": 0.1
    },
    "BulletRadiusOverride": {
      "Name": null,
      "Value": 7
    },
    "BulletSpread": {
      "Name": "Gun Spread",
      "Value": -1
    },
    "Damage": {
      "Name": "Damage",
      "Value": 40
    },
    "DebuffDuration": {
      "Name": "Debuff Duration",
      "Value": 3
    },
    "LingerDuration": {
      "Name": "Linger Duration",
      "Value": 0.1
    },
    "ProcChance": {
      "Name": "Proc Chance",
      "Value": 100
    },
    "RecoilDelayFactor": {
      "Name": null,
      "Value": 0.05
    },
    "RecoilRecoverySpeed": {
      "Name": null,
      "Value": 0.1
    },
    "RecoilSpeed": {
      "Name": null,
      "Value": 12
    },
    "RecoilStrength": {
      "Name": null,
      "Value": 12
    },
    "SpreadPenaltyPerShot": {
      "Name": null,
      "Value": 0.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "BaseAttackDamagePercent": 15
    },
    {
      "AbilityCooldown": -10
    },
    {
      "BonusCurrentHealthDamagePercentage": 7,
      "DescKey": "ability_werewolf_unloadgun_t3_desc",
      "MaxStacks": 3,
      "StackDuration": 3
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
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "slam fire",
      "name": "Slam Fire",
      "type": "ability"
    }
  ]
}
````
