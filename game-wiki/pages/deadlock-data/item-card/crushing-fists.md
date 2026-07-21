---
title: "Crushing Fists"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_crushing_fists"
canonical_name: "Crushing Fists"
snapshot_id: 40322
source_document_id: 7073
payload_hash: "a84320f44975b69dba26c04d82535ce946b8705ad451409a5a17d5815d30f830"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.390616+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Crushing Fists

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_crushing_fists`
- Snapshot ID: `40322`
- Source-Dokument: `7073`
- Kurzinfo: Crushing Fists aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_melee_charge"
  ],
  "Cost": 6400,
  "Description": "Your <span class=\"highlight\">{g:citadel_inline_attribute:'MeleeDamage'}</span> will <span class=\"highlight\">restore ammo</span> and apply a <span class=\"highlight\">stacking bullet resist debuff</span> on enemies. Heavy melee applies 2 stacks. <br><br>If the target reaches max stacks, they will be <span class=\"highlight\">stunned</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusMeleeDamagePercent",
        "Type": "melee_damage",
        "Value": 22
      },
      {
        "Key": "BulletResist",
        "Type": "bullet_armor_up",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": null,
    "Main": [
      {
        "Key": "MeleeDistanceScale",
        "Value": 60
      }
    ],
    "Type": "Innate"
  },
  "Info2": {
    "Alt": [],
    "ChargeUp": null,
    "Cooldown": 5,
    "DescKey": "#upgrade_melee_charge_desc",
    "Main": [
      {
        "Key": "BonusHeavyMeleeDamage",
        "Type": "melee_damage",
        "Value": 25
      }
    ],
    "Type": "Passive"
  },
  "Info3": {
    "Alt": [
      {
        "Key": "StunDuration",
        "Value": 0.5
      },
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 8
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_crushing_fists_desc",
    "Main": [
      {
        "Key": "LightMeleeAmmo",
        "Type": "clipsize",
        "Value": 15
      },
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -4
      },
      {
        "Key": "MaxStacks",
        "Value": 6
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crushing_fists",
  "Name": "Crushing Fists",
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
    "HeavyMeleeMultiplier": {
      "Key": "HeavyMeleeMultiplier",
      "Value": 2
    },
    "LightMeleeStacks": {
      "Key": "LightMeleeStacks",
      "Value": 1
    }
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BonusMeleeDamagePercent": 15,
    "BulletResist": 12,
    "BulletResistReduction": -4,
    "MeleeDistanceScale": 40
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
      "lookup": "crushing fists",
      "name": "Crushing Fists",
      "type": "item"
    }
  ]
}
````
