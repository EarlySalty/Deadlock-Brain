---
title: "Queen of Shadows"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_nano_shadow_step"
canonical_name: "Queen of Shadows"
snapshot_id: 39493
source_document_id: 7070
payload_hash: "7a2a7018fd218c65f0b074f9d42ab8cf10483ccc39f82b5452f9288671e893d8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.472784+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Queen of Shadows

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_shadow_step`
- Snapshot ID: `39493`
- Source-Dokument: `7070`
- Kurzinfo: Queen of Shadows aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 110.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0465
    },
    "Value": 12
  },
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "BulletArmorReductionDuration": 6,
  "BulletArmorReductionHeavy": 15,
  "BulletArmorReductionLight": 5,
  "ChannelMoveSpeed": -1,
  "DamageAmplification": 20,
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 0.25,
  "InvisMoveSpeedMod": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0372
    },
    "Value": 2.0
  },
  "IsDisabled": false,
  "Key": "ability_nano_shadow_step",
  "MeleeAttackSpeedBonus": 20,
  "Name": "Queen of Shadows",
  "PurgeModifier": {
    "Class": "Base",
    "Subclass": "Purge"
  },
  "RevealOnDamageDuration": 0.7,
  "RevealOnSpottedDuration": 0.7,
  "ShadowModifier": {
    "Class": "NanoShadowStep",
    "DesatFactor": 0.6,
    "MaxCloak": 0.99,
    "MinCloak": 0.7,
    "SilenceModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "SlowBase"
    },
    "Subclass": "NanoShadowStep"
  },
  "SlowPercent": 30,
  "SpottedRadius": 15,
  "Upgrades": [
    {
      "StaminaCooldownReduction": 30
    },
    {
      "PurgeOnActivate": 1,
      "SlowResistancePercent": 40
    },
    {
      "DamageAmplification": 20,
      "SilenceOnHeavyDuration": 3
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
  }
}
````
