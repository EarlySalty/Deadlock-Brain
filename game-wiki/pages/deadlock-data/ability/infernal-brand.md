---
title: "Infernal Brand"
entity_type: "ability"
source: "deadlock_data"
external_id: "gunslinger_demonMark"
canonical_name: "Infernal Brand"
snapshot_id: 39684
source_document_id: 7070
payload_hash: "e0f36c22614bdadd806bf11080110cf465de3a69c5de281c93668c543f665c89"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.943994+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Infernal Brand

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `gunslinger_demonMark`
- Snapshot ID: `39684`
- Source-Dokument: `7070`
- Kurzinfo: Infernal Brand aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 40,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": null,
  "BonusFireRate": 25,
  "BonusMoveSpeed": 2,
  "BuffDuration": 3,
  "Damage": {
    "Scale": {
      "Type": "weapon_damage",
      "Value": 0.93
    },
    "Value": 100
  },
  "IsDisabled": false,
  "Key": "gunslinger_demonMark",
  "MarkDuration": 5,
  "MarkModifier": {
    "BuffModifier": {
      "Class": "Base",
      "Subclass": "DemonmarkBuff"
    },
    "Class": "GunslingerDemonmark",
    "Subclass": "Demonmark"
  },
  "Name": "Infernal Brand",
  "ProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.744
    },
    "Value": 100
  },
  "SearchAngle": 20,
  "SearchRadius": 20,
  "SearchRate": 1,
  "Upgrades": [
    {
      "AbilityCooldown": -3
    },
    {
      "ProcDamage": 50
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
    "card_name": "Infernal Brand",
    "hero_key": "hero_gunslinger",
    "hero_name": "Gunslinger",
    "slot": "3"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_gunslinger",
      "hero_name": "Gunslinger",
      "lookup": "infernal brand",
      "name": "Infernal Brand",
      "type": "ability"
    }
  ]
}
````
