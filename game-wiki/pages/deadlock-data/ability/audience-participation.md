---
title: "Audience Participation"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_copyult"
canonical_name: "Audience Participation"
snapshot_id: 39477
source_document_id: 7070
payload_hash: "dc07a1c5487aac5d344af9781efa4d43d6bbec568b40714b3416d987d7821d52"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.433784+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Audience Participation

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_copyult`
- Snapshot ID: `39477`
- Source-Dokument: `7070`
- Kurzinfo: Audience Participation aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 20,
  "AbilityCooldown": 85,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "CopiedUltSpawnedEntityModifier": {
    "Class": "CitadelCopiedultSpawnedentity",
    "EnabledStateMask": [
      "MaterialOverride"
    ],
    "Subclass": "CopiedultSpawnedentity"
  },
  "CopiedUltWindow": 12,
  "CopyCooldownPercentage": 40,
  "CopyInternalCooldown": 0.5,
  "InformTargetUltCopiedModifier": {
    "Class": "Base",
    "Subclass": "CopyultInformtargetultcopied"
  },
  "IsDisabled": false,
  "Key": "ability_magician_copyult",
  "Name": "Audience Participation",
  "UltActiveModifier": {
    "Class": "CitadelCopyult",
    "Subclass": "CopyultModifier"
  },
  "UltCopiedModifier": {
    "Class": "CitadelCopyultpending",
    "Subclass": "Ultcopied"
  },
  "Upgrades": [
    {},
    {},
    {}
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
    "card_name": "Audience Participation",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "audience participation",
      "name": "Audience Participation",
      "type": "ability"
    }
  ]
}
````
