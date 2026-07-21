---
title: "Blessed Tac-Vest"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_antispiritvest"
canonical_name: "Blessed Tac-Vest"
snapshot_id: 39509
source_document_id: 7070
payload_hash: "75e20baf06d3d8a0ed34ba747e9a8e636fd173394d678611910eaecc32082605"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.508556+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Blessed Tac-Vest

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_antispiritvest`
- Snapshot ID: `39509`
- Source-Dokument: `7070`
- Kurzinfo: Blessed Tac-Vest aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BlockBufferDuration": 0.5,
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "BulletResist": 20,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 100
  },
  "IsDisabled": false,
  "Key": "ability_priest_antispiritvest",
  "Name": "Blessed Tac-Vest",
  "ShieldBreakModifier": {
    "Class": "Base",
    "Subclass": "Shieldbreak"
  },
  "StackingModifier": {
    "Class": "PriestStackingdefense",
    "Subclass": "Stackingdefense"
  },
  "TechResist": 20,
  "Upgrades": [
    {
      "CombatBarrier": 50
    },
    {
      "AbilityCooldown": -4
    },
    {
      "BaseAttackDamagePercent": 35,
      "BuffDuration": 6
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
