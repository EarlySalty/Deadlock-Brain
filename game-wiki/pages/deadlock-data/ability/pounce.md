---
title: "Pounce"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_nano_pounce"
canonical_name: "Pounce"
snapshot_id: 39489
source_document_id: 7070
payload_hash: "a714e18caaa34334cd2b424328ee1f6e54531334c8fe5cda118a3f525b0ea669"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.463329+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Pounce

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_pounce`
- Snapshot ID: `39489`
- Source-Dokument: `7070`
- Kurzinfo: Pounce aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0744
    },
    "Value": 14
  },
  "AbilityCharges": 2,
  "AbilityCooldown": 26.0,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorMovement"
  ],
  "CameraDistance": 250,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 60
  },
  "DoublePounceModifier": {
    "Class": "Base",
    "Subclass": "DoublePounceNotify"
  },
  "DoublePounceTime": 3,
  "ExplodeRadius": 6,
  "IsDisabled": false,
  "JumpHeight": 3,
  "Key": "ability_nano_pounce",
  "LeapModifier": {
    "Class": "CitadelNanoPounceSelf",
    "Subclass": "CitadelNanoPounceSelf"
  },
  "MinTimeToTarget": 0.5,
  "MoveSpeedToTarget": 25,
  "Name": "Pounce",
  "SlashRange": 3,
  "SlowDuration": 2.0,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "SlowBase"
  },
  "SlowPercent": 30,
  "Upgrades": [
    {
      "SlowDuration": 1
    },
    {
      "ActiveReloadPercent": 20,
      "FireRateSlow": 30
    },
    {
      "AbilityCharges": 1,
      "Damage": 60
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
