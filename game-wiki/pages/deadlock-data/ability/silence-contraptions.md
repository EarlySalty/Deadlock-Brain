---
title: "Silence Contraptions"
entity_type: "ability"
source: "deadlock_data"
external_id: "cadence_ability_silencecontraptions"
canonical_name: "Silence Contraptions"
snapshot_id: 39595
source_document_id: 7070
payload_hash: "a6618cf2427c9b02ff2153b4f50d159f242540016991b753b966e754caf84834"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.727626+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Silence Contraptions

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `cadence_ability_silencecontraptions`
- Snapshot ID: `39595`
- Source-Dokument: `7070`
- Kurzinfo: Silence Contraptions aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "DashDistance": 8,
  "DebuffDuration": 2,
  "IsDisabled": false,
  "Key": "cadence_ability_silencecontraptions",
  "MeleeEMP": 1,
  "Name": "Silence Contraptions",
  "SilenceContraptionsModifier": {
    "Class": "CadenceSilenceContraptions",
    "DebuffModifier": {
      "Class": "CadenceSilenceContraptionsDebuff",
      "Subclass": "CadenceSilenceContraptionsDebuff"
    },
    "Subclass": "CadenceSilenceContraptions"
  },
  "Upgrades": [
    {
      "SlowPercent": 40
    },
    {
      "DashDistance": 4
    },
    {
      "DebuffDuration": 1.5
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_cadence",
      "hero_name": "Cadence",
      "lookup": "silence contraptions",
      "name": "Silence Contraptions",
      "type": "ability"
    }
  ]
}
````
