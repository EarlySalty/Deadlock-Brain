---
title: "Arctic Blast"
entity_type: "item"
source: "deadlock_data"
external_id: "upgrade_arctic_blast"
canonical_name: "Arctic Blast"
snapshot_id: 39996
source_document_id: 7072
payload_hash: "9ebc10a2f199195fed0ff0e59032ce4a9b803e2b0653e57afd8cb700a20856a1"
source_content_hash: "b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65"
source_url: "https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json"
source_raw_path: "data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json"
fetched_at: "2026-07-09T19:36:24.628648+00:00"
generated_at: "2026-07-21T17:48:52.496692210+00:00"
tags: ["deadlock", "game-knowledge", "item"]
---

# Arctic Blast

## Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_arctic_blast`
- Snapshot ID: `39996`
- Source-Dokument: `7072`
- Kurzinfo: Arctic Blast aus `deadlock_data` / `item` mit vollstaendiger Payload.

## Vollstaendige Payload

````json
{
  "AbilityCooldown": 24.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_cold_front"
  ],
  "Cost": 6400,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.6975
    },
    "Value": 175
  },
  "DamageHeight": "7m",
  "Description": "Release an expanding ice blast that deals {g:citadel_inline_attribute:'SpiritDamage'}, <span class=\"highlight\">Freezing</span> and then <span class=\"highlight\">Slowing</span> targets it hits.<br><br>Slowed targets have their <span class=\"highlight\">stamina regen frozen</span>",
  "EndRadius": "16m",
  "FreezeDuration": 1,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_arctic_blast",
  "NPCDamageMult": 1,
  "Name": "Arctic Blast",
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "Damage": 150,
    "FreezeDuration": 0.25,
    "TechResist": 15
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption",
    "Durability"
  ],
  "Slot": "Tech",
  "SlowDuration": 4,
  "SlowPercent": 60,
  "SpreadDuration": 0.6,
  "StartRadius": "2m",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechResist": 10,
  "Tier": 4,
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
      "lookup": "arctic blast",
      "name": "Arctic Blast",
      "type": "item"
    }
  ]
}
````
