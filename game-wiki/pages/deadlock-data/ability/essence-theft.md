---
title: "Essence Theft"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_necro_fear"
canonical_name: "Essence Theft"
snapshot_id: 39495
source_document_id: 7070
payload_hash: "11000da5071c50c853ca665764458780daf731b1489289894e233d9686781daa"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.476292+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Essence Theft

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_necro_fear`
- Snapshot ID: `39495`
- Source-Dokument: `7070`
- Kurzinfo: Essence Theft aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "DebuffModifier": {
    "BuildUpDecayDelay": 2.0,
    "Class": "NecroRampup",
    "CycleTimeDelayAdd": 0.1,
    "Subclass": "Debuff"
  },
  "DelayBeforeLoss": 0.5,
  "IsDisabled": false,
  "Key": "ability_necro_fear",
  "MaxStolenAttackDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.25
    },
    "Value": 25
  },
  "MaxStolenSpiritResist": 10,
  "MaxStolenTargets": 3,
  "Name": "Essence Theft",
  "ProgressLossMultiplier": 2.3,
  "ProgressLossPerSecond": 1,
  "ShootDurationForMax": 4,
  "TickInterval": 0.15,
  "Upgrades": [
    {
      "MaxStolenSpiritResist": 5
    },
    {
      "MaxStolenAttackDamage": 20
    },
    {
      "SkullBuildUp": 0.15,
      "ZombieExplosionBuildUp": 1.0,
      "ZombieMeleeBuildUp": 0.15
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
    "card_name": "Essence Theft",
    "hero_key": "hero_necro",
    "hero_name": "Graves",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_necro",
      "hero_name": "Graves",
      "lookup": "essence theft",
      "name": "Essence Theft",
      "type": "ability"
    }
  ]
}
````
