---
title: "Medicinal Specter"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_mobile_resupply"
canonical_name: "Medicinal Specter"
snapshot_id: 39630
source_document_id: 7070
payload_hash: "7c9713d6d4a85d8de86c8675c0ffe448431abc7aba569e567e9f5c333d4841ca"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.815209+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Medicinal Specter

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_mobile_resupply`
- Snapshot ID: `39630`
- Source-Dokument: `7070`
- Kurzinfo: Medicinal Specter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 15,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "AmbientParticleRadiusControlPoint": 1,
    "Class": "MobileResupplyAura",
    "ModifierProvidedByAuraDuration": 1.0,
    "ProvidedByAura": {
      "Class": "MobileResupply",
      "Subclass": "MobileResupply"
    },
    "Subclass": "MobileResupplyAura"
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanHealPlayers",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "ExternalBonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 25
  },
  "HealInterval": 0.1,
  "HealRadius": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_mobile_resupply",
  "Name": "Medicinal Specter",
  "TurretHealMult": 1.0,
  "Upgrades": [
    {
      "SpiritResist": 40
    },
    {
      "AbilityCooldown": -20.0,
      "StaminaCooldownReduction": 100.0
    },
    {
      "AbilityDuration": 1.5,
      "HealRadius": 3,
      "MaxHealthRegenPct": 2.0
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
    "card_name": "Medicinal Specter",
    "hero_key": "hero_forge",
    "hero_name": "McGinnis",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_forge",
      "hero_name": "McGinnis",
      "lookup": "medicinal specter",
      "name": "Medicinal Specter",
      "type": "ability"
    }
  ]
}
````
