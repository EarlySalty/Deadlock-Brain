---
title: "Shakedown"
entity_type: "ability"
source: "deadlock_data"
external_id: "yakuza_shakedown_target"
canonical_name: "Shakedown"
snapshot_id: 39745
source_document_id: 7070
payload_hash: "06ae0e838e9c9e4fa8c9ecac9d1f72fde53f40057b7eb7ea0d61a9ca493e3f28"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.109260+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Shakedown

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `yakuza_shakedown_target`
- Snapshot ID: `39745`
- Source-Dokument: `7070`
- Kurzinfo: Shakedown aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTimeDisplay": 5,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 45
  },
  "IsDisabled": false,
  "Key": "yakuza_shakedown_target",
  "Name": "Shakedown",
  "PulseModifier": {
    "Class": "CitadelShakedownPulse",
    "Subclass": "CitadelShakedownPulse"
  },
  "Radius": 6,
  "RootModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Immobilized"
    ],
    "Subclass": "CitadelShakedownTarget"
  },
  "ShareDamagePercent": 33,
  "ShareDamageThreshold": 20,
  "TickTime": 0.5,
  "Upgrades": [
    {
      "WeaponPowerDebuff": -30
    },
    {
      "IgnoreChannelSlow": 1
    },
    {
      "ShareDamagePercent": 67
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yakuza",
      "hero_name": "The Boss",
      "lookup": "shakedown",
      "name": "Shakedown",
      "type": "ability"
    }
  ]
}
````
