---
title: "Warp Stone"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_warp_stone"
canonical_name: "Warp Stone"
snapshot_id: 40497
source_document_id: 7073
payload_hash: "a5733dc5b916bf629e2a982343f37986951f1fc434710674935fcf6867afe9bc"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.715928+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Warp Stone

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_warp_stone`
- Snapshot ID: `40497`
- Source-Dokument: `7073`
- Kurzinfo: Warp Stone aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Teleport</span> straight ahead, gaining <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "CasterBuffDuration",
        "Type": "duration",
        "Value": 6
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_warp_stone_desc",
    "Main": [
      {
        "Key": "AbilityCastRange",
        "LocTokenOverride": "WarpStoneRange",
        "Type": "range",
        "Value": "11m"
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_warp_stone",
  "Name": "Warp Stone",
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
    "WeaponDamage",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCastRange": "9m",
    "AbilityCooldown": -3,
    "BulletResist": 20
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
      "lookup": "warp stone",
      "name": "Warp Stone",
      "type": "item"
    }
  ]
}
````
