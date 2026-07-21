---
title: "Sanguine Retreat"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_vampirebat_batblink"
canonical_name: "Sanguine Retreat"
snapshot_id: 39555
source_document_id: 7070
payload_hash: "85ff2bcbe229c0aec74a4406d3490cc606e2d5d178fbfbb7a47963ad480676fc"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.625485+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Sanguine Retreat

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batblink`
- Snapshot ID: `39555`
- Source-Dokument: `7070`
- Kurzinfo: Sanguine Retreat aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.02
    },
    "Value": 9
  },
  "AbilityCooldown": 32,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.65,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorPreventBotUsage",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorAllowAltCast",
    "BehaviorMovement",
    "BehaviorCanSetQuickCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "EndJumpVelocity": 200,
  "ExitVelocity": 5,
  "IsDisabled": false,
  "Key": "ability_vampirebat_batblink",
  "MaxRecasts": 1,
  "Name": "Sanguine Retreat",
  "RecastWindow": 3.5,
  "SelfBuffModifier": {
    "Class": "Base",
    "StatusEffectPriority": 101,
    "Subclass": "Selfbuff"
  },
  "Upgrades": [
    {
      "BonusBullets": 8,
      "BonusFireRate": 25,
      "BuffDuration": 8
    },
    {
      "AbilityCooldown": -10
    },
    {
      "AbilityCastRange": 3,
      "MaxRecasts": 1
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
    "card_name": "Sanguine Retreat",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "sanguine retreat",
      "name": "Sanguine Retreat",
      "type": "ability"
    }
  ]
}
````
