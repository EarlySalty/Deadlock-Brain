---
title: "Patron Laser"
entity_type: "ability"
source: "deadlock_data"
external_id: "citadel_ability_tier3boss_laser_beam"
canonical_name: "Patron Laser"
snapshot_id: 39659
source_document_id: 7070
payload_hash: "c3cabd5c6967a0ca852a60fa43d1e89d313002fc943f735910121d1e1cc899a9"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:23.885249+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Patron Laser

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `citadel_ability_tier3boss_laser_beam`
- Snapshot ID: `39659`
- Source-Dokument: `7070`
- Kurzinfo: Patron Laser aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastRange": 30.48,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BeamModifier": {
    "AuraDropTickRate": 0.5,
    "Class": "Tier3bossElectricBeam",
    "GroundAuraModifier": {
      "AuraRadius": 160.0,
      "Class": "Tier3bossLaserAura",
      "ModifierProvidedByAuraDuration": 4.0,
      "ProvidedByAura": {
        "Class": "Tier3bossLaserDebuff",
        "Duration": 4,
        "MaxHealthDPS": 4,
        "NPCDPS": 80,
        "PlayerDPS": 100,
        "Subclass": "Tier3bossLaserDebuff",
        "TickRate": 0.5
      },
      "Subclass": "Tier3bossLaserAura"
    },
    "LaserDPSMaxHealth": 5,
    "LaserDPSToNPCs": 80.0,
    "LaserDPSToPlayers": 440.0,
    "Subclass": "Tier3bossElectricBeam"
  },
  "BehaviourBits": [
    "BehaviorNoTarget"
  ],
  "BulletArmorReduction": -10,
  "ChannelMoveSpeed": -1,
  "IsDisabled": false,
  "Key": "citadel_ability_tier3boss_laser_beam",
  "Name": "Patron Laser",
  "PathLength": 2000,
  "PathWidth": 40,
  "TechArmorReduction": -10,
  "Upgrades": [],
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
  }
}
````
