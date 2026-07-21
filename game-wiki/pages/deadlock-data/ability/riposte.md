---
title: "Riposte"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_fencer_riposte"
canonical_name: "Riposte"
snapshot_id: 39433
source_document_id: 7070
payload_hash: "68dffef73ee5b8960c2a2a019d373ac500b713b432d8c72a73ce061346d962d4"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.313715+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Riposte

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_riposte`
- Snapshot ID: `39433`
- Source-Dokument: `7070`
- Kurzinfo: Riposte aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": 0.8,
  "AbilityCooldown": 22,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 50,
  "AbilityUnitTargetLimit": 1,
  "AutoChannelModifier": {
    "Class": "Base",
    "Subclass": "RiposteCastModifier"
  },
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "ChannelMoveSpeed": -1,
  "CounterattackAntiMashDelay": 0.2,
  "DamageThreshold": {
    "Scale": {
      "Type": "power_increase",
      "Value": 4
    },
    "Value": 60
  },
  "DampingFactor": 0.5,
  "DashGraceWindow": 1.3,
  "DashRadius": 2.2,
  "DashRange": 35,
  "DashSpeed": 76.2,
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "FencerRiposteAttackDebuff"
  },
  "IsDisabled": false,
  "Key": "ability_fencer_riposte",
  "LiftHeight": 240,
  "MeleeResistReduction": -22,
  "MeleeResistReductionDuration": 3.0,
  "MoveSpeedMax": 4,
  "Name": "Riposte",
  "ParryWindow": 0.3,
  "SideMoveSpeed": -100,
  "SlashConeAngle": 90,
  "SlashHalfWidth": 1,
  "SlashRadius": 6,
  "SlowDuration": 4,
  "SlowPercent": 40,
  "StunDuration": 0.6,
  "TurnRateMax": 10,
  "Upgrades": [
    {
      "AbilityCooldown": -8
    },
    {
      "MeleeResistReduction": -30
    },
    {
      "StunDuration": 1.6
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
    "card_name": "Riposte",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "riposte",
      "name": "Riposte",
      "type": "ability"
    }
  ]
}
````
