---
title: "Seismic Impact"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_bull_leap"
canonical_name: "Seismic Impact"
snapshot_id: 39599
source_document_id: 7070
payload_hash: "a1259f051a0d2a3e3a6189e9be3d66cfc339fd93a42acf9f2b65339d61aa9313"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.738669+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Seismic Impact

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_leap`
- Snapshot ID: `39599`
- Source-Dokument: `7070`
- Kurzinfo: Seismic Impact aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 215.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ActiveModifier": {
    "Class": "Base",
    "Subclass": "LeapActive"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorMovement",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "BoostModifier": {
    "Class": "CitadelBullLeapBoosting",
    "Subclass": "CitadelBullLeapBoosting"
  },
  "ChannelMoveSpeed": -1,
  "CrashModifier": {
    "Class": "CitadelBullLeapBoostingCrash",
    "Subclass": "CitadelBullLeapBoostingCrash"
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.325
    },
    "Value": 100
  },
  "DragModifier": {
    "Class": "ChargeDragEnemy",
    "ForwardOffset": 200,
    "Subclass": "ChargeDragEnemy",
    "VerticalOffset": 0
  },
  "ImmunityModifier": {
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
    "Subclass": "Unstoppable"
  },
  "ImpactHeight": 6,
  "ImpactRadius": 9,
  "IsDisabled": false,
  "Key": "citadel_ability_bull_leap",
  "LandingBonusesModifier": {
    "Class": "CitadelBullLeapLandingBonuses",
    "Subclass": "CitadelBullLeapLandingBonuses"
  },
  "Name": "Seismic Impact",
  "StunDuration": 1.6,
  "TossSpeed": 450,
  "Upgrades": [
    {
      "AbilityCooldown": -30.0
    },
    {
      "StunDuration": 0.8
    },
    {
      "ImmunityDuration": 6,
      "ImpactRadius": 6
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
    "card_name": "Seismic Impact",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "seismic impact",
      "name": "Seismic Impact",
      "type": "ability"
    }
  ]
}
````
