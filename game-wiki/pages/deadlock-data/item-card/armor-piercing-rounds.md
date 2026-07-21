---
title: "Armor Piercing Rounds"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_aprounds"
canonical_name: "Armor Piercing Rounds"
snapshot_id: 40270
source_document_id: 7073
payload_hash: "20f507cef1b4f4bda3e56479a1bd878f82d3d39b0183cc562162623dc9f2f474"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.299686+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Armor Piercing Rounds

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aprounds`
- Snapshot ID: `40270`
- Source-Dokument: `7073`
- Kurzinfo: Armor Piercing Rounds aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 6400,
  "Description": "Your Bullets have a chance to become unavoidable, <span class=\"highlight\">piercing through</span> enemies and <span class=\"highlight\">ignoring their Bullet Resistance</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusBulletSpeedPercent",
        "Value": 60
      },
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
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
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_aprounds_desc",
    "Main": [
      {
        "Key": "ProcChance",
        "Value": 55
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_aprounds",
  "Name": "Armor Piercing Rounds",
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
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BaseAttackDamagePercent": 30,
    "BonusBulletSpeedPercent": 55,
    "ProcChance": 20
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
      "lookup": "armor piercing rounds",
      "name": "Armor Piercing Rounds",
      "type": "item"
    }
  ]
}
````
