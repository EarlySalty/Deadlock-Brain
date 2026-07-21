---
title: "Bio Blast"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_scrap_blast"
canonical_name: "Bio Blast"
snapshot_id: 39525
source_document_id: 7070
payload_hash: "19d8e0db51315110fe29d3e94fd2b0ce12fa3510042a0152a09e0b7261f8752d"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.544329+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bio Blast

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_scrap_blast`
- Snapshot ID: `39525`
- Source-Dokument: `7070`
- Kurzinfo: Bio Blast aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 15,
  "AbilityCharges": 2,
  "AbilityCooldown": 64.0,
  "AbilityCooldownBetweenCharge": 3,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BlastRadius": 10,
  "ChannelMoveSpeed": -1,
  "DebuffModifier": {
    "Class": "ScrapBlastDebuff",
    "Subclass": "ScrapBlastDebuff"
  },
  "EnemyMoveSlow": 10,
  "EnemyMoveSlowDuration": 5,
  "IsDisabled": false,
  "Key": "ability_scrap_blast",
  "Name": "Bio Blast",
  "ScrapDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.731203
    },
    "Value": 75
  },
  "Upgrades": [
    {
      "AbilityCharges": 2
    },
    {
      "ScrapDamage": 55
    },
    {
      "EnemyMoveSlow": 20
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
    "card_name": "Bio Blast",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "bio blast",
      "name": "Bio Blast",
      "type": "ability"
    }
  ]
}
````
