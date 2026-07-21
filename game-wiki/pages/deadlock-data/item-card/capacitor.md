---
title: "Capacitor"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_capacitor"
canonical_name: "Capacitor"
snapshot_id: 40296
source_document_id: 7073
payload_hash: "31d987a0e117b37641ba8e7d252828a4b2b633d235fc758d02a1b5fa4e91b53d"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.343452+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Capacitor

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_capacitor`
- Snapshot ID: `40296`
- Source-Dokument: `7073`
- Kurzinfo: Capacitor aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": [
    "upgrade_chain_lightning"
  ],
  "Cost": 6400,
  "Description": "Launch a projectile that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span>, applies a strong slow that recovers over time, <span class=\"highlight\">prevents Stamina usage</span> and <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 5
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "ChainCount",
        "Value": 6
      },
      {
        "Key": "ChainRadius",
        "Type": "distance",
        "Value": "10m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 0.2,
    "DescKey": "#upgrade_chain_lightning_desc",
    "Main": [
      {
        "Key": "DamagePerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.19
        },
        "Type": "tech_damage",
        "Value": 43.0
      },
      {
        "Key": "ProcChance",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "SlowDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 40,
    "DescKey": "#upgrade_capacitor_desc",
    "Main": [
      {
        "Key": "Damage",
        "Type": "tech_damage",
        "Value": 100
      },
      {
        "Key": "MaxSlowPercent",
        "Type": "slow",
        "Value": 75
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_capacitor",
  "Name": "Capacitor",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
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
    "BonusPerChain": {
      "Key": "BonusPerChain",
      "Scale": {
        "Type": "spirit",
        "Value": 0.19
      },
      "Type": "tech_damage",
      "Value": 43.0
    },
    "ChainTickRate": {
      "Key": "ChainTickRate",
      "Value": 0.4
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "BossEnemy",
    "MinionEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -32,
    "BonusFireRate": 15,
    "Damage": 25,
    "DamagePerChain": 25,
    "ProcChance": 5
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
      "lookup": "capacitor",
      "name": "Capacitor",
      "type": "item"
    }
  ]
}
````
