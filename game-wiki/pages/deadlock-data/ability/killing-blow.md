---
title: "Killing Blow"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_shiv_killing_blow"
canonical_name: "Killing Blow"
snapshot_id: 39642
source_document_id: 7070
payload_hash: "9a6c0b7d4ce00c167735c520275475e6cbbdba801d9676b90379674ce9a67eaf"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.845017+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Killing Blow

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_killing_blow`
- Snapshot ID: `39642`
- Source-Dokument: `7070`
- Kurzinfo: Killing Blow aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.05,
  "AbilityCastRange": 12,
  "AbilityCooldown": 145.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BonusAbilityResource": 10,
  "BuffDamage": 8,
  "CameraDistance": 400,
  "ChannelMoveSpeed": -1,
  "Damage": 200,
  "EnemyHealthPercent": 20,
  "EnemyHealthPercentBuffer": 3,
  "FailedExecuteCooldownPenalty": 30,
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_killing_blow",
  "KillableModifier": {
    "Class": "KillingBlowGlow",
    "Subclass": "KillingBlowGlow"
  },
  "LeapModifier": {
    "Class": "CitadelShivKillingblowLeap",
    "Subclass": "CitadelShivKillingblowLeap"
  },
  "MinTimeToTarget": 0.5,
  "MoveSpeedToTarget": 30,
  "Name": "Killing Blow",
  "RageDrainDelayDuration": 12,
  "RageDrainRate": 0.25,
  "RagePerHeavyMelee": 2.85384,
  "RagePerLightMelee": 1.55664,
  "RagePerSpiritDamage": 0.01452864,
  "RagePerWeaponDamage": 0.0158766,
  "RecastWindow": 20,
  "SlashRange": 90,
  "Upgrades": [
    {
      "AbilityCastRange": 6,
      "BonusMoveSpeed": 2
    },
    {
      "AbilityCooldown": -25,
      "BuffDamage": 16
    },
    {
      "EnemyHealthPercent": 8
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
    "card_name": "Killing Blow",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "killing blow",
      "name": "Killing Blow",
      "type": "ability"
    }
  ]
}
````
