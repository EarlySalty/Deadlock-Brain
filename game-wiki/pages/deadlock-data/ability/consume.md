---
title: "Consume"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_wrecker_salvage"
canonical_name: "Consume"
snapshot_id: 39586
source_document_id: 7070
payload_hash: "124fa09cd7b5b3b3b11534c89698e459303519f6e6ddc2f6f1fe909446eb0865"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.700800+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Consume

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_wrecker_salvage`
- Snapshot ID: `39586`
- Source-Dokument: `7070`
- Kurzinfo: Consume aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 15,
  "AbilityCooldown": 12.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled"
  ],
  "BuffModifier": {
    "Class": "WreckerSalvageBuff",
    "Subclass": "WreckerSalvageBuff"
  },
  "ChannelMoveSpeed": 3.8,
  "ConsumeHealPercentage": 50,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.974938
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "ability_wrecker_salvage",
  "MaxRange": 20,
  "Name": "Consume",
  "SalvageDuration": 4,
  "SalvageEnemyModifier": {
    "Class": "WreckerSalvage",
    "Subclass": "WreckerSalvage"
  },
  "StunEnemyModifier": {
    "Class": "CitadelStunned",
    "Subclass": "WreckerSalvageStun"
  },
  "TickInterval": 0.25,
  "Upgrades": [
    {
      "ConsumeHealPercentage": 25
    },
    {
      "DPS": 40
    },
    {
      "AbilityUnitTargetLimit": 2
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
    "card_name": "Consume",
    "hero_key": "hero_wrecker",
    "hero_name": "Wrecker",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_wrecker",
      "hero_name": "Wrecker",
      "lookup": "consume",
      "name": "Consume",
      "type": "ability"
    }
  ]
}
````
