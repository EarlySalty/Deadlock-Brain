---
title: "upgrade_aoe_silence"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_aoe_silence"
canonical_name: "upgrade_aoe_silence"
snapshot_id: 40266
source_document_id: 7073
payload_hash: "0fff16e08b30bb4a7f7dd3db062b0411f13649dd7143b702ef46142b053b2d2f"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.293768+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_aoe_silence

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_silence`
- Snapshot ID: `40266`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_aoe_silence aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 150
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 3
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
        "Key": "Radius",
        "Type": "distance",
        "Value": "8m"
      },
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": 32.0,
    "DescKey": "#upgrade_aoe_silence_desc",
    "Main": [
      {
        "Key": "StatusEffectEMP",
        "Value": null
      },
      {
        "Key": "TechResist",
        "Type": "tech_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_aoe_silence",
  "Name": null,
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.2
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 0.25
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
    "StartRadius": {
      "Key": "StartRadius",
      "Type": "distance",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
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
