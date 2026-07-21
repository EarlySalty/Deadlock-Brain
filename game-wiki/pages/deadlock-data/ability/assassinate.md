---
title: "Assassinate"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_hornet_snipe"
canonical_name: "Assassinate"
snapshot_id: 39617
source_document_id: 7070
payload_hash: "5e3c651ee71c60c7caa58cac0b52332ee05e05a989759edde56bacecfcb32db6"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.785026+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Assassinate

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_hornet_snipe`
- Snapshot ID: `39617`
- Source-Dokument: `7070`
- Kurzinfo: Assassinate aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCharges": 2,
  "AbilityCooldown": 55.0,
  "AbilityCooldownBetweenCharge": 2.5,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact"
  ],
  "BonusGoldOnKill": 250,
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 90
  },
  "GlowEnemyModifier": {
    "Class": "LowHealthGlow",
    "EnabledStateMask": [
      "AssassinateLowhealthTarget"
    ],
    "Subclass": "LowHealthGlow"
  },
  "HeadshotBonus": 20,
  "IsDisabled": false,
  "Key": "citadel_ability_hornet_snipe",
  "KillCheckModifier": {
    "Class": "Base",
    "Subclass": "KillcheckModifier"
  },
  "LowHealthEnemyDamageBonus": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.3
    },
    "Value": 90
  },
  "LowHealthEnemyThresholdPct": 50,
  "MaxSoundDistance": 2000,
  "MinChargeDamagePercent": 50,
  "MoveSpeed": 4,
  "Name": "Assassinate",
  "Range": 1000,
  "ShotRadius": 4.0,
  "SnipeModifier": {
    "Class": "CitadelHornetSnipe",
    "Subclass": "CitadelHornetSnipe"
  },
  "TimeToFullCharge": 1.0,
  "Upgrades": [
    {
      "AbilityCooldown": -15.0
    },
    {
      "LowHealthEnemyDamageBonus": 80
    },
    {
      "WeaponDamageBonusPerKill": 4
    }
  ],
  "ViewPunch": 2.5,
  "WeaponDamageBonusPerKill": 6,
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
    "card_name": "Assassinate",
    "hero_key": "hero_hornet",
    "hero_name": "Vindicta",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_hornet",
      "hero_name": "Vindicta",
      "lookup": "assassinate",
      "name": "Assassinate",
      "type": "ability"
    }
  ]
}
````
