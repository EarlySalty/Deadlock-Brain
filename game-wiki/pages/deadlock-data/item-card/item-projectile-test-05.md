---
title: "item_projectile_test_05"
entity_type: "item_card"
source: "deadlock_data"
external_id: "item_projectile_test_05"
canonical_name: "item_projectile_test_05"
snapshot_id: 40247
source_document_id: 7073
payload_hash: "87e9f061a686f8fa72dae5fd72803f7ce57d65169acec40af17516426a88bda3"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.265186+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# item_projectile_test_05

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `item_projectile_test_05`
- Snapshot ID: `40247`
- Source-Dokument: `7073`
- Kurzinfo: item_projectile_test_05 aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "0.5m"
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "30m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.0,
    "DescKey": "#item_projectile_test_05_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "item_projectile_test_05",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.01
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
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
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "0.5m"
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
