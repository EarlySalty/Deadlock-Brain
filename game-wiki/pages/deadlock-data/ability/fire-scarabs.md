---
title: "Fire Scarabs"
entity_type: "ability"
source: "deadlock_data"
external_id: "mirage_fire_beetles"
canonical_name: "Fire Scarabs"
snapshot_id: 39692
source_document_id: 7070
payload_hash: "2215efcae706493170be261e3ad839574d0aa154890babd40a5e66f933ac1f7c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.963704+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Fire Scarabs

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `mirage_fire_beetles`
- Snapshot ID: `39692`
- Source-Dokument: `7070`
- Kurzinfo: Fire Scarabs aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.05,
  "AbilityCharges": 2,
  "AbilityCooldown": 35,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Value": 8
  },
  "IsDisabled": false,
  "Key": "mirage_fire_beetles",
  "MaxStacks": 100,
  "Name": "Fire Scarabs",
  "OutgoingDamagePenaltyPercent": -20,
  "StatStolenDebuffModifier": {
    "Class": "MirageFireScarabsHealthLoss",
    "EnabledStateMask": [
      "HasFirebeetlesDebuff"
    ],
    "Subclass": "MirageFireScarabsHealthLoss"
  },
  "StealDuration": 7,
  "Upgrades": [
    {
      "DPS": 7
    },
    {
      "AbilityCharges": 2
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.17
        },
        "Value": 0
      },
      "OutgoingDamagePenaltyPercent": -15
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
    "card_name": "Fire Scarabs",
    "hero_key": "hero_mirage",
    "hero_name": "Mirage",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_mirage",
      "hero_name": "Mirage",
      "lookup": "fire scarabs",
      "name": "Fire Scarabs",
      "type": "ability"
    }
  ]
}
````
