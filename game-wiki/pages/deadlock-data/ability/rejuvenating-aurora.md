---
title: "Rejuvenating Aurora"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_nikuman"
canonical_name: "Rejuvenating Aurora"
snapshot_id: 39631
source_document_id: 7070
payload_hash: "ef96e504fd501bc2f9bc4667dd578f2091486274072f740e8631194d5ff93560"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.818120+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Rejuvenating Aurora

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_nikuman`
- Snapshot ID: `39631`
- Source-Dokument: `7070`
- Kurzinfo: Rejuvenating Aurora aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityChannelTime": 5,
  "AbilityCooldown": 48.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AuraLingerDuration": 1.0,
  "BehaviourBits": [
    "BehaviorChannelled",
    "BehaviorNoTarget",
    "BehaviorCanHealPlayers",
    "BehaviorDisplaysDamageImpact",
    "BehaviorRequireAbilityButtonToCancel",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "HealingPerSecond": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 30
  },
  "IsDisabled": false,
  "Key": "citadel_ability_nikuman",
  "Name": "Rejuvenating Aurora",
  "NikumanModifier": {
    "Class": "Nikuman",
    "ProvidedByAura": {
      "Class": "Base",
      "Subclass": "HealTarget"
    },
    "Subclass": "Nikuman"
  },
  "ShareWithFriendsRadius": 8,
  "Upgrades": [
    {
      "MovementSpeedBonus": 4,
      "MovementSpeedBonusDuration": 8
    },
    {
      "AbilityChannelTime": 1.0,
      "AbilityCooldown": -20.0
    },
    {
      "HealMaxHealthPercent": 2.5,
      "NoChannel": 1
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
    "card_name": "Rejuvenating Aurora",
    "hero_key": "hero_dynamo",
    "hero_name": "Dynamo",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_dynamo",
      "hero_name": "Dynamo",
      "lookup": "rejuvenating aurora",
      "name": "Rejuvenating Aurora",
      "type": "ability"
    }
  ]
}
````
