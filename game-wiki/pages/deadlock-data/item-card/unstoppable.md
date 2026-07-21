---
title: "Unstoppable"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_unstoppable"
canonical_name: "Unstoppable"
snapshot_id: 40493
source_document_id: 7073
payload_hash: "abdd2358d8ac7d7d1d97e09dc5038ce1ce957b85a7b61c6fe01d6665a76b02d9"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.708082+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Unstoppable

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstoppable`
- Snapshot ID: `40493`
- Source-Dokument: `7073`
- Kurzinfo: Unstoppable aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "Description": "Temporarily suppress <span class=\"highlight\">negative status effects</span> and become <span class=\"highlight\">immune</span> to <span class=\"highlight\">Stun, Silence, Sleep, Root, and Disarm</span>. <br>Cannot be used while <span class=\"highlight\">Stunned</span> or <span class=\"highlight\">Slept</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 25
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 60.0,
    "DescKey": "#upgrade_unstoppable_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 5.5
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstoppable",
  "Name": "Unstoppable",
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
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -35,
    "AbilityDuration": 1.25,
    "BonusHealth": 75,
    "StatusResistancePercent": 15
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
      "lookup": "unstoppable",
      "name": "Unstoppable",
      "type": "item"
    }
  ]
}
````
