---
title: "Tail Whack"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_cripplingslash"
canonical_name: "Tail Whack"
snapshot_id: 39573
source_document_id: 7070
payload_hash: "9b76d7a5bab18efef63e13fc06bb85e22f6ae529950f39b76dc12a93e8f78539"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.673811+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Tail Whack

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_cripplingslash`
- Snapshot ID: `39573`
- Source-Dokument: `7070`
- Kurzinfo: Tail Whack aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityPostCastDuration": 0.2,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.5
    },
    "Value": 45
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "EnabledStateMask": [
      "Slowed"
    ],
    "Subclass": "Debuff"
  },
  "DisarmDuration": 2.0,
  "DisarmModifier": {
    "Class": "CitadelDisarmed",
    "Subclass": "Disarm"
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_cripplingslash",
  "LeftForce": 400,
  "Name": "Tail Whack",
  "PushForce": 300,
  "SlashHeight": 3,
  "SlashRadius": 10,
  "SlowDuration": 2.0,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "SlowPercent": 30,
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "SlowPercent": 40
    },
    {
      "DisarmDuration": 1.5,
      "SlowDuration": 1.5
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
    "card_name": "Tail Whack",
    "hero_key": "hero_werewolf_transformed",
    "hero_name": "Silver (Transformed)",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf_transformed",
      "hero_name": "Silver (Transformed)",
      "lookup": "tail whack",
      "name": "Tail Whack",
      "type": "ability"
    }
  ]
}
````
