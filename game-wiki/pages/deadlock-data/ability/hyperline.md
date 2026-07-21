---
title: "Hyperline"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_zip_line"
canonical_name: "Hyperline"
snapshot_id: 39668
source_document_id: 7070
payload_hash: "ce2e46d31af866992afe161ce155d00bab59df8984e21efe079f0a7b8a5103d8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.905506+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Hyperline

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_zip_line`
- Snapshot ID: `39668`
- Source-Dokument: `7070`
- Kurzinfo: Hyperline aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontBreakInvisibility",
    "BehaviorDontInterruptSprint",
    "BehaviorNotSilencable",
    "BehaviorNoTarget",
    "BehaviorNonCombat",
    "BehaviorCastableWhileHidden"
  ],
  "ChannelMoveSpeed": -1,
  "DamageCooldown": 3,
  "DismountHorizontalMaxSpeedPercent": 85,
  "DismountHorizontalMinSpeedPercent": 40,
  "DismountVerticalSpeed": 300,
  "IsDisabled": false,
  "Key": "citadel_ability_zip_line",
  "KnockedOffDamagePct": 15,
  "KnockedOffSlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "KnockedOffZiplineSlow"
  },
  "LatchEndSpeed": 750,
  "LatchInitialSpeed": 600,
  "LatchMaxTime": 0.5,
  "LatchSpeed": 1500,
  "LatchVisualSnapProgress": 0.85,
  "MaxMountDistance2D": 15,
  "Name": "Hyperline",
  "PlayerSpeedCheckScale": 0.55,
  "RegenZoneDismountHorizontalMaxSpeed": 300,
  "RidingZipLineModifier": {
    "Class": "Base",
    "Subclass": "RidingZipline"
  },
  "SlowDuration": 8,
  "StunDuration": 2.5,
  "Upgrades": [],
  "ZipAcc": 1000,
  "ZipLineIntroModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "ZiplineIntro",
      "ZiplineLocked"
    ],
    "Subclass": "ZiplineIntro"
  },
  "ZipLineKnockdownImmuneModifier": {
    "Class": "ZiplineKnockdownImmune",
    "Subclass": "ZiplineKnockdownImmune"
  },
  "ZipLineSlowModifier": {
    "Class": "ZiplineSpeed",
    "PercentageMultiplierEnd": 0,
    "PercentageMultiplierStart": -70,
    "RampUpTime": 2.0,
    "Subclass": "ZiplineProtectionSlow"
  },
  "ZipSpeedInner": 693,
  "ZipSpeedOuter": 810,
  "ZiplineProtectionDamageAmp": 35,
  "ZiplineProtectionSlowDurationOnHit": 2.0,
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
