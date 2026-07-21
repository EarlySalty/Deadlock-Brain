---
title: "Active Reload"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_active_reload"
canonical_name: "Active Reload"
snapshot_id: 40260
source_document_id: 7073
payload_hash: "05baebd7f6847dbf1f7b441789381ff56bf8c6826b606d1559d7ee2187bdfa21"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.283941+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Active Reload

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_active_reload`
- Snapshot ID: `40260`
- Source-Dokument: `7073`
- Kurzinfo: Active Reload aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "While reloading, pressing {g:citadel_binding:'Reload'} during the highlighted portion will <span class=\"highlight\">instantly finish your reload</span> and grant you <span class=\"highlight\">Fire Rate</span>, <span class=\"highlight\">Bullet Lifesteal</span> and <span class=\"highlight\">Move Speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
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
    "Cooldown": 12,
    "DescKey": "#upgrade_active_reload_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "BulletLifestealPercent",
        "Type": "healing",
        "UsageFlags": "ConditionallyApplied",
        "Value": 16
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_active_reload",
  "Name": "Active Reload",
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
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusClipSizePercent": 10,
    "BonusFireRate": 15,
    "BonusMoveSpeed": "3m",
    "BulletLifestealPercent": 12
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
      "lookup": "active reload",
      "name": "Active Reload",
      "type": "item"
    }
  ]
}
````
