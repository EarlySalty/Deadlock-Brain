---
title: "upgrade_stabilizing_tripod"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_stabilizing_tripod"
canonical_name: "upgrade_stabilizing_tripod"
snapshot_id: 40469
source_document_id: 7073
payload_hash: "7a1af5a7269c738ed89654dd8e15ecac79a79c4f4256e5ccee213a594c817346"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.662935+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# upgrade_stabilizing_tripod

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_stabilizing_tripod`
- Snapshot ID: `40469`
- Source-Dokument: `7073`
- Kurzinfo: upgrade_stabilizing_tripod aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Press",
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "Info1": {
    "Alt": [
      {
        "Key": "StationaryWeaponPower",
        "Value": 40
      },
      {
        "Key": "StationaryRecoilReduction",
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
        "Key": "TurretAttackRange",
        "Type": "distance",
        "Value": "50m"
      },
      {
        "Key": "CasterHealthPercent",
        "Value": 100
      }
    ],
    "ChargeUp": null,
    "Cooldown": 64.0,
    "DescKey": "#upgrade_stabilizing_tripod_desc",
    "Main": [
      {
        "Key": "TurretLifetime",
        "Type": "duration",
        "Value": 20
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": true,
  "IsImbue": false,
  "Key": "upgrade_stabilizing_tripod",
  "Name": null,
  "Other": {
    "AbilityCastRange": {
      "Key": "AbilityCastRange",
      "Type": "range",
      "Value": "15m"
    },
    "AbilityCooldownBetweenCharge": {
      "Key": "AbilityCooldownBetweenCharge",
      "Type": "charge_cooldown",
      "Value": -1.0
    },
    "AbilityDuration": {
      "Key": "AbilityDuration",
      "Type": "duration",
      "Value": 20
    },
    "AbilityUnitTargetLimit": {
      "Key": "AbilityUnitTargetLimit",
      "Value": 1
    },
    "AttackConeAngle": {
      "Key": "AttackConeAngle",
      "Value": 10
    },
    "BulletSpeedOverride": {
      "Key": "BulletSpeedOverride",
      "Value": 6500
    },
    "ChannelMoveSpeed": {
      "Key": "ChannelMoveSpeed",
      "Type": "move_speed",
      "Value": -1
    },
    "ModelScale": {
      "Key": "ModelScale",
      "Value": 1.0
    },
    "TickRate": {
      "Key": "TickRate",
      "Value": 0.5
    },
    "TrackingSpeed": {
      "Key": "TrackingSpeed",
      "Value": 125
    },
    "TurretAttackDelay": {
      "Key": "TurretAttackDelay",
      "Value": 0.35
    },
    "TurretDeployTime": {
      "Key": "TurretDeployTime",
      "Type": "duration",
      "Value": 2.0
    }
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
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
  }
}
````
