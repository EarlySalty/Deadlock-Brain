---
title: "Gutshot"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_knockback"
canonical_name: "Gutshot"
snapshot_id: 39513
source_document_id: 7070
payload_hash: "dc1f69092dec5b8af20d1bf95bb065bede4e4c4c7e41677bdfe7d804ca0d5a35"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.518293+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Gutshot

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_knockback`
- Snapshot ID: `39513`
- Source-Dokument: `7070`
- Kurzinfo: Gutshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": 10,
  "AbilityCooldown": 23,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 99,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BonusDamage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 0.8
    },
    "Value": 30
  },
  "BonusKnockbackDistance": 3.5,
  "BuffModifier": {
    "Class": "PriestKnockbackBuff",
    "Subclass": "Buff"
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 0.7
    },
    "Value": 60
  },
  "DebuffModifier": {
    "Class": "Base",
    "Subclass": "Debuff"
  },
  "IsDisabled": false,
  "Key": "ability_priest_knockback",
  "KnockbackModifier": {
    "Class": "Priestknockback",
    "MomentumMaintained": 0.3,
    "Subclass": "PriestKnockback"
  },
  "KnockbackSpeed": 1200,
  "KnockbackToWallModifier": {
    "Class": "Priestknockback",
    "Subclass": "PriestKnockbacktowall"
  },
  "MaxPushForceHorizontal": 1400,
  "MaxPushForceVertical": 500,
  "Name": "Gutshot",
  "PushForce": 6,
  "SelfPushForce": 500,
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "Slow"
  },
  "StunDuration": 0.6,
  "TargetingConeAngle": 60,
  "Upgrades": [
    {
      "Damage": 25
    },
    {
      "AbilityCooldown": -10,
      "StunDuration": 0.4
    },
    {
      "BuffDuration": 5
    }
  ],
  "WallStunDistance": 7,
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
    "card_name": "Gutshot",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "gutshot",
      "name": "Gutshot",
      "type": "ability"
    }
  ]
}
````
