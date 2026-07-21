---
title: "Static Charge"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_static_charge_v2"
canonical_name: "Static Charge"
snapshot_id: 39646
source_document_id: 7070
payload_hash: "e12526d9b7074bb877bd72b65a267b14357be93d4ace8f5ecd6779e5bedbf554"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.854185+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Static Charge

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_static_charge_v2`
- Snapshot ID: `39646`
- Source-Dokument: `7070`
- Kurzinfo: Static Charge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0651
    },
    "Value": 16
  },
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.792137
    },
    "Value": 35
  },
  "IsDisabled": false,
  "Key": "citadel_ability_static_charge_v2",
  "Name": "Static Charge",
  "ShockDelay": 3.5,
  "ShockRadius": 5,
  "StaticChargeModifier": {
    "Class": "CitadelStaticcharge",
    "Subclass": "CitadelStaticcharge"
  },
  "StaticChargeWorldModifier": {
    "Class": "CitadelStaticcharge",
    "Subclass": "CitadelStaticchargeWorld"
  },
  "StunDuration": 0.9,
  "Upgrades": [
    {
      "AbilityCooldown": -19.0
    },
    {
      "ShockRadius": 7
    },
    {
      "StunDuration": 0.9
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
