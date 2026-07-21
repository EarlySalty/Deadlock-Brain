---
title: "Silence Wave"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_targeted_silence"
canonical_name: "Silence Wave"
snapshot_id: 40476
source_document_id: 7073
payload_hash: "8a46d7689a3ffc075a7d58267a3a07a9ece88df36a6554a594a87a6367e1b1ff"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.676688+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Silence Wave

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_targeted_silence`
- Snapshot ID: `40476`
- Source-Dokument: `7073`
- Kurzinfo: Silence Wave aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Launch an expanding projectile which <span class=\"highlight\">Silences</span> enemies for a short duration and deals impact damage. <br><br><span class=\"diminish\">Silence does not interrupt channeling abilities.</span>",
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
    "Alt": [
      {
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "40m"
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "#SilenceDuration",
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 42.0,
    "DescKey": "#upgrade_targeted_silence_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.7
        },
        "Type": "tech_damage",
        "Value": 75.0
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_targeted_silence",
  "Name": "Silence Wave",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
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
    "CooldownOnMiss": {
      "Key": "CooldownOnMiss",
      "Value": 30.0
    },
    "GrowthPerMeter": {
      "Key": "GrowthPerMeter",
      "Value": "0.15m"
    },
    "HeightOffGround": {
      "Key": "HeightOffGround",
      "Value": "1m"
    },
    "InitialWidth": {
      "Key": "InitialWidth",
      "Value": "5.0m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 75,
    "Damage": 125
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
      "lookup": "silence wave",
      "name": "Silence Wave",
      "type": "item"
    }
  ]
}
````
