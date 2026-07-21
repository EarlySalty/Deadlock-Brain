---
title: "Aerial Supremacy"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_aerial_supremacy"
canonical_name: "Aerial Supremacy"
snapshot_id: 40262
source_document_id: 7073
payload_hash: "4bdacc47838a046c10869da209f200c7e1b5d7db01a5aa2108c766d1fbcf6975"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.287304+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Aerial Supremacy

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aerial_supremacy`
- Snapshot ID: `40262`
- Source-Dokument: `7073`
- Kurzinfo: Aerial Supremacy aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_superior_stamina"
  ],
  "Cost": 9999,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "Stamina",
        "Value": 7
      },
      {
        "Key": "AirMoveIncreasePercent",
        "Value": 70
      },
      {
        "Key": "StaminaCooldownReduction",
        "Value": 40
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_aerial_supremacy_passive",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aerial_supremacy",
  "Name": "Aerial Supremacy",
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
    "Movement",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "AirMoveIncreasePercent": 30,
    "Stamina": 3,
    "StaminaCooldownReduction": 10
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
  }
}
````
