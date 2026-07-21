---
title: "json generic data"
entries: 1
---

# json generic data

<!-- game-wiki-entry source="deadlock_data" entity_type="json_generic_data" external_id="json_generic_data" title="json_generic_data" -->

## json_generic_data

### Kurzueberblick

- Typ: `json_generic_data`
- Quelle: `deadlock_data`
- External ID: `json_generic_data`
- Snapshot ID: `40572`
- Source-Dokument: `7076`
- Kurzinfo: json_generic_data aus `deadlock_data` / `json_generic_data` mit vollstaendiger Payload.

### Provenienz

- Canonical Name: `json_generic_data`
- Payload Hash: `abd45475a6102b708e4ab0a25e88e83e29fb8d3761711e1d01eaa9e35c83f515`
- Source Content Hash: `341a82864c036bfd5d93c15079951fd34a6ddff323d4cc2a810b2c10e776cdec`
- Source URL: `https://github.com/deadlock-wiki/deadlock-data/blob/e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06/data/json/generic-data.json`
- Source Raw Path: `data/raw/deadlock_data/data_json_generic-data.json.341a82864c036bfd.json`
- Fetched At: `2026-07-09T19:36:25.777166+00:00`

### Vollstaendige Payload

````json
{
  "_deadlock_data": {
    "commit_sha": "e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06",
    "commit_time": "2026-07-08T15:57:33+00:00",
    "file_path": "data/json/generic-data.json",
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
    "AimSpringStrength": [
      0.0,
      2.0,
      10.0,
      300.0
    ],
    "CurrencyTypeSounds": {
      "AbilityPoints": {
        "SourceSounds": {
          "BossKill": "Player.AcquireAp",
          "Cheats": "Player.AcquireAp",
          "LevelUp": "Player.AcquireAp"
        }
      },
      "Gold": {
        "SourceSounds": {
          "ItemPurchase": "Player.PurchaseUpgrade",
          "ItemSale": "",
          "ItemUpgrade": "Player.PurchaseUpgrade",
          "NeutralWorldPickup": "Neutral.Bounty.Pickup",
          "OrbBaseSentry": "Player.ClaimOrb",
          "OrbDeny": "Player.ClaimOrb",
          "OrbDeployable": "Player.ClaimOrb",
          "OrbLaneTrooper": "Player.ClaimOrb",
          "OrbNeutralTrooper": "Player.ClaimOrb",
          "OrbPlayer": "Player.ClaimOrb",
          "OrbTier1TrooperBoss": "Player.ClaimOrb",
          "OrbTier2TrooperBoss": "Player.ClaimOrb",
          "OrbTier3TrooperBoss": "Player.ClaimOrb",
          "OrbTreasureChest": "Player.ClaimOrb",
          "OrbTrophyKill": "Player.ClaimOrb",
          "TreasureChest": ""
        }
      }
    },
    "DamageFlash": {
      "FlashType_BulletDamage": {
        "Brightness": 0.2,
        "BrightnessInLightSensitivityMode": 0.0,
        "Color": [
          255,
          128,
          0
        ],
        "Coverage": 1.0,
        "Duration": 0.1,
        "Hardness": 1.0
      },
      "FlashType_CritDamage": {
        "Brightness": 0.5,
        "Color": [
          255,
          255,
          0
        ],
        "Coverage": 0.8,
        "Duration": 0.33,
        "Hardness": 0.9,
        "m_bHeadOnly": true
      },
      "FlashType_Healing": {
        "Brightness": 1.2,
        "BrightnessInLightSensitivityMode": 0.6,
        "Color": [
          0,
          195,
          45
        ],
        "Coverage": 0.5,
        "Duration": 1.0,
        "Hardness": 0.0
      },
      "FlashType_MeleeActivate": {
        "Brightness": 0.8,
        "Color": [
          49,
          169,
          250
        ],
        "Coverage": 1.0,
        "Duration": 0.1,
        "Hardness": 0.0
      },
      "FlashType_PatronHit": {
        "Brightness": 0.3,
        "Color": [
          255,
          134,
          5
        ],
        "Coverage": 0.99,
        "Duration": 0.05,
        "Hardness": 1.0
      },
      "FlashType_TechDamage": {
        "Brightness": 0.2,
        "Color": [
          165,
          51,
          255
        ],
        "Coverage": 1.0,
        "Duration": 0.1,
        "Hardness": 1.0
      }
    },
    "DamageFlashLowViolence": {
      "FlashType_BulletDamage": {
        "Brightness": 3.5,
        "Color": [
          12,
          0,
          15
        ],
        "Coverage": 0.5,
        "Duration": 0.25,
        "Hardness": 0.7
      }
    },
    "DamageIndicatorSounds": {
      "DamageCrit": "Damage.Send.Crit",
      "DamageDOT": "Damage.Send.DOT",
      "DamageDefault": "Damage.Send",
      "DamageHealthTransfer": "Player.Heal.SiphonTick",
      "DamageInvulnerable": "Damage.Send.Invulnerable",
      "DamageLethal": "Damage.Send.Lethal"
    },
    "DamageReceivedSounds": {
      "DamageReceiveDOT": "Damage.Receive.DOT",
      "DamageReceiveDefault": "Damage.Receive.Default",
      "DamageReceiveMelee": "Damage.Receive.Melee",
      "DamageReceiveMeleeNPC": "Damage.Receive.Melee.NPC",
      "DamageReceiveNPC": "Damage.Receive.NPC",
      "DamageReceiveShield": "Damage.Receive.Shield"
    },
    "DefaultDOF": {
      "DofFarBlurry": 0.0,
      "DofFarCrisp": 0.0,
      "DofNearBlurry": 0.0,
      "DofNearCrisp": 0.0
    },
    "GlitchSettings": {
      "BreakupStrength": 0.0,
      "DistortStrength": 1.25,
      "FrameRate": 5.0,
      "JumpStrength": 1.0,
      "QuantizeScale": 0.5,
      "QuantizeStrength": 0.2,
      "QuantizeType": 2.0,
      "ScanlineStrength": 1.5,
      "Speed": 10.0,
      "Strength": 1.0,
      "WhiteNoiseStrength": 3.0
    },
    "HealingReceivedSounds": {
      "DirectHealingMedium": "Player.Heal.Burst.Medium",
      "DirectHealingSmall": "Player.Heal.Burst.Small",
      "HOTLoopSounds": {
        "CITADEL_AUDIO_LOOP_LOOP_SOUND": "Player.Heal.OverTime.Lp",
        "CITADEL_AUDIO_LOOP_START_SOUND": "Player.Heal.OverTime.Start",
        "CITADEL_AUDIO_LOOP_STOP_SOUND": "Player.Heal.OverTime.Stop"
      },
      "HOTToppedOff": "Player.Heal.OverTime.TopOff",
      "Priority": 1
    },
    "HeroKillGoldShareFrac": [
      1.25,
      0.575,
      0.283,
      0.175,
      0.11,
      0.083
    ],
    "HeroTestingTargetDummyUpgrades": [
      "upgrade_toughness_2",
      "upgrade_health_regen_1",
      "upgrade_toughness_3",
      "upgrade_health_percent_large",
      "upgrade_out_of_combat_health_regen",
      "upgrade_zipine_mastery"
    ],
    "IdolParams": {
      "CrateModel": "models/props_gameplay/gold_crate.vmdl",
      "IdolDropDuration": 12.5,
      "IdolDropHeight": 1400.0,
      "IdolDroppingParticle": "particles/environment/soul_jar_drop.vpcf",
      "IdolModel": "models/props_gameplay/idol_urn/idol_urn.vmdl",
      "IdolReturnLocationParticle": "particles/environment/soul_jar_return_location.vpcf",
      "IdolReturnLocationParticleScale": 2.0,
      "IdolSpawnCompleteSound": "Soul.Urn.Spawn.Complete",
      "IdolSpawnLocationParticle": "particles/environment/soul_jar_summon.vpcf",
      "IdolSpawnSound": "Soul.Urn.Spawn",
      "LoopingSequenceName": "golden_idol_idle",
      "ParachuteModel": "models/props_gameplay/gold_crate_parachute.vmdl"
    },
    "ItemPricePerTier": [
      0,
      800,
      1600,
      3200,
      6400,
      9999
    ],
    "KillStreakFireParticle": "particles/ui/ui_topbar_killstreak.vpcf",
    "KothParams": {
      "KothEarlyWarningParticle": "particles/generic/koth_indicator.vpcf",
      "KothOnSpawnParticle": "particles/generic/koth_comet_drop_endcap.vpcf",
      "KothPreSpawnLoopSound": "Koth.Pre.Spawn.Lp",
      "KothRadius": 20.0,
      "KothSpawnCompleteSound": "Koth.Spawn.Complete",
      "KothSpawnLocationParticle": "particles/generic/koth_rift_timer.vpcf",
      "KothSpawnLoopSound": "Koth.Spawn.Lp",
      "KothSpawnLoopStartSound": "Koth.Spawn.Start"
    },
    "LaneInfo": [
      {
        "CSSClass": "whiteLane",
        "Color": [
          200,
          200,
          200
        ],
        "LaneName": "White",
        "MinimapZiplineColorOverride": [
          200,
          200,
          200
        ],
        "ObjectiveColor": [
          200,
          200,
          200
        ]
      },
      {
        "CSSClass": "yellowLane",
        "Color": [
          241,
          204,
          48
        ],
        "LaneName": "Yellow",
        "MinimapZiplineColorOverride": [
          190,
          169,
          78
        ],
        "ObjectiveColor": [
          249,
          212,
          5
        ]
      },
      {
        "Color": [
          0,
          0,
          0
        ],
        "LaneName": "Yellow + Orange"
      },
      {
        "CSSClass": "greenLane",
        "Color": [
          255,
          20,
          147
        ],
        "LaneName": "Green",
        "MinimapZiplineColorOverride": [
          255,
          20,
          147
        ],
        "ObjectiveColor": [
          255,
          20,
          147
        ]
      },
      {
        "CSSClass": "blueLane",
        "Color": [
          41,
          177,
          204
        ],
        "LaneName": "Blue",
        "MinimapZiplineColorOverride": [
          85,
          155,
          190
        ],
        "ObjectiveColor": [
          67,
          125,
          220
        ]
      },
      {
        "Color": [
          0,
          0,
          0
        ],
        "LaneName": " + Purple"
      },
      {
        "CSSClass": "purpleLane",
        "Color": [
          89,
          178,
          71
        ],
        "LaneName": "Purple",
        "MinimapZiplineColorOverride": [
          102,
          153,
          92
        ],
        "ObjectiveColor": [
          89,
          178,
          71
        ]
      }
    ],
    "MidbossIndicatorRespawningParticle": "",
    "MidbossIndicatorSpawnedParticle": "",
    "MiniMapOffsets": [
      {
        "eEntityClass": "CLASS_BOSS_TIER_3",
        "iLane": 0,
        "vOffset2D": [
          0.0,
          1000.0
        ]
      },
      {
        "eEntityClass": "CLASS_BOSS_BARRACKS",
        "iLane": 1,
        "vOffset2D": [
          500.0,
          0.0
        ]
      },
      {
        "eEntityClass": "CLASS_BOSS_BARRACKS",
        "iLane": 6,
        "vOffset2D": [
          500.0,
          0.0
        ]
      },
      {
        "eEntityClass": "CLASS_DESTROYABLE_BUILDING",
        "vOffset2D": [
          0.0,
          400.0
        ]
      }
    ],
    "MinimapTeamCombineColor": [
      83,
      51,
      51
    ],
    "MinimapTeamRebelsColor": [
      58,
      91,
      66
    ],
    "MinimapZiplinesParticle": "particles/ui/ui_minimap.vpcf",
    "NewPlayerMetrics": [
      {
        "AbilitiesUpgraded": 1,
        "BossDamage": 300,
        "DamageTaken": 6600,
        "LastHits": 1,
        "ModsPurchased": 1,
        "NetWorth": 1000,
        "OrbsDenied": 1,
        "OrbsSecured": 1,
        "PlayerDamage": 900,
        "SkillTierName": "Bronze"
      },
      {
        "AbilitiesUpgraded": 1,
        "BossDamage": 400,
        "DamageTaken": 5800,
        "LastHits": 1,
        "ModsPurchased": 1,
        "NetWorth": 1000,
        "OrbsDenied": 1,
        "OrbsSecured": 1,
        "PlayerDamage": 1400,
        "SkillTierName": "Silver"
      },
      {
        "AbilitiesUpgraded": 1,
        "BossDamage": 600,
        "DamageTaken": 5000,
        "LastHits": 1,
        "ModsPurchased": 1,
        "NetWorth": 1000,
        "OrbsDenied": 1,
        "OrbsSecured": 1,
        "PlayerDamage": 1800,
        "SkillTierName": "Gold"
      },
      {
        "AbilitiesUpgraded": 1,
        "BossDamage": 700,
        "DamageTaken": 4600,
        "LastHits": 1,
        "ModsPurchased": 1,
        "NetWorth": 1000,
        "OrbsDenied": 1,
        "OrbsSecured": 1,
        "PlayerDamage": 2300,
        "SkillTierName": "Platinum"
      }
    ],
    "NoLaneZip": {
      "CSSClass": "noLane",
      "Color": [
        0,
        0,
        0
      ],
      "LaneName": "NoLane"
    },
    "ObjectiveParams": {
      "BaseGuardiansGoldKill": 1000,
      "BaseGuardiansGoldOrbs": 0,
      "GoldPerOrb": 0,
      "NearPlayerSplitPct": 40.0,
      "PatronPhase1GoldKill": 0,
      "PatronPhase1GoldOrbs": 0,
      "ShrinesGoldKill": 2000,
      "ShrinesGoldOrbs": 0,
      "Tier1GoldKill": 1250,
      "Tier1GoldOrbs": 0,
      "Tier2GoldKill": 3500,
      "Tier2GoldOrbs": 0
    },
    "OutlineColorEnemy": [
      230,
      25,
      25,
      255
    ],
    "OutlineColorEnemyHero": [
      162,
      34,
      34,
      255
    ],
    "OutlineColorFriend": [
      215,
      201,
      175,
      255
    ],
    "OutlineColorNeutral": [
      0,
      125,
      125,
      255
    ],
    "OutlineColorTeam1": [
      231,
      182,
      89,
      255
    ],
    "OutlineColorTeam2": [
      91,
      121,
      230,
      255
    ],
    "RejuvParams": {
      "PlayerRespawnMult": [
        0.5,
        0.4,
        0.3
      ],
      "RejuvPickupSound": "Rejuv.Pickup",
      "RejuvinatorBuffDuration": 180,
      "RejuvinatorDropDuration": 6,
      "RejuvinatorDropHeight": 500,
      "RejuvinatorExpirationWarningTiming": 30,
      "RejuvinatorRebirthDuration": [
        180,
        180,
        180
      ],
      "TrooperHealthMult": [
        1.7,
        2.0,
        2.3
      ]
    },
    "ResourceTypes": {
      "ResourceType_Rage": {
        "CantCastOutOfResourceToken": "#CITADEL_ABILITY_INVALID_NOT_RAGE",
        "HUDSnippetName": "rage"
      }
    },
    "ShoppingEffect": "particles/generic/hero_shopping_status.vpcf",
    "StatTypeImages": {
      "BaseHealthRegen": "file://{images}/upgrades/mods_armor/health_regen.psd",
      "BulletArmorDamageReduction": "file://{images}/upgrades/mods_armor/bullet_armor.psd",
      "TechArmorDamageReduction": "file://{images}/upgrades/mods_armor/tech_armor.psd"
    },
    "StreetBrawl": {
      "BuyTimeGracePeriod": 60.0,
      "ComebackBonusHealth": 1200,
      "ComebackBonusHealthCritical": 2800,
      "ItemTierToItemDraftBuckets": {
        "ModTier_1": {
          "BucketName": "Tier 1",
          "Buckets": {
            "Good": 60.0,
            "Normal": 40.0
          }
        },
        "ModTier_2": {
          "BucketName": "Tier 2",
          "Buckets": {
            "Good": 40.0,
            "Normal": 60.0
          }
        },
        "ModTier_3": {
          "BucketName": "Tier 3",
          "Buckets": {
            "Good": 40.0,
            "Normal": 60.0
          }
        },
        "ModTier_4": {
          "BucketName": "Tier 4",
          "Buckets": {
            "Good": 35.0,
            "Normal": 65.0
          }
        },
        "ModTier_5": {
          "BucketName": "Tier 5",
          "Buckets": {
            "Good": 20.0,
            "Normal": 80.0
          }
        }
      },
      "OvertimeRespawnTimeIncrease": [
        25,
        25,
        25,
        25,
        30
      ],
      "OvertimeRespawnTimeIncreaseUrgent": [
        40,
        40,
        40,
        40,
        45
      ],
      "OvertimeTrooperDamageScale": [
        1.44,
        1.44,
        1.44,
        1.44,
        1.56
      ],
      "OvertimeTrooperHealthScale": [
        1.75,
        1.75,
        1.75,
        2.0,
        2.5
      ],
      "ScoringTime": 6,
      "Tier1MaxResistTime": 2.2,
      "Tier2BonusHealth": 4000,
      "Tier2MaxResistTime": 2.2,
      "TrooperModifier": "modifier_streetbrawl_trooper",
      "TrooperSpawnBeforeRoundStartTimer": 5.0,
      "TrooperSpawnTimer": [
        18,
        18,
        15,
        15,
        15
      ],
      "ZipBoostCooldownOnStart": 20.0,
      "m_iLaneNumber": 4,
      "m_iScoreToWin": 3,
      "m_iUltimateUnlockRound": 1,
      "m_vecAPPerRound": [
        6,
        6,
        5,
        5,
        10
      ],
      "m_vecBuyTime": [
        50,
        50,
        50,
        50,
        50
      ],
      "m_vecGoldPerRound": [
        5600,
        7400,
        9400,
        11600,
        14000
      ],
      "m_vecItemDraftRerollsPerRound": [
        1,
        1,
        1,
        1,
        1
      ],
      "m_vecItemDraftRoundsPerGameRound": [
        {
          "m_chanceEnhanced": {
            "OutcomesToWeights": {
              "0": 10.0,
              "1": 20.0,
              "2": 20.0,
              "3": 35.0,
              "4": 15.0,
              "5": 0.0,
              "6": 0.0
            }
          },
          "m_chanceRare": {
            "OutcomesToWeights": {
              "0": 0.0,
              "1": 0.0,
              "2": 10.0,
              "3": 60.0,
              "4": 40.0,
              "5": 20.0,
              "6": 0.0
            }
          },
          "m_vecItemDraftRounds": [
            {
              "m_eNormalModTier": "EModTier_1",
              "m_eRareModTier": "EModTier_2"
            },
            {
              "m_eNormalModTier": "EModTier_2",
              "m_eRareModTier": "EModTier_3"
            },
            {
              "m_eNormalModTier": "EModTier_3",
              "m_eRareModTier": "EModTier_4"
            }
          ]
        },
        {
          "m_chanceEnhanced": {
            "OutcomesToWeights": {
              "0": 10.0,
              "1": 20.0,
              "2": 20.0,
              "3": 35.0,
              "4": 15.0,
              "5": 0.0,
              "6": 0.0
            }
          },
          "m_chanceRare": {
            "OutcomesToWeights": {
              "0": 0.0,
              "1": 0.0,
              "2": 10.0,
              "3": 60.0,
              "4": 40.0,
              "5": 20.0,
              "6": 0.0
            }
          },
          "m_vecItemDraftRounds": [
            {
              "m_eNormalModTier": "EModTier_2",
              "m_eRareModTier": "EModTier_3"
            },
            {
              "m_eNormalModTier": "EModTier_3",
              "m_eRareModTier": "EModTier_4"
            },
            {
              "m_eNormalModTier": "EModTier_3",
              "m_eRareModTier": "EModTier_4"
            }
          ]
        },
        {
          "m_chanceEnhanced": {
            "OutcomesToWeights": {
              "0": 10.0,
              "1": 20.0,
              "2": 20.0,
              "3": 35.0,
              "4": 15.0,
              "5": 0.0,
              "6": 0.0
            }
          },
          "m_chanceRare": {
            "OutcomesToWeights": {
              "0": 0.0,
              "1": 0.0,
              "2": 10.0,
              "3": 60.0,
              "4": 40.0,
              "5": 20.0,
              "6": 0.0
            }
          },
          "m_vecItemDraftRounds": [
            {
              "RareWeight": 50,
              "m_eNormalModTier": "EModTier_2",
              "m_eRareModTier": "EModTier_3"
            },
            {
              "RareWeight": 50,
              "m_eNormalModTier": "EModTier_3",
              "m_eRareModTier": "EModTier_4"
            },
            {
              "RareWeight": 0,
              "m_eNormalModTier": "EModTier_4",
              "m_eRareModTier": "EModTier_5"
            }
          ]
        },
        {
          "m_chanceEnhanced": {
            "OutcomesToWeights": {
              "0": 10.0,
              "1": 20.0,
              "2": 20.0,
              "3": 35.0,
              "4": 15.0,
              "5": 0.0,
              "6": 0.0
            }
          },
          "m_chanceRare": {
            "OutcomesToWeights": {
              "0": 0.0,
              "1": 0.0,
              "2": 10.0,
              "3": 60.0,
              "4": 40.0,
              "5": 20.0,
              "6": 0.0
            }
          },
          "m_vecItemDraftRounds": [
            {
              "RareWeight": 40,
              "m_eNormalModTier": "EModTier_3",
              "m_eRareModTier": "EModTier_4"
            },
            {
              "RareWeight": 40,
              "m_eNormalModTier": "EModTier_3",
              "m_eRareModTier": "EModTier_4"
            },
            {
              "RareWeight": 20,
              "m_eNormalModTier": "EModTier_4",
              "m_eRareModTier": "EModTier_5"
            }
          ]
        },
        {
          "m_chanceEnhanced": {
            "OutcomesToWeights": {
              "0": 0.0,
              "1": 0.0,
              "2": 5.0,
              "3": 35.0,
              "4": 15.0,
              "5": 0.0,
              "6": 0.0
            }
          },
          "m_chanceRare": {
            "OutcomesToWeights": {
              "0": 5.0,
              "1": 10.0,
              "2": 40.0,
              "3": 10.0,
              "4": 0.0,
              "5": 0.0
            }
          },
          "m_vecItemDraftRounds": [
            {
              "m_eNormalModTier": "EModTier_4",
              "m_eRareModTier": "EModTier_5"
            },
            {
              "m_eNormalModTier": "EModTier_4",
              "m_eRareModTier": "EModTier_5"
            },
            {
              "m_eNormalModTier": "EModTier_4",
              "m_eRareModTier": "EModTier_5"
            }
          ]
        }
      ],
      "m_vecObjectiveMaxHealth": [
        1800,
        3200,
        4600,
        6000,
        7400
      ],
      "m_vecPreBuyTime": [
        6.0,
        0.0,
        0.0,
        0.0,
        0.0
      ],
      "m_vecRespawnTimes": [
        25,
        25,
        30,
        30,
        30
      ],
      "m_vecRoundLengthMinutes": [
        3,
        3,
        3,
        3,
        3
      ],
      "m_vecRoundLengthMinutesUrgent": [
        4.5,
        4.5,
        4.5,
        4.5,
        4.5
      ]
    },
    "TargetingSpringStrength": [
      0.0,
      2.0,
      10.0,
      30.0
    ],
    "TeleporterParams": {
      "EndEffect": "particles/environment/teleporter_pad_end.vpcf",
      "EnterSound": "Teleport.Enter",
      "StartEffect": "particles/environment/teleporter_pad_start.vpcf",
      "StartSound": "Teleport.Buildup",
      "TeleportedSound": "Teleport.Appear"
    },
    "TrooperKillGoldShareFrac": [
      1.0,
      0.54,
      0.36,
      0.25,
      0.2,
      0.16
    ],
    "generic_data_type": "CitadelGenericData_t",
    "m_enemyObjectivesColor": [
      220,
      76,
      47
    ],
    "m_enemyZiplineColor": [
      140,
      55,
      38
    ],
    "m_vecArmorGroups": [
      {
        "m_eShopGroup": "EMoreHealth",
        "m_vecUpgrades": [
          "upgrade_health",
          "upgrade_health_2",
          "upgrade_chonky",
          "upgrade_colossus",
          "upgrade_unstoppable"
        ]
      },
      {
        "m_eShopGroup": "EHealing",
        "m_vecUpgrades": [
          "upgrade_medic_bullets",
          "upgrade_health_stimpak",
          "upgrade_health_nova",
          "upgrade_rescue_beam"
        ]
      },
      {
        "m_eShopGroup": "ERevitalization",
        "m_vecUpgrades": [
          "upgrade_endurance",
          "upgrade_restorative_locket",
          "upgrade_vex_barrier",
          "upgrade_healing_booster",
          "upgrade_healbuff"
        ]
      },
      {
        "m_eShopGroup": "EDebuffs",
        "m_vecUpgrades": [
          "upgrade_debuff_reducer",
          "upgrade_reduce_debuff_duration",
          "upgrade_inhibitor"
        ]
      },
      {
        "m_eShopGroup": "ESpiritProtection",
        "m_vecUpgrades": [
          "upgrade_tech_armor",
          "upgrade_magic_shield",
          "upgrade_tech_purge",
          "upgrade_cheat_death",
          "upgrade_absorbing_armor",
          "upgrade_phantom_strike",
          "upgrade_spellbreaker"
        ]
      },
      {
        "m_eShopGroup": "EBulletProtection",
        "m_vecUpgrades": [
          "upgrade_return_fire",
          "upgrade_regenerating_bullet_shield",
          "upgrade_bullet_armor",
          "upgrade_improved_bullet_armor",
          "upgrade_deflecting_armor"
        ]
      },
      {
        "m_eShopGroup": "ELifesteal",
        "m_vecUpgrades": [
          "upgrade_vampire",
          "upgrade_surging_power",
          "upgrade_health_stealing_magic",
          "upgrade_infuser",
          "upgrade_damage_recycler"
        ]
      },
      {
        "m_eShopGroup": "EMelee",
        "m_vecUpgrades": [
          "upgrade_lifestrike_gauntlets",
          "upgrade_melee_rebuttal",
          "upgrade_boxing_glove"
        ]
      },
      {
        "m_eShopGroup": "EMovementAndAgility",
        "m_vecUpgrades": [
          "upgrade_improved_stamina",
          "upgrade_sprint_booster",
          "upgrade_cardio_calibrator",
          "upgrade_superior_stamina",
          "upgrade_rocket_booster",
          "upgrade_juggernaut"
        ]
      },
      {
        "m_eShopGroup": "EAntiHealing",
        "m_vecUpgrades": [
          "upgrade_healbane"
        ]
      },
      {
        "m_eShopGroup": "EAdditionalProtection",
        "m_vecUpgrades": [
          "upgrade_weapon_shielding",
          "upgrade_spirit_bubble",
          "upgrade_savior",
          "upgrade_metal_skin",
          "upgrade_veil_walker",
          "upgrade_siphon_bullets",
          "upgrade_diviners_kevlar"
        ]
      }
    ],
    "m_vecSpiritGroups": [
      {
        "m_eShopGroup": "EMoreSpirit",
        "m_vecUpgrades": [
          "upgrade_improved_spirit",
          "upgrade_magic_storm",
          "upgrade_soaring_spirit",
          "upgrade_boundless_spirit"
        ]
      },
      {
        "m_eShopGroup": "EBurstDamage",
        "m_vecUpgrades": [
          "upgrade_magic_burst",
          "upgrade_quick_silver",
          "upgrade_magic_shock",
          "upgrade_ultimate_burst"
        ]
      },
      {
        "m_eShopGroup": "ESpiritRange",
        "m_vecUpgrades": [
          "upgrade_magic_reach",
          "upgrade_arcane_surge",
          "upgrade_tech_range"
        ]
      },
      {
        "m_eShopGroup": "EDuration",
        "m_vecUpgrades": [
          "upgrade_arcane_extension",
          "upgrade_imbued_duration_extender",
          "upgrade_magic_carpet",
          "upgrade_spirit_burn"
        ]
      },
      {
        "m_eShopGroup": "ECharges",
        "m_vecUpgrades": [
          "upgrade_extra_charge",
          "upgrade_rapid_recharge"
        ]
      },
      {
        "m_eShopGroup": "ECooldown",
        "m_vecUpgrades": [
          "upgrade_magic_tempo",
          "upgrade_cooldown_reduction",
          "upgrade_ability_power_shard"
        ]
      },
      {
        "m_eShopGroup": "EMeleeSpirit",
        "m_vecUpgrades": [
          "upgrade_acolytes_glove",
          "upgrade_spirit_snatch"
        ]
      },
      {
        "m_eShopGroup": "ESlowingMagic",
        "m_vecUpgrades": [
          "upgrade_suppressor",
          "upgrade_magic_slow",
          "upgrade_mystic_reverb"
        ]
      },
      {
        "m_eShopGroup": "EMagicVulnerability",
        "m_vecUpgrades": [
          "upgrade_magic_vulnerability",
          "upgrade_escalating_exposure"
        ]
      },
      {
        "m_eShopGroup": "ECrownControl",
        "m_vecUpgrades": [
          "upgrade_containment",
          "upgrade_target_stun",
          "upgrade_aoe_root"
        ]
      },
      {
        "m_eShopGroup": "EAntiGun",
        "m_vecUpgrades": [
          "upgrade_withering_whip",
          "upgrade_greater_withering_whip"
        ]
      },
      {
        "m_eShopGroup": "EMiscSpirit",
        "m_vecUpgrades": [
          "upgrade_mystic_regeneration",
          "upgrade_bullet_resist_shredder",
          "upgrade_tech_damage_pulse"
        ]
      },
      {
        "m_eShopGroup": "EActives",
        "m_vecUpgrades": [
          "upgrade_spirit_sap",
          "upgrade_rupture",
          "upgrade_cold_front",
          "upgrade_self_bubble",
          "upgrade_targeted_silence",
          "upgrade_glitch",
          "upgrade_discord",
          "upgrade_focus_lens",
          "upgrade_ability_refresher"
        ]
      }
    ],
    "m_vecWeaponGroups": [
      {
        "m_eShopGroup": "EMagazines",
        "m_vecUpgrades": [
          "upgrade_clip_size",
          "upgrade_active_reload",
          "upgrade_intensifying_clip",
          "upgrade_titan_round",
          "upgrade_infinitemagazine",
          "upgrade_reinforcing_casings"
        ]
      },
      {
        "m_eShopGroup": "ERateOfFire",
        "m_vecUpgrades": [
          "upgrade_rapid_rounds",
          "upgrade_blitz_bullets",
          "upgrade_burst_fire",
          "upgrade_ricochet"
        ]
      },
      {
        "m_eShopGroup": "EBulletVelocity",
        "m_vecUpgrades": [
          "upgrade_weighted_shots",
          "upgrade_high_velocity_mag",
          "upgrade_long_range",
          "upgrade_split_shot",
          "upgrade_pristine_emblem",
          "upgrade_sharpshooter",
          "upgrade_aprounds"
        ]
      },
      {
        "m_eShopGroup": "EHeadshotEnthusiasts",
        "m_vecUpgrades": [
          "upgrade_headshot_booster",
          "upgrade_headshot_booster2",
          "upgrade_headhunter",
          "upgrade_banshee_slugs"
        ]
      },
      {
        "m_eShopGroup": "EWeaponRange",
        "m_vecUpgrades": [
          "upgrade_close_range",
          "upgrade_close_quarter_combat",
          "upgrade_bullet_armor_reduction_aura",
          "upgrade_cloaking_device_active",
          "upgrade_proc_silence"
        ]
      },
      {
        "m_eShopGroup": "ETankingBullets",
        "m_vecUpgrades": [
          "upgrade_melee_charge",
          "upgrade_berserker",
          "upgrade_fervor"
        ]
      },
      {
        "m_eShopGroup": "ESpiritSlinger",
        "m_vecUpgrades": [
          "upgrade_crackshot",
          "upgrade_tech_defense_shredders",
          "upgrade_spellslinger_headshots",
          "upgrade_chain_lightning",
          "upgrade_tech_overflow"
        ]
      },
      {
        "m_eShopGroup": "EActives",
        "m_vecUpgrades": [
          "upgrade_fleetfoot_boots",
          "upgrade_thermal_detonator",
          "upgrade_dps_aura",
          "upgrade_warp_stone"
        ]
      },
      {
        "m_eShopGroup": "EMisc",
        "m_vecUpgrades": [
          "upgrade_hollow_point_rounds",
          "upgrade_non_player_bonus",
          "upgrade_kinetic_sash",
          "upgrade_toxic_bullets",
          "upgrade_critshot"
        ]
      },
      {
        "m_eShopGroup": "ESlowingBullets",
        "m_vecUpgrades": [
          "upgrade_slowing_bullets",
          "upgrade_glass_cannon"
        ]
      }
    ]
  }
}
````

