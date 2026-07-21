---
title: "Torment Pulse"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_tech_damage_pulse"
canonical_name: "Torment Pulse"
snapshot_id: 40479
source_document_id: 7073
payload_hash: "62bcf83f658e3494177c7e68aa802e09d58c7ae52fd17a6703f51e1e0d861bcc"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.682084+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Torment Pulse

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_damage_pulse`
- Snapshot ID: `40479`
- Source-Dokument: `7073`
- Kurzinfo: Torment Pulse aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Periodically deals {g:citadel_inline_attribute:'SpiritDamage'} to the closest two enemies nearby.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 100
      },
      {
        "Key": "MeleeResistPercent",
        "Value": 18
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
        "Key": "DamagePulseRadius",
        "Type": "distance",
        "Value": "9m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.4,
    "DescKey": "#upgrade_tech_damage_pulse_desc",
    "Main": [
      {
        "Key": "DamagePulseAmount",
        "Scale": {
          "Type": "spirit",
          "Value": 0.23
        },
        "Type": "tech_damage",
        "Value": 25.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_damage_pulse",
  "Name": "Torment Pulse",
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
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 75,
    "DamagePulseAmount": 30
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
      "lookup": "torment pulse",
      "name": "Torment Pulse",
      "type": "item"
    }
  ]
}
````
