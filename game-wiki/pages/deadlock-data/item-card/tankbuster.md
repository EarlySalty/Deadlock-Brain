---
title: "Tankbuster"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_magic_shock"
canonical_name: "Tankbuster"
snapshot_id: 40398
source_document_id: 7073
payload_hash: "5bb7eda0a1db80a700ac4811d99f5ff51b88871f0c197f9c63c9305b3e275cbb"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.523743+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Tankbuster

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_shock`
- Snapshot ID: `40398`
- Source-Dokument: `7073`
- Kurzinfo: Tankbuster aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_magic_burst"
  ],
  "Cost": 3200,
  "Description": "Charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, causing abilities dealing more than <span class=\"highlight\">165</span> damage to deal additional damage. <span class=\"highlight\">Ignores Spirit Resistance.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 50
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
    "ChargeUp": 14,
    "Cooldown": null,
    "DescKey": "#upgrade_magic_shock_desc",
    "Main": [
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 40
      },
      {
        "Key": "CurrentHealthDamage",
        "LocTokenOverride": "MagicShockDamage",
        "Type": "tech_damage",
        "Value": 8
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_shock",
  "Name": "Tankbuster",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 14
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
    "MinimumDamage": {
      "Key": "MinimumDamage",
      "Value": 165
    },
    "ReProcLockoutTime": {
      "Key": "ReProcLockoutTime",
      "Value": 5
    },
    "WatcherMaxDuration": {
      "Key": "WatcherMaxDuration",
      "Value": 30
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 100,
    "CurrentHealthDamage": 5,
    "Damage": 60
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
      "lookup": "tankbuster",
      "name": "Tankbuster",
      "type": "item"
    }
  ]
}
````
