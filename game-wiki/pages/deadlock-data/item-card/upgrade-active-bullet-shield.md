---
title: "upgrade_active_bullet_shield"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_active_bullet_shield"
canonical_name: "upgrade_active_bullet_shield"
snapshot_id: 40259
source_document_id: 7073
payload_hash: "3d680bbb0062d688e4c33bbe21f3e54066b675b8099d0267acad24888780e1ec"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.282516+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_active_bullet_shield

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_active_bullet_shield`
- Snapshot ID: `40259`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_active_bullet_shield aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 15
      }
    ],
    "ChargeUp": null,
    "Cooldown": 64.0,
    "DescKey": "#upgrade_active_bullet_shield_desc",
    "Main": [],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_generic_bullet_shield_desc",
    "Main": [],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_sprint_booster_desc",
    "Main": [],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_active_bullet_shield",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.25
    },
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "25m"
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
    },
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "30m"
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly",
    "MinionFriendly"
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
