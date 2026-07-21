---
title: "Exploding Uppercut"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_uppercut"
canonical_name: "Exploding Uppercut"
snapshot_id: 39756
source_document_id: 7071
payload_hash: "19267aa737398bb542fe1d3f1b1d413c7bd1bb88f0b0d5f33ad6ce6564a2daf6"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.142117+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Exploding Uppercut

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_uppercut`
- Snapshot ID: `39756`
- Source-Dokument: `7071`
- Kurzinfo: Exploding Uppercut aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 22.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "citadel_ability_uppercut_desc",
  "Duration": {
    "ExplodeDebuffDuration": {
      "Name": "Fire Rate Slow Duration",
      "Type": "duration",
      "Value": 5
    }
  },
  "HeroKey": "hero_bebop",
  "HeroName": "Bebop",
  "Info1": {
    "Alt": [],
    "DescKey": "citadel_ability_uppercut_desc",
    "Main": {
      "Props": [
        {
          "Key": "UppercutDamage",
          "Name": "Uppercut Damage",
          "Scale": {
            "Type": "melee",
            "Value": 1.0
          },
          "Type": "melee_damage",
          "Value": 0.01
        },
        {
          "Key": "MissingHPHeal",
          "Name": "Missing HP Heal",
          "Type": "healing",
          "Value": 0
        },
        {
          "Key": "TossDuration",
          "Name": "Duration",
          "StatusEffect": "Displacement",
          "Value": 0.5
        }
      ]
    }
  },
  "Info2": {
    "Alt": [
      {
        "Key": "OnLandDamageRadius",
        "Name": "Landing Radius",
        "Type": "distance",
        "Value": 14
      }
    ],
    "Main": {
      "Props": [
        {
          "Key": "LandingDamage",
          "Name": "Area Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.6
          },
          "Title": "On Landing:",
          "Type": "tech_damage",
          "Value": 75
        },
        {
          "Key": "BonusFireRate",
          "Name": "Fire Rate",
          "Scale": {
            "Type": "spirit",
            "Value": -0.186
          },
          "Title": "On Landing:",
          "Type": "slow",
          "Value": -14
        }
      ]
    }
  },
  "Key": "citadel_ability_uppercut",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Exploding Uppercut",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "BuffGunRangePercent": {
      "Name": null,
      "Value": 100
    },
    "EnemyHeroTossVelocity": {
      "Name": null,
      "Value": 20
    },
    "ForceReductionOnAngleDown": {
      "Name": null,
      "Value": 0.75
    },
    "MeleeHalfAngle": {
      "Name": null,
      "Value": 60
    },
    "TossDurationFriendly": {
      "Name": null,
      "Value": 0.3
    },
    "TossVelocity": {
      "Name": null,
      "Value": 25
    }
  },
  "Range": {
    "MeleeAttackLength": {
      "Name": "Melee Range",
      "Type": "distance",
      "Value": 6
    },
    "MeleeRadius": {
      "Name": null,
      "Type": "distance",
      "Value": 2.5
    }
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "BuffBaseWeaponPct": 30,
      "DescKey": "citadel_ability_uppercut_t2_desc",
      "UppercutBuffOnHit": 9
    },
    {
      "DescKey": "citadel_ability_uppercut_t3_desc",
      "MissingHPHeal": 18,
      "RestoreHookCooldown": 1
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
      "hero_key": "hero_bebop",
      "hero_name": "Bebop",
      "lookup": "exploding uppercut",
      "name": "Exploding Uppercut",
      "type": "ability"
    }
  ]
}
````
