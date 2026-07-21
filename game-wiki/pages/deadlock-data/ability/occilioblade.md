---
title: "Occilioblade"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_kali_spinning_blade"
canonical_name: "Occilioblade"
snapshot_id: 39622
source_document_id: 7070
payload_hash: "240f6061c6b21122ceb5fecc4b353741a9a8086563d76fbf647b03d2b60cf6de"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.796754+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Occilioblade

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_kali_spinning_blade`
- Snapshot ID: `39622`
- Source-Dokument: `7070`
- Kurzinfo: Occilioblade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 50,
  "AbilityCharges": 2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDontTriggerSpellBlock",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSprint"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 120
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "IsDisabled": false,
  "Key": "citadel_ability_kali_spinning_blade",
  "MinReflectionDOTResult": -0.95,
  "MinReflectionZ": 0.3,
  "Name": "Occilioblade",
  "NoClipDuration": 1,
  "ProjectileFlyOutTime": 0.6,
  "ProjectileFlyReturnTime": 1.0,
  "ReflectionSpeedFactor": 0.5,
  "ReturnOffSetTargetDistance": 150,
  "ReturnUpVelocity": 200,
  "TechCleaveExpireTime": 0.2,
  "Upgrades": [
    {
      "SlowDuration": 4,
      "SlowPercent": 30
    },
    {
      "Damage": {
        "Scale": {
          "Multiply": true,
          "Type": "spirit",
          "Value": 1.218672
        },
        "Value": 40
      }
    },
    {
      "CooldownReductionOnHit": -7
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
      "lookup": "occilioblade",
      "name": "Occilioblade",
      "type": "ability"
    }
  ]
}
````
