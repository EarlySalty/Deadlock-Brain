---
title: "Snowball"
entity_type: "item"
source: "deadlock_data"
external_id: "cosmetic_item_snowball"
canonical_name: "Snowball"
snapshot_id: 39962
source_document_id: 7072
payload_hash: "9a8112154833e700282f875cd7066d7c4b09c4500e9c376940f82957f38817cf"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.548883+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Snowball

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `cosmetic_item_snowball`
- Snapshot ID: `39962`
- Source-Dokument: `7072`
- Kurzinfo: Snowball aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCharges": 1,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": 1.5,
  "AbilityPostCastDuration": 1,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": null,
  "Damage": 1,
  "Description": "Throw snowballs at your friends and enemies. Distance and other properties improve as you make progress in the <span class=\"highlight\">2025 Holiday Challenge</span>.<br><br><span class=\"diminish\">Throwing a Snowball at an ally won't inflict damage and will reset its cooldown.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "cosmetic_item_snowball",
  "MaxLevel": 32,
  "Name": "Snowball",
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
  "Radius": 5,
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": null,
  "SnowballCount": 1,
  "SnowballSpeed": 630,
  "Spread": 3,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly",
    "AllEnemy"
  ],
  "Tier": null,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-data.json",
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
