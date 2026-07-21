---
title: "Lucky Shot"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_critshot"
canonical_name: "Lucky Shot"
snapshot_id: 40321
source_document_id: 7073
payload_hash: "267ec7ada193b1e12c0254c127ce1fff72a4512a34f7b371d432e4d0a6b6bab3"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.388470+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Lucky Shot

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_critshot`
- Snapshot ID: `40321`
- Source-Dokument: `7073`
- Kurzinfo: Lucky Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets have a chance to be empowered, causing them to deal <span class=\"highlight\">bonus weapon damage</span> on hit.<br><span class=\"diminish\">Bonus damage cannot Crit.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusClipSizePercent",
        "Value": 30
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
        "Key": "ProcChance",
        "Value": 25
      }
    ],
    "ChargeUp": null,
    "Cooldown": null,
    "DescKey": "#upgrade_critshot_desc",
    "Main": [
      {
        "Key": "CritDamagePercent",
        "Type": "bullet_damage",
        "Value": 100
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_critshot",
  "Name": "Lucky Shot",
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
    "Radius": {
      "Key": "Radius",
      "Type": "distance",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 4,
  "Upgrades": {
    "BonusClipSizePercent": 40,
    "CritDamagePercent": 30,
    "ProcChance": 5
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
      "lookup": "lucky shot",
      "name": "Lucky Shot",
      "type": "item"
    }
  ]
}
````
