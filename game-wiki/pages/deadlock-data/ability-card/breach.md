---
title: "Breach"
entity_type: "ability_card"
source: "deadlock_data"
external_id: "fathom_breach"
canonical_name: "Breach"
snapshot_id: 39892
source_document_id: 7071
payload_hash: "b0a290c1e1ba1fbbd40c9a60437ae4af66c0c06884c074c4d4721362a7c07c01"
source_content_hash: "1265c6bc5f8d6fd25b08c9f92232827e0591a9c4b90810ef6f6bec49b8141b7c"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-cards.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-cards.json.1265c6bc5f8d6fd2.json"
fetched_at: "2026-07-09T19:36:24.406570+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability_card"]
---

# Breach

## Kurzueberblick

- Typ: `ability_card`
- Quelle: `deadlock_data`
- External ID: `fathom_breach`
- Snapshot ID: `39892`
- Source-Dokument: `7071`
- Kurzinfo: Breach aus `deadlock_data` / `ability_card` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Name": "Cast Range",
    "Type": "range",
    "Value": 20
  },
  "AbilityCooldown": {
    "Name": "Cooldown",
    "Type": "cooldown",
    "Value": 22.0
  },
  "AbilityCooldownBetweenCharge": {
    "Name": "Charge Delay",
    "Type": "charge_cooldown",
    "Value": -1.0
  },
  "DescKey": "fathom_breach_desc",
  "HeroKey": "hero_slork",
  "HeroName": "Fathom",
  "Info1": {
    "Alt": [],
    "DescKey": "fathom_breach_desc",
    "Main": {
      "Props": [
        {
          "Key": "Damage",
          "Name": "Damage",
          "Scale": {
            "Type": "spirit",
            "Value": 0.651
          },
          "Type": "tech_damage",
          "Value": 80
        }
      ]
    }
  },
  "Key": "fathom_breach",
  "Move": {
    "ChannelMoveSpeed": {
      "Name": "Channel Move Speed",
      "Type": "move_speed",
      "Value": -1
    }
  },
  "Name": "Breach",
  "Other": {
    "AbilityUnitTargetLimit": {
      "Name": null,
      "Value": 1
    },
    "GravityScale": {
      "Name": "Gravity Scale",
      "Value": 1.4
    },
    "TossSpeed": {
      "Name": null,
      "Value": 350
    },
    "WallImpactLookAheadDistance": {
      "Name": null,
      "Value": 100
    }
  },
  "Range": {
    "ExplosionRadius": {
      "Name": "Explosion Radius",
      "Type": "distance",
      "Value": 6
    }
  },
  "Slot": "2",
  "Upgrades": [
    {
      "ExplosionRadius": 3
    },
    {
      "AbilityCooldown": -8
    },
    {
      "Damage": 120
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
      "hero_key": "hero_slork",
      "hero_name": "Fathom",
      "lookup": "breach",
      "name": "Breach",
      "type": "ability"
    }
  ]
}
````
