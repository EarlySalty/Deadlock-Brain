---
title: "upgrade_belt_fed_magazine"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_belt_fed_magazine"
canonical_name: "upgrade_belt_fed_magazine"
snapshot_id: 40281
source_document_id: 7073
payload_hash: "a618db9902e78f5c1f27fba36d6f4bd2877849960a72a01cf6892558973d205b"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.319461+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_belt_fed_magazine

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_belt_fed_magazine`
- Snapshot ID: `40281`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_belt_fed_magazine aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "SpinUpTime",
        "Value": 3.5
      },
      {
        "Key": "InitialFireRateDecrease",
        "Value": 40
      },
      {
        "Key": "MaxFireRateIncrease",
        "Value": 60
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_belt_fed_magazine_desc",
    "Main": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 125
      }
    ],
    "Type": null
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_belt_fed_magazine",
  "Name": null,
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
    "SpinUpDecay": {
      "Key": "SpinUpDecay",
      "Value": 1.0
    }
  },
  "ShopFilters": [
    "FireRate",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
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
