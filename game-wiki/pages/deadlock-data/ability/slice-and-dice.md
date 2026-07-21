---
title: "Slice and Dice"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_shiv_dash"
canonical_name: "Slice and Dice"
snapshot_id: 39640
source_document_id: 7070
payload_hash: "eee179a5662c0eae0eee710faeaf18fca5bc2ff94e10d4c4104907feaadd8a54"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.839367+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Slice and Dice

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_dash`
- Snapshot ID: `39640`
- Source-Dokument: `7070`
- Kurzinfo: Slice and Dice aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "DashAngleThreshold": 89,
  "DashModifier": {
    "Class": "CitadelShivDash",
    "Subclass": "CitadelShivDash"
  },
  "DashRadius": 2.5,
  "DashRange": 12,
  "DashSpeed": 60.96,
  "DebuffDuration": 14,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "ShivDashDebuff"
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.4415
    },
    "Value": 75
  },
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_dash",
  "MoveSpeedPenaltyMaxSpeed": 200,
  "Name": "Slice and Dice",
  "SideMoveSpeedReduction": -100,
  "TechArmorDamageReduction": -6,
  "TechCleaveExpireTime": 0.35,
  "Upgrades": [
    {
      "AbilityCooldown": -6
    },
    {
      "DashRange": 2,
      "TechArmorDamageReduction": -6
    },
    {
      "CooldownReductionOnHit": 2,
      "CooldownReductionOnHitNonHero": 1,
      "ImpactDamage": 50,
      "MaxCooldownReductionsFromHits": 8
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
    "card_name": "Slice and Dice",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "slice and dice",
      "name": "Slice and Dice",
      "type": "ability"
    }
  ]
}
````
