---
title: "Affliction"
entity_type: "ability"
source: "deadlock_data"
external_id: "synth_affliction"
canonical_name: "Affliction"
snapshot_id: 39708
source_document_id: 7070
payload_hash: "2027406db26dda1635b9d0af4e5a731a3fc413f44c45c335980e933b50640433"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.009634+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Affliction

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_affliction`
- Snapshot ID: `39708`
- Source-Dokument: `7070`
- Kurzinfo: Affliction aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCooldown": 170.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "CanBePurged": 1,
  "ChannelMoveSpeed": 1.3,
  "CurrentHealthDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.22
    },
    "Value": 34
  },
  "DamageInterval": 0.5,
  "DebuffDuration": 11,
  "DebuffModifier": {
    "Class": "SynthAfflictionDebuff",
    "Subclass": "SynthAfflictionDebuff"
  },
  "IsDisabled": false,
  "Key": "synth_affliction",
  "Name": "Affliction",
  "Radius": 9,
  "Upgrades": [
    {
      "AbilityCooldown": -35
    },
    {
      "DebuffDuration": 3,
      "Radius": 4
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.11
        },
        "Value": 14
      },
      "DisableHealing": 1
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
    "card_name": "Affliction",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "affliction",
      "name": "Affliction",
      "type": "ability"
    }
  ]
}
````
