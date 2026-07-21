---
title: "Entangling Bola"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_werewolf_netshot"
canonical_name: "Entangling Bola"
snapshot_id: 39579
source_document_id: 7070
payload_hash: "5676d8cc98c3eaec8acf3a48e2d684a10ca623f91221fd599645e131ee21c2b1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.687534+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Entangling Bola

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_werewolf_netshot`
- Snapshot ID: `39579`
- Source-Dokument: `7070`
- Kurzinfo: Entangling Bola aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.24,
  "AbilityCooldown": 23,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectileFiredAsBullet",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusDebuffModifier": {
    "Class": "Base",
    "Subclass": "Bonusdebuff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.6
    },
    "Value": 40
  },
  "DebuffDuration": 1.5,
  "DebuffModifier": {
    "Class": "SlowBase",
    "EnabledStateMask": [
      "SilenceMovementAbilites",
      "SprintDisabled",
      "DashDisabledDebuff",
      "Slowed",
      "StaminaRegenPaused"
    ],
    "Subclass": "Debuff"
  },
  "IsDisabled": false,
  "Key": "ability_werewolf_netshot",
  "Name": "Entangling Bola",
  "RootModifier": {
    "Class": "CitadelRoot",
    "Subclass": "Root"
  },
  "SlowPercent": 20,
  "Upgrades": [
    {
      "SlowPercent": 25
    },
    {
      "AbilityCooldown": -8
    },
    {
      "DebuffDuration": 0.75,
      "RicochetCount": 2,
      "RicochetRange": 15
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
    "card_name": "Entangling Bola",
    "hero_key": "hero_werewolf",
    "hero_name": "Silver",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_werewolf",
      "hero_name": "Silver",
      "lookup": "entangling bola",
      "name": "Entangling Bola",
      "type": "ability"
    }
  ]
}
````
