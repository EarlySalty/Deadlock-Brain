---
title: "Ice Path"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_icepath"
canonical_name: "Ice Path"
snapshot_id: 39464
source_document_id: 7070
payload_hash: "f3cb6c5646a729dd2cefee7650d1de31f717405741e0e64336c9b581cbcd092c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.392096+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Ice Path

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_icepath`
- Snapshot ID: `39464`
- Source-Dokument: `7070`
- Kurzinfo: Ice Path aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 50.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorCastableWhileBusy",
    "BehaviorInterruptMeleeOnCast",
    "BehaviorNoTarget",
    "BehaviorPreventBotUsage",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast",
    "BehaviorRequireAbilityButtonToCancel"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "IcePathAuraDuration": 18,
  "IcePathEdgeWidth": 0.7,
  "IcePathInterval": 0.5,
  "IcePathModifier": {
    "BonusSpiritLingerModifier": {
      "Class": "IcepathTechPowerLinger",
      "Subclass": "IcepathTechPowerLinger"
    },
    "Class": "Icepath",
    "EnabledStateMask": [
      "AbilityMovement"
    ],
    "FriendlyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "IcepathFriendlyModifier",
        "Subclass": "IcepathBuff"
      },
      "Subclass": "IcepathFriendlyAura"
    },
    "Subclass": "Icepath"
  },
  "IcePathPullInStrength": 20,
  "IcePathShardRadius": 1.2,
  "IsDisabled": false,
  "Key": "ability_icepath",
  "MinHeight": 20,
  "ModifierRadius": 5,
  "MoveSpeedBonus": 2,
  "MoveWhileShootingSpeedPenaltyReductionPercent": 100,
  "MoveWhileZoomedSpeedPenaltyReductionPercent": 100,
  "Name": "Ice Path",
  "PopupForce": 30,
  "SlideScale": 50,
  "SlowResistancePercent": 60,
  "SprintSpeedBonus": 2,
  "Upgrades": [
    {
      "BulletResist": 35,
      "MoveSpeedBonus": 2
    },
    {
      "AbilityCooldown": -25.0
    },
    {
      "BonusSpirit": 20,
      "BonusSpiritPct": 35
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
    "card_name": "Ice Path",
    "hero_key": "hero_kelvin",
    "hero_name": "Kelvin",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kelvin",
      "hero_name": "Kelvin",
      "lookup": "ice path",
      "name": "Ice Path",
      "type": "ability"
    }
  ]
}
````
