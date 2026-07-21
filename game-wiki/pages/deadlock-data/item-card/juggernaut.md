---
title: "Juggernaut"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_juggernaut"
canonical_name: "Juggernaut"
snapshot_id: 40387
source_document_id: 7073
payload_hash: "04974f1a0818f0e6374848431ef6d1e26a3d8df65d3e10b98c0b80a1af3f2296"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.506173+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Juggernaut

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_juggernaut`
- Snapshot ID: `40387`
- Source-Dokument: `7073`
- Kurzinfo: Juggernaut aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_cardio_calibrator"
  ],
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "2.5m"
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 25
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SlowResistancePercent",
        "Type": "move_speed",
        "Value": 50
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "FireRateSlowDuration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_juggernaut_slow_desc",
    "Main": [
      {
        "Key": "FireRateSlow",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyEnemyApplied",
        "Value": 40
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_juggernaut",
  "Name": "Juggernaut",
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
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusHealthRegen": 8,
    "BonusMoveSpeed": "3.5m",
    "FireRateSlow": 20,
    "MeleeResistPercent": 15,
    "SlowResistancePercent": 15
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
      "lookup": "juggernaut",
      "name": "Juggernaut",
      "type": "item"
    }
  ]
}
````
