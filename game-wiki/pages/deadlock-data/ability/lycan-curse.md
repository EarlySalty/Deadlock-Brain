---
title: "Lycan Curse"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_transformation"
canonical_name: "Lycan Curse"
snapshot_id: 39583
source_document_id: 7070
payload_hash: "7687a1cf6cf774125fad24564e0fe6b168a3e8d8e64a8489e0b5185e7c6e60e4"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.694571+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Lycan Curse

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_transformation`
- Snapshot ID: `39583`
- Source-Dokument: `7070`
- Kurzinfo: Lycan Curse aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityChargesConditionally": 1,
  "AbilityCooldown": 80,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 15,
  "AbilityPostCastDuration": 0.5,
  "AbilityUnitTargetLimit": 1,
  "AutoActivateHealthThreshold": 20,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusDurationOnBullet": 0.15,
  "BonusDurationOnHeavyMelee": 1.5,
  "BonusDurationOnLightMelee": 0.5,
  "BonusDurationPerHealthPercentLost": 0.1,
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.45
    },
    "Value": 60
  },
  "BonusHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 15.0
    },
    "Value": 125
  },
  "BonusMoveSpeed": 1.5,
  "CameraTurnRateMax": 188,
  "ChannelMoveSpeed": 7.62,
  "EndingWarningSoundDuration": 3.0,
  "HeadshotResist": -20,
  "HealAmount": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.0
    },
    "Value": 0
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_transformation",
  "KillCreditModifier": {
    "Class": "Base",
    "Subclass": "Killcredit"
  },
  "LowHealthFraction": 30,
  "LowHealthRageBonus": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.8
    },
    "Value": 40
  },
  "MaxRage": {
    "Scale": {
      "Type": "power_increase",
      "Value": 9.4
    },
    "Value": 100
  },
  "MaxStacks": 15,
  "MissingHealthPercentHeal": 30,
  "Name": "Lycan Curse",
  "RagePerDamage": 0.255,
  "RagePercentagePerSecondInCombat": 1,
  "RagePercentagePerSecondOutOfCombat": -3,
  "ReadyDuration": 3,
  "ReadyModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "WerewolfReady"
    ],
    "Subclass": "Ready"
  },
  "StackDuration": 5,
  "Upgrades": [
    {
      "BulletResist": 20,
      "TechResist": 20
    },
    {
      "BonusHealth": 200,
      "BonusMoveSpeed": 4
    },
    {
      "KillCreditWindow": 1.5,
      "KillDurationBonus": 15
    }
  ],
  "WerewolfModifier": {
    "Class": "Werewolf",
    "ModelScale": 1.0,
    "StackingBuffModifier": {
      "Class": "WerewolfStackingbuff",
      "Subclass": "Stackingbuff"
    },
    "StatusEffectPriority": 10,
    "Subclass": "Werewolf"
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
    "card_name": "Lycan Curse",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "lycan curse",
      "name": "Lycan Curse",
      "type": "ability"
    }
  ]
}
````
