---
title: "Plot Armor"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_bookworm_knightbarrier"
canonical_name: "Plot Armor"
snapshot_id: 39409
source_document_id: 7070
payload_hash: "f078a5b9974788738fb0248ce3a3f8f8ac9d6cbbea8e76728c13b67d9ca30c1a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.252033+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Plot Armor

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_knightbarrier`
- Snapshot ID: `39409`
- Source-Dokument: `7070`
- Kurzinfo: Plot Armor aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 35,
  "AbilityCooldown": 28.0,
  "AbilityDuration": 5,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AttackModifier": {
    "Class": "Base",
    "Subclass": "Attackbuff"
  },
  "BarrierModifier": {
    "Class": "BookwormKnightbarrier",
    "DebuffModifier": {
      "Class": "CitadelDisarmProc",
      "EnabledStateMask": [
        "MeleeDisabled"
      ],
      "Subclass": "Disarm"
    },
    "Subclass": "Knightbarrier"
  },
  "BaseAttackDamagePercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 25
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorAllowSelfCast",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusFireRate": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "BonusSpiritDamagePercent": 15,
  "BonusTargetRadius": 30,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 125
  },
  "IsDisabled": false,
  "Key": "ability_bookworm_knightbarrier",
  "Name": "Plot Armor",
  "PushForce": 900,
  "ShoveRadius": 6,
  "Upgrades": [
    {
      "BonusFireRate": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.16
        },
        "Value": 14
      }
    },
    {
      "AbilityDuration": 2,
      "CombatBarrier": 100
    },
    {
      "BonusTargets": 2,
      "BonusTargetsBarrierPercentage": 100,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.5
        },
        "Value": 0
      }
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
    "card_name": "Plot Armor",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "plot armor",
      "name": "Plot Armor",
      "type": "ability"
    }
  ]
}
````
