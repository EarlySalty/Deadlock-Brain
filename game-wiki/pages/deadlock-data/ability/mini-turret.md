---
title: "Mini Turret"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_shieldedsentry"
canonical_name: "Mini Turret"
snapshot_id: 39638
source_document_id: 7070
payload_hash: "4973553537b3d18a3164c0488a6dd777adedb14237d34ccf065bffc0948ac761"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.834880+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Mini Turret

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shieldedsentry`
- Snapshot ID: `39638`
- Source-Dokument: `7070`
- Kurzinfo: Mini Turret aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 20,
  "AbilityCharges": 1,
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityUnitTargetLimit": 1,
  "AttackConeAngle": 10,
  "AttackSpeedMult": 100,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventTrainingBotUsage",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BossDamagePercentIncoming": 50,
  "BossDamagePercentOutgoing": 30,
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "ForgeMiniTurretDebuff"
  },
  "DecayingResist": 80,
  "DecayingResistDuration": 6,
  "InnateModifier": {
    "Class": "ForgeMiniTurretInnateModifier",
    "Subclass": "ForgeMiniTurretInnateModifier"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_shieldedsentry",
  "MeleeResist": 35,
  "ModelScale": 0.8,
  "Name": "Mini Turret",
  "NonHeroDamagePercentOutgoing": 50,
  "TechResist": 35,
  "TickRate": 0.5,
  "TrackingSpeed": 430,
  "TurretAttackDelay": 0.2,
  "TurretAttackFalloffEnd": 30,
  "TurretAttackFalloffStart": 20,
  "TurretAttackRange": 30,
  "TurretBaseHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 7.8
    },
    "Value": 90
  },
  "TurretDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.42
    },
    "Value": 24
  },
  "TurretDeployTime": 0.25,
  "TurretLifetime": 35,
  "Upgrades": [
    {
      "TurretAttackRange": 10,
      "TurretDPS": 10
    },
    {
      "AbilityCharges": 2
    },
    {
      "AttackSpeedMult": 25,
      "TurretLifetime": 12
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
    "card_name": "Mini Turret",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "mini turret",
      "name": "Mini Turret",
      "type": "ability"
    }
  ]
}
````
