---
title: "Mauling Leap"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_maulingleap"
canonical_name: "Mauling Leap"
snapshot_id: 39578
source_document_id: 7070
payload_hash: "e980c13472c1c5fd1fe2e03c3aeb699a1fb48ad21117bfefd03192d61e491acc"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.685184+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Mauling Leap

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_maulingleap`
- Snapshot ID: `39578`
- Source-Dokument: `7070`
- Kurzinfo: Mauling Leap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": 18.1,
  "AbilityChannelTime": 0.55,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.1,
  "AbilityUnitTargetLimit": 1,
  "AllowRamMultiple": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanCancelDuringCastDelay",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BulletArmorReduction": -8,
  "CameraTurnRateMax": 188,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.15
    },
    "Value": 15
  },
  "Damage": {
    "Scale": {
      "Type": "melee",
      "Value": 1.5
    },
    "Value": 0
  },
  "DebuffDuration": 6,
  "DebuffModifier": {
    "Class": "CitadelWerewolfMaulingleapdebuff",
    "Subclass": "Shred"
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_maulingleap",
  "LeapForwardOffset": 0.3,
  "LeapMultiHitRadius": 1.5,
  "LeapRadius": 2.8,
  "LeapingModifier": {
    "Class": "CitadelWerewolfLeaping",
    "Subclass": "Leaping"
  },
  "Name": "Mauling Leap",
  "TickRate": 0.5,
  "Upgrades": [
    {
      "DPS": 10
    },
    {
      "AbilityCooldown": -9
    },
    {
      "BulletArmorReduction": -12
    }
  ],
  "WorldImpactRadius": 25,
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
    "card_name": "Mauling Leap",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "mauling leap",
      "name": "Mauling Leap",
      "type": "ability"
    }
  ]
}
````
