---
title: "Recharging Rush"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_rechargingbullets"
canonical_name: "Recharging Rush"
snapshot_id: 40430
source_document_id: 7073
payload_hash: "00fa9e1390fb60f4970d5ce0c49feb2bea10e51083016c1544be07a05034f2ec"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.578188+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Recharging Rush

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rechargingbullets`
- Snapshot ID: `40430`
- Source-Dokument: `7073`
- Kurzinfo: Recharging Rush aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Dealing significant {g:citadel_inline_attribute:'WeaponDamage'} replenishes a charge for <span class=\"highlight\">each of your charged abilities</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 10
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
        "Key": "DamageWindow",
        "Value": 3.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25,
    "DescKey": "#upgrade_rechargingbullets_desc",
    "Main": [
      {
        "Key": "DamageThreshold",
        "Value": 200
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rechargingbullets",
  "Name": "Recharging Rush",
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
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -12,
    "BaseAttackDamagePercent": 30,
    "BonusClipSizePercent": 30
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
      "lookup": "recharging rush",
      "name": "Recharging Rush",
      "type": "item"
    }
  ]
}
````
