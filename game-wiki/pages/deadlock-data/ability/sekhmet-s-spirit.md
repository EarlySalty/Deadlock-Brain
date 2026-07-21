---
title: "Sekhmet's Spirit"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_perched_predator"
canonical_name: "Sekhmet's Spirit"
snapshot_id: 39505
source_document_id: 7070
payload_hash: "97b1da7a8c99cfe2b1871297509d4c18fe9524694211474a277cf8c4189e94d9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.498483+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Sekhmet's Spirit

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_perched_predator`
- Snapshot ID: `39505`
- Source-Dokument: `7070`
- Kurzinfo: Sekhmet's Spirit aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorInhibitSoftCameraCollision"
  ],
  "CatAboveGround": 0.1,
  "CatAccel": 15,
  "CatClimbHeight": 3,
  "CatDropDownRate": 5,
  "CatLifetime": 2.5,
  "CatMaxSpeed": 25,
  "CatStartSpeed": 5,
  "ChannelMoveSpeed": -1,
  "ChargeDragVerticalOffset": 30,
  "ChargeRadius": 75,
  "ExplosionDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.209
    },
    "Value": 100
  },
  "ExplosionRadius": 8,
  "IsDisabled": false,
  "Key": "ability_perched_predator",
  "ModifierDragEnemy": {
    "Class": "PerchedPredatorDrag",
    "Subclass": "PerchedPredatorDrag"
  },
  "Name": "Sekhmet's Spirit",
  "TossSpeed": 400,
  "Upgrades": [
    {
      "ExplosionRadius": 4
    },
    {
      "AbilityCooldown": -11.5
    },
    {
      "ExplosionDamage": 120
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
