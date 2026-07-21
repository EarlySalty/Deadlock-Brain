---
title: "Frozen Shelter"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_ice_dome"
canonical_name: "Frozen Shelter"
snapshot_id: 39460
source_document_id: 7070
payload_hash: "d998c0d8a76d8ee6d34a2ddfe1402c7e08e37c95f4d292881cf652fee462cdee"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.381578+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Frozen Shelter

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_ice_dome`
- Snapshot ID: `39460`
- Source-Dokument: `7070`
- Kurzinfo: Frozen Shelter aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 8,
  "AbilityCooldown": 195,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorDisplaysDamageImpact",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorCanSetQuickCast",
    "BehaviorAllowSelfCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlockerScaleFactor": 115,
  "BonusHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 90
  },
  "ChannelMoveSpeed": -1,
  "EnemyDragSpeed": 25.4,
  "GrowTime": 0.2,
  "IceDomeModifier": {
    "Class": "IceDome",
    "EnabledStateMask": [
      "SinclairTaxUltActive"
    ],
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
  "Key": "ability_ice_dome",
  "Name": "Frozen Shelter",
  "Radius": 10,
  "SlowPercent": 35,
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "AbilityDuration": 1.5
    },
    {
      "BonusHealthRegen": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.0
        },
        "Value": 65
      },
      "PurgeOnCast": 1
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
    "card_name": "Frozen Shelter",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "frozen shelter",
      "name": "Frozen Shelter",
      "type": "ability"
    }
  ]
}
````
