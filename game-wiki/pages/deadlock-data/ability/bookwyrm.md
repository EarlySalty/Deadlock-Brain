---
title: "Bookwyrm"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_bookworm_dragonfire"
canonical_name: "Bookwyrm"
snapshot_id: 39408
source_document_id: 7070
payload_hash: "85c4845d7dfbfe3e22ac1c41cedff204628f5ad398eb1402c9ce9d9a11f1cabf"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.249651+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bookwyrm

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_bookworm_dragonfire`
- Snapshot ID: `39408`
- Source-Dokument: `7070`
- Kurzinfo: Bookwyrm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCharges": 1,
  "AbilityCooldown": 33,
  "AbilityCooldownBetweenCharge": 7,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 0.1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.3
    },
    "Value": 30
  },
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 60
  },
  "DebuffDuration": 1.5,
  "DragonConeRange": 5,
  "DragonRangePerSecond": 500,
  "DragonSearchRadius": 8.5,
  "DragonSearchTickRate": 0.1,
  "DragonTravelRange": 20,
  "DragonUpwardSpeed": 400,
  "GroundAuraModifier": {
    "Class": "DragonfireGroundAura",
    "ProvidedByAura": {
      "Class": "BookwormDragonfire",
      "Subclass": "GroundauraBurn"
    },
    "Subclass": "Groundaura"
  },
  "GroundAuraSpacing": 1,
  "GroundFlameDuration": 3.0,
  "IsDisabled": false,
  "Key": "ability_bookworm_dragonfire",
  "Name": "Bookwyrm",
  "Radius": 4,
  "StartupDelay": 0.3,
  "TickRate": 0.3,
  "Upgrades": [
    {
      "AbilityCooldown": -12
    },
    {
      "AbilityCharges": 1,
      "GroundFlameDuration": 2,
      "Radius": 1
    },
    {
      "DPS": 30.0,
      "Damage": 100,
      "DragonTravelRange": 12
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
    "card_name": "Bookwyrm",
    "hero_key": "hero_bookworm",
    "hero_name": "Paige",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_bookworm",
      "hero_name": "Paige",
      "lookup": "bookwyrm",
      "name": "Bookwyrm",
      "type": "ability"
    }
  ]
}
````
