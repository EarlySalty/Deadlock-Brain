---
title: "upgrade_charge_mastery"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_charge_mastery"
canonical_name: "upgrade_charge_mastery"
snapshot_id: 40300
source_document_id: 7073
payload_hash: "74acf5a9a05aab27246c996890cd5089fc51969870ada3caa3159bc7bb1a26af"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.351088+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_charge_mastery

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_charge_mastery`
- Snapshot ID: `40300`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_charge_mastery aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_charge_mastery_passive",
    "Main": [],
    "Type": "Passive"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "BonusAbilityCharges",
        "Type": "cast",
        "Value": 4
      },
      {
        "Key": "CooldownBetweenChargeReduction",
        "Type": "cooldown",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [],
    "Type": "Innate"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_charge_mastery",
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
    "BonusChargedAbilityDamage": {
      "Key": "BonusChargedAbilityDamage",
      "Value": 15
    },
    "BonusChargedCooldownReduction": {
      "Key": "BonusChargedCooldownReduction",
      "Type": "cooldown",
      "Value": 15
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
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
