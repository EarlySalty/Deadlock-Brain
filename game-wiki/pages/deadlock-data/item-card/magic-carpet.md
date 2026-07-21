---
title: "Magic Carpet"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_magic_carpet"
canonical_name: "Magic Carpet"
snapshot_id: 40393
source_document_id: 7073
payload_hash: "1f010ba8b42fb08f6a0add7459e02701a52541db292606942719300df0035c1b"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.515680+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Magic Carpet

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_carpet`
- Snapshot ID: `40393`
- Source-Dokument: `7073`
- Kurzinfo: Magic Carpet aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 6400,
  "Description": "Summon a Magic Carpet that will <span class=\"highlight\">fly</span> you away. While flying you are immune to slows and doing any action will dismiss the carpet. <span class=\"diminish\"><br>Cannot use abilities while the carpet is being summoned.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusAbilityDurationPercent",
        "Type": "duration",
        "Value": 15
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      },
      {
        "Key": "TechPower",
        "Value": 14
      },
      {
        "Key": "GravityScale",
        "Value": -15
      },
      {
        "Key": "AirControlPercent",
        "Type": "move_speed",
        "Value": 25
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
        "Key": "FlyMoveSpeed",
        "LocTokenOverride": "MagicCarpetMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "7m"
      },
      {
        "Key": "SummonDuration",
        "Value": 1.3
      }
    ],
    "ChargeUp": null,
    "Cooldown": 32.0,
    "DescKey": "#upgrade_magic_carpet_desc",
    "Main": [
      {
        "Key": "AbilityDuration",
        "Type": "duration",
        "Value": 12
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_carpet",
  "Name": "Magic Carpet",
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
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "AbilityDuration": 8,
    "BonusAbilityDurationPercent": 15,
    "FlyMoveSpeed": "6m",
    "SummonDuration": -0.3,
    "TechPower": 46
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
      "lookup": "magic carpet",
      "name": "Magic Carpet",
      "type": "item"
    }
  ]
}
````
