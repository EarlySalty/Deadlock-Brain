---
title: "Rusted Barrel"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_withering_whip"
canonical_name: "Rusted Barrel"
snapshot_id: 40506
source_document_id: 7073
payload_hash: "9b2385bf3a06deef212bccb40bb5fc299b14ac55d19552487d046f5091fa084e"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.730362+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Rusted Barrel

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_withering_whip`
- Snapshot ID: `40506`
- Source-Dokument: `7073`
- Kurzinfo: Rusted Barrel aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 800,
  "Description": "Target an enemy to reduce their <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Bullet Resistance</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 60
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.5m"
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
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "32m"
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "WitheringWhipDisarmDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_withering_whip_desc",
    "Main": [
      {
        "Key": "FireRateSlow",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyEnemyApplied",
        "Value": 32
      },
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -8
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_withering_whip",
  "Name": "Rusted Barrel",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
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
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 130,
    "BulletArmorReduction": -4,
    "FireRateSlow": 20
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
      "lookup": "rusted barrel",
      "name": "Rusted Barrel",
      "type": "item"
    }
  ]
}
````
