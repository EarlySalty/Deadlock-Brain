---
title: "Siphon Life"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_bull_heal"
canonical_name: "Siphon Life"
snapshot_id: 39598
source_document_id: 7070
payload_hash: "9fe7e3594f724ed4670891165897462273a6fedaf2adf898e3236ddd06a3221c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.736+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Siphon Life

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_bull_heal`
- Snapshot ID: `39598`
- Source-Dokument: `7070`
- Kurzinfo: Siphon Life aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "AuraRadius": 0.0,
    "Class": "BullHealAura",
    "ProvidedByAura": {
      "Class": "BullHealTarget",
      "Subclass": "BullHealTarget"
    },
    "Subclass": "BullHealAura"
  },
  "BehaviourBits": [
    "BehaviorCastableWhileBusy",
    "BehaviorNoTarget"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 22
  },
  "HealingFactor": 70,
  "IsDisabled": false,
  "Key": "citadel_ability_bull_heal",
  "Name": "Siphon Life",
  "NonHeroHealingFactor": 35,
  "Radius": 8,
  "TickRate": 0.25,
  "Upgrades": [
    {
      "AbilityCooldown": -20.0
    },
    {
      "AbilityDuration": 2
    },
    {
      "DPS": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.12
        },
        "Value": 18
      },
      "Radius": 2
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
    "card_name": "Siphon Life",
    "hero_key": "hero_atlas",
    "hero_name": "Abrams",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_atlas",
      "hero_name": "Abrams",
      "lookup": "siphon life",
      "name": "Siphon Life",
      "type": "ability"
    }
  ]
}
````
