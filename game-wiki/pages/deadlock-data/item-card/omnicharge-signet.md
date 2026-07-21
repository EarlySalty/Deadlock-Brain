---
title: "Omnicharge Signet"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_omnicharge_pendant"
canonical_name: "Omnicharge Signet"
snapshot_id: 40415
source_document_id: 7073
payload_hash: "9572460ee55bab927f7cc08fffdbd66b2394831a6006f9c8046176e7de836207"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.553409+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Omnicharge Signet

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_omnicharge_pendant`
- Snapshot ID: `40415`
- Source-Dokument: `7073`
- Kurzinfo: Omnicharge Signet aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Imbue any active non-ultimate ability with <span class=\"highlight\">Bonus Ability Charges</span>. Already charged abilities receive more bonus charges.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSpiritForChargedAbilities",
        "Value": 50
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "CooldownBetweenChargeReduction",
        "Type": "cooldown",
        "Value": 70
      },
      {
        "Key": "CooldownReductionOnChargedAbilities",
        "Value": 30
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_omnicharge_pendant_desc",
    "Main": [
      {
        "Key": "BonusAbilityCharges",
        "LocTokenOverride": "OmniPendantBonusAbilityCharges",
        "Type": "cast",
        "Value": 4
      },
      {
        "Key": "BonusAbilityChargesNonCharge",
        "Type": "cast",
        "Value": 2
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_omnicharge_pendant",
  "Name": "Omnicharge Signet",
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
    "EnableAbilityCharges": {
      "Key": "EnableAbilityCharges",
      "Value": 1
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "BonusAbilityCharges": 2,
    "BonusSpiritForChargedAbilities": 30,
    "CooldownBetweenChargeReduction": 5,
    "CooldownReductionOnChargedAbilities": 10
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
      "lookup": "omnicharge signet",
      "name": "Omnicharge Signet",
      "type": "item"
    }
  ]
}
````
