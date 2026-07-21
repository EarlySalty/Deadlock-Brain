---
title: "Shadow Step"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_shadow_step"
canonical_name: "Shadow Step"
snapshot_id: 40447
source_document_id: 7073
payload_hash: "f84c4a25f850b7da3026bcc3ad171a7499b94100e6ebdf4038fca28ab45fc433"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.607548+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Shadow Step

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_shadow_step`
- Snapshot ID: `40447`
- Source-Dokument: `7073`
- Kurzinfo: Shadow Step aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "<span class=\"highlight\">Teleport</span> straight ahead.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 3.0,
    "DescKey": "#upgrade_shadow_step_desc",
    "Main": [
      {
        "Key": "AbilityCastRange",
        "LocTokenOverride": "WarpStoneRange",
        "Type": "range",
        "Value": "12m"
      }
    ],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "DamagePulseInterval",
        "Value": 0.5
      },
      {
        "Key": "DamagePulseRadius",
        "Type": "distance",
        "Value": "13m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_shadow_step_passive_desc",
    "Main": [
      {
        "Key": "DamagePulseAmount",
        "Type": "tech_damage",
        "Value": 50
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_shadow_step",
  "Name": "Shadow Step",
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
    "WeaponDamage",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
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
