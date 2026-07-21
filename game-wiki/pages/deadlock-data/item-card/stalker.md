---
title: "Stalker"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_weapon_backstabber"
canonical_name: "Stalker"
snapshot_id: 40498
source_document_id: 7073
payload_hash: "ef793a64cc9ed61a923a6607a68ad8e3fc54ae1b987e7db4c692483942dc9608"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.717882+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Stalker

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_backstabber`
- Snapshot ID: `40498`
- Source-Dokument: `7073`
- Kurzinfo: Stalker aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Dealing {g:citadel_inline_attribute:'WeaponDamage'} at close range opens a wound and grants you {g:citadel_inline_attribute:'BonusMoveSpeed'}. <br><br>Wounded enemies take {g:citadel_inline_attribute:'SpiritDPS'}, have reduced {g:citadel_inline_attribute:'BulletResist'}, and are revealed <span class=\"highlight\">through walls</span>.",
  "Info1": {
    "Alt": [
      {
        "Key": "ReduceFootstepSound",
        "Value": -50
      },
      {
        "Key": "BonusHealth",
        "Type": "health",
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
    "Alt": [
      {
        "Key": "DebuffDuration",
        "Value": 5
      },
      {
        "Key": "ProcRadius",
        "LocTokenOverride": "ProcRadius",
        "Type": "distance",
        "Value": "8m"
      }
    ],
    "ChargeUp": null,
    "Cooldown": 6,
    "DescKey": "#upgrade_weapon_backstabber_desc",
    "Main": [
      {
        "Key": "DPS",
        "Type": "tech_damage",
        "Value": 17
      },
      {
        "Key": "BulletResistReduction",
        "Type": "bullet_armor_down",
        "UsageFlags": "ConditionallyApplied",
        "Value": -6
      },
      {
        "Key": "BonusMoveSpeed",
        "Type": "move_speed",
        "UsageFlags": "ConditionallyApplied",
        "Value": "1.5m"
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_backstabber",
  "Name": "Stalker",
  "Other": {
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 5
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
    "DebuffRadius": {
      "Key": "DebuffRadius",
      "LocTokenOverride": "BackstabberRadius",
      "Type": "distance",
      "Value": "25m"
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    }
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "Upgrades": {
    "BonusMoveSpeed": "2m",
    "BulletResistReduction": -10,
    "DPS": 20,
    "ReduceFootstepSound": -50
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
      "lookup": "stalker",
      "name": "Stalker",
      "type": "item"
    }
  ]
}
````
