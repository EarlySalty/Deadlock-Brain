---
title: "Divine Barrier"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_divine_barrier"
canonical_name: "Divine Barrier"
snapshot_id: 40328
source_document_id: 7073
payload_hash: "a84009c2b47fef7a6ccbaa951df847b8c4e5bb6ae6adc3d8dfa89d2e86255a25"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.401669+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Divine Barrier

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_divine_barrier`
- Snapshot ID: `40328`
- Source-Dokument: `7073`
- Kurzinfo: Divine Barrier aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_guardian_ward"
  ],
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Remove all non-stun debuffs</span> from the target and provide them with a <span class=\"highlight\">Barrier</span> and <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast. Cooldown is reduced by half when cast on someone else.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 10
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1.5
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
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 6
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45,
    "DescKey": "#upgrade_divine_barrier_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 600
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "2.75m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_divine_barrier",
  "Name": "Divine Barrier",
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
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "CooldownReductionPctOnOthers": {
      "Key": "CooldownReductionPctOnOthers",
      "Value": 50
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 10
    }
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -27,
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
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": null,
      "hero_name": null,
      "lookup": "divine barrier",
      "name": "Divine Barrier",
      "type": "item"
    }
  ]
}
````
