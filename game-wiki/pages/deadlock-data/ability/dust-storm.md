---
title: "Dust Storm"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_kali_dust_storm"
canonical_name: "Dust Storm"
snapshot_id: 39468
source_document_id: 7070
payload_hash: "1fc9542105e4112e7289cc4869fa674bf687bfdbbde8dcfec3e7e4ec07dcf3d4"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.403323+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Dust Storm

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_kali_dust_storm`
- Snapshot ID: `39468`
- Source-Dokument: `7070`
- Kurzinfo: Dust Storm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDontInterruptSprint",
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "ClimbHeight": 1,
  "CloseRangeSpeed": 80,
  "DamagePerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 50
  },
  "DebuffDuration": 3.0,
  "DistanceAboveGround": 2,
  "DropDownRate": 2,
  "FireRateSlow": 20,
  "GrenadeTrailModifier": {
    "Class": "CitadelDustStormThrown",
    "Subclass": "CitadelDustStormThrown"
  },
  "GroundDashReductionPercent": -30,
  "IsDisabled": false,
  "Key": "ability_kali_dust_storm",
  "Name": "Dust Storm",
  "OpenHeight": 2,
  "Radius": 3.5,
  "SlowPercent": 20,
  "ThrownObjectRadius": 20,
  "TickRate": 0.25,
  "TornadoSpeed": 600,
  "TrackingDistance": 15,
  "Upgrades": [
    {
      "SlowPercent": 30
    },
    {
      "FireRateSlow": 40
    },
    {
      "AbilityCooldown": -9.5,
      "AbilityDuration": 3
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kali",
      "hero_name": "Kali",
      "lookup": "dust storm",
      "name": "Dust Storm",
      "type": "ability"
    }
  ]
}
````
