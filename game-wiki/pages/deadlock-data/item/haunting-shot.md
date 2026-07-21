---
title: "Haunting Shot"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_eldritch_shot"
canonical_name: "Haunting Shot"
snapshot_id: 40055
source_document_id: 7072
payload_hash: "970c446a97d269ea115414dc8657970e96255d075a000c77050b2ef5538ee638"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.763100+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Haunting Shot

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_eldritch_shot`
- Snapshot ID: `40055`
- Source-Dokument: `7072`
- Kurzinfo: Haunting Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 2.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletRadius": "1.5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "DebuffDuration": 4,
  "Description": "Your next bullet applies a powerful debuff reducing the enemy's damage output, healing and movement speed. It also deals {g:citadel_inline_attribute:'BonusSpiritDamage'} based on the targets current Health. <br><br>The bullet is larger and penetrates through targets.",
  "GroundDashReductionPercent": -40,
  "HealAmpReceivePenaltyPercent": -40,
  "HealAmpRegenPenaltyPercent": -40,
  "HealthPctDamage": 10,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eldritch_shot",
  "MovementSpeedSlow": 40,
  "Name": "Haunting Shot",
  "OutgoingDamagePenaltyPercent": -40,
  "ProcChance": 100,
  "ProcCooldown": 1,
  "PropertyUpgrades": {
    "AbilityCooldown": -1,
    "GroundDashReductionPercent": -10,
    "HealAmpReceivePenaltyPercent": -15,
    "HealAmpRegenPenaltyPercent": -15,
    "HealthPctDamage": 5,
    "MovementSpeedSlow": 10,
    "OutgoingDamagePenaltyPercent": -15
  },
  "Radius": "1m",
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
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-data.json",
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
