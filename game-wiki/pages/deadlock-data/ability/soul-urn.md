---
title: "Soul Urn"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_golden_idol"
canonical_name: "Soul Urn"
snapshot_id: 39447
source_document_id: 7070
payload_hash: "884ebb8e45b1461f93d9060d1cc164a5a44fd5bcbfe1c2c2ba6387db40890d32"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.347798+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Soul Urn

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_golden_idol`
- Snapshot ID: `39447`
- Source-Dokument: `7070`
- Kurzinfo: Soul Urn aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirMoveIncreasePercent": 10,
  "AutoIntrinsicModifiers": [
    {
      "Class": "IntrinsicBase",
      "Subclass": "IntrinsicBase"
    }
  ],
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNotSilencable",
    "BehaviorNoTarget"
  ],
  "BonusMoveSpeed": 3.5,
  "BonusSprintAcceleration": 14,
  "BonusSprintSpeed": 2,
  "ChannelMoveSpeed": -1,
  "DropOffTimer": 0.1,
  "DropoffTimerModifier": {
    "Class": "IdolReturnTimer",
    "EnabledStateMask": [
      "ReturningIdol"
    ],
    "Subclass": "Timer"
  },
  "FixedMoveSpeed": 15,
  "GroundMoveIncreasePercent": 10,
  "HoldingIdolModifier": {
    "Class": "CitadelHoldingGoldenIdol",
    "EnabledStateMask": [
      "HoldingIdol",
      "TeleporterDisabled",
      "ZiplineDisabled",
      "AttributeCannotBePurged",
      "ParryDisabled"
    ],
    "Subclass": "CitadelHoldingGoldenIdol"
  },
  "IsDisabled": false,
  "Key": "ability_golden_idol",
  "Lifetime": 3,
  "Name": "Soul Urn",
  "Radius": 20,
  "SlowResistancePercent": 100,
  "Stamina": 1,
  "StaminaCooldownReduction": 15,
  "TrailingTeamBonusArmorDamageResist": 35,
  "TrailingTeamBonusDebuffResist": 35,
  "TrailingTeamBonusSprintSpeed": 7,
  "TrailingTeamBonusTechResist": 35,
  "Upgrades": [],
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
