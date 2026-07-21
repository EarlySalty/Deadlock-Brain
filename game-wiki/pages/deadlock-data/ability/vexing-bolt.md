---
title: "Vexing Bolt"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_magicbolt"
canonical_name: "Vexing Bolt"
snapshot_id: 39479
source_document_id: 7070
payload_hash: "a979a8df3e10d8d9ee56eaf22a7aed3c98d4ba8fc297eb99653846a60f75257e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.439711+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Vexing Bolt

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_magicbolt`
- Snapshot ID: `39479`
- Source-Dokument: `7070`
- Kurzinfo: Vexing Bolt aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 500,
  "AbilityCharges": 1,
  "AbilityCooldown": 24,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorTargetThroughWalls",
    "BehaviorCleaveDisabled",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "CloneBoltDelay": 0.1,
  "CloneDamagePercentage": 50.0,
  "InitialProjectileVelocity": 800,
  "IsDisabled": false,
  "Key": "ability_magician_magicbolt",
  "MaxDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.86
    },
    "Value": 120
  },
  "MaxDamageTime": 2,
  "MinDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 60
  },
  "Name": "Vexing Bolt",
  "ProjectileLifetime": 4,
  "ProjectileRedirectCount": 1,
  "Radius": 3.25,
  "RedirectVelocity": 1500,
  "TargetDebuffModifier": {
    "Class": "Base",
    "Subclass": "MagicianMagicboltDebuff"
  },
  "Upgrades": [
    {
      "DebuffDuration": 5,
      "FireRateSlow": 25
    },
    {
      "AbilityCooldown": -13
    },
    {
      "CloneDamagePercentage": 50.0,
      "MaxDamage": 126.0
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
    "card_name": "Vexing Bolt",
    "hero_key": "hero_magician",
    "hero_name": "Sinclair",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_magician",
      "hero_name": "Sinclair",
      "lookup": "vexing bolt",
      "name": "Vexing Bolt",
      "type": "ability"
    }
  ]
}
````
