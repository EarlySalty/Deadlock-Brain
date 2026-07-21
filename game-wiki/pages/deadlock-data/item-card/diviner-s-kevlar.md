---
title: "Diviner's Kevlar"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_diviners_kevlar"
canonical_name: "Diviner's Kevlar"
snapshot_id: 40329
source_document_id: 7073
payload_hash: "209d8f4c011cf0b2589a8f85aafac0f7aba3c2c3ff353cea44c3c826d52e0a82"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.403662+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Diviner's Kevlar

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_diviners_kevlar`
- Snapshot ID: `40329`
- Source-Dokument: `7073`
- Kurzinfo: Diviner's Kevlar aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Upon casting an <span class=\"highlight\">ultimate ability</span> gain a <span class=\"highlight\">Barrier</span> and temporary <span class=\"highlight\">Spirit Power</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 15
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
        "Value": 20
      }
    ],
    "ChargeUp": null,
    "Cooldown": 40.0,
    "DescKey": "#upgrade_diviners_kevlar_desc",
    "Main": [
      {
        "Key": "CombatBarrier",
        "Type": "combat_barrier",
        "UsageFlags": "ConditionallyApplied",
        "Value": 1000
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 35
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_diviners_kevlar",
  "Name": "Diviner's Kevlar",
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
    "Durability",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -14,
    "BonusAbilityDurationPercent": 15,
    "CombatBarrier": 500,
    "TechPower": 55
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
      "lookup": "diviner's kevlar",
      "name": "Diviner's Kevlar",
      "type": "item"
    }
  ]
}
````
