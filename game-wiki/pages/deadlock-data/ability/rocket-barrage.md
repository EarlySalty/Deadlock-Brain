---
title: "Rocket Barrage"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_tier2boss_rocket_barrage"
canonical_name: "Rocket Barrage"
snapshot_id: 39657
source_document_id: 7070
payload_hash: "542a623342fd21db4b312ea1556a75f78998ab4fb2bed49f572ccc689c81ebb8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.879713+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rocket Barrage

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tier2boss_rocket_barrage`
- Snapshot ID: `39657`
- Source-Dokument: `7070`
- Kurzinfo: Rocket Barrage aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 35.56,
  "AbilityCooldown": 10,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 100,
  "AuraModifier": {
    "Class": "Tier2bossRocketDamageAura",
    "ModifierProvidedByAuraDuration": 4.0,
    "ProvidedByAura": {
      "Class": "Tier2bossRocketDamageAuraDebuff",
      "Subclass": "Tier2bossRocketDamageAuraDebuff"
    },
    "Subclass": "Tier2bossRocketDamageAura"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorTargetThroughWalls"
  ],
  "BulletArmorReduction": -10,
  "ChannelMoveSpeed": -1,
  "Damage": 200,
  "DetonateTimer": 10,
  "ExplosionFalloffDisabled": 1,
  "FireDPS": 25,
  "FireDPSTrooperFactor": 0.3,
  "FireDuration": 5,
  "FireRadius": 6,
  "FireTickInterval": 0.5,
  "GrenadesInVolley": 6,
  "InitialVolleyInaccuracy": 100,
  "IsDisabled": false,
  "Key": "citadel_ability_tier2boss_rocket_barrage",
  "MaxSimultaneousVolley": 1,
  "Name": "Rocket Barrage",
  "PerVolleyInaccuracy": 25,
  "Radius": 5,
  "TechArmorReduction": -10,
  "Upgrades": [],
  "VolleyInterval": 0.5,
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
