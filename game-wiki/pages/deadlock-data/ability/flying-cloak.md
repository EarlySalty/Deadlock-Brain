---
title: "Flying Cloak"
entity_type: "ability"
source: "deadlock_data"
external_id: "synth_plasma_flux"
canonical_name: "Flying Cloak"
snapshot_id: 39710
source_document_id: 7070
payload_hash: "00c7e45b120cb4aa0b2cfc81c55f3b27c390d1d4ba28783e586ac578f50ff7eb"
source_content_hash: "2a14b7d15c5818dc88be1b26fa3d1edb2bfa8c532d09029ff7ebb4792f29f849"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/ability-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_ability-data.json.2a14b7d15c5818dc.json"
fetched_at: "2026-07-09T19:36:24.014973+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "ability"]
---

# Flying Cloak

## Kurzueberblick

- Typ: `ability`
- Quelle: `deadlock_data`
- External ID: `synth_plasma_flux`
- Snapshot ID: `39710`
- Source-Dokument: `7070`
- Kurzinfo: Flying Cloak aus `deadlock_data` / `ability` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "BehaviourBits": [
    "BehaviorProjectile",
    "BehaviorNoTarget",
    "BehaviorDisplaysDamageImpact",
    "BehaviorTriggerCancelMashProtectionOnCast",
    "BehaviorDontInterruptSlideOnCast"
  ],
  "ChannelMoveSpeed": -1,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.3
    },
    "Value": 60
  },
  "IsDisabled": false,
  "Key": "synth_plasma_flux",
  "MaxLifetime": 3.8,
  "Name": "Flying Cloak",
  "Radius": 5,
  "TickRate": 0.1,
  "Upgrades": [
    {
      "Damage": 70
    },
    {
      "WeaponDamageBonus": 5,
      "WeaponDamageBonusDuration": 6
    },
    {
      "AbilityCooldown": -11,
      "MaxLifetime": 1.6
    }
  ],
  "WeaponDamageBonusModifier": {
    "Class": "SynthPlasmaFluxWeaponDamage",
    "Subclass": "SynthPlasmaFluxWeaponDamage"
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
    "card_name": "Flying Cloak",
    "hero_key": "hero_synth",
    "hero_name": "Pocket",
    "slot": "2"
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
