---
title: "upgrade_galvanic_storm"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_galvanic_storm"
canonical_name: "upgrade_galvanic_storm"
snapshot_id: 40350
source_document_id: 7073
payload_hash: "e09e894ef62e2fdfe001e7d47acbe0ca30cf3ee487c303887dc8d2f4bf7dd02b"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.442837+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_galvanic_storm

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_galvanic_storm`
- Snapshot ID: `40350`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_galvanic_storm aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

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
        "Key": "BonusPerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.457002
        },
        "Type": "tech_damage",
        "Value": 75.0
      },
      {
        "Key": "ProcChance",
        "Value": 30
      },
      {
        "Key": "ChainRadius",
        "Type": "distance",
        "Value": "7m"
      },
      {
        "Key": "ChainCount",
        "Value": 7
      },
      {
        "Key": "ChainTickRate",
        "Value": 0.2
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_galvanic_storm_passive1",
    "Main": [
      {
        "Key": "DamagePerChain",
        "Scale": {
          "Type": "spirit",
          "Value": 0.304668
        },
        "Type": "tech_damage",
        "Value": 50.0
      }
    ],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BuffMoveSpeedBonus",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3m"
      },
      {
        "Key": "GalvanicBuffDuration",
        "Type": "duration",
        "Value": 7
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_galvanic_storm_passive2",
    "Main": [
      {
        "Key": "BuffDamageMult",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_regenerating_tech_shield",
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_galvanic_storm",
  "Name": null,
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
    "ExplodeRadius": {
      "Key": "ExplodeRadius",
      "Type": "distance",
      "Value": "10m"
    },
    "GalvanicDebuffDuration": {
      "Key": "GalvanicDebuffDuration",
      "Type": "duration",
      "Value": 5
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 0.4
    },
    "TechShieldMaxHealth": {
      "Key": "TechShieldMaxHealth",
      "Type": "tech_armor_up",
      "Value": 400
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
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
