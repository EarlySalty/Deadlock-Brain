---
title: "item"
generated_at: "2026-07-21T18:03:55.468852924+00:00"
entries: 175
---

# item

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_active_reload" title="Active Reload" -->

## Active Reload

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_active_reload`
- Snapshot ID: `39981`
- Source-Dokument: `7072`
- Kurzinfo: Active Reload aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Active Reload`
- Payload Hash: `15dbb1f76530b22ef23c593f26b16344cff486807a0310c1fadc4b9260bfe649`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.594161+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 12,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusClipSizePercent": 20,
  "BonusFireRate": 25,
  "BonusMoveSpeed": "0.75m",
  "BulletLifestealPercent": 16,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "While reloading, pressing {g:citadel_binding:'Reload'} during the highlighted portion will <span class=\"highlight\">instantly finish your reload</span> and grant you <span class=\"highlight\">Fire Rate</span>, <span class=\"highlight\">Bullet Lifesteal</span> and <span class=\"highlight\">Move Speed</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_active_reload",
  "Name": "Active Reload",
  "PropertyUpgrades": {
    "BonusClipSizePercent": 10,
    "BonusFireRate": 15,
    "BonusMoveSpeed": "3m",
    "BulletLifestealPercent": 12
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "active reload",
      "name": "Active Reload",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_thermal_detonator" title="Alchemical Fire" -->

## Alchemical Fire

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_thermal_detonator`
- Snapshot ID: `40205`
- Source-Dokument: `7072`
- Kurzinfo: Alchemical Fire aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Alchemical Fire`
- Payload Hash: `33a49678ce9a4859035cdd1a31042889ae0e9603c3876198a6d299a012a006e4`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.164857+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BulletArmorReduction": {
    "Scale": {
      "Type": "spirit",
      "Value": -0.055
    },
    "Value": -7
  },
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.2
    },
    "Value": 45
  },
  "DPSIncrease": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 7
  },
  "DPSMax": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 95
  },
  "Description": "Throw a flask that explodes on contact, creating an area that does increasing {g:citadel_inline_attribute:'SpiritDamage'} <span class=\"highlight\">per second</span> and reduces enemy <span class=\"highlight\">Bullet Resist</span>.<br><br>50% less effective vs non-heroes.",
  "HeightOffGround": 50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_thermal_detonator",
  "Name": "Alchemical Fire",
  "NonHeroReductionPercent": 50,
  "PropertyUpgrades": {
    "BulletArmorReduction": -8,
    "DPS": 30,
    "DPSMax": 30,
    "SpiritPower": 15
  },
  "Radius": "10m",
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "SpiritPower": 10,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TickRate": 0.5,
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
      "lookup": "alchemical fire",
      "name": "Alchemical Fire",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_arcane_surge" title="Arcane Surge" -->

## Arcane Surge

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_surge`
- Snapshot ID: `39995`
- Source-Dokument: `7072`
- Kurzinfo: Arcane Surge aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Arcane Surge`
- Payload Hash: `de5c1d685a4cf5c406afc24b395a92b0515e8ec1ae0d607806cbc5dc6e981955`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.626513+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityDurationPercent": 15,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 1600,
  "Description": "After you <span class=\"highlight\">Dash-Jump</span>, the <span class=\"highlight\">next ability you use</span> within 7s will have bonus <span class=\"highlight\">Range, Duration,</span> and <span class=\"highlight\">Spirit Power</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_arcane_surge",
  "Name": "Arcane Surge",
  "PropertyUpgrades": {
    "BonusAbilityDurationPercent": 15,
    "SpiritPower": 25,
    "Stamina": 1,
    "StaminaCooldownReduction": 14,
    "TechRadiusMultiplierBuff": 15,
    "TechRangeMultiplierBuff": 15
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate",
    "ClipSize",
    "Movement",
    "Durability"
  ],
  "Slot": "Tech",
  "SpiritPower": 20,
  "Stamina": 1,
  "StaminaCooldownReduction": 12,
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechRadiusMultiplierBuff": 12,
  "TechRangeMultiplierBuff": 12,
  "Tier": 2,
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
      "lookup": "arcane surge",
      "name": "Arcane Surge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_arctic_blast" title="Arctic Blast" -->

## Arctic Blast

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_arctic_blast`
- Snapshot ID: `39996`
- Source-Dokument: `7072`
- Kurzinfo: Arctic Blast aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Arctic Blast`
- Payload Hash: `9ebc10a2f199195fed0ff0e59032ce4a9b803e2b0653e57afd8cb700a20856a1`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.628648+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_aprounds" title="Armor Piercing Rounds" -->

## Armor Piercing Rounds

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_aprounds`
- Snapshot ID: `39991`
- Source-Dokument: `7072`
- Kurzinfo: Armor Piercing Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Armor Piercing Rounds`
- Payload Hash: `0ea6359f2250329ce4e4baf27a0bb0a6db7b99125d897880d42a24e710271311`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.618746+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BonusBulletSpeedPercent": 60,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 6400,
  "Description": "Your Bullets have a chance to become unavoidable, <span class=\"highlight\">piercing through</span> enemies and <span class=\"highlight\">ignoring their Bullet Resistance</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_aprounds",
  "Name": "Armor Piercing Rounds",
  "ProcChance": 55,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 30,
    "BonusBulletSpeedPercent": 55,
    "ProcChance": 20
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "armor piercing rounds",
      "name": "Armor Piercing Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_bulletshredimbue" title="Ballistic Enchantment" -->

## Ballistic Enchantment

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_bulletshredimbue`
- Snapshot ID: `40013`
- Source-Dokument: `7072`
- Kurzinfo: Ballistic Enchantment aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ballistic Enchantment`
- Payload Hash: `fdfb0b18555f514df85eebfbb4bdd641a440e3f2c6b99f06c553dcd8d7cd33f6`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.664929+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 14,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_reach"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with increased <span class=\"highlight\">range</span>. Dealing damage with that ability grants you increased <span class=\"highlight\">weapon damage</span> per <span class=\"highlight\">unique hero hit</span>. Has reduced effect on non-heroes.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_bulletshredimbue",
  "Name": "Ballistic Enchantment",
  "NonHeroStackLimit": 8,
  "PropertyUpgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15,
    "WeaponPowerPerStack": 15
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechRadiusMultiplier": 22,
  "TechRangeMultiplier": 22,
  "Tier": 3,
  "WeaponPowerPerStack": 20,
  "WeaponPowerPerStackNonHero": 5,
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
      "lookup": "ballistic enchantment",
      "name": "Ballistic Enchantment",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_regenerating_bullet_shield" title="Battle Vest" -->

## Battle Vest

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_regenerating_bullet_shield`
- Snapshot ID: `40153`
- Source-Dokument: `7072`
- Kurzinfo: Battle Vest aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Battle Vest`
- Payload Hash: `1378260fa1488897f73a97cd17396c9e0f28a856b3062e3d4918ad7f4ace2865`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.022602+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 18,
  "BonusFireRate": 7,
  "BulletResist": 18,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "While you are <span class=\"highlight\">above 65% health</span>, gain <span class=\"highlight\">{g:citadel_inline_attribute:'WeaponDamage'}</span> and <span class=\"highlight\">{g:citadel_inline_attribute:'BonusFireRate'}</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_regenerating_bullet_shield",
  "LifeThreshold": 65,
  "Name": "Battle Vest",
  "OutOfCombatHealthRegen": 3,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusFireRate": 8,
    "BulletResist": 12,
    "OutOfCombatHealthRegen": 3
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "battle vest",
      "name": "Battle Vest",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_berserker" title="Berserker" -->

## Berserker

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_berserker`
- Snapshot ID: `40003`
- Source-Dokument: `7072`
- Kurzinfo: Berserker aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Berserker`
- Payload Hash: `2a9f0b5c0d0d5d8256c53276fba7813471cac60c7302fd1fa19332dd5c029f19`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.642994+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletResist": 8,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DamageDuration": 10,
  "DamageToStack": 120,
  "Description": "Your <span class=\"highlight\">Weapon Damage</span> increases as you take sustained damage.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_berserker",
  "MaxStacks": 10,
  "Name": "Berserker",
  "PropertyUpgrades": {
    "BulletResist": 8,
    "MaxStacks": 8,
    "WeaponPowerPerStack": 3
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 3,
  "WeaponPowerPerStack": 7,
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
      "lookup": "berserker",
      "name": "Berserker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_blood_tribute" title="Blood Tribute" -->

## Blood Tribute

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_blood_tribute`
- Snapshot ID: `40005`
- Source-Dokument: `7072`
- Kurzinfo: Blood Tribute aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Blood Tribute`
- Payload Hash: `da5fbda709b6c9a51bcc688ded00e2e17d9c4cbf0493c06aad32c31cc4459864`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.647654+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCastToggle",
  "BonusFireRate": 35,
  "BonusMoveSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Toggle: Continually sacrifice Health to improve {g:citadel_inline_attribute:'FireRate'}, <span class=\"highlight\">Debuff Resistance</span> and <span class=\"highlight\">Move Speed</span>.",
  "HealthDrainedPerSecond": 50,
  "InnateStatusResistancePercent": 8,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_blood_tribute",
  "Name": "Blood Tribute",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "BonusFireRate": 30,
    "HealthDrainedPerSecond": -20,
    "OutOfCombatHealthRegen": 8,
    "TechResist": 14
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StatusResistancePercent": 35,
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 8,
  "TickRate": 0.1,
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
      "lookup": "blood tribute",
      "name": "Blood Tribute",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_boundless_spirit" title="Boundless Spirit" -->

## Boundless Spirit

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_boundless_spirit`
- Snapshot ID: `40007`
- Source-Dokument: `7072`
- Kurzinfo: Boundless Spirit aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Boundless Spirit`
- Payload Hash: `979de2bfa961e3620646105c7706e3f336a89fd9fb1558b94d999bc8d398a1d9`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.651128+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_soaring_spirit"
  ],
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_boundless_spirit",
  "Name": "Boundless Spirit",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "BonusHealth": 100,
    "OutOfCombatHealthRegen": 4,
    "TechPower": 25,
    "TechPowerPercent": 10
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 30,
  "TechPowerPercent": 15,
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
      "lookup": "boundless spirit",
      "name": "Boundless Spirit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_vampire" title="Bullet Lifesteal" -->

## Bullet Lifesteal

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_vampire`
- Snapshot ID: `40215`
- Source-Dokument: `7072`
- Kurzinfo: Bullet Lifesteal aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Lifesteal`
- Payload Hash: `5a042ed9a666410a4173ab93d6ede3673a8543c64ea0e9a51f59f38d673c6b87`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.193361+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 6,
  "BonusHealth": 90,
  "BulletLifestealPercent": 13,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_vampire",
  "Name": "Bullet Lifesteal",
  "PropertyUpgrades": {
    "BonusHealth": 120,
    "BulletLifestealPercent": 16
  },
  "ShopFilters": [
    "Durability",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "bullet lifesteal",
      "name": "Bullet Lifesteal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_improved_bullet_armor" title="Bullet Resilience" -->

## Bullet Resilience

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_bullet_armor`
- Snapshot ID: `40100`
- Source-Dokument: `7072`
- Kurzinfo: Bullet Resilience aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Resilience`
- Payload Hash: `87e7f01d4901f71cd50618cd865c30aaebca6ca2666456c0eab652596a72c099`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.870963+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletResist": 30,
  "BulletResistBelowThreshold": 15,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "When below <span class=\"highlight\">50% health</span>, gain additional <span class=\"highlight\">Bullet Resist</span>.",
  "HealthThreshold": 50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_bullet_armor",
  "Name": "Bullet Resilience",
  "OutOfCombatHealthRegen": 3,
  "PropertyUpgrades": {
    "BulletResist": 10,
    "BulletResistBelowThreshold": 10
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "bullet resilience",
      "name": "Bullet Resilience",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_bullet_resist_shredder" title="Bullet Resist Shredder" -->

## Bullet Resist Shredder

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_resist_shredder`
- Snapshot ID: `40012`
- Source-Dokument: `7072`
- Kurzinfo: Bullet Resist Shredder aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Bullet Resist Shredder`
- Payload Hash: `52ebc01dacb63265b9ca83930f0874c9abcf7bca6c94ded70595b3654cf5c49c`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.662726+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 9,
  "BulletArmorReduction": -10,
  "BulletResist": 9,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Reduces <span class=\"highlight\">Bullet Resist</span> on enemies when you deal {g:citadel_inline_attribute:'SpiritDamage'}.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_bullet_resist_shredder",
  "Name": "Bullet Resist Shredder",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BulletArmorReduction": -11,
    "BulletResist": 7
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "Tier": 2,
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
      "lookup": "bullet resist shredder",
      "name": "Bullet Resist Shredder",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_burst_fire" title="Burst Fire" -->

## Burst Fire

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_burst_fire`
- Snapshot ID: `40014`
- Source-Dokument: `7072`
- Kurzinfo: Burst Fire aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Burst Fire`
- Payload Hash: `60b960c6b733700760b41f5e0d45214111d6c271777577f2d316aada981c16a3`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.667219+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.5,
  "AbilityUnitTargetLimit": 1,
  "ActivatedFireRate": 32,
  "Activation": "Passive",
  "BonusFireRate": 10,
  "BonusMoveSpeed": "1.25m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_rapid_rounds"
  ],
  "Cost": 3200,
  "Description": "Briefly gain <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Move Speed</span> when one of your bullets hits an enemy hero.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_burst_fire",
  "Name": "Burst Fire",
  "PropertyUpgrades": {
    "AbilityCooldown": -1,
    "ActivatedFireRate": 15,
    "BonusFireRate": 14,
    "BonusMoveSpeed": "1.5m",
    "SlideScale": 50
  },
  "ShopFilters": [
    "FireRate"
  ],
  "SlideScale": 50,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "burst fire",
      "name": "Burst Fire",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_capacitor" title="Capacitor" -->

## Capacitor

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_capacitor`
- Snapshot ID: `40017`
- Source-Dokument: `7072`
- Kurzinfo: Capacitor aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Capacitor`
- Payload Hash: `a744342a237ff2ed60e733b37e7cd581d33dae7dc68bdddda0f844c5cc77cb1a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.673340+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 40,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusFireRate": 5,
  "BonusPerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.19
    },
    "Value": 43
  },
  "ChainCount": 6,
  "ChainRadius": "10m",
  "ChainTickRate": 0.4,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_chain_lightning"
  ],
  "Cost": 6400,
  "Damage": 100,
  "DamagePerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.19
    },
    "Value": 43
  },
  "Description": "Launch a projectile that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span>, applies a strong slow that recovers over time, <span class=\"highlight\">prevents Stamina usage</span> and <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_capacitor",
  "MaxSlowPercent": 75,
  "Name": "Capacitor",
  "ProcChance": 20,
  "ProcCooldown": 0.2,
  "PropertyUpgrades": {
    "AbilityCooldown": -32,
    "BonusFireRate": 15,
    "Damage": 25,
    "DamagePerChain": 25,
    "ProcChance": 5
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "SlowDuration": 3,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "BossEnemy",
    "MinionEnemy"
  ],
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
      "lookup": "capacitor",
      "name": "Capacitor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_celestial_guidance" title="Celestial Blessing" -->

## Celestial Blessing

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_celestial_guidance`
- Snapshot ID: `40019`
- Source-Dokument: `7072`
- Kurzinfo: Celestial Blessing aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Celestial Blessing`
- Payload Hash: `b1f44d7c36f234287824f37259034602b463a0395a1ef18745c229587ac49377`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.677691+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BuffDuration": 6,
  "BuffMoveSpeedBonus": "5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "Applies an extremely powerful cleanse that replenishes your allies globally.",
  "HealPercentAmount": 60,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_celestial_guidance",
  "MinHeal": 400,
  "Name": "Celestial Blessing",
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "BuffDuration": 2,
    "MinHeal": 200
  },
  "Radius": "999m",
  "ShopFilters": null,
  "Slot": "Armor",
  "StaminaCooldownReduction": 100,
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
  "Tier": 5,
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
      "lookup": "celestial blessing",
      "name": "Celestial Blessing",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_cheat_death" title="Cheat Death" -->

## Cheat Death

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cheat_death`
- Snapshot ID: `40023`
- Source-Dokument: `7072`
- Kurzinfo: Cheat Death aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cheat Death`
- Payload Hash: `493ed99a946c26347ae3646ee2b622d2d10b3cb3322f47c060a6d2f9a104f8d3`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.686124+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 90.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 200,
  "BonusMoveSpeed": "0m",
  "BulletResist": 15,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DeathImmunityDamageReduction": -60,
  "DeathImmunityDuration": 4.5,
  "Description": null,
  "HealAmpReceivePenaltyPercent": -60,
  "HealAmpRegenPenaltyPercent": -60,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cheat_death",
  "Name": "Cheat Death",
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "DeathImmunityDamageReduction": 90,
    "DeathImmunityDuration": 0.5,
    "HealAmpReceivePenaltyPercent": 90,
    "HealAmpRegenPenaltyPercent": 90
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "cheat death",
      "name": "Cheat Death",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_cloak_of_opportunity" title="Cloak of Opportunity" -->

## Cloak of Opportunity

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloak_of_opportunity`
- Snapshot ID: `40030`
- Source-Dokument: `7072`
- Kurzinfo: Cloak of Opportunity aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cloak of Opportunity`
- Payload Hash: `57ecfa3bd00b1c876abeeeef7c7ebf15cc575eaa27efb4bdff97010d2f71a3a2`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.702172+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 12.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusMoveSpeed": "3m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": 500,
  "Components": null,
  "Cost": 9999,
  "Description": "Block the next debuff that would apply <span class=\"highlight\">movement lock, Stun, Chained, Immobilize, or Sleep</span> and become <span class=\"highlight\">Unstoppable</span>. Also gain a <span class=\"highlight\">Barrier</span> and bonus {g:citadel_inline_attribute:'MoveSpeed'}",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cloak_of_opportunity",
  "Name": "Cloak of Opportunity",
  "PropertyUpgrades": {
    "AbilityCooldown": -4,
    "BonusMoveSpeed": "4m",
    "CombatBarrier": 300
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StatusImmuneDuration": 4,
  "StreetBrawl": true,
  "TargetTypes": [
    "AllFriendly"
  ],
  "Tier": 5,
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
      "lookup": "cloak of opportunity",
      "name": "Cloak of Opportunity",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_close_range" title="Close Quarters" -->

## Close Quarters

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_close_range`
- Snapshot ID: `40034`
- Source-Dokument: `7072`
- Kurzinfo: Close Quarters aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Close Quarters`
- Payload Hash: `f94f29454dbf694f7a4e2d3e49d34120f55ffb60c5cc308b5d8c3c35886b8b1a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.712585+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "CloseRangeBonusDamageRange": "15m",
  "CloseRangeBonusWeaponPower": 20,
  "Components": null,
  "Cost": 800,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when in <span class=\"highlight\">close range</span> to your target.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_close_range",
  "MeleeResistPercent": 20,
  "Name": "Close Quarters",
  "PropertyUpgrades": {
    "CloseRangeBonusWeaponPower": 15,
    "MeleeResistPercent": 10
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "close quarters",
      "name": "Close Quarters",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_cold_front" title="Cold Front" -->

## Cold Front

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cold_front`
- Snapshot ID: `40035`
- Source-Dokument: `7072`
- Kurzinfo: Cold Front aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cold Front`
- Payload Hash: `e7d5f0c34f88f607cbfb5f3a98ec0f2bf8b376c9557f876264d2a84a66b71c73`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.714915+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 95
  },
  "DamageHeight": "7m",
  "Description": "Release an expanding ice blast that deals {g:citadel_inline_attribute:'SpiritDamage'} and <span class=\"highlight\">Slows</span> targets it hits.",
  "EndRadius": "10m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cold_front",
  "MovementSpeedSlow": 60,
  "NPCDamageMult": 1,
  "Name": "Cold Front",
  "PropertyUpgrades": {
    "AbilityCooldown": -13,
    "Damage": 60,
    "TechResist": 8
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption",
    "Durability"
  ],
  "Slot": "Tech",
  "SpreadDuration": 0.6,
  "StartRadius": "2m",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechResist": 6,
  "Tier": 2,
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
      "lookup": "cold front",
      "name": "Cold Front",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_colossus" title="Colossus" -->

## Colossus

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_colossus`
- Snapshot ID: `40036`
- Source-Dokument: `7072`
- Kurzinfo: Colossus aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Colossus`
- Payload Hash: `62a488d46877fac3c7664bbeea9742357c4ea43d75a6d85ee04c56668d67ac2f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.717315+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 37.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BaseAttackDamagePercent": 15,
  "BonusBaseHealth": 25,
  "BonusMeleeDamagePercent": 30,
  "BuffBulletResist": 35,
  "BuffTechResist": 35,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health"
  ],
  "Cost": 6400,
  "Description": "Grow <span class=\"highlight\">larger in size</span>, gaining {g:citadel_inline_attribute:'BulletResist'}, {g:citadel_inline_attribute:'SpiritResist'}, and {g:citadel_inline_attribute:'MeleeDamage'}. <br><br>Nearby enemies suffer from {g:citadel_inline_attribute:'Slow'} and have reduced <span class=\"highlight\">dash speed</span>.",
  "GroundDashReductionPercent": -25,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_colossus",
  "ModelScaleGrowth": 1.2,
  "ModelScaleGrowthTooltip": 20,
  "Name": "Colossus",
  "PropertyUpgrades": {
    "AbilityCooldown": -7,
    "BonusBaseHealth": 15,
    "BuffBulletResist": 10,
    "BuffTechResist": 10,
    "ModelScaleGrowth": 0.2,
    "ModelScaleGrowthTooltip": 20
  },
  "Radius": "14m",
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "SlowPercent": 30,
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "colossus",
      "name": "Colossus",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_tempo" title="Compress Cooldown" -->

## Compress Cooldown

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_tempo`
- Snapshot ID: `40122`
- Source-Dokument: `7072`
- Kurzinfo: Compress Cooldown aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Compress Cooldown`
- Payload Hash: `a68e769c4ba92cfb3cc6547b9e35759038d71fe3b72c48ebc9f892bad1e857ec`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.930775+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReduction": 18,
  "Cost": 1600,
  "Description": "Imbue an ability to reduce its <span class=\"highlight\">Cooldown</span>.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_tempo",
  "Name": "Compress Cooldown",
  "PropertyUpgrades": {
    "CooldownReduction": 10
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "compress cooldown",
      "name": "Compress Cooldown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_counterspell" title="Counterspell" -->

## Counterspell

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_counterspell`
- Snapshot ID: `40040`
- Source-Dokument: `7072`
- Kurzinfo: Counterspell aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Counterspell`
- Payload Hash: `b00708db209afd3838c75c0da08490c2dbf5e44c72bddd9248ae4b3b67672e0e`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.729548+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 23.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "BonusMoveSpeed": "1.75m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Your next parry <span class=\"highlight\">protects you from the damage and effects of enemy abilities and items</span>. On a successful spell parry {g:citadel_inline_attribute:'Heal'} and gain {g:citadel_inline_attribute:'MoveSpeed'} and {g:citadel_inline_attribute:'Spirit'}.",
  "HealOnSuccess": 150,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_counterspell",
  "Name": "Counterspell",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 150,
    "BonusMoveSpeed": "2m",
    "HealOnSuccess": 250,
    "SpiritPower": 20
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "SpellParryDuration": 0.8,
  "SpiritPower": 20,
  "SpiritPowerInnate": 5,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "counterspell",
      "name": "Counterspell",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_banshee_slugs" title="Crippling Headshot" -->

## Crippling Headshot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_banshee_slugs`
- Snapshot ID: `40000`
- Source-Dokument: `7072`
- Kurzinfo: Crippling Headshot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crippling Headshot`
- Payload Hash: `2b2e7593261ff913c875439dd7d3163f0a924321d40dc88d209b82065202a273`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.636704+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 125,
  "BulletResistReduction": -16,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_headshot_booster2"
  ],
  "Cost": 6400,
  "DebuffDuration": 12,
  "Description": null,
  "DiminishingMultiplier": 0.5,
  "HealAmpReceivePenaltyPercent": -35,
  "HealAmpRegenPenaltyPercent": -35,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_banshee_slugs",
  "MagicResistReduction": -16,
  "Name": "Crippling Headshot",
  "PropertyUpgrades": {
    "BonusHealth": 150,
    "BulletResistReduction": -12,
    "HealAmpReceivePenaltyPercent": -25,
    "HealAmpRegenPenaltyPercent": -25,
    "MagicResistReduction": -12
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "crippling headshot",
      "name": "Crippling Headshot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_crushing_fists" title="Crushing Fists" -->

## Crushing Fists

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_crushing_fists`
- Snapshot ID: `40043`
- Source-Dokument: `7072`
- Kurzinfo: Crushing Fists aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Crushing Fists`
- Payload Hash: `32f3f2b006d9e6c48c78ff7f1bd1c6ba96480f3b3dfb98b2871358d2c3af6aeb`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.736287+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHeavyMeleeDamage": 25,
  "BonusMeleeDamagePercent": 22,
  "BulletResist": 12,
  "BulletResistReduction": -4,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_melee_charge"
  ],
  "Cost": 6400,
  "DebuffDuration": 8,
  "Description": "Your <span class=\"highlight\">{g:citadel_inline_attribute:'MeleeDamage'}</span> will <span class=\"highlight\">restore ammo</span> and apply a <span class=\"highlight\">stacking bullet resist debuff</span> on enemies. Heavy melee applies 2 stacks. <br><br>If the target reaches max stacks, they will be <span class=\"highlight\">stunned</span>.",
  "HeavyMeleeMultiplier": 2,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crushing_fists",
  "LightMeleeAmmo": 15,
  "LightMeleeStacks": 1,
  "MaxStacks": 6,
  "MeleeDistanceScale": 60,
  "Name": "Crushing Fists",
  "PropertyUpgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BonusMeleeDamagePercent": 15,
    "BulletResist": 12,
    "BulletResistReduction": -4,
    "MeleeDistanceScale": 40
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "StunDuration": 0.5,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
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
      "lookup": "crushing fists",
      "name": "Crushing Fists",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_non_player_bonus_sacrifice" title="Cultist Sacrifice" -->

## Cultist Sacrifice

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_non_player_bonus_sacrifice`
- Snapshot ID: `40134`
- Source-Dokument: `7072`
- Kurzinfo: Cultist Sacrifice aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cultist Sacrifice`
- Payload Hash: `b88605b42a95e4156412dac823409e08aa37ada95d334135016a2036eefd232b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.967416+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": "7m",
  "AbilityCooldown": 270,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 160,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BaseAttackDamagePercent": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.8
    },
    "Value": 10
  },
  "BonusAbilityCharges": 1,
  "BonusHealth": {
    "Scale": {
      "Type": "power_increase",
      "Value": 4
    },
    "Value": 50
  },
  "BonusSoulsPct": 180,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_non_player_bonus"
  ],
  "Cost": 3200,
  "Description": "Target an enemy NPC and consume it for <span class=\"highlight\">180% Bonus Souls</span> and grants a powerful long lasting buff.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_non_player_bonus_sacrifice",
  "Name": "Cultist Sacrifice",
  "NonPlayerBonusWeaponPower": 30,
  "NonPlayerBulletResist": 30,
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 47,
    "BonusHealth": 300,
    "NonPlayerBonusWeaponPower": 30,
    "NonPlayerBulletResist": 30,
    "TechRadiusMultiplier": 40,
    "TechRangeMultiplier": 40
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "TrooperEnemy",
    "Neutral",
    "MinionEnemy"
  ],
  "TechRadiusMultiplier": 12,
  "TechRangeMultiplier": 12,
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
      "lookup": "cultist sacrifice",
      "name": "Cultist Sacrifice",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_glitch" title="Cursed Relic" -->

