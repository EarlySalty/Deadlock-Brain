---
title: "Headhunter"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_headhunter"
canonical_name: "Headhunter"
snapshot_id: 40359
source_document_id: 7073
payload_hash: "36789912e15a766783e628704e3611fb88bd568e6c5e6d3d1c7bf295e8e9a534"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.458260+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Headhunter

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_headhunter`
- Snapshot ID: `40359`
- Source-Dokument: `7073`
- Kurzinfo: Headhunter aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_headshot_booster"
  ],
  "Cost": 3200,
  "Description": "Your next <span class=\"highlight\">headshot</span> against an enemy Hero deals {g:citadel_inline_attribute:'BonusWeaponDamage'}, {g:citadel_inline_attribute:'Heal'} you, and briefly grants {g:citadel_inline_attribute:'BonusMoveSpeed'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 5
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
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
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.75m"
      },
      {
        "Key": "MovementSpeedBonusDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 8,
    "DescKey": "#upgrade_headhunter_desc",
    "Main": [
      {
        "Key": "HeadShotBonusDamage",
        "Scale": {
          "Type": "power_increase",
          "Value": 4.0
        },
        "Type": "bullet_damage",
        "Value": 75.0
      },
      {
        "Key": "HealPercentPerHeadshot",
        "Scale": {
          "Type": "power_increase",
          "Value": 0.06
        },
        "Type": "healing",
        "Value": 4
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headhunter",
  "Name": "Headhunter",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -3,
    "HeadShotBonusDamage": 75,
    "HealPercentPerHeadshot": 4
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
      "lookup": "headhunter",
      "name": "Headhunter",
      "type": "item"
    }
  ]
}
````
