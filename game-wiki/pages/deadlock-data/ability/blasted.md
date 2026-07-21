---
title: "Blasted"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_punkgoat_blasted"
canonical_name: "Blasted"
snapshot_id: 39519
source_document_id: 7070
payload_hash: "e079cceef0fcf21a8539dfa7964b24ecb0ab4b9add79a55bb4656278a77a026f"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.531031+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Blasted

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_punkgoat_blasted`
- Snapshot ID: `39519`
- Source-Dokument: `7070`
- Kurzinfo: Blasted aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.28,
  "AbilityCooldown": 27,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "PunkgoatBlastedhealthwatcher",
      "Subclass": "Blastedhealthwatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorCooldownOnChannelEnd",
    "BehaviorAllowGunFireAfterCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "BlastedModifier": {
    "Class": "PunkgoatBlastedactive",
    "Subclass": "Blastedactive"
  },
  "BlastedPassiveModifier": {
    "Class": "PunkgoatBlastedpassive",
    "Subclass": "Blasted"
  },
  "BlastedRateOnBulletPct": 50,
  "BulletDamageAmp": 10,
  "BulletDamageAmpDuration": 7.0,
  "BulletsReloadedPerHeavyMeleePct": 100,
  "BulletsReloadedPerLightMeleePct": 35,
  "ChannelMoveSpeed": -1,
  "DurationPerHeavyMelee": 4.5,
  "DurationPerLightMelee": 2.8,
  "HealthBoostDuration": 11,
  "HealthDisplayModifier": {
    "Class": "Base",
    "Subclass": "Blastedhealthdisplay"
  },
  "HealthModifier": {
    "Class": "PunkgoatBlastedhealth",
    "Subclass": "Blastedhealth"
  },
  "IsDisabled": false,
  "Key": "ability_punkgoat_blasted",
  "LightMeleeScalePct": 40,
  "MaxDuration": 35.0,
  "MaxHealthMelee": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6
    },
    "Value": 70
  },
  "Name": "Blasted",
  "NonPlayerResourceScalePct": 25,
  "ShredModifier": {
    "Class": "PunkgoatBlastedshred",
    "Subclass": "Blastedshred"
  },
  "Upgrades": [
    {
      "BonusMoveSpeed": 2.25
    },
    {
      "BulletDamageAmp": 7,
      "GainSlamOnUse": 1
    },
    {
      "MaxHealthMelee": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.6
        },
        "Value": 50
      }
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
    "card_name": "Blasted",
    "hero_key": "hero_punkgoat",
    "hero_name": "Billy",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_punkgoat",
      "hero_name": "Billy",
      "lookup": "blasted",
      "name": "Blasted",
      "type": "ability"
    }
  ]
}
````
