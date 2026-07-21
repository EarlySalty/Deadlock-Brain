---
title: "Lurker's Ambush"
entity_type: "ability"
source: "deadlock_data"
external_id: "fathom_lurkers_ambush"
canonical_name: "Lurker's Ambush"
snapshot_id: 39677
source_document_id: 7070
payload_hash: "b1b7c94c41b77937c1e939bb41c33404b21f609b3db6edae0b0fbec8bb876dad"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.926298+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Lurker's Ambush

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_lurkers_ambush`
- Snapshot ID: `39677`
- Source-Dokument: `7070`
- Kurzinfo: Lurker's Ambush aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 30,
  "AbilityChannelTime": 9999,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCannotCancelDuringChannel"
  ],
  "ChannelMoveSpeed": 5,
  "ChannelTimeForMaxDebuff": 1.5,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 60
  },
  "DebuffMaxDuration": 3.5,
  "DebuffMinDuration": 1.0,
  "DebuffModifier": {
    "Class": "FathomLurkersAmbushDebuff",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "FathomLurkersAmbushDebuff"
  },
  "EnemySlowPct": 60,
  "InitialHeight": 350,
  "InvisFadeToDuration": 1.5,
  "InvisModifier": {
    "Class": "LurkersAmbushInvis",
    "InvisBias": 0.8,
    "Subclass": "Invis"
  },
  "IsDisabled": false,
  "Key": "fathom_lurkers_ambush",
  "Name": "Lurker's Ambush",
  "NonLatchedDurationPct": 50,
  "NotSeenByEnemiesRegen": 3,
  "RegenModifier": {
    "Class": "Base",
    "Subclass": "Regen"
  },
  "RevealOnDamageDuration": 0.5,
  "RevealOnSpottedDuration": 3,
  "SpottedRadius": 999,
  "StandStillMinTime": 0.5,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -15
    },
    {
      "DebuffMaxDuration": 1
    },
    {
      "NotSeenByEnemiesRegen": 2
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
    "card_name": "Lurker's Ambush",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "lurker's ambush",
      "name": "Lurker's Ambush",
      "type": "ability"
    }
  ]
}
````
