---
title: "Refresher"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_ability_refresher"
canonical_name: "Refresher"
snapshot_id: 40256
source_document_id: 7073
payload_hash: "a98d4b7aa2fb195f8ce044ef7c581e1ad0beef2a9162cde5a55115ec46e530df"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.277206+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Refresher

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ability_refresher`
- Snapshot ID: `40256`
- Source-Dokument: `7073`
- Kurzinfo: Refresher aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Reset the cooldown</span> of all your abilities and <span class=\"highlight\">restore all your charges</span>.",
  "Info1": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 300,
    "DescKey": "#upgrade_ability_refresher_desc",
    "Main": [],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ability_refresher",
  "Name": "Refresher",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.6
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
    "BulletResist": {
      "Key": "BulletResist",
      "Type": "bullet_armor_up",
      "Value": 15
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "TechResist": {
      "Key": "TechResist",
      "Type": "tech_armor_up",
      "Value": 14
    }
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityCooldown": -210
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
      "lookup": "refresher",
      "name": "Refresher",
      "type": "item"
    }
  ]
}
````