## Cursed Relic

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_glitch`
- Snapshot ID: `40074`
- Source-Dokument: `7072`
- Kurzinfo: Cursed Relic aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Cursed Relic`
- Payload Hash: `39636a95b836a2f613b5e13493af53b88441f228f432678614b7135311ab82eb`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.809490+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "20m",
  "AbilityCooldown": 55.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.25,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Curses an enemy - <span class=\"highlight\">interrupting, Silencing, Disarming</span>, and <span class=\"highlight\">preventing item usage</span>. <span class=\"highlight\">Removes all non-ultimate buffs</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glitch",
  "Name": "Cursed Relic",
  "OutgoingDamagePenaltyPercent": -14,
  "PropertyUpgrades": {
    "AbilityCooldown": -40,
    "AbilityDuration": 0.25
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "SkipFrames": 6,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
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
      "lookup": "cursed relic",
      "name": "Cursed Relic",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_debuff_reducer" title="Debuff Reducer" -->

## Debuff Reducer

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_debuff_reducer`
- Snapshot ID: `40045`
- Source-Dokument: `7072`
- Kurzinfo: Debuff Reducer aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Debuff Reducer`
- Payload Hash: `9f517317b5d3c68a7d8a1283e5f9e2666bb2dbc27c47da7fd88128145af74f9e`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.740694+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 90,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Reduces the <span class=\"highlight\">duration</span> of all negative effects applied to you.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_debuff_reducer",
  "Name": "Debuff Reducer",
  "PropertyUpgrades": {
    "StatusResistancePercent": 15
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Armor",
  "StatusResistancePercent": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "debuff reducer",
      "name": "Debuff Reducer",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_rupture" title="Decay" -->

## Decay

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rupture`
- Snapshot ID: `40166`
- Source-Dokument: `7072`
- Kurzinfo: Decay aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Decay`
- Payload Hash: `fe3eb2646428090f80eb211240c531d996531826890c482d4fdb39a1637a5540`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.057402+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.1
    },
    "Value": "20m"
  },
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 65,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Inflict <span class=\"highlight\">damage over time</span> to a target, dealing damage based on their current health.<br>Decay's damage is non-lethal and does not apply item procs.",
  "DotHealthPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.004
    },
    "Value": 2.6
  },
  "HealAmpReceivePenaltyPercent": -50,
  "HealAmpRegenPenaltyPercent": -50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rupture",
  "Name": "Decay",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "BonusHealth": 90,
    "DotHealthPercent": 0.5,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "TechPower": 12
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPower": 8,
  "TickRate": 1.0,
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
      "lookup": "decay",
      "name": "Decay",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_greater_withering_whip" title="Disarming Hex" -->

## Disarming Hex

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_greater_withering_whip`
- Snapshot ID: `40076`
- Source-Dokument: `7072`
- Kurzinfo: Disarming Hex aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Disarming Hex`
- Payload Hash: `f544578f0950bfd7e96f3b25557362cc1e4b6617bba97ab173edada3a85f8183`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.814991+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "32m",
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.25,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 75,
  "BonusSprintSpeed": "0.75m",
  "BulletArmorReduction": -13,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_withering_whip"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Disarms</span> enemy target and reduces their <span class=\"highlight\">Bullet Resist</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_greater_withering_whip",
  "Name": "Disarming Hex",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 175,
    "BulletArmorReduction": -7
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
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
      "lookup": "disarming hex",
      "name": "Disarming Hex",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_reduce_debuff_duration" title="Dispel Magic" -->

## Dispel Magic

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_reduce_debuff_duration`
- Snapshot ID: `40152`
- Source-Dokument: `7072`
- Kurzinfo: Dispel Magic aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Dispel Magic`
- Payload Hash: `389dd6b83e51d17f4c38d9cf91e90f0a506614577f6e3275bcf2fa8745ab5fd8`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.019826+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 45.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveBonusMoveSpeed": "2m",
  "BuffDuration": 3,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": null,
  "HealOnActivate": 250,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_reduce_debuff_duration",
  "Name": "Dispel Magic",
  "PropertyUpgrades": {
    "AbilityCooldown": -25,
    "HealOnActivate": 150,
    "TechResist": 20
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 10,
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
      "lookup": "dispel magic",
      "name": "Dispel Magic",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_divine_barrier" title="Divine Barrier" -->

## Divine Barrier

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_divine_barrier`
- Snapshot ID: `40049`
- Source-Dokument: `7072`
- Kurzinfo: Divine Barrier aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Divine Barrier`
- Payload Hash: `577d61fc9c176d759f941222711252cbbcff426b75e519aee19716984d15ca5a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.749474+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusMoveSpeed": "2.75m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": 600,
  "Components": [
    "upgrade_guardian_ward"
  ],
  "CooldownReductionPctOnOthers": 50,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Remove all non-stun debuffs</span> from the target and provide them with a <span class=\"highlight\">Barrier</span> and <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast. Cooldown is reduced by half when cast on someone else.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_divine_barrier",
  "Name": "Divine Barrier",
  "OutOfCombatHealthRegen": 1.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -27,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechRadiusMultiplier": 10,
  "TechRangeMultiplier": 10,
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
      "lookup": "divine barrier",
      "name": "Divine Barrier",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_diviners_kevlar" title="Diviner's Kevlar" -->

## Diviner's Kevlar

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_diviners_kevlar`
- Snapshot ID: `40050`
- Source-Dokument: `7072`
- Kurzinfo: Diviner's Kevlar aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Diviner's Kevlar`
- Payload Hash: `b943d6b42ecf50472669abc3ec730ade8e91c0c8187274afaa0d359b94dc94d4`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.752056+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityDurationPercent": 15,
  "BuffDuration": 20,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": 1000,
  "Components": null,
  "Cost": 6400,
  "Description": "Upon casting an <span class=\"highlight\">ultimate ability</span> gain a <span class=\"highlight\">Barrier</span> and temporary <span class=\"highlight\">Spirit Power</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_diviners_kevlar",
  "Name": "Diviner's Kevlar",
  "PropertyUpgrades": {
    "AbilityCooldown": -14,
    "BonusAbilityDurationPercent": 15,
    "CombatBarrier": 500,
    "TechPower": 55
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 35,
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
      "lookup": "diviner's kevlar",
      "name": "Diviner's Kevlar",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_arcane_extension" title="Duration Extender" -->

## Duration Extender

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_arcane_extension`
- Snapshot ID: `39993`
- Source-Dokument: `7072`
- Kurzinfo: Duration Extender aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Duration Extender`
- Payload Hash: `d486ec7570fb644be1065f00e3ef54aaf09c01dfa26b1894f176ed6c6df9e291`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.622539+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityDurationPercent": 22,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Imbue an ability to increase its <span class=\"highlight\">Duration</span>.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_arcane_extension",
  "Name": "Duration Extender",
  "PropertyUpgrades": {
    "BonusAbilityDurationPercent": 12
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "duration extender",
      "name": "Duration Extender",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_ability_power_shard" title="Echo Shard" -->

## Echo Shard

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ability_power_shard`
- Snapshot ID: `39976`
- Source-Dokument: `7072`
- Kurzinfo: Echo Shard aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Echo Shard`
- Payload Hash: `26ed2e38adacd1fd1dbb219f0e681937c9abfea8c4f324ceb666f5c2dfe5bb34`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.582451+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusFireRate": 5,
  "BulletResist": 5,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_ability_power_shard",
  "Name": "Echo Shard",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "BonusFireRate": 5,
    "BulletResist": 5,
    "TechRadiusMultiplier": 5,
    "TechRangeMultiplier": 5,
    "TechResist": 5
  },
  "ShopFilters": [
    "Movement",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 5,
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
      "lookup": "echo shard",
      "name": "Echo Shard",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_electric_slippers" title="Electric Slippers" -->

## Electric Slippers

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_electric_slippers`
- Snapshot ID: `40056`
- Source-Dokument: `7072`
- Kurzinfo: Electric Slippers aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Electric Slippers`
- Payload Hash: `843239a061260eb6a327f08236f381c80a3e28d8c3e29cfc8a2d1fd29e899b2b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.766260+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Damage": 100,
  "Description": "When sliding, evade bullets, gain fire rate and deal damage to enemies around you. <span class=\"diminish\"><br><br>Cooldown is per target.</span>",
  "EvasionWhileSliding": 60,
  "FireRateWhileSliding": 60,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_electric_slippers",
  "Name": "Electric Slippers",
  "ProcCooldown": 0.3,
  "PropertyUpgrades": {
    "Damage": 50,
    "EvasionWhileSliding": 15,
    "FireRateWhileSliding": 20,
    "Stamina": 2
  },
  "Radius": "12m",
  "ShopFilters": null,
  "SlideScale": 80,
  "SlideTurnScale": 100,
  "Slot": "Armor",
  "Stamina": 2,
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "electric slippers",
      "name": "Electric Slippers",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_shield" title="Enchanter's Emblem" -->

## Enchanter's Emblem

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_shield`
- Snapshot ID: `40118`
- Source-Dokument: `7072`
- Kurzinfo: Enchanter's Emblem aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enchanter's Emblem`
- Payload Hash: `e648b5541f80d96528936677f1d97901f85d0a966cbb4fea22ee1806a4999961`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.919272+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReduction": 5,
  "Cost": 1600,
  "Description": "While you are <span class=\"highlight\">above 65% health</span>, gain bonus <span class=\"highlight\">{g:citadel_inline_attribute:'Spirit'}</span> and <span class=\"highlight\">Cooldown Reduction</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_shield",
  "LifeThreshold": 65,
  "Name": "Enchanter's Emblem",
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "CooldownReduction": 7,
    "OutOfCombatHealthRegen": 3,
    "TechPower": 15,
    "TechResist": 13
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 15,
  "TechResist": 18,
  "Tier": 2,
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
      "lookup": "enchanter's emblem",
      "name": "Enchanter's Emblem",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_cardio_calibrator" title="Enduring Speed" -->

## Enduring Speed

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cardio_calibrator`
- Snapshot ID: `40018`
- Source-Dokument: `7072`
- Kurzinfo: Enduring Speed aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Enduring Speed`
- Payload Hash: `3bd350b37ab38a5d674136c11ca20657ac3e02266c5056de08ca64fadbf0a034`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.675594+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusMoveSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 1600,
  "Description": "Reduces the effect of enemy <span class=\"highlight\">Move Speed</span> penalties.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cardio_calibrator",
  "Name": "Enduring Speed",
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "BonusMoveSpeed": "2m",
    "OutOfCombatHealthRegen": 8,
    "SlowResistancePercent": 30
  },
  "ShopFilters": [
    "Movement",
    "Durability"
  ],
  "Slot": "Armor",
  "SlowResistancePercent": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "enduring speed",
      "name": "Enduring Speed",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_escalating_exposure" title="Escalating Exposure" -->

## Escalating Exposure

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_escalating_exposure`
- Snapshot ID: `40059`
- Source-Dokument: `7072`
- Kurzinfo: Escalating Exposure aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Escalating Exposure`
- Payload Hash: `e9a51317287477cfeda2f19b641c3ab38f30b89d6c2dc170b3b3b801d4542b4a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.774617+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_vulnerability"
  ],
  "Cost": 6400,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} applies a stacking <span class=\"highlight\">Spirit Amp</span> that increases your {g:citadel_inline_attribute:'SpiritDamage'} to the target.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_escalating_exposure",
  "MagicIncreasePerStack": 4.5,
  "MaxStacks": 12,
  "Name": "Escalating Exposure",
  "ProcCooldown": 0.7,
  "PropertyUpgrades": {
    "MagicIncreasePerStack": 1.5,
    "MaxStacks": 6,
    "TechArmorDamageReduction": -10,
    "TechResist": 8
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechArmorDamageReduction": -8,
  "TechResist": 17,
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
      "lookup": "escalating exposure",
      "name": "Escalating Exposure",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_reinforcing_casings" title="Escalating Resilience" -->

## Escalating Resilience

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_reinforcing_casings`
- Snapshot ID: `40155`
- Source-Dokument: `7072`
- Kurzinfo: Escalating Resilience aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Escalating Resilience`
- Payload Hash: `44e8b241b5fc9ec59b40c4696f8d3afb66ab3155398292613598c85445faa15a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.027358+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 18,
  "BonusClipSizePercent": 35,
  "BonusHealth": 75,
  "BulletResistDuration": 24,
  "BulletResistPerStack": 2,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_clip_size"
  ],
  "Cost": 3200,
  "Description": "Grants <span class=\"highlight\">Bullet Resist</span> when your bullets hit an enemy hero. <span class=\"highlight\">Each shot can only grant one stack.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_reinforcing_casings",
  "MaxArmorStacks": 30,
  "Name": "Escalating Resilience",
  "PropertyUpgrades": {
    "BonusClipSizePercent": 30,
    "BonusHealth": 125,
    "BulletResistPerStack": 2,
    "MaxArmorStacks": 20,
    "WeaponPower": 10
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "escalating resilience",
      "name": "Escalating Resilience",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_eternal_gift" title="Eternal Gift" -->

## Eternal Gift

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_eternal_gift`
- Snapshot ID: `40060`
- Source-Dokument: `7072`
- Kurzinfo: Eternal Gift aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Eternal Gift`
- Payload Hash: `729a5b9e8fb70d7c448cacbf1a60642f11becfe340b702984007ef14bc865f1f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.776795+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 165,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BuffFrequency": 2,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "Periodically gain a random permanent stat buff.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eternal_gift",
  "Name": "Eternal Gift",
  "PropertyUpgrades": {
    "BuffFrequency": -0.5
  },
  "RespawnTime": -70,
  "ShopFilters": null,
  "Slot": "Armor",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
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
      "lookup": "eternal gift",
      "name": "Eternal Gift",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_self_bubble" title="Ethereal Shift" -->

## Ethereal Shift

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_self_bubble`
- Snapshot ID: `40167`
- Source-Dokument: `7072`
- Kurzinfo: Ethereal Shift aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ethereal Shift`
- Payload Hash: `8286edec7d34d791d1a37b75248584c6b9d9064ce2e5fb8477385fb9aae5726c`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.059739+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusMoveSpeed": "3m",
  "BonusSpirit": 20,
  "BuffDuration": 5,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DampingFactor": 3,
  "Description": "You enter a void state and become <span class=\"highlight\">untargetable and invincible</span> for a short duration, during which you float slowly and cannot perform actions. Afterwards you gain <span class=\"highlight\">Spirit Power, Move Speed, and Spirit Resist</span>.<br>Can be canceled early.<br><span class=\"diminish\">Activation cancels any active ability.</span>",
  "FloatMoveSpeed": "2.5m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_self_bubble",
  "LiftHeight": 200,
  "Name": "Ethereal Shift",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "AbilityDuration": 2,
    "BonusMoveSpeed": "2m",
    "BonusSpirit": 30,
    "FloatMoveSpeed": "3.5m",
    "TechResist": 10
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 30,
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
      "lookup": "ethereal shift",
      "name": "Ethereal Shift",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_express_shot" title="Express Shot" -->

## Express Shot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_express_shot`
- Snapshot ID: `40062`
- Source-Dokument: `7072`
- Kurzinfo: Express Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Express Shot`
- Payload Hash: `7211485234d99276e59fd9b9c1e6610dc192be1becd1c6a8a2ccc259c2dd6e9f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.782025+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BonusBulletSpeedPercent": 60,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 3200,
  "Description": "Your next attack will <span class=\"highlight\">fire twice</span> in quick succession with <span class=\"highlight\">increased damage</span> and velocity. This attack consumes extra ammo.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_express_shot",
  "Name": "Express Shot",
  "ProcAmmoConsumed": 2,
  "ProcBaseAttackDamagePercent": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2
    },
    "Value": 125
  },
  "ProcBaseAttackDamagePercentAltFire": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.3
    },
    "Value": 40
  },
  "ProcBulletVelocity": 100,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusBulletSpeedPercent": 45,
    "ProcBaseAttackDamagePercent": 75,
    "ProcBaseAttackDamagePercentAltFire": 25
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "express shot",
      "name": "Express Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_clip_size" title="Extended Magazine" -->

