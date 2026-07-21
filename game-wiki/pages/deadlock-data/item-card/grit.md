---
title: "Grit"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_grit"
canonical_name: "Grit"
snapshot_id: 40356
source_document_id: 7073
payload_hash: "ba2d70d60fb1c22ab2e84beba7f5228b2f40224c924fd23625aa58884c908ad6"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.452486+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Grit

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_grit`
- Snapshot ID: `40356`
- Source-Dokument: `7073`
- Kurzinfo: Grit aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 800,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> for a short duration.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.0
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BarrierDuration",
        "Type": "duration",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60,
    "DescKey": "#upgrade_grit_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 200
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_grit",
  "Name": "Grit",
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
  "Tier": 1,
  "Upgrades": {
    "AbilityCooldown": -25,
    "CombatBarrier": 250,
    "OutOfCombatHealthRegen": 10
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
      "lookup": "grit",
      "name": "Grit",
      "type": "item"
    }
  ]
}
````
