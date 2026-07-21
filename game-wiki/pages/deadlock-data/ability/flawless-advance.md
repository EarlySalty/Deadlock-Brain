---
title: "Flawless Advance"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_fencer_lunge"
canonical_name: "Flawless Advance"
snapshot_id: 39432
source_document_id: 7070
payload_hash: "d193ee5ed6896bf359e591c7c5c881b6e662e2a2ddcb8b3a61281e2eee2585d2"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.310628+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flawless Advance

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_lunge`
- Snapshot ID: `39432`
- Source-Dokument: `7070`
- Kurzinfo: Flawless Advance aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 26,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "AttackDashRange": 3.0,
  "AttackingDashSpeed": 55.88,
  "BaseDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 25
  },
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DashAngleThreshold": 89,
  "DashBuffModifier": {
    "Class": "Base",
    "Subclass": "Dashmodifier"
  },
  "DashRadius": 1.85,
  "DashRange": 5.0,
  "DashSpeed": 27.94,
  "HealFixedHealth": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "HoldDurationMax": 1.1,
  "HoldDurationMin": 0.25,
  "IsDisabled": false,
  "Key": "ability_fencer_lunge",
  "MaxDamageBeforePerfect": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.9
    },
    "Value": 40
  },
  "MaxProcBleedDamagePercent": 50,
  "MaxStabs": 3,
  "MaxStacks": 2,
  "Name": "Flawless Advance",
  "ParryCooldownReduction": 5,
  "PctTravelDistanceToDamageIn": 80,
  "PerfectDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.55
    },
    "Value": 65
  },
  "PerfectHoldTimeStart": 0.525,
  "PerfectWindowDuration": 0.25,
  "RecastTime": 5,
  "SlashCollisionRadius": 4.05,
  "SlashLength": 13,
  "SlashRadius": 1.6,
  "Upgrades": [
    {
      "HealFixedHealth": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.3
        },
        "Value": 35
      }
    },
    {
      "AbilityCooldown": -12,
      "BulletResist": 60,
      "DashBuffDuration": 1.5
    },
    {
      "AttackDashRange": 3.0,
      "BaseDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 30
      },
      "DashSpeed": 13.97,
      "MaxDamageBeforePerfect": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 45
      },
      "PerfectDamage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.15
        },
        "Value": 65
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
    "card_name": "Flawless Advance",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "flawless advance",
      "name": "Flawless Advance",
      "type": "ability"
    }
  ]
}
````