## Extended Magazine

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_clip_size`
- Snapshot ID: `40025`
- Source-Dokument: `7072`
- Kurzinfo: Extended Magazine aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extended Magazine`
- Payload Hash: `c18719f972ec0f3e4300e5f83d6f9e90d7d690f41f04ed9b8f902f1ae1632229`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.691464+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BonusClipSizePercent": 30,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_clip_size",
  "Name": "Extended Magazine",
  "PropertyUpgrades": {
    "BonusClipSizePercent": 30
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "extended magazine",
      "name": "Extended Magazine",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_extra_charge" title="Extra Charge" -->

## Extra Charge

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_extra_charge`
- Snapshot ID: `40063`
- Source-Dokument: `7072`
- Kurzinfo: Extra Charge aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Charge`
- Payload Hash: `2fbbf14a1f18d084c5755b2ef3c49d7feec99b5623e09132eb3ebcf972cf00bd`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.784206+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityCharges": 1,
  "BonusSpiritForChargedAbilities": 7,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Adds one to your ability max <span class=\"highlight\">charges</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_extra_charge",
  "Name": "Extra Charge",
  "PropertyUpgrades": {
    "BonusAbilityCharges": 1,
    "BonusSpiritForChargedAbilities": 7
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "extra charge",
      "name": "Extra Charge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_health" title="Extra Health" -->

## Extra Health

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_health`
- Snapshot ID: `40087`
- Source-Dokument: `7072`
- Kurzinfo: Extra Health aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Health`
- Payload Hash: `a75385a19acfa246e0591aae3596eb328cc850039f28952319610553038e30ae`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.841547+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 210,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health",
  "Name": "Extra Health",
  "PropertyUpgrades": {
    "BonusHealth": 115
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "extra health",
      "name": "Extra Health",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_endurance" title="Extra Regen" -->

## Extra Regen

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_endurance`
- Snapshot ID: `40058`
- Source-Dokument: `7072`
- Kurzinfo: Extra Regen aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Regen`
- Payload Hash: `76166886073a3e8e442598a5f343e76da516db9f520087f6cb3478ddd77dc580`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.772325+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealthRegen": 2.5,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_endurance",
  "Name": "Extra Regen",
  "OutOfCombatHealthRegen": 1.5,
  "PropertyUpgrades": {
    "BonusHealthRegen": 9
  },
  "ShopFilters": [
    "ClipSize",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "extra regen",
      "name": "Extra Regen",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_improved_spirit" title="Extra Spirit" -->

## Extra Spirit

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_spirit`
- Snapshot ID: `40101`
- Source-Dokument: `7072`
- Kurzinfo: Extra Spirit aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Spirit`
- Payload Hash: `8af574bc09f5f10771bc3b45ba6105e88866f70dd8756ddeb2bcb93762be0d79`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.873436+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_spirit",
  "Name": "Extra Spirit",
  "PropertyUpgrades": {
    "TechPower": 10
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 10,
  "Tier": 1,
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
      "lookup": "extra spirit",
      "name": "Extra Spirit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_improved_stamina" title="Extra Stamina" -->

## Extra Stamina

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_improved_stamina`
- Snapshot ID: `40102`
- Source-Dokument: `7072`
- Kurzinfo: Extra Stamina aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Extra Stamina`
- Payload Hash: `d06c1ec070440bb096c455a26f548a51ede842b3aea9d2126651278ace316294`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.875477+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_improved_stamina",
  "Name": "Extra Stamina",
  "PropertyUpgrades": {
    "Stamina": 1,
    "StaminaCooldownReduction": 14
  },
  "ShopFilters": [
    "FireRate",
    "Movement"
  ],
  "Slot": "Armor",
  "Stamina": 1,
  "StaminaCooldownReduction": 12,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "extra stamina",
      "name": "Extra Stamina",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_fleetfoot_boots" title="Fleetfoot" -->

## Fleetfoot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_fleetfoot_boots`
- Snapshot ID: `40066`
- Source-Dokument: `7072`
- Kurzinfo: Fleetfoot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fleetfoot`
- Payload Hash: `ab497130643656f03bbe187b7d4619658073b17711a31a4ff96cdb77f9f662fe`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.790975+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveBonusMoveSpeed": "3.0m",
  "BaseAttackDamagePercent": 6,
  "BulletResist": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fleetfoot_boots",
  "MoveWhileShootingSpeedPenaltyReductionPercent": 100,
  "MoveWhileZoomedSpeedPenaltyReductionPercent": 100,
  "Name": "Fleetfoot",
  "PropertyUpgrades": {
    "ActiveBonusMoveSpeed": "3m",
    "BulletResist": 12,
    "SlowResistancePercent": 30
  },
  "ShopFilters": [
    "ClipSize",
    "Movement"
  ],
  "SlideScale": 35,
  "Slot": "Weapon",
  "SlowResistancePercent": 40,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "fleetfoot",
      "name": "Fleetfoot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_focus_lens" title="Focus Lens" -->

## Focus Lens

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_focus_lens`
- Snapshot ID: `40067`
- Source-Dokument: `7072`
- Kurzinfo: Focus Lens aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Focus Lens`
- Payload Hash: `08104334ff24364e9b3ed2ec001d43e3aeb9e726d8c45c27e1ea38a56346d39f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.793268+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "20m",
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusFireRate": 10,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_spirit_sap"
  ],
  "Cost": 6400,
  "Description": "Target an enemy to <span class=\"highlight\">Silence</span> them. A portion of <span class=\"highlight\">all damage dealt</span> during the silence gets applied to the target when the silence wears off.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_focus_lens",
  "MagicResistReduction": -9,
  "Name": "Focus Lens",
  "PercentDamage": 30,
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "AbilityDuration": 0.25,
    "BonusFireRate": 20,
    "MagicResistReduction": -12,
    "PercentDamage": 20,
    "TechPowerReduction": -26
  },
  "ResistReductionDuration": 12,
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPowerReduction": -30,
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
      "lookup": "focus lens",
      "name": "Focus Lens",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_chonky" title="Fortitude" -->

## Fortitude

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_chonky`
- Snapshot ID: `40024`
- Source-Dokument: `7072`
- Kurzinfo: Fortitude aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fortitude`
- Payload Hash: `fe73f5eddc3218844b3d207af7f9905058e95a0eeee3987ac067cd30f54db6e3`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.688364+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 375,
  "BonusMoveSpeed": "1.5m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health"
  ],
  "Cost": 3200,
  "Description": "After not taking damage for a period, gain health regen.",
  "HealLifePercentOutOfCombat": 2,
  "HealthThreshold": 75,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_chonky",
  "Name": "Fortitude",
  "PropertyUpgrades": {
    "BonusHealth": 375,
    "BonusMoveSpeed": "1m",
    "HealLifePercentOutOfCombat": 1,
    "RestoreDelay": -6
  },
  "RestoreDelay": 10,
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "fortitude",
      "name": "Fortitude",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_fervor" title="Frenzy" -->

## Frenzy

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_fervor`
- Snapshot ID: `40064`
- Source-Dokument: `7072`
- Kurzinfo: Frenzy aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frenzy`
- Payload Hash: `b02129d7583c545bc7485082fb933d34f929de5864388ee19f3f78adfc67a2d2`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.786656+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 16,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 15,
  "BonusHealth": 160,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "FervorFireRate": 40,
  "FervorMovespeed": "4m",
  "FervorStatusResistancePercent": 40,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fervor",
  "LowHealthThreshold": 50,
  "Name": "Frenzy",
  "PropertyUpgrades": {
    "BonusHealth": 125,
    "FervorFireRate": 20,
    "FervorMovespeed": "3m",
    "FervorStatusResistancePercent": 15
  },
  "ShopFilters": [
    "WeaponDamage",
    "FireRate",
    "ClipSize",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "frenzy",
      "name": "Frenzy",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_shivas_bracelet" title="Frostbite Charm" -->

## Frostbite Charm

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_shivas_bracelet`
- Snapshot ID: `40171`
- Source-Dokument: `7072`
- Kurzinfo: Frostbite Charm aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Frostbite Charm`
- Payload Hash: `6925778c55d36e1577d7969cab41d8a6e7bf380851b9a2a7dc1ab275b2941f77`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.072123+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 10.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Damage": 200,
  "Description": "Imbued ability has its <span class=\"highlight\">Cooldown reduced</span> and gains <span class=\"highlight\">Spirit Power</span>. When the ability deals damage, <span class=\"highlight\">freeze</span> the target and apply <span class=\"highlight\">bonus damage</span>. <span class=\"diminish\"><br><br>Cooldown is per target.</span>",
  "FreezeDuration": 1,
  "ImbuedCooldownReduction": 50,
  "ImbuedTechPower": 70,
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_shivas_bracelet",
  "Name": "Frostbite Charm",
  "PropertyUpgrades": {
    "AbilityCooldown": -2,
    "Damage": 100,
    "FreezeDuration": 0.3,
    "ImbuedCooldownReduction": 15,
    "ImbuedTechPower": 40
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "frostbite charm",
      "name": "Frostbite Charm",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_fury_trance" title="Fury Trance" -->

## Fury Trance

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_fury_trance`
- Snapshot ID: `40070`
- Source-Dokument: `7072`
- Kurzinfo: Fury Trance aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Fury Trance`
- Payload Hash: `232f7abb60ee8265674d4b28a79b59dd269b92a38d0722892132c9fbab4ea633`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.799027+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveBonusFireRate": 32,
  "BaseAttackDamagePercent": 6,
  "BonusHealth": 100,
  "BulletLifestealPercent": 14,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_vampire"
  ],
  "Cost": 3200,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_fury_trance",
  "Name": "Fury Trance",
  "PropertyUpgrades": {
    "ActiveBonusFireRate": 25,
    "BonusHealth": 110,
    "BulletLifestealPercent": 28,
    "TechResist": 20
  },
  "ShopFilters": [
    "FireRate",
    "WeaponDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechResist": 40,
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
      "lookup": "fury trance",
      "name": "Fury Trance",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_glass_cannon" title="Glass Cannon" -->

## Glass Cannon

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_glass_cannon`
- Snapshot ID: `40072`
- Source-Dokument: `7072`
- Kurzinfo: Glass Cannon aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Glass Cannon`
- Payload Hash: `3e1e8b17cbd42bbc2989f5d245f9b44e565af0f8e79d5a8c2486b82a02eba812`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.803309+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 80,
  "BonusClipPerKill": 2,
  "BuildUpDuration": 2,
  "BuildUpPerShot": 1.2,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Each hero kill grants <span class=\"highlight\">permanent Fire Rate</span> (up to a max of 8 times). Death results in the loss of 1 stack.",
  "FireRatePerKill": 7,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_glass_cannon",
  "MaxHealthLossPercent": -13,
  "MaxStacks": 8,
  "Name": "Glass Cannon",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 60,
    "FireRatePerKill": 8
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize",
    "FireRate"
  ],
  "Slot": "Weapon",
  "SlowDuration": 3,
  "SlowPercent": 30,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
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
      "lookup": "glass cannon",
      "name": "Glass Cannon",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_goose_egg" title="Golden Goose Egg" -->

## Golden Goose Egg

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_goose_egg`
- Snapshot ID: `40075`
- Source-Dokument: `7072`
- Kurzinfo: Golden Goose Egg aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Golden Goose Egg`
- Payload Hash: `ef34b3758a997cdcc97a9d7398d455b7b220fee894bb759a1ada85c8fdc38f8a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.812310+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChannelTime": 2,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusBuffsPerGold": 80,
  "BonusGoldPerMinute": 90,
  "BonusSprintSpeed": "1m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Gain <span class=\"highlight\">souls over time</span>, as long as you are alive.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_goose_egg",
  "Name": "Golden Goose Egg",
  "OutOfCombatHealthRegen": 1,
  "OutgoingDamagePenaltyPercent": -10,
  "PropertyUpgrades": {
    "BonusBuffsPerGold": -50,
    "BonusSprintSpeed": 5,
    "OutOfCombatHealthRegen": 10,
    "OutgoingDamagePenaltyPercent": 20
  },
  "ShopFilters": [
    "Movement"
  ],
  "Slot": "Tech",
  "StartingGold": 400,
  "StreetBrawl": false,
  "TargetTypes": null,
  "ThinkRate": 3,
  "Tier": 1,
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
      "lookup": "golden goose egg",
      "name": "Golden Goose Egg",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_tech_range" title="Greater Expansion" -->

## Greater Expansion

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_range`
- Snapshot ID: `40204`
- Source-Dokument: `7072`
- Kurzinfo: Greater Expansion aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Greater Expansion`
- Payload Hash: `4c78db6d1fe733a216ab2c189911d30f9e3e750998a4a55eb7333cdb77c1dee8`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.162635+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_reach"
  ],
  "Cost": 3200,
  "Description": "Increases the <span class=\"highlight\">range</span> and <span class=\"highlight\">effect radius</span> of your abilities and items.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_range",
  "Name": "Greater Expansion",
  "PropertyUpgrades": {
    "TechRadiusMultiplier": 20,
    "TechRangeMultiplier": 20,
    "TechResist": 10
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechRadiusMultiplier": 30,
  "TechRangeMultiplier": 30,
  "TechResist": 10,
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
      "lookup": "greater expansion",
      "name": "Greater Expansion",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_grit" title="Grit" -->

## Grit

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_grit`
- Snapshot ID: `40077`
- Source-Dokument: `7072`
- Kurzinfo: Grit aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Grit`
- Payload Hash: `6dd951e935c9a1123f3149d409119e2b0c593cc2d1f4277dd2655a6c26b639b7`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.817215+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BarrierDuration": 4,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": 200,
  "Components": null,
  "Cost": 800,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> for a short duration.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_grit",
  "Name": "Grit",
  "OutOfCombatHealthRegen": 1.0,
  "PropertyUpgrades": {
    "AbilityCooldown": -25,
    "CombatBarrier": 250,
    "OutOfCombatHealthRegen": 10
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "grit",
      "name": "Grit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_guardian_ward" title="Guardian Ward" -->

## Guardian Ward

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_guardian_ward`
- Snapshot ID: `40078`
- Source-Dokument: `7072`
- Kurzinfo: Guardian Ward aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Guardian Ward`
- Payload Hash: `db8d1434556daec39d7f60e5fbc3aef1d32cabb53e0abf622fd8272963999f7f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.819449+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusMoveSpeed": "2.75m",
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_grit"
  ],
  "CooldownReductionPctOnOthers": 50,
  "Cost": 1600,
  "Description": "Provide the target with a <span class=\"highlight\">Barrier</span> and temporary <span class=\"highlight\">Move Speed</span>. <span class=\"diminish\"><br>Can be self-cast.<br>Cooldown is reduced by half when cast on someone else.</span>",
  "GuardianWardCombatBarrier": 250,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_guardian_ward",
  "Name": "Guardian Ward",
  "OutOfCombatHealthRegen": 1.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "ChannelMoveSpeed": 2,
    "GuardianWardCombatBarrier": 250,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechRadiusMultiplier": 8,
  "TechRangeMultiplier": 8,
  "Tier": 2,
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
      "lookup": "guardian ward",
      "name": "Guardian Ward",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_eldritch_shot" title="Haunting Shot" -->

## Haunting Shot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_eldritch_shot`
- Snapshot ID: `40055`
- Source-Dokument: `7072`
- Kurzinfo: Haunting Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Haunting Shot`
- Payload Hash: `970c446a97d269ea115414dc8657970e96255d075a000c77050b2ef5538ee638`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.763100+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 2.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletRadius": "1.5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "DebuffDuration": 4,
  "Description": "Your next bullet applies a powerful debuff reducing the enemy's damage output, healing and movement speed. It also deals {g:citadel_inline_attribute:'BonusSpiritDamage'} based on the targets current Health. <br><br>The bullet is larger and penetrates through targets.",
  "GroundDashReductionPercent": -40,
  "HealAmpReceivePenaltyPercent": -40,
  "HealAmpRegenPenaltyPercent": -40,
  "HealthPctDamage": 10,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_eldritch_shot",
  "MovementSpeedSlow": 40,
  "Name": "Haunting Shot",
  "OutgoingDamagePenaltyPercent": -40,
  "ProcChance": 100,
  "ProcCooldown": 1,
  "PropertyUpgrades": {
    "AbilityCooldown": -1,
    "GroundDashReductionPercent": -10,
    "HealAmpReceivePenaltyPercent": -15,
    "HealAmpRegenPenaltyPercent": -15,
    "HealthPctDamage": 5,
    "MovementSpeedSlow": 10,
    "OutgoingDamagePenaltyPercent": -15
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "haunting shot",
      "name": "Haunting Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_headhunter" title="Headhunter" -->

## Headhunter

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_headhunter`
- Snapshot ID: `40080`
- Source-Dokument: `7072`
- Kurzinfo: Headhunter aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Headhunter`
- Payload Hash: `2387da7b38b96a620d782c1bb3d0b5bdd161234885acfa0b1c9082432ec8fb25`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.824259+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 5,
  "BonusHealth": 50,
  "BonusMoveSpeed": "1.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_headshot_booster"
  ],
  "Cost": 3200,
  "Description": "Your next <span class=\"highlight\">headshot</span> against an enemy Hero deals {g:citadel_inline_attribute:'BonusWeaponDamage'}, {g:citadel_inline_attribute:'Heal'} you, and briefly grants {g:citadel_inline_attribute:'BonusMoveSpeed'}.",
  "HeadShotBonusDamage": {
    "Scale": {
      "Type": "power_increase",
      "Value": 4
    },
    "Value": 75
  },
  "HealPercentPerHeadshot": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.06
    },
    "Value": 4
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headhunter",
  "MovementSpeedBonusDuration": 3,
  "Name": "Headhunter",
  "ProcChance": 100,
  "PropertyUpgrades": {
    "AbilityCooldown": -3,
    "HeadShotBonusDamage": 75,
    "HealPercentPerHeadshot": 4
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "headhunter",
      "name": "Headhunter",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_headshot_booster" title="Headshot Booster" -->

## Headshot Booster

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_headshot_booster`
- Snapshot ID: `40081`
- Source-Dokument: `7072`
- Kurzinfo: Headshot Booster aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Headshot Booster`
- Payload Hash: `6a814f0e7ebcd3cc4a705b250992c108ab27c539ad47f447e1ff85bc9595e3f4`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.826757+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 30,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Your next <span class=\"highlight\">headshot</span> against an enemy Hero deals {g:citadel_inline_attribute:'BonusWeaponDamage'}.",
  "HeadShotBonusDamage": 45,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headshot_booster",
  "Name": "Headshot Booster",
  "ProcChance": 100,
  "PropertyUpgrades": {
    "HeadShotBonusDamage": 55
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "headshot booster",
      "name": "Headshot Booster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_healbane" title="Healbane" -->

## Healbane

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_healbane`
- Snapshot ID: `40084`
- Source-Dokument: `7072`
- Kurzinfo: Healbane aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healbane`
- Payload Hash: `3f2d5923ff880dff8ec39038919f2a523f4e6b41b201a3ea00619473b15bc124`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.834881+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 8,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Your {g:citadel_inline_attribute:'SpiritDamage'} applies <span class=\"highlight\">Healing Reduction</span>. If an enemy hero dies under this effect, you receive a large heal.",
  "HealAmpReceivePenaltyPercent": -35,
  "HealAmpRegenPenaltyPercent": -35,
  "HealOnKill": 275,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healbane",
  "Name": "Healbane",
  "PropertyUpgrades": {
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "HealOnKill": 125,
    "TechPower": 11
  },
  "ShopFilters": [
    "Healing",
    "Disruption",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 7,
  "Tier": 2,
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
      "lookup": "healbane",
      "name": "Healbane",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_healing_booster" title="Healing Booster" -->

## Healing Booster

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_healing_booster`
- Snapshot ID: `40086`
- Source-Dokument: `7072`
- Kurzinfo: Healing Booster aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Booster`
- Payload Hash: `856e2e0681dfe8ac2504bd9cf893cb92aee40d45e8ca1e07548a48d69e1c12a6`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.839454+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealthRegen": 3,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_endurance"
  ],
  "Cost": 1600,
  "Description": "Increases the effectiveness of your <span class=\"highlight\">healing</span>.",
  "HealAmpCastPercent": 20,
  "HealAmpRegenPercent": 20,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healing_booster",
  "Name": "Healing Booster",
  "OutOfCombatHealthRegen": 1,
  "PropertyUpgrades": {
    "BonusHealthRegen": 9,
    "HealAmpCastPercent": 15,
    "HealAmpRegenPercent": 15
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "healing booster",
      "name": "Healing Booster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_health_nova" title="Healing Nova" -->

## Healing Nova

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_nova`
- Snapshot ID: `40089`
- Source-Dokument: `7072`
- Kurzinfo: Healing Nova aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Nova`
- Payload Hash: `e9e7ba4b2f2d0a0afd8085fbb5db76e7b5c632fb41acb72b7f3c4d36808c5cb4`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.846528+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCooldown": 60,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "AuraRadius": "18m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> yourself and nearby allies.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_nova",
  "Name": "Healing Nova",
  "PropertyUpgrades": {
    "TechPower": 12,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12,
    "TotalHealthRegen": 425
  },
  "RegenDuration": 2,
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "SpiritPower": 8,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly",
    "TrooperFriendly",
    "MinionFriendly"
  ],
  "TechRadiusMultiplier": 5,
  "TechRangeMultiplier": 5,
  "Tier": 3,
  "TotalHealthRegen": {
    "Scale": {
      "Type": "power_increase",
      "Value": 6
    },
    "Value": 325
  },
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
      "lookup": "healing nova",
      "name": "Healing Nova",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_health_stimpak" title="Healing Rite" -->

## Healing Rite

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stimpak`
- Snapshot ID: `40093`
- Source-Dokument: `7072`
- Kurzinfo: Healing Rite aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Rite`
- Payload Hash: `8ce7cc703c17cdef06da36b4abcdfa1d5e99659550c0e098ce423e6e3491ae92`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.854687+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "30m",
  "AbilityCooldown": 70,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusSprintSpeed": "2m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Grant <span class=\"highlight\">Regen</span> and <span class=\"highlight\">Sprint Speed</span> to the target. Gets dispelled if you take damage from enemy players or objectives. Can be self-cast.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stimpak",
  "Name": "Healing Rite",
  "PropertyUpgrades": {
    "AbilityCooldown": -60,
    "BonusSprintSpeed": "6m",
    "TotalHealthRegen": 600
  },
  "RegenDuration": 20,
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 1,
  "TotalHealthRegen": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.1
    },
    "Value": 300
  },
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
      "lookup": "healing rite",
      "name": "Healing Rite",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_healbuff" title="Healing Tempo" -->

## Healing Tempo

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_healbuff`
- Snapshot ID: `40085`
- Source-Dokument: `7072`
- Kurzinfo: Healing Tempo aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Healing Tempo`
- Payload Hash: `8a538d6f4a7fd6bfda4a3b3a006244f002cc6e94669608aed99727275e3750bf`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.837227+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 35,
  "BonusHealthRegen": 6,
  "BonusMoveSpeed": "1.25m",
  "BuffDuration": 7,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_healing_booster"
  ],
  "Cost": 6400,
  "Description": "Applying {g:citadel_inline_attribute:'Heal'} to yourself or an ally grants the target {g:citadel_inline_attribute:'BonusFireRate'} and {g:citadel_inline_attribute:'BonusMoveSpeed'}.<br><br><span class=\"diminish\">Does not apply on innate Regen or passive Bullet/Spirit Lifesteals.</span>",
  "HealAmpCastPercent": 25,
  "HealAmpRegenPercent": 25,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_healbuff",
  "MinimumHealAmount": 1,
  "Name": "Healing Tempo",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "BonusFireRate": 20,
    "BonusHealthRegen": 6,
    "BonusMoveSpeed": "2m",
    "HealAmpCastPercent": 10,
    "HealAmpRegenPercent": 10,
    "TechResist": 10
  },
  "ShopFilters": [
    "FireRate",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "healing tempo",
      "name": "Healing Tempo",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_dps_aura" title="Heroic Aura" -->

## Heroic Aura

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_dps_aura`
- Snapshot ID: `40052`
- Source-Dokument: `7072`
- Kurzinfo: Heroic Aura aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Heroic Aura`
- Payload Hash: `85029a8156c07e1a7c2e9d1c7d687699d634f1e811adbe8cafcd7d7934efd90b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.756358+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 22.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveBonusMoveSpeed": "2.25m",
  "ActiveRadius": "35m",
  "BonusFireRate": 26,
  "BonusSprintSpeed": "1.5m",
  "BulletResist": 17,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Provides <span class=\"highlight\">Bullet Resist</span> to nearby friendly units.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_dps_aura",
  "Name": "Heroic Aura",
  "NonHeroMult": 2,
  "PropertyUpgrades": {
    "ActiveBonusMoveSpeed": "3m",
    "ActiveRadius": "15m",
    "BonusFireRate": 34,
    "BulletResist": 10
  },
  "Radius": "35m",
  "ShopFilters": [
    "FireRate",
    "Healing",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly"
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
      "lookup": "heroic aura",
      "name": "Heroic Aura",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_high_velocity_mag" title="High-Velocity Rounds" -->

## High-Velocity Rounds

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_high_velocity_mag`
- Snapshot ID: `40095`
- Source-Dokument: `7072`
- Kurzinfo: High-Velocity Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `High-Velocity Rounds`
- Payload Hash: `e4fe1e5e640b71c4f6be2e7e45cf4d0f514f32ee5379924c4ff7c8a0c21f4c05`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.858933+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BonusBulletSpeedPercent": 60,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_high_velocity_mag",
  "Name": "High-Velocity Rounds",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusBulletSpeedPercent": 45
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "high-velocity rounds",
      "name": "High-Velocity Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_hollow_point_rounds" title="Hollow Point" -->

## Hollow Point

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_hollow_point_rounds`
- Snapshot ID: `40096`
- Source-Dokument: `7072`
- Kurzinfo: Hollow Point aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hollow Point`
- Payload Hash: `49543e88cbb81c5fafd9895ed3493e4b2605cceca7d91b080d17d931f08386f5`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.861152+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 35,
  "BonusHealth": 125,
  "BulletArmorReduction": -9,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DebuffDuration": 8,
  "Description": "When you are <span class=\"highlight\">above 65% health</span>, deal additional <span class=\"highlight\">Weapon Damage</span> and your bullets reduce enemy <span class=\"highlight\">Bullet Resist</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_hollow_point_rounds",
  "LifeThreshold": 65,
  "Name": "Hollow Point",
  "OutOfCombatHealthRegen": 4.5,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 25,
    "BonusHealth": 150,
    "BulletArmorReduction": -12
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "hollow point",
      "name": "Hollow Point",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_bullet_armor_reduction_aura" title="Hunter's Aura" -->

## Hunter's Aura

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_bullet_armor_reduction_aura`
- Snapshot ID: `40010`
- Source-Dokument: `7072`
- Kurzinfo: Hunter's Aura aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Hunter's Aura`
- Payload Hash: `54b8a449dda59af6ae7be301a52bb792d1ec5ec395434464e34347e134542ce5`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.658204+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 100,
  "BonusSprintSpeed": "0.75m",
  "BulletArmorReduction": -10,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Reduces nearby enemies' <span class=\"highlight\">Bullet Resist and Fire Rate</span>. If there is only one enemy hero nearby, this <span class=\"highlight\">effect is doubled</span>.",
  "FireRateSlow": 15,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_bullet_armor_reduction_aura",
  "Name": "Hunter's Aura",
  "PropertyUpgrades": {
    "BonusHealth": 125,
    "BonusSprintSpeed": "3m",
    "BulletArmorReduction": -6,
    "FireRateSlow": 5
  },
  "Radius": "15m",
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "ClipSize"
  ],
  "SingleTargetPlayerMultiplier": 2,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
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
      "lookup": "hunter's aura",
      "name": "Hunter's Aura",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_soaring_spirit" title="Improved Spirit" -->

## Improved Spirit

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_soaring_spirit`
- Snapshot ID: `40179`
- Source-Dokument: `7072`
- Kurzinfo: Improved Spirit aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Improved Spirit`
- Payload Hash: `2a7ecb18f2e95f782756cd17fd397d03b82d9f047703ef20ce15d602558d1178`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.096487+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "BonusSprintSpeed": "1m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_improved_spirit"
  ],
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_soaring_spirit",
  "Name": "Improved Spirit",
  "OutOfCombatHealthRegen": 1.5,
  "PropertyUpgrades": {
    "OutOfCombatHealthRegen": 3,
    "TechPower": 22
  },
  "ShopFilters": [
    "Movement",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 18,
  "Tier": 2,
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
      "lookup": "improved spirit",
      "name": "Improved Spirit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_auto_cleanse" title="Indomitable" -->

## Indomitable

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_auto_cleanse`
- Snapshot ID: `39999`
- Source-Dokument: `7072`
- Kurzinfo: Indomitable aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Indomitable`
- Payload Hash: `9b29bc614d4edd65fc60c72d2ac4d044320a6037b634c187e32c4842dac21442`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.634517+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 55,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletResist": 10,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_vex_barrier"
  ],
  "CooldownReductionOnProc": 20,
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_auto_cleanse",
  "Name": "Indomitable",
  "OutOfCombatHealthRegen": 2.0,
  "PropertyUpgrades": {
    "AbilityCooldown": -35,
    "BulletResist": 14,
    "TechResist": 14,
    "VexBarrierCombatBarrier": 450
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 10,
  "Tier": 4,
  "VexBarrierCombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 2.0
    },
    "Value": 325
  },
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
      "lookup": "indomitable",
      "name": "Indomitable",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_infinite_rounds" title="Infinite Rounds" -->

## Infinite Rounds

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_infinite_rounds`
- Snapshot ID: `40103`
- Source-Dokument: `7072`
- Kurzinfo: Infinite Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infinite Rounds`
- Payload Hash: `1366be693a17c3cf56d3fcb0dc693c26e179f26f974f19c5d65e85dadbad53fe`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.877988+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusBulletSpeedPercent": 200,
  "BonusFireRate": 35,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "You have <span class=\"highlight\">infinite</span> ammo.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_infinite_rounds",
  "Name": "Infinite Rounds",
  "ProcChance": 65,
  "PropertyUpgrades": {
    "BonusBulletSpeedPercent": 100,
    "BonusFireRate": 20,
    "ProcChance": 10
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
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
      "lookup": "infinite rounds",
      "name": "Infinite Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_infuser" title="Infuser" -->

## Infuser

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_infuser`
- Snapshot ID: `40105`
- Source-Dokument: `7072`
- Kurzinfo: Infuser aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Infuser`
- Payload Hash: `a6fccf247ce4b65d0890c1750ccc7e4051e6782494fe662a035c7f4dc1455a4e`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.884229+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityLifestealPercentHero": 70,
  "AbilityLifestealPercentHeroPassive": 13,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusHealth": 100,
  "BonusSpirit": 30,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Gain <span class=\"highlight\">Spirit Lifesteal</span> and <span class=\"highlight\">Spirit Power</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_infuser",
  "Name": "Infuser",
  "NonHeroAbilityLifestealTooltipOnly": 3,
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "AbilityLifestealPercentHeroPassive": 16,
    "BonusHealth": 50,
    "BonusSpirit": 30,
    "TechResist": 10
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 6,
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
      "lookup": "infuser",
      "name": "Infuser",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_inhibitor" title="Inhibitor" -->

## Inhibitor

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_inhibitor`
- Snapshot ID: `40106`
- Source-Dokument: `7072`
- Kurzinfo: Inhibitor aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Inhibitor`
- Payload Hash: `308e90e199c2ec6d0a14eb89cb147c136da8cf81f25ad8645fe13b47548b64ec`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.886833+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 10,
  "BonusHealth": 150,
  "BuildUpDuration": 5,
  "BuildUpPerShot": 0.77,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DebuffDuration": 5,
  "Description": "Your bullets build up to reduce the target's <span class=\"highlight\">outgoing damage</span> and apply <span class=\"highlight\">healing reduction</span>.",
  "HealAmpReceivePenaltyPercent": -40,
  "HealAmpRegenPenaltyPercent": -40,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_inhibitor",
  "Name": "Inhibitor",
  "OutgoingDamagePenaltyPercent": -30,
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 20,
    "BonusHealth": 125,
    "HealAmpReceivePenaltyPercent": -20,
    "HealAmpRegenPenaltyPercent": -20,
    "OutgoingDamagePenaltyPercent": -20
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "inhibitor",
      "name": "Inhibitor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_intensifying_clip" title="Intensifying Magazine" -->

## Intensifying Magazine

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_intensifying_clip`
- Snapshot ID: `40107`
- Source-Dokument: `7072`
- Kurzinfo: Intensifying Magazine aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Intensifying Magazine`
- Payload Hash: `12f06d8eb39623581866529dfae51d9ae6dd16269da90f25a28a3be76095a998`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.889772+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": "",
  "BaseAttackDamagePercentAtMaxDuration": 45,
  "BonusClipSizePercent": 20,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Increases <span class=\"highlight\">Weapon Damage</span> as you continuously fire your weapon.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_intensifying_clip",
  "Name": "Intensifying Magazine",
  "PropertyUpgrades": {
    "BaseAttackDamagePercentAtMaxDuration": 55,
    "BonusClipSizePercent": 40
  },
  "ShootDurationForMax": 2.5,
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "intensifying magazine",
      "name": "Intensifying Magazine",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_juggernaut" title="Juggernaut" -->

## Juggernaut

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_juggernaut`
- Snapshot ID: `40108`
- Source-Dokument: `7072`
- Kurzinfo: Juggernaut aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Juggernaut`
- Payload Hash: `7dd6d5963f5f933ced11d22557f8a4b298a58c6e5819820483519679030acf80`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.892547+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealthRegen": 8,
  "BonusMoveSpeed": "2.5m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_cardio_calibrator"
  ],
  "Cost": 6400,
  "Description": null,
  "FireRateSlow": 40,
  "FireRateSlowDuration": 4,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_juggernaut",
  "MeleeResistPercent": 25,
  "Name": "Juggernaut",
  "PropertyUpgrades": {
    "BonusHealthRegen": 8,
    "BonusMoveSpeed": "3.5m",
    "FireRateSlow": 20,
    "MeleeResistPercent": 15,
    "SlowResistancePercent": 15
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "SlowResistancePercent": 50,
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "juggernaut",
      "name": "Juggernaut",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_kinetic_sash" title="Kinetic Dash" -->

## Kinetic Dash

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_kinetic_sash`
- Snapshot ID: `40109`
- Source-Dokument: `7072`
- Kurzinfo: Kinetic Dash aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Kinetic Dash`
- Payload Hash: `8bc6d2ac069d36e257f6c700ec43fe0be6304587747f998aa79500cfaf7d53e2`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.895075+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusClipSize": 6,
  "BonusFireRate": 25,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 1600,
  "Description": "When you <span class=\"highlight\">Dash-Jump</span> you gain <span class=\"highlight\">Fire Rate</span> and bonus <span class=\"highlight\">Ammo</span> until your next reload. Lasts up to 7s.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_kinetic_sash",
  "Name": "Kinetic Dash",
  "PropertyUpgrades": {
    "BonusClipSize": 6,
    "BonusFireRate": 20,
    "Stamina": 1,
    "StaminaCooldownReduction": 14
  },
  "ShopFilters": [
    "FireRate",
    "ClipSize",
    "Movement",
    "Durability"
  ],
  "Slot": "Weapon",
  "Stamina": 1,
  "StaminaCooldownReduction": 12,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "kinetic dash",
      "name": "Kinetic Dash",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_target_stun" title="Knockdown" -->

## Knockdown

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_target_stun`
- Snapshot ID: `40196`
- Source-Dokument: `7072`
- Kurzinfo: Knockdown aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Knockdown`
- Payload Hash: `c724b14ce85676fcd81b72671074e9e7ec36c3abcf1a164baa8a548a5c478d2d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.142251+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "45m",
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 75,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Apply a <span class=\"highlight\">Stun</span> after <span class=\"highlight\">2s</span>. Stun duration is increased against <span class=\"highlight\">airborne</span> targets.<br><br><span class=\"diminish\">Increases the target's gravity for the duration of the stun.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_target_stun",
  "MaxBonusDuration": 1.5,
  "MaxHeightForBonus": "30m",
  "Name": "Knockdown",
  "PropertyUpgrades": {
    "BonusHealth": 75,
    "StunDuration": 0.75,
    "TechRadiusMultiplier": 6,
    "TechRangeMultiplier": 6
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "StunDelay": 2,
  "StunDuration": 0.5,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechRadiusMultiplier": 5,
  "TechRangeMultiplier": 5,
  "Tier": 3,
  "VisualContractRadius": "3m",
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
      "lookup": "knockdown",
      "name": "Knockdown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_damage_recycler" title="Leech" -->

## Leech

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_damage_recycler`
- Snapshot ID: `40044`
- Source-Dokument: `7072`
- Kurzinfo: Leech aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Leech`
- Payload Hash: `7e3cecedd05edb2b65f5e6080f23c57fd0a7dada41d371d218cfae8adaf0d1de`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.738460+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 25,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 12,
  "BonusHealth": 180,
  "BulletLifestealPercent": 25,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_vampire",
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Reduces the effect of enemy applied <span class=\"highlight\">healing reduction</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_damage_recycler",
  "Name": "Leech",
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 15,
    "BaseAttackDamagePercent": 15,
    "BonusHealth": 200,
    "BulletLifestealPercent": 15,
    "TechPower": 15
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 12,
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
      "lookup": "leech",
      "name": "Leech",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_boxing_glove" title="Lifestrike" -->

## Lifestrike

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_boxing_glove`
- Snapshot ID: `40008`
- Source-Dokument: `7072`
- Kurzinfo: Lifestrike aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lifestrike`
- Payload Hash: `e7b8a1b088cc527ebafe8a8eb04d02dd2afa993cf4aa3a727db211b34ca506bd`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.653122+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 4,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 125,
  "BonusMeleeDamagePercent": 16,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_lifestrike_gauntlets"
  ],
  "Cost": 3200,
  "Description": "Your <span class=\"highlight\">Melee Attack</span> applies <span class=\"highlight\">Movement Slow</span> and <span class=\"highlight\">heals you</span> for a percentage of the <span class=\"highlight\">Melee Damage</span> dealt plus a fixed amount. <span class=\"diminish\"><br><br>This heal is 40% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_boxing_glove",
  "LifestealHeal": {
    "Scale": {
      "Type": "power_increase",
      "Value": 1.5
    },
    "Value": 100
  },
  "LifestealHealPercent": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.5
    },
    "Value": 30
  },
  "LightMeleeCooldownMult": 1.5,
  "Name": "Lifestrike",
  "NonHeroHealPct": 40,
  "PropertyUpgrades": {
    "AbilityCooldown": -3,
    "BonusHealth": 125,
    "BonusMeleeDamagePercent": 10
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "SlowDuration": 2.5,
  "SlowPercent": 60,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "lifestrike",
      "name": "Lifestrike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_ultimate_burst" title="Lightning Scroll" -->

## Lightning Scroll

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ultimate_burst`
- Snapshot ID: `40212`
- Source-Dokument: `7072`
- Kurzinfo: Lightning Scroll aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lightning Scroll`
- Payload Hash: `833cb023ac150555302539994591a09d1159b1a81f003a9047ca558f08324fcb`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.183811+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "BonusSprintSpeed": "0.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_slow"
  ],
  "Cost": 6400,
  "Damage": 150,
  "DelayBeforeStun": 3,
  "Description": "Damage from your ultimate applies a {g:citadel_inline_attribute:'Stun'} and deals {g:citadel_inline_attribute:'BonusSpiritDamage'} after a short delay.</span>",
  "GroundDashReductionPercent": -12,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ultimate_burst",
  "MovementSpeedSlow": 30,
  "Name": "Lightning Scroll",
  "PropertyUpgrades": {
    "BonusHealth": 100,
    "BonusSprintSpeed": "5m",
    "Damage": 100,
    "StunDuration": 0.75
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "SlowPercent": 80,
  "StreetBrawl": false,
  "StunDuration": 0.75,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "lightning scroll",
      "name": "Lightning Scroll",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_long_range" title="Long Range" -->

## Long Range

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_long_range`
- Snapshot ID: `40111`
- Source-Dokument: `7072`
- Kurzinfo: Long Range aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Long Range`
- Payload Hash: `21901cf2cec4e8df7e65c59061bf9011d2b3a48de6a3f9cc6199197c7862133a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.900414+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAttackRangePercent": 8,
  "BonusSprintSpeed": "0.75m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when <span class=\"highlight\">beyond a minimum distance</span> from your target.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_long_range",
  "LongRangeBonusWeaponPower": 40,
  "LongRangeBonusWeaponPowerMinRange": "15m",
  "Name": "Long Range",
  "PropertyUpgrades": {
    "BonusAttackRangePercent": 8,
    "LongRangeBonusWeaponPower": 30
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "long range",
      "name": "Long Range",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_critshot" title="Lucky Shot" -->

## Lucky Shot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_critshot`
- Snapshot ID: `40042`
- Source-Dokument: `7072`
- Kurzinfo: Lucky Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Lucky Shot`
- Payload Hash: `d053c5d34a05bd391ed4ebcd36932387ffbfb61c737b559f08171406abacac62`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.734111+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusClipSizePercent": 30,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "CritDamagePercent": 100,
  "Description": "Your bullets have a chance to be empowered, causing them to deal <span class=\"highlight\">bonus weapon damage</span> on hit.<br><span class=\"diminish\">Bonus damage cannot Crit.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_critshot",
  "Name": "Lucky Shot",
  "ProcChance": 25,
  "PropertyUpgrades": {
    "BonusClipSizePercent": 40,
    "CritDamagePercent": 30,
    "ProcChance": 5
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
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
      "lookup": "lucky shot",
      "name": "Lucky Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_carpet" title="Magic Carpet" -->

## Magic Carpet

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_carpet`
- Snapshot ID: `40114`
- Source-Dokument: `7072`
- Kurzinfo: Magic Carpet aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Magic Carpet`
- Payload Hash: `d419fb1f3355f6191617bba2d1c268182d882634c95b26e3a9108c06aa103d30`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.908117+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 32.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "AirControlPercent": 25,
  "BonusAbilityDurationPercent": 15,
  "BonusHealth": 125,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Summon a Magic Carpet that will <span class=\"highlight\">fly</span> you away. While flying you are immune to slows and doing any action will dismiss the carpet. <span class=\"diminish\"><br>Cannot use abilities while the carpet is being summoned.</span>",
  "FlyMoveSpeed": "7m",
  "GravityScale": -15,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_carpet",
  "Name": "Magic Carpet",
  "PropertyUpgrades": {
    "AbilityDuration": 8,
    "BonusAbilityDurationPercent": 15,
    "FlyMoveSpeed": "6m",
    "SummonDuration": -0.3,
    "TechPower": 46
  },
  "ShopFilters": [
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "SummonDuration": 1.3,
  "TargetTypes": null,
  "TechPower": 14,
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
      "lookup": "magic carpet",
      "name": "Magic Carpet",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_rocket_booster" title="Majestic Leap" -->

## Majestic Leap

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rocket_booster`
- Snapshot ID: `40163`
- Source-Dokument: `7072`
- Kurzinfo: Majestic Leap aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Majestic Leap`
- Payload Hash: `a4ca362d06f1e7546f885dc9d7fd6901e11a69419ecd06b981d6b01cc3f7ff92`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.048616+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "AirControlPercent": 100,
  "AirControlPercentBarrier": 50,
  "BarrierDuration": 8,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "power_increase",
      "Value": 12
    },
    "Value": 200
  },
  "Components": null,
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Launch yourself</span> high into the air and grant yourself a <span class=\"highlight\">Barrier</span>. While in the air, you can use the active again to drop down faster.<br><br><span class=\"diminish\">Cannot be used for 5s if attacked by enemy Hero.</span>",
  "DropDownSpeed": "35m",
  "ImpactHeight": "2m",
  "InterruptCooldown": 5,
  "IsDisabled": false,
  "IsImbue": false,
  "JumpVelocityHidden": "27m",
  "Key": "upgrade_rocket_booster",
  "MaxLandingSpeed": "20m",
  "MinAimAngle": 30,
  "Name": "Majestic Leap",
  "PropertyUpgrades": {
    "AbilityCooldown": -35,
    "CombatBarrier": 275,
    "InterruptCooldown": -3
  },
  "ShopFilters": [
    "Movement"
  ],
  "SlamDownRadius": "10m",
  "Slot": "Armor",
  "SlowDuration": 2.5,
  "SlowPercent": 40,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "TossSpeed": 500,
  "VerticalDifferenceTolerance": "2m",
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
      "lookup": "majestic leap",
      "name": "Majestic Leap",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_melee_charge" title="Melee Charge" -->

## Melee Charge

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_melee_charge`
- Snapshot ID: `40125`
- Source-Dokument: `7072`
- Kurzinfo: Melee Charge aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Melee Charge`
- Payload Hash: `8bffcaaff3845b0e1a07cb6ae591c91faf63e7303632a97658e17b8d21098778`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.940355+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHeavyMeleeDamage": 25,
  "BonusMeleeDamagePercent": 10,
  "BulletResist": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Your next <span class=\"highlight\">Heavy Melee</span> attack against an enemy <span class=\"highlight\">deals increased damage</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_melee_charge",
  "MeleeDistanceScale": 50,
  "Name": "Melee Charge",
  "PropertyUpgrades": {
    "BonusHeavyMeleeDamage": 15,
    "BulletResist": 12,
    "MeleeDistanceScale": 30
  },
  "ShopFilters": [
    "Melee"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "TrooperEnemy",
    "MinionEnemy",
    "Neutral"
  ],
  "Tier": 2,
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
      "lookup": "melee charge",
      "name": "Melee Charge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_lifestrike_gauntlets" title="Melee Lifesteal" -->

## Melee Lifesteal

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_lifestrike_gauntlets`
- Snapshot ID: `40110`
- Source-Dokument: `7072`
- Kurzinfo: Melee Lifesteal aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Melee Lifesteal`
- Payload Hash: `c12636ba0815bbe2595551ccc4e5bf50717a70a313954997c95b61b9722b17a2`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.897575+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusMeleeDamagePercent": 12,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Your next <span class=\"highlight\">Melee</span> attack <span class=\"highlight\">heals you</span>. <span class=\"diminish\"><br><br>This heal is 30% effective vs non-heroes. <br>Cooldown is 1.5x as long for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_lifestrike_gauntlets",
  "LifestrikeHeal": 100,
  "LightMeleeCooldownMult": 1.5,
  "Name": "Melee Lifesteal",
  "NonHeroHealPct": 30,
  "PropertyUpgrades": {
    "AbilityCooldown": -6,
    "BonusMeleeDamagePercent": 12
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
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
      "lookup": "melee lifesteal",
      "name": "Melee Lifesteal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_ethereal_bullets" title="Mercurial Magnum" -->

## Mercurial Magnum

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ethereal_bullets`
- Snapshot ID: `40061`
- Source-Dokument: `7072`
- Kurzinfo: Mercurial Magnum aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mercurial Magnum`
- Payload Hash: `de3a2836f26b7bc1ef9b415b5577b3c877cf9a3df10f31e571ae58f8605edb03`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.779811+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 14,
  "AbilityCooldown": 15,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AmmoReloadPercent": 100,
  "BonusClipSizePercent": 20,
  "BonusFireRate": 22,
  "BuffDuration": 12,
  "BulletsBonusMagicDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.49
    },
    "Value": 25
  },
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_quick_silver"
  ],
  "Cost": 6400,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.16
    },
    "Value": 60
  },
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use. Until your next reload, your <span class=\"highlight\">bullets deal {g:citadel_inline_attribute:'BonusSpiritDamage'}</span> based on your Spirit Power.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_ethereal_bullets",
  "Name": "Mercurial Magnum",
  "PropertyUpgrades": {
    "BonusClipSizePercent": 60,
    "BonusFireRate": 20,
    "BulletsBonusMagicDamage": 20,
    "Damage": 120,
    "TechPower": 15
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 7,
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
      "lookup": "mercurial magnum",
      "name": "Mercurial Magnum",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_metal_skin" title="Metal Skin" -->

## Metal Skin

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_metal_skin`
- Snapshot ID: `40127`
- Source-Dokument: `7072`
- Kurzinfo: Metal Skin aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Metal Skin`
- Payload Hash: `36f9a0a44380cc5a1e7e93ce97ef4763e8a450d7d4c0ea97448b6463ba4bea7f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.945888+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 24.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveMoveSpeedPenalty": "-1.5m",
  "BulletResist": 12,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Become <span class=\"highlight\">immune to bullets</span>.",
  "GroundDashReductionPercent": -20,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_metal_skin",
  "Name": "Metal Skin",
  "PropertyUpgrades": {
    "AbilityCooldown": -2,
    "ActiveMoveSpeedPenalty": "6.5m",
    "BulletResist": 5,
    "GroundDashReductionPercent": 60
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "metal skin",
      "name": "Metal Skin",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_non_player_bonus" title="Monster Rounds" -->

## Monster Rounds

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_non_player_bonus`
- Snapshot ID: `40133`
- Source-Dokument: `7072`
- Kurzinfo: Monster Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Monster Rounds`
- Payload Hash: `f91e4c31ae80b73299eff07ec80cb37a9c818a622a53c74e33654d712a0df3aa`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.964793+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_non_player_bonus",
  "Name": "Monster Rounds",
  "NonPlayerBonusWeaponPower": 25,
  "NonPlayerBulletResist": 25,
  "OutOfCombatHealthRegen": 1,
  "PropertyUpgrades": {
    "NonPlayerBonusWeaponPower": 35,
    "NonPlayerBulletResist": 35,
    "OutOfCombatHealthRegen": 1
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "monster rounds",
      "name": "Monster Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_burst" title="Mystic Burst" -->

## Mystic Burst

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_burst`
- Snapshot ID: `40113`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Burst aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Burst`
- Payload Hash: `38c257e7a04d1571bef0d9af05cf8fc1b508334acf4aa3e03aeff02ffeed642c`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.905054+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 14,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Damage": 40,
  "Description": "Charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, causing abilities dealing more than <span class=\"highlight\">80</span> damage to deal additional damage.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_burst",
  "MinimumDamage": 80,
  "Name": "Mystic Burst",
  "PropertyUpgrades": {
    "Damage": 60
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
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
      "lookup": "mystic burst",
      "name": "Mystic Burst",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_patrons_blessing" title="Mystic Conduit" -->

## Mystic Conduit

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_patrons_blessing`
- Snapshot ID: `40137`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Conduit aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Conduit`
- Payload Hash: `e3fa9ded35ddf9163ce36fef8b4b776acd3f266416736e5a78487e943fecc0d5`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.978425+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 25,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AllyPercentage": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReduction": 40,
  "Cost": 9999,
  "DamageThreshold": 300,
  "DamageThresholdDuration": 5,
  "Description": "Provides yourself and allies with a powerful spirit focused aura. Has reduced values on allies.",
  "HealAmount": 700,
  "HealRadius": "35m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_patrons_blessing",
  "Name": "Mystic Conduit",
  "PropertyUpgrades": {
    "CooldownReduction": 10,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "Radius": "25m",
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllFriendly"
  ],
  "TechPower": 40,
  "TechRadiusMultiplier": 40,
  "TechRangeMultiplier": 40,
  "Tier": 5,
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
      "lookup": "mystic conduit",
      "name": "Mystic Conduit",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_reach" title="Mystic Expansion" -->

## Mystic Expansion

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_reach`
- Snapshot ID: `40117`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Expansion aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Expansion`
- Payload Hash: `efc91fb39978591fb856fbbdaf586590bcb8562c94057c9be7661c664d08d7c0`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.916119+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Imbue an ability to increase its <span class=\"highlight\">range</span> and <span class=\"highlight\">effect radius</span>.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_reach",
  "Name": "Mystic Expansion",
  "PropertyUpgrades": {
    "TechRadiusMultiplier": 15,
    "TechRangeMultiplier": 15
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechRadiusMultiplier": 20,
  "TechRangeMultiplier": 20,
  "Tier": 1,
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
      "lookup": "mystic expansion",
      "name": "Mystic Expansion",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_mystic_regeneration" title="Mystic Regeneration" -->

## Mystic Regeneration

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystic_regeneration`
- Snapshot ID: `40129`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Regeneration aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Regeneration`
- Payload Hash: `e4e97b28bf3cdec2547999118d60d011f9e3ebe42c2f99bd782395fa308219c9`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.951223+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Dealing {g:citadel_inline_attribute:'SpiritDamage'} to enemy Heroes grants you Bonus {g:citadel_inline_attribute:'Regen'}. Stacks when dealing damage to different heroes.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_mystic_regeneration",
  "Name": "Mystic Regeneration",
  "PropertyUpgrades": {
    "BonusHealth": 150,
    "Regeneration": 8
  },
  "Regeneration": 4,
  "RegenerationDuration": 7,
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "mystic regeneration",
      "name": "Mystic Regeneration",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_mystic_reverb" title="Mystic Reverb" -->

## Mystic Reverb

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystic_reverb`
- Snapshot ID: `40130`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Reverb aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Reverb`
- Payload Hash: `77f5478bb8a6c666072479692c0dd512f3f6eafc3042934d5f357aaa1260a066`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.954766+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 6.25,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 8,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DelayDuration": 3,
  "Description": null,
  "ImbueAbilityLifesteal": 22,
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_mystic_reverb",
  "MaxHealthDamage": 10,
  "MinimumDamage": 100,
  "MovementSpeedSlow": 40,
  "Name": "Mystic Reverb",
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 25,
    "TechDamagePercent": 20
  },
  "Radius": "16m",
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechDamagePercent": 50,
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
      "lookup": "mystic reverb",
      "name": "Mystic Reverb",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_crackshot" title="Mystic Shot" -->

## Mystic Shot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_crackshot`
- Snapshot ID: `40041`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Shot`
- Payload Hash: `77bbad67be252de63e89dac56b5715a66eef18804a8c4aa9a31d600d6a634fc4`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.732027+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Your next bullet deals bonus {g:citadel_inline_attribute:'SpiritDamage'}.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_crackshot",
  "Name": "Mystic Shot",
  "ProcBonusMagicDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.2
    },
    "Value": 40
  },
  "ProcChance": 100,
  "ProcCooldown": 1,
  "PropertyUpgrades": {
    "ProcBonusMagicDamage": 109,
    "SpiritPower": 14
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "SpiritPower": 7,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
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
      "lookup": "mystic shot",
      "name": "Mystic Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_slow" title="Mystic Slow" -->

## Mystic Slow

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_slow`
- Snapshot ID: `40120`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Slow aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Slow`
- Payload Hash: `045c3c13cdce19ff65c3fd07edfa43a04ff2e8463e2fff42633dd77efed6a4b8`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.924162+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 2,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "BonusSprintSpeed": "0.75m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "When the target takes {g:citadel_inline_attribute:'SpiritDamage'}, they have their <span class=\"highlight\">Move Speed</span> reduced.",
  "GroundDashReductionPercent": -12,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_slow",
  "MovementSpeedSlow": 30,
  "Name": "Mystic Slow",
  "PropertyUpgrades": {
    "BonusHealth": 100,
    "BonusSprintSpeed": 1,
    "GroundDashReductionPercent": -10,
    "MovementSpeedSlow": 15
  },
  "ShopFilters": [
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
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
      "lookup": "mystic slow",
      "name": "Mystic Slow",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_vulnerability" title="Mystic Vulnerability" -->

## Mystic Vulnerability

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_vulnerability`
- Snapshot ID: `40123`
- Source-Dokument: `7072`
- Kurzinfo: Mystic Vulnerability aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystic Vulnerability`
- Payload Hash: `a70e730f4dd5e2476ca6d505b093e856a6b927d17338e74fda1386711a9ee557`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.933969+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "When an enemy takes {g:citadel_inline_attribute:'SpiritDamage'}, they have their {g:citadel_inline_attribute:'SpiritResist'} reduced.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_vulnerability",
  "Name": "Mystic Vulnerability",
  "PropertyUpgrades": {
    "TechArmorDamageReduction": -10,
    "TechResist": 8
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechArmorDamageReduction": -8,
  "TechResist": 8,
  "Tier": 2,
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
      "lookup": "mystic vulnerability",
      "name": "Mystic Vulnerability",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_mystical_piano" title="Mystical Piano" -->

## Mystical Piano

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_mystical_piano`
- Snapshot ID: `40131`
- Source-Dokument: `7072`
- Kurzinfo: Mystical Piano aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Mystical Piano`
- Payload Hash: `80630321891f6255233e56bf3bc5f2c33acef74f556002af93482d771d475701`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.957894+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCooldown": 23.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 1.7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "DazeDuration": 2.0,
  "DazeMoveSpeed": "2m",
  "Description": "After a short delay, enemies in the target area will be stunned and have their stamina depleted. After the stun they will be temporarily dazed.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_mystical_piano",
  "Name": "Mystical Piano",
  "PropertyUpgrades": {
    "AbilityCooldown": -15,
    "Radius": "3m"
  },
  "Radius": "12m",
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "StunDelay": 1.7,
  "StunDuration": 2.0,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "mystical piano",
      "name": "Mystical Piano",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_nullification_aura" title="Nullification Burst" -->

## Nullification Burst

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_nullification_aura`
- Snapshot ID: `40135`
- Source-Dokument: `7072`
- Kurzinfo: Nullification Burst aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Nullification Burst`
- Payload Hash: `9194e1728b5cda8bae70911b3c4e429c60abbf89445841beb5a9da80cc45fa8a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.970674+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 18.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 7,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusHealth": 300,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.465
    },
    "Value": 250
  },
  "DamageHeight": "4m",
  "Description": "Removes any positive buffs and prevents stamina usage and healing effects on enemies.",
  "EndRadius": "20m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_nullification_aura",
  "Name": "Nullification Burst",
  "PropertyUpgrades": {
    "BonusHealth": 300,
    "Damage": 200,
    "EndRadius": "6m",
    "StatusResistancePercent": 20
  },
  "ShopFilters": [
    "Disruption"
  ],
  "Slot": "Armor",
  "SpreadDuration": 0.5,
  "StartRadius": "2m",
  "StatusResistancePercent": 40,
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "nullification burst",
      "name": "Nullification Burst",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_omnicharge_pendant" title="Omnicharge Signet" -->

## Omnicharge Signet

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_omnicharge_pendant`
- Snapshot ID: `40136`
- Source-Dokument: `7072`
- Kurzinfo: Omnicharge Signet aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Omnicharge Signet`
- Payload Hash: `e986bad1ed53bbf2d388524b6d866bfa6191c862dbf57cee0de243cdd4e14ba5`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.974752+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityCharges": 4,
  "BonusAbilityChargesNonCharge": 2,
  "BonusSpiritForChargedAbilities": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownBetweenChargeReduction": 70,
  "CooldownReductionOnChargedAbilities": 30,
  "Cost": 9999,
  "Description": "Imbue any active non-ultimate ability with <span class=\"highlight\">Bonus Ability Charges</span>. Already charged abilities receive more bonus charges.",
  "EnableAbilityCharges": 1,
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_omnicharge_pendant",
  "Name": "Omnicharge Signet",
  "PropertyUpgrades": {
    "BonusAbilityCharges": 2,
    "BonusSpiritForChargedAbilities": 30,
    "CooldownBetweenChargeReduction": 5,
    "CooldownReductionOnChargedAbilities": 10
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
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
      "lookup": "omnicharge signet",
      "name": "Omnicharge Signet",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_pristine_emblem" title="Opening Rounds" -->

## Opening Rounds

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_pristine_emblem`
- Snapshot ID: `40142`
- Source-Dokument: `7072`
- Kurzinfo: Opening Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Opening Rounds`
- Payload Hash: `bd21b469b25dec90d0740a425738b3eb16c02ad2995911e2d388ef0fecfb41ea`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.993291+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 8,
  "BaseAttackDamagePercentBonus": 25,
  "BonusBulletSpeedPercent": 60,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_high_velocity_mag"
  ],
  "Cost": 1600,
  "Description": "Your attacks have additional <span class=\"highlight\">Weapon Damage</span> against <span class=\"highlight\">enemies above 50% health</span>.",
  "EnemyLifeThreshold": 50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_pristine_emblem",
  "Name": "Opening Rounds",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BaseAttackDamagePercentBonus": 25,
    "BonusBulletSpeedPercent": 45,
    "TechPower": 18
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 7,
  "Tier": 2,
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
      "lookup": "opening rounds",
      "name": "Opening Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_phantom_strike" title="Phantom Strike" -->

## Phantom Strike

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_phantom_strike`
- Snapshot ID: `40139`
- Source-Dokument: `7072`
- Kurzinfo: Phantom Strike aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Phantom Strike`
- Payload Hash: `260b6a31b5869231c363675141b51cc886988617d999b705e830d910761402ce`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.985311+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.35,
  "AbilityCastRange": "25m",
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BaseAttackDamagePercent": 15,
  "ChannelMoveSpeed": "1.3m",
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Teleport</span> to an enemy target and pull them to the ground. Dealing <span class=\"highlight\">damage</span>, <span class=\"highlight\">Move speed</span> reduction and <span class=\"highlight\">Disarm</span>.",
  "ImpactDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.93
    },
    "Value": 75
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_phantom_strike",
  "Name": "Phantom Strike",
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "BaseAttackDamagePercent": 20,
    "ImpactDamage": 100,
    "TechPower": 12
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Movement",
    "Disruption"
  ],
  "Slot": "Armor",
  "SlowDuration": 3,
  "SlowPercent": 50,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPower": 8,
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
      "lookup": "phantom strike",
      "name": "Phantom Strike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="cosmetic_item_voting_poster" title="Place Poster" -->

## Place Poster

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `cosmetic_item_voting_poster`
- Snapshot ID: `39963`
- Source-Dokument: `7072`
- Kurzinfo: Place Poster aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Place Poster`
- Payload Hash: `417103fc0d9c660d159c2323a2818784b77fcb22cf6c610f6c82630a9321a258`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.552603+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.15,
  "AbilityCastRange": 550,
  "AbilityCooldown": 0.5,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "OnRelease",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": null,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "cosmetic_item_voting_poster",
  "Name": "Place Poster",
  "Radius": 5,
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": null,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": null,
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
      "lookup": "place poster",
      "name": "Place Poster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_deflecting_armor" title="Plated Armor" -->

## Plated Armor

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_deflecting_armor`
- Snapshot ID: `40046`
- Source-Dokument: `7072`
- Kurzinfo: Plated Armor aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Plated Armor`
- Payload Hash: `43feac162a72b0c7e5b39fdd1381caf0fed7a15c91b923ce2924a2eb66255e1e`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.742895+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 130,
  "BulletProcDeflectionPercent": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DeflectionPercent": 30,
  "DeflectionRandomness": 1,
  "Description": "Gain a chance to either deflect incoming bullets, preventing all {g:citadel_inline_attribute:'WeaponDamage'} or prevent all <span class=\"highlight\">on-hit effects</span> from bullets.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_deflecting_armor",
  "Name": "Plated Armor",
  "PropertyUpgrades": {
    "BulletProcDeflectionPercent": 15,
    "DeflectionPercent": 15
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "plated armor",
      "name": "Plated Armor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_close_quarter_combat" title="Point Blank" -->

## Point Blank

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_close_quarter_combat`
- Snapshot ID: `40033`
- Source-Dokument: `7072`
- Kurzinfo: Point Blank aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Point Blank`
- Payload Hash: `2f6cc8d20e979d15178a3459ba6651b633f3b3d881c0c8d4b9bdedf6aef9ac90`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.709936+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "ChannelMoveSpeed": -1,
  "CloseRangeBonusDamageRange": "15m",
  "CloseRangeBonusWeaponPower": 50,
  "Components": [
    "upgrade_close_range"
  ],
  "Cost": 3200,
  "Description": "When in <span class=\"highlight\">close range</span> to your target, gain <span class=\"highlight\">Weapon Damage</span> and your bullets apply a <span class=\"highlight\">Movement Slow</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_close_quarter_combat",
  "MeleeResistPercent": 30,
  "Name": "Point Blank",
  "PropertyUpgrades": {
    "BonusHealth": 150,
    "CloseRangeBonusWeaponPower": 30,
    "MeleeResistPercent": 30,
    "SlowPercent": 5
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability",
    "Disruption"
  ],
  "Slot": "Weapon",
  "SlowDuration": 2,
  "SlowPercent": 25,
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
      "lookup": "point blank",
      "name": "Point Blank",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_prism_blast" title="Prism Blast" -->

## Prism Blast

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_prism_blast`
- Snapshot ID: `40141`
- Source-Dokument: `7072`
- Kurzinfo: Prism Blast aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Prism Blast`
- Payload Hash: `0c069e503a36f39559e42c304f87c00637c2a670c9aa4a34e71c2e5922b43184`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.989823+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 40.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BeamLength": "30m",
  "BeamWidth": "2.9m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.75
    },
    "Value": 270
  },
  "DampingFactor": 3,
  "Description": "You enter a void state and become <span class=\"highlight\">untargetable and invincible</span> for a short duration, during which lasers blast out and rotate around you.",
  "FloatMoveSpeed": "2.5m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_prism_blast",
  "LiftHeight": 100,
  "Name": "Prism Blast",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "AbilityDuration": 2
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "prism blast",
      "name": "Prism Blast",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_quick_silver" title="Quicksilver Reload" -->

## Quicksilver Reload

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_quick_silver`
- Snapshot ID: `40147`
- Source-Dokument: `7072`
- Kurzinfo: Quicksilver Reload aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Quicksilver Reload`
- Payload Hash: `f3f3b2c97485abca84d10c53f8513844b50bae10692ebf1cef1fbdf4c3302e54`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.005999+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 18,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AmmoReloadPercent": 100,
  "BonusFireRate": 10,
  "BuffDuration": 12,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Damage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.16
    },
    "Value": 44
  },
  "Description": "Your imbued ability charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, {g:citadel_inline_attribute:'BonusFireRate'}, and <span class=\"highlight\">reloads bullets</span> on use.",
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_quick_silver",
  "Name": "Quicksilver Reload",
  "PropertyUpgrades": {
    "AbilityChargeUpTime": -4,
    "AbilityCooldown": -4,
    "BonusFireRate": 20,
    "Damage": 56
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 2,
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
      "lookup": "quicksilver reload",
      "name": "Quicksilver Reload",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_resonant_healing" title="Radiant Regeneration" -->

## Radiant Regeneration

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_resonant_healing`
- Snapshot ID: `40159`
- Source-Dokument: `7072`
- Kurzinfo: Radiant Regeneration aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Radiant Regeneration`
- Payload Hash: `583c4cb3df12106df55f91286bde34d5b8ac9a314e4ec67aa816d85387ec2f08`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.037816+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 90,
  "BonusMoveSpeed": "1.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_mystic_regeneration"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heal</span> and gain bonus <span class=\"highlight\">Movement Speed</span> for a short duration when you cast an ability.",
  "HealingPerCast": {
    "Scale": {
      "Type": "power_increase",
      "Value": 2.0
    },
    "Value": 70
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_resonant_healing",
  "Name": "Radiant Regeneration",
  "PropertyUpgrades": {
    "BonusHealth": 110,
    "BonusMoveSpeed": "1m",
    "HealingPerCast": 60,
    "Regeneration": 9
  },
  "Regeneration": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.04
    },
    "Value": 4
  },
  "RegenerationDuration": 7,
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "radiant regeneration",
      "name": "Radiant Regeneration",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_rapid_recharge" title="Rapid Recharge" -->

## Rapid Recharge

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rapid_recharge`
- Snapshot ID: `40148`
- Source-Dokument: `7072`
- Kurzinfo: Rapid Recharge aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rapid Recharge`
- Payload Hash: `1e1d3b44a18f36e98460f7db731ec70ca554efaad7c67f41286544cde644aae4`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.008348+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityCharges": 2,
  "BonusSpiritForChargedAbilities": 14,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_extra_charge"
  ],
  "CooldownBetweenChargeReduction": 30,
  "CooldownReductionOnChargedAbilities": 14,
  "Cost": 3200,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rapid_recharge",
  "Name": "Rapid Recharge",
  "PropertyUpgrades": {
    "BonusAbilityCharges": 2,
    "BonusSpiritForChargedAbilities": 20,
    "CooldownBetweenChargeReduction": 5,
    "CooldownReductionOnChargedAbilities": 15
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "rapid recharge",
      "name": "Rapid Recharge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_rapid_rounds" title="Rapid Rounds" -->

## Rapid Rounds

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rapid_rounds`
- Snapshot ID: `40149`
- Source-Dokument: `7072`
- Kurzinfo: Rapid Rounds aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rapid Rounds`
- Payload Hash: `916a02e66a2c77661c436b27cf08d01bc63d4d5a57538fa68c8b3589848a6087`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.011331+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 9,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rapid_rounds",
  "Name": "Rapid Rounds",
  "PropertyUpgrades": {
    "BonusFireRate": 15
  },
  "ShopFilters": [
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "rapid rounds",
      "name": "Rapid Rounds",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_vex_barrier" title="Reactive Barrier" -->

## Reactive Barrier

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_vex_barrier`
- Snapshot ID: `40217`
- Source-Dokument: `7072`
- Kurzinfo: Reactive Barrier aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Reactive Barrier`
- Payload Hash: `4d05fb9c1e49e0661f24e0715eba37e64f90197bfcd70b93cfb1667820d94aa3`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.199243+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 55,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> when you are <span class=\"highlight\">Stunned, Chained, Immobilized, Slept or Silenced</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_vex_barrier",
  "Name": "Reactive Barrier",
  "OutOfCombatHealthRegen": 1.0,
  "PropertyUpgrades": {
    "AbilityCooldown": -15,
    "VexBarrierCombatBarrier": 375
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "VexBarrierCombatBarrier": {
    "Scale": {
      "Type": "spirit",
      "Value": 1.8
    },
    "Value": 325
  },
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
      "lookup": "reactive barrier",
      "name": "Reactive Barrier",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_melee_rebuttal" title="Rebuttal" -->

## Rebuttal

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_melee_rebuttal`
- Snapshot ID: `40126`
- Source-Dokument: `7072`
- Kurzinfo: Rebuttal aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rebuttal`
- Payload Hash: `97da5a1fdb3bd1ed2d7f84df6894c3c73559621f62ef61d312bdbc8fea5ce885`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.943340+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusDamagePercent": 30,
  "BonusHealth": 75,
  "BuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "On a successful <span class=\"highlight\">Parry</span> against an enemy Hero, <span class=\"highlight\">Heal</span> yourself for the damage parried and returns that damage to the target, and temporarily gain increased <span class=\"highlight\">damage.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_melee_rebuttal",
  "MeleeResistPercent": 18,
  "Name": "Rebuttal",
  "ParryCooldownReduction": 1.75,
  "ParrySuccessHealPercentage": 100,
  "PropertyUpgrades": {
    "BonusDamagePercent": 20,
    "BonusHealth": 150,
    "MeleeResistPercent": 22,
    "ParryCooldownReduction": 0.5
  },
  "ShopFilters": [
    "Durability",
    "Melee",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
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
      "lookup": "rebuttal",
      "name": "Rebuttal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_rechargingbullets" title="Recharging Rush" -->

## Recharging Rush

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rechargingbullets`
- Snapshot ID: `40151`
- Source-Dokument: `7072`
- Kurzinfo: Recharging Rush aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Recharging Rush`
- Payload Hash: `30ee0989ec48275b32468059158cc16dcff250a849f503d9c515c600427b8ca0`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.017067+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 25,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 10,
  "BonusClipSizePercent": 20,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "DamageThreshold": 200,
  "DamageWindow": 3.5,
  "Description": "Dealing significant {g:citadel_inline_attribute:'WeaponDamage'} replenishes a charge for <span class=\"highlight\">each of your charged abilities</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rechargingbullets",
  "Name": "Recharging Rush",
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "BaseAttackDamagePercent": 30,
    "BonusClipSizePercent": 30
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "recharging rush",
      "name": "Recharging Rush",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_ability_refresher" title="Refresher" -->

## Refresher

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ability_refresher`
- Snapshot ID: `39977`
- Source-Dokument: `7072`
- Kurzinfo: Refresher aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Refresher`
- Payload Hash: `1077b809cc48c61126b87a1c7e28938e5fb90718ac035bd594ac7e7f2e84803d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.585383+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.6,
  "AbilityCooldown": 300,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BulletResist": 15,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "<span class=\"highlight\">Reset the cooldown</span> of all your abilities and <span class=\"highlight\">restore all your charges</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ability_refresher",
  "Name": "Refresher",
  "PropertyUpgrades": {
    "AbilityCooldown": -210
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 14,
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
      "lookup": "refresher",
      "name": "Refresher",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_rescue_beam" title="Rescue Beam" -->

## Rescue Beam

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_rescue_beam`
- Snapshot ID: `40157`
- Source-Dokument: `7072`
- Kurzinfo: Rescue Beam aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rescue Beam`
- Payload Hash: `c895637fd29291e4712f2cf20d4240689e4ab9cc0401658ae539a55e9142adc1`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.032052+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": "35m",
  "AbilityChannelTime": 2.5,
  "AbilityCooldown": 60.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusMoveSpeed": "0m",
  "BonusSprintSpeed": "0.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health_stimpak"
  ],
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Heals</span> a target allied hero and yourself for a percentage of <span class=\"highlight\">Max Health</span>. Once while healing, you can <span class=\"highlight\">Pull</span> the target towards you. Can be self-cast.",
  "HealInterval": 0.2,
  "HealPercentAmount": 20,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_rescue_beam",
  "Name": "Rescue Beam",
  "PropertyUpgrades": {
    "AbilityCooldown": -45,
    "HealPercentAmount": 15,
    "TechRadiusMultiplier": 20,
    "TechRangeMultiplier": 20
  },
  "SelfModifier": 100,
  "ShopFilters": [
    "MagicDamage",
    "Durability",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechRadiusMultiplier": 6,
  "TechRangeMultiplier": 6,
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
      "lookup": "rescue beam",
      "name": "Rescue Beam",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_restorative_locket" title="Restorative Locket" -->

## Restorative Locket

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_restorative_locket`
- Snapshot ID: `40160`
- Source-Dokument: `7072`
- Kurzinfo: Restorative Locket aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Restorative Locket`
- Payload Hash: `cdd317a9e0db065af8bd69ed1e088699fcf5fed5b82952d62a12bab49b7f8faa`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.040660+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "35m",
  "AbilityCooldown": 20.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "HealPerStack": {
    "Scale": {
      "Type": "power_increase",
      "Value": 0.32
    },
    "Value": 16
  },
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_restorative_locket",
  "MaxStacks": 25,
  "MaxStaminaRestore": 3,
  "Name": "Restorative Locket",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "HealPerStack": 30,
    "MaxStaminaRestore": 2,
    "MinStaminaRestore": 2,
    "TechResist": 10
  },
  "Radius": "35m",
  "ShopFilters": [
    "Healing",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechResist": 10,
  "Tier": 2,
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
      "lookup": "restorative locket",
      "name": "Restorative Locket",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_medic_bullets" title="Restorative Shot" -->

## Restorative Shot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_medic_bullets`
- Snapshot ID: `40124`
- Source-Dokument: `7072`
- Kurzinfo: Restorative Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Restorative Shot`
- Payload Hash: `062505acb32557b477da6a1296fdd5034bb689f4c4a116dfc5f5b3e46df5c187`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.936872+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Your next bullet will <span class=\"highlight\">heal</span> you based on what target you hit.",
  "HealFromHero": 50,
  "HealFromNPC": 20,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_medic_bullets",
  "Name": "Restorative Shot",
  "ProcChance": 100,
  "PropertyUpgrades": {
    "HealFromHero": 100,
    "HealFromNPC": 40
  },
  "Radius": "1m",
  "ShopFilters": [
    "WeaponDamage",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 1,
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
      "lookup": "restorative shot",
      "name": "Restorative Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_return_fire" title="Return Fire" -->

## Return Fire

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_return_fire`
- Snapshot ID: `40161`
- Source-Dokument: `7072`
- Kurzinfo: Return Fire aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Return Fire`
- Payload Hash: `afb67a05baa31299dc8329c7aa125ce6328600c6563e42a18d49523cae0b2f55`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.043005+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 23.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BulletDamageReflectedPct": 65,
  "BulletResist": 10,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Automatically <span class=\"highlight\">fire a bullet</span> towards any attacker who damages you with their abilities or weapon.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_return_fire",
  "Name": "Return Fire",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "BulletDamageReflectedPct": 25,
    "BulletResist": 16,
    "SpiritDamageReflectedPct": 15
  },
  "ShopFilters": [
    "Durability",
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Armor",
  "SpiritDamageReflectedPct": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "return fire",
      "name": "Return Fire",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_ricochet" title="Ricochet" -->

## Ricochet

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_ricochet`
- Snapshot ID: `40162`
- Source-Dokument: `7072`
- Kurzinfo: Ricochet aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Ricochet`
- Payload Hash: `f886875c09c8287a1a6e8bde3d94b1ac31ff48cc944ac437a6a37bc03002554f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.045885+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 18,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Your bullets will <span class=\"highlight\">ricochet</span> on enemies near your target, <span class=\"highlight\">applying any bullet procs</span> and <span class=\"highlight\">dealing a percentage of the original damage.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_ricochet",
  "Name": "Ricochet",
  "PropertyUpgrades": {
    "BonusFireRate": 25,
    "RicochetDamagePercent": 15
  },
  "RicochetDamagePercent": 65,
  "RicochetRadius": "13m",
  "RicochetTargetsTooltipOnly": 2,
  "ShopFilters": [
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "BossEnemy",
    "TrooperEnemy",
    "PropEnemy",
    "MinionEnemy",
    "Neutral"
  ],
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
      "lookup": "ricochet",
      "name": "Ricochet",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_runed_gauntlets" title="Runed Gauntlets" -->

## Runed Gauntlets

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_runed_gauntlets`
- Snapshot ID: `40165`
- Source-Dokument: `7072`
- Kurzinfo: Runed Gauntlets aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Runed Gauntlets`
- Payload Hash: `2d31b8bb5936146bc4e99f416781d9c791f6ee244be8c5ef9ef6611b40acdf8d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.054218+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 10,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusMeleeDamagePercent": 30,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReductionOnHitMin": 4,
  "CooldownReductionOnHitPct": 16,
  "Cost": 9999,
  "Description": "Everytime you land a heavy melee, your existing cooldowns get reduced.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_runed_gauntlets",
  "MeleeDistanceScale": 150,
  "MeleeResistPercent": 50,
  "Name": "Runed Gauntlets",
  "PropertyUpgrades": {
    "CooldownReductionOnHitPct": 6,
    "MeleeDistanceScale": 50,
    "MeleeResistPercent": 15
  },
  "ShopFilters": null,
  "Slot": "Weapon",
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 5,
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
      "lookup": "runed gauntlets",
      "name": "Runed Gauntlets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_withering_whip" title="Rusted Barrel" -->

## Rusted Barrel

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_withering_whip`
- Snapshot ID: `40227`
- Source-Dokument: `7072`
- Kurzinfo: Rusted Barrel aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Rusted Barrel`
- Payload Hash: `2cf2f5895af96d95c43c63635b08a423e3074497b4c2fc9cc62dd29b93a8871b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.224201+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "32m",
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 60,
  "BonusSprintSpeed": "0.5m",
  "BulletArmorReduction": -8,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "Target an enemy to reduce their <span class=\"highlight\">Fire Rate</span> and <span class=\"highlight\">Bullet Resistance</span>.",
  "FireRateSlow": 32,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_withering_whip",
  "Name": "Rusted Barrel",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BonusHealth": 130,
    "BulletArmorReduction": -4,
    "FireRateSlow": 20
  },
  "ShopFilters": [
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 1,
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
      "lookup": "rusted barrel",
      "name": "Rusted Barrel",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_discord" title="Scourge" -->

## Scourge

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_discord`
- Snapshot ID: `40048`
- Source-Dokument: `7072`
- Kurzinfo: Scourge aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Scourge`
- Payload Hash: `fcec22a2167b448b7ef153e5cd1298cc858a3dea98a5c2828af59e1a1d823c8d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.747027+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.25,
  "AbilityCastRange": "35m",
  "AbilityCooldown": 35.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "AuraRadius": "10m",
  "BonusHealth": 100,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": "Apply <span class=\"highlight\">Spirit Resist</span> and an aura on a friendly target that deals <span class=\"highlight\">{g:citadel_inline_attribute:'SpiritIcon'}damage</span> to enemies proportional to their max health. <br>Can be self cast.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_discord",
  "MaxHealthPercentAsDPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.0055
    },
    "Value": 2.6
  },
  "Name": "Scourge",
  "PropertyUpgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 125,
    "CombatBarrier": 300,
    "MaxHealthPercentAsDPS": 2,
    "StatusResistancePercent": 20
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StatusResistancePercent": 17,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "TechResist": 40,
  "TickRate": 0.25,
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
      "lookup": "scourge",
      "name": "Scourge",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_icarus_wings" title="Seraphim Wings" -->

## Seraphim Wings

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_icarus_wings`
- Snapshot ID: `40097`
- Source-Dokument: `7072`
- Kurzinfo: Seraphim Wings aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Seraphim Wings`
- Payload Hash: `822b236d43146e818f62f36f112e7217c8cdc4448ab1ece900f69183b007628c`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.863331+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AirBonusDamageGiven": 40,
  "AirBonusDamageTaken": -40,
  "AirControlAccelPercent": 50,
  "AirControlPercent": 100,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "When Airborne, You deal more damage and take reduced damage. Allows <span class=\"highlight\">unlimited</span> Air Dash and Jumping.",
  "GravityScale": -70,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_icarus_wings",
  "Name": "Seraphim Wings",
  "PropertyUpgrades": {
    "AirBonusDamageGiven": 10,
    "AirBonusDamageTaken": -10,
    "StaminaCooldownReduction": 30
  },
  "ShopFilters": null,
  "Slot": "Armor",
  "StaminaCooldownReduction": 120,
  "StreetBrawl": true,
  "TargetTypes": null,
  "Tier": 5,
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
      "lookup": "seraphim wings",
      "name": "Seraphim Wings",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_shadow_strike" title="Shadow Strike" -->

## Shadow Strike

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_shadow_strike`
- Snapshot ID: `40169`
- Source-Dokument: `7072`
- Kurzinfo: Shadow Strike aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Strike`
- Payload Hash: `84f4c114eefe18b86301e1893d624444d46c403363bf9bcbfbe7675662485ef5`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.065624+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 350,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.4
    },
    "Value": 125
  },
  "Description": "Go <span class=\"highlight\">Invisible</span> on <span class=\"highlight\">Stamina use</span> with no detection range. Doing a <span class=\"highlight\">melee attack</span> while invisible will cause you to <span class=\"highlight\">steal bullet and spirit resistance</span> from them and deal <span class=\"highlight\">damage over time</span>.",
  "InvisAlertWhenFading": 1,
  "InvisFadeToDuration": 0.2,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shadow_strike",
  "MaxStacks": 1,
  "Name": "Shadow Strike",
  "PropertyUpgrades": {
    "AbilityDuration": 3,
    "BonusHealth": 250,
    "DPS": 125,
    "ResistStealAmount": 20,
    "Stamina": 1
  },
  "ResistStealAmount": 40,
  "RevealOnDamageDuration": 1.5,
  "RevealOnSpottedDuration": 0.25,
  "ShopFilters": null,
  "Slot": "Armor",
  "SpottedRadius": "0m",
  "Stamina": 3,
  "StealDuration": 6,
  "StreetBrawl": true,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TickRate": 1,
  "Tier": 5,
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
      "lookup": "shadow strike",
      "name": "Shadow Strike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_cloaking_device_active" title="Shadow Weave" -->

## Shadow Weave

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cloaking_device_active`
- Snapshot ID: `40032`
- Source-Dokument: `7072`
- Kurzinfo: Shadow Weave aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shadow Weave`
- Payload Hash: `6429851c2d120d06fab8ccd8794c74d434de0c2ade4c8e67919b3fb8911a7d23`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.707417+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 45.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 13,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "AmbushBonusFireRate": 25,
  "AmbushBonusMeleeDamage": 25,
  "AmbushBonusTechPower": 25,
  "AmbushDuration": 5,
  "BonusSprintSpeed": "1.5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Become <span class=\"highlight\">Stealthed</span>. Whenever you take damage while Stealthed you get briefly revealed.",
  "FullInvisDistance": "30m",
  "InvisAlertWhenFading": 1,
  "InvisCancelOnDamage": 1,
  "InvisFadeToDuration": 0.6,
  "InvisMoveSpeedMod": "5m",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cloaking_device_active",
  "Name": "Shadow Weave",
  "OutOfCombatHealthRegen": 5,
  "PropertyUpgrades": {
    "AbilityCooldown": -17,
    "AmbushBonusFireRate": 35,
    "AmbushBonusMeleeDamage": 30,
    "AmbushBonusTechPower": 35,
    "OutOfCombatHealthRegen": 20
  },
  "RevealOnDamageDuration": 1.5,
  "RevealOnSpottedDuration": 1.5,
  "ShopFilters": [
    "MagicDamage",
    "FireRate",
    "Movement"
  ],
  "Slot": "Weapon",
  "SpottedRadius": "20m",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "shadow weave",
      "name": "Shadow Weave",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_sharpshooter" title="Sharpshooter" -->

## Sharpshooter

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_sharpshooter`
- Snapshot ID: `40170`
- Source-Dokument: `7072`
- Kurzinfo: Sharpshooter aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sharpshooter`
- Payload Hash: `e5a557c1bc988b764490e9499c4144b56ccea4dd41eed0b4425408ae66424053`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.069412+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 10,
  "BonusAttackRangePercent": 20,
  "BonusBulletSpeedPercent": 60,
  "BonusMoveSpeed": "-0.7m",
  "BonusSprintSpeed": "1.0m",
  "BonusZoomPercent": 25,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_long_range",
    "upgrade_high_velocity_mag"
  ],
  "Cost": 3200,
  "Description": "Deal additional <span class=\"highlight\">Weapon Damage</span> when <span class=\"highlight\">beyond a minimum distance</span> from your target.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_sharpshooter",
  "LongRangeBonusWeaponPower": 60,
  "LongRangeBonusWeaponPowerMinRange": "15m",
  "Name": "Sharpshooter",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 15,
    "BonusAttackRangePercent": 10,
    "BonusBulletSpeedPercent": 45,
    "LongRangeBonusWeaponPower": 40
  },
  "ShopFilters": [
    "WeaponDamage",
    "ClipSize"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "sharpshooter",
      "name": "Sharpshooter",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_shrink_ray" title="Shrink Ray" -->

## Shrink Ray

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_shrink_ray`
- Snapshot ID: `40172`
- Source-Dokument: `7072`
- Kurzinfo: Shrink Ray aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Shrink Ray`
- Payload Hash: `ed6da1491f5564e960143d633b4b2b18d84b0694f550224a6ef35c1a1342b6b0`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.075873+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 30,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusFireRate": 20,
  "BonusMoveSpeed": "5.0m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "Reduces <span class=\"highlight\">Model Size</span> and grants <span class=\"highlight\">Move Speed</span> to the target. Allows <span class=\"highlight\">usage of tunnels</span> in this mode. Can be self-cast.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_shrink_ray",
  "ModelScaleGrowth": 0.5,
  "ModelScaleGrowthTooltip": -50,
  "Name": "Shrink Ray",
  "PropertyUpgrades": {
    "AbilityCooldown": -15,
    "BonusFireRate": 20,
    "BonusMoveSpeed": "2m",
    "ModelScaleGrowth": -0.15,
    "ModelScaleGrowthTooltip": -15
  },
  "ShopFilters": [
    "Healing"
  ],
  "ShrinkDuration": 60,
  "Slot": "Tech",
  "StreetBrawl": true,
  "TargetTypes": [
    "HeroFriendly"
  ],
  "Tier": 5,
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
      "lookup": "shrink ray",
      "name": "Shrink Ray",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_targeted_silence" title="Silence Wave" -->

## Silence Wave

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_targeted_silence`
- Snapshot ID: `40197`
- Source-Dokument: `7072`
- Kurzinfo: Silence Wave aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silence Wave`
- Payload Hash: `323b9de78ec8b19de8ffad8325faed4112ad545079ddb94c153197db8d47c74b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.144752+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

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

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_proc_silence" title="Silencer" -->

## Silencer

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_proc_silence`
- Snapshot ID: `40144`
- Source-Dokument: `7072`
- Kurzinfo: Silencer aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Silencer`
- Payload Hash: `6b0889c48678c3874ee40dfbe273c770721cfcca4ba968e80402181165ba1ea6`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.998006+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BuildUpDuration": 5,
  "BuildUpPerShot": 1.04,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "DebuffDuration": 6,
  "Description": "Your bullets build up to a <span class=\"highlight\">Silence</span>. Victims are immune to the build up for <span class=\"highlight\">10s</span> after silence expires.",
  "ImmunityDuration": 10,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_proc_silence",
  "Name": "Silencer",
  "PropertyUpgrades": {
    "SilenceDuration": 1.25,
    "TechDamageReduction": -15,
    "TechResist": 15
  },
  "ShopFilters": [
    "Disruption",
    "WeaponDamage"
  ],
  "SilenceDuration": 2.5,
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechDamageReduction": -25,
  "TechResist": 12,
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
      "lookup": "silencer",
      "name": "Silencer",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_siphon_bullets" title="Siphon Bullets" -->

## Siphon Bullets

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_siphon_bullets`
- Snapshot ID: `40174`
- Source-Dokument: `7072`
- Kurzinfo: Siphon Bullets aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Siphon Bullets`
- Payload Hash: `f5c425ea20bdd3a943513b43236be552edc87775973fd93fc453b9b5a81ab7bf`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.083397+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 15,
  "BulletResist": 10,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 6400,
  "Description": null,
  "HealthStealPctHero": 2.5,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_siphon_bullets",
  "MaxStacks": 9999,
  "Name": "Siphon Bullets",
  "ParticleRadius": "1m",
  "ProcCooldown": 1.2,
  "PropertyUpgrades": {
    "BulletResist": 10,
    "HealthStealPctHero": 1.5
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing"
  ],
  "Slot": "Armor",
  "StackLostPerDeath": 2,
  "StealDuration": 17,
  "StealPerHit": 1,
  "StealPerKill": 1,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
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
      "lookup": "siphon bullets",
      "name": "Siphon Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_slowing_bullets" title="Slowing Bullets" -->

## Slowing Bullets

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_slowing_bullets`
- Snapshot ID: `40176`
- Source-Dokument: `7072`
- Kurzinfo: Slowing Bullets aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slowing Bullets`
- Payload Hash: `adce4eddb70abf920bfbc23876ea066b92663dc8d1f7460c44e606b612c4787c`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.088673+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 15,
  "BuildUpDuration": 5,
  "BuildUpPerShot": 0.7,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Your bullets build up a <span class=\"highlight\">Movement Slow</span> on enemies.",
  "GroundDashReductionPercent": -22,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_slowing_bullets",
  "Name": "Slowing Bullets",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 30,
    "GroundDashReductionPercent": -10,
    "SlowPercent": 20
  },
  "ShopFilters": [
    "WeaponDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "SlowDuration": 3.5,
  "SlowPercent": 30,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
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
      "lookup": "slowing bullets",
      "name": "Slowing Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_containment" title="Slowing Hex" -->

## Slowing Hex

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_containment`
- Snapshot ID: `40037`
- Source-Dokument: `7072`
- Kurzinfo: Slowing Hex aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Slowing Hex`
- Payload Hash: `d3e403174ad51aa1c0fc8c50aba52080c3fa9b23a2097c42430545a7f546f66e`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.720397+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "25m",
  "AbilityCooldown": 27,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 3.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusSprintSpeed": "0.5m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "<span class=\"highlight\">Slows movement</span> of enemy target. Also <span class=\"highlight\">Silences</span> their <span class=\"highlight\">movement-based items and abilities</span>.<br><span class=\"diminish\">Increases the target's gravity.<br>Does not affect target's stamina usage.</span>",
  "GroundDashReductionPercent": -30,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_containment",
  "Name": "Slowing Hex",
  "PropertyUpgrades": {
    "AbilityCooldown": -18,
    "GroundDashReductionPercent": -6,
    "SlowPercent": 10
  },
  "ShopFilters": [
    "Movement",
    "Disruption",
    "MagicDamage"
  ],
  "Slot": "Tech",
  "SlowPercent": 20,
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "Tier": 2,
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
      "lookup": "slowing hex",
      "name": "Slowing Hex",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="cosmetic_item_snowball" title="Snowball" -->

## Snowball

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `cosmetic_item_snowball`
- Snapshot ID: `39962`
- Source-Dokument: `7072`
- Kurzinfo: Snowball aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Snowball`
- Payload Hash: `9a8112154833e700282f875cd7066d7c4b09c4500e9c376940f82957f38817cf`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.548883+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCharges": 1,
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": 1.5,
  "AbilityPostCastDuration": 1,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": null,
  "Damage": 1,
  "Description": "Throw snowballs at your friends and enemies. Distance and other properties improve as you make progress in the <span class=\"highlight\">2025 Holiday Challenge</span>.<br><br><span class=\"diminish\">Throwing a Snowball at an ally won't inflict damage and will reset its cooldown.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "cosmetic_item_snowball",
  "MaxLevel": 32,
  "Name": "Snowball",
  "Progression": {
    "Charges": {
      "Behavior": "UsePrevious",
      "Levels": {
        "0": 1.0,
        "16": 3.0,
        "24": 4.0,
        "32": 5.0,
        "8": 2.0
      }
    },
    "Cooldown": {
      "Levels": {
        "0": 18.0,
        "1": 16.0,
        "2": 14.0,
        "3": 12.0,
        "4": 10.0,
        "5": 8.0
      }
    },
    "Damage": {
      "Levels": {
        "0": 1.0,
        "1": 2.0,
        "2": 4.0,
        "3": 8.0,
        "32": 20.0,
        "4": 12.0
      }
    },
    "Radius": {
      "Levels": {
        "0": 5.0,
        "32": 20.0
      }
    },
    "SnowballCount": {
      "Behavior": "UsePrevious",
      "Levels": {
        "0": 1.0,
        "16": 2.0,
        "32": 3.0
      }
    },
    "Speed": {
      "Levels": {
        "0": 630.0,
        "1": 787.0,
        "2": 944.0,
        "3": 1102.0,
        "32": 2756.0,
        "4": 1260.0,
        "5": 1417.0,
        "6": 1575.0
      }
    }
  },
  "Radius": 5,
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": null,
  "SnowballCount": 1,
  "SnowballSpeed": 630,
  "Spread": 3,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllFriendly",
    "AllEnemy"
  ],
  "Tier": null,
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
      "lookup": "snowball",
      "name": "Snowball",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_spellbreaker" title="Spellbreaker" -->

## Spellbreaker

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellbreaker`
- Snapshot ID: `40180`
- Source-Dokument: `7072`
- Kurzinfo: Spellbreaker aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spellbreaker`
- Payload Hash: `1a02adfbac7346bcf7f2e8d87f0851ffb60e0e97acb75f530963f082f502d0a7`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.099389+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 9,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 90,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "DamageThreshold": 175,
  "Description": "The next instance of high {g:citadel_inline_attribute:'SpiritDamage'} you take is significantly reduced.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellbreaker",
  "Name": "Spellbreaker",
  "PropertyUpgrades": {
    "AbilityCooldown": -3,
    "StatusResistancePercent": 15,
    "TechResist": 15
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "SpiritDamageReductionProc": 65,
  "StatusResistancePercent": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 18,
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
      "lookup": "spellbreaker",
      "name": "Spellbreaker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_enchanted_holsters" title="Spellslinger" -->

## Spellslinger

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_enchanted_holsters`
- Snapshot ID: `40057`
- Source-Dokument: `7072`
- Kurzinfo: Spellslinger aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spellslinger`
- Payload Hash: `95e88656959070fa2766dcd40eb7649b0ba09c3ba4fc892753fc1a76e0255cae`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.769771+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 11,
  "BuffDuration": 18,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReduction": 5,
  "Cost": 6400,
  "Description": "While in-combat whenever you cast an ability or item, gain a stacking buff that improves fire rate and reload speed. <br><span class=\"diminish\">Each stack refreshes the duration.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_enchanted_holsters",
  "MaxStacks": 6,
  "Name": "Spellslinger",
  "PropertyUpgrades": {
    "BonusFireRate": 6,
    "CooldownReduction": 8,
    "ReloadSpeedMultipler": -3
  },
  "ReloadSpeedMultipler": -10,
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "spellslinger",
      "name": "Spellslinger",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_spirit_burn" title="Spirit Burn" -->

## Spirit Burn

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_burn`
- Snapshot ID: `40184`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Burn aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Burn`
- Payload Hash: `ba24442524a3b3acc7e78418a08023479c56c89cec039c1f05ff390a14162b3d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.109801+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 20,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReductionPctOnNonHeroes": 50,
  "Cost": 6400,
  "DPS": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.06
    },
    "Value": 24
  },
  "DamagePctVsNonHeroes": 50,
  "DamageThreshold": 500,
  "DamageThresholdDuration": 5,
  "DebuffDuration": 8,
  "Description": "Dealing significant {g:citadel_inline_attribute:'SpiritDamage'} to an enemy within 5s causes an explosion dealing damage and a burn to nearby enemies. While burning, enemies take damage over time and receive reduced healing.<br><span class=\"diminish\">Deals half-damage and has half-cooldown on non-heroes.</span>",
  "ExplosionDamage": 110,
  "ExplosionRadius": "12m",
  "HealAmpReceivePenaltyPercent": -70,
  "HealAmpRegenPenaltyPercent": -70,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_burn",
  "Name": "Spirit Burn",
  "PropertyUpgrades": {
    "AbilityCooldown": -6,
    "DPS": 20,
    "ExplosionDamage": 160,
    "TechRadiusMultiplier": 12,
    "TechRangeMultiplier": 12
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechRadiusMultiplier": 6,
  "TechRangeMultiplier": 6,
  "TickRate": 0.5,
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
      "lookup": "spirit burn",
      "name": "Spirit Burn",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_health_stealing_magic" title="Spirit Lifesteal" -->

## Spirit Lifesteal

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_health_stealing_magic`
- Snapshot ID: `40092`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Lifesteal aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Lifesteal`
- Payload Hash: `a8d95d68d9e6940b217b6dca50f77f5441674357c638fd3995b1215a06d8f7be`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.852553+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 13,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 90,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_health_stealing_magic",
  "Name": "Spirit Lifesteal",
  "NonHeroAbilityLifestealTooltipOnly": 3,
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 14,
    "BonusHealth": 80,
    "TechPower": 9
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 6,
  "Tier": 2,
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
      "lookup": "spirit lifesteal",
      "name": "Spirit Lifesteal",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_spellslinger_headshots" title="Spirit Rend" -->

## Spirit Rend

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spellslinger_headshots`
- Snapshot ID: `40182`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Rend aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Rend`
- Payload Hash: `8676e7d8ab1a57208a1dbcbd343119cf341c4fe1e80c5540c74ebde40faccf9e`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.104588+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_tech_defense_shredders"
  ],
  "Cost": 3200,
  "DebuffDuration": 8,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spellslinger_headshots",
  "MagicResistReduction": -7,
  "MaxStacks": 4,
  "Name": "Spirit Rend",
  "ProcCooldown": 2,
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 10,
    "MagicResistReduction": -5,
    "TechArmorDamageReduction": -10
  },
  "ShopFilters": [
    "WeaponDamage",
    "MagicDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechArmorDamageReduction": -8,
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
      "lookup": "spirit rend",
      "name": "Spirit Rend",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_tech_purge" title="Spirit Resilience" -->

## Spirit Resilience

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_purge`
- Snapshot ID: `40203`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Resilience aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Resilience`
- Payload Hash: `732bb269b8726d78dc19cd69458cdc01ce7286542a2833cc4c806f3e744a9dc0`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.159977+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "When below <span class=\"highlight\">50% health</span>, gain additional Spirit Resist.",
  "HealthThreshold": 50,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_purge",
  "Name": "Spirit Resilience",
  "OutOfCombatHealthRegen": 3,
  "PropertyUpgrades": {
    "TechResist": 10,
    "TechResistBelowThreshold": 10
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 30,
  "TechResistBelowThreshold": 15,
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
      "lookup": "spirit resilience",
      "name": "Spirit Resilience",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_spirit_sap" title="Spirit Sap" -->

## Spirit Sap

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_sap`
- Snapshot ID: `40185`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Sap aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Sap`
- Payload Hash: `da1d41937d4d7308aca67961334d1e3adeddd56337064ea8c4bd5f9a1284401a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.112516+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.1,
  "AbilityCastRange": "40m",
  "AbilityCooldown": 18,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 12,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusHealth": 50,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Target an enemy to <span class=\"highlight\">reduce their Spirit Resist</span> and <span class=\"highlight\">Spirit Power</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_sap",
  "MagicResistReduction": -9,
  "Name": "Spirit Sap",
  "PropertyUpgrades": {
    "AbilityCooldown": -12,
    "BonusHealth": 150,
    "MagicResistReduction": -12,
    "TechPowerReduction": -26
  },
  "ShopFilters": [
    "MagicDamage"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPowerReduction": -30,
  "Tier": 2,
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
      "lookup": "spirit sap",
      "name": "Spirit Sap",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_spirit_bubble" title="Spirit Shielding" -->

## Spirit Shielding

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_bubble`
- Snapshot ID: `40183`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Shielding aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Shielding`
- Payload Hash: `659548580220c290ba528264527a5910741f0cb853840cd586f0eebb82ff6b91`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.107407+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 45,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BarrierDuration": 8,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "power_increase",
      "Value": 5
    },
    "Value": 300
  },
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "DamageThreshold": 225,
  "DamageWindow": 3.5,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'SpiritDamage'} from enemy Heroes in a small time frame.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_bubble",
  "Name": "Spirit Shielding",
  "OutOfCombatHealthRegen": 2.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "CombatBarrier": 175,
    "OutOfCombatHealthRegen": 3,
    "TechResist": 20
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechResist": 18,
  "Tier": 2,
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
      "lookup": "spirit shielding",
      "name": "Spirit Shielding",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_tech_defense_shredders" title="Spirit Shredder Bullets" -->

## Spirit Shredder Bullets

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_defense_shredders`
- Snapshot ID: `40201`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Shredder Bullets aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Shredder Bullets`
- Payload Hash: `0ab374306777777de43803389423dd214f36c0723f6969d93f0a430ee3c9aeba`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.155380+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityLifestealPercentHero": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "DebuffDuration": 8,
  "Description": "Your bullets apply a debuff that reduces the <span class=\"highlight\">Spirit Resist</span> of the target and grants you and your allies <span class=\"highlight\">Spirit Lifesteal</span> against them.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_defense_shredders",
  "Name": "Spirit Shredder Bullets",
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 10,
    "TechArmorDamageReduction": -10
  },
  "ShopFilters": [
    "MagicDamage",
    "Healing"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechArmorDamageReduction": -8,
  "Tier": 2,
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
      "lookup": "spirit shredder bullets",
      "name": "Spirit Shredder Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_spirit_snatch" title="Spirit Snatch" -->

## Spirit Snatch

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_spirit_snatch`
- Snapshot ID: `40186`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Snatch aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Snatch`
- Payload Hash: `7fd5fd88024c83f73b5237bcf2395ca32d0f79640616d3154841ef8a12f85655`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.115270+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 10,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 75,
  "BonusMeleeDamagePercent": 7,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_acolytes_glove"
  ],
  "Cost": 3200,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, the attack deals extra {g:citadel_inline_attribute:'SpiritDamage'} and steals <span class=\"highlight\">Spirit Resist</span> and <span class=\"highlight\">Spirit Power</span>.<span class=\"diminish\"><br><br>Effects are reduced by 30% for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_spirit_snatch",
  "LightMeleeReduction": 30,
  "Name": "Spirit Snatch",
  "PropertyUpgrades": {
    "SpiritDamage": 50,
    "TechArmorDamageReduction": -5,
    "TechArmorGain": 5,
    "TechPowerGain": 35,
    "TechPowerReduction": -35
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "SpiritDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.84
    },
    "Value": 50
  },
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechArmorDamageReduction": -12,
  "TechArmorGain": 12,
  "TechPowerGain": 25,
  "TechPowerReduction": -25,
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
      "lookup": "spirit snatch",
      "name": "Spirit Snatch",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_acolytes_glove" title="Spirit Strike" -->

## Spirit Strike

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_acolytes_glove`
- Snapshot ID: `39979`
- Source-Dokument: `7072`
- Kurzinfo: Spirit Strike aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spirit Strike`
- Payload Hash: `347c352726986b72a0af1ca57e18261caf1e6a310b502491aebfd7b6ce33b20c`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.589937+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 8,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 6,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": "When you perform a <span class=\"highlight\">Light or Heavy Melee</span> attack against a hero, deal extra {g:citadel_inline_attribute:'SpiritDamage'} with the attack and reduce the target's <span class=\"highlight\">Spirit Resist</span>.<span class=\"diminish\"><br><br>Cooldown is 2x longer for Light Melee hits.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_acolytes_glove",
  "LightMeleeCooldownMult": 2,
  "Name": "Spirit Strike",
  "PropertyUpgrades": {
    "SpiritDamage": 80,
    "TechArmorDamageReduction": -5
  },
  "ShopFilters": [
    "MagicDamage",
    "Melee"
  ],
  "Slot": "Tech",
  "SpiritDamage": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.372
    },
    "Value": 40
  },
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechArmorDamageReduction": -6,
  "Tier": 1,
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
      "lookup": "spirit strike",
      "name": "Spirit Strike",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_tech_overflow" title="Spiritual Overflow" -->

## Spiritual Overflow

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_overflow`
- Snapshot ID: `40202`
- Source-Dokument: `7072`
- Kurzinfo: Spiritual Overflow aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Spiritual Overflow`
- Payload Hash: `de48819b7a059c79ea4af614ba1ad814d34298f9c8119b45a181255447b3066d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.157678+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 15,
  "AbilityLifestealPercentHero": 13,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityDurationPercent": 15,
  "BonusFireRate": 32,
  "BonusHealth": 90,
  "BonusSpirit": 40,
  "BuildUpDuration": 5,
  "BuildUpPerShot": 0.75,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_health_stealing_magic"
  ],
  "Cost": 6400,
  "Description": "Gain bonus <span class=\"highlight\">Fire Rate</span>, <span class=\"highlight\">Spirit Power</span> and <span class=\"highlight\">Spirit Lifesteal</span> by <span class=\"highlight\">charging up</span> when shooting enemy heroes.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_overflow",
  "Name": "Spiritual Overflow",
  "NonHeroAbilityLifestealTooltipOnly": 3,
  "PropertyUpgrades": {
    "AbilityLifestealPercentHero": 15,
    "BonusAbilityDurationPercent": 15,
    "BonusFireRate": 20,
    "BonusHealth": 80,
    "BonusSpirit": 30,
    "TechPower": 9
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy"
  ],
  "TechPower": 6,
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
      "lookup": "spiritual overflow",
      "name": "Spiritual Overflow",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_split_shot" title="Split Shot" -->

## Split Shot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_split_shot`
- Snapshot ID: `40187`
- Source-Dokument: `7072`
- Kurzinfo: Split Shot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Split Shot`
- Payload Hash: `7abcd056a1bc7c7707f9a98c6f24d0d4c1b3d0fb86f029e372a97b48e0c1ae22`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.118208+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 27,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusShotsDuration": 5,
  "BulletSplitShot": 5,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "Make your weapon fire <span class=\"highlight\">multishot</span>. <br><br> Hitting more than one Hero per attack will grant a <span class=\"highlight\">stacking weapon damage bonus</span>. <br><br><span class=\"diminish\">Targets can only be hit once per multishot.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_split_shot",
  "MaxStacks": 5,
  "Name": "Split Shot",
  "PropertyUpgrades": {
    "AbilityCooldown": -8,
    "BulletSplitShot": 4,
    "WeaponDamagePerStack": 8
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "SpreadAngleDegrees": 45,
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
  "WeaponDamageBonusDuration": 12,
  "WeaponDamagePerStack": 8,
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
      "lookup": "split shot",
      "name": "Split Shot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_sprint_booster" title="Sprint Boots" -->

## Sprint Boots

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_sprint_booster`
- Snapshot ID: `40188`
- Source-Dokument: `7072`
- Kurzinfo: Sprint Boots aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Sprint Boots`
- Payload Hash: `c8657849024a18256b3286f57ce29812f5ce91c9b405270827996f4b12014736`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.120975+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusSprintSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 800,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_sprint_booster",
  "Name": "Sprint Boots",
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "BonusSprintSpeed": "12m",
    "OutOfCombatHealthRegen": 8
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 1,
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
      "lookup": "sprint boots",
      "name": "Sprint Boots",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_weapon_backstabber" title="Stalker" -->

## Stalker

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_backstabber`
- Snapshot ID: `40219`
- Source-Dokument: `7072`
- Kurzinfo: Stalker aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stalker`
- Payload Hash: `a5eaf0029f086141601716ecdc2f2ede4ec5b7122c4d0e3adee233030eb847c1`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.204699+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 6,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "BonusMoveSpeed": "1.5m",
  "BulletResistReduction": -6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "DPS": 17,
  "DebuffDuration": 5,
  "DebuffRadius": "25m",
  "Description": "Dealing {g:citadel_inline_attribute:'WeaponDamage'} at close range opens a wound and grants you {g:citadel_inline_attribute:'BonusMoveSpeed'}. <br><br>Wounded enemies take {g:citadel_inline_attribute:'SpiritDPS'}, have reduced {g:citadel_inline_attribute:'BulletResist'}, and are revealed <span class=\"highlight\">through walls</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_backstabber",
  "Name": "Stalker",
  "ProcRadius": "8m",
  "PropertyUpgrades": {
    "BonusMoveSpeed": "2m",
    "BulletResistReduction": -10,
    "DPS": 20,
    "ReduceFootstepSound": -50
  },
  "ReduceFootstepSound": -50,
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TickRate": 0.5,
  "Tier": 2,
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
      "lookup": "stalker",
      "name": "Stalker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_superior_stamina" title="Stamina Mastery" -->

## Stamina Mastery

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_superior_stamina`
- Snapshot ID: `40193`
- Source-Dokument: `7072`
- Kurzinfo: Stamina Mastery aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Stamina Mastery`
- Payload Hash: `8299a635be2d832ee133827db92097f8d94e10d88d1bf5a91e7e1d8ed8fb0d9a`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.134606+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "AirMoveIncreasePercent": 23,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_improved_stamina"
  ],
  "Cost": 3200,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_superior_stamina",
  "Name": "Stamina Mastery",
  "PropertyUpgrades": {
    "AirMoveIncreasePercent": 40,
    "Stamina": 2,
    "StaminaCooldownReduction": 15
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement",
    "FireRate"
  ],
  "Slot": "Armor",
  "Stamina": 2,
  "StaminaCooldownReduction": 18,
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "stamina mastery",
      "name": "Stamina Mastery",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_cooldown_reduction" title="Superior Cooldown" -->

## Superior Cooldown

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_cooldown_reduction`
- Snapshot ID: `40038`
- Source-Dokument: `7072`
- Kurzinfo: Superior Cooldown aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Superior Cooldown`
- Payload Hash: `a4cc0056aba64f33fcfcd46e4ac8b338ea762e4448f1271aaef7875e1083d54d`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.723637+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_tempo"
  ],
  "CooldownReduction": 20,
  "Cost": 3200,
  "Description": "Reduces the <span class=\"highlight\">Cooldown</span> of your abilities.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_cooldown_reduction",
  "Name": "Superior Cooldown",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "CooldownReduction": 10,
    "OutOfCombatHealthRegen": 6
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "superior cooldown",
      "name": "Superior Cooldown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_imbued_duration_extender" title="Superior Duration" -->

## Superior Duration

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_imbued_duration_extender`
- Snapshot ID: `40099`
- Source-Dokument: `7072`
- Kurzinfo: Superior Duration aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Superior Duration`
- Payload Hash: `702ccde25ba4d7299513334f4ab738d9149bbada4eeeac3055ce793785dd2b2f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.868398+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusAbilityDurationPercent": 28,
  "BulletResist": 8,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_arcane_extension"
  ],
  "Cost": 3200,
  "Description": "Increases the <span class=\"highlight\">duration</span> of your abilities and items.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_imbued_duration_extender",
  "Name": "Superior Duration",
  "PropertyUpgrades": {
    "BonusAbilityDurationPercent": 12,
    "BulletResist": 8
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "superior duration",
      "name": "Superior Duration",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_suppressor" title="Suppressor" -->

## Suppressor

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_suppressor`
- Snapshot ID: `40194`
- Source-Dokument: `7072`
- Kurzinfo: Suppressor aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Suppressor`
- Payload Hash: `984d82f9f1426f06cf4b7d59a9935f637053092b4fe148e6bb0e4be02da68928`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.137276+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BulletResist": 8,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "Description": "When you deal {g:citadel_inline_attribute:'SpiritDamage'} to enemies, you also reduce their <span class=\"highlight\">Fire Rate</span>.",
  "FireRateSlow": 28,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_suppressor",
  "Name": "Suppressor",
  "PropertyUpgrades": {
    "BulletResist": 16,
    "FireRateSlow": 20,
    "TechPower": 12
  },
  "ShopFilters": [
    "Healing",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 6,
  "Tier": 2,
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
      "lookup": "suppressor",
      "name": "Suppressor",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_storm" title="Surge of Power" -->

## Surge of Power

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_storm`
- Snapshot ID: `40121`
- Source-Dokument: `7072`
- Kurzinfo: Surge of Power aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Surge of Power`
- Payload Hash: `9546d5665167d1195fea568ad94de11a2e9cc96c843e052529b3b8d8e497994b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.927112+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusMoveSpeed": "1.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_improved_spirit"
  ],
  "Cost": 3200,
  "Description": "Imbue an ability with <span class=\"highlight\">permanent Spirit Power</span>. When that ability is used, gain bonus <span class=\"highlight\">Move Speed</span> and maintain full speed while attacking.",
  "FireRateBonus": 20,
  "ImbuedTechPower": 28,
  "IsDisabled": false,
  "IsImbue": true,
  "Key": "upgrade_magic_storm",
  "MoveWhileShootingSpeedPenaltyReductionPercent": 100,
  "MoveWhileZoomedSpeedPenaltyReductionPercent": 100,
  "MovementSpeedBonusDuration": 8,
  "Name": "Surge of Power",
  "PropertyUpgrades": {
    "BonusMoveSpeed": "2m",
    "FireRateBonus": 18,
    "ImbuedTechPower": 32
  },
  "ShopFilters": [
    "MagicDamage",
    "Movement"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "surge of power",
      "name": "Surge of Power",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_blitz_bullets" title="Swift Striker" -->

## Swift Striker

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_blitz_bullets`
- Snapshot ID: `40004`
- Source-Dokument: `7072`
- Kurzinfo: Swift Striker aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Swift Striker`
- Payload Hash: `d15f3d3123ca5368c36e2479febb75b242e5ee58e30441c119fe76c8a8405469`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.645388+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusFireRate": 20,
  "BonusSprintSpeed": "0.75m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_rapid_rounds"
  ],
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_blitz_bullets",
  "Name": "Swift Striker",
  "PropertyUpgrades": {
    "BonusFireRate": 15,
    "BonusSprintSpeed": "4m"
  },
  "ShopFilters": [
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "swift striker",
      "name": "Swift Striker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_magic_shock" title="Tankbuster" -->

## Tankbuster

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_magic_shock`
- Snapshot ID: `40119`
- Source-Dokument: `7072`
- Kurzinfo: Tankbuster aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tankbuster`
- Payload Hash: `65fda72a00d971e3779f7b91f1f0e60c0c84ec5764bf3a4ffbb6ba88cd0a77e9`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.921647+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityChargeUpTime": 14,
  "AbilityCooldown": 14,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 50,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_magic_burst"
  ],
  "Cost": 3200,
  "CurrentHealthDamage": 8,
  "Damage": 40,
  "Description": "Charges up over time with {g:citadel_inline_attribute:'BonusSpiritDamage'}, causing abilities dealing more than <span class=\"highlight\">165</span> damage to deal additional damage. <span class=\"highlight\">Ignores Spirit Resistance.</span>",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_magic_shock",
  "MinimumDamage": 165,
  "Name": "Tankbuster",
  "PropertyUpgrades": {
    "BonusHealth": 100,
    "CurrentHealthDamage": 5,
    "Damage": 60
  },
  "ReProcLockoutTime": 5,
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "Tier": 3,
  "WatcherMaxDuration": 30,
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
      "lookup": "tankbuster",
      "name": "Tankbuster",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_chain_lightning" title="Tesla Bullets" -->

## Tesla Bullets

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_chain_lightning`
- Snapshot ID: `40020`
- Source-Dokument: `7072`
- Kurzinfo: Tesla Bullets aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Tesla Bullets`
- Payload Hash: `429b27b1d2260e52a1fd89f7ef84169cb774caf8d5e080f6b32264fae41a8900`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.680865+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusPerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.19
    },
    "Value": 33
  },
  "ChainCount": 4,
  "ChainRadius": "8m",
  "ChainTickRate": 0.4,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DamagePerChain": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.19
    },
    "Value": 33
  },
  "Description": "Your bullets have a chance to <span class=\"highlight\">shock</span> your target. The <span class=\"highlight\">shock</span> will jump to a nearby enemy.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_chain_lightning",
  "Name": "Tesla Bullets",
  "ProcChance": 15,
  "ProcCooldown": 0.2,
  "PropertyUpgrades": {
    "BonusPerChain": 25,
    "DamagePerChain": 25
  },
  "ShopFilters": [
    "MagicDamage",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "tesla bullets",
      "name": "Tesla Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_titan_round" title="Titanic Magazine" -->

## Titanic Magazine

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_titan_round`
- Snapshot ID: `40207`
- Source-Dokument: `7072`
- Kurzinfo: Titanic Magazine aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Titanic Magazine`
- Payload Hash: `da597b3eebe115b083d05c13fc2f90551378175eded2966334ea3831385b9491`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.170713+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 14,
  "BonusClipSizePercent": 100,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_clip_size"
  ],
  "Cost": 1600,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_titan_round",
  "Name": "Titanic Magazine",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 18,
    "BonusClipSizePercent": 70
  },
  "ShopFilters": [
    "ClipSize",
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "titanic magazine",
      "name": "Titanic Magazine",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_tech_damage_pulse" title="Torment Pulse" -->

## Torment Pulse

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_tech_damage_pulse`
- Snapshot ID: `40200`
- Source-Dokument: `7072`
- Kurzinfo: Torment Pulse aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Torment Pulse`
- Payload Hash: `88b7a5dcfd600d5b0248d67cf917513ecdf76043796bbf82dc1d2a6f4eb3df38`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.152957+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 1.4,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 100,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "DamagePulseAmount": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.23
    },
    "Value": 25
  },
  "DamagePulseRadius": "9m",
  "Description": "Periodically deals {g:citadel_inline_attribute:'SpiritDamage'} to the closest two enemies nearby.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_tech_damage_pulse",
  "MeleeResistPercent": 18,
  "Name": "Torment Pulse",
  "PropertyUpgrades": {
    "BonusHealth": 75,
    "DamagePulseAmount": 30
  },
  "ShopFilters": [
    "MagicDamage",
    "Durability"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
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
      "lookup": "torment pulse",
      "name": "Torment Pulse",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_toxic_bullets" title="Toxic Bullets" -->

## Toxic Bullets

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_toxic_bullets`
- Snapshot ID: `40209`
- Source-Dokument: `7072`
- Kurzinfo: Toxic Bullets aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Toxic Bullets`
- Payload Hash: `468457b61b170a5ecb993231398450290f6ad22b8bfe7c9a0bf8bd24a416de06`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.175593+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BuildUpDuration": 5,
  "BuildUpPerShot": 1.28,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Bleed</span> on enemies, causing them to lose a <span class=\"highlight\">percentage</span> of their <span class=\"highlight\">Max Health</span> over time. Also applies <span class=\"highlight\">Healing Reduction</span> on the bleeding target.",
  "DotDuration": 4,
  "DotHealthPercent": {
    "Scale": {
      "Type": "spirit",
      "Value": 0.005
    },
    "Value": 1.9
  },
  "DotMultiplerTroopers": 0.5,
  "HealAmpReceivePenaltyPercent": -35,
  "HealAmpRegenPenaltyPercent": -35,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_toxic_bullets",
  "Name": "Toxic Bullets",
  "PropertyUpgrades": {
    "DotHealthPercent": 0.7,
    "HealAmpReceivePenaltyPercent": -30,
    "HealAmpRegenPenaltyPercent": -30
  },
  "ShopFilters": [
    "MagicDamage",
    "Disruption"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroEnemy",
    "CreepEnemy",
    "MinionEnemy"
  ],
  "TickRate": 0.5,
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
      "lookup": "toxic bullets",
      "name": "Toxic Bullets",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_transcendent_cooldown" title="Transcendent Cooldown" -->

## Transcendent Cooldown

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_transcendent_cooldown`
- Snapshot ID: `40210`
- Source-Dokument: `7072`
- Kurzinfo: Transcendent Cooldown aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Transcendent Cooldown`
- Payload Hash: `879da1875ee4cffa50768829941dedc5f09ea2a21aacdcc476fc2a5eb21f0e9b`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.178387+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_cooldown_reduction"
  ],
  "CooldownReduction": 25,
  "Cost": 6400,
  "Description": "Reduces the <span class=\"highlight\">Cooldown</span> of your abilities and items.",
  "IsDisabled": false,
  "IsImbue": false,
  "ItemCooldownReduction": 25,
  "Key": "upgrade_transcendent_cooldown",
  "Name": "Transcendent Cooldown",
  "OutOfCombatHealthRegen": 4,
  "PropertyUpgrades": {
    "CooldownReduction": 15,
    "ItemCooldownReduction": 10,
    "OutOfCombatHealthRegen": 10
  },
  "ShopFilters": [
    "Healing"
  ],
  "Slot": "Tech",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "transcendent cooldown",
      "name": "Transcendent Cooldown",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_trophy_collector" title="Trophy Collector" -->

## Trophy Collector

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_trophy_collector`
- Snapshot ID: `40211`
- Source-Dokument: `7072`
- Kurzinfo: Trophy Collector aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Trophy Collector`
- Payload Hash: `9b412b522ae6c61493dbab47d2457114d5cbafcd35ff1e4e2b16a1d5045f16e3`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.181126+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusSprintSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 1600,
  "Description": "Whenever you score an <span class=\"highlight\">assist or kill</span>, gain extra <span class=\"highlight\">sprint</span>, <span class=\"highlight\">ability range</span> and <span class=\"highlight\">passive soul generation</span>. This effect stacks and persists through death.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_trophy_collector",
  "MaxStacks": 16,
  "Name": "Trophy Collector",
  "NonPlayerBonusWeaponPower": -15,
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "BonusSprintSpeed": "12m",
    "MaxStacks": 83,
    "OutOfCombatHealthRegen": 6,
    "StackingTechRadiusMultiplier": 3,
    "StackingTechRangeMultiplier": 3
  },
  "ShopFilters": [
    "WeaponDamage",
    "Healing",
    "Movement"
  ],
  "Slot": "Armor",
  "StackingBonusSprintSpeed": "0.15m",
  "StackingGoldPerMinute": 18,
  "StackingTechRadiusMultiplier": 0.75,
  "StackingTechRangeMultiplier": 0.75,
  "StreetBrawl": false,
  "TargetTypes": null,
  "ThinkRate": 3,
  "Tier": 2,
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
      "lookup": "trophy collector",
      "name": "Trophy Collector",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_unstable_concoction" title="Unstable Concoction" -->

## Unstable Concoction

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstable_concoction`
- Snapshot ID: `40213`
- Source-Dokument: `7072`
- Kurzinfo: Unstable Concoction aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unstable Concoction`
- Payload Hash: `65baf8dce1c0b8ed76a1b7046210684c01bf680ce08fc121b24d73ebace9fad1`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.186393+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 25.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BaseAttackDamagePercent": 150,
  "BonusHealth": 3000,
  "BonusMoveSpeed": "10m",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 9999,
  "Description": "Consume a concoction that grants you <span class=\"highlight\">Unstoppable</span> and increased <span class=\"highlight\">speed, health, spirit and weapon damage</span>. After a short duration <span class=\"highlight\">you die and explode</span>, stunning nearby enemies and dealing damage based on your maximum health. Dying this way reduces your respawn time by <span class=\"highlight\">50%</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstable_concoction",
  "MaxHPDamage": 30,
  "Name": "Unstable Concoction",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 50,
    "BonusHealth": 1300,
    "StunDuration": 0.5,
    "TechPower": 50
  },
  "Radius": "22m",
  "RespawnTimeMod": 50,
  "ShopFilters": null,
  "Slot": "Tech",
  "StreetBrawl": true,
  "StunDuration": 3.0,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechPower": 150,
  "Tier": 5,
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
      "lookup": "unstable concoction",
      "name": "Unstable Concoction",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_unstoppable" title="Unstoppable" -->

## Unstoppable

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_unstoppable`
- Snapshot ID: `40214`
- Source-Dokument: `7072`
- Kurzinfo: Unstoppable aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Unstoppable`
- Payload Hash: `3e9c60e6de4c1689395685a8188b573ee299743cee164a44441a41ea52947dd8`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.190130+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 60.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5.5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BonusHealth": 125,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_debuff_reducer"
  ],
  "Cost": 6400,
  "Description": "Temporarily suppress <span class=\"highlight\">negative status effects</span> and become <span class=\"highlight\">immune</span> to <span class=\"highlight\">Stun, Silence, Sleep, Root, and Disarm</span>. <br>Cannot be used while <span class=\"highlight\">Stunned</span> or <span class=\"highlight\">Slept</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_unstoppable",
  "Name": "Unstoppable",
  "PropertyUpgrades": {
    "AbilityCooldown": -35,
    "AbilityDuration": 1.25,
    "BonusHealth": 75,
    "StatusResistancePercent": 15
  },
  "ShopFilters": [
    "Durability",
    "Movement"
  ],
  "Slot": "Armor",
  "StatusResistancePercent": 25,
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "unstoppable",
      "name": "Unstoppable",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_surging_power" title="Vampiric Burst" -->

## Vampiric Burst

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_surging_power`
- Snapshot ID: `40195`
- Source-Dokument: `7072`
- Kurzinfo: Vampiric Burst aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vampiric Burst`
- Payload Hash: `e0958fd67cf163f7dc76b15ef312851c062da15368e330a11c18189918b0ace8`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.139782+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 30.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 5,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "ActiveBonusFireRate": 34,
  "ActiveBonusLifesteal": 70,
  "ActiveReloadPercent": 75,
  "BaseAttackDamagePercent": 6,
  "BonusHealth": 100,
  "BulletLifestealPercent": 13,
  "BulletResist": 10,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_vampire"
  ],
  "Cost": 6400,
  "Description": null,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_surging_power",
  "Name": "Vampiric Burst",
  "PropertyUpgrades": {
    "AbilityCooldown": -10,
    "ActiveBonusFireRate": 25,
    "BonusHealth": 110,
    "BulletLifestealPercent": 16,
    "BulletResist": 10
  },
  "ShopFilters": [
    "FireRate",
    "WeaponDamage",
    "Movement",
    "Healing"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": [
    "HeroFriendly"
  ],
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
      "lookup": "vampiric burst",
      "name": "Vampiric Burst",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_veil_walker" title="Veil Walker" -->

## Veil Walker

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_veil_walker`
- Snapshot ID: `40216`
- Source-Dokument: `7072`
- Kurzinfo: Veil Walker aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Veil Walker`
- Payload Hash: `9188bbf33a0b11701cc5541971421d442658d2f8202aea695ba573461efb83c9`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.196162+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 15.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 16,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 125,
  "BonusMoveSpeed": "3.5m",
  "BonusSprintSpeed": "2.0m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_sprint_booster"
  ],
  "Cost": 3200,
  "Description": "Walking through a <span class=\"highlight\">cosmic veil</span> grants you <span class=\"highlight\">Stealth</span>, <span class=\"highlight\">Heal</span> and increased <span class=\"highlight\">Move Speed</span>.",
  "HealOnVeil": {
    "Scale": {
      "Type": "power_increase",
      "Value": 8
    },
    "Value": 85
  },
  "InvisAlertWhenFading": 1,
  "InvisDuration": 8,
  "InvisFadeToDuration": 0.25,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_veil_walker",
  "Name": "Veil Walker",
  "OutOfCombatHealthRegen": 2,
  "PropertyUpgrades": {
    "AbilityCooldown": -9,
    "BonusMoveSpeed": "4m",
    "BonusSprintSpeed": "12m",
    "HealOnVeil": 300,
    "InvisDuration": 4,
    "InvisMoveSpeedMod": "6m",
    "OutOfCombatHealthRegen": 8,
    "SpiritPower": 25
  },
  "RevealOnDamageDuration": 0.5,
  "RevealOnSpottedDuration": 1.25,
  "ShopFilters": [
    "Durability",
    "ClipSize"
  ],
  "Slot": "Armor",
  "SpiritPower": 10,
  "SpottedRadius": "20m",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "veil walker",
      "name": "Veil Walker",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_aoe_root" title="Vortex Web" -->

## Vortex Web

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_aoe_root`
- Snapshot ID: `39986`
- Source-Dokument: `7072`
- Kurzinfo: Vortex Web aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Vortex Web`
- Payload Hash: `649d0bc665ab137aafa4d323d7d9b1c8b6ecaadd9e72f51f60acfeba7dc06e3f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.607954+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastDelay": 0.2,
  "AbilityCastRange": "30m",
  "AbilityCooldown": 42.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityDuration": 4.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Press",
  "BonusSprintSpeed": "0.75m",
  "CaptureRadius": "12m",
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_containment"
  ],
  "Cost": 6400,
  "Description": null,
  "GroundDashReductionPercent": -40,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_aoe_root",
  "Name": "Vortex Web",
  "PropertyUpgrades": {
    "AbilityCooldown": -22,
    "BonusSprintSpeed": "9m",
    "SlowPercent": 15,
    "TechRadiusMultiplier": 10,
    "TechRangeMultiplier": 10
  },
  "ShopFilters": null,
  "Slot": "Tech",
  "SlowPercent": 35,
  "StreetBrawl": false,
  "TargetTypes": [
    "AllEnemy"
  ],
  "TechRadiusMultiplier": 8,
  "TechRangeMultiplier": 8,
  "TetherDuration": 0.5,
  "TetherRadius": "1m",
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
      "lookup": "vortex web",
      "name": "Vortex Web",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_warp_stone" title="Warp Stone" -->

## Warp Stone

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_warp_stone`
- Snapshot ID: `40218`
- Source-Dokument: `7072`
- Kurzinfo: Warp Stone aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Warp Stone`
- Payload Hash: `0c8dce1aa7229a891e98f6471510ac440f3f477e2b94c310b125998926f1b06f`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.201721+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCastRange": "11m",
  "AbilityCooldown": 16.0,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "InstantCast",
  "BulletResist": 30,
  "CasterBuffDuration": 6,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 3200,
  "Description": "<span class=\"highlight\">Teleport</span> straight ahead, gaining <span class=\"highlight\">Bullet Resist</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_warp_stone",
  "Name": "Warp Stone",
  "PropertyUpgrades": {
    "AbilityCastRange": "9m",
    "AbilityCooldown": -3,
    "BulletResist": 20
  },
  "ShopFilters": [
    "WeaponDamage",
    "Movement"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
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
      "lookup": "warp stone",
      "name": "Warp Stone",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_headshot_booster2" title="Weakening Headshot" -->

## Weakening Headshot

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_headshot_booster2`
- Snapshot ID: `40082`
- Source-Dokument: `7072`
- Kurzinfo: Weakening Headshot aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Weakening Headshot`
- Payload Hash: `8cc5f9b6e88d569e976920ba33802dc54398a062b161c93232184173c30f7db9`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.829392+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BonusHealth": 60,
  "BulletResistReduction": -13,
  "ChannelMoveSpeed": -1,
  "Components": null,
  "Cost": 1600,
  "DebuffDuration": 12,
  "Description": "Landing a <span class=\"highlight\">Headshot</span> reduces their <span class=\"highlight\">Bullet Resist</span>.",
  "DiminishingMultiplier": 0.5,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_headshot_booster2",
  "Name": "Weakening Headshot",
  "PropertyUpgrades": {
    "BonusHealth": 125,
    "BulletResistReduction": -7
  },
  "ShopFilters": [
    "Durability",
    "FireRate"
  ],
  "Slot": "Weapon",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "weakening headshot",
      "name": "Weakening Headshot",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_weapon_shielding" title="Weapon Shielding" -->

## Weapon Shielding

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_weapon_shielding`
- Snapshot ID: `40225`
- Source-Dokument: `7072`
- Kurzinfo: Weapon Shielding aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Weapon Shielding`
- Payload Hash: `17843652070c40db54f318228c836af4aa77f37b0ed03a4d09f39928820bc6ec`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.218061+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 35,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BarrierDuration": 8,
  "BulletResist": 18,
  "ChannelMoveSpeed": -1,
  "CombatBarrier": {
    "Scale": {
      "Type": "power_increase",
      "Value": 5
    },
    "Value": 300
  },
  "Components": [
    "upgrade_grit"
  ],
  "Cost": 1600,
  "DamageThreshold": 250,
  "DamageWindow": 4.0,
  "Description": "Gain a <span class=\"highlight\">Barrier</span> whenever you take significant {g:citadel_inline_attribute:'WeaponDamage'} from enemy Heroes in a small time frame.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weapon_shielding",
  "Name": "Weapon Shielding",
  "OutOfCombatHealthRegen": 2.5,
  "PropertyUpgrades": {
    "AbilityCooldown": -20,
    "BulletResist": 15,
    "CombatBarrier": 225,
    "OutOfCombatHealthRegen": 3
  },
  "ShopFilters": [
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "Tier": 2,
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
      "lookup": "weapon shielding",
      "name": "Weapon Shielding",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_weighted_shots" title="Weighted Shots" -->

## Weighted Shots

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_weighted_shots`
- Snapshot ID: `40226`
- Source-Dokument: `7072`
- Kurzinfo: Weighted Shots aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Weighted Shots`
- Payload Hash: `4d8a6f016fbd4b310610eb4e32c17f084d83199fd5eae2ca9cda85aa56572d03`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:25.220745+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "BaseAttackDamagePercent": 40,
  "BonusMoveSpeed": "-0.5m",
  "BuildUpDuration": 5,
  "BuildUpPerShot": 0.7,
  "ChannelMoveSpeed": -1,
  "Components": [
    "upgrade_slowing_bullets"
  ],
  "Cost": 3200,
  "Description": "Your bullets build up a <span class=\"highlight\">Movement Slow</span> on enemies.",
  "GroundDashReductionPercent": -22,
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_weighted_shots",
  "Name": "Weighted Shots",
  "PropertyUpgrades": {
    "BaseAttackDamagePercent": 35,
    "GroundDashReductionPercent": -10,
    "SlowPercent": 20,
    "StatusResistancePercent": 10
  },
  "ShopFilters": [
    "WeaponDamage"
  ],
  "Slot": "Weapon",
  "SlowDuration": 3.5,
  "SlowPercent": 30,
  "StaminaCooldownReduction": -14,
  "StatusResistancePercent": 22,
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
      "lookup": "weighted shots",
      "name": "Weighted Shots",
      "type": "item"
    }
  ]
}
````

