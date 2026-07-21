---
title: "Hex-Lined Snap Trap"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_beartrap"
canonical_name: "Hex-Lined Snap Trap"
snapshot_id: 39511
source_document_id: 7070
payload_hash: "b7b088b0bd7d3b9888eb1a493d13c5b71d0d66742bab0d74338786cf115a13c4"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.512987+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hex-Lined Snap Trap

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_beartrap`
- Snapshot ID: `39511`
- Source-Dokument: `7070`
- Kurzinfo: Hex-Lined Snap Trap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCharges": 2,
  "AbilityCooldown": 28,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 0.5,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventTrainingBotUsage",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.2
    },
    "Value": 80
  },
  "DebuffModifier": {
    "Class": "PriestBeartrapDebuff",
    "EnabledStateMask": [
      "VisibleToEnemy",
      "GlowThroughWallsToEnemy"
    ],
    "Subclass": "Debuff"
  },
  "ImmobilizeDuration": 1.25,
  "ImmobilizeModifier": {
    "Class": "PriestImmobilize",
    "Subclass": "Immobilize"
  },
  "IsDisabled": false,
  "Key": "ability_priest_beartrap",
  "Lifetime": 30,
  "Name": "Hex-Lined Snap Trap",
  "Radius": 2,
  "RevealDuration": 6,
  "TetherDuration": 0.6,
  "TetherModifier": {
    "Class": "PriestTether",
    "Subclass": "Tether"
  },
  "TetherRadius": 0.3,
  "TickRate": 0.1,
  "TrapHeight": 2,
  "TripUpSpeed": 6.35,
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1.0
    },
    {
      "AbilityCharges": 1,
      "IncomingDamagePercentFromCaster": 30
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
    "card_name": "Hex-Lined Snap Trap",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "hex-lined snap trap",
      "name": "Hex-Lined Snap Trap",
      "type": "ability"
    }
  ]
}
````
