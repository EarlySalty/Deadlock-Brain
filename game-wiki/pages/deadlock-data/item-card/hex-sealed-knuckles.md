---
title: "Hex-Sealed Knuckles"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_superacolytes_glove"
canonical_name: "Hex-Sealed Knuckles"
snapshot_id: 40471
source_document_id: 7073
payload_hash: "d7de6c069472b22bc0c7186245a42f64d8e8d9572f01aaf176b9aa3b1d65f426"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.666107+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Hex-Sealed Knuckles

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_superacolytes_glove`
- Snapshot ID: `40471`
- Source-Dokument: `7073`
- Kurzinfo: Hex-Sealed Knuckles aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 3200,
  "Description": "After dealing an accumulated <span class=\"highlight\">{s:StoredSpiritDamage}</span> spirit damage, your next <span class=\"highlight\">Heavy Melee Attack</span> deals an additional <span class=\"highlight\">{s:StoredSpiritDamage} spirit damage</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 1
      },
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
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
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 3,
    "DescKey": "#upgrade_superacolytes_glove_desc",
    "Main": [
      {
        "Key": "StoredSpiritDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.93
        },
        "Type": "tech_damage",
        "Value": 200.0
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_superacolytes_glove",
  "Name": "Hex-Sealed Knuckles",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 13
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
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
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
