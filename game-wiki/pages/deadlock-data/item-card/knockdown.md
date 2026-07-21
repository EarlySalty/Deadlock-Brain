---
title: "Knockdown"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_target_stun"
canonical_name: "Knockdown"
snapshot_id: 40475
source_document_id: 7073
payload_hash: "95c0053c6b3ef810eb3a1458cbe9d8a6fdf52d9b376d805eadedfa61b4b21eb6"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.674645+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Knockdown

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_target_stun`
- Snapshot ID: `40475`
- Source-Dokument: `7073`
- Kurzinfo: Knockdown aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": "Apply a <span class=\"highlight\">Stun</span> after <span class=\"highlight\">2s</span>. Stun duration is increased against <span class=\"highlight\">airborne</span> targets.<br><br><span class=\"diminish\">Increases the target's gravity for the duration of the stun.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 75
      },
      {
        "Key": "TechRangeMultiplier",
        "Type": "distance",
        "Value": 5
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
        "Key": "AbilityCastRange",
        "Type": "range",
        "Value": "45m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 35.0,
    "DescKey": "#upgrade_target_stun_desc",
    "Main": [
      {
        "Key": "StatusEffectStun",
        "Value": null
      },
      {
        "Key": "StunDuration",
        "Type": "duration",
        "Value": 0.5
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_target_stun",
  "Name": "Knockdown",
  "Other": {
    "AbilityCastDelay": {
      "Key": "AbilityCastDelay",
      "Type": "cast",
      "Value": 0.1
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
    "MaxBonusDuration": {
      "Key": "MaxBonusDuration",
      "Value": 1.5
    },
    "MaxHeightForBonus": {
      "Key": "MaxHeightForBonus",
      "Value": "30m"
    },
    "StunDelay": {
      "Key": "StunDelay",
      "Type": "duration",
      "Value": 2
    },
    "TechRadiusMultiplier": {
      "Key": "TechRadiusMultiplier",
      "Type": "distance",
      "Value": 5
    },
    "VisualContractRadius": {
      "Key": "VisualContractRadius",
      "Value": "3m"
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "Upgrades": {
    "BonusHealth": 75,
    "StunDuration": 0.75,
    "TechRadiusMultiplier": 6,
    "TechRangeMultiplier": 6
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
      "lookup": "knockdown",
      "name": "Knockdown",
      "type": "item"
    }
  ]
}
````
