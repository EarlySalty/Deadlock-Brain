---
title: "Gloom Bombs"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_nano_clustergrenade"
canonical_name: "Gloom Bombs"
snapshot_id: 39487
source_document_id: 7070
payload_hash: "21704ff150ada49b2e9b675eb39090b61d538b8af52d410495f347ed3ca0ceaa"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.457672+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Gloom Bombs

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_nano_clustergrenade`
- Snapshot ID: `39487`
- Source-Dokument: `7070`
- Kurzinfo: Gloom Bombs aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.644
    },
    "Value": 45
  },
  "DebuffModifier": {
    "Class": "NanoClusterGrenadeDebuff",
    "Subclass": "NanoClusterGrenadeDebuff"
  },
  "GrenadeAngleVariance": 0.08,
  "GrenadeCount": 4,
  "IsDisabled": false,
  "Key": "ability_nano_clustergrenade",
  "Lifetime": 0.75,
  "ModifierDragEnemy": {
    "Class": "PerchedPredatorDrag",
    "Subclass": "PerchedPredatorDrag"
  },
  "MultiHitPenaltyPercentage": 65,
  "Name": "Gloom Bombs",
  "Radius": 3.0,
  "TimeBetweenGrenades": 0.05,
  "TossSpeed": 400,
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "MeleeResistReduction": -6,
      "MeleeResistReductionDuration": 6
    },
    {
      "GrenadeCount": 3
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
    "card_name": "Gloom Bombs",
    "hero_key": "hero_nano",
    "hero_name": "Calico",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "gloom bombs",
      "name": "Gloom Bombs",
      "type": "ability"
    }
  ]
}
````
