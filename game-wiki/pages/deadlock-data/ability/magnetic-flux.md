---
title: "Magnetic Flux"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_wrecker_ultimate"
canonical_name: "Magnetic Flux"
snapshot_id: 39588
source_document_id: 7070
payload_hash: "31a8f8ded09e328d6d73534342bc3349a130c8a3dc78e3fffed02667b1951d37"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.706800+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Magnetic Flux

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_ultimate`
- Snapshot ID: `39588`
- Source-Dokument: `7070`
- Kurzinfo: Magnetic Flux aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1.0,
  "AbilityChannelTime": 5,
  "AbilityCooldown": 170.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AccelSpeed": 2400,
  "ActiveModifier": {
    "Class": "WreckerUltimate",
    "EnemyDamageModifier": {
      "Class": "DamageOnHitGround",
      "Subclass": "WreckerUltimateDamageEnemy"
    },
    "EnemyGrabModifier": {
      "Class": "WreckerUltimateGrabEnemy",
      "Subclass": "WreckerUltimateGrabEnemy"
    },
    "EnemyThrowModifier": {
      "Class": "WreckerUltimateThrowEnemy",
      "Subclass": "WreckerUltimateThrowEnemy"
    },
    "InvincibleModifier": {
      "Class": "WreckerUltimateInvincible",
      "EnabledStateMask": [
        "Invulnerable",
        "TechUntargetable",
        "UnitStatusHealthHidden",
        "IgnoreBullets",
        "IgnoreMelee"
      ],
      "Subclass": "WreckerInvincible"
    },
    "Subclass": "WreckerUltimateSubclass"
  },
  "BeamLength": 20,
  "BeamWidth": 40,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": 5,
  "GrabDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 0
  },
  "GrabRange": 5,
  "HoldDistance": 2,
  "HoldHeight": 120,
  "IsDisabled": false,
  "Key": "ability_wrecker_ultimate",
  "Name": "Magnetic Flux",
  "PullSpeed": 800,
  "StunDuration": 1.5,
  "ThrowDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 160
  },
  "ThrowSpeed": 500,
  "TimeUntilStasis": 0.5,
  "TrackingSpeed": 70,
  "Upgrades": [
    {
      "BeamLength": 10
    },
    {
      "AbilityCooldown": -38.0
    },
    {
      "Invulnerable": 1
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
