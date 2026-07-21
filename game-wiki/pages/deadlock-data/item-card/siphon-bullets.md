---
title: "Siphon Bullets"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_siphon_bullets"
canonical_name: "Siphon Bullets"
snapshot_id: 40453
source_document_id: 7073
payload_hash: "0b9d380c25e44b12b10f1e53e59f67b22d454cb7b3e6afd7dfafeed0985380d5"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.618292+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Siphon Bullets

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_siphon_bullets`
- Snapshot ID: `40453`
- Source-Dokument: `7073`
- Kurzinfo: Siphon Bullets aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 15
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 10
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
        "Key": "StealDuration",
        "Type": "duration",
        "Value": 17
      }
    ],
    "ChargeUp": null,
    "Cooldown": 1.2,
    "DescKey": "#upgrade_siphon_bullets_desc_passive2",
    "Main": [
      {
        "Key": "HealthStealPctHero",
        "LocTokenOverride": "SiphonBullets_HealthStealPctHero",
        "Type": "health",
        "Value": 2.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_siphon_bullets",
  "Name": "Siphon Bullets",
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
    "MaxStacks": {
      "Key": "MaxStacks",
      "Value": 9999
    },
    "ParticleRadius": {
      "Key": "ParticleRadius",
      "Type": "distance",
      "Value": "1m"
    },
    "StackLostPerDeath": {
      "Key": "StackLostPerDeath",
      "Value": 2
    },
    "StealPerHit": {
      "Key": "StealPerHit",
      "Value": 1
    },
    "StealPerKill": {
      "Key": "StealPerKill",
      "Value": 1
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BulletResist": 10,
    "HealthStealPctHero": 1.5
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
      "lookup": "siphon bullets",
      "name": "Siphon Bullets",
      "type": "item"
    }
  ]
}
````
