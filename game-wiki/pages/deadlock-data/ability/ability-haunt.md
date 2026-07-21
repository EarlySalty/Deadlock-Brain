---
title: "ability_haunt"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_haunt"
canonical_name: "ability_haunt"
snapshot_id: 39458
source_document_id: 7070
payload_hash: "fbb46915898f2f2e286032838bbdcd3cb376daf210db5fda3206e56e86332584"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.377291+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# ability_haunt

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_haunt`
- Snapshot ID: `39458`
- Source-Dokument: `7070`
- Kurzinfo: ability_haunt aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "AfterburnDotModifier": {
        "Class": "AfterburnDot",
        "StatusEffectPriority": 50,
        "Subclass": "AfterburnDot"
      },
      "BuildUpModifier": {
        "Class": "CitadelBaseBuildup",
        "Subclass": "CitadelBaseBuildup"
      },
      "Class": "AfterburnWatcher",
      "Subclass": "AfterburnWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorCleaveDisabled",
    "BehaviorDisplaysDamageImpact"
  ],
  "BuildUpBulletPercentPerHit": 8.33,
  "BuildUpDuration": 0.1,
  "BurnDuration": 0.3,
  "ChannelMoveSpeed": -1,
  "CritBuildup": 16,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 15
  },
  "IsDisabled": false,
  "Key": "ability_haunt",
  "Name": null,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AfterburnSpiritDamageReduction": -30
    },
    {
      "BurnDuration": 1
    },
    {
      "DPS": 30
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
    "card_name": "Unknown(ability_haunt)",
    "hero_key": "hero_vandal",
    "hero_name": "Vandal",
    "slot": "3"
  }
}
````
