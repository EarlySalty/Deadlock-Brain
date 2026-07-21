---
title: "Captivating Read"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_bookworm_aoemagic"
canonical_name: "Captivating Read"
snapshot_id: 39407
source_document_id: 7070
payload_hash: "ab9a3fc954d0fb563d8e50e86d87bb6ea243b2feaa9b3ac48cdc3d6b66795df8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.247136+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Captivating Read

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_aoemagic`
- Snapshot ID: `39407`
- Source-Dokument: `7070`
- Kurzinfo: Captivating Read aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 30,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AreaModifier": {
    "Class": "BookwormAoemagicAreamodifier",
    "DebuffModifier": {
      "Class": "Base",
      "Subclass": "Debuff"
    },
    "RootModifier": {
      "Class": "BookwormImmobilize",
      "Subclass": "Root"
    },
    "SlowModifier": {
      "Class": "SlowBase",
      "Subclass": "Slowmodifier"
    },
    "Subclass": "Areamodifier"
  },
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 90
  },
  "DetonationDelay": 1.25,
  "Height": 8,
  "ImmobilizeDuration": 1.0,
  "IsDisabled": false,
  "Key": "ability_bookworm_aoemagic",
  "Name": "Captivating Read",
  "Radius": 7.5,
  "SlowDuration": 0.5,
  "SlowPercent": 45,
  "Upgrades": [
    {
      "AbilityCooldown": -11
    },
    {
      "ImmobilizeDuration": 1
    },
    {
      "DebuffDuration": 6,
      "Radius": 1,
      "TechArmorDamageReduction": -18
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
    "card_name": "Captivating Read",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "captivating read",
      "name": "Captivating Read",
      "type": "ability"
    }
  ]
}
````
