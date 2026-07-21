---
title: "Hook Blade"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_viper_hookdagger"
canonical_name: "Hook Blade"
snapshot_id: 39563
source_document_id: 7070
payload_hash: "95b703e874742e652f31a37b47780da5813c6e7a7d2396dc32f14208c88ee317"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.644112+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hook Blade

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_hookdagger`
- Snapshot ID: `39563`
- Source-Dokument: `7070`
- Kurzinfo: Hook Blade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCleaveDisabled"
  ],
  "CatchRadius": 3,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_viper_hookdagger",
  "Name": "Hook Blade",
  "OutgoingDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 45.0
  },
  "OutgoingProjectileLifetime": {
    "Scale": {
      "Type": "range",
      "Value": 1.0
    },
    "Value": 0.4
  },
  "ReturnDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 90
  },
  "ReturnRadius": 1,
  "ReturnVelocity": 2300,
  "SlowDebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "ViperSlow"
  },
  "SlowDuration": 1,
  "SlowPercent": 35,
  "SpreadAngle": 90,
  "TickRate": 0.01,
  "Upgrades": [
    {
      "AbilityCooldown": -4
    },
    {
      "SlowPercent": 35
    },
    {}
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
