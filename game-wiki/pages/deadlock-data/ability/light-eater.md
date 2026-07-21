---
title: "Light Eater"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_unicorn_radiantblast"
canonical_name: "Light Eater"
snapshot_id: 39554
source_document_id: 7070
payload_hash: "1958d6b77deba1102e7c5e667dc67f066e5905aff4223e893e2f6d58cd65ea52"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.622560+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Light Eater

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_unicorn_radiantblast`
- Snapshot ID: `39554`
- Source-Dokument: `7070`
- Kurzinfo: Light Eater aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.3,
  "AbilityCastRange": 10,
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.05
    },
    "Value": 8
  },
  "AbilityLifestealPercentHero": 20,
  "AbilityUnitTargetLimit": 100,
  "BehaviourBits": [
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorShowCastRangeAsSatSphereWhileCasting",
    "BehaviorUseLagCompensationForUnitTargeting",
    "BehaviorDontInterruptSlideOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.34
    },
    "Value": 15
  },
  "DebuffModifier": {
    "Class": "UnicornRadiantFlareDamage",
    "Subclass": "UnicornRadiantblastDebuff"
  },
  "ExtraSweepRadius": 2,
  "FlareDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.47
    },
    "Value": 40
  },
  "IsDisabled": false,
  "Key": "ability_unicorn_radiantblast",
  "Name": "Light Eater",
  "TargetingConeAngle": 70,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityLifestealPercentHero": 15
    },
    {
      "AbilityCastRange": 3,
      "AbilityCooldown": -10
    },
    {
      "Damage": {
        "Scale": {
          "Type": "spirit",
          "Value": 0.2
        },
        "Value": 25
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
    "card_name": "Light Eater",
    "hero_key": "hero_unicorn",
    "hero_name": "Celeste",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_unicorn",
      "hero_name": "Celeste",
      "lookup": "light eater",
      "name": "Light Eater",
      "type": "ability"
    }
  ]
}
````
