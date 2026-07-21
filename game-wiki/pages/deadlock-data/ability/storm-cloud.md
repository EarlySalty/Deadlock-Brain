---
title: "Storm Cloud"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_storm_cloud"
canonical_name: "Storm Cloud"
snapshot_id: 39649
source_document_id: 7070
payload_hash: "67a76769c2c75772872101b3c60e00badb18259eda229408a25cd85d2fa5cb50"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.861179+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Storm Cloud

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_storm_cloud`
- Snapshot ID: `39649`
- Source-Dokument: `7070`
- Kurzinfo: Storm Cloud aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": 7,
  "AbilityCooldown": 205.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorExclusiveUse",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraDistance": 600,
  "CloudHeight": 120,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 95
  },
  "DamageInterval": 0.3,
  "EndingSoonTime": 2,
  "ExpandTime": 3.5,
  "FlightControlEnabled": 1.5,
  "InitialRadius": 10,
  "IsDisabled": false,
  "Key": "citadel_ability_storm_cloud",
  "LightningStrikeAOEModifier": {
    "Class": "LightningStrikeArea",
    "Subclass": "LightningStrike"
  },
  "LightningStrikeDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.5
    },
    "Value": 75.0
  },
  "LightningStrikeDelay": 0.25,
  "LightningStrikeKnockBackForce": 500,
  "LightningStrikeRadius": 7,
  "LightningStrikes": 1,
  "Name": "Storm Cloud",
  "Radius": 30,
  "StormCloudModifier": {
    "Class": "CitadelStormcloud",
    "Subclass": "CitadelStormcloud"
  },
  "Upgrades": [
    {
      "BulletResistOnActive": 55
    },
    {
      "AbilityChannelTime": 7,
      "InitialRadius": 5,
      "Radius": 10
    },
    {
      "DPS": 65.0,
      "FlightControlEnabled": 4
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
    "card_name": "Storm Cloud",
    "hero_key": "hero_gigawatt",
    "hero_name": "Seven",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gigawatt",
      "hero_name": "Seven",
      "lookup": "storm cloud",
      "name": "Storm Cloud",
      "type": "ability"
    }
  ]
}
````
