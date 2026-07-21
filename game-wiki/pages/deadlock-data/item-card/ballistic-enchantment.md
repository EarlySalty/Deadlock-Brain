---
title: "Ballistic Enchantment"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_bulletshredimbue"
canonical_name: "Ballistic Enchantment"
snapshot_id: 40292
source_document_id: 7073
payload_hash: "3f6c9de048b8302d389efcf4cd41c612cf66142ccfa5b02f018e2b4e54002452"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.337826+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Ballistic Enchantment

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_bulletshredimbue`
- Snapshot ID: `40292`
- Source-Dokument: `7073`
- Kurzinfo: Ballistic Enchantment aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_reach"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with increased <span class=\"highlight\">range</span>. Dealing damage with that ability grants you increased <span class=\"highlight\">weapon damage</span> per <span class=\"highlight\">unique hero hit</span>. Has reduced effect on non-heroes.",
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 14
      },
      {
        "Key": "WeaponPowerPerStackNonHero",
        "Value": 5
      },
      {
        "Key": "NonHeroStackLimit",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_bulletshredimbue_desc",
    "Main": [
      {
        "Key": "WeaponPowerPerStack",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 22
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_bulletshredimbue",
  "Name": "Ballistic Enchantment",
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
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 22
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15,
    "WeaponPowerPerStack": 15
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
      "lookup": "ballistic enchantment",
      "name": "Ballistic Enchantment",
      "type": "item"
    }
  ]
}
````
