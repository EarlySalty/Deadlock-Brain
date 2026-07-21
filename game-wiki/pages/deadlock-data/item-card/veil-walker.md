---
title: "Veil Walker"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_veil_walker"
canonical_name: "Veil Walker"
snapshot_id: 40495
source_document_id: 7073
payload_hash: "a38a761c3c13b45a91ecda1750c9a461db5028a72b2ecdc1a84c58ad59896069"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.711758+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Veil Walker

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_veil_walker`
- Snapshot ID: `40495`
- Source-Dokument: `7073`
- Kurzinfo: Veil Walker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 3200,
  "Description": "Walking through a <span class=\"highlight\">cosmic veil</span> grants you <span class=\"highlight\">Stealth</span>, <span class=\"highlight\">Heal</span> and increased <span class=\"highlight\">Move Speed</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "2.0m"
      },
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 2
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
        "Value": 125
      },
      {
        "Key": "SpiritPower",
        "Type": "tech_damage",
        "Value": 10
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
        "Key": "InvisDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": 15.0,
    "DescKey": "#upgrade_veil_walker_desc",
    "Main": [
      {
        "Key": "StatusEffectInvisible",
        "Value": null
      },
      {
        "Key": "BonusMoveSpeed",
        "LocTokenOverride": "#VeilWalker_MoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "3.5m"
      },
      {
        "Key": "HealOnVeil",
        "LocTokenOverride": "#VeilWalker_Heal",
        "Scale": {
          "Type": "power_increase",
          "Value": 8
        },
        "Type": "healing",
        "Value": 85
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_veil_walker",
  "Name": "Veil Walker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 16
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
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 0.25
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 0.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 1.25
    },
    "SpottedRadius": {
      "Key": "SpottedRadius",
      "Value": "20m"
    }
  },
  "ShopFilters": [
    "Durability",
    "ClipSize"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -9,
    "BonusMoveSpeed": "4m",
    "BonusSprintSpeed": "12m",
    "HealOnVeil": 300,
    "InvisDuration": 4,
    "InvisMoveSpeedMod": "6m",
    "OutOfCombatHealthRegen": 8,
    "SpiritPower": 25
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
      "lookup": "veil walker",
      "name": "Veil Walker",
      "type": "item"
    }
  ]
}
````
