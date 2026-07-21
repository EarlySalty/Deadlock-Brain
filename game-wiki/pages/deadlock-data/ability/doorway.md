---
title: "Doorway"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_doorman_doorway"
canonical_name: "Doorway"
snapshot_id: 39420
source_document_id: 7070
payload_hash: "b07bdb0ec632b5b8b014ee22950f950cf45e0569e502b987cf38207ae3400f29"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.276272+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Doorway

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_doorman_doorway`
- Snapshot ID: `39420`
- Source-Dokument: `7070`
- Kurzinfo: Doorway aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 50,
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 20,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorAllowAltCast",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "DoorwayCloseCooldown": 8,
  "DoorwayDistance": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 70
  },
  "DoorwayTimerModifier": {
    "Class": "Base",
    "Subclass": "DoorwayTimerModifier"
  },
  "IsDisabled": false,
  "Key": "ability_doorman_doorway",
  "Name": "Doorway",
  "PortalBarrierModifier": {
    "Class": "Base",
    "Subclass": "DoorwayBarrier"
  },
  "Upgrades": [
    {
      "AbilityDuration": 15
    },
    {
      "BarrierDuration": 12,
      "CombatBarrier": {
        "Scale": {
          "Type": "spirit",
          "Value": 1.5
        },
        "Value": 250
      }
    },
    {
      "AbilityCastRange": 30,
      "DoorwayDistance": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.15
        },
        "Value": 45
      }
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
    "card_name": "Doorway",
    "hero_key": "hero_doorman",
    "hero_name": "The Doorman",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_doorman",
      "hero_name": "The Doorman",
      "lookup": "doorway",
      "name": "Doorway",
      "type": "ability"
    }
  ]
}
````
