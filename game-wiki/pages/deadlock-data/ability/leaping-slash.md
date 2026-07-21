---
title: "Leaping Slash"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_nano_dash"
canonical_name: "Leaping Slash"
snapshot_id: 39488
source_document_id: 7070
payload_hash: "764e9f918e71526d5c777a45177d99902c4538a3e7b23702161e89617700042b"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.460616+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Leaping Slash

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_dash`
- Snapshot ID: `39488`
- Source-Dokument: `7070`
- Kurzinfo: Leaping Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 9,
  "AbilityCooldown": 13,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BountyModifier": {
    "Class": "NanoBounty",
    "Subclass": "Bounty"
  },
  "CameraDistance": 550,
  "ChannelMoveSpeed": -1,
  "DashAngleThreshold": 89,
  "DashModifier": {
    "Class": "CitadelShivDash",
    "Subclass": "CitadelShivDash"
  },
  "DashRadius": 2.0,
  "DashSpeed": 60.96,
  "HealAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.4
    },
    "Value": 40
  },
  "ImpactDamage": {
    "Scale": {
      "Type": "melee",
      "Value": 0.8
    },
    "Value": 10.0
  },
  "IsDisabled": false,
  "Key": "ability_nano_dash",
  "MoveSpeedPenaltyMaxSpeed": 200,
  "Name": "Leaping Slash",
  "PostDashMaintainedVelocityRatio": 0.15,
  "SideMoveSpeedReduction": -90,
  "SlashForwardOffset": 1.5,
  "SlashHeight": 2.5,
  "SlashRadius": 4,
  "Upgrades": [
    {
      "HealAmount": 25
    },
    {
      "BonusGoldOnKill": 200,
      "BountyDuration": 3
    },
    {
      "CooldownRefundPercent": 50,
      "ImpactDamage": 60
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
    "card_name": "Leaping Slash",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "leaping slash",
      "name": "Leaping Slash",
      "type": "ability"
    }
  ]
}
````
