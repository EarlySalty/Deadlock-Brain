---
title: "Phantom Strike"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_phantom_strike"
canonical_name: "Phantom Strike"
snapshot_id: 40418
source_document_id: 7073
payload_hash: "f1e312cf75061c51886131af1ca950551a09e138928f9bb64eb89d40d9e2311e"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.558616+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Phantom Strike

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_phantom_strike`
- Snapshot ID: `40418`
- Source-Dokument: `7073`
- Kurzinfo: Phantom Strike aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Teleport</span> to an enemy target and pull them to the ground. Dealing <span class=\"highlight\">damage</span>, <span class=\"highlight\">Move speed</span> reduction and <span class=\"highlight\">Disarm</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BaseAttackDamagePercent",
        "Type": "bullet_damage",
        "Value": 15
      },
      {
        "Key": "TechPower",
        "Type": "tech_damage",
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
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "25m"
      },
      {
        "Key": "SlowDuration",
        "LocTokenOverride": "PhantomStrikeDebuffDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_phantom_strike_desc",
    "Main": [
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      },
      {
        "Key": "SlowPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 50
      },
      {
        "Key": "ImpactDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.93
        },
        "Type": "tech_damage",
        "Value": 75.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_phantom_strike",
  "Name": "Phantom Strike",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.35
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
      "Value": "1.3m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Movement",
    "Disruption"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -20,
    "BaseAttackDamagePercent": 20,
    "ImpactDamage": 100,
    "TechPower": 12
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
      "lookup": "phantom strike",
      "name": "Phantom Strike",
      "type": "item"
    }
  ]
}
````
