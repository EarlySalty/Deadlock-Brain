---
title: "Spirit Lifesteal"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_health_stealing_magic"
canonical_name: "Spirit Lifesteal"
snapshot_id: 40371
source_document_id: 7073
payload_hash: "99ede903cb5f30b4264db8ab25a77cfafa02ee7d7d05c5020d6846c712363e20"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.478898+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spirit Lifesteal

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stealing_magic`
- Snapshot ID: `40371`
- Source-Dokument: `7073`
- Kurzinfo: Spirit Lifesteal aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
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
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 13
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stealing_magic",
  "Name": "Spirit Lifesteal",
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
    "MagicDamage",
    "Healing",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityLifestealPercentHero": 14,
    "BonusHealth": 80,
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
      "lookup": "spirit lifesteal",
      "name": "Spirit Lifesteal",
      "type": "item"
    }
  ]
}
````
