---
title: "Skipshot"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_boho_bouncyprojectile"
canonical_name: "Skipshot"
snapshot_id: 39400
source_document_id: 7070
payload_hash: "7fff562432e093d771b9bd51b9d6b37888dca6bf8524a9b51888580f58ef1263"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.230095+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Skipshot

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_boho_bouncyprojectile`
- Snapshot ID: `39400`
- Source-Dokument: `7070`
- Kurzinfo: Skipshot aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": 14,
  "AbilityCharges": 1,
  "AbilityCooldown": 15,
  "AbilityCooldownBetweenCharge": 7,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorProjectilePassThroughWorld",
    "BehaviorProjectileFiredAsBullet"
  ],
  "BounceCount": 3,
  "BounceRadius": 18,
  "ChannelMoveSpeed": -1,
  "CooldownReductionPercentagePerHit": 15,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "ability_boho_bouncyprojectile",
  "Name": "Skipshot",
  "Upgrades": [
    {
      "AbilityCooldown": -2
    },
    {
      "Damage": 18.0
    },
    {
      "BounceCount": 2
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
    "card_name": "Skipshot",
    "hero_key": "hero_boho",
    "hero_name": "Boho",
    "slot": "1"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_boho",
      "hero_name": "Boho",
      "lookup": "skipshot",
      "name": "Skipshot",
      "type": "ability"
    }
  ]
}
````
