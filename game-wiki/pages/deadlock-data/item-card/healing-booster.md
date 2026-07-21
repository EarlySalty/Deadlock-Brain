---
title: "Healing Booster"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_healing_booster"
canonical_name: "Healing Booster"
snapshot_id: 40365
source_document_id: 7073
payload_hash: "86c24961e526ab823eeab883c076c98562248ea420098cdd6579eb2bcb45bbf8"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.468997+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Healing Booster

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_healing_booster`
- Snapshot ID: `40365`
- Source-Dokument: `7073`
- Kurzinfo: Healing Booster aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_endurance"
  ],
  "Cost": 1600,
  "Description": "Increases the effectiveness of your <span class=\"highlight\">healing</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealthRegen",
        "Type": "healing",
        "Value": 3
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 1
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
    "Cooldown": null,
    "DescKey": "#upgrade_healing_booster_desc",
    "Main": [
      {
        "Key": "HealAmpCastPercent",
        "Value": 20
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healing_booster",
  "Name": "Healing Booster",
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
    "HealAmpRegenPercent": {
      "Key": "HealAmpRegenPercent",
      "Value": 20
    }
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusHealthRegen": 9,
    "HealAmpCastPercent": 15,
    "HealAmpRegenPercent": 15
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
      "lookup": "healing booster",
      "name": "Healing Booster",
      "type": "item"
    }
  ]
}
````
