---
title: "Enduring Spirit"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_resilience"
canonical_name: "Enduring Spirit"
snapshot_id: 40437
source_document_id: 7073
payload_hash: "e85d9ffdc29f9aa945fe2bfffd5ffdb84d646e9d1d76c083d33c96e7f0888c4d"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.589391+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Enduring Spirit

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_resilience`
- Snapshot ID: `40437`
- Source-Dokument: `7073`
- Kurzinfo: Enduring Spirit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "AbilityLifestealPercentHero",
        "Type": "healing",
        "Value": 7
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_resilience",
  "Name": "Enduring Spirit",
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
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
  }
}
````
