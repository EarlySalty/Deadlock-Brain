---
title: "Lethal Venom"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_viper_venom"
canonical_name: "Lethal Venom"
snapshot_id: 39568
source_document_id: 7070
payload_hash: "ad8e1a45c522be6ba99f6bd02d08a768c3260e7b82a48417605dbe38fa1a7221"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.656742+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Lethal Venom

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_viper_venom`
- Snapshot ID: `39568`
- Source-Dokument: `7070`
- Kurzinfo: Lethal Venom aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.2
    },
    "Value": 10
  },
  "AbilityCooldown": 28.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "AutoIntrinsicModifiers": [
    {
      "Class": "CitadelViperVenomProcWatcher",
      "Subclass": "VenomProcWatcher"
    }
  ],
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact",
    "BehaviorUseInstantCastUnitTargetUi",
    "BehaviorCanSetQuickCast"
  ],
  "BuildUpDuration": 5,
  "BuildUpModifier": {
    "BuildUpDecayDelay": 2.0,
    "Class": "CitadelBaseBuildup",
    "Subclass": "CitadelBaseBuildup"
  },
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "ability_viper_venom",
  "Name": "Lethal Venom",
  "Upgrades": [
    {
      "VenomMaxDamage": 31.5
    },
    {
      "AbilityCooldown": -12,
      "HealAmpReceivePenaltyPercent": -40,
      "HealAmpRegenPenaltyPercent": -40
    },
    {
      "BuildUpPerShot": 4.5
    }
  ],
  "VenomBuildupPerShot": 1,
  "VenomDuration": 3,
  "VenomMaxDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.79
    },
    "Value": 140
  },
  "VenomMaxDamageHealthPercentage": 30,
  "VenomMinDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.651
    },
    "Value": 20
  },
  "VenomMinDamageHealthPercentage": 100,
  "VenomModifier": {
    "Class": "ViperVenom",
    "Subclass": "ViperVenomModifierSubclass"
  },
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
    "card_name": "Lethal Venom",
    "hero_key": "hero_viper",
    "hero_name": "Vyper",
    "slot": "2"
  },
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_viper",
      "hero_name": "Vyper",
      "lookup": "lethal venom",
      "name": "Lethal Venom",
      "type": "ability"
    }
  ]
}
````
