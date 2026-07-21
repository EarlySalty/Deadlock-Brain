---
title: "Rescue Beam"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_rescue_beam"
canonical_name: "Rescue Beam"
snapshot_id: 40436
source_document_id: 7073
payload_hash: "693d0b1aa42401afcac3a90b68ca4c9b635c38c526fac263b5674341ac758020"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.587637+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Rescue Beam

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_rescue_beam`
- Snapshot ID: `40436`
- Source-Dokument: `7073`
- Kurzinfo: Rescue Beam aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heals</span> a target allied hero and yourself for a percentage of <span class=\"highlight\">Max Health</span>. Once while healing, you can <span class=\"highlight\">Pull</span> the target towards you. Can be self-cast.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 6
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
        "Key": "AbilityChannelTime",
        "Type": "cast",
        "Value": 2.5
      },
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "35m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60.0,
    "DescKey": "#upgrade_rescue_beam_desc",
    "Main": [
      {
        "Key": "HealPercentAmount",
        "Type": "healing",
        "Value": 20
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "0m"
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rescue_beam",
  "Name": "Rescue Beam",
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
    "HealInterval": {
      "Key": "HealInterval",
      "Value": 0.2
    },
    "SelfModifier": {
      "Key": "SelfModifier",
      "Value": 100
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 6
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -45,
    "HealPercentAmount": 15,
    "TechRadiusMultiplier": 20,
    "TechRangeMultiplier": 20
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
      "lookup": "rescue beam",
      "name": "Rescue Beam",
      "type": "item"
    }
  ]
}
````
