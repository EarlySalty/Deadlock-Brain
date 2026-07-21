---
title: "Bloodscent"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_drifter_hunger"
canonical_name: "Bloodscent"
snapshot_id: 39424
source_document_id: 7070
payload_hash: "c06ef8ffef5aafc9e8d2ea029cf97632bc2653bfc9c2ac734b64747562c293d1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.285040+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bloodscent

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_drifter_hunger`
- Snapshot ID: `39424`
- Source-Dokument: `7070`
- Kurzinfo: Bloodscent aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 80,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AmpDamagePercent": 15,
  "BehaviourBits": null,
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "DrifterHungerBuff"
  },
  "ChannelMoveSpeed": -1,
  "DelayBeforeInvisStarts": 0.6,
  "HealOnKillPct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "InvisDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "InvisFadeToDuration": 0.3,
  "InvisModifier": {
    "Class": "Invis",
    "Subclass": "Invis"
  },
  "IsDisabled": false,
  "IsolationAssistPercentValue": 100,
  "IsolationRange": 20,
  "Key": "ability_drifter_hunger",
  "KillDuration": 300,
  "LowHealthThreshold": 30,
  "MaxTrailTargets": 2,
  "Name": "Bloodscent",
  "RevealOnDamageDuration": 0.25,
  "RevealOnSpottedDuration": 1.5,
  "SpottedRadius": 15,
  "TargetLingerDuration": 3,
  "TargetModifier": {
    "Class": "HungerTarget",
    "Subclass": "HungerTarget"
  },
  "TickRate": 1,
  "TrailDuration": 10,
  "Upgrades": [
    {
      "BonusMoveSpeed": 3
    },
    {
      "HealOnKillPct": 24,
      "StaminaToRestore": 2
    },
    {
      "AmpDamagePercent": 12.0
    }
  ],
  "WeaponDmgPerIsolationKill": 3,
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
    "card_name": "Bloodscent",
    "hero_key": "hero_drifter",
    "hero_name": "Drifter",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_drifter",
      "hero_name": "Drifter",
      "lookup": "bloodscent",
      "name": "Bloodscent",
      "type": "ability"
    }
  ]
}
````
