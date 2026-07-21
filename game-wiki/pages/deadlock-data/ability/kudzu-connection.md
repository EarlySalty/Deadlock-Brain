---
title: "Kudzu Connection"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_tangotether"
canonical_name: "Kudzu Connection"
snapshot_id: 39650
source_document_id: 7070
payload_hash: "148aa34ebd9d7970d081a1993e07b18db9a123c607452bafdfb14f86d62d75b9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.863606+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Kudzu Connection

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tangotether`
- Snapshot ID: `39650`
- Source-Dokument: `7070`
- Kurzinfo: Kudzu Connection aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 16,
  "AbilityCooldown": 37.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.18
    },
    "Value": 10
  },
  "BulletLifestealPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.15
    },
    "Value": 15
  },
  "ChannelMoveSpeed": -1,
  "GrappleTargetModifier": {
    "Class": "CitadelModifierTangotetherTarget",
    "Subclass": "Target"
  },
  "HealingPerGlub": 20,
  "IsDisabled": false,
  "Key": "citadel_ability_tangotether",
  "MoveWhileShootingSpeedPenaltyReductionPercent": 100,
  "MoveWhileZoomedSpeedPenaltyReductionPercent": 100,
  "Name": "Kudzu Connection",
  "TetherModifier": {
    "BuffModifier": {
      "Class": "CitadelModifierTangotetherTetherReceiver",
      "EnabledStateMask": [
        "CoopTetherActive"
      ],
      "Subclass": "CitadelModifierTangotetherTetherReceiver"
    },
    "CandidateCloserDistance": 400.0,
    "Class": "CitadelModifierTangotetherTether",
    "DisconnectDistanceBuffer": 100.0,
    "LockedTargetModifier": {
      "Class": "Base",
      "EnabledStateMask": [
        "CoopTetherLockedTarget"
      ],
      "Subclass": "TetherLockedModifier"
    },
    "MinConnectTime": 2.0,
    "NoConnectionModifier": {
      "Class": "TetherNoConnection",
      "Subclass": "TetherNoConnection"
    },
    "StatusEffectPriority": 50,
    "Subclass": "CitadelModifierTangotetherTether",
    "TargetAwayDistance": 250.0
  },
  "TetherSharedHealPct": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.85
    },
    "Value": 35
  },
  "TickRate": 0.1,
  "TotalTetherTargets": 1,
  "Upgrades": [
    {
      "MoveSpeedBonus": 2
    },
    {
      "BonusFireRate": 8,
      "BulletLifestealPercent": 8
    },
    {
      "AbilityCooldown": -37,
      "AbilityDuration": -13
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
    "card_name": "Kudzu Connection",
    "hero_key": "hero_tengu",
    "hero_name": "Ivy",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_tengu",
      "hero_name": "Ivy",
      "lookup": "kudzu connection",
      "name": "Kudzu Connection",
      "type": "ability"
    }
  ]
}
````
