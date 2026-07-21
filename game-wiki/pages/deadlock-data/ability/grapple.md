---
title: "Grapple"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_lash"
canonical_name: "Grapple"
snapshot_id: 39623
source_document_id: 7070
payload_hash: "3cff016b23ea73b8f1c9dfcbb8760ec0b03bdcad5be43191295f6c1532a93d5a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.799904+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Grapple

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_lash`
- Snapshot ID: `39623`
- Source-Dokument: `7070`
- Kurzinfo: Grapple aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityCharges": 1,
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": 2,
  "AbilityUnitTargetLimit": 1,
  "AirControlModifier": {
    "Class": "GrappleAirControl",
    "Subclass": "AirControl"
  },
  "BehaviourBits": [
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "GrappleBuff"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "JumpSlowResistance": 0.667,
  "JumpVelocity": 20,
  "Key": "citadel_ability_lash",
  "LashFriendlies": 1,
  "Name": "Grapple",
  "Upgrades": [
    {
      "AbilityCooldown": -17.0
    },
    {
      "AbilityCastRange": 20,
      "WeaponDamageBonus": 7.0,
      "WeaponDamageBonusDuration": 10
    },
    {
      "AbilityCharges": 1,
      "AirControlPercent": 60,
      "RestoreStaminaOnUse": 1
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-data.json",
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
  "_deadlock_data_card": {
    "card_name": "Grapple",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "grapple",
      "name": "Grapple",
      "type": "ability"
    }
  ]
}
````
