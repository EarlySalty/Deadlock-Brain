---
title: "Grapple"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "citadel_ability_lash"
canonical_name: "Grapple"
snapshot_id: 39844
source_document_id: 7071
payload_hash: "626e9bd832af66139f97b9848f609d5bf411cc73ef6e94557e0b33f4fe17f3d9"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.314438+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Grapple

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash`
- Snapshot ID: `39844`
- Source-Dokument: `7071`
- Kurzinfo: Grapple aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 30
  },
  "AbilityCharges": {
    "Name": "Charges",
    "Type": "cast",
    "Value": 1
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 35.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": 2
  },
  "DescKey": "citadel_ability_lash_desc",
  "HeroKey": "hero_lash",
  "HeroName": "Lash",
  "Info1": {
    "Alt": [
      {
        "Key": "WeaponDamageBonus",
        "Name": "Weapon Damage",
        "Type": "bullet_damage",
        "Value": 0
      },
      {
        "Key": "WeaponDamageBonusDuration",
        "Name": "Bonus Duration",
        "Type": "duration",
        "Value": 0
      },
      {
        "Key": "WeaponFireRateBonus",
        "Name": "Fire Rate",
        "Type": "fire_rate",
        "Value": 0
      }
    ],
    "DescKey": "citadel_ability_lash_desc",
    "Main": {
      "Props": [
        {
          "Key": "JumpVelocity",
          "Name": "Jump Velocity",
          "Type": "move_speed",
          "Value": 20
        }
      ]
    }
  },
  "Key": "citadel_ability_lash",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Grapple",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "JumpSlowResistance": {
      "Name": null,
      "Value": 0.667
    },
    "LashFriendlies": {
      "Name": null,
      "Value": 1
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "AbilityCooldown": -17.0
    },
    {
      "AbilityCastRange": 20,
      "DescKey": "citadel_ability_lash_t2_desc",
      "WeaponDamageBonus": 7.0,
      "WeaponDamageBonusDuration": 10
    },
    {
      "AbilityCharges": 1,
      "AirControlPercent": 60,
      "DescKey": "citadel_ability_lash_t3_desc",
      "RestoreStaminaOnUse": 1
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
      "lookup": "grapple",
      "name": "Grapple",
      "type": "ability"
    }
  ]
}
````
