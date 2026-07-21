---
title: "Infuser"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_infuser"
canonical_name: "Infuser"
snapshot_id: 40384
source_document_id: 7073
payload_hash: "8b7a7444c94aa4c55c3d01de7f8e9b2b6d9466851cd7daf5644a95032943d5f5"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.501336+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Infuser

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_infuser`
- Snapshot ID: `40384`
- Source-Dokument: `7073`
- Kurzinfo: Infuser aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Gain <span class=\"highlight\">Spirit Lifesteal</span> and <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityLifestealPercentHeroPassive",
        "Type": "healing",
        "Value": 13
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
      },
      {
        "Key": "BonusHealth",
        "Value": 100
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
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": 30.0,
    "DescKey": "#upgrade_infuser_desc",
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 70
      },
      {
        "Key": "BonusSpirit",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_infuser",
  "Name": "Infuser",
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
    "NonHeroAbilityLifestealTooltipOnly": {
      "Key": "NonHeroAbilityLifestealTooltipOnly",
      "Value": 3
    }
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -10,
    "AbilityLifestealPercentHeroPassive": 16,
    "BonusHealth": 50,
    "BonusSpirit": 30,
    "TechResist": 10
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
      "lookup": "infuser",
      "name": "Infuser",
      "type": "item"
    }
  ]
}
````
