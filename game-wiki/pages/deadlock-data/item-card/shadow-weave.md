---
title: "Shadow Weave"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_cloaking_device_active"
canonical_name: "Shadow Weave"
snapshot_id: 40311
source_document_id: 7073
payload_hash: "736169e4977c9832635c8437b0476dbd976be44f72c38430c9a7019ecdf6c852"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.367113+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Shadow Weave

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloaking_device_active`
- Snapshot ID: `40311`
- Source-Dokument: `7073`
- Kurzinfo: Shadow Weave aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 3200,
  "Description": "Become <span class=\"highlight\">Stealthed</span>. Whenever you take damage while Stealthed you get briefly revealed.",
  "Info1": {
    "Alt": [
      {
        "Key": "OutOfCombatHealthRegen",
        "Type": "healing",
        "Value": 5
      },
      {
        "Key": "BonusSprintSpeed",
        "Type": "move_speed",
        "Value": "1.5m"
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
        "Key": "SpottedRadius",
        "Value": "20m"
      },
      {
        "Key": "InvisMoveSpeedMod",
        "Type": "move_speed",
        "Value": "5m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 45.0,
    "DescKey": "#upgrade_cloaking_device_active_desc",
    "Main": [
      {
        "Key": "StatusEffectInvisible",
        "Value": null
      },
      {
        "Key": "AbilityDuration",
        "LocTokenOverride": "StealthDuration",
        "Type": "duration",
        "Value": 13
      }
    ],
    "Type": "Active"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "AmbushDuration",
        "Type": "duration",
        "Value": 5
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_cloaking_device_active_ambush_desc",
    "Main": [
      {
        "Key": "AmbushBonusFireRate",
        "Type": "bullet_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "AmbushBonusTechPower",
        "Type": "tech_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      },
      {
        "Key": "AmbushBonusMeleeDamage",
        "Type": "melee_damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": 25
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cloaking_device_active",
  "Name": "Shadow Weave",
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
    "FullInvisDistance": {
      "Key": "FullInvisDistance",
      "Value": "30m"
    },
    "InvisAlertWhenFading": {
      "Key": "InvisAlertWhenFading",
      "Value": 1
    },
    "InvisCancelOnDamage": {
      "Key": "InvisCancelOnDamage",
      "Value": 1
    },
    "InvisFadeToDuration": {
      "Key": "InvisFadeToDuration",
      "Type": "duration",
      "Value": 0.6
    },
    "RevealOnDamageDuration": {
      "Key": "RevealOnDamageDuration",
      "Type": "duration",
      "Value": 1.5
    },
    "RevealOnSpottedDuration": {
      "Key": "RevealOnSpottedDuration",
      "Type": "duration",
      "Value": 1.5
    }
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "Upgrades": {
    "AbilityCooldown": -17,
    "AmbushBonusFireRate": 35,
    "AmbushBonusMeleeDamage": 30,
    "AmbushBonusTechPower": 35,
    "OutOfCombatHealthRegen": 20
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
      "lookup": "shadow weave",
      "name": "Shadow Weave",
      "type": "item"
    }
  ]
}
````
