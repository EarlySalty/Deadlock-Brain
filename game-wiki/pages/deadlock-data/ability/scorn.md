---
title: "Scorn"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_intimidate"
canonical_name: "Scorn"
snapshot_id: 39467
source_document_id: 7070
payload_hash: "05f682adeb03c013b277ce86416bb5041e5148e60b4ebda74319cfd0c23216b0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.400759+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Scorn

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_intimidate`
- Snapshot ID: `39467`
- Source-Dokument: `7070`
- Kurzinfo: Scorn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 13,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.75
    },
    "Value": 50
  },
  "DamageHealMult": 1.2,
  "DamageHealMultNonHero": 0.35,
  "DebuffModifier": {
    "Class": "IntimidateDebuff",
    "Subclass": "IntimidateDebuff"
  },
  "EnemyModifier": {
    "Class": "Intimidated",
    "Subclass": "Intimidated"
  },
  "IsDisabled": false,
  "Key": "ability_intimidate",
  "Name": "Scorn",
  "Radius": 9,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "Damage": 35
    },
    {
      "AbilityCooldown": -5,
      "Radius": 1
    },
    {
      "DamageBonus": 15,
      "DebuffDuration": 16
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
    "card_name": "Scorn",
    "hero_key": "hero_krill",
    "hero_name": "Mo & Krill",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_krill",
      "hero_name": "Mo & Krill",
      "lookup": "scorn",
      "name": "Scorn",
      "type": "ability"
    }
  ]
}
````
