---
title: "Crackshot"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_crackshot"
canonical_name: "Crackshot"
snapshot_id: 39417
source_document_id: 7070
payload_hash: "01e93d8ec81c4484746e5b74a35540ee5e9df7a1c061a7af85c66d18b5e84b46"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.269519+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Crackshot

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_crackshot`
- Snapshot ID: `39417`
- Source-Dokument: `7070`
- Kurzinfo: Crackshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CrackshotNPCCDReduction": 50,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.116
    },
    "Value": 55
  },
  "DebuffDuration": 2,
  "DebuffModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "ExplosionRadius": 2,
  "FadingSlowPercent": 50,
  "IsDisabled": false,
  "Key": "ability_crackshot",
  "Name": "Crackshot",
  "Upgrades": [
    {
      "FadingSlowPercent": 25
    },
    {
      "Damage": 49.5
    },
    {
      "AbilityCooldownPerHeadshot": -6,
      "AbilityCooldownPerHeadshotNPC": -3
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
    "card_name": "Crackshot",
    "hero_key": "hero_astro",
    "hero_name": "Holliday",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "crackshot",
      "name": "Crackshot",
      "type": "ability"
    }
  ]
}
````
