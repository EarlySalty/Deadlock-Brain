---
title: "citadel_ability_vandal_surge"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_vandal_surge"
canonical_name: "citadel_ability_vandal_surge"
snapshot_id: 39663
source_document_id: 7070
payload_hash: "15dbed6ff28bb378ffc83d176988be7fb2de282c4561237f753a73fd98166fd3"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.893949+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# citadel_ability_vandal_surge

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_vandal_surge`
- Snapshot ID: `39663`
- Source-Dokument: `7070`
- Kurzinfo: citadel_ability_vandal_surge aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCastRange": 20,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.25,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectilePassThroughWorld",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 100
  },
  "DampingFactor": 0.5,
  "IsDisabled": false,
  "Key": "citadel_ability_vandal_surge",
  "LiftHeight": 120,
  "LiftModifier": {
    "Class": "CitadelVandalsurge",
    "EnabledStateMask": [
      "Stunned"
    ],
    "Subclass": "Lift"
  },
  "Name": null,
  "Upgrades": [
    {
      "AbilityCooldown": -28.0
    },
    {
      "AbilityDuration": 0.5
    },
    {
      "AbilityUnitTargetLimit": 5
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
    "card_name": "Unknown(citadel_ability_vandal_surge)",
    "hero_key": "hero_vandal",
    "hero_name": "Vandal",
    "slot": "1"
  }
}
````
