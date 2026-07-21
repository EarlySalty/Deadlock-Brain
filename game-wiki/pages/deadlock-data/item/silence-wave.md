---
title: "Silence Wave"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_targeted_silence"
canonical_name: "Silence Wave"
snapshot_id: 40197
source_document_id: 7072
payload_hash: "323b9de78ec8b19de8ffad8325faed4112ad545079ddb94c153197db8d47c74b"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:25.144752+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Silence Wave

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_targeted_silence`
- Snapshot ID: `40197`
- Source-Dokument: `7072`
- Kurzinfo: Silence Wave aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusHealth": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownOnMiss": 30.0,
  "Cost": 3200,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.7
    },
    "Value": 75
  },
  "Description": "Launch an expanding projectile which <span class=\"highlight\">Silences</span> enemies for a short duration and deals impact damage. <br><br><span class=\"diminish\">Silence does not interrupt channeling abilities.</span>",
  "GrowthPerMeter": "0.15m",
  "HeightOffGround": "1m",
  "InitialWidth": "5.0m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_targeted_silence",
  "Name": "Silence Wave",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 75,
    "Damage": 125
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 3,
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/item-data.json",
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
      "hero_key": null,
      "hero_name": null,
      "lookup": "silence wave",
      "name": "Silence Wave",
      "type": "item"
    }
  ]
}
````
