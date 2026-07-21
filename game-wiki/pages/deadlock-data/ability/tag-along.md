---
title: "Tag Along"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_familiar_attach"
canonical_name: "Tag Along"
snapshot_id: 39429
source_document_id: 7070
payload_hash: "37bcc630cad3d37293ae2b79c425f015c477b9ff0c6128aa0918e31321bc6f4b"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.302468+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Tag Along

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_attach`
- Snapshot ID: `39429`
- Source-Dokument: `7070`
- Kurzinfo: Tag Along aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 23,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "AllyLockoutModifier": {
    "Class": "Base",
    "Subclass": "Tagalonglockout"
  },
  "AttachHealModifier": {
    "Class": "FamiliarAttachHeal",
    "Subclass": "Attachheal"
  },
  "AttachedModifier": {
    "AttachEndingModifier": {
      "Class": "Base",
      "Subclass": "Attachending"
    },
    "Class": "FamiliarAttached",
    "EndingWarningDuration": 5.0,
    "HostModifier": {
      "Class": "FamiliarAttachhost",
      "Subclass": "Host"
    },
    "ReplicatedBarrierModifier": {
      "Class": "CitadelFamiliarReplicatedbarrier",
      "Subclass": "Familiarbarrier"
    },
    "Subclass": "Attached"
  },
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "Castdelay"
  },
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDoNotAllowSpamProc"
  ],
  "BonusMoveSpeed": 4,
  "CameraDummyModifier": {
    "Class": "FamiliarCamdummy",
    "Subclass": "Cameradummy"
  },
  "ChannelMoveSpeed": 5,
  "DeathBarrierModifier": {
    "Class": "CitadelFamiliarReplicatedbarrier",
    "Subclass": "Deathbarrier"
  },
  "HealingPerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 42
  },
  "HealthRegenDuration": 2,
  "HopOffBuffModifier": {
    "Class": "Base",
    "Subclass": "Hopoutbuff"
  },
  "HopOutLockoutDuration": 0.3,
  "HopOutLockoutModifier": {
    "Class": "FamiliarHopoutlockout",
    "Subclass": "Lockout"
  },
  "IsDisabled": false,
  "Key": "ability_familiar_attach",
  "LaunchTossModifier": {
    "Class": "FamiliarAttachLaunchoff",
    "Subclass": "Launchtoss"
  },
  "LaunchedSelfModifier": {
    "Class": "CitadelModifierFamiliarSpeedlines",
    "Subclass": "Launch"
  },
  "MissingHealthBurstPct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.03
    },
    "Value": 15
  },
  "MovingToAttachModifier": {
    "Class": "FamiliarMovingtoattach",
    "Subclass": "Movingtoattach"
  },
  "Name": "Tag Along",
  "SpeedModifier": {
    "Class": "Base",
    "Subclass": "Speed"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "BonusBarrierAmpPercent": 35,
      "BonusItemDurationPercent": 35,
      "BonusItemRangePercent": 35
    },
    {
      "BonusSpiritPower": 35,
      "HealingPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.34
        },
        "Value": 0
      },
      "HopOffEffecDuration": 10,
      "MissingHealthBurstPct": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.016
        },
        "Value": 0
      },
      "TechPowerPercent": 15
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
    "card_name": "Tag Along",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "tag along",
      "name": "Tag Along",
      "type": "ability"
    }
  ]
}
````
