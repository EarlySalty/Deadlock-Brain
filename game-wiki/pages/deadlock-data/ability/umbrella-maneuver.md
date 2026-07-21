---
title: "Umbrella Maneuver"
entity_type: "ability"
source: "deadlock_data"
external_id: "operative_umbrella_maneuver"
canonical_name: "Umbrella Maneuver"
snapshot_id: 39700
source_document_id: 7070
payload_hash: "32dd93d1ae0911eef97f2787715d6e747b662ac10cf55d2ecf9503bc4a99171a"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.986317+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Umbrella Maneuver

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `operative_umbrella_maneuver`
- Snapshot ID: `39700`
- Source-Dokument: `7070`
- Kurzinfo: Umbrella Maneuver aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "ActivateTime": 0.2,
  "AirHangModifier": {
    "AirDrag": -1,
    "AirSpeed": -1,
    "Class": "OperativeUmbrellaManeuverAirHang",
    "FallSpeed": 10,
    "Subclass": "OperativeUmbrellaManeuverAirHang"
  },
  "AirSpeedMax": 3.81,
  "BackwardsVelocity": 13.0,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.395
    },
    "Value": 100
  },
  "ExplodeRadius": 5,
  "FallSpeedMax": 1.524,
  "IsDisabled": false,
  "Key": "operative_umbrella_maneuver",
  "Name": "Umbrella Maneuver",
  "TimeBeforeProjectileLaunch": 1.25,
  "UpImpulse": 15.0,
  "Upgrades": [
    {
      "AbilityCooldown": -14
    },
    {
      "Damage": 50
    },
    {
      "AbilityCooldown": 0
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
    "card_name": "Umbrella Maneuver",
    "hero_key": "hero_operative",
    "hero_name": "Raven",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_operative",
      "hero_name": "Raven",
      "lookup": "umbrella maneuver",
      "name": "Umbrella Maneuver",
      "type": "ability"
    }
  ]
}
````
