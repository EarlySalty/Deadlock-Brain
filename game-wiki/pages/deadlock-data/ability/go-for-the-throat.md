---
title: "Go For The Throat"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_frenzy"
canonical_name: "Go For The Throat"
snapshot_id: 39574
source_document_id: 7070
payload_hash: "0b567247941e007b6d955441f6b95c11863eeff0d59c655373413b0dbe26045a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.677247+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Go For The Throat

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_frenzy`
- Snapshot ID: `39574`
- Source-Dokument: `7070`
- Kurzinfo: Go For The Throat aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 7.5,
  "AbilityCooldown": 6.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.5,
  "AbilityUnitTargetLimit": 16,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorUseLagCompensationForUnitTargeting"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.5
    },
    "Value": 0.0
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_frenzy",
  "LifeStealPercentOnHit": 40,
  "MissingHealthDamagePercentage": 6,
  "Name": "Go For The Throat",
  "TargetModifier": {
    "Class": "DrifterRendBulletLifesteal",
    "Subclass": "DrifterRendBulletLifesteal"
  },
  "TargetingConeAngle": 40,
  "Upgrades": [
    {
      "Damage": 30
    },
    {
      "LifeStealPercentOnHit": 25
    },
    {
      "MissingHealthDamagePercentage": 4
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
    "card_name": "Go For The Throat",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "go for the throat",
      "name": "Go For The Throat",
      "type": "ability"
    }
  ]
}
````
