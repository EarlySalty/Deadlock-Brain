---
title: "Chain Lightning"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_chain_lightning"
canonical_name: "Chain Lightning"
snapshot_id: 39601
source_document_id: 7070
payload_hash: "48063f21215923f5d5e4e4358ccfcfffc42b1ea94185bd9142912b3189df112c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.743725+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Chain Lightning

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_chain_lightning`
- Snapshot ID: `39601`
- Source-Dokument: `7070`
- Kurzinfo: Chain Lightning aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 0.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "CitadelLightningBullet",
      "Subclass": "CitadelLightningBullet"
    }
  ],
  "BehaviourBits": null,
  "ChannelMoveSpeed": -1,
  "ConeAngle": 45,
  "ConeRadius": 6,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.55
    },
    "Value": 15
  },
  "IsDisabled": false,
  "Key": "citadel_ability_chain_lightning",
  "Name": "Chain Lightning",
  "Upgrades": [
    {
      "ConeRadius": 4
    },
    {
      "Damage": 19.8
    },
    {
      "MultiChain": 1
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
