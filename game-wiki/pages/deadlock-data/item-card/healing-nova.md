---
title: "Healing Nova"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_health_nova"
canonical_name: "Healing Nova"
snapshot_id: 40368
source_document_id: 7073
payload_hash: "1bdd4e84c74818c4af0d39f421991ce2002e807a07b80f1c8303daa2ba0fb14f"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.474139+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Healing Nova

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_nova`
- Snapshot ID: `40368`
- Source-Dokument: `7073`
- Kurzinfo: Healing Nova aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> yourself and nearby allies.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 5
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 8
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
        "Key": "RegenDuration",
        "Type": "duration",
        "Value": 2
      },
      {
        "Key": "AuraRadius",
        "Type": "distance",
        "Value": "18m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 60,
    "DescKey": "#upgrade_health_nova_desc",
    "Main": [
      {
        "Key": "TotalHealthRegen",
        "Scale": {
          "Type": "power_increase",
          "Value": 6
        },
        "Type": "healing",
        "Value": 325
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_nova",
  "Name": "Healing Nova",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
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
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 5
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
  "Tier": 3,
  "Upgrades": {
    "TechPower": 12,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12,
    "TotalHealthRegen": 425
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
      "lookup": "healing nova",
      "name": "Healing Nova",
      "type": "item"
    }
  ]
}
````
