---
title: "Split Shot"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_split_shot"
canonical_name: "Split Shot"
snapshot_id: 40466
source_document_id: 7073
payload_hash: "c5e8200723f1862b7ae405ac26a28305eb796b84fc03751e2aaacd5e47019f8f"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.655350+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Split Shot

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_split_shot`
- Snapshot ID: `40466`
- Source-Dokument: `7073`
- Kurzinfo: Split Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "InstantCast",
  "Components": null,
  "Cost": 1600,
  "Description": "Make your weapon fire <span class=\"highlight\">multishot</span>. <br><br> Hitting more than one Hero per attack will grant a <span class=\"highlight\">stacking weapon damage bonus</span>. <br><br><span class=\"diminish\">Targets can only be hit once per multishot.</span>",
  "Info1": {
    "Alt": [
      {
        "Key": "BonusShotsDuration",
        "LocTokenOverride": "BuffDuration",
        "Value": 5
      },
      {
        "Key": "MaxStacks",
        "Value": 5
      },
      {
        "Key": "WeaponDamageBonusDuration",
        "LocTokenOverride": "SplitShotWeaponDuration",
        "Value": 12
      }
    ],
    "ChargeUp": null,
    "Cooldown": 27,
    "DescKey": "#upgrade_split_shot_desc",
    "Main": [
      {
        "Key": "BulletSplitShot",
        "Type": "fire_rate",
        "UsageFlags": "ConditionallyApplied",
        "Value": 5
      },
      {
        "Key": "WeaponDamagePerStack",
        "Type": "bullet_damage",
        "Value": 8
      }
    ],
    "Type": "Active"
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_split_shot",
  "Name": "Split Shot",
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
    "SpreadAngleDegrees": {
      "Key": "SpreadAngleDegrees",
      "Value": 45
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
    "AbilityCooldown": -8,
    "BulletSplitShot": 4,
    "WeaponDamagePerStack": 8
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
      "lookup": "split shot",
      "name": "Split Shot",
      "type": "item"
    }
  ]
}
````
