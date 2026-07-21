---
title: "Slam Fire"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_unloadgun"
canonical_name: "Slam Fire"
snapshot_id: 39585
source_document_id: 7070
payload_hash: "44dd678259df4e87e5609899f3b6ebf6d3a1f29bafb5553c866d3cb70a61f7a3"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.698474+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Slam Fire

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_unloadgun`
- Snapshot ID: `39585`
- Source-Dokument: `7070`
- Kurzinfo: Slam Fire aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.45,
  "AbilityCastRange": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 0.0254
  },
  "AbilityCooldown": 25,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AccuracyPercentage": -30,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "Channeling"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusFireRate": 300,
  "BuffModifier": {
    "Class": "WerewolfUnloadgun2",
    "MaxBulletsToProcInShot": 1.0,
    "StackingModifier": {
      "Class": "Base",
      "Subclass": "Stacking"
    },
    "Subclass": "Buff"
  },
  "BulletEffectiveness": 0.1,
  "BulletRadiusOverride": 7,
  "BulletSpread": -1,
  "ChannelMoveSpeed": 5.08,
  "CurrentHealthDamagePercentage": 2.5,
  "Damage": 40,
  "DebuffDuration": 3,
  "IsDisabled": false,
  "Key": "ability_werewolf_unloadgun",
  "LingerDuration": 0.1,
  "MaxShots": 3,
  "Name": "Slam Fire",
  "ProcChance": 100,
  "RecoilDelayFactor": 0.05,
  "RecoilRecoverySpeed": 0.1,
  "RecoilSpeed": 12,
  "RecoilStrength": 12,
  "SpreadPenaltyPerShot": 0.5,
  "Upgrades": [
    {
      "BaseAttackDamagePercent": 15
    },
    {
      "AbilityCooldown": -10
    },
    {
      "BonusCurrentHealthDamagePercentage": 7,
      "MaxStacks": 3,
      "StackDuration": 3
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
    "card_name": "Slam Fire",
    "hero_key": "hero_werewolf",
    "hero_name": "Silver",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "slam fire",
      "name": "Slam Fire",
      "type": "ability"
    }
  ]
}
````
