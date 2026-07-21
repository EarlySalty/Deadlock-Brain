---
title: "Singularity"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_self_vacuum"
canonical_name: "Singularity"
snapshot_id: 39637
source_document_id: 7070
payload_hash: "3257b06055d46663a585fc09f77cc7f7841be457896d5626096f1e96a9868e12"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.832710+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Singularity

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_self_vacuum`
- Snapshot ID: `39637`
- Source-Dokument: `7070`
- Kurzinfo: Singularity aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChannelTime": 2.75,
  "AbilityCooldown": 265.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraDistance": 400,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.28
    },
    "Value": 75
  },
  "IsDisabled": false,
  "Key": "citadel_ability_self_vacuum",
  "Name": "Singularity",
  "Speed": 5.08,
  "TickRate": 0.25,
  "TossAngle": 45,
  "TossSpeed": 8.89,
  "Upgrades": [
    {
      "VacuumRadius": 2
    },
    {
      "AbilityChannelTime": 0.75
    },
    {
      "DPSPercentHealth": 6
    }
  ],
  "VacuumAuraModifier": {
    "Class": "Vacuumaura",
    "ProvidedByAura": {
      "Class": "VacuumauraTarget",
      "EnabledStateMask": [
        "ModNoCleanse"
      ],
      "OuterSpeedScale": 10.0,
      "StatusEffectPriority": 60,
      "Subclass": "VacuumauraTarget"
    },
    "StatusEffectPriority": 0,
    "Subclass": "Vacuumaura"
  },
  "VacuumRadius": 7,
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
    "card_name": "Singularity",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "singularity",
      "name": "Singularity",
      "type": "ability"
    }
  ]
}
````
