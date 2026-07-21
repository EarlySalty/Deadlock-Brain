---
title: "Captive Audience"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_magician_stage"
canonical_name: "Captive Audience"
snapshot_id: 39482
source_document_id: 7070
payload_hash: "25e588109b2b71a734cb4eb143d9792e201cf049c6b07c9c8524cf418a15f640"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.446159+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Captive Audience

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_magician_stage`
- Snapshot ID: `39482`
- Source-Dokument: `7070`
- Kurzinfo: Captive Audience aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 20,
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowSelfCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BlockerScaleFactor": 115,
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 120
  },
  "ChannelMoveSpeed": -1,
  "EnemyDragSpeed": 25.4,
  "GrowTime": 0.2,
  "IceDomeModifier": {
    "Class": "IceDome",
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IcedomeAuramodifierBase",
        "Duration": 0.5,
        "EnabledStateMask": [
          "Slowed"
        ],
        "Subclass": "Debuff"
      },
      "Subclass": "IceDomeEnemyAura"
    },
    "FriendlyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IceDomeFriendly",
        "Duration": 0.5,
        "Subclass": "IceDomeFriendly"
      },
      "Subclass": "IceDomeFriendlyAura"
    },
    "Subclass": "IceDome"
  },
  "IsDisabled": false,
  "Key": "ability_magician_stage",
  "Name": "Captive Audience",
  "Radius": 15,
  "SlowPercent": 35,
  "Upgrades": [
    {
      "FireRateSlow": 40
    },
    {
      "AbilityCooldown": -38.0
    },
    {
      "BonusHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.52334
        },
        "Value": 70
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
  }
}
````
