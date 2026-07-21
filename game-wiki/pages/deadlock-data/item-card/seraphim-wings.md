---
title: "Seraphim Wings"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_icarus_wings"
canonical_name: "Seraphim Wings"
snapshot_id: 40376
source_document_id: 7073
payload_hash: "4783927190797b29541985751a3c35f5c9947f8eea06cbc1f782cda2619cbb75"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.487326+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Seraphim Wings

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_icarus_wings`
- Snapshot ID: `40376`
- Source-Dokument: `7073`
- Kurzinfo: Seraphim Wings aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "When Airborne, You deal more damage and take reduced damage. Allows <span class=\"highlight\">unlimited</span> Air Dash and Jumping.",
  "Info1": {
    "Alt": [
      {
        "Key": "StaminaCooldownReduction",
        "Value": 120
      },
      {
        "Key": "GravityScale",
        "Value": -70
      },
      {
        "Key": "AirControlPercent",
        "Type": "move_speed",
        "Value": 100
      },
      {
        "Key": "AirControlAccelPercent",
        "Type": "move_speed",
        "Value": 50
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
    "DescKey": "#upgrade_icarus_wings_desc",
    "Main": [
      {
        "Key": "AirBonusDamageGiven",
        "Value": 40
      },
      {
        "Key": "AirBonusDamageTaken",
        "Value": -40
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_icarus_wings",
  "Name": "Seraphim Wings",
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
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
  "Upgrades": {
    "AirBonusDamageGiven": 10,
    "AirBonusDamageTaken": -10,
    "StaminaCooldownReduction": 30
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
      "lookup": "seraphim wings",
      "name": "Seraphim Wings",
      "type": "item"
    }
  ]
}
````
