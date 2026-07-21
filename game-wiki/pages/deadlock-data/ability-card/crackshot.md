---
title: "Crackshot"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "ability_crackshot"
canonical_name: "Crackshot"
snapshot_id: 39750
source_document_id: 7071
payload_hash: "aeee41f188da30c16a3b2e190651919510dd6180b81cfcb8f4d485050a9c295b"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.129825+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Crackshot

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `ability_crackshot`
- Snapshot ID: `39750`
- Source-Dokument: `7071`
- Kurzinfo: Crackshot aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

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
    "Value": 20
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "ability_crackshot_desc",
  "HeroKey": "hero_astro",
  "HeroName": "Holliday",
  "Info1": {
    "Alt": [],
    "DescKey": "ability_crackshot_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.116
          },
          "Type": "bullet_damage",
          "Value": 55
        },
        {
          "Key": "FadingSlowPercent",
          "Name": "Fading Move Speed",
          "Type": "slow",
          "Value": 50
        },
        {
          "Key": "DebuffDuration",
          "Name": "Debuff Duration",
          "Type": "duration",
          "Value": 2
        }
      ]
    }
  },
  "Key": "ability_crackshot",
  "Name": "Crackshot",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "CrackshotNPCCDReduction": {
      "Name": null,
      "Value": 50
    }
  },
  "Range": {
    "ExplosionRadius": {
      "Name": "Explosion Radius",
      "Type": "distance",
      "Value": 2
    }
  },
  "Slot": "3",
  "Upgrades": [
    {
      "DescKey": "ability_crackshot_t1_desc",
      "FadingSlowPercent": 25
    },
    {
      "Damage": 49.5
    },
    {
      "AbilityCooldownPerHeadshot": -6,
      "AbilityCooldownPerHeadshotNPC": -3,
      "DescKey": "ability_crackshot_t3_desc"
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
      "hero_key": "hero_astro",
      "hero_name": "Holliday",
      "lookup": "crackshot",
      "name": "Crackshot",
      "type": "ability"
    }
  ]
}
````
