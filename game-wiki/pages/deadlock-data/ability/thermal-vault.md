---
title: "Thermal Vault"
entity_type: "ability"
source: "deadlock_data"
external_id: "tokamak_heat_sinks"
canonical_name: "Thermal Vault"
snapshot_id: 39725
source_document_id: 7070
payload_hash: "3097ec3b5712df0a364f44b6b64ca95a9996102ac67c02d76d52cfaaa9441532"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.057008+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Thermal Vault

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `tokamak_heat_sinks`
- Snapshot ID: `39725`
- Source-Dokument: `7070`
- Kurzinfo: Thermal Vault aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BaseHeatPower": 20.0,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "HeatDotModifier": {
    "Class": "TokamakHeatSinksDot",
    "Subclass": "TokamakHeatSinksDot"
  },
  "IsDisabled": false,
  "Key": "tokamak_heat_sinks",
  "MeleeBurnDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.487469
    },
    "Value": 80
  },
  "MeleeHitCount": 3,
  "MeleeIgniteTime": 0.5,
  "Name": "Thermal Vault",
  "TickRate": 0.1,
  "TossSpeed": 400,
  "Upgrades": [
    {
      "MeleeSpeedBonusPercentage": 30
    },
    {
      "WeaponDamagePerHeat": 1.0
    },
    {
      "BaseHeatPower": 20.0,
      "MaxHeatPower": 40.0
    }
  ],
  "WeaponDamagePerHeat": 1.0,
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
