---
title: "Disarming Hex"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_greater_withering_whip"
canonical_name: "Disarming Hex"
snapshot_id: 40355
source_document_id: 7073
payload_hash: "19d0b7c9fa92dcc9d2143e03e826f2845e3242e30cd42070f7ba68edf78c8ce3"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.450737+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Disarming Hex

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_greater_withering_whip`
- Snapshot ID: `40355`
- Source-Dokument: `7073`
- Kurzinfo: Disarming Hex aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": [
    "upgrade_withering_whip"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Disarms</span> enemy target and reduces their <span class=\"highlight\">Bullet Resist</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "0.75m"
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
        "Value": "32m"
      },
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 4.25
      }
    ],
    "ChargeUp": null,
    "Cooldown": 16.0,
    "DescKey": "#upgrade_greater_withering_whip_desc",
    "Main": [
      {
        "Key": "StatusEffectDisarmed",
        "Value": null
      },
      {
        "Key": "BulletArmorReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -13
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_greater_withering_whip",
  "Name": "Disarming Hex",
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
    }
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 175,
    "BulletArmorReduction": -7
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
      "lookup": "disarming hex",
      "name": "Disarming Hex",
      "type": "item"
    }
  ]
}
````
