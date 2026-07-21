---
title: "Binding Word"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_warden_lock_down"
canonical_name: "Binding Word"
snapshot_id: 39571
source_document_id: 7070
payload_hash: "ee977588d5b2eaab916fbd63a3f7e787fcb7b339ca4fe91cc2f1c060cd5910b7"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.666948+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Binding Word

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_warden_lock_down`
- Snapshot ID: `39571`
- Source-Dokument: `7070`
- Kurzinfo: Binding Word aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 15,
  "AbilityCooldown": 34,
  "AbilityUnitTargetLimit": 1,
  "AdditionalTargetRadius": 20,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.437344
    },
    "Value": 110
  },
  "DebuffModifier": {
    "BulletResistModifier": {
      "Class": "LockdownBulletResist",
      "Subclass": "LockdownBulletResist"
    },
    "Class": "WardenLockdownDebuff",
    "RootModifier": {
      "Class": "CitadelRoot",
      "Subclass": "CitadelRoot"
    },
    "SilencedModifier": {
      "Class": "CitadelSilenced",
      "Subclass": "CitadelSilenced"
    },
    "Subclass": "WardenLockdownDebuff"
  },
  "EscapeRange": 20,
  "EscapeTime": 2.8,
  "ImmobilizeDuration": 1.75,
  "IsDisabled": false,
  "Key": "ability_warden_lock_down",
  "Name": "Binding Word",
  "Upgrades": [
    {
      "BulletArmorReduction": 20,
      "BulletArmorReductionDuration": 5
    },
    {
      "ImmobilizeDuration": 0.75
    },
    {
      "AbilityCooldown": -14,
      "SilenceDebuff": 1
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
    "card_name": "Binding Word",
    "hero_key": "hero_warden",
    "hero_name": "Warden",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_warden",
      "hero_name": "Warden",
      "lookup": "binding word",
      "name": "Binding Word",
      "type": "ability"
    }
  ]
}
````
