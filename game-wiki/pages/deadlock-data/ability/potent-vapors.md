---
title: "Potent Vapors"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_selfheal"
canonical_name: "Potent Vapors"
snapshot_id: 39514
source_document_id: 7070
payload_hash: "27afca2d4bec2404bd3d295d45b9c5a92a6848ac7c5c63ccd2648f7d7ffb2d6e"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.520708+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Potent Vapors

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_selfheal`
- Snapshot ID: `39514`
- Source-Dokument: `7070`
- Kurzinfo: Potent Vapors aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.125,
  "AbilityChannelTime": 4,
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.125,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorCastableWhileCmdRestricted",
    "BehaviorDisplaysDamageImpact",
    "BehaviorNonCombat",
    "BehaviorCannotCancelDuringChannel"
  ],
  "ChannelMoveSpeed": -1,
  "FlatHealthHealing": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.79
    },
    "Value": 120
  },
  "IncomingDamagePercent": -35,
  "IsDisabled": false,
  "Key": "ability_priest_selfheal",
  "Name": "Potent Vapors",
  "SelfModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Disarmed"
    ],
    "Subclass": "Selfheal"
  },
  "TickRate": 0.25,
  "Upgrades": [
    {
      "FlatHealthHealing": 80
    },
    {
      "AbilityCooldown": -15
    },
    {
      "SlowResistance": 100
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
  }
}
````
