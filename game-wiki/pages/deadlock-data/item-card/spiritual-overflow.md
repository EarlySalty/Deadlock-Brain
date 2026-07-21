---
title: "Spiritual Overflow"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_tech_overflow"
canonical_name: "Spiritual Overflow"
snapshot_id: 40481
source_document_id: 7073
payload_hash: "4b71d951c1463a0a8275107545972de67f34e5f4b4418e5dda3d1666bdeff96a"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.685909+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spiritual Overflow

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_overflow`
- Snapshot ID: `40481`
- Source-Dokument: `7073`
- Kurzinfo: Spiritual Overflow aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Gain bonus <span class=\"highlight\">Fire Rate</span>, <span class=\"highlight\">Spirit Power</span> and <span class=\"highlight\">Spirit Lifesteal</span> by <span class=\"highlight\">charging up</span> when shooting enemy heroes.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 15
      },
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 13
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 90
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "Value": 6
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
        "Key": "BuildUpPerShot",
        "Value": 0.75
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_tech_overflow_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 32
      },
      {
        "Key": "BonusSpirit",
        "Type": "tech_damage",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 40
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_overflow",
  "Name": "Spiritual Overflow",
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
    "BuildUpDuration": {
      "Key": "BuildUpDuration",
      "Type": "duration",
      "Value": 5
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "NonHeroAbilityLifestealTooltipOnly": {
      "Key": "NonHeroAbilityLifestealTooltipOnly",
      "Value": 3
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityLifestealPercentHero": 15,
    "BonusAbilityDurationPercent": 15,
    "BonusFireRate": 20,
    "BonusHealth": 80,
    "BonusSpirit": 30,
    "TechPower": 9
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
      "lookup": "spiritual overflow",
      "name": "Spiritual Overflow",
      "type": "item"
    }
  ]
}
````
