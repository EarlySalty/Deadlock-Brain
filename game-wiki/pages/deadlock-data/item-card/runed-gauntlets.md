---
title: "Runed Gauntlets"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_runed_gauntlets"
canonical_name: "Runed Gauntlets"
snapshot_id: 40444
source_document_id: 7073
payload_hash: "47994bb6eae4301fa80654b86058ace5eabfde0c0641d05b4e2a4a2d097c9d0c"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.602350+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Runed Gauntlets

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_runed_gauntlets`
- Snapshot ID: `40444`
- Source-Dokument: `7073`
- Kurzinfo: Runed Gauntlets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Everytime you land a heavy melee, your existing cooldowns get reduced.",
  "Info1": {
    "Alt": [
      {
        "Key": "MeleeResistPercent",
        "Value": 50
      },
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 30
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "MeleeDistanceScale",
        "Value": 150
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 10,
    "DescKey": "#upgrade_runed_gauntlets_parry_desc",
    "Main": [],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_runed_gauntlets_desc",
    "Main": [
      {
        "Key": "CooldownReductionOnHitPct",
        "Type": "cooldown",
        "Value": 16
      },
      {
        "Key": "CooldownReductionOnHitMin",
        "Type": "cooldown",
        "Value": 4
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_runed_gauntlets",
  "Name": "Runed Gauntlets",
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
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "CooldownReductionOnHitPct": 6,
    "MeleeDistanceScale": 50,
    "MeleeResistPercent": 15
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
      "lookup": "runed gauntlets",
      "name": "Runed Gauntlets",
      "type": "item"
    }
  ]
}
````
