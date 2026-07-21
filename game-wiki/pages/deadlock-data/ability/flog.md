---
title: "Flog"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_lash_flog"
canonical_name: "Flog"
snapshot_id: 39470
source_document_id: 7070
payload_hash: "fecd391eb8400da0d680cbbf47839b7b83595f5b5451652bfea0a2d37866cc21"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.413444+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flog

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_lash_flog`
- Snapshot ID: `39470`
- Source-Dokument: `7070`
- Kurzinfo: Flog aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 20,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 30,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorNoTarget",
    "BehaviorShowCastRangeAsSatSphereWhileCasting"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.85
    },
    "Value": 65
  },
  "FlogDebuffModifier": {
    "Class": "LashFlogDebuff",
    "Subclass": "LashFlogDebuff"
  },
  "HealPctVsHeroes": 50,
  "HealPctVsNonHeroes": 16,
  "IsDisabled": false,
  "Key": "ability_lash_flog",
  "Name": "Flog",
  "TargetingConeAngle": 38,
  "Upgrades": [
    {
      "EnemySlowDuration": 3,
      "EnemySlowPct": 35
    },
    {
      "AbilityCooldown": -16.0,
      "FireRateSlow": 30
    },
    {
      "Damage": 80,
      "HealPctVsHeroes": 20,
      "HealPctVsNonHeroes": 6,
      "TargetingConeAngle": 40
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
    "card_name": "Flog",
    "hero_key": "hero_lash",
    "hero_name": "Lash",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_lash",
      "hero_name": "Lash",
      "lookup": "flog",
      "name": "Flog",
      "type": "ability"
    }
  ]
}
````
