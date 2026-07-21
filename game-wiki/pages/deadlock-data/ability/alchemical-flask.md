---
title: "Alchemical Flask"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_warden_crowd_control"
canonical_name: "Alchemical Flask"
snapshot_id: 39569
source_document_id: 7070
payload_hash: "06a8c418a40429e6f670930a79af1c7e7a14bcaa36fccaa2fe1bd317624e1e8e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.660418+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Alchemical Flask

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_crowd_control`
- Snapshot ID: `39569`
- Source-Dokument: `7070`
- Kurzinfo: Alchemical Flask aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCooldown": 12.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.63
    },
    "Value": 60
  },
  "DebuffDuration": 7,
  "DebuffModifier": {
    "Class": "WardenCrowdControlDebuff",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "WardenCrowdControlDebuff"
  },
  "ForwardVelocity": 800,
  "IsDisabled": false,
  "Key": "ability_warden_crowd_control",
  "MoveSpeedSlowPct": 20,
  "Name": "Alchemical Flask",
  "ProjectileLifetime": 60,
  "Radius": 5.5,
  "SlowDuration": 3,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "Upgrades": [
    {
      "StaminaReduction": 1
    },
    {
      "Damage": 35,
      "WeaponPowerDebuff": -25
    },
    {
      "AbilityCooldown": -7,
      "FireRateSlow": 30,
      "Radius": 2
    }
  ],
  "WeaponPowerDebuff": -25,
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
    "card_name": "Alchemical Flask",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "alchemical flask",
      "name": "Alchemical Flask",
      "type": "ability"
    }
  ]
}
````
