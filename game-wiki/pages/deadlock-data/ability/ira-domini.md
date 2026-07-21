---
title: "Ira Domini"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_weaponswap"
canonical_name: "Ira Domini"
snapshot_id: 39518
source_document_id: 7070
payload_hash: "049b98b731cd7813d1424706929cf60cf9ab2f53622eec95dbd4891214443381"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.528340+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Ira Domini

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_weaponswap`
- Snapshot ID: `39518`
- Source-Dokument: `7070`
- Kurzinfo: Ira Domini aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCooldown": 160,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCannotCancelDuringChannel",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorRefundHalfCooldownOnChannelInterrupt"
  ],
  "BonusAmpToVampire": 5,
  "BonusDamage": {
    "Scale": {
      "Type": "power_increase",
      "Value": 3.0
    },
    "Value": 100
  },
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 1.5
    },
    "Value": 120
  },
  "ExecuteThreshold": 8,
  "ExplodeRadius": 0.2,
  "IsDisabled": false,
  "Key": "ability_priest_weaponswap",
  "Name": "Ira Domini",
  "PushForce": 500,
  "SelfModifier": {
    "Class": "PriestCrossbowequipped",
    "EnabledStateMask": [
      "AmmoChangesDisabled",
      "SinclairTaxUltActive"
    ],
    "Subclass": "Self"
  },
  "SlowModifier": {
    "Class": "DiminishingSlow",
    "Subclass": "BarrageSlowModifier"
  },
  "StakeCount": 3,
  "SwapEndDelay": 0.6,
  "Upgrades": [
    {
      "BonusMoveSpeed": 1.2
    },
    {
      "AbilityCooldown": -15,
      "BonusDamage": 65
    },
    {
      "AllStakesBlessed": 1
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
    "card_name": "Ira Domini",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "4"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "ira domini",
      "name": "Ira Domini",
      "type": "ability"
    }
  ]
}
````
