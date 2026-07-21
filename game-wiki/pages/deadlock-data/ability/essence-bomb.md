---
title: "Essence Bomb"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_blood_bomb"
canonical_name: "Essence Bomb"
snapshot_id: 39396
source_document_id: 7070
payload_hash: "90a35685c55fa52f9aff1a12970ce3716fe0c007296a065f2d93951f08fedcf9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.222529+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Essence Bomb

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_blood_bomb`
- Snapshot ID: `39396`
- Source-Dokument: `7070`
- Kurzinfo: Essence Bomb aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 14.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ArmingDuration": 0.65,
  "BeepSoundBuildupCount": 4,
  "BeepSoundIntervalBias": 0.55,
  "BeepSoundMaxFrequency": 0.1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.22
    },
    "Value": 90
  },
  "IsDisabled": false,
  "Key": "ability_blood_bomb",
  "Name": "Essence Bomb",
  "Radius": 7,
  "SelfDamagePct": 30,
  "SpilledBloodModifier": {
    "Class": "SpilledBloodThinker",
    "Height": 80.0,
    "Subclass": "SpilledBloodThinker",
    "TickRate": 0.5
  },
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 50,
      "Radius": 2
    },
    {
      "BloodSpillDPSPercent": 26,
      "BloodSpillDuration": 6
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
    "card_name": "Essence Bomb",
    "hero_key": "hero_ghost",
    "hero_name": "Lady Geist",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_ghost",
      "hero_name": "Lady Geist",
      "lookup": "essence bomb",
      "name": "Essence Bomb",
      "type": "ability"
    }
  ]
}
````
