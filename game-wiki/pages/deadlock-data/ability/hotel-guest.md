---
title: "Hotel Guest"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_doorman_hotel"
canonical_name: "Hotel Guest"
snapshot_id: 39422
source_document_id: 7070
payload_hash: "8a559577238853e9bbffab8ca11866211d578bf6ad9b6146b5871ea2328f6cfd"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.280402+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hotel Guest

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_hotel`
- Snapshot ID: `39422`
- Source-Dokument: `7070`
- Kurzinfo: Hotel Guest aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 7,
  "AbilityChannelTime": 1,
  "AbilityCooldown": 140,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityPostCastDuration": 0.7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.0
    },
    "Value": 75
  },
  "DamageModifier": {
    "Class": "Base",
    "Subclass": "DamageTimer"
  },
  "FreezeModifier": {
    "Class": "DoormanHotelTransitionFreeze",
    "Subclass": "Freeze"
  },
  "HotelModifier": {
    "Class": "DoormanHotelVictim",
    "EnabledStateMask": [
      "InvalidTeleportTarget",
      "IgnoreOutOfPlayAreaCheck"
    ],
    "Subclass": "Hotel"
  },
  "HotelTimeScale": null,
  "ImposterModifier": {
    "Class": "DoormanHotelImposter",
    "ImposterModifierFX": {
      "Class": "DoormanHotelImposterFx",
      "Subclass": "Fx"
    },
    "Subclass": "Imposter"
  },
  "IsDisabled": false,
  "Key": "ability_doorman_hotel",
  "LateCheckoutDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 125
  },
  "Name": "Hotel Guest",
  "NoDrawModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "OutOfGame",
      "IgnoredByNpcTargeting",
      "UnitStatusHidden",
      "DoNotDrawModel"
    ],
    "Subclass": "Nodraw"
  },
  "PreTeleportModifier": {
    "Class": "Base",
    "Subclass": "PreTeleport"
  },
  "TeleportFXModifier": {
    "Class": "DoormanHotelTeleportFx",
    "Subclass": "TeleportFx"
  },
  "TimeSlowDuration": 1.0,
  "TimeSlowPercentage": 100,
  "TimeslowModifier": {
    "Class": "DoormanDiminishingTimestop",
    "Subclass": "DoormanExitTimeslow"
  },
  "UnstoppableWhileChannelingModifier": {
    "Class": "Unstoppable",
    "DisabledStateMask": [
      "Disarmed",
      "Muted",
      "Silenced",
      "SilenceMovementAbilites",
      "Slowed",
      "Glitched",
      "MeleeDisabledDebuff",
      "DashDisabledDebuff"
    ],
    "EnabledStateMask": [
      "StatusImmune",
      "SlowImmune",
      "KnockdownImmune",
      "Unstoppable"
    ],
    "StatusEffectPriority": 25,
    "Subclass": "DoormanHotelUnstoppable"
  },
  "Upgrades": [
    {
      "AbilityCooldown": -20,
      "StaminaDrain": 1
    },
    {
      "Damage": 150,
      "LateCheckoutDamage": 150,
      "LateCheckoutStun": 1.5
    },
    {
      "LateCheckoutCooldown": 13,
      "UnstoppableWhileHotelOccupied": 1
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
    "card_name": "Hotel Guest",
    "hero_key": "hero_doorman",
    "hero_name": "The Doorman",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "hotel guest",
      "name": "Hotel Guest",
      "type": "ability"
    }
  ]
}
````
