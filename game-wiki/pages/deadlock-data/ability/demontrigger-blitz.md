---
title: "Demontrigger Blitz"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_gunslinger_salvo"
canonical_name: "Demontrigger Blitz"
snapshot_id: 39455
source_document_id: 7070
payload_hash: "92aef4dacdc8ec70cd164559354371e5055452eaf9fb7676c7f4e274adbea2eb"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.367545+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Demontrigger Blitz

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_gunslinger_salvo`
- Snapshot ID: `39455`
- Source-Dokument: `7070`
- Kurzinfo: Demontrigger Blitz aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 1,
  "AbilityCastRange": 60,
  "AbilityChannelTime": 1,
  "AbilityCooldown": 90,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisarmable",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCastableWhileDodging",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": 3.8,
  "Damage": null,
  "IsDisabled": false,
  "Key": "ability_gunslinger_salvo",
  "Name": "Demontrigger Blitz",
  "OverrideBulletRadius": 0.3,
  "ProcChance": 100,
  "ProcDamagePercentage": 220,
  "ProcWatcherModifier": {
    "Class": "SalvoBullet",
    "MaxBulletsToProcInShot": 1.0,
    "Subclass": "SalvoBulletWatcher"
  },
  "TickRate": 0.5,
  "TotalShots": 4,
  "Upgrades": [
    {
      "AbilityCooldown": -20
    },
    {
      "DebuffDuration": 6
    },
    {
      "TotalShots": 2
    }
  ],
  "VictimWarningModifier": {
    "Class": "Base",
    "Subclass": "GunslingerSalvoWarningModifier"
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
    "card_name": "Demontrigger Blitz",
    "hero_key": "hero_skyrunner",
    "hero_name": "Skyrunner",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "demontrigger blitz",
      "name": "Demontrigger Blitz",
      "type": "ability"
    }
  ]
}
````
