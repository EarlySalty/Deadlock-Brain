---
title: "upgrade_aoe_smoke_bomb"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_aoe_smoke_bomb"
canonical_name: "upgrade_aoe_smoke_bomb"
snapshot_id: 40267
source_document_id: 7073
payload_hash: "d4952aad8ab6dae3507efa323d8cdc879fd3c10db4a170e84225fc4dd41b5439"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.295323+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_aoe_smoke_bomb

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_smoke_bomb`
- Snapshot ID: `40267`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_aoe_smoke_bomb aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 25
      },
      {
        "Key": "Radius",
        "Type": "distance",
        "Value": "30m"
      },
      {
        "Key": "InvisMoveSpeedMod",
        "Type": "move_speed",
        "Value": "1.5m"
      },
      {
        "Key": "SpottedRadius",
        "Value": "20m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 64.0,
    "DescKey": "#upgrade_aoe_smoke_bomb_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aoe_smoke_bomb",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 1.5
    },
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
    "FullInvisDistance": {
      "Key": "FullInvisDistance",
      "Value": "50m"
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisCancelOnDamage": {
      "Key": "InvisCancelOnDamage",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 0.3
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 0.25
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 3,
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
