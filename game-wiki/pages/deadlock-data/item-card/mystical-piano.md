---
title: "Mystical Piano"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_mystical_piano"
canonical_name: "Mystical Piano"
snapshot_id: 40410
source_document_id: 7073
payload_hash: "8a5b244d971c381ce3a56e1c5891124e3a88f472a7d7b9378a3636e4732ed230"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.544368+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Mystical Piano

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystical_piano`
- Snapshot ID: `40410`
- Source-Dokument: `7073`
- Kurzinfo: Mystical Piano aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 9999,
  "Description": "After a short delay, enemies in the target area will be stunned and have their stamina depleted. After the stun they will be temporarily dazed.",
  "Info1": {
    "Alt": [
      {
        "Key": "StunDelay",
        "Value": 1.7
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "12m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 23.0,
    "DescKey": "#upgrade_mystical_piano_desc",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 2.0
      },
      {
        "Key": "DazeDuration",
        "Type": "duration",
        "Value": 2.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_mystical_piano",
  "Name": "Mystical Piano",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 1.7
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
    "DazeMoveSpeed": {
      "Key": "DazeMoveSpeed",
      "Type": "slow",
      "UsageFlags": "ConditionallyApplied",
      "Value": "2m"
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -15,
    "Radius": "3m"
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
      "lookup": "mystical piano",
      "name": "Mystical Piano",
      "type": "item"
    }
  ]
}
````
