---
title: "Trapper's Delight"
entity_type: "ability"
source: "deadlock_data"
external_id: "ability_kali_trappers_bolo"
canonical_name: "Trapper's Delight"
snapshot_id: 39469
source_document_id: 7070
payload_hash: "b1795a64876a71a8160193103363e59797b9fa1bf2b8412b6c8bf8ea86ea84a1"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.408883+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Trapper's Delight

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `ability_kali_trappers_bolo`
- Snapshot ID: `39469`
- Source-Dokument: `7070`
- Kurzinfo: Trapper's Delight aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": 25,
  "AbilityCooldown": 127.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorDisplaysDamageImpact"
  ],
  "BoloBounceCount": 6,
  "BoloBounceSpeed": 800,
  "BoloContractRadius": 5,
  "BoloHitDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 25
  },
  "BoloProcDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.609336
    },
    "Value": 75
  },
  "BoloRadius": 0.8,
  "ChannelMoveSpeed": -1,
  "DebuffDelay": 2,
  "DebuffModifier": {
    "Class": "CitadelBolo",
    "ReverseLeechModifier": {
      "Class": "CitadelBoloLeech",
      "Subclass": "CitadelBoloLeech"
    },
    "Subclass": "CitadelBolo",
    "TrapModifier": {
      "Class": "CitadelRoot",
      "EnabledStateMask": [
        "GlowThroughWallsToProvider",
        "GlowToProvider"
      ],
      "Subclass": "CitadelRoot"
    }
  },
  "ImmobilizeDuration": 2.0,
  "IsDisabled": false,
  "Key": "ability_kali_trappers_bolo",
  "MaxGroundDashReduction": -50,
  "MaxSlow": 100,
  "Name": "Trapper's Delight",
  "Upgrades": [
    {
      "ReverseLifeLeech": 30,
      "ReverseLifeLeechDuration": 8
    },
    {
      "AbilityCooldown": -47.0
    },
    {
      "StunsTargets": 1
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
  "_deadlock_data_lookup": [
    {
      "hero_key": "hero_kali",
      "hero_name": "Kali",
      "lookup": "trapper's delight",
      "name": "Trapper's Delight",
      "type": "ability"
    }
  ]
}
````
