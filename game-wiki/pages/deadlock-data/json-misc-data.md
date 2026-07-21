---
title: "json misc data"
generated_at: "2026-07-21T18:03:55.468852924+00:00"
entries: 1
---

# json misc data

<!-- game-wiki-entry source="deadlock_data" entity_type="json_misc_data" external_id="json_misc_data" title="json_misc_data" -->

## json_misc_data

### Kurzueberblick

- Typ: `json_misc_data`
- Quelle: `deadlock_data`
- External ID: `json_misc_data`
- Snapshot ID: `40575`
- Source-Dokument: `7079`
- Kurzinfo: json_misc_data aus `deadlock_data` / `json_misc_data` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `json_misc_data`
- Payload Hash: `9ea07fa00ff247924c9fd469ecbf9a1d38470fd82195fe8c91a40b25264d2835`
- Source Content Hash: `0ec2773dccc063eb0437620a50c85439dd47809a886dcf2de9043408dfda5b55`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/misc-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_misc-data.json.0ec2773dccc063eb.json`
- Fetched At: `2026-07-09T19:36:25.782532+00:00`
- Generated At: `2026-07-21T18:03:55.468852924+00:00`

### Vollstaendige Payload

````json
{
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/misc-data.json",
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
  "values": {
    "_include": [
      "scripts/misc/breakable_props.vdata_inc",
      "scripts/misc/xp_orbs.vdata_inc",
      "scripts/misc/neutral_camps.vdata_inc",
      "scripts/misc/pickups.vdata_inc",
      "scripts/misc/capture_points.vdata_inc"
    ],
    "ammo_permanent_pickup": {
      "Color": [
        255,
        165,
        0
      ],
      "DefaultMaterialGroupName": "orange",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT",
            "value": 3
          }
        ],
        "SmallIconCssClass": "ammoIncrease"
      },
      "NameLocString": "ammo_permanent_pickup_label",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Ammo",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "ammo_permanent_pickup_lv2": {
      "Color": [
        255,
        165,
        0
      ],
      "DefaultMaterialGroupName": "orange",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT",
            "value": 5
          }
        ],
        "SmallIconCssClass": "ammoIncrease"
      },
      "NameLocString": "ammo_permanent_pickup_label_lv2",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Ammo",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "ammo_permanent_pickup_lv3": {
      "Color": [
        255,
        165,
        0
      ],
      "DefaultMaterialGroupName": "orange",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT",
            "value": 7
          }
        ],
        "SmallIconCssClass": "ammoIncrease"
      },
      "NameLocString": "ammo_permanent_pickup_label_lv3",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Ammo",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "capture_point_escort": {
      "DecaySpeed": 0.01,
      "EnabledLoopSounds": {
        "CITADEL_AUDIO_LOOP_LOOP_SOUND": "Ability.A1.SiphonLife.Loop"
      },
      "EnabledParticle": "particles/npc/escort/escort_arrived.vpcf",
      "EnemyCapturingLoopSounds": {
        "CITADEL_AUDIO_LOOP_LOOP_SOUND": "Ability.Bebop.StickyBomb.Loop"
      },
      "FriendlyCapturingLoopSounds": {
        "CITADEL_AUDIO_LOOP_LOOP_SOUND": "Dynamo.A3.Heal.Loop"
      },
      "InitialEnableTimeInSeconds": [
        840.0,
        960.0
      ],
      "OnBecomeEnableParticle": "particles/npc/escort/escort_spawn.vpcf",
      "OnFullyCapturedParticle": "particles/npc/escort/escort_spawn.vpcf",
      "PreEnableParticle": "particles/npc/escort/escort_arriving.vpcf",
      "PreEnableWindowInSeconds": 10.0,
      "RespawnRangeInSeconds": [
        60.0,
        180.0
      ],
      "TotalHealthToCapture": 0.0,
      "modifierCapturer": {
        "HudDisplayLocation": "DISPLAY_HUD_CENTER",
        "ParticleEffect": "particles/npc/escort/escort_summon_debuff.vpcf"
      },
      "remapCapturersToCaptureTime": [
        1.0,
        6.0,
        12.0,
        2.0
      ]
    },
    "casting_powerup_pickup": {
      "AmbientSound": "Powerup.Casting_Lp",
      "AuraModifier": {
        "AuraRadius": 300.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_HEAVY_PUNCHABLE_ITEM",
          "IsHidden": true
        }
      },
      "CollectionMethod": "Punch",
      "CollisionRadius": 40.0,
      "Color": [
        203,
        143,
        252
      ],
      "DamagedParticle": "particles/environment/powerup_spawner_ambient_damaged.vpcf",
      "HitSound": "Powerup.Pickup.Punch",
      "HitsRequired": 1,
      "MinimapCssClasses": [
        "powerup_casting",
        "powerup_spawn"
      ],
      "Modifer": {
        "AlwaysShowInStatModifierUI": [
          "MODIFIER_VALUE_TECH_POWER",
          "MODIFIER_VALUE_COOLDOWN_REDUCTION_PERCENTAGE"
        ],
        "BuffParticle": "particles/environment/powerup_spawner_buff.vpcf",
        "Color": [
          185,
          103,
          253
        ],
        "DrawOverheadStatus": "OVERHEAD_DRAW_FOR_EVERYONE",
        "Duration": 160.0,
        "HudDisplayLocation": "DISPLAY_HUD_NONE",
        "HudIcon": "file://{images}/hud/modifiers/icon_powerup.svg",
        "LocalizationName": "casting_powerup_pickup",
        "ModifierDisplayLocaiton": "MODIFIER_DISPLAY_HEALTHBAR",
        "ModifierValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_TECH_POWER",
            "valueMax": 65.0,
            "valueMin": 15.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_COOLDOWN_REDUCTION_PERCENTAGE",
            "valueMax": 20.0,
            "valueMin": 12.0
          }
        ],
        "SmallIconCssClass": "casting_pickup",
        "TimeMax": 40,
        "TimeMin": 5
      },
      "NameLocString": "casting_powerup_pickup",
      "NameOffset": 150,
      "ParryCheckModifier": {
        "ParryCheckRadius": 60.0
      },
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Generic",
      "ShowOnMinimap": true,
      "SpawnSound": "Powerup.BridgeBuff.Spawn",
      "TempParticleSheetIndex": 7,
      "enemyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "friendlyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "gainedParticle": "particles/generic/bridge_buff.vpcf",
      "hModel": "models/null.vmdl"
    },
    "cd_permanent_pickup": {
      "Color": [
        0,
        0,
        200
      ],
      "DefaultMaterialGroupName": "blue",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_COOLDOWN_REDUCTION_PERCENTAGE",
            "value": 0.5
          }
        ],
        "SmallIconCssClass": "cooldownReduction"
      },
      "NameLocString": "cd_permanent_pickup_label",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Spirit",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "cd_permanent_pickup_lv2": {
      "Color": [
        0,
        0,
        200
      ],
      "DefaultMaterialGroupName": "blue",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_COOLDOWN_REDUCTION_PERCENTAGE",
            "value": 0.75
          }
        ],
        "SmallIconCssClass": "cooldownReduction"
      },
      "NameLocString": "cd_permanent_pickup_label_lv2",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Spirit",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "cd_permanent_pickup_lv3": {
      "Color": [
        0,
        0,
        200
      ],
      "DefaultMaterialGroupName": "blue",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_COOLDOWN_REDUCTION_PERCENTAGE",
            "value": 1.0
          }
        ],
        "SmallIconCssClass": "cooldownReduction"
      },
      "NameLocString": "cd_permanent_pickup_label_lv3",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Spirit",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "change_team_powerup_pickup": {
      "AmbientSound": "Powerup.Survival_Lp",
      "AuraModifier": {
        "AuraRadius": 150.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_PUNCHABLE_CHANGE_TEAM",
          "IsHidden": true
        }
      },
      "CollectionMethod": "Punch",
      "CollisionRadius": 40.0,
      "Color": [
        195,
        82,
        26
      ],
      "DamagedParticle": "particles/environment/powerup_spawner_ambient_damaged.vpcf",
      "HitSound": "Powerup.Pickup.Punch",
      "HitsRequired": 1,
      "Modifer": {
        "HudDisplayLocation": "DISPLAY_HUD_NONE"
      },
      "NameLocString": "change_team_powerup_pickup",
      "NameOffset": 120,
      "ParryCheckModifier": {
        "ParryCheckRadius": 60.0
      },
      "ParticleRadius": 1,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Generic",
      "ShowOnMinimap": true,
      "SpawnSound": "Powerup.BridgeBuff.Spawn",
      "TempParticleSheetIndex": 11,
      "enemyParticle": "particles/environment/powerup_spawner_ambient_simple.vpcf",
      "friendlyParticle": "particles/environment/powerup_spawner_ambient_simple.vpcf",
      "gainedParticle": "particles/generic/bridge_buff.vpcf",
      "hModel": "models/null.vmdl"
    },
    "citadel_base_glass_horiz_01": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": false,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": true,
      "RespawnTime": 60.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": true,
      "hModel": "models/props_structures/caldera_base_glass_horiz_01_breakable.vmdl"
    },
    "citadel_base_glass_vert_01": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": false,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": true,
      "RespawnTime": 60.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": true,
      "hModel": "models/props_structures/caldera_base_glass_vert_01_breakable.vmdl"
    },
    "citadel_breakable_item_container": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "glass.break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "InitialSpawnTimeTest": 1.0,
      "LootListDeckSize": 5,
      "MatchTimeMinsForLevel2Pickups": 10,
      "MatchTimeMinsForLevel3Pickups": 30,
      "Pickups_lv2": [
        {
          "Pickup": "spirit_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "firerate_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "ammo_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "hp_permanent_pickup_lv2",
          "PickupWeight": 2.0
        },
        {
          "Pickup": "cd_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "wp_permanent_pickup_lv2",
          "PickupWeight": 1.0
        }
      ],
      "Pickups_lv3": [
        {
          "Pickup": "spirit_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "firerate_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "ammo_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "hp_permanent_pickup_lv3",
          "PickupWeight": 2.0
        },
        {
          "Pickup": "cd_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "wp_permanent_pickup_lv3",
          "PickupWeight": 1.0
        }
      ],
      "PrimaryDropChance": 50.0,
      "PrimaryPickups": [
        {
          "Pickup": "spirit_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "firerate_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "ammo_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "hp_permanent_pickup",
          "PickupWeight": 2.0
        },
        {
          "Pickup": "cd_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "wp_permanent_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RespawnTimeTest": 5.0,
      "RollType": "ECitadelRandomRoll_BreakablePowerupPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/item_container_breakable/item_container.vmdl"
    },
    "citadel_breakable_jar_01": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pottery.Break",
      "DamageSound": "Pottery.BulletImpact",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/jar_01/jar_01.vmdl"
    },
    "citadel_breakable_lion_statue": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pottery.Break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/lion_statue_02/lion_statue_02.vmdl"
    },
    "citadel_breakable_prop_box_multi": {
      "BreakOnDodgeTouch": true,
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/test/physics/break_box_multi.vmdl"
    },
    "citadel_breakable_prop_car": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/car_04_breakable.vmdl"
    },
    "citadel_breakable_prop_drop_gold": {
      "BreakOnDodgeTouch": true,
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false
    },
    "citadel_breakable_prop_drop_powerups": {
      "BreakOnDodgeTouch": true,
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "InitialSpawnTimeTest": 1.0,
      "LootListDeckSize": 5,
      "MatchTimeMinsForLevel2Pickups": 10,
      "MatchTimeMinsForLevel3Pickups": 30,
      "Pickups_lv2": [
        {
          "Pickup": "spirit_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "firerate_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "ammo_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "hp_permanent_pickup_lv2",
          "PickupWeight": 2.0
        },
        {
          "Pickup": "cd_permanent_pickup_lv2",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "wp_permanent_pickup_lv2",
          "PickupWeight": 1.0
        }
      ],
      "Pickups_lv3": [
        {
          "Pickup": "spirit_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "firerate_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "ammo_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "hp_permanent_pickup_lv3",
          "PickupWeight": 2.0
        },
        {
          "Pickup": "cd_permanent_pickup_lv3",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "wp_permanent_pickup_lv3",
          "PickupWeight": 1.0
        }
      ],
      "PrimaryDropChance": 50.0,
      "PrimaryPickups": [
        {
          "Pickup": "spirit_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "firerate_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "ammo_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "hp_permanent_pickup",
          "PickupWeight": 2.0
        },
        {
          "Pickup": "cd_permanent_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "wp_permanent_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RespawnTimeTest": 5.0,
      "RollType": "ECitadelRandomRoll_BreakablePowerupPickup",
      "SolidAfterDeath": false
    },
    "citadel_breakable_prop_pumpkin01": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pumpkin.Smash_Large",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props/pumpkin_a.vmdl"
    },
    "citadel_breakable_prop_pumpkin02": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pumpkin.Smash_Large",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props/pumpkin_b.vmdl"
    },
    "citadel_breakable_prop_pumpkin03": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pumpkin.Smash_Small",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props/pumpkin_c.vmdl"
    },
    "citadel_breakable_prop_pumpkin04": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pumpkin.Smash_Small",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props/pumpkin_d.vmdl"
    },
    "citadel_breakable_prop_vase": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Pottery.Large.Break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/vase_large/vase_large.vmdl"
    },
    "citadel_breakable_prop_wooden_crate": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Wood.Crate.Break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "SpawnSound": "Wood.Crate.Spawn",
      "hModel": "models/items/item_crate.vmdl"
    },
    "citadel_breakable_prop_xmaspresent01": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Present.Smash_Large",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/item_container_breakable/xmas_giftbox.vmdl"
    },
    "citadel_breakable_prop_xmaspresent02": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Present.Smash_Large",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/item_container_breakable/xmas_giftbox02.vmdl"
    },
    "citadel_breakable_prop_xmaspresent03": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Present.Smash_Large",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/item_container_breakable/xmas_giftbox03.vmdl"
    },
    "citadel_breakable_sake_barrel": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Wood.Crate.Break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/sake_barrel_01/sake_barrel_01.vmdl"
    },
    "citadel_breakable_wooden_crate_02": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Wood.Crate.Break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/wooden_crate_02/wooden_crate_02.vmdl"
    },
    "citadel_breakable_wooden_crate_03": {
      "BreakOnDodgeTouch": true,
      "BreakSound": "Wood.Crate.Break",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "Health": 1,
      "InitialSpawnTime": 180.0,
      "IsMantleable": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_gameplay/wooden_crate_03/wooden_crate_03.vmdl"
    },
    "citadel_change_team_spawner": {
      "ActiveParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "InactiveParticle": "particles/environment/powerup_spawner_inactive_ambient.vpcf",
      "InitialSpawnTime": -1,
      "InitialSpawnTimeTest": -1,
      "ModelScale": 0.5,
      "RespawnTime": 1,
      "RespawnTimeTest": 1,
      "RespawnTimerStartsAfterPickup": true,
      "SinglePickupOverride": "change_team_powerup_pickup",
      "hModel": ""
    },
    "citadel_herotest_orbspawner": {
      "AmbientParticle": "particles/environment/powerup_spawner_inactive_ambient.vpcf",
      "FirstSpawnTime": 180.0,
      "GoldValue": 10,
      "ModelScale": 1.5,
      "SpawnParticle": "particles/trooper/trooper_death_sn.vpcf",
      "SpawnRate": 2.0,
      "hModel": "models/npc/sewer_beast/sewer.vmdl"
    },
    "citadel_item_koth_spawner": {
      "_editor": {
        "folder_name": "Pickup Items"
      }
    },
    "citadel_item_pickup": {
      "AmbientParticle": "particles/generic/holding_gold_dropped.vpcf",
      "_editor": {
        "folder_name": "Pickup Items"
      }
    },
    "citadel_item_pickup_idol": {
      "AmbientParticle": "particles/generic/holding_gold_dropped.vpcf",
      "PickUpAura": {
        "AuraRadius": 70.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "EnabledStateMask": "MODIFIER_STATE_IS_MELEE_TARGET",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_TELEPORTER_DISABLED | MODIFIER_STATE_PICKING_UP_IDOL",
          "IsHidden": true
        }
      },
      "WalkBackModifier": {
        "AmbientLoopingSound": "",
        "BiasEffectNegative": "particles/generic/idol_pickup_indicator_enemy.vpcf",
        "BiasEffectPositive": "particles/generic/idol_pickup_indicator.vpcf",
        "IdleParticle": "particles/generic/idol_pickup_aura.vpcf",
        "IdlingLoopSound": "Soul.Urn.Idling.Lp",
        "MoveSpeed": 2.0,
        "RunningParticle": "particles/generic/idol_pickup_aura_running.vpcf",
        "StopDistance": -1.0,
        "Tolerance": 200.0,
        "VerticalOffset": 0.0,
        "WaitTimeLimit": 180.0,
        "WalkingLoopSound": "Soul.Urn.Walking.Lp"
      },
      "_editor": {
        "folder_name": "Pickup Items"
      }
    },
    "citadel_item_pickup_rejuv": {
      "AbilityProjectile": "ability_item_pickup_effects",
      "PunchPickupModifier": {
        "AoEHealParticle": "particles/upgrades/health_nova_cast.vpcf",
        "DamagedParticle": "particles/environment/rejuv_damaged.vpcf",
        "EnabledStateMask": "MODIFIER_STATE_IS_MELEE_TARGET",
        "HitSound": "Powerup.Pickup.Punch",
        "IsDroppingParticle": "particles/environment/rejuv_ambient.vpcf",
        "IsFrozenParticle": "particles/environment/rejuv_frozen_model.vpcf",
        "IsHidden": true,
        "IsPunchableParticle": "particles/environment/rejuv_attackable.vpcf",
        "NearRejuvAuraModifier": {
          "AuraRadius": 300.0,
          "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
          "IsHidden": true,
          "modifierProvidedByAura": {
            "EnabledStateMask": "MODIFIER_STATE_NEAR_REJUVINATOR",
            "IsHidden": true
          }
        },
        "ParryCheckModifier": {
          "ParryCheckRadius": 80.0
        }
      },
      "RebirthModifier": {
        "DeployParticle": "particles/modifiers/rejuv_pending.vpcf",
        "DeploySound": "Rejuv.Rebirth.Start",
        "HudDisplayLocation": "DISPLAY_HUD_NONE",
        "HudIcon": "file://{images}/upgrades/mods_armor/portable_rejuvenator.png",
        "HudMessageText": "#modifier_citadel_respawn_credit",
        "LocalizationName": "modifier_citadel_respawn_credit",
        "ModifierDisplayLocaiton": "MODIFIER_DISPLAY_HEALTHBAR",
        "ParticleEffect": "particles/modifiers/respawn_credit_buff.vpcf",
        "ParticleStatusEffect": "particles/status_fx/status_fx_respawn_credit.vpcf",
        "ParticleStatusEffectConfig": "60",
        "RespawnDelay": 3.0,
        "RespawnLifePct": 100.0,
        "RespawnParticle": "particles/upgrades/portable_rejuvinator_respawn.vpcf",
        "RespawnSound": "Rejuv.Rebirth.Revive",
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_MAX_PERCENT",
            "value": 0.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_FIRE_RATE",
            "value": 0.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_TECH_DAMAGE_MULTIPLIER",
            "value": 0.0
          }
        ],
        "SmallIconCssClass": "rejuv_credit"
      },
      "_editor": {
        "folder_name": "Pickup Items"
      }
    },
    "citadel_item_pickup_rejuv_herotest": {
      "AbilityProjectile": "ability_item_pickup_effects",
      "PunchPickupModifier": {
        "AoEHealParticle": "particles/upgrades/health_nova_cast.vpcf",
        "DamagedParticle": "particles/environment/rejuv_damaged.vpcf",
        "EnabledStateMask": "MODIFIER_STATE_IS_MELEE_TARGET",
        "HitSound": "Powerup.Pickup.Punch",
        "IsDroppingParticle": "particles/environment/rejuv_ambient.vpcf",
        "IsFrozenParticle": "particles/environment/rejuv_frozen_model.vpcf",
        "IsHidden": true,
        "IsPunchableParticle": "particles/environment/rejuv_attackable.vpcf",
        "NearRejuvAuraModifier": {
          "AuraRadius": 300.0,
          "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
          "IsHidden": true,
          "modifierProvidedByAura": {
            "EnabledStateMask": "MODIFIER_STATE_NEAR_REJUVINATOR",
            "IsHidden": true
          }
        },
        "ParryCheckModifier": {
          "ParryCheckRadius": 80.0
        }
      },
      "RebirthModifier": {
        "DeployParticle": "particles/modifiers/rejuv_pending.vpcf",
        "DeploySound": "Rejuv.Rebirth.Start",
        "HudDisplayLocation": "DISPLAY_HUD_NONE",
        "HudIcon": "file://{images}/upgrades/mods_armor/portable_rejuvenator.png",
        "HudMessageText": "#modifier_citadel_respawn_credit",
        "LocalizationName": "modifier_citadel_respawn_credit",
        "ModifierDisplayLocaiton": "MODIFIER_DISPLAY_HEALTHBAR",
        "ParticleEffect": "particles/modifiers/respawn_credit_buff.vpcf",
        "ParticleStatusEffect": "particles/status_fx/status_fx_respawn_credit.vpcf",
        "ParticleStatusEffectConfig": "60",
        "RespawnDelay": 3.0,
        "RespawnLifePct": 100.0,
        "RespawnParticle": "particles/upgrades/portable_rejuvinator_respawn.vpcf",
        "RespawnSound": "Rejuv.Rebirth.Revive",
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_MAX_PERCENT",
            "value": 0.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_FIRE_RATE",
            "value": 0.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_TECH_DAMAGE_MULTIPLIER",
            "value": 0.0
          }
        ],
        "SmallIconCssClass": "rejuv_credit"
      },
      "_editor": {
        "folder_name": "Pickup Items"
      }
    },
    "citadel_item_powerup_spawner": {
      "ActiveParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "InactiveParticle": "particles/environment/powerup_spawner_inactive_ambient.vpcf",
      "InitialSpawnTime": 300,
      "InitialSpawnTimeTest": 1.0,
      "ModelScale": 1.5,
      "PrimaryPickups": [
        {
          "Pickup": "gun_powerup_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "survival_powerup_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "casting_powerup_pickup",
          "PickupWeight": 1.0
        },
        {
          "Pickup": "movement_powerup_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RespawnTimeTest": 10.0,
      "SpawnInterval": 300,
      "hModel": "models/npc/sewer_beast/sewer.vmdl"
    },
    "citadel_item_punchable_gold": {
      "AmbientParticle": "particles/generic/holding_gold_dropped.vpcf",
      "BobFrequency": 1.5,
      "BobHeight": 2.0,
      "GroundOffset": 60,
      "PunchPickupModifier": {
        "AuraRadius": 200.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "EnabledStateMask": "MODIFIER_STATE_IS_MELEE_TARGET",
        "HitSound": "Powerup.Punchable.Souls",
        "IsHidden": true,
        "PhysicsRadius": 20.0,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_PUNCHABLE_PICKUP",
          "IsHidden": true
        }
      },
      "_editor": {
        "folder_name": "Pickup Items"
      }
    },
    "citadel_koth_cashin": {
      "AuraModifier": {
        "AuraRadius": 787.402,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "AuraTargetingCylinderHalfHeight": 1000.0,
        "AuraTargetingCylinderUpOffset": 400.0,
        "IsHidden": true,
        "ModifierProvidedByAuraDuration": 1.0,
        "modifierProvidedByAura": {
          "Attributes": "MODIFIER_ATTRIBUTE_CANNOT_BE_PURGED",
          "EnabledStateMask": "MODIFIER_STATE_NEAR_IDOL_CASHIN | MODIFIER_STATE_VISIBLE_TO_ENEMY",
          "IsHidden": true
        }
      },
      "ComebackAuraModifier": {
        "AuraRadius": 787.402,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO_FRIENDLY",
        "AuraTargetingCylinderHalfHeight": 1000.0,
        "AuraTargetingCylinderUpOffset": 400.0,
        "IsHidden": true,
        "ModifierProvidedByAuraDuration": 1.0,
        "modifierProvidedByAura": {
          "Attributes": "MODIFIER_ATTRIBUTE_CANNOT_BE_PURGED",
          "BulletResist": 35,
          "IsHidden": true,
          "StatusResist": 35,
          "TechResist": 35
        }
      },
      "ComebackBounty": 170,
      "DecaySpeed": 0.0,
      "DestroyNearbyNeutrals": false,
      "DropOffPlayerBonusPercent": 35,
      "EnabledParticle": "",
      "EndParticleEnemy": "particles/generic/idol_pickup_end_enemy.vpcf",
      "EndParticleFriendly": "particles/generic/idol_pickup_end.vpcf",
      "GiveUpOrbs": 13,
      "HoldAtPercent": 99.0,
      "KothBlockedSound": "Koth.Zone.Blocked",
      "KothCaptureStartAnnounce": "Koth.Zone.Capture.Start.Announcement",
      "KothCashedInSoundEnemy": "Koth.Cashin.Complete.Opponent",
      "KothCashedInSoundFriendly": "Koth.Cashin.Complete.Team",
      "KothCashinLoopSound": "Koth.Zone.Capturing.Lp",
      "KothContestedLoopSound": "Koth.Zone.Contested.Lp",
      "KothContestedSound": "Koth.Zone.Contested",
      "KothGiveUpSound": "Koth.Cashin.Complete.Opponent",
      "KothGivingUpWarningLoopSound": "Koth.Timeout.Warning.Lp",
      "OnBecomeEnableParticle": "particles/npc/escort/escort_spawn.vpcf",
      "OnFullyCapturedParticle": "particles/npc/escort/escort_spawn.vpcf",
      "PingMinimapOnActive": false,
      "PreEnableParticle": "particles/npc/escort/escort_arriving.vpcf",
      "TimeToGiveUp": 60.0,
      "TimeToWarnAboutGivingUp": 50.0,
      "TotalTimeToCapture": 12,
      "TotalTimeToCaptureFavored": 6,
      "TotalTimeToCaptureUnfavored": 18,
      "TrooperModifier": {
        "DamagePercents": [
          100,
          120,
          140,
          160
        ],
        "DebuffType": "MODIFIER_DEBUFF_NO",
        "HealthPercents": [
          100,
          120,
          140,
          160
        ],
        "ModelScaleFractions": [
          1.57,
          1.6,
          1.63,
          1.66
        ],
        "ParticleStatusEffect": "particles/npc/unstable_rift/unstable_rift_trooper.vpcf",
        "StatusEffectPriority": 100
      },
      "ZoneHeightMeters": 14.0,
      "ZoneParticle": "particles/generic/koth_aura.vpcf",
      "modifierCapturer": {
        "HudDisplayLocation": "DISPLAY_HUD_CENTER",
        "IsHidden": true
      }
    },
    "citadel_refresh_spawner": {
      "ActiveParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "InactiveParticle": "particles/environment/powerup_spawner_inactive_ambient.vpcf",
      "InitialSpawnTime": 5.0,
      "InitialSpawnTimeTest": 5.0,
      "ModelScale": 0.5,
      "RespawnTime": 1,
      "RespawnTimeTest": 1,
      "RespawnTimerStartsAfterPickup": true,
      "SinglePickupOverride": "refresh_powerup_pickup",
      "hModel": ""
    },
    "dropped_necro_pickup": {
      "CollectionMethod": "VacuumTrigger",
      "Color": [
        0,
        0,
        0,
        0
      ],
      "FallGravity": 0.6,
      "HoverOffset": 15.0,
      "InitialVacuumSideSpeed": [
        100.0,
        -400.0
      ],
      "InitialVacuumUpSpeed": [
        120.0,
        350.0
      ],
      "OutlineColor": [
        193,
        245,
        81,
        255
      ],
      "OutlineRange": 2000.0,
      "PickupExpirationDuration": {
        "Base": 10.0,
        "MaxValue": 0.0,
        "PerMinuteAfterStart": 0.0,
        "StartMinute": 0.0
      },
      "PickupRadius": {
        "Base": 708.0,
        "PerMinuteAfterStart": 0,
        "StartMinute": 0.0
      },
      "PickupSound": "Necro.HauntingDead.Pickup",
      "SameTeamOnly": true,
      "VacuumInitialVelSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": 6.37255,
            "x": 0.0,
            "y": 0.538667
          },
          {
            "SlopeIncoming": 3.153351,
            "SlopeOutgoing": 3.153351,
            "x": 0.033019,
            "y": 0.749083
          },
          {
            "SlopeIncoming": 0.399116,
            "SlopeOutgoing": 0.399116,
            "x": 0.088081,
            "y": 0.816417
          },
          {
            "SlopeIncoming": 0.267879,
            "SlopeOutgoing": 0.267879,
            "x": 0.433696,
            "y": 0.909
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          }
        ],
        "vDomainMaxs": [
          0.5,
          1.0
        ],
        "vDomainMins": [
          0.0,
          0.0
        ]
      },
      "VacuumToPlayerSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": -8609.929688,
            "x": 0.0,
            "y": 467.916687
          },
          {
            "SlopeIncoming": -8609.929688,
            "SlopeOutgoing": 9633.584961,
            "x": 0.105087,
            "y": -436.874969
          },
          {
            "SlopeIncoming": 9633.584961,
            "SlopeOutgoing": 4603.740723,
            "x": 0.275455,
            "y": 1204.375
          },
          {
            "SlopeIncoming": 2216.488281,
            "SlopeOutgoing": 0.0,
            "x": 0.634412,
            "y": 2000.0
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          }
        ],
        "vDomainMaxs": [
          1.0,
          2000.0
        ],
        "vDomainMins": [
          0.0,
          -500.0
        ]
      },
      "bPhysicallyDropToTheGroundOnSpawn": true,
      "enemyParticle": "particles/abilities/necro/necro_pickup_enemy.vpcf",
      "fInitialSpawnXYSpeed": [
        85.0,
        125.0
      ],
      "fInitialSpawnZSpeed": [
        30.0,
        110.0
      ],
      "friendlyModelParticle": "",
      "friendlyParticle": "particles/abilities/necro/necro_pickup.vpcf",
      "gainedParticle": "particles/abilities/necro/necro_pickup_gained.vpcf",
      "hModel": "",
      "vacuumStartParticle": "particles/abilities/necro/necro_pickup_splat.vpcf"
    },
    "dropped_soul_orb": {
      "CollectionMethod": "VacuumTrigger",
      "Color": [
        0,
        0,
        0,
        0
      ],
      "FallGravity": 0.4,
      "HoverOffset": 20.0,
      "InitialVacuumSideSpeed": [
        200.0,
        200.0
      ],
      "InitialVacuumUpSpeed": [
        100.0,
        100.0
      ],
      "OutlineColor": [
        112,
        248,
        193,
        255
      ],
      "OutlineRange": 2000.0,
      "PickupExpirationDuration": {
        "Base": 18.0,
        "MaxValue": 40.0,
        "PerMinuteAfterStart": 4.0,
        "StartMinute": 10.0
      },
      "PickupRadius": {
        "Base": 709,
        "PerMinuteAfterStart": 0,
        "StartMinute": 0.0
      },
      "PickupSound": "Pickup.XpOrb",
      "SameTeamOnly": true,
      "VacuumInitialVelSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": 3.105128,
            "x": 0.0,
            "y": 0.0
          },
          {
            "SlopeIncoming": 7.669573,
            "SlopeOutgoing": 7.669573,
            "x": 0.037948,
            "y": 0.117833
          },
          {
            "SlopeIncoming": 0.423242,
            "SlopeOutgoing": 0.423242,
            "x": 0.130385,
            "y": 1.0
          },
          {
            "SlopeIncoming": -2.70552,
            "SlopeOutgoing": -2.70552,
            "x": 0.356127,
            "y": 0.2525
          },
          {
            "SlopeIncoming": -1.755022,
            "SlopeOutgoing": 0.0,
            "x": 0.5,
            "y": 0.0
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          }
        ],
        "vDomainMaxs": [
          0.5,
          1.0
        ],
        "vDomainMins": [
          0.0,
          0.0
        ]
      },
      "VacuumToPlayerSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": -4004.629639,
            "x": 0.0,
            "y": -16.041637
          },
          {
            "SlopeIncoming": -4004.629639,
            "SlopeOutgoing": 4603.734863,
            "x": 0.105087,
            "y": -436.874969
          },
          {
            "SlopeIncoming": 4603.734863,
            "SlopeOutgoing": 0.0,
            "x": 0.634412,
            "y": 2000.0
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          }
        ],
        "vDomainMaxs": [
          1.0,
          2000.0
        ],
        "vDomainMins": [
          0.0,
          -500.0
        ]
      },
      "bPhysicallyDropToTheGroundOnSpawn": true,
      "enemyParticle": "particles/environment/spirit_orb_dropped_ambient_enemy.vpcf",
      "fInitialSpawnXYSpeed": [
        85.0,
        125.0
      ],
      "fInitialSpawnZSpeed": [
        50.0,
        50.0
      ],
      "friendlyParticle": "particles/environment/spirit_orb_dropped_ambient.vpcf",
      "gainedParticle": "particles/environment/spirit_orb_dropped_gained.vpcf",
      "hModel": "",
      "vacuumStartParticle": "particles/environment/spirit_orb_dropped_claim.vpcf"
    },
    "firerate_permanent_pickup": {
      "Color": [
        200,
        0,
        0
      ],
      "DefaultMaterialGroupName": "red",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_FIRE_RATE",
            "value": 1.5
          }
        ],
        "SmallIconCssClass": "fire_rate"
      },
      "NameLocString": "firerate_permanent_pickup_label",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Weapon",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "firerate_permanent_pickup_lv2": {
      "Color": [
        200,
        0,
        0
      ],
      "DefaultMaterialGroupName": "red",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_FIRE_RATE",
            "value": 2.0
          }
        ],
        "SmallIconCssClass": "fire_rate"
      },
      "NameLocString": "firerate_permanent_pickup_label_lv2",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Weapon",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "firerate_permanent_pickup_lv3": {
      "Color": [
        200,
        0,
        0
      ],
      "DefaultMaterialGroupName": "red",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_FIRE_RATE",
            "value": 2.5
          }
        ],
        "SmallIconCssClass": "fire_rate"
      },
      "NameLocString": "firerate_permanent_pickup_label_lv3",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Weapon",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "gun_powerup_pickup": {
      "AmbientSound": "Powerup.Gun_Lp",
      "AuraModifier": {
        "AuraRadius": 300.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_HEAVY_PUNCHABLE_ITEM",
          "IsHidden": true
        }
      },
      "CollectionMethod": "Punch",
      "CollisionRadius": 40.0,
      "Color": [
        218,
        148,
        65
      ],
      "DamagedParticle": "particles/environment/powerup_spawner_ambient_damaged.vpcf",
      "HitSound": "Powerup.Pickup.Punch",
      "HitsRequired": 1,
      "MinimapCssClasses": [
        "powerup_gun",
        "powerup_spawn"
      ],
      "Modifer": {
        "AlwaysShowInStatModifierUI": [
          "MODIFIER_VALUE_FIRE_RATE",
          "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT"
        ],
        "BuffParticle": "particles/environment/powerup_spawner_buff.vpcf",
        "Color": [
          218,
          148,
          65
        ],
        "DrawOverheadStatus": "OVERHEAD_DRAW_FOR_EVERYONE",
        "Duration": 160.0,
        "HudDisplayLocation": "DISPLAY_HUD_NONE",
        "HudIcon": "file://{images}/hud/modifiers/icon_powerup.svg",
        "LocalizationName": "gun_powerup_pickup",
        "ModifierDisplayLocaiton": "MODIFIER_DISPLAY_HEALTHBAR",
        "ModifierValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_FIRE_RATE",
            "valueMax": 35.0,
            "valueMin": 12.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_AMMO_CLIP_SIZE_PERCENT",
            "valueMax": 70.0,
            "valueMin": 35.0
          }
        ],
        "SmallIconCssClass": "gunpower_pickup",
        "TimeMax": 40,
        "TimeMin": 5
      },
      "NameLocString": "gun_powerup_pickup",
      "NameOffset": 150,
      "ParryCheckModifier": {
        "ParryCheckRadius": 60.0
      },
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Generic",
      "ShowOnMinimap": true,
      "SpawnSound": "Powerup.BridgeBuff.Spawn",
      "TempParticleSheetIndex": 9,
      "enemyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "friendlyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "gainedParticle": "particles/generic/bridge_buff.vpcf",
      "hModel": "models/null.vmdl"
    },
    "hp_permanent_pickup": {
      "Color": [
        50,
        205,
        50
      ],
      "DefaultMaterialGroupName": "green",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_MAX",
            "value": 15
          }
        ]
      },
      "NameLocString": "hp_permanent_pickup_label",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Health",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "hp_permanent_pickup_lv2": {
      "Color": [
        50,
        205,
        50
      ],
      "DefaultMaterialGroupName": "green",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_MAX",
            "value": 20
          }
        ]
      },
      "NameLocString": "hp_permanent_pickup_label_lv2",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Health",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "hp_permanent_pickup_lv3": {
      "Color": [
        50,
        205,
        50
      ],
      "DefaultMaterialGroupName": "green",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_MAX",
            "value": 30
          }
        ]
      },
      "NameLocString": "hp_permanent_pickup_label_lv3",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Health",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "info_neutral_trooper_camp": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "SpawnIntervalChange": 0,
      "SpawnIntervalMin": 0
    },
    "item_pickup": {
      "BecomeInteractiveSound": "Powerup.Pickup.Health",
      "CollectionMethod": "Punch",
      "Color": [
        241,
        215,
        130
      ],
      "HeavyMeleeOnly": false,
      "HoverOffset": 10.0,
      "PickupExpirationDuration": {
        "Base": 500.0
      },
      "PickupRadius": {
        "Base": 5.0
      },
      "PickupSound": "UI.Shop.Mod.Activate",
      "SpawnSound": "Powerup.Spawn.Generic",
      "bPhysicallyDropToTheGroundOnSpawn": true,
      "fInitialSpawnXYSpeed": [
        -50.0,
        50.0
      ],
      "fInitialSpawnZSpeed": [
        300.0,
        300.0
      ],
      "friendlyParticle": "particles/environment/breakable_item_drop_beam.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "medic_trooper_aoe_health_pickup_amber": {
      "AOERadius": 1377.95,
      "AOETargetFlags": "",
      "AOETargetTypes": "CITADEL_UNIT_TARGET_HERO_FRIENDLY | CITADEL_UNIT_TARGET_TROOPER_FRIENDLY",
      "CollectionMethod": "VacuumTrigger",
      "Color": [
        0,
        0,
        0,
        0
      ],
      "EffectDistanceToRadiusRemap": [
        250.0,
        20.0,
        1.0,
        0.0
      ],
      "HealFixed": {
        "Base": 0.0,
        "PerMinuteAfterStart": 0.0
      },
      "HoverOffset": 0.0,
      "InitialVacuumSideSpeed": [
        0.0,
        0.0
      ],
      "InitialVacuumUpSpeed": [
        400.0,
        400.0
      ],
      "MissingPctRegen": {
        "Base": 13.0
      },
      "ParticleAOEHeal": "particles/environment/health_pack_aoe_heal.vpcf",
      "PickupExpirationDuration": {
        "Base": 18.0,
        "MaxValue": 40.0,
        "PerMinuteAfterStart": 2.0,
        "StartMinute": 10.0
      },
      "PickupRadius": {
        "Base": 709,
        "MaxValue": 1181.1,
        "PerMinuteAfterStart": 59.0125,
        "StartMinute": 10.0
      },
      "PickupSound": "Powerup.HealthPack.Pickup",
      "RegenDuration": 4.0,
      "RegenDurationTroopers": 8.0,
      "RegenFixed": {
        "Base": 150.0,
        "PerMinuteAfterStart": 2.0
      },
      "RegenHPS": 0.0,
      "RegenModifier": {
        "HealingLoopSoundOverride": {
          "EndSound": "",
          "LoopSound": "",
          "StartSound": "Player.Heal.OverTime.LowPriority.Start"
        },
        "IsHidden": true
      },
      "RegenTrooperMulti": 1.3,
      "SameTeamOnly": true,
      "SolidRadius": 10.0,
      "UseFixedDuration": true,
      "VacuumInitialVelSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": 3.105128,
            "x": 0.0,
            "y": 0.0
          },
          {
            "SlopeIncoming": 7.669573,
            "SlopeOutgoing": 7.669573,
            "x": 0.037948,
            "y": 0.117833
          },
          {
            "SlopeIncoming": 0.423242,
            "SlopeOutgoing": 0.423242,
            "x": 0.130385,
            "y": 1.0
          },
          {
            "SlopeIncoming": -2.70552,
            "SlopeOutgoing": -2.70552,
            "x": 0.356127,
            "y": 0.2525
          },
          {
            "SlopeIncoming": -1.755022,
            "SlopeOutgoing": 0.0,
            "x": 0.5,
            "y": 0.0
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          }
        ],
        "vDomainMaxs": [
          0.5,
          1.0
        ],
        "vDomainMins": [
          0.0,
          0.0
        ]
      },
      "VacuumStartSound": "Powerup.HealthPack.Vacuum",
      "VacuumToPlayerSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": -15303.386719,
            "x": 0.0,
            "y": -16.041656
          },
          {
            "SlopeIncoming": -483.901276,
            "SlopeOutgoing": -483.901276,
            "x": 0.031624,
            "y": -500.0
          },
          {
            "SlopeIncoming": 7392.94043,
            "SlopeOutgoing": 7392.94043,
            "x": 0.086967,
            "y": -58.124969
          },
          {
            "SlopeIncoming": 3395.157715,
            "SlopeOutgoing": 4762.609375,
            "x": 0.173933,
            "y": 552.083374
          },
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": 2265.070801,
            "x": 0.452622,
            "y": 1183.333374
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_FREE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_FREE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          }
        ],
        "vDomainMaxs": [
          1.0,
          2000.0
        ],
        "vDomainMins": [
          0.0,
          -500.0
        ]
      },
      "bPhysicallyDropToTheGroundOnSpawn": true,
      "enemyInteractiveParticle": "particles/environment/health_pack_dropped_ambient_amber_otg_enemy.vpcf",
      "enemyModelParticle": "particles/environment/health_pack_dropped_ambient_amber_entity_enemy.vpcf",
      "enemyParticle": "particles/environment/health_pack_dropped_ambient_amber_enemy.vpcf",
      "fInitialSpawnXYSpeed": [
        0.0,
        0.0
      ],
      "fInitialSpawnZSpeed": [
        50.0,
        200.0
      ],
      "friendlyInteractiveParticle": "particles/environment/health_pack_dropped_ambient_amber_otg.vpcf",
      "friendlyModelParticle": "particles/environment/health_pack_dropped_ambient_amber_entity.vpcf",
      "friendlyParticle": "particles/environment/health_pack_dropped_ambient_amber.vpcf",
      "gainedParticle": "particles/environment/health_pack_projectile_endcap.vpcf",
      "hModel": "",
      "vacuumStartParticle": "particles/environment/spirit_orb_dropped_claim.vpcf"
    },
    "medic_trooper_aoe_health_pickup_sapphire": {
      "AOERadius": 1377.95,
      "AOETargetFlags": "",
      "AOETargetTypes": "CITADEL_UNIT_TARGET_HERO_FRIENDLY | CITADEL_UNIT_TARGET_TROOPER_FRIENDLY",
      "CollectionMethod": "VacuumTrigger",
      "Color": [
        0,
        0,
        0,
        0
      ],
      "EffectDistanceToRadiusRemap": [
        250.0,
        20.0,
        1.0,
        0.0
      ],
      "HealFixed": {
        "Base": 0.0,
        "PerMinuteAfterStart": 0.0
      },
      "HoverOffset": 0.0,
      "InitialVacuumSideSpeed": [
        0.0,
        0.0
      ],
      "InitialVacuumUpSpeed": [
        400.0,
        400.0
      ],
      "MissingPctRegen": {
        "Base": 13.0
      },
      "ParticleAOEHeal": "particles/environment/health_pack_aoe_heal.vpcf",
      "PickupExpirationDuration": {
        "Base": 18.0,
        "MaxValue": 40.0,
        "PerMinuteAfterStart": 2.0,
        "StartMinute": 10.0
      },
      "PickupRadius": {
        "Base": 709,
        "MaxValue": 1181.1,
        "PerMinuteAfterStart": 59.0125,
        "StartMinute": 10.0
      },
      "PickupSound": "Powerup.HealthPack.Pickup",
      "RegenDuration": 4.0,
      "RegenDurationTroopers": 8.0,
      "RegenFixed": {
        "Base": 150.0,
        "PerMinuteAfterStart": 2.0
      },
      "RegenHPS": 0.0,
      "RegenModifier": {
        "HealingLoopSoundOverride": {
          "EndSound": "",
          "LoopSound": "",
          "StartSound": "Player.Heal.OverTime.LowPriority.Start"
        },
        "IsHidden": true
      },
      "RegenTrooperMulti": 1.3,
      "SameTeamOnly": true,
      "SolidRadius": 10.0,
      "UseFixedDuration": true,
      "VacuumInitialVelSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": 3.105128,
            "x": 0.0,
            "y": 0.0
          },
          {
            "SlopeIncoming": 7.669573,
            "SlopeOutgoing": 7.669573,
            "x": 0.037948,
            "y": 0.117833
          },
          {
            "SlopeIncoming": 0.423242,
            "SlopeOutgoing": 0.423242,
            "x": 0.130385,
            "y": 1.0
          },
          {
            "SlopeIncoming": -2.70552,
            "SlopeOutgoing": -2.70552,
            "x": 0.356127,
            "y": 0.2525
          },
          {
            "SlopeIncoming": -1.755022,
            "SlopeOutgoing": 0.0,
            "x": 0.5,
            "y": 0.0
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_LINEAR"
          }
        ],
        "vDomainMaxs": [
          0.5,
          1.0
        ],
        "vDomainMins": [
          0.0,
          0.0
        ]
      },
      "VacuumStartSound": "Powerup.HealthPack.Vacuum",
      "VacuumToPlayerSpeedCurve": {
        "spline": [
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": -15303.386719,
            "x": 0.0,
            "y": -16.041656
          },
          {
            "SlopeIncoming": -483.901276,
            "SlopeOutgoing": -483.901276,
            "x": 0.031624,
            "y": -500.0
          },
          {
            "SlopeIncoming": 7392.94043,
            "SlopeOutgoing": 7392.94043,
            "x": 0.086967,
            "y": -58.124969
          },
          {
            "SlopeIncoming": 3395.157715,
            "SlopeOutgoing": 4762.609375,
            "x": 0.173933,
            "y": 552.083374
          },
          {
            "SlopeIncoming": 0.0,
            "SlopeOutgoing": 2265.070801,
            "x": 0.452622,
            "y": 1183.333374
          }
        ],
        "tangents": [
          {
            "IncomingTangent": "CURVE_TANGENT_LINEAR",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_SPLINE",
            "OutgoingTangent": "CURVE_TANGENT_FREE"
          },
          {
            "IncomingTangent": "CURVE_TANGENT_FREE",
            "OutgoingTangent": "CURVE_TANGENT_SPLINE"
          }
        ],
        "vDomainMaxs": [
          1.0,
          2000.0
        ],
        "vDomainMins": [
          0.0,
          -500.0
        ]
      },
      "bPhysicallyDropToTheGroundOnSpawn": true,
      "enemyInteractiveParticle": "particles/environment/health_pack_dropped_ambient_sapphire_otg_enemy.vpcf",
      "enemyModelParticle": "particles/environment/health_pack_dropped_ambient_sapphire_entity_enemy.vpcf",
      "enemyParticle": "particles/environment/health_pack_dropped_ambient_sapphire_enemy.vpcf",
      "fInitialSpawnXYSpeed": [
        0.0,
        0.0
      ],
      "fInitialSpawnZSpeed": [
        50.0,
        200.0
      ],
      "friendlyInteractiveParticle": "particles/environment/health_pack_dropped_ambient_sapphire_otg.vpcf",
      "friendlyModelParticle": "particles/environment/health_pack_dropped_ambient_sapphire_entity.vpcf",
      "friendlyParticle": "particles/environment/health_pack_dropped_ambient_sapphire.vpcf",
      "gainedParticle": "particles/environment/health_pack_projectile_endcap.vpcf",
      "hModel": "",
      "vacuumStartParticle": "particles/environment/spirit_orb_dropped_claim.vpcf"
    },
    "movement_powerup_pickup": {
      "AmbientSound": "Powerup.Movement_Lp",
      "AuraModifier": {
        "AuraRadius": 300.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_HEAVY_PUNCHABLE_ITEM",
          "IsHidden": true
        }
      },
      "CollectionMethod": "Punch",
      "CollisionRadius": 40.0,
      "Color": [
        50,
        50,
        255
      ],
      "DamagedParticle": "particles/environment/powerup_spawner_ambient_damaged.vpcf",
      "HitSound": "Powerup.Pickup.Punch",
      "HitsRequired": 1,
      "MinimapCssClasses": [
        "powerup_movement",
        "powerup_spawn"
      ],
      "Modifer": {
        "AlwaysShowInStatModifierUI": [
          "MODIFIER_VALUE_SPRINT_SPEED_BONUS",
          "MODIFIER_VALUE_SPRINT_SPEED_BONUS",
          "MODIFIER_VALUE_ZIP_LINE_SPEED_PERCENTAGE"
        ],
        "BuffParticle": "particles/environment/powerup_spawner_buff.vpcf",
        "Color": [
          50,
          50,
          255
        ],
        "DrawOverheadStatus": "OVERHEAD_DRAW_FOR_EVERYONE",
        "Duration": 160.0,
        "HudDisplayLocation": "DISPLAY_HUD_NONE",
        "HudIcon": "file://{images}/hud/modifiers/icon_powerup.svg",
        "LocalizationName": "movement_powerup_pickup",
        "ModifierDisplayLocaiton": "MODIFIER_DISPLAY_HEALTHBAR",
        "ModifierValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_STAMINA",
            "valueMax": 4,
            "valueMin": 2
          },
          {
            "ModifierValue": "MODIFIER_VALUE_SPRINT_SPEED_BONUS",
            "valueMax": 157.48,
            "valueMin": 59.0551
          },
          {
            "ModifierValue": "MODIFIER_VALUE_ZIP_LINE_SPEED_PERCENTAGE",
            "valueMax": 80,
            "valueMin": 40
          },
          {
            "ModifierValue": "MODIFIER_VALUE_STAMINA_REGEN_PER_SECOND_PERCENTAGE",
            "valueMax": 50,
            "valueMin": 20
          }
        ],
        "SmallIconCssClass": "movement_pickup",
        "TimeMax": 40,
        "TimeMin": 5
      },
      "NameLocString": "movement_powerup_pickup",
      "NameOffset": 150,
      "ParryCheckModifier": {
        "ParryCheckRadius": 60.0
      },
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Generic",
      "ShowOnMinimap": true,
      "SpawnSound": "Powerup.BridgeBuff.Spawn",
      "TempParticleSheetIndex": 8,
      "enemyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "friendlyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "gainedParticle": "particles/generic/bridge_buff.vpcf",
      "hModel": "models/null.vmdl"
    },
    "neutral_camp_bug": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 120,
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 120,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_bug_herotest": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 0,
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 10,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_medium": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 300,
      "NeutralType": "NEUTRAL_TROOPER_NORMAL",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 290,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_medium_herotest": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 0,
      "NeutralType": "NEUTRAL_TROOPER_NORMAL",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 30,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_midboss": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 0,
      "NeutralType": "NEUTRAL_SUPER",
      "SpawnIntervalChange": -60,
      "SpawnIntervalInSeconds": 420,
      "SpawnIntervalMin": 300
    },
    "neutral_camp_strong": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 480,
      "NeutralType": "NEUTRAL_TROOPER_STRONG",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 335,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_strong_herotest": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 0,
      "NeutralType": "NEUTRAL_TROOPER_STRONG",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 30,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_vaults": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 480,
      "NeutralType": "NEUTRAL_VAULT",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 300,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_vaults_herotest": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 0,
      "NeutralType": "NEUTRAL_VAULT",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 30,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_weak": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 120,
      "NeutralType": "NEUTRAL_TROOPER_WEAK",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 85,
      "SpawnIntervalMin": 0
    },
    "neutral_camp_weak_herotest": {
      "AlertAmbient": "",
      "IdleAmbient": "",
      "InitialSpawnDelayInSeconds": 0,
      "NeutralType": "NEUTRAL_TROOPER_WEAK",
      "SpawnIntervalChange": 0,
      "SpawnIntervalInSeconds": 30,
      "SpawnIntervalMin": 0
    },
    "refresh_powerup_pickup": {
      "AmbientSound": "Powerup.Survival_Lp",
      "AuraModifier": {
        "AuraRadius": 300.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_HEAVY_PUNCHABLE_ITEM",
          "IsHidden": true
        }
      },
      "CollectionMethod": "Punch",
      "CollisionRadius": 40.0,
      "Color": [
        203,
        143,
        252
      ],
      "DamagedParticle": "particles/environment/powerup_spawner_ambient_damaged.vpcf",
      "HitSound": "Powerup.Pickup.Punch",
      "HitsRequired": 1,
      "MinimapCssClasses": [
        "powerup_refresh",
        "powerup_spawn"
      ],
      "Modifer": {
        "HudDisplayLocation": "DISPLAY_HUD_NONE"
      },
      "NameLocString": "refresh_powerup_pickup",
      "NameOffset": 150,
      "ParryCheckModifier": {
        "ParryCheckRadius": 60.0
      },
      "ParticleRadius": 1,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Generic",
      "ShowOnMinimap": true,
      "SpawnSound": "Powerup.BridgeBuff.Spawn",
      "TempParticleSheetIndex": 10,
      "enemyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "friendlyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "gainedParticle": "particles/generic/bridge_buff.vpcf",
      "hModel": "models/null.vmdl"
    },
    "small_gold_pickup": {
      "Color": [
        0,
        255,
        204
      ],
      "DefaultMaterialGroupName": "gold",
      "GoldAmount": 23,
      "GoldPerMinuteAmount": 2.0,
      "IsPermanentPickup": true,
      "NameLocString": "#small_gold_pickup_label:p",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Souls",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "spirit_permanent_pickup": {
      "Color": [
        88,
        20,
        180
      ],
      "DefaultMaterialGroupName": "purple",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_TECH_POWER",
            "value": 2.0
          }
        ]
      },
      "NameLocString": "spirit_permanent_pickup_label",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Spirit",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "spirit_permanent_pickup_lv2": {
      "Color": [
        88,
        20,
        180
      ],
      "DefaultMaterialGroupName": "purple",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_TECH_POWER",
            "value": 3.0
          }
        ]
      },
      "NameLocString": "spirit_permanent_pickup_label_lv2",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Spirit",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "spirit_permanent_pickup_lv3": {
      "Color": [
        88,
        20,
        180
      ],
      "DefaultMaterialGroupName": "purple",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_TECH_POWER",
            "value": 4.0
          }
        ]
      },
      "NameLocString": "spirit_permanent_pickup_label_lv3",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Spirit",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "survival_powerup_pickup": {
      "AmbientSound": "Powerup.Survival_Lp",
      "AuraModifier": {
        "AuraRadius": 300.0,
        "AuraSearchType": "CITADEL_UNIT_TARGET_HERO",
        "IsHidden": true,
        "modifierProvidedByAura": {
          "EnabledStateMask": "MODIFIER_STATE_NEAR_HEAVY_PUNCHABLE_ITEM",
          "IsHidden": true
        }
      },
      "CollectionMethod": "Punch",
      "CollisionRadius": 40.0,
      "Color": [
        122,
        184,
        29
      ],
      "DamagedParticle": "particles/environment/powerup_spawner_ambient_damaged.vpcf",
      "HitSound": "Powerup.Pickup.Punch",
      "HitsRequired": 1,
      "MinimapCssClasses": [
        "powerup_survival",
        "powerup_spawn"
      ],
      "Modifer": {
        "AlwaysShowInStatModifierUI": [
          "MODIFIER_VALUE_HEALTH_MAX",
          "MODIFIER_VALUE_HEALTH_REGEN_PER_SECOND"
        ],
        "BuffParticle": "particles/environment/powerup_spawner_buff.vpcf",
        "Color": [
          122,
          184,
          29
        ],
        "DrawOverheadStatus": "OVERHEAD_DRAW_FOR_EVERYONE",
        "Duration": 160.0,
        "HudDisplayLocation": "DISPLAY_HUD_NONE",
        "HudIcon": "file://{images}/hud/modifiers/icon_powerup.svg",
        "LocalizationName": "survival_powerup_pickup",
        "ModifierDisplayLocaiton": "MODIFIER_DISPLAY_HEALTHBAR",
        "ModifierValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_MAX",
            "valueMax": 750.0,
            "valueMin": 200.0
          },
          {
            "ModifierValue": "MODIFIER_VALUE_HEALTH_REGEN_PER_SECOND",
            "valueMax": 40,
            "valueMin": 4
          }
        ],
        "SmallIconCssClass": "survival_pickup",
        "TimeMax": 40,
        "TimeMin": 5
      },
      "NameLocString": "survival_powerup_pickup",
      "NameOffset": 150,
      "ParryCheckModifier": {
        "ParryCheckRadius": 60.0
      },
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 300,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Generic",
      "ShowOnMinimap": true,
      "SpawnSound": "Powerup.BridgeBuff.Spawn",
      "TempParticleSheetIndex": 6,
      "enemyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "friendlyParticle": "particles/environment/powerup_spawner_ambient.vpcf",
      "gainedParticle": "particles/generic/bridge_buff.vpcf",
      "hModel": "models/null.vmdl"
    },
    "vehicle_bike_flying": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/bike_breakable.vmdl"
    },
    "vehicle_car_01": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/car_01_breakable.vmdl"
    },
    "vehicle_car_04": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/car_04_breakable.vmdl"
    },
    "vehicle_car_05": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/car_05_breakable.vmdl"
    },
    "vehicle_ground_ship_03": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/ground_ship_03_breakable.vmdl"
    },
    "vehicle_large_truck_01": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/large_truck_01_breakable.vmdl"
    },
    "vehicle_large_truck_02": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/large_truck_02_breakable.vmdl"
    },
    "vehicle_mini_truck_03": {
      "AnimgraphParamDamageReceived": "",
      "AnimgraphParamOnHit": "b_Hit_Trigger",
      "BreakOnDodgeTouch": false,
      "DamageSound": "",
      "DamagedByAbilities": true,
      "DamagedByBullets": true,
      "DamagedByMelee": true,
      "DropChance": 0.0,
      "Health": 100000000,
      "InitialSpawnTime": 0,
      "IsMantleable": true,
      "IsPermanent": true,
      "PrimaryDropChance": 60.0,
      "PrimaryPickups": [
        {
          "Pickup": "small_gold_pickup",
          "PickupWeight": 1.0
        }
      ],
      "RenderAfterDeath": false,
      "RespawnTime": 180.0,
      "RollType": "ECitadelRandomRoll_BreakableGoldPickup",
      "SolidAfterDeath": false,
      "hModel": "models/props_vehicles/mini_truck_03_breakable.vmdl"
    },
    "wp_permanent_pickup": {
      "Color": [
        200,
        85,
        0
      ],
      "DefaultMaterialGroupName": "orange",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_WEAPON_DAMAGE_INCREASE",
            "value": 3
          }
        ]
      },
      "NameLocString": "wp_permanent_pickup_label",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Weapon",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "wp_permanent_pickup_lv2": {
      "Color": [
        200,
        85,
        0
      ],
      "DefaultMaterialGroupName": "orange",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_WEAPON_DAMAGE_INCREASE",
            "value": 4
          }
        ]
      },
      "NameLocString": "wp_permanent_pickup_label_lv2",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Weapon",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "wp_permanent_pickup_lv3": {
      "Color": [
        200,
        85,
        0
      ],
      "DefaultMaterialGroupName": "orange",
      "IsPermanentPickup": true,
      "Modifer": {
        "ScriptValues": [
          {
            "ModifierValue": "MODIFIER_VALUE_WEAPON_DAMAGE_INCREASE",
            "value": 6
          }
        ]
      },
      "NameLocString": "wp_permanent_pickup_label_lv3",
      "NameOffset": 60,
      "ParticleRadius": 1.5,
      "PickupExpirationDuration": {
        "Base": 30,
        "PerMinuteAfterStart": 0
      },
      "PickupRadius": {
        "Base": 85.0,
        "PerMinuteAfterStart": 0
      },
      "PickupSound": "Powerup.Pickup.Weapon",
      "ShowOnMinimap": false,
      "SpawnSound": "Powerup.Spawn.Generic",
      "enemyParticle": "particles/environment/breakable_item_drop.vpcf",
      "friendlyParticle": "particles/environment/breakable_item_drop.vpcf",
      "gainedParticle": "particles/generic/powerup_spawner_gained.vpcf",
      "hModel": "models/props_gameplay/powerup_idol/powerup_idol.vmdl"
    },
    "xp_orb": {
      "BurstSpeedDuration": 0.1,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.12,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneOffset": 0,
      "LateralSpeedMax": 40.0,
      "LateralSpeedMin": 40.0,
      "LifeTime": 3.0,
      "OrbClaimWindow": 0,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.3,
      "OrbSpawnOffsetRandomXYZ": 0,
      "OrbSpawnOffsetZ": 24,
      "OscillateFrequency": 1.0,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "UpSpeedMax": 65.0,
      "UpSpeedMin": 65.0,
      "UseKillerPlaneOffsets": false
    },
    "xp_orb_idol_dropoff": {
      "BurstSpeedDuration": 0.1,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.12,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneOffset": 0,
      "LateralSpeedMax": 60.0,
      "LateralSpeedMin": 10.0,
      "LifeTime": 3.0,
      "OrbClaimWindow": 0,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.3,
      "OrbSpawnOffsetRandomXYZ": 10.0,
      "OrbSpawnOffsetZ": 10.0,
      "OscillateFrequency": 1.0,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "UpSpeedMax": 60.0,
      "UpSpeedMin": 10.0,
      "UseKillerPlaneOffsets": false
    },
    "xp_orb_koth_giveup": {
      "BurstSpeedDuration": 2,
      "BurstSpeedMultiplier": 10,
      "CollisionRadius": 15.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 1.0,
      "InvulDuration": 0.12,
      "InvulDurationMax": 0.5,
      "InvulDurationMin": 0.4,
      "KillerPlaneOffset": 0,
      "LateralSpeedMax": 10.0,
      "LateralSpeedMin": 5.0,
      "LifeTime": 25.0,
      "OrbClaimWindow": 0,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.1,
      "OrbSpawnOffsetRandomXYZ": 20.0,
      "OrbSpawnOffsetZ": 10.0,
      "OscillateFrequency": 0.5,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "Radius": 1.5,
      "UpSpeedMax": 30.0,
      "UpSpeedMin": 20.0,
      "UseKillerPlaneOffsets": false
    },
    "xp_orb_neutral": {
      "BurstSpeedDuration": 0.2,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.2,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneHorizontalDecayRate": 15.0,
      "KillerPlaneHorizontalSpeedX": 40.0,
      "KillerPlaneHorizontalSpeedY": 55.0,
      "KillerPlaneLaunchDelay": 0.1,
      "KillerPlaneLaunchOffset": 15.0,
      "KillerPlaneOffset": 15.0,
      "KillerPlaneSpeedNoise": 10.0,
      "KillerPlaneVerticalSpeed": 70.0,
      "LateralMoveDuration": 0.3,
      "LateralSpeedMax": 40.0,
      "LateralSpeedMin": 40.0,
      "LifeTime": 1.7,
      "OrbClaimWindow": 0.06,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.7,
      "OrbSpawnDelayMin": 0.4,
      "OrbSpawnOffsetRandomXYZ": 64.0,
      "OrbSpawnOffsetZ": 30,
      "OscillateFrequency": 0.01,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "Radius": 1.2,
      "UpSpeedMax": 40.0,
      "UpSpeedMin": 30.0,
      "UseKillerPlaneOffsets": true
    },
    "xp_orb_objective": {
      "BurstSpeedDuration": 0.1,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.12,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "IsObjective": true,
      "KillerPlaneOffset": 0,
      "LateralSpeedMax": 60.0,
      "LateralSpeedMin": 40.0,
      "LifeTime": 3.0,
      "OrbClaimWindow": 0,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.3,
      "OrbSpawnOffsetRandomXYZ": 64,
      "OrbSpawnOffsetZ": 0,
      "OscillateFrequency": 1.0,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "UpSpeedMax": 65.0,
      "UpSpeedMin": 65.0,
      "UseKillerPlaneOffsets": false
    },
    "xp_orb_player_kill": {
      "BurstSpeedDuration": 0.1,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.12,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneOffset": 0,
      "LateralSpeedMax": 40.0,
      "LateralSpeedMin": 40.0,
      "LifeTime": 3.0,
      "OrbClaimWindow": 0,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.3,
      "OrbSpawnOffsetRandomXYZ": 0,
      "OrbSpawnOffsetZ": 24,
      "OscillateFrequency": 1.0,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "UpSpeedMax": 65.0,
      "UpSpeedMin": 65.0,
      "UseKillerPlaneOffsets": false
    },
    "xp_orb_siege_trooper": {
      "BurstSpeedDuration": 0.2,
      "BurstSpeedMultiplier": 1.2,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.2,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneHorizontalDecayRate": 15.0,
      "KillerPlaneHorizontalSpeedX": 65.0,
      "KillerPlaneHorizontalSpeedY": 55.0,
      "KillerPlaneLaunchDelay": 0.1,
      "KillerPlaneLaunchOffset": 15.0,
      "KillerPlaneOffset": 15.0,
      "KillerPlaneSpeedNoise": 10.0,
      "KillerPlaneVerticalSpeed": 115.0,
      "LateralSpeedMax": 20.0,
      "LateralSpeedMin": 20.0,
      "LifeTime": 3.0,
      "OrbClaimWindow": 0.06,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.3,
      "OrbSpawnOffsetRandomXYZ": 64.0,
      "OrbSpawnOffsetZ": 30,
      "OscillateFrequency": 0.01,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "Radius": 2.0,
      "UpSpeedMax": 25.0,
      "UpSpeedMin": 25.0,
      "UseKillerPlaneOffsets": true
    },
    "xp_orb_spawner": {
      "BurstSpeedDuration": 0.2,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.2,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneHorizontalDecayRate": 15.0,
      "KillerPlaneHorizontalSpeedX": 40.0,
      "KillerPlaneHorizontalSpeedY": 55.0,
      "KillerPlaneLaunchDelay": 0.1,
      "KillerPlaneLaunchOffset": 15.0,
      "KillerPlaneOffset": 15.0,
      "KillerPlaneSpeedNoise": 10.0,
      "KillerPlaneVerticalSpeed": 115.0,
      "LateralMoveDuration": 0.3,
      "LateralSpeedMax": 40.0,
      "LateralSpeedMin": 40.0,
      "LifeTime": 0.7,
      "OrbClaimWindow": 0.06,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.7,
      "OrbSpawnDelayMin": 0.4,
      "OrbSpawnOffsetRandomXYZ": 64.0,
      "OrbSpawnOffsetZ": 30,
      "OscillateFrequency": 0.01,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "Radius": 1.2,
      "UpSpeedMax": 65.0,
      "UpSpeedMin": 65.0,
      "UseKillerPlaneOffsets": true
    },
    "xp_orb_trooper": {
      "BurstSpeedDuration": 0.2,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.2,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneHorizontalDecayRate": 15.0,
      "KillerPlaneHorizontalSpeedX": 40.0,
      "KillerPlaneHorizontalSpeedY": 55.0,
      "KillerPlaneLaunchDelay": 0.1,
      "KillerPlaneLaunchOffset": 15.0,
      "KillerPlaneOffset": 15.0,
      "KillerPlaneSpeedNoise": 10.0,
      "KillerPlaneVerticalSpeed": 115.0,
      "LateralMoveDuration": 0.3,
      "LateralSpeedMax": 40.0,
      "LateralSpeedMin": 40.0,
      "LifeTime": 0.7,
      "OrbClaimWindow": 0.06,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.7,
      "OrbSpawnDelayMin": 0.4,
      "OrbSpawnOffsetRandomXYZ": 64.0,
      "OrbSpawnOffsetZ": 30,
      "OscillateFrequency": 0.01,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "Radius": 1.2,
      "UpSpeedMax": 65.0,
      "UpSpeedMin": 65.0,
      "UseKillerPlaneOffsets": true
    },
    "xp_orb_vault": {
      "BurstSpeedDuration": 0.1,
      "BurstSpeedMultiplier": 1.5,
      "CollisionRadius": 12.0,
      "DownSpeed": 20.0,
      "EnemyGlowParticle": "particles/generic/spirit_orb_ambient_enemy.vpcf",
      "EnemyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit_enemy.vpcf",
      "EnemyOrbDeniedParticle": "particles/generic/spirit_xp_denied.vpcf",
      "EnemyOrbEarnedParticle": "particles/generic/spirit_xp_earned_enemy.vpcf",
      "FriendlyGlowParticle": "particles/generic/spirit_orb_ambient.vpcf",
      "FriendlyHitConfirmParticle": "particles/generic/spirit_orb_ambient_hit.vpcf",
      "FriendlyOrbDeniedParticle": "particles/generic/spirit_xp_denied_enemy.vpcf",
      "FriendlyOrbEarnedParticle": "particles/generic/spirit_xp_earned.vpcf",
      "GoldReceivedParticle": "",
      "GravityScale": 0.0,
      "InvulDuration": 0.12,
      "InvulDurationMax": 0.65,
      "InvulDurationMin": 0.35,
      "KillerPlaneOffset": 0,
      "LateralSpeedMax": 0.0,
      "LateralSpeedMin": 0.0,
      "LifeTime": 0.3,
      "OrbClaimWindow": 0,
      "OrbClaimed": "Player.ClaimOrb",
      "OrbClaimedTeammate": "Teammate.ClaimOrb",
      "OrbDenied": "Player.Deny",
      "OrbDeniedPlayer": "Player.Denied",
      "OrbHitConfirm": "Player.ClaimOrb.Hit",
      "OrbHitPredicted": "Player.ClaimOrb.Hit",
      "OrbModel": "",
      "OrbSpawnDelayMax": 0.3,
      "OrbSpawnDelayMin": 0.3,
      "OrbSpawnOffsetRandomXYZ": 0,
      "OrbSpawnOffsetZ": 24,
      "OscillateFrequency": 1.0,
      "PredictedHitLimboGlowParticle": "particles/generic/spirit_orb_processing.vpcf",
      "UpSpeedMax": 105.0,
      "UpSpeedMin": 85.0,
      "UseKillerPlaneOffsets": false
    }
  }
}
````

