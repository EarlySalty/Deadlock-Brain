---
title: "Hat Trick"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_hat_trick"
canonical_name: "Hat Trick"
snapshot_id: 39457
source_document_id: 7070
payload_hash: "98aba3225017cde8ecfb8fdebe315f68595f9d8ad905041f47afd39a43b957d3"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.374305+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hat Trick

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_hat_trick`
- Snapshot ID: `39457`
- Source-Dokument: `7070`
- Kurzinfo: Hat Trick aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 21.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93744
    },
    "Value": 100
  },
  "DebuffDuration": 5,
  "DebuffModifier": {
    "Class": "SlowBase",
    "EnabledStateMask": [
      "GlowThroughWallsToProvider",
      "SilenceMovementAbilites"
    ],
    "Subclass": "Slow"
  },
  "ExplosionRadius": 2,
  "IsDisabled": false,
  "Key": "ability_hat_trick",
  "Name": "Hat Trick",
  "SlowPercent": 20,
  "Upgrades": [
    {
      "Damage": 50
    },
    {
      "AbilityCooldown": -9.5
    },
    {
      "SlowPercent": 30
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
