---
title: "Concussive Combustion"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_fire_bomb"
canonical_name: "Concussive Combustion"
snapshot_id: 39437
source_document_id: 7070
payload_hash: "342d3e04c015e8ecc5f8823463a1dcdb7c3285b0341076eff8d9abc918cbfcda"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.322591+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Concussive Combustion

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fire_bomb`
- Snapshot ID: `39437`
- Source-Dokument: `7070`
- Kurzinfo: Concussive Combustion aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 190.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorCastableWhileBusy",
    "BehaviorDisplaysDamageImpact"
  ],
  "BuffModifier": {
    "Class": "FirebombBuff",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 125
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "ExplodeDelay": 3.25,
  "FireBombModifier": {
    "Class": "Firebomb",
    "StatusEffectPriority": 45,
    "Subclass": "Firebomb"
  },
  "IsDisabled": false,
  "Key": "ability_fire_bomb",
  "Name": "Concussive Combustion",
  "ProgressBarModifier": {
    "Class": "Base",
    "Subclass": "Progressbar"
  },
  "Radius": 12,
  "StunDuration": 1.25,
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -65.0,
      "LifeStealPercentOnHit": 100
    },
    {
      "Radius": 10,
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
  },
  "_deadlock_data_card": {
    "card_name": "Concussive Combustion",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "concussive combustion",
      "name": "Concussive Combustion",
      "type": "ability"
    }
  ]
}
````
