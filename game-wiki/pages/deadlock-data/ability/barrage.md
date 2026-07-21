---
title: "Barrage"
entity_type: "ability"
source: "deadlock_data"
external_id: "synth_barrage"
canonical_name: "Barrage"
snapshot_id: 39709
source_document_id: 7070
payload_hash: "3a8bafadb73a7c8f727b848626eed83eef6fd08562a42f643404a53c980447d8"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.012439+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Barrage

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_barrage`
- Snapshot ID: `39709`
- Source-Dokument: `7070`
- Kurzinfo: Barrage aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityChannelTime": 2,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 0.3,
  "AirSpeedMax": 2.54,
  "AmpDuration": 15,
  "AmpModifier": {
    "Class": "SynthBarrageAmp",
    "Subclass": "SynthBarrageAmp"
  },
  "AmpPercentPerStack": 6,
  "AutoChannelModifier": {
    "Class": "IntrinsicBase",
    "Subclass": "IntrinsicBase"
  },
  "BarrageCasterModifier": {
    "Class": "SynthBarrageCaster",
    "Subclass": "SynthBarrageCaster"
  },
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorChannelled",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "DamagePerProjectile": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 32
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "SynthBarrageDebuff"
  },
  "FallSpeedMax": 10,
  "IsDisabled": false,
  "Key": "synth_barrage",
  "MoveSlowPercent": 30,
  "Name": "Barrage",
  "ProjectileAmount": 4,
  "Radius": 4.5,
  "SlowDuration": 1.5,
  "Upgrades": [
    {
      "DamagePerProjectile": 16
    },
    {
      "AbilityCooldown": -16.0
    },
    {
      "AmpPercentPerStack": 4,
      "Radius": 3
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
    "card_name": "Barrage",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "barrage",
      "name": "Barrage",
      "type": "ability"
    }
  ]
}
````
