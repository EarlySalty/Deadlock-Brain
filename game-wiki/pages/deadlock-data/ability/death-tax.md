---
title: "Death Tax"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_death_tax"
canonical_name: "Death Tax"
snapshot_id: 39418
source_document_id: 7070
payload_hash: "a13078d675b57e25ab2ff1e80a7a536b8814156cf90d6afde077c9118b424497"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.271659+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Death Tax

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_death_tax`
- Snapshot ID: `39418`
- Source-Dokument: `7070`
- Kurzinfo: Death Tax aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "DeathTaxHeal": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 30
  },
  "IsDisabled": false,
  "Key": "ability_death_tax",
  "Name": "Death Tax",
  "Upgrades": [
    {
      "CooldownReductionOnKill": 1
    },
    {
      "DeathTaxHeal": 30
    },
    {
      "TechPowerAmpBonus": 10,
      "TechPowerAmpBonusDuration": 10,
      "TechPowerAmpBonusMaxStacks": 10
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
