---
title: "Spellslinger"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_enchanted_holsters"
canonical_name: "Spellslinger"
snapshot_id: 40336
source_document_id: 7073
payload_hash: "dbaa85f13ad7c66f14120623f9b39d5cbadded59ea376dfb401138c35a7b763d"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.416732+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Spellslinger

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_enchanted_holsters`
- Snapshot ID: `40336`
- Source-Dokument: `7073`
- Kurzinfo: Spellslinger aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "While in-combat whenever you cast an ability or item, gain a stacking buff that improves fire rate and reload speed. <br><span class=\"diminish\">Each stack refreshes the duration.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "CooldownReduction",
        "Type": "cooldown",
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
        "Key": "MaxStacks",
        "Value": 6
      },
      {
        "Key": "BuffDuration",
        "Type": "duration",
        "Value": 18
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_enchanted_holsters_desc",
    "Main": [
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": 11
      },
      {
        "Key": "ReloadSpeedMultipler",
        "Type": "reload_speed",
        "UsageFlags": "IntrinsicallyProvidedInAbility",
        "Value": -10
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_enchanted_holsters",
  "Name": "Spellslinger",
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
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 4,
  "Upgrades": {
    "BonusFireRate": 6,
    "CooldownReduction": 8,
    "ReloadSpeedMultipler": -3
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
      "lookup": "spellslinger",
      "name": "Spellslinger",
      "type": "item"
    }
  ]
}
````
