---
title: "Force Field"
entity_type: "ability"
source: "deadlock_data"
external_id: "rutger_force_field"
canonical_name: "Force Field"
snapshot_id: 39702
source_document_id: 7070
payload_hash: "bf9b6c7f867618ff1b72cf39e3485b2371ce64263e9bd2e3b66225a065623610"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.992336+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Force Field

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `rutger_force_field`
- Snapshot ID: `39702`
- Source-Dokument: `7070`
- Kurzinfo: Force Field aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "RutgerForceFieldAura",
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "RutgerForceField"
    },
    "Subclass": "ForceFieldAura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ChargeUpTime": 0.5,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 70
  },
  "EdgePushDuration": 0.15,
  "ForceFieldThinkRate": 0.05,
  "Height": 150,
  "IsDisabled": false,
  "Key": "rutger_force_field",
  "Name": "Force Field",
  "SlowDuration": 0.3,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 60,
  "SpherePushExtraDistance": 1.5,
  "SphereRadius": 5,
  "Upgrades": [
    {
      "AbilityCooldown": -14.0
    },
    {
      "Damage": 70
    },
    {
      "AbilityDuration": 3
    }
  ],
  "VictimPushModifier": {
    "Class": "RutgerForceFieldPushOut",
    "Duration": -1.0,
    "EnabledStateMask": [
      "Silenced",
      "CommandRestricted",
      "AirDuckingForced"
    ],
    "Subclass": "RutgerForceFieldPushOut"
  },
  "VictimPushTime": 0.4,
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
      "hero_key": "hero_rutger",
      "hero_name": "Rutger",
      "lookup": "force field",
      "name": "Force Field",
      "type": "ability"
    }
  ]
}
````