<!-- game-wiki-entry source="deadlock_data" entity_type="item" external_id="upgrade_absorbing_armor" title="Witchmail" -->

## Witchmail

### Kurzueberblick

- Typ: `item`
- Quelle: `deadlock_data`
- External ID: `upgrade_absorbing_armor`
- Snapshot ID: `39978`
- Source-Dokument: `7072`
- Kurzinfo: Witchmail aus `deadlock_data` / `item` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `Witchmail`
- Payload Hash: `8038e19ae42cb15210cece64d0386ea27023f553ce601d30d59698985e2635ec`
- Source Content Hash: `b01d3bc127fcd39965acccbb97da1891f783e723ccea23aa74a15b0187b14b65`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/item-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_item-data.json.b01d3bc127fcd399.json`
- Fetched At: `2026-07-09T19:36:24.587757+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "AbilityCooldown": 1,
  "AbilityCooldownBetweenCharge": -1.0,
  "AbilityUnitTargetLimit": 1,
  "Activation": "Passive",
  "ChannelMoveSpeed": -1,
  "Components": null,
  "CooldownReduction": 7,
  "CooldownReductionPerHit": 4,
  "Cost": 6400,
  "DamageThreshold": 75,
  "Description": "Taking heavy hits of {g:citadel_inline_attribute:'SpiritDamage'} from an enemy reduces a <span class=\"highlight\">random ability cooldown</span>.",
  "IsDisabled": false,
  "IsImbue": false,
  "Key": "upgrade_absorbing_armor",
  "Name": "Witchmail",
  "PropertyUpgrades": {
    "CooldownReductionPerHit": 2,
    "TechPower": 26,
    "TechResist": 5
  },
  "ShopFilters": [
    "WeaponDamage",
    "Durability"
  ],
  "Slot": "Armor",
  "StreetBrawl": false,
  "TargetTypes": null,
  "TechPower": 14,
  "TechResist": 22,
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
      "lookup": "witchmail",
      "name": "Witchmail",
      "type": "item"
    }
  ]
}
````

