---
title: "Napalm"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_incendiary_projectile"
canonical_name: "Napalm"
snapshot_id: 39466
source_document_id: 7070
payload_hash: "c8af275f5a4de911de42a4e18eb9a0fb9431e2be7a487fe0a5ca82be450e5680"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.397584+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Napalm

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_incendiary_projectile`
- Snapshot ID: `39466`
- Source-Dokument: `7070`
- Kurzinfo: Napalm aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 20,
  "AbilityCharges": 1,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 18,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 40.0
  },
  "DebuffDuration": 8,
  "DebuffModifier": {
    "Class": "IncendiaryDebuff",
    "StatusEffectPriority": 50,
    "Subclass": "IncendiaryDebuff"
  },
  "GrowthPerMeter": 0.5,
  "HeightOffGround": 50,
  "IncomingDamagePercentFromCaster": 16,
  "InitialWidth": 1,
  "IsDisabled": false,
  "Key": "ability_incendiary_projectile",
  "Name": "Napalm",
  "ParticleRadiusMultiplier": 1.15,
  "SlowDuration": 4,
  "SlowModifier": {
    "Class": "SlowBase",
    "Subclass": "Slow"
  },
  "SlowPercent": 35,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "LifestealPercentHero": 15
    },
    {
      "HealAmpReceivePenaltyPercent": -33,
      "HealAmpRegenPenaltyPercent": -33,
      "IncomingDamagePercentFromCaster": 17
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
    "card_name": "Napalm",
    "hero_key": "hero_inferno",
    "hero_name": "Infernus",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_inferno",
      "hero_name": "Infernus",
      "lookup": "napalm",
      "name": "Napalm",
      "type": "ability"
    }
  ]
}
````
