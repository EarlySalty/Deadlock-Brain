---
title: "Crimson Slash"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_healing_slash"
canonical_name: "Crimson Slash"
snapshot_id: 39613
source_document_id: 7070
payload_hash: "812c87fa3e850b4fd0848929534071d1a150c306a8a58a2e1cffa5c9187eb122"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.775781+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Crimson Slash

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_healing_slash`
- Snapshot ID: `39613`
- Source-Dokument: `7070`
- Kurzinfo: Crimson Slash aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BuffDuration": {
    "Scale": {
      "Type": "duration",
      "Value": 1.0
    },
    "Value": 0
  },
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "HealingSlashBuffModifier"
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.37
    },
    "Value": 55
  },
  "DebuffDuration": 4,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "FireRateSlow": 30,
  "HealFixedHealth": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.035871
    },
    "Value": 55
  },
  "IsDisabled": false,
  "Key": "citadel_ability_healing_slash",
  "Name": "Crimson Slash",
  "Radius": 13,
  "Upgrades": [
    {
      "BuffDuration": 4,
      "BuffMeleeDamage": 30
    },
    {
      "HealMaxHealth": 6
    },
    {
      "AbilityCooldown": -10,
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 0
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
    "card_name": "Crimson Slash",
    "hero_key": "hero_yamato",
    "hero_name": "Yamato",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_yamato",
      "hero_name": "Yamato",
      "lookup": "crimson slash",
      "name": "Crimson Slash",
      "type": "ability"
    }
  ]
}
````
