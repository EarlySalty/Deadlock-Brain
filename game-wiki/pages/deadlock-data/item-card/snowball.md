---
title: "Snowball"
entity_type: "item_card"
source: "deadlock_data"
external_id: "cosmetic_item_snowball"
canonical_name: "Snowball"
snapshot_id: 40241
source_document_id: 7073
payload_hash: "8936d1796a3f37db025f4842299c23ec8d9124318b9c0232565374da630de2fb"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.255887+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Snowball

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `cosmetic_item_snowball`
- Snapshot ID: `40241`
- Source-Dokument: `7073`
- Kurzinfo: Snowball aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": null,
  "Description": "Throw snowballs at your friends and enemies. Distance and other properties improve as you make progress in the <span class=\"highlight\">2025 Holiday Challenge</span>.<br><br><span class=\"diminish\">Throwing a Snowball at an ally won't inflict damage and will reset its cooldown.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "SnowballSpeed",
        "Value": 630
      },
      {
        "Key": "SnowballCount",
        "Value": 1
      },
      {
        "Key": "Radius",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 18,
    "DescKey": "#cosmetic_item_snowball_desc",
    "Main": [
      {
        "Key": "Damage",
        "Value": 1
      },
      {
        "Key": "AbilityCharges",
        "Type": "cast",
        "Value": 1
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "cosmetic_item_snowball",
  "Name": "Snowball",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": 1.5
    },
    "AbilityPostCastDuration": {
      "Key": "AbilityPostCastDuration",
      "Value": 1
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
    "MaxLevel": {
      "Key": "MaxLevel",
      "Value": 32
    },
    "Progression": {
      "Charges": {
        "Behavior": "UsePrevious",
        "Levels": {
          "0": 1.0,
          "16": 3.0,
          "24": 4.0,
          "32": 5.0,
          "8": 2.0
        }
      },
      "Cooldown": {
        "Levels": {
          "0": 18.0,
          "1": 16.0,
          "2": 14.0,
          "3": 12.0,
          "4": 10.0,
          "5": 8.0
        }
      },
      "Damage": {
        "Levels": {
          "0": 1.0,
          "1": 2.0,
          "2": 4.0,
          "3": 8.0,
          "32": 20.0,
          "4": 12.0
        }
      },
      "Key": "Progression",
      "Radius": {
        "Levels": {
          "0": 5.0,
          "32": 20.0
        }
      },
      "SnowballCount": {
        "Behavior": "UsePrevious",
        "Levels": {
          "0": 1.0,
          "16": 2.0,
          "32": 3.0
        }
      },
      "Speed": {
        "Levels": {
          "0": 630.0,
          "1": 787.0,
          "2": 944.0,
          "3": 1102.0,
          "32": 2756.0,
          "4": 1260.0,
          "5": 1417.0,
          "6": 1575.0
        }
      }
    },
    "Spread": {
      "Key": "Spread",
      "Value": 3
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": null,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly",
    "AllEnemy"
  ],
  "Tier": null,
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
      "lookup": "snowball",
      "name": "Snowball",
      "type": "item"
    }
  ]
}
````
