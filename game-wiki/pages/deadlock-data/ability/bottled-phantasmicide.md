---
title: "Bottled Phantasmicide"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_trapper_poisonjar"
canonical_name: "Bottled Phantasmicide"
snapshot_id: 39545
source_document_id: 7070
payload_hash: "8e0217558ae5f06dcdc634b0f9556788157adbb749eb9efc27c55518d6cc5fde"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.595720+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bottled Phantasmicide

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_poisonjar`
- Snapshot ID: `39545`
- Source-Dokument: `7070`
- Kurzinfo: Bottled Phantasmicide aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "CitadelTrapperPoisonjarAura",
    "ProvidedByAura": {
      "Class": "CitadelSilenced",
      "Subclass": "PoisonjarDebuff"
    },
    "Subclass": "PoisonjarAura"
  },
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "Height": 2,
  "InitialRadius": 6,
  "IsDisabled": false,
  "Key": "ability_trapper_poisonjar",
  "Name": "Bottled Phantasmicide",
  "RadiusPerSecond": 0.25,
  "SlowPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 25
  },
  "TickRate": 0.25,
  "Upgrades": [
    {
      "SlowPercent": 20
    },
    {
      "AbilityDuration": 4
    },
    {
      "TechArmorDamageReduction": -25
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
    "card_name": "Bottled Phantasmicide",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "bottled phantasmicide",
      "name": "Bottled Phantasmicide",
      "type": "ability"
    }
  ]
}
````
