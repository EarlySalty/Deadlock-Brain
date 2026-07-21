---
title: "Life Drain"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_life_drain"
canonical_name: "Life Drain"
snapshot_id: 39471
source_document_id: 7070
payload_hash: "27ef44b219df92606fe8ef7e24dbf2bdca15e1fdd15a0e5e9713041b9f7f48e8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.416881+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Life Drain

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_life_drain`
- Snapshot ID: `39471`
- Source-Dokument: `7070`
- Kurzinfo: Life Drain aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 18,
  "AbilityCooldown": 34.0,
  "AbilityDuration": 2.5,
  "AbilityUnitTargetLimit": 10,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_life_drain",
  "LifeDrainCasterModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "FamiliarAbilityKeepalive"
    ],
    "Subclass": "LifeDrainCaster"
  },
  "LifeDrainHealthMult": 75,
  "LifeDrainPerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.43
    },
    "Value": 32
  },
  "LifeDrainTargetModifier": {
    "Class": "LifeDrain",
    "SilenceModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "LifeDrainSilence"
    },
    "Subclass": "LifeDrain"
  },
  "MaxRange": 28,
  "MoveSpeedReduction": 40,
  "Name": "Life Drain",
  "TickRate": 0.1,
  "Upgrades": [
    {
      "LifeDrainPerSecond": 18
    },
    {
      "AbilityDuration": 2.5
    },
    {
      "AbilityCharges": 3,
      "AbilityCooldownBetweenCharge": 0.1,
      "LifeDrainPerSecond": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.4
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
    "card_name": "Life Drain",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "life drain",
      "name": "Life Drain",
      "type": "ability"
    }
  ]
}
````
