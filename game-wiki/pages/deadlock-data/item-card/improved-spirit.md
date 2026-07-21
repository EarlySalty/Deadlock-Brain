---
title: "Improved Spirit"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_soaring_spirit"
canonical_name: "Improved Spirit"
snapshot_id: 40458
source_document_id: 7073
payload_hash: "8d4479c86f4a8b99fc35c866d623fc5cd564218c85454c570c3fdb93afd96577"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.626872+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Improved Spirit

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_soaring_spirit`
- Snapshot ID: `40458`
- Source-Dokument: `7073`
- Kurzinfo: Improved Spirit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_spirit"
  ],
  "Cost": 1600,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "TechPower",
        "Value": 18
      }
    ],
    "Type": "Innate"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_soaring_spirit",
  "Name": "Improved Spirit",
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
    "BonusHealth": {
      "Key": "BonusHealth",
      "Type": "health",
      "Value": 75
    },
    "BonusSprintSpeed": {
      "Key": "BonusSprintSpeed",
      "Type": "move_speed",
      "Value": "1m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "Movement",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "OutOfCombatHealthRegen": 3,
    "TechPower": 22
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
      "lookup": "improved spirit",
      "name": "Improved Spirit",
      "type": "item"
    }
  ]
}
````
