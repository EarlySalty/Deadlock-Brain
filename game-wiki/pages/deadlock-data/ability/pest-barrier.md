---
title: "Pest Barrier"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_trapper_spidershield"
canonical_name: "Pest Barrier"
snapshot_id: 39546
source_document_id: 7070
payload_hash: "390b02746536386d4732f2df70fcdcbd2e69c563e982649a616bc77311d78886"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.599933+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Pest Barrier

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_spidershield`
- Snapshot ID: `39546`
- Source-Dokument: `7070`
- Kurzinfo: Pest Barrier aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 45,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorEqualUnitTargetPriority",
    "BehaviorAllowSelfCast",
    "BehaviorCanHealPlayers",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BuffModifier": {
    "Class": "TrapperSpidershield",
    "PulseDebuffModifier": {
      "Class": "Base",
      "Subclass": "TrapperSpidershieldPulsedebuff"
    },
    "StatusEffectPriority": 100,
    "Subclass": "TrapperSpidershield"
  },
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.046
    },
    "Value": 200
  },
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 30
  },
  "DebuffDuration": 0.5,
  "IsDisabled": false,
  "Key": "ability_trapper_spidershield",
  "Name": "Pest Barrier",
  "Radius": 5,
  "SlowPercent": 30,
  "TickRate": 1,
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "CombatBarrier": 200
    },
    {
      "Radius": 5
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
    "card_name": "Pest Barrier",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "pest barrier",
      "name": "Pest Barrier",
      "type": "ability"
    }
  ]
}
````
