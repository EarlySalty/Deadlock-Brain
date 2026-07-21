---
title: "Return to Shadows"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_nano_shadow_pulse"
canonical_name: "Return to Shadows"
snapshot_id: 39492
source_document_id: 7070
payload_hash: "3cb1a53f3626b108b6c3bd22a0a86c8c63048286bac2b5bc181f376abcfcb8ee"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.470409+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Return to Shadows

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_shadow_pulse`
- Snapshot ID: `39492`
- Source-Dokument: `7070`
- Kurzinfo: Return to Shadows aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": 3,
  "AbilityCooldown": 115,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 4,
  "AirSpeedMax": 100,
  "AutoChannelModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "DoNotDrawModel"
    ],
    "Subclass": "Channeling"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCastableWhileDodging",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorInhibitSoftCameraCollision"
  ],
  "BonusMoveSpeedPercent": 20,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 150.0
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "FallSpeedMax": 0.254,
  "IsDisabled": false,
  "Key": "ability_nano_shadow_pulse",
  "Name": "Return to Shadows",
  "Radius": 7.5,
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "BonusMoveSpeedPercent": 20,
      "Damage": 75
    },
    {
      "HealAmount": 450,
      "RefundCooldowns": 1
    }
  ],
  "ZAcceleration": 800,
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
    "card_name": "Return to Shadows",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "return to shadows",
      "name": "Return to Shadows",
      "type": "ability"
    }
  ]
}
````
