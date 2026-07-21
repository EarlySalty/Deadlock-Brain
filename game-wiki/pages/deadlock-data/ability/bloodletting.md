---
title: "Bloodletting"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_shiv_defer_damage"
canonical_name: "Bloodletting"
snapshot_id: 39641
source_document_id: 7070
payload_hash: "0b51a554c9de2523efb9ae3167a49228087510c0ce465dc93c016d681f781cce"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.841821+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Bloodletting

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_shiv_defer_damage`
- Snapshot ID: `39641`
- Source-Dokument: `7070`
- Kurzinfo: Bloodletting aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 20.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoCastDelayModifier": {
    "Class": "Base",
    "Subclass": "Cast"
  },
  "BehaviourBits": [
    "BehaviorDamageDoesntWakeFromSleep",
    "BehaviorDontConsumeAbilityResourceOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "DamagePctDeferred": 25,
  "DamagePctDeferredMaxRage": 15,
  "DeferClearPct": 35,
  "DeferredDamageDuration": 6,
  "IsDisabled": false,
  "Key": "citadel_ability_shiv_defer_damage",
  "Name": "Bloodletting",
  "Upgrades": [
    {
      "AbilityCooldown": -10
    },
    {
      "DeferClearPct": 35
    },
    {
      "DamagePctDeferred": 15
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
    "card_name": "Bloodletting",
    "hero_key": "hero_shiv",
    "hero_name": "Shiv",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_shiv",
      "hero_name": "Shiv",
      "lookup": "bloodletting",
      "name": "Bloodletting",
      "type": "ability"
    }
  ]
}
````
