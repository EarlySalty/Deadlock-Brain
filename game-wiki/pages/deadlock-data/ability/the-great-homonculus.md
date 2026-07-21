---
title: "The Great Homonculus!"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_shadowclone"
canonical_name: "The Great Homonculus!"
snapshot_id: 39481
source_document_id: 7070
payload_hash: "333e25925cf0087829c5cd070c530b2a4b9242be6aa364efb680332b2b940ce5"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.444068+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# The Great Homonculus!

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_shadowclone`
- Snapshot ID: `39481`
- Source-Dokument: `7070`
- Kurzinfo: The Great Homonculus! aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 20,
  "AbilityChannelTime": 1,
  "AbilityCooldown": 145.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "CloneDamagePercentage": 30,
  "CloneGoldToGive": 10000,
  "CloneHealthPercentage": 40,
  "CloneLifetime": 60,
  "CloneModifier": {
    "Class": "Shadowclone",
    "Subclass": "ShadowcloneModifierMagician"
  },
  "CloneSpawnDistance": 2,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.046
    },
    "Value": 220
  },
  "IsDisabled": false,
  "Key": "ability_magician_shadowclone",
  "Name": "The Great Homonculus!",
  "Upgrades": [
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
    },
    {
      "AbilityCooldown": 0
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
  }
}
````
