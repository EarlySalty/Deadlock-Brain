---
title: "Disengaging Sigil"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_fencer_throwblade"
canonical_name: "Disengaging Sigil"
snapshot_id: 39435
source_document_id: 7070
payload_hash: "80e8bb9c64ec0e589cabcddd69c4fa16e7b1713c4b43818d6f27cc86c7f6d0d0"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.317702+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Disengaging Sigil

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_fencer_throwblade`
- Snapshot ID: `39435`
- Source-Dokument: `7070`
- Kurzinfo: Disengaging Sigil aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.5,
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AirDrag": 2.0,
  "AirSpeedMax": 70,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorMovement"
  ],
  "BuffModifier": {
    "Class": "Base",
    "Subclass": "FencerSigilBuff"
  },
  "ChannelMoveSpeed": 1.3,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 85
  },
  "DebuffModifier": {
    "Class": "SlowBase",
    "Subclass": "FencerSigilSlow"
  },
  "DisarmModifier": {
    "Class": "Base",
    "EnabledStateMask": [
      "Disarmed"
    ],
    "Subclass": "UpgradeGreaterWitheringWhipDebuff"
  },
  "FallSpeedMax": 1,
  "IsDisabled": false,
  "JumpVelocityHidden": 16,
  "Key": "ability_fencer_throwblade",
  "Name": "Disengaging Sigil",
  "SigilRadius": 6.5,
  "SlowDuration": 4,
  "SlowPercent": 30,
  "TraceToGroundDistance": 1000,
  "Upgrades": [
    {
      "BonusBulletSpeedPercent": 25,
      "BonusFireRate": 25,
      "BuffDuration": 8
    },
    {
      "ResetsAirLimit": 1,
      "StaminaToRestore": 1
    },
    {
      "RecastTime": 4
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
    "card_name": "Disengaging Sigil",
    "hero_key": "hero_fencer",
    "hero_name": "Apollo",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_fencer",
      "hero_name": "Apollo",
      "lookup": "disengaging sigil",
      "name": "Disengaging Sigil",
      "type": "ability"
    }
  ]
}
````
