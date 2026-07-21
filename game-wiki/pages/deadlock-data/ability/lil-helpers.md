---
title: "Lil Helpers"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_familiar_helpinghands"
canonical_name: "Lil Helpers"
snapshot_id: 39431
source_document_id: 7070
payload_hash: "a02b23c2285da7540b33c7d6a017c9b824a86adc341e4e5f908f7c3e899d8ced"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.307450+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Lil Helpers

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_familiar_helpinghands`
- Snapshot ID: `39431`
- Source-Dokument: `7070`
- Kurzinfo: Lil Helpers aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AIAggroModifier": {
    "Class": "NeutralAggro",
    "Subclass": "Aiaggro"
  },
  "AIPhysicsModifier": {
    "Class": "FamiliarAiPhysics",
    "Subclass": "Aiphysics"
  },
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": {
    "Scale": {
      "Type": "range",
      "Value": 0.0
    },
    "Value": 45
  },
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ArmTime": 0.1,
  "AuraAttackHeight": 10,
  "AuraRadius": 10,
  "AuraSoftRadius": 10,
  "BehaviourBits": [
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSprint",
    "BehaviorAllowSelfCast",
    "BehaviorDamageDoesntWakeFromSleep",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDoNotAllowSpamProc",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusMoveSpeed": 3.0,
  "ChannelMoveSpeed": -1,
  "DPSPerSprite": 1,
  "Damage": 20,
  "HelperChoreCooldownDuration": 5,
  "HelperCount": 1,
  "HelperDowntimeDuration": 15.1,
  "HelpersPerPatrol": 4,
  "InfestBurstHealthPercent": 75,
  "InfestDamageTakenPercent": 30,
  "InfestHeal": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.14
    },
    "Value": 8
  },
  "InfestHealInterval": 2.0,
  "InfestModifier": {
    "Class": "CitadelModifierFamiliarInfested",
    "Subclass": "Infest"
  },
  "InfestWaitingModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "FamiliarInfestWaiting"
    ],
    "Subclass": "Infestwaiting"
  },
  "InvisWatcherModifier": {
    "Class": "FamiliarHelperInvisWatcher",
    "Subclass": "FamiliarHelperInvisWatcher"
  },
  "IsDisabled": false,
  "Key": "ability_familiar_helpinghands",
  "NPCInfestDuration": 50,
  "Name": "Lil Helpers",
  "PatrolDamageCooldown": 10,
  "PlayerInfestDuration": 8,
  "TechArmorGain": 12,
  "TickRate": 0.2,
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.5,
      "HelperCount": 1
    },
    {
      "HelperCount": 1,
      "InfestDamageTakenPercent": 15
    },
    {
      "HelperCount": 1,
      "TechArmorGain": 15
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
    "card_name": "Lil Helpers",
    "hero_key": "hero_familiar",
    "hero_name": "Rem",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_familiar",
      "hero_name": "Rem",
      "lookup": "lil helpers",
      "name": "Lil Helpers",
      "type": "ability"
    }
  ]
}
````
