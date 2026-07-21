---
title: "Haunting Scream"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_haunting_scream"
canonical_name: "Haunting Scream"
snapshot_id: 40358
source_document_id: 7073
payload_hash: "cc28b8779a0140f35f35d6d9f0c8df6c14e8cb6cda0fc10ceb94bdf5c0b692c2"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.456081+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Haunting Scream

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_haunting_scream`
- Snapshot ID: `40358`
- Source-Dokument: `7073`
- Kurzinfo: Haunting Scream aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 9999,
  "Description": "Launch an expanding projectile which <span class=\"highlight\">Curses</span> enemies for a short duration.",
  "Info1": {
    "Alt": [
      {
        "Key": "TechPowerPercent",
        "Value": 30
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
        "Type": "duration",
        "Value": 3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 25.0,
    "DescKey": "#upgrade_haunting_scream_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_haunting_scream",
  "Name": "Haunting Scream",
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
    "Damage": {
      "Key": "Damage",
      "Value": 400
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
    },
    "SkipFrames": {
      "Key": "SkipFrames",
      "UsageFlags": "ConditionallyApplied",
      "Value": 6
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -8,
    "AbilityDuration": 0.25,
    "TechPowerPercent": 10
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
  }
}
````
