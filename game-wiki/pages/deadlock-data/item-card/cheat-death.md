---
title: "Cheat Death"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_cheat_death"
canonical_name: "Cheat Death"
snapshot_id: 40302
source_document_id: 7073
payload_hash: "0a1d70b7f79e6e9c72800587b2751d2db656de2d998d068afc97123f431606c2"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.353554+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Cheat Death

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cheat_death`
- Snapshot ID: `40302`
- Source-Dokument: `7073`
- Kurzinfo: Cheat Death aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

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
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 200
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 15
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
        "Key": "DeathImmunityDamageReduction",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -60
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "UsageFlags": "ConditionallyApplied",
        "Value": -60
      }
    ],
    "ChargeUp": null,
    "Cooldown": 90.0,
    "DescKey": "#upgrade_cheat_death_unkillable_passive",
    "Main": [
      {
        "Key": "DeathImmunityDuration",
        "Type": "duration",
        "Value": 4.5
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cheat_death",
  "Name": "Cheat Death",
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
    "BonusMoveSpeed": {
      "Key": "BonusMoveSpeed",
      "Type": "move_speed",
      "UsageFlags": "ConditionallyApplied",
      "Value": "0m"
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "UsageFlags": "ConditionallyApplied",
      "Value": -60
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -20,
    "DeathImmunityDamageReduction": 90,
    "DeathImmunityDuration": 0.5,
    "HealAmpReceivePenaltyPercent": 90,
    "HealAmpRegenPenaltyPercent": 90
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
      "lookup": "cheat death",
      "name": "Cheat Death",
      "type": "item"
    }
  ]
}
````
