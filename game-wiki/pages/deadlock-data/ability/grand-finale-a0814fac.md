---
title: "Grand Finale!"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_bigbolt"
canonical_name: "Grand Finale!"
snapshot_id: 39474
source_document_id: 7070
payload_hash: "e8c74b45c318dca990ce9b722b291d7d3f22db21ce257dd6584311e114261199"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.426611+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Grand Finale!

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_bigbolt`
- Snapshot ID: `39474`
- Source-Dokument: `7070`
- Kurzinfo: Grand Finale! aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 500,
  "AbilityChannelTime": 8,
  "AbilityCooldown": 100,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityPostCastDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "AirSpeedMax": 70,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCooldownOnChannelEnd"
  ],
  "BoltHitModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "MagicianBigboltModifier"
  },
  "BoltRefundPerKill": 1,
  "CasterModifier": {
    "Class": "Base",
    "Subclass": "MagicianCasterBigboltModifier"
  },
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.395
    },
    "Value": 120
  },
  "DamagePerShot": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 50
  },
  "DebuffDuration": 2,
  "FallSpeedMax": 1,
  "InitialProjectileVelocity": 1000,
  "IsDisabled": false,
  "Key": "ability_magician_bigbolt",
  "Name": "Grand Finale!",
  "ProjectileLifetime": 3,
  "ProjectileRedirectCount": 1,
  "Radius": 3,
  "RedirectVelocity": 1500,
  "ShootDelay": 0.7,
  "SlowPercent": 25,
  "TotalBolts": 3,
  "Upgrades": [
    {
      "SlowPercent": 25
    },
    {
      "AbilityCooldown": -40
    },
    {
      "BoltRefundPerKill": 1,
      "DamagePerShot": 50
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
