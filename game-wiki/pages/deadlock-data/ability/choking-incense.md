---
title: "Choking Incense"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_silencebomb"
canonical_name: "Choking Incense"
snapshot_id: 39515
source_document_id: 7070
payload_hash: "9957eb85790bdddae4fb02afcdc4a311651738499ad3fac92212634fa08371de"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.522718+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Choking Incense

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_silencebomb`
- Snapshot ID: `39515`
- Source-Dokument: `7070`
- Kurzinfo: Choking Incense aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 33,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 99,
  "AuraModifier": {
    "Class": "CitadelPriestSilencebombAura",
    "ProvidedByAura": {
      "Class": "SilencebombDebuff",
      "Subclass": "Debuff"
    },
    "Subclass": "Aura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": 5,
  "DebuffDuration": 3,
  "InitialRadius": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.01
    },
    "Value": 4
  },
  "IsDisabled": false,
  "Key": "ability_priest_silencebomb",
  "Name": "Choking Incense",
  "RadiusPerSecond": 0.2,
  "SlowDuration": 2,
  "SlowPercent": 30,
  "SmokeGrenadeModifier": {
    "Class": "Smokegrenade",
    "EnemyAuraModifier": {
      "Class": "BaseAura",
      "ProvidedByAura": {
        "Class": "Base",
        "Subclass": "Debuff"
      },
      "Subclass": "Enemyaura"
    },
    "Subclass": "PriestSmokegrenade"
  },
  "TickRate": 0.15,
  "Upgrades": [
    {
      "AbilityDuration": 3
    },
    {
      "SlowPercent": 20
    },
    {
      "SilenceStamina": 1
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
