---
title: "Naptime"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_familiar_ability01"
canonical_name: "Naptime"
snapshot_id: 39427
source_document_id: 7070
payload_hash: "6bd7cf5ef5bdf8f1dd92678531bceabbe541b93d1e9af0228d6f1e68ceb56023"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.295634+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Naptime

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_ability01`
- Snapshot ID: `39427`
- Source-Dokument: `7070`
- Kurzinfo: Naptime aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.18,
  "AbilityCastRange": 24,
  "AbilityChannelTime": {
    "Scale": {
      "Type": "duration",
      "Value": 0.0
    },
    "Value": 1.9
  },
  "AbilityCooldown": 200.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "Channeling"
  },
  "AwakeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 120
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorRefundHalfCooldownOnChannelInterrupt"
  ],
  "ChannelMoveSpeed": -1,
  "DamageResistPctWhileChanneling": 30,
  "EffectModifier": {
    "Class": "CitadelModifierFamiliarAsleep",
    "Subclass": "Effect"
  },
  "Height": 20,
  "IsDisabled": false,
  "Key": "ability_familiar_ability01",
  "MinSleepTime": 0.5,
  "MoveSpeedAndDashSlowPct": 25,
  "Name": "Naptime",
  "Radius": 19.0,
  "SleepDamageThreshold": {
    "Scale": {
      "Type": "power_increase",
      "Value": 3.1
    },
    "Value": 100
  },
  "SleepDuration": 4.0,
  "SleepMoveSpeed": 1.5,
  "StaringModifier": {
    "Class": "CitadelModifierFamiliarStaring",
    "Subclass": "Staring"
  },
  "UnstoppableWhileChannelingModifier": {
    "Class": "Unstoppable",
    "DisabledStateMask": [
      "Disarmed",
      "Muted",
      "Silenced",
      "SilenceMovementAbilites",
      "Slowed",
      "Glitched",
      "MeleeDisabledDebuff",
      "DashDisabledDebuff"
    ],
    "EnabledStateMask": [
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "StatusEffectPriority": 25,
    "Subclass": "FamiliarUltUnstoppable"
  },
  "Upgrades": [
    {
      "ConsumeStaminaOnWake": 1,
      "NoStaminaRegenDuringSleep": 1
    },
    {
      "Radius": 3,
      "SleepDuration": 0.75
    },
    {
      "AbilityCooldown": -55,
      "DamageResistPctWhileChanneling": 50,
      "UnstoppableWhileChanneling": 1
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
    "card_name": "Naptime",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "naptime",
      "name": "Naptime",
      "type": "ability"
    }
  ]
}
````
