---
title: "Teargas Grenade"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_smokegrenade"
canonical_name: "Teargas Grenade"
snapshot_id: 39516
source_document_id: 7070
payload_hash: "a4b84d55da11ab4aaf0ea9df741ddc1a8ee8284a2886fd61cf6f8159c8fdeaf0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.524509+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Teargas Grenade

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_smokegrenade`
- Snapshot ID: `39516`
- Source-Dokument: `7070`
- Kurzinfo: Teargas Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 10,
  "AbilityCooldown": 24,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 99,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlockerScaleFactor": 115,
  "BombFriction": 12,
  "BombRestitution": 2.0,
  "BonusDamge": 12,
  "BonusFireRate": -30,
  "BonusHealthRegen": 1,
  "ChannelMoveSpeed": -1,
  "DebuffDuration": 3,
  "GrowTime": 0.2,
  "IsDisabled": false,
  "Key": "ability_priest_smokegrenade",
  "Name": "Teargas Grenade",
  "ProjectileLifetime": 2,
  "Radius": 6,
  "SlowDuration": 2,
  "SmokeGrenadeModifier": {
    "Class": "Smokegrenade",
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "Base",
        "Subclass": "Debuff"
      },
      "Subclass": "Enemyaura"
    },
    "Subclass": "PriestSmokegrenade"
  },
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "StaminaDrain": -1
    },
    {
      "Radius": 2
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
