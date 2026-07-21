---
title: "Gloom Bombs"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_nano_clustergrenade"
canonical_name: "Gloom Bombs"
snapshot_id: 39855
source_document_id: 7071
payload_hash: "a7030fbb41e4830b1997e7d7436aac7765e303c2890337a7dd3be7a786309fb2"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.335818+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Gloom Bombs

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_nano_clustergrenade`
- Snapshot ID: `39855`
- Source-Dokument: `7071`
- Kurzinfo: Gloom Bombs aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.15
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 14
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_nano_clustergrenade_desc",
  "HeroKey": "hero_nano",
  "HeroName": "Calico",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_nano_clustergrenade_desc",
    "Main": {
      "Props": [
        {
          "Key": "GrenadeCount",
          "Name": "Bomb Count",
          "Type": "cast",
          "Value": 4
        },
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.644
          },
          "Type": "tech_damage",
          "Value": 45
        },
        {
          "Key": "BonusDamageVsBarrier",
          "Name": "Bonus Damage vs Barriers",
          "Type": "tech_damage",
          "Value": 0
        }
      ]
    }
  },
  "Key": "ability_nano_clustergrenade",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Gloom Bombs",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GrenadeAngleVariance": {
      "Name": null,
      "Value": 0.08
    },
    "Lifetime": {
      "Name": "Lifetime",
      "Value": 0.75
    },
    "MultiHitPenaltyPercentage": {
      "Name": null,
      "Value": 65
    },
    "TimeBetweenGrenades": {
      "Name": null,
      "Value": 0.05
    },
    "TossSpeed": {
      "Name": null,
      "Value": 400
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 3.0
  },
  "Slot": "1",
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "DescKey": "ability_nano_clustergrenade_t2_desc",
      "MeleeResistReduction": -6,
      "MeleeResistReductionDuration": 6
    },
    {
      "GrenadeCount": 3
    }
  ],
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/ability-cards.json",
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_nano",
      "hero_name": "Calico",
      "lookup": "gloom bombs",
      "name": "Gloom Bombs",
      "type": "ability"
    }
  ]
}
````
