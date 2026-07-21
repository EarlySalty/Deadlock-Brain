---
title: "Consecrating Grenade"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_priest_flashbang"
canonical_name: "Consecrating Grenade"
snapshot_id: 39512
source_document_id: 7070
payload_hash: "f83ae2d7811f69d0a99b5aca2c92254e860f3d896b9e2d4a14fb4dcf58ba9cb6"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.515752+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Consecrating Grenade

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_priest_flashbang`
- Snapshot ID: `39512`
- Source-Dokument: `7070`
- Kurzinfo: Consecrating Grenade aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.03,
  "AbilityCooldown": 25,
  "AbilityPostCastDuration": 0.15,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BounceGrenadeSpeed": 1100,
  "BounceLifetime": 0.5,
  "BurnDuration": 3.5,
  "BurnLingerDuration": 0.15,
  "BurnRadius": 4.5,
  "CameraTurnRateMax": 15,
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.6
    },
    "Value": 10
  },
  "Damage": {
    "Scale": {
      "Type": "weapon_damage_increase",
      "Value": 1.0
    },
    "Value": 35
  },
  "EnemyDebuffModifier": {
    "AuraRadius": 0.0,
    "Class": "PriestFlashbangburnaura",
    "FlashFadeOutTime": 0.1,
    "ProvidedByAura": {
      "Class": "PriestFlashbangburn",
      "Subclass": "Burn"
    },
    "Subclass": "Burnaura"
  },
  "HealAmpReceivePenaltyPercent": -30,
  "HealAmpRegenPenaltyPercent": -30,
  "IsDisabled": false,
  "Key": "ability_priest_flashbang",
  "Name": "Consecrating Grenade",
  "PreBounceLifetime": 15,
  "Radius": 4.5,
  "TickRate": 0.2,
  "Upgrades": [
    {
      "AbilityCooldown": -5
    },
    {
      "BurnDuration": 1.0,
      "BurnRadius": 1.5,
      "Radius": 1.5
    },
    {
      "AbilityCharges": 1,
      "AbilityCooldownBetweenCharge": 3,
      "HealAmpReceivePenaltyPercent": -20,
      "HealAmpRegenPenaltyPercent": -20
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
    "card_name": "Consecrating Grenade",
    "hero_key": "hero_priest",
    "hero_name": "Venator",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_priest",
      "hero_name": "Venator",
      "lookup": "consecrating grenade",
      "name": "Consecrating Grenade",
      "type": "ability"
    }
  ]
}
````
