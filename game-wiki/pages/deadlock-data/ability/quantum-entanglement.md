---
title: "Quantum Entanglement"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_void_sphere"
canonical_name: "Quantum Entanglement"
snapshot_id: 39664
source_document_id: 7070
payload_hash: "00828e47d728a47184088500fb3977e8f84db6696bb95cbb821bacd1cfd52854"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.895561+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Quantum Entanglement

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_void_sphere`
- Snapshot ID: `39664`
- Source-Dokument: `7070`
- Kurzinfo: Quantum Entanglement aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 10,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.4,
  "AbilityUnitTargetLimit": 1,
  "AllyDistance": 13,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BubbleModifier": {
    "BuffModifier": {
      "Class": "VoidsphereBuff",
      "Subclass": "VoidsphereBuff"
    },
    "Class": "VoidSphere",
    "StatusEffectPriority": 100,
    "Subclass": "VoidSphere"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_void_sphere",
  "Name": "Quantum Entanglement",
  "StaminaRestore": 1,
  "TrailInterval": 0.01,
  "Upgrades": [
    {
      "AbilityCastRange": 6
    },
    {
      "AbilityCooldown": -6
    },
    {
      "ChargeReplenish": 1,
      "ReduceDebuffs": 50
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
    "card_name": "Quantum Entanglement",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "quantum entanglement",
      "name": "Quantum Entanglement",
      "type": "ability"
    }
  ]
}
````
