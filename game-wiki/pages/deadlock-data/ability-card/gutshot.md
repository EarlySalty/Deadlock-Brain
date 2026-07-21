---
title: "Gutshot"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_priest_knockback"
canonical_name: "Gutshot"
snapshot_id: 39872
source_document_id: 7071
payload_hash: "6c0a41169bae4897edb97df0bc90a6d8476d58a343100ec3ce5d54a00a074fd0"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.368502+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Gutshot

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_priest_knockback`
- Snapshot ID: `39872`
- Source-Dokument: `7071`
- Kurzinfo: Gutshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.25
  },
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 10
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 23
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_priest_knockback_desc",
  "HeroKey": "hero_priest",
  "HeroName": "Venator",
  "Info1": {
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Name": "Debuff Duration",
        "Value": 0
      },
      {
        "Key": "WallStunDistance",
        "Name": "Wall Stun Range",
        "Type": "distance",
        "Value": 7
      }
    ],
    "DescKey": "ability_priest_knockback_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 0.7
          },
          "Type": "bullet_damage",
          "Value": 60
        },
        {
          "Key": "BonusDamage",
          "Name": "Bonus Damage",
          "Scale": {
            "Type": "weapon_damage_increase",
            "Value": 0.8
          },
          "Title": "On Wall Hit:",
          "Type": "bullet_damage",
          "Value": 30
        },
        {
          "Key": "StunDuration",
          "Name": "Stun Duration",
          "StatusEffect": "Stun",
          "Title": "On Wall Hit:",
          "Value": 0.6
        }
      ]
    }
  },
  "Key": "ability_priest_knockback",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Gutshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 99
    },
    "BonusKnockbackDistance": {
      "Name": null,
      "Value": 3.5
    },
    "KnockbackSpeed": {
      "Name": null,
      "Value": 1200
    },
    "MaxPushForceHorizontal": {
      "Name": null,
      "Value": 1400
    },
    "MaxPushForceVertical": {
      "Name": null,
      "Value": 500
    },
    "PushForce": {
      "Name": null,
      "Value": 6
    },
    "SelfPushForce": {
      "Name": null,
      "Value": 500
    },
    "TargetingConeAngle": {
      "Name": "Attack Angle",
      "Value": 60
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "AbilityCooldown": -10,
      "DescKey": "ability_priest_knockback_t2_desc",
      "StunDuration": 0.4
    },
    {
      "BuffDuration": 5,
      "DescKey": "ability_priest_knockback_t3_desc"
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
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "gutshot",
      "name": "Gutshot",
      "type": "ability"
    }
  ]
}
````
