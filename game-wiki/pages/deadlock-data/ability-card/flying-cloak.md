---
title: "Flying Cloak"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "synth_plasma_flux"
canonical_name: "Flying Cloak"
snapshot_id: 39900
source_document_id: 7071
payload_hash: "891f2a70199b5f3c737e0dd1630237dcc5836dee3037be1260c17c9f6da2f51f"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.423117+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Flying Cloak

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `synth_plasma_flux`
- Snapshot ID: `39900`
- Source-Dokument: `7071`
- Kurzinfo: Flying Cloak aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": {
    "Name": "Cast Delay",
    "Type": "cast",
    "Value": 0.2
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 25.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "synth_plasma_flux_desc",
  "HeroKey": "hero_synth",
  "HeroName": "Pocket",
  "Info1": {
    "Alt": [
      {
        "Key": "MaxLifetime",
        "Name": "Lifetime",
        "Type": "duration",
        "Value": 3.8
      }
    ],
    "DescKey": "synth_plasma_flux_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 1.3
          },
          "Type": "tech_damage",
          "Value": 60
        }
      ]
    }
  },
  "Key": "synth_plasma_flux",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Flying Cloak",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "TickRate": {
      "Name": null,
      "Value": 0.1
    }
  },
  "Radius": {
    "Name": "Radius",
    "Type": "distance",
    "Value": 5
  },
  "Slot": "2",
  "Upgrades": [
    {
      "Damage": 70
    },
    {
      "DescKey": "synth_plasma_flux_t2_desc",
      "WeaponDamageBonus": 5,
      "WeaponDamageBonusDuration": 6
    },
    {
      "AbilityCooldown": -11,
      "DescKey": "synth_plasma_flux_t3_desc",
      "MaxLifetime": 1.6
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
      "hero_key": "hero_synth",
      "hero_name": "Pocket",
      "lookup": "flying cloak",
      "name": "Flying Cloak",
      "type": "ability"
    }
  ]
}
````
