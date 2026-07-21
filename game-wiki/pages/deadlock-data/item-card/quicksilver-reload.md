---
title: "Quicksilver Reload"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_quick_silver"
canonical_name: "Quicksilver Reload"
snapshot_id: 40426
source_document_id: 7073
payload_hash: "530ff391e994a19f64963989b62ade159ef711822547d873d52550440a234a7c"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.570645+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Quicksilver Reload

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_quick_silver`
- Snapshot ID: `40426`
- Source-Dokument: `7073`
- Kurzinfo: Quicksilver Reload aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 1600,
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use.",
  "Info1": {
    "Alt": [],
    "ChargeUp": 18,
    "Cooldown": null,
    "DescKey": "#upgrade_quick_silver_desc",
    "Main": [
      {
        "Key": "Damage",
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Type": "tech_damage",
        "Value": 44.0
      },
      {
        "Key": "BonusFireRate",
        "Type": "fire_rate",
        "Value": 10
      },
      {
        "Key": "AmmoReloadPercent",
        "Type": "clipsize",
        "Value": 100
      }
    ],
    "Type": "Passive"
  },
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_quick_silver",
  "Name": "Quicksilver Reload",
  "Other": {
    "AbilityCooldown": {
      "Key": "AbilityCooldown",
      "Type": "cooldown",
      "Value": 18
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
  "Tier": 2,
  "Upgrades": {
    "AbilityChargeUpTime": -4,
    "AbilityCooldown": -4,
    "BonusFireRate": 20,
    "Damage": 56
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
      "lookup": "quicksilver reload",
      "name": "Quicksilver Reload",
      "type": "item"
    }
  ]
}
````
