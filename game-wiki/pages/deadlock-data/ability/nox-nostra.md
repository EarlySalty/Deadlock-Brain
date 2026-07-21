---
title: "Nox Nostra"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_vampirebat_batswarm"
canonical_name: "Nox Nostra"
snapshot_id: 39558
source_document_id: 7070
payload_hash: "4a667041015c5c33bc9742595e1bada632ec4dc35f4f80d4def38dbcf69b1406"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.631509+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Nox Nostra

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_vampirebat_batswarm`
- Snapshot ID: `39558`
- Source-Dokument: `7070`
- Kurzinfo: Nox Nostra aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCastRange": 40,
  "AbilityCooldown": 150,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 12,
  "AirDrag": 12,
  "BatCount": 75,
  "BatCountPerWave": 1,
  "BatEffectiveness": 0.2,
  "BatPerSecond": 30,
  "BatSpawnRadius": 1.5,
  "BatSpawnRandomAngle": 0.15,
  "BatSpawnRandomVelocity": 300,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusBatsMax": 50,
  "BonusBatsPerProc": 2,
  "ChannelMoveSpeed": -1,
  "CurrentHealthDamageCapToBosses": 20,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.094
    },
    "Value": 4.6
  },
  "DebuffDuration": 1.25,
  "DebuffModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "Debuff"
  },
  "FallSpeedMax": 1,
  "GroundAccelerationPercentage": -80,
  "GroundFrictioNpercentage": -80,
  "IsDisabled": false,
  "JumpCeilingCheckDistance": 11,
  "JumpPitch": -60,
  "JumpSpeed": 17,
  "Key": "ability_vampirebat_batswarm",
  "LockonConeAngle": 40,
  "MaxBatTargets": 2,
  "MaxLockonStacks": 1,
  "Name": "Nox Nostra",
  "NotInConeLosesLock": 1,
  "StacksCanDecay": 1,
  "TargetingConeAngle": 20,
  "TimeToGainLockonStack": 0.01,
  "TimeToLoseLockonStack": 0.3,
  "Upgrades": [
    {
      "Damage": 1.9
    },
    {
      "AbilityCooldown": -45
    },
    {
      "CurrentHealthPercent": 0.5
    }
  ],
  "VerticalDrag": 1,
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
    "card_name": "Nox Nostra",
    "hero_key": "hero_vampirebat",
    "hero_name": "Mina",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_vampirebat",
      "hero_name": "Mina",
      "lookup": "nox nostra",
      "name": "Nox Nostra",
      "type": "ability"
    }
  ]
}
````
