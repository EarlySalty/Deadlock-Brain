---
title: "Silktrap"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_trapper_webwall"
canonical_name: "Silktrap"
snapshot_id: 39548
source_document_id: 7070
payload_hash: "78ee7e42a32e0ea24f503d85b5649bcbdcd122c230cdc0e068d8df9aaa4a204c"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.606632+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Silktrap

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_trapper_webwall`
- Snapshot ID: `39548`
- Source-Dokument: `7070`
- Kurzinfo: Silktrap aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.22,
  "AbilityCastRange": 40,
  "AbilityCharges": 1,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": 1,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorAlwaysPreviewRadius",
    "BehaviorDisplaysDamageImpact",
    "BehaviorCanSetQuickCast"
  ],
  "ChannelMoveSpeed": -1,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.279
    },
    "Value": 40
  },
  "DebuffDuration": 3,
  "DebuffModifier": {
    "Class": "WebwallDebuff",
    "Subclass": "WebwallDebuff"
  },
  "IsDisabled": false,
  "Key": "ability_trapper_webwall",
  "MaxWallToWallDistance": 100,
  "MinWallToWallDistance": 3,
  "Name": "Silktrap",
  "Radius": 0.6,
  "SilenceModifier": {
    "Class": "CitadelSilenced",
    "Subclass": "WebwallSilence"
  },
  "SlowPercent": 99,
  "TickRate": 0.5,
  "Upgrades": [
    {
      "AbilityCharges": 1
    },
    {
      "WebDuration": 120
    },
    {
      "DebuffDuration": 2,
      "SilenceDuration": 2.5
    }
  ],
  "WebArmTime": 0.5,
  "WebDuration": 120,
  "WebWallTickRate": 0.15,
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
    "card_name": "Silktrap",
    "hero_key": "hero_trapper",
    "hero_name": "Trapper",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_trapper",
      "hero_name": "Trapper",
      "lookup": "silktrap",
      "name": "Silktrap",
      "type": "ability"
    }
  ]
}
````
