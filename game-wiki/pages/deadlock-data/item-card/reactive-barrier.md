---
title: "Reactive Barrier"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_vex_barrier"
canonical_name: "Reactive Barrier"
snapshot_id: 40496
source_document_id: 7073
payload_hash: "5586dab38e9349ed6963d2b982e78c49511565de120a2eb2fbe330a62c5eadca"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.713869+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Reactive Barrier

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_vex_barrier`
- Snapshot ID: `40496`
- Source-Dokument: `7073`
- Kurzinfo: Reactive Barrier aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> when you are <span class=\"highlight\">Stunned, Chained, Immobilized, Slept or Silenced</span>.",
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
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 10
      }
    ],
    "ChargeUp": null,
    "Cooldown": 55,
    "DescKey": "#upgrade_vex_barrier_desc",
    "Main": [
      {
        "Key": "VexBarrierCombatBarrier",
        "LocTokenOverride": "CombatBarrier",
        "Scale": {
          "Type": "spirit",
          "Value": 1.8
        },
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 325.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_vex_barrier",
  "Name": "Reactive Barrier",
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
  "Tier": 2,
  "Upgrades": {
    "AbilityCooldown": -15,
    "VexBarrierCombatBarrier": 375
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
      "lookup": "reactive barrier",
      "name": "Reactive Barrier",
      "type": "item"
    }
  ]
}
````
