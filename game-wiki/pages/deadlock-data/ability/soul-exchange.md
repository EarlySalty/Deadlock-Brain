---
title: "Soul Exchange"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_health_swap"
canonical_name: "Soul Exchange"
snapshot_id: 39459
source_document_id: 7070
payload_hash: "60f3feb0c9e0cb5c591e8013d5ac7822fbceb8e3a3a665eab311d3c7c5e22653"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.379297+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Soul Exchange

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_health_swap`
- Snapshot ID: `39459`
- Source-Dokument: `7070`
- Kurzinfo: Soul Exchange aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 5.5,
  "AbilityCooldown": 220.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Base"
  },
  "ChannelMoveSpeed": 2,
  "EnemyMinHealthPct": 30,
  "EnemySlowPct": 70,
  "InitialUpSpeed": 150,
  "IsDisabled": false,
  "Key": "ability_health_swap",
  "MinDiffToCast": 0.1,
  "MinHealthTakenPct": 30,
  "Name": "Soul Exchange",
  "PostCastHoldTime": 0.2,
  "PreCastModifier": {
    "Class": "HealthswapPrecast",
    "Subclass": "HealthswapPrecast"
  },
  "SilenceModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "CitadelSilenced"
  },
  "SwapModifier": {
    "Class": "HealthSwapDebuff",
    "Subclass": "HealthSwapDebuff"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -50.0
    },
    {
      "SilenceDuration": 3,
      "SilenceRadius": 25
    },
    {
      "BonusFireRate": 40,
      "BonusSpirit": 60,
      "SelfBuffDuration": 8,
      "TechResist": 50
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
    "card_name": "Soul Exchange",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "soul exchange",
      "name": "Soul Exchange",
      "type": "ability"
    }
  ]
}
````
