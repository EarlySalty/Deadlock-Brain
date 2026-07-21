---
title: "Haunting Shot"
entity_type: "item_card"
source: "deadlock_data"
external_id: "upgrade_eldritch_shot"
canonical_name: "Haunting Shot"
snapshot_id: 40334
source_document_id: 7073
payload_hash: "c7f4d522559709b2d2f6591281f5f66760395a31ee8aee7ce3d6dab3ad0a3820"
source_content_hash: "9af40bc46f0979c8c35e73f4f5306c8fe4514a0b07a7321457a182f97f85920f"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-cards.json.9af40bc46f0979c8.json"
fetched_at: "2026-07-09T19:36:25.412294+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item_card"]
---

# Haunting Shot

## Kurzueberblick

- Typ: `item_card`
- Quelle: `deadlock_data`
- External ID: `upgrade_eldritch_shot`
- Snapshot ID: `40334`
- Source-Dokument: `7073`
- Kurzinfo: Haunting Shot aus `deadlock_data` / `item_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "Activation": "Passive",
  "Components": null,
  "Cost": 9999,
  "Description": "Your next bullet applies a powerful debuff reducing the enemy's damage output, healing and movement speed. It also deals {g:citadel_inline_attribute:'BonusSpiritDamage'} based on the targets current Health. <br><br>The bullet is larger and penetrates through targets.",
  "Info1": {
    "Alt": [
      {
        "Key": "BulletRadius",
        "Value": "1.5m"
      },
      {
        "Key": "DebuffDuration",
        "Type": "duration",
        "Value": 4
      },
      {
        "Key": "MovementSpeedSlow",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": 40
      },
      {
        "Key": "GroundDashReductionPercent",
        "Type": "slow",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "ChargeUp": null,
    "Cooldown": 2.5,
    "DescKey": "#upgrade_eldritch_shot_desc",
    "Main": [
      {
        "Key": "HealthPctDamage",
        "Type": "tech_damage",
        "Value": 10
      },
      {
        "Key": "OutgoingDamagePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      },
      {
        "Key": "HealAmpReceivePenaltyPercent",
        "Type": "damage",
        "UsageFlags": "ConditionallyApplied",
        "Value": -40
      }
    ],
    "Type": null
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eldritch_shot",
  "Name": "Haunting Shot",
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
    "HealAmpRegenPenaltyPercent": {
      "Key": "HealAmpRegenPenaltyPercent",
      "Type": "damage",
      "UsageFlags": "ConditionallyApplied",
      "Value": -40
    },
    "ProcChance": {
      "Key": "ProcChance",
      "Value": 100
    },
    "ProcCooldown": {
      "Key": "ProcCooldown",
      "Type": "cooldown",
      "Value": 1
    },
    "Radius": {
      "Key": "Radius",
      "Value": "1m"
    }
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
  "Upgrades": {
    "AbilityCooldown": -1,
    "GroundDashReductionPercent": -10,
    "HealAmpReceivePenaltyPercent": -15,
    "HealAmpRegenPenaltyPercent": -15,
    "HealthPctDamage": 5,
    "MovementSpeedSlow": 10,
    "OutgoingDamagePenaltyPercent": -15
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
      "lookup": "haunting shot",
      "name": "Haunting Shot",
      "type": "item"
    }
  ]
}
````
