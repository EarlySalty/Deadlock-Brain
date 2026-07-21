---
title: "Telekinesis"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_psychic_lift"
canonical_name: "Telekinesis"
snapshot_id: 39635
source_document_id: 7070
payload_hash: "fb8aba40700ccd9b6e3b4524fd55ab6ef720a0b0c1b9a4edd5a757028eca7a34"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.827921+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Telekinesis

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_psychic_lift`
- Snapshot ID: `39635`
- Source-Dokument: `7070`
- Kurzinfo: Telekinesis aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.45,
  "AbilityCastRange": 10,
  "AbilityChannelTime": 0.65,
  "AbilityCooldown": 150,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 100
  },
  "DampingFactor": 0.3,
  "IsDisabled": false,
  "Key": "citadel_ability_psychic_lift",
  "LiftChainRadius": 20,
  "LiftDuration": 2,
  "LiftHeight": 80,
  "LiftModifier": {
    "Class": "CitadelPsychiclift",
    "DisarmModifier": {
      "Class": "CitadelDisarmed",
      "EnabledStateMask": [
        "Disarmed"
      ],
      "Subclass": "Disarm"
    },
    "EnabledStateMask": [
      "Stunned"
    ],
    "OccilateDegreesPerSecond": 300.0,
    "OccilateMaxDistance": 400.0,
    "RiseAcc": 3000.0,
    "RiseDecayFracEnd": 0.95,
    "RiseDecayFracStart": 0.3,
    "RiseMaxSpeed": 750.0,
    "RiseTime": 0.75,
    "SilenceModifier": {
      "Class": "CitadelSilenced",
      "EnabledStateMask": [
        "Silenced"
      ],
      "Subclass": "Silence"
    },
    "SlamAcc": 6000.0,
    "SlamImpactRadius": 100.0,
    "SlamTime": 0.5,
    "SlowModifier": {
      "Class": "SlowBase",
      "EnabledStateMask": [
        "Slowed",
        "DashDisabledDebuff",
        "SilenceMovementAbilites"
      ],
      "Subclass": "LiftSlowModifier"
    },
    "Subclass": "Lift"
  },
  "Name": "Telekinesis",
  "SlowPercent": 40,
  "TossDistance": 13,
  "TossUpStrength": 220,
  "Upgrades": [
    {
      "Damage": 100
    },
    {
      "AbilityCooldown": -45
    },
    {
      "AbilityCastRange": 6,
      "AbilityDuration": 1.5,
      "TossDistance": 6
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
    "card_name": "Telekinesis",
    "hero_key": "hero_wraith",
    "hero_name": "Wraith",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wraith",
      "hero_name": "Wraith",
      "lookup": "telekinesis",
      "name": "Telekinesis",
      "type": "ability"
    }
  ]
}
````
