---
title: "Cultist Sacrifice"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_non_player_bonus_sacrifice"
canonical_name: "Cultist Sacrifice"
snapshot_id: 40413
source_document_id: 7073
payload_hash: "ef373bc5e9dbe7e86881384b718fc8143a8e2d1cf554659cc985cf71f1a2029f"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.549725+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Cultist Sacrifice

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_non_player_bonus_sacrifice`
- Snapshot ID: `40413`
- Source-Dokument: `7073`
- Kurzinfo: Cultist Sacrifice aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_non_player_bonus"
  ],
  "Cost": 3200,
  "Description": "Target an enemy NPC and consume it for <span class=\"highlight\">180% Bonus Souls</span> and grants a powerful long lasting buff.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      },
      {
        "Key": "NonPlayerBonusWeaponPower",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "NonPlayerBulletResist",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 160
      }
    ],
    "ChargeUp": null,
    "Cooldown": 270,
    "DescKey": "#upgrade_non_player_bonus_sacrifice_desc",
    "Main": [
      {
        "Key": "BaseAttackDamagePercent",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.8
        },
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 10.0
      },
      {
        "Key": "BonusHealth",
        "Scale": {
          "Type": "power_increase",
          "Value": 4.0
        },
        "Type": "health",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50.0
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "UsageFlags": "ConditionallyApplied",
        "Value": 12
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_non_player_bonus_sacrifice",
  "Name": "Cultist Sacrifice",
  "Other": {
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "7m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "BonusAbilityCharges": {
      "Key": "BonusAbilityCharges",
      "Type": "cast",
      "UsageFlags": "ConditionallyApplied",
      "Value": 1
    },
    "BonusSoulsPct": {
      "Key": "BonusSoulsPct",
      "Value": 180
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "UsageFlags": "ConditionallyApplied",
      "Value": 12
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "TrooperEnemy",
    "Neutral",
    "MinionEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BaseAttackDamagePercent": 47,
    "BonusHealth": 300,
    "NonPlayerBonusWeaponPower": 30,
    "NonPlayerBulletResist": 30,
    "TechRadiusMultiplier": 40,
    "TechRangeMultiplier": 40
  },
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-cards.json",
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
      "hero_key": null,
      "hero_name": null,
      "lookup": "cultist sacrifice",
      "name": "Cultist Sacrifice",
      "type": "item"
    }
  ]
}
````
