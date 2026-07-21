---
title: "Blood Tribute"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_blood_tribute"
canonical_name: "Blood Tribute"
snapshot_id: 40284
source_document_id: 7073
payload_hash: "4aa5799a19f54ca0a87e1f249ff5e6f5c83678d13d7f3227493c7826066cc671"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.324804+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Blood Tribute

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_blood_tribute`
- Snapshot ID: `40284`
- Source-Dokument: `7073`
- Kurzinfo: Blood Tribute aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCastToggle",
  "Components": null,
  "Cost": 3200,
  "Description": "Toggle: Continually sacrifice Health to improve {g:citadel_inline_attribute:'FireRate'}, <span class=\"highlight\">Debuff Resistance</span> and <span class=\"highlight\">Move Speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "InnateStatusResistancePercent",
        "Type": "duration",
        "Value": 8
      },
      {
        "Key": "TechResist",
        "Value": 8
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 4
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
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      },
      {
        "Key": "StatusResistancePercent",
        "Type": "duration",
        "Value": 35
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_blood_tribute_desc",
    "Main": [
      {
        "Key": "HealthDrainedPerSecond",
        "Type": "damage",
        "Value": 50
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_blood_tribute",
  "Name": "Blood Tribute",
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
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.1
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusFireRate": 30,
    "HealthDrainedPerSecond": -20,
    "OutOfCombatHealthRegen": 8,
    "TechResist": 14
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
      "lookup": "blood tribute",
      "name": "Blood Tribute",
      "type": "item"
    }
  ]
}
````
