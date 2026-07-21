---
title: "Aura of Suffering"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_frank_painaura"
canonical_name: "Aura of Suffering"
snapshot_id: 39443
source_document_id: 7070
payload_hash: "faf001b3eccb74e2aa34a56cea5761bd84f6130869bbf9bd1812bec7f3c7f8a2"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.336865+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Aura of Suffering

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_frank_painaura`
- Snapshot ID: `39443`
- Source-Dokument: `7070`
- Kurzinfo: Aura of Suffering aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 2.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "AuraModifier": {
    "Class": "FrankPainaura",
    "DebuffModifier": {
      "Class": "FrankPainauraTarget",
      "Subclass": "Painauradebuff"
    },
    "Subclass": "Painaura"
  },
  "AuraOffModifier": {
    "Class": "Base",
    "Subclass": "Auraoff"
  },
  "BehaviourBits": [
    "BehaviorStartCooldownOnToggleOff",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorDoNotAllowSpamProc",
    "BehaviorCanCastOnZipline"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 0
  },
  "DebuffDuration": 0.5,
  "IsDisabled": false,
  "Key": "ability_frank_painaura",
  "MaxDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.72
    },
    "Value": 58
  },
  "MinDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.15
    },
    "Value": 13
  },
  "Name": "Aura of Suffering",
  "Radius": 8,
  "SelfDPS": 15,
  "SelfDamagePercentage": 70,
  "TickRate": 0.25,
  "ToggleOffDelay": 0.5,
  "Upgrades": [
    {
      "DebuffDuration": 0.5,
      "EnemyDashSlowPercent": -25,
      "SlowPercent": 25
    },
    {
      "MaxDPS": 34.0,
      "MinDps": 6
    },
    {
      "IncomingDamagePercent": 15,
      "Radius": 1
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
    "card_name": "Aura of Suffering",
    "hero_key": "hero_frank",
    "hero_name": "Victor",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_frank",
      "hero_name": "Victor",
      "lookup": "aura of suffering",
      "name": "Aura of Suffering",
      "type": "ability"
    }
  ]
}
````
