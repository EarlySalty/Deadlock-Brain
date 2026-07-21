---
title: "Burst Fire"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_burst_fire"
canonical_name: "Burst Fire"
snapshot_id: 40293
source_document_id: 7073
payload_hash: "726ffb34db24e0e6960dfdbc79d18c3eac7f556919172ec70cfa769036e3e9ab"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.339445+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Burst Fire

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_burst_fire`
- Snapshot ID: `40293`
- Source-Dokument: `7073`
- Kurzinfo: Burst Fire aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_rapid_rounds"
  ],
  "Cost": 3200,
  "Description": "Briefly gain <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Move Speed</span> when one of your bullets hits an enemy hero.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "SlideScale",
        "Type": "movement_speed",
        "Value": 50
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 10
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 9,
    "DescKey": "#upgrade_burst_fire_desc",
    "Main": [
      {
        "Key": "ActivatedFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 32
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.25m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_burst_fire",
  "Name": "Burst Fire",
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
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -1,
    "ActivatedFireRate": 15,
    "BonusFireRate": 14,
    "BonusMoveSpeed": "1.5m",
    "SlideScale": 50
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
      "lookup": "burst fire",
      "name": "Burst Fire",
      "type": "item"
    }
  ]
}
````
