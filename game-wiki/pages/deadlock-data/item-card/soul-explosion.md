---
title: "Soul Explosion"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_corpse_explosion"
canonical_name: "Soul Explosion"
snapshot_id: 40318
source_document_id: 7073
payload_hash: "8cafc7ad44c33cddcde0eaa1a00a4c93a2f1eb1259bb34852fd06b179e654fb7"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.381924+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Soul Explosion

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_corpse_explosion`
- Snapshot ID: `40318`
- Source-Dokument: `7073`
- Kurzinfo: Soul Explosion aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "Kills or assists cause an <span class=\"highlight\">explosion</span> where the victim dies. Kills against heroes have greater radius and damage.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 110
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 3
      },
      {
        "Key": "TechPower",
        "Value": 6
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1m"
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
        "Key": "HeroMultiplier",
        "Value": 150
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_corpse_explosion_desc",
    "Main": [
      {
        "Key": "ExplosionDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.93
        },
        "Type": "tech_damage",
        "Value": 125
      },
      {
        "Key": "ExplosionRadius",
        "Value": "4m"
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_corpse_explosion",
  "Name": "Soul Explosion",
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
    "ArmingTime": {
      "Key": "ArmingTime",
      "Value": 0.1
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
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
