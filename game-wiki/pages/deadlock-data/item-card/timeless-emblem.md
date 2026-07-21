---
title: "Timeless Emblem"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_timeless_emblem"
canonical_name: "Timeless Emblem"
snapshot_id: 40485
source_document_id: 7073
payload_hash: "014af0641a96f410949a11ea3cb31b3e403c47cd27f9f960cdc1bf8384d03314"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.693931+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Timeless Emblem

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_timeless_emblem`
- Snapshot ID: `40485`
- Source-Dokument: `7073`
- Kurzinfo: Timeless Emblem aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_transcendent_cooldown"
  ],
  "Cost": 9999,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
        "Value": 50
      },
      {
        "Key": "ItemCooldownReduction",
        "Type": "cooldown",
        "Value": 50
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 50
      },
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_timeless_emblem",
  "Name": "Timeless Emblem",
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
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 50
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BonusAbilityDurationPercent": 10,
    "CooldownReduction": 10,
    "ItemCooldownReduction": 10,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
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
