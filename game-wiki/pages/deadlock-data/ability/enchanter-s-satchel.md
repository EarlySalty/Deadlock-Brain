---
title: "Enchanter's Satchel"
entity_type: "ability"
source: "deadlock_data"
external_id: "synth_pulse"
canonical_name: "Enchanter's Satchel"
snapshot_id: 39712
source_document_id: 7070
payload_hash: "eac3ca8f573aeee7ee5db3d29b612ae441b95f235de5ada48727f5c2e967502c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.019466+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Enchanter's Satchel

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_pulse`
- Snapshot ID: `39712`
- Source-Dokument: `7070`
- Kurzinfo: Enchanter's Satchel aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityChannelTime": 1.5,
  "AbilityCooldown": 17.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 65
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "EscapeModifier": {
    "Class": "SynthPulseEscape",
    "EnabledStateMask": [
      "DashDisabled",
      "MantleDisabled",
      "DuckingDisabled",
      "MeleeDisabled"
    ],
    "Subclass": "SynthPulseEscape"
  },
  "FallSpeedMax": 0.0254,
  "IsDisabled": false,
  "Key": "synth_pulse",
  "Name": "Enchanter's Satchel",
  "Radius": 12,
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "Damage": 90
    },
    {
      "AbilityChannelTime": 1.5,
      "DebuffDuration": 4.0,
      "FireRateSlow": 40,
      "MoveSlowPercent": 40,
      "Radius": 4
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
    "card_name": "Enchanter's Satchel",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "enchanter's satchel",
      "name": "Enchanter's Satchel",
      "type": "ability"
    }
  ]
}
````
