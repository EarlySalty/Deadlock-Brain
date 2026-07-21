---
title: "upgrade_stasis_bomb"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_stasis_bomb"
canonical_name: "upgrade_stasis_bomb"
snapshot_id: 40470
source_document_id: 7073
payload_hash: "2e1c9051b724b3232f15b61944386b2f635ef90feef9115744c907c5e4292006"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.664605+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_stasis_bomb

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_stasis_bomb`
- Snapshot ID: `40470`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_stasis_bomb aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

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
        "Key": "StasisRadius",
        "Type": "distance",
        "Value": "7m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 37.0,
    "DescKey": "#upgrade_stasis_bomb_active1",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "Info2": {
    "Alt": [
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 1.5
      },
      {
        "Key": "ExplodeDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 1.52334
        },
        "Type": "tech_damage",
        "Value": 250.0
      },
      {
        "Key": "StasisRadius",
        "Type": "distance",
        "Value": "7m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_stasis_bomb_active2",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      }
    ],
    "Type": "Active"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "EMPDuration",
        "Value": 3
      },
      {
        "Key": "BuildUpPerShot",
        "Value": 1.6
      },
      {
        "Key": "BuildUpDuration",
        "Type": "duration",
        "Value": 10
      },
      {
        "Key": "ImmunityDuration",
        "Value": 4
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_stasis_bomb_desc_passive",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_stasis_bomb",
  "Name": null,
  "Other": {
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "20m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 6
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
    "DampingFactor": {
      "Key": "DampingFactor",
      "Value": 0.5
    },
    "EMPProcChance": {
      "Key": "EMPProcChance",
      "Value": 100
    },
    "ExplodeRadius": {
      "Key": "ExplodeRadius",
      "Type": "distance",
      "Value": "7m"
    },
    "LiftHeight": {
      "Key": "LiftHeight",
      "Value": 120
    },
    "MoveSpeedMax": {
      "Key": "MoveSpeedMax",
      "Type": "slow",
      "Value": "4m"
    },
    "SlamdownSpeed": {
      "Key": "SlamdownSpeed",
      "Value": 500
    }
  },
  "ShopFilters": null,
  "Slot": "Armor",
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
