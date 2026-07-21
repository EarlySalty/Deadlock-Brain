---
title: "Time Wall"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_chrono_time_wall"
canonical_name: "Time Wall"
snapshot_id: 39606
source_document_id: 7070
payload_hash: "d1ff39485770a7d910670ad723a4b388d270d91f845e9695521536b5f4091956"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.758716+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Time Wall

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chrono_time_wall`
- Snapshot ID: `39606`
- Source-Dokument: `7070`
- Kurzinfo: Time Wall aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 5.08,
  "AbilityCooldown": 25.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "AuraEffectDuration": 2,
  "AuraModifier": {
    "AuraRadius": 0.001,
    "Class": "CitadelChronoTimeWallAura",
    "DebuffModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "ProvidedByAura": {
      "Class": "CitadelChronoTimeWallEffect",
      "Subclass": "CitadelChronoTimeWallEffect"
    },
    "Subclass": "CitadelChronoTimeWallAura"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "FriendlyBulletDamageBonus": 30,
  "IsDisabled": false,
  "Key": "citadel_ability_chrono_time_wall",
  "MovementSlowPct": 80,
  "Name": "Time Wall",
  "TimeScaleDuration": 0.5,
  "TimeWallDepth": 0.5,
  "TimeWallDepthVisualScale": 0.16,
  "TimeWallFormationTime": 0.5,
  "TimeWallHeight": 4,
  "TimeWallTimeScale": 0.0001,
  "TimeWallTimeScaleFriendly": 2,
  "TimeWallWidth": 8,
  "Upgrades": [
    {
      "AbilityDuration": 3.5,
      "TimeWallHeight": 1,
      "TimeWallWidth": 3
    },
    {
      "DebuffDuration": 2.3,
      "FriendlyBulletDamageBonus": 35
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 2
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
    "card_name": "Time Wall",
    "hero_key": "hero_chrono",
    "hero_name": "Paradox",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_chrono",
      "hero_name": "Paradox",
      "lookup": "time wall",
      "name": "Time Wall",
      "type": "ability"
    }
  ]
}
````
