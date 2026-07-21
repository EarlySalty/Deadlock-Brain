---
title: "Afterburn"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_afterburn"
canonical_name: "Afterburn"
snapshot_id: 39392
source_document_id: 7070
payload_hash: "4f48e675969b2b2c9453e4e1daabcfd27679c41c85e865172be48dfe780931ad"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.208391+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Afterburn

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_afterburn`
- Snapshot ID: `39392`
- Source-Dokument: `7070`
- Kurzinfo: Afterburn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "AfterburnDotModifier": {
        "Class": "AfterburnDot",
        "EnabledStateMask": [
          "Afterburning"
        ],
        "StatusEffectPriority": 50,
        "Subclass": "AfterburnDot"
      },
      "BuildUpModifier": {
        "Class": "CitadelBaseBuildup",
        "Subclass": "CitadelBaseBuildup"
      },
      "Class": "AfterburnWatcher",
      "HeavyMeleeBuildUp": 35,
      "HeavyMeleeRefresh": 3.0,
      "LightMeleeBuildUp": 20,
      "LightMeleeRefresh": 1.5,
      "Subclass": "AfterburnWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorCleaveDisabled",
    "BehaviorDisplaysDamageImpact"
  ],
  "BuildUpBulletPercentPerHit": 8.1,
  "BuildUpDuration": 17,
  "BurnDuration": 3,
  "BurnDurationBase": 3,
  "ChannelMoveSpeed": -1,
  "CritBuildup": 15.4,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.66
    },
    "Value": 14.0
  },
  "IsDisabled": false,
  "Key": "ability_afterburn",
  "Name": "Afterburn",
  "RefillDuration": 0.5,
  "RefillDurationCrit": 1.0,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "DPS": 16
    },
    {
      "OutgoingTechDamagePercent": -35
    },
    {
      "BurnDuration": 3
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
    "card_name": "Afterburn",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "afterburn",
      "name": "Afterburn",
      "type": "ability"
    }
  ]
}
````
