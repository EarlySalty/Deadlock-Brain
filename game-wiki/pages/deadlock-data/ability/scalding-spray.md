---
title: "Scalding Spray"
entity_type: "ability"
source: "deadlock_data"
external_id: "fathom_scalding_spray"
canonical_name: "Scalding Spray"
snapshot_id: 39679
source_document_id: 7070
payload_hash: "1a27c21b505bd14ab34eb918467487dcaf33919c9fac2e6c1247dc1e6866a743"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.933247+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Scalding Spray

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `fathom_scalding_spray`
- Snapshot ID: `39679`
- Source-Dokument: `7070`
- Kurzinfo: Scalding Spray aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCharges": 1,
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": 8,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "AuraRadius": 0.0,
    "AuraTargetingConeAngle": 60.0,
    "AuraTargetingConeHalfWidth": 50.0,
    "BuffModifier": {
      "Class": "FathomScaldingSprayWeaponDamage",
      "Subclass": "FathomScaldingSprayWeaponDamage"
    },
    "Class": "FathomScaldingSprayAura",
    "ProvidedByAura": {
      "Class": "FathomScaldingSprayTarget",
      "Subclass": "FathomScaldingSprayTarget"
    },
    "Subclass": "FathomScaldingSprayAura"
  },
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.372
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "fathom_scalding_spray",
  "Name": "Scalding Spray",
  "Radius": 12,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": 55
    }
  ],
  "WeaponDamageBonusDuration": 12,
  "WeaponDamageBonusPerSec": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0372
    },
    "Value": 5
  },
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
    "card_name": "Scalding Spray",
    "hero_key": "hero_slork",
    "hero_name": "Fathom",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "scalding spray",
      "name": "Scalding Spray",
      "type": "ability"
    }
  ]
}
````
