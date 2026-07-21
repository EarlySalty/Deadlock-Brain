---
title: "Surge of Power"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_magic_storm"
canonical_name: "Surge of Power"
snapshot_id: 40400
source_document_id: 7073
payload_hash: "018b55c919e8eddc8a0333f6134875371a2ab6d2f86b8ecfbdd9bbd84f48d758"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.527099+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Surge of Power

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_storm`
- Snapshot ID: `40400`
- Source-Dokument: `7073`
- Kurzinfo: Surge of Power aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_improved_spirit"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with <span class=\"highlight\">permanent Spirit Power</span>. When that ability is used, gain bonus <span class=\"highlight\">Move Speed</span> and maintain full speed while attacking.",
  "Info1": {
    "Alt": [
      {
        "Key": "MovementSpeedBonusDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 14,
    "DescKey": "#upgrade_magic_storm_desc",
    "Main": [
      {
        "Key": "ImbuedTechPower",
        "Type": "tech_damage",
        "Value": 28
      },
      {
        "Key": "FireRateBonus",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 20
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.75m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_storm",
  "Name": "Surge of Power",
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
    "MoveWhileShootingSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileShootingSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    },
    "MoveWhileZoomedSpeedPenaltyReductionPercent": {
      "Key": "MoveWhileZoomedSpeedPenaltyReductionPercent",
      "Type": "move_speed",
      "UsageFlags": "ConditionallyApplied",
      "Value": 100
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "BonusMoveSpeed": "2m",
    "FireRateBonus": 18,
    "ImbuedTechPower": 32
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
      "lookup": "surge of power",
      "name": "Surge of Power",
      "type": "item"
    }
  ]
}
````
