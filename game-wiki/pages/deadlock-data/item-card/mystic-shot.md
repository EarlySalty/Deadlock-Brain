---
title: "Mystic Shot"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_crackshot"
canonical_name: "Mystic Shot"
snapshot_id: 40320
source_document_id: 7073
payload_hash: "b68c8b9d4a1743a6575dfb611ba76b1712c808a427c0a1b65f2544480ccceb9e"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.385969+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Mystic Shot

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_crackshot`
- Snapshot ID: `40320`
- Source-Dokument: `7073`
- Kurzinfo: Mystic Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your next bullet deals bonus {g:citadel_inline_attribute:'SpiritDamage'}.",
  "Info1": {
    "Alt": [
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 7
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
    "Cooldown": 9,
    "DescKey": "#upgrade_crackshot_desc",
    "Main": [
      {
        "Key": "ProcBonusMagicDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 1.2
        },
        "Type": "tech_damage",
        "Value": 40.0
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crackshot",
  "Name": "Mystic Shot",
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
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 1
    },
    "Radius": {
      "Key": "Radius",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
  "Upgrades": {
    "ProcBonusMagicDamage": 109,
    "SpiritPower": 14
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
      "lookup": "mystic shot",
      "name": "Mystic Shot",
      "type": "item"
    }
  ]
}
````
