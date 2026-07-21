---
title: "Grasping Hands"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_zombiewall"
canonical_name: "Grasping Hands"
snapshot_id: 39503
source_document_id: 7070
payload_hash: "269ba003b65f8c9413b31f58d5b37c6e710543dd45e32b5ce765dc2120228360"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.494352+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Grasping Hands

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_zombiewall`
- Snapshot ID: `39503`
- Source-Dokument: `7070`
- Kurzinfo: Grasping Hands aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 24,
  "AbilityCooldown": 34,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AuraRadius": 0.75,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowAltCast",
    "BehaviorCanSetQuickCast",
    "BehaviorSuppressAltCastOnceSelected"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 90
  },
  "DebuffDuration": 0.5,
  "GroundAuraModifier": {
    "Class": "ZombiewallGroundAura",
    "ProvidedByAura": {
      "Class": "NecroZombiewallDebuff",
      "Subclass": "Debuff"
    },
    "Subclass": "Zombiewall"
  },
  "GroundAuraPopDelay": 1.1,
  "GroundAuraPriorityModifier": {
    "Class": "ZombiewallGroundAura",
    "ProvidedByAura": {
      "Class": "NecroZombiewallDebuff",
      "Subclass": "Debuff"
    },
    "Subclass": "Zombiewall"
  },
  "GroundAuraSpacing": 1,
  "ImmobilizeDuration": 1.25,
  "ImmobilizeModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Immobilize"
  },
  "IsDisabled": false,
  "Key": "ability_necro_zombiewall",
  "Name": "Grasping Hands",
  "SlowPercent": 40,
  "SummonCount": 1,
  "TetherDuration": 1,
  "TetherModifier": {
    "Class": "NecroZombiewallTether",
    "Subclass": "Tether"
  },
  "TetherRadius": 0.1,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "AbilityDuration": 2
    },
    {
      "Damage": 90,
      "ZombieWallLength": 10
    },
    {
      "AbilityCooldown": -10,
      "ImmobilizeDuration": 0.75,
      "SummonCount": 1
    }
  ],
  "ZombieWallDeployTime": 0.6,
  "ZombieWallHeight": 2.5,
  "ZombieWallLength": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.05
    },
    "Value": 14
  },
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
    "card_name": "Grasping Hands",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "grasping hands",
      "name": "Grasping Hands",
      "type": "ability"
    }
  ]
}
````
