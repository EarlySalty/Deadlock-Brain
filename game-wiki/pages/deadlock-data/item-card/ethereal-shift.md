---
title: "Ethereal Shift"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_self_bubble"
canonical_name: "Ethereal Shift"
snapshot_id: 40446
source_document_id: 7073
payload_hash: "22501bf537dc90fec9d53b7974af32689f86972d413c236fafd38c37f844f996"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.605833+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Ethereal Shift

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_self_bubble`
- Snapshot ID: `40446`
- Source-Dokument: `7073`
- Kurzinfo: Ethereal Shift aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "You enter a void state and become <span class=\"highlight\">untargetable and invincible</span> for a short duration, during which you float slowly and cannot perform actions. Afterwards you gain <span class=\"highlight\">Spirit Power, Move Speed, and Spirit Resist</span>.<br>Can be canceled early.<br><span class=\"diminish\">Activation cancels any active ability.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSpirit",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_up",
        "UsageFlags": "ConditionallyApplied",
        "Value": 30
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3m"
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 5
      },
      {
        "Key": "FloatMoveSpeed",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2.5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_self_bubble_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "ShiftingVeilDuration",
        "Type": "duration",
        "Value": 4.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_self_bubble",
  "Name": "Ethereal Shift",
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
    "DampingFactor": {
      "Key": "DampingFactor",
      "Value": 3
    },
    "LiftHeight": {
      "Key": "LiftHeight",
      "Value": 200
    }
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -10,
    "AbilityDuration": 2,
    "BonusMoveSpeed": "2m",
    "BonusSpirit": 30,
    "FloatMoveSpeed": "3.5m",
    "TechResist": 10
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
      "lookup": "ethereal shift",
      "name": "Ethereal Shift",
      "type": "item"
    }
  ]
}
````
