---
title: "Mercurial Magnum"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_ethereal_bullets"
canonical_name: "Mercurial Magnum"
snapshot_id: 40340
source_document_id: 7073
payload_hash: "824b31e0c01b68ae62a9f384e9404af3e127608c66fc8a5dbadea139a257e9e9"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.424586+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Mercurial Magnum

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_ethereal_bullets`
- Snapshot ID: `40340`
- Source-Dokument: `7073`
- Kurzinfo: Mercurial Magnum aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": [
    "upgrade_quick_silver"
  ],
  "Cost": 6400,
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use. Until your next reload, your <span class=\"highlight\">bullets deal {g:citadel_inline_attribute:'BonusSpiritDamage'}</span> based on your Spirit Power.",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 20
      },
      {
        "Key": "TechPower",
        "Value": 7
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
        "Key": "AmmoReloadPercent",
        "Value": 100
      }
    ],
    "ChargeUp": 14,
    "Cooldown": null,
    "DescKey": "#upgrade_ethereal_bullets_desc",
    "Main": [
      {
        "Key": "BulletsBonusMagicDamage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.49
        },
        "Type": "tech_damage",
        "Value": 25.0
      },
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Type": "tech_damage",
        "Value": 60.0
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 22
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_ethereal_bullets",
  "Name": "Mercurial Magnum",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 15
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
    "BuffDuration": {
      "Key": "BuffDuration",
      "Value": 12
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
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusClipSizePercent": 60,
    "BonusFireRate": 20,
    "BulletsBonusMagicDamage": 20,
    "Damage": 120,
    "TechPower": 15
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
      "lookup": "mercurial magnum",
      "name": "Mercurial Magnum",
      "type": "item"
    }
  ]
}
````
