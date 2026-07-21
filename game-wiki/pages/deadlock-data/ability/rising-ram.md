---
title: "Rising Ram"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_punkgoat_goatflip"
canonical_name: "Rising Ram"
snapshot_id: 39520
source_document_id: 7070
payload_hash: "c7e6781570a2295d2c05eb87cf663b9029cd4caa13d98e8a7ece1bc11cf6972f"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.533669+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rising Ram

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_goatflip`
- Snapshot ID: `39520`
- Source-Dokument: `7070`
- Kurzinfo: Rising Ram aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCooldown": 32,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 0.3,
  "AbilityUnitTargetLimit": 1,
  "AirControlAccelPercent": 50.0,
  "AirControlDashReductionPct": -70.0,
  "AirControlDebuffDuration": 1.5,
  "AirControlPercent": 50.0,
  "AllowRamMultiple": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontTriggerPostCastOnCastComplete",
    "BehaviorMovement",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDeactivateCrouchToggleOnCast"
  ],
  "CameraTurnRateMax": 188,
  "ChargeMultiHitRadius": 1.5,
  "ChargeRadius": 2.54,
  "ChargeSpeed": 1200,
  "ChargeStrikeDistance": 165,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.9
    },
    "Value": 40
  },
  "DealMaxHealthDamagePct": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0
    },
    "Value": 0
  },
  "GoingBackAwaySpeed": -100,
  "GoingUpDistance": 3.1,
  "GoingUpEnemyDistancePercent": 95,
  "GoingUpSpeed": 430,
  "HoverGravityScale": 0.75,
  "IsDisabled": false,
  "Key": "ability_punkgoat_goatflip",
  "KnockAwaySpeed": 170,
  "Name": "Rising Ram",
  "NearbyHeroKillDistance": 10,
  "ReduceCooldownOnHitPct": 50,
  "TimeBeforeGoUpForLagComp": 0.1,
  "TimeGoingUpEnemy": 0.2,
  "Upgrades": [
    {
      "WeaponDamageBurst": 25,
      "WeaponDamageBurstDuration": 5
    },
    {
      "AbilityDuration": 0.4
    },
    {
      "AbilityCooldown": -13,
      "DealMaxHealthDamagePct": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.035
        },
        "Value": 8
      }
    }
  ],
  "WorldImpactRadius": 25,
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
    "card_name": "Rising Ram",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "rising ram",
      "name": "Rising Ram",
      "type": "ability"
    }
  ]
}
````
