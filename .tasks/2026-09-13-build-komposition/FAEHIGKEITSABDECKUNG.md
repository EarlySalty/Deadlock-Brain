# Fähigkeitsabdeckung auf FROZEN-V2

Rohfelder sind Prüfkandidaten, keine ungeprüft addierbaren Effekte. Prozentwerte, Schwellen und Hilfsfelder bleiben getrennt im JSON sichtbar. `base_effect` und Skalierungen stammen aus der alten Baseline, Rohwerte aus demselben eingefrorenen Snapshot. Ein Nullwert ist bei reinen Kontroll-/Tauschfähigkeiten nicht automatisch falsch. ID 0 darf Fähigkeiten nicht aus dem Kampf entfernen; die ID bleibt für Imbue ungültig.

| Held | Slot / Fähigkeit | ID | Basis / Typ | Skalierungen | Positive rohe Schadensfelder | Befund |
|---|---|---:|---|---|---|---|
| Infernus | 3 / ability_afterburn | 1593133799 | 0 / Spirit | DPS:0.66 | BurnDuration=3, BurnDurationBase=3, DPS=14 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Infernus | 4 / ability_fire_bomb | 1142270357 | 125 / Spirit | Damage:0.974938 | Damage=125 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Infernus | 1 / ability_incendiary_projectile | 0 | 40 / Spirit |  | Damage=40, IncomingDamagePercentFromCaster=16 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Infernus | 2 / ability_flame_dash | 0 | 90 / Spirit |  | DPS=30 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Seven | 1 / citadel_ability_lightning_ball | 1065103387 | 0 / Spirit | DPS:0.5 | DPS=75 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Seven | 3 / ability_power_surge | 539192269 | 0 / Spirit | BonusPerChain:0.32, DamagePerChain:0.14 | DamagePerChain=10 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Seven | 4 / citadel_ability_storm_cloud | 2061574352 | 665 / Spirit | DPS:0.6, LightningStrikeDamage:0.5 | DPS=95, DamageInterval=0.3, LightningStrikeDamage=75 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Seven | 2 / citadel_ability_static_charge | 0 | 35 / Spirit |  | Damage=35 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Vindicta | 1 / citadel_ability_hornet_chain | 537527508 | 40 / Spirit | Damage:0.5 | Damage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Vindicta | 3 / citadel_ability_hornet_sting | 2048438176 | 40 / Spirit | ImpactDamage:0.744 | DotHealthPercent=2.2, ImpactDamage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Vindicta | 4 / citadel_ability_hornet_snipe | 775377419 | 90 / Spirit | Damage:0.93, LowHealthEnemyDamageBonus:2.3 | Damage=90, HeadshotBonus=20, LowHealthEnemyDamageBonus=90, MinChargeDamagePercent=50, WeaponDamageBonusPerKill=6 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Vindicta | 2 / citadel_ability_hornet_leap | 0 | 0 / Spirit |  | MagicDamagePerBullet=10 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie; Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Lady Geist | 2 / ability_life_drain | 218284420 | 0 / Spirit | LifeDrainPerSecond:0.3225 | LifeDrainHealthMult=100, LifeDrainPerSecond=24 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Lady Geist | 3 / ability_blood_shards | 3230530872 | 0 / Spirit | HealthToDamage:0.558 | HealthToDamage=23, SelfDamagePct=9, VulnerabilityPerStack=7 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Lady Geist | 4 / ability_health_swap | 1307289689 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Lady Geist | 1 / ability_blood_bomb | 0 | 90 / Spirit |  | Damage=90, SelfDamagePct=30 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Abrams | 1 / citadel_ability_bull_heal | 4072270083 | 88 / Spirit | DPS:0.6 | DPS=22 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Abrams | 2 / citadel_ability_bull_charge | 2824119765 | 30 / Spirit | Damage:1.4 | Damage=30 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Abrams | 3 / citadel_ability_passive_beefy | 715762406 | 0 / Spirit |  | RegenDamageInterval=1, RegenIncomingDamageDuration=20, RegenIncomingDamagePercent=13 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Abrams | 4 / citadel_ability_bull_leap | 509856396 | 100 / Spirit | Damage:2.325 | Damage=100 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Wraith | 1 / citadel_ability_card_toss | 1999680326 | 45 / Spirit | Damage:0.55 | Damage=45, SpadeDamageBonus=60 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Wraith | 3 / citadel_ability_wraith_rapidfire | 1842576017 | 0 / Spirit | MagicDamagePerBullet:0.03 | MagicDamagePerBullet=2 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Wraith | 4 / citadel_ability_psychic_lift | 2981692841 | 100 / Spirit | Damage:1.0 | Damage=100 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Wraith | 2 / citadel_ability_projectmind | 0 | 0 / Spirit |  |  | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| McGinnis | 1 / citadel_ability_shieldedsentry | 3133377790 | 0 / Spirit | TurretDPS:0.42 | BossDamagePercentIncoming=50, BossDamagePercentOutgoing=30, NonHeroDamagePercentOutgoing=50, TurretDPS=24 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| McGinnis | 2 / citadel_ability_mobile_resupply | 2142734020 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| McGinnis | 4 / citadel_ability_rocket_barrage | 3503044146 | 0 / Spirit | DamagePerRocket:0.2 | DamagePerRocket=21 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| McGinnis | 3 / citadel_ability_fissure_wall | 0 | 60 / Spirit |  | Damage=60 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Paradox | 1 / citadel_ability_chrono_pulse_grenade | 58655583 | 0 / Spirit | PulseDamage:0.3 | DamageAmplificationPerStack=4, PulseDamage=35 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Paradox | 2 / citadel_ability_chrono_time_wall | 1366719170 | 0 / Spirit |  | FriendlyBulletDamageBonus=30 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Paradox | 3 / citadel_ability_chrono_kinetic_carbine | 1128670012 | 0 / Spirit | SpeedChange:0.13 | HeadshotBonus=14, MaxBonusBulletDamage=5, MinBonusBulletDamage=5 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Paradox | 4 / citadel_ability_chrono_swap | 2917891787 | 0 / Spirit | CombatBarrier:0.0, SwapDamage:1.1 | SwapDamage=150 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Dynamo | 1 / citadel_ability_stomp | 3760705623 | 115 / Spirit | Damage:1.55 | Damage=115 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Dynamo | 2 / citadel_ability_void_sphere | 2031714424 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Dynamo | 3 / citadel_ability_nikuman | 492030745 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Dynamo | 4 / citadel_ability_self_vacuum | 249410288 | 206.25 / Spirit | DPS:0.28 | DPS=75 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Kelvin | 1 / ability_ice_grenade | 18921423 | 60 / Spirit | Damage:0.6 | Damage=60 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Kelvin | 2 / ability_icepath | 1963397252 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Kelvin | 3 / ability_icebeam | 2351041382 | 225 / Spirit | DPS:0.38 | DPS=45 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Kelvin | 4 / ability_ice_dome | 3826390464 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Haze | 1 / ability_sleep_dagger | 2948410412 | 65 / Spirit | Damage:2.2, SleepWakeUpDelay:0.002 | Damage=65 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Haze | 2 / ability_smoke_bomb | 2414191464 | 0 / Spirit | AbilityDuration:0.1 | RevealOnDamageDuration=1.5 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Haze | 3 / ability_stacking_damage | 1080948381 | 0 / Spirit | ProcDamage:0.0 | DamageBonusFixedPerStack=0.2 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Haze | 4 / ability_bullet_flurry | 731943444 | 0 / Spirit | AbilityDuration:0.03 | WeaponDamageBonus=7 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Holliday | 1 / ability_explosive_barrel | 1235098866 | 40 / Spirit | DPS:0.175, ImpactDamage:0.525 | BurnDuration=3, DPS=13.3333, ImpactDamage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Holliday | 2 / ability_bounce_pad | 2240607294 | 0 / Spirit | StompDamage:0.372 | StompDamage=60 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Holliday | 4 / ability_gravity_lasso | 3190606822 | 80 / Spirit | Damage:0.93 | Damage=80 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Holliday | 3 / ability_crackshot | 0 | 55 / Spirit |  | Damage=55 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Bebop | 1 / citadel_ability_uppercut | 3089858203 | 0 / Spirit | BonusFireRate:-0.186, LandingDamage:0.6 | LandingDamage=75, UppercutDamage=0.01 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Bebop | 2 / citadel_ability_sticky_bomb | 2521902222 | 85 / Spirit | BonusDamagePctPerPlayerHit:0.0015, BonusDamagePctPerPlayerKilled:0.01, Damage:1.5 | BonusDamagePctPerPlayerHit=1, BonusDamagePctPerPlayerKilled=2.5, Damage=85, SelfDamagePercent=20 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Bebop | 3 / citadel_ability_hook | 1928108461 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Bebop | 4 / citadel_ability_bebop_laser_beam | 3832675871 | 1760 / Spirit | DPS:2.511 | BeamCloseDamagePercent=75, DPS=160 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Calico | 1 / ability_nano_clustergrenade | 4131517918 | 45 / Spirit | Damage:0.644 | Damage=45 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Calico | 2 / ability_nano_dash | 1426567660 | 10 / Spirit |  | ImpactDamage=10 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Calico | 3 / ability_nano_catform | 1009029159 | 0 / Spirit |  | EnemyDamageSpeedPenalty=65 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Calico | 4 / ability_nano_shadow_pulse | 2054144742 | 150 / Spirit | Damage:0.609336 | Damage=150 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Grey Talon | 2 / ability_power_jump | 3452399392 | 0 / Spirit |  | WeaponDamageBonus=3 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Grey Talon | 3 / ability_immobilize_trap | 512733154 | 25 / Spirit |  | Damage=25 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Grey Talon | 4 / ability_guided_arrow | 3242902780 | 230 / Spirit | Damage:0.93744 | BonusTechPowerPerKill=8, Damage=230 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Grey Talon | 1 / ability_charged_shot | 0 | 80 / Spirit |  | Damage=80 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Mo & Krill | 2 / ability_burrow | 2406758797 | 375 / Spirit | DPS:1.488 | DPS=75, EnemyDamageSpeedPenalty=0.5 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Mo & Krill | 3 / ability_throw_sand | 1914797280 | 40 / Spirit |  | Damage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Mo & Krill | 4 / ability_ult_combo | 1917840730 | 96 / Spirit | DPS:0.6 | DPS=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Mo & Krill | 1 / ability_intimidate | 0 | 50 / Spirit |  | Damage=50, DamageHealMult=1.2, DamageHealMultNonHero=0.35 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Shiv | 1 / citadel_ability_shiv_dagger | 2460791803 | 0 / Spirit | BleedDPSPerStack:0.13, ImpactDamage:0.0 | BleedDPSPerStack=10, BleedDuration=5, BleedTickRate=1 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Shiv | 2 / citadel_ability_shiv_dash | 1458044103 | 75 / Spirit | ImpactDamage:1.2 | ImpactDamage=75 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Shiv | 3 / citadel_ability_shiv_defer_damage | 1537272748 | 0 / Spirit |  | DamagePctDeferred=25, DamagePctDeferredMaxRage=15, DeferredDamageDuration=6 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Shiv | 4 / citadel_ability_shiv_killing_blow | 1835738020 | 200 / Spirit |  | BuffDamage=8, Damage=200, RagePerSpiritDamage=0.01452864, RagePerWeaponDamage=0.0158766 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Ivy | 1 / citadel_ability_tengu_urn | 4111222521 | 160 / Spirit | DPS:0.55 | DPS=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Ivy | 2 / citadel_ability_tangotether | 1531378655 | 0 / Spirit | BonusFireRate:0.18 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Ivy | 3 / citadel_ability_tengu_stone_form | 3642273386 | 75 / Spirit | Damage:0.6 | Damage=75 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Ivy | 4 / citadel_ability_tengu_airlift | 1247583368 | 0 / Spirit | AirDropBulletShield:0.0, ExplodeDamage:0.7 | AirDropOutgoingDamagePercent=20, ExplodeDamage=115 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Warden | 1 / ability_warden_crowd_control | 2656490109 | 60 / Spirit | Damage:0.63 | Damage=60 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Warden | 2 / ability_warden_high_alert | 2751689917 | 0 / Spirit | CombatBarrier:0.8 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Warden | 3 / ability_warden_lock_down | 1656913918 | 110 / Spirit | Damage:2.437344 | Damage=110 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Warden | 4 / ability_warden_riot_protocol | 2702908623 | 420 / Spirit | PulseDPS:1.3 | PulseDPS=70 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Yamato | 1 / citadel_ability_power_slash | 3255651252 | 0 / Spirit | FullChargeDamage:1.85 | FullChargeDamage=145, MediumChargeDamagePct=50, ShortChargeDamagePct=30 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Yamato | 2 / citadel_ability_flying_strike | 2566573207 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Yamato | 3 / citadel_ability_healing_slash | 2366960452 | 55 / Spirit | Damage:0.37 | Damage=55, FireRateSlow=30 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Yamato | 4 / citadel_ability_infinity_slash | 3319782965 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Lash | 1 / citadel_ability_lash_down_strike | 3561817145 | 0 / Spirit | StompDamage:0.7905, StompDamagePerMeterPrimary:0.04, StompDamagePerMeterSecondary:0.008137 | StompDamage=60, StompDamagePerMeterPrimary=5.5, StompDamagePerMeterSecondary=4.2 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Lash | 2 / citadel_ability_lash | 2670099061 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Lash | 3 / ability_lash_flog | 519124136 | 65 / Spirit | Damage:0.85 | Damage=65 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Lash | 4 / citadel_ability_lash_ultimate | 397010810 | 105 / Spirit | ImpactDamage:0.974938, ThrowDistance:0.14 | ImpactDamage=105 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Viscous | 1 / viscous_goo_grenade | 3247040238 | 55 / Spirit | Damage:0.7 | Damage=55, FourthHitDamagePercentage=0.26, SecondHitDamagePercentage=0.5, ThirdHitDamagePercentage=0.38 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Viscous | 2 / viscous_restorative_goo | 3788152387 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Viscous | 3 / viscous_telepunch | 1020817390 | 20 / Spirit |  | Damage=20 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Viscous | 4 / viscous_goo_bowling_ball | 4206531918 | 110 / Spirit | Damage:1.0 | BreakablePropDamageRadius=75, Damage=110 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Pocket | 1 / synth_barrage | 938149308 | 0 / Spirit | DamagePerProjectile:0.465 | AmpPercentPerStack=6, DamagePerProjectile=32 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Pocket | 2 / synth_plasma_flux | 1976701714 | 60 / Spirit | Damage:1.3 | Damage=60 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Pocket | 4 / synth_affliction | 2954330093 | 0 / Spirit | CurrentHealthDamage:0.0, DPS:0.21 | DPS=32, DamageInterval=0.5 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Pocket | 3 / synth_pulse | 0 | 65 / Spirit |  | Damage=65 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Mirage | 1 / mirage_fire_beetles | 3733594387 | 0 / Spirit | DPS:0.1 | DPS=8 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Mirage | 2 / mirage_tornado | 1336069669 | 65 / Spirit | Damage:0.3 | Damage=65 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Mirage | 3 / mirage_sand_phantom | 2221949202 | 0 / Spirit | ProcDamageBase:0.35 | DMarkMultiplierPerStack=2, ProcDamageBase=11 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Mirage | 4 / mirage_teleport | 2604653402 | 0 / Spirit | CombatBarrier:0.0 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Vyper | 1 / ability_viper_debuffdagger | 645773760 | 50 / Spirit | Damage:0.8, DamagePerStack:0.4 | Damage=50, DamagePerStack=25 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Vyper | 2 / ability_viper_venom | 3428732129 | 0 / Spirit | VenomMaxDamage:2.79, VenomMinDamage:0.651 | VenomMaxDamage=140, VenomMaxDamageHealthPercentage=30, VenomMinDamage=20, VenomMinDamageHealthPercentage=100 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Vyper | 3 / ability_viper_snakedash | 1469918191 | 0 / Spirit | CombatBarrier:0.0 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Vyper | 4 / ability_viper_petrifybola | 1594727071 | 50 / Spirit | Damage:0.744, PetrifyDamage:2.046 | Damage=50, PetrifyDamage=180, PetrifyDamageBreakThreshold=200 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Sinclair | 1 / ability_magician_magicbolt | 3156905386 | 0 / Spirit | MaxDamage:1.86, MinDamage:0.93 | CloneDamagePercentage=50, MaxDamage=120, MaxDamageTime=2, MinDamage=60 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Sinclair | 2 / ability_magician_cloneturret | 0 | 15 / Spirit |  | Damage=15 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Sinclair | 3 / ability_magician_animalhexarea | 0 | 0 / Spirit |  | DamageAmpPercentage=15 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie; Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Sinclair | 4 / ability_magician_copyult | 0 | 0 / Spirit |  |  | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Mina | 1 / ability_vampirebat_steallife | 193508101 | 60 / Spirit | Damage:1.0, RakeHealPerKill:0.3 | Damage=60, MissingHealthDamagePercentage=6 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Mina | 2 / ability_vampirebat_batblink | 2398133749 | 0 / Spirit | AbilityCastRange:0.02 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Mina | 3 / ability_vampirebat_lovebites | 1233782561 | 0 / Spirit | BonusDamage:1.85, MagicDamagePerBullet:0.09 | BonusDamage=45, MagicDamagePerBullet=4 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Mina | 4 / ability_vampirebat_batswarm | 3379486147 | 4.6 / Spirit | Damage:0.094 | CurrentHealthDamageCapToBosses=20, Damage=4.6 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Drifter | 1 / drifter_blood_blast | 3120550633 | 0 / Spirit | BonusDamage:1.4 | BonusDamage=40 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Drifter | 2 / drifter_shadow_mark | 1264175662 | 0 / Spirit | DotHealthPercent:0.015, TeleportDamage:0.5 | DotHealthPercent=2 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Drifter | 3 / ability_drifter_hunger | 1033019133 | 0 / Spirit | InvisDuration:0.0 | AmpDamagePercent=15, RevealOnDamageDuration=0.25, WeaponDmgPerIsolationKill=3 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Drifter | 4 / drifter_darkness | 1168182161 | 0 / Spirit | BonusFireRate:0.0 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Venator | 3 / ability_priest_beartrap | 11154161 | 80 / Spirit | Damage:2.2 | Damage=80 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Venator | 4 / ability_priest_weaponswap | 2918874166 | 120 / Spirit |  | BonusDamage=115, Damage=120, ExecuteThreshold=8 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Venator | 1 / ability_priest_flashbang | 0 | 35 / Spirit |  | BurnDuration=3.5, BurnLingerDuration=0.15, DPS=10, Damage=35 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Venator | 2 / ability_priest_knockback | 0 | 60 / Spirit |  | BonusDamage=30, Damage=60 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Victor | 2 / ability_frank_selfzap | 3727084744 | 0 / Spirit |  | CurrentHealthPercentDamage=15 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Victor | 3 / ability_frank_painaura | 1472593493 | 0 / Spirit | MaxDPS:0.72, MinDPS:0.15 | MaxDPS=58, MinDPS=13, SelfDPS=15, SelfDamagePercentage=70 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Victor | 4 / ability_frank_revive | 3493884851 | 200 / Spirit | BonusDamagePerBullet:0.0, Damage:2.0 | Damage=200 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Victor | 1 / ability_frank_shocktarget2 | 0 | 100 / Spirit |  | Damage=100, StoredDamageHealthPercentRequired=40 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
| Paige | 1 / ability_bookworm_dragonfire | 604448162 | 210 / Spirit | DPS:0.3, Damage:1.3 | DPS=30, Damage=60 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Paige | 2 / ability_bookworm_knightbarrier | 3553292912 | 0 / Weapon | BaseAttackDamagePercent:0.3, BonusFireRate:0.0, CombatBarrier:1.5 | BaseAttackDamagePercent=25, BonusSpiritDamagePercent=15 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Paige | 3 / ability_bookworm_aoemagic | 3054993162 | 90 / Spirit | Damage:1.3 | Damage=90 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Paige | 4 / ability_bookworm_knightcharge | 1841901343 | 125 / Spirit | Damage:1.0 | Damage=125, MaxAmp=100 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| The Doorman | 1 / ability_doorman_bomb | 103496908 | 40 / Spirit | ExplosionDamage:1.2, ImpactDamage:0.7 | ExplosionDamage=55, ImpactDamage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| The Doorman | 2 / ability_doorman_doorway | 109766091 | 0 / Spirit | CombatBarrier:0.0, DoorwayDistance:0.0 |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| The Doorman | 3 / ability_doorman_luggage_cart | 1719512318 | 0 / Spirit | CartDamage:0.75, WallImpactDamage:0.0 | CartDamage=60 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| The Doorman | 4 / ability_doorman_hotel | 1703045691 | 75 / Spirit | Damage:1.0, LateCheckoutDamage:1.5 | Damage=75, LateCheckoutDamage=125 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Billy | 1 / ability_punkgoat_ult | 1841661392 | 35 / Spirit | Damage:1.1 | Damage=35 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Billy | 2 / ability_punkgoat_goatflip | 1792773251 | 40 / Spirit | Damage:1.9, DealMaxHealthDamagePct:0.0 | Damage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Billy | 3 / ability_punkgoat_blasted | 3296031213 | 0 / Spirit | MaxHealthMelee:0.6 | BulletDamageAmp=10, BulletDamageAmpDuration=7 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Billy | 4 / ability_punkgoat_tether | 3762861984 | 246 / Spirit | DPS:0.9, Damage:0.7, DamageIncreasePct:0.0 | DPS=45, Damage=120 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Graves | 1 / ability_necro_hauntingskull | 3214055642 | 16 / Spirit | Damage:0.25 | Damage=16, SummonTakesDamage=1 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Graves | 2 / ability_necro_zombiewall | 2001589780 | 90 / Spirit | Damage:1.6, ZombieWallLength:0.05 | Damage=90 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Graves | 3 / ability_necro_fear | 629925354 | 0 / Spirit | MaxStolenAttackDamage:0.25 | MaxStolenAttackDamage=25 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Graves | 4 / ability_necro_gravestone | 4110547059 | 115 / Spirit | AbilityDuration:0.04, Damage:1.27, SummonMeleeDamage:0.5 | BonusSpiritDamagePercentage=15, Damage=115, DamageSlowDuration=0.5, DamageSlowPercent=20, GravestoneTakesDamage=1, SummonMeleeDamage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Apollo | 1 / ability_fencer_throwblade | 725153709 | 85 / Spirit | Damage:1.3 | Damage=85 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Apollo | 2 / ability_fencer_riposte | 2006773718 | 0 / Spirit |  | DamageThreshold=60 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Apollo | 3 / ability_fencer_lunge | 147493976 | 25 / Spirit | BaseDamage:0.55, MaxDamageBeforePerfect:0.9, PerfectDamage:1.55 | BaseDamage=25, MaxDamageBeforePerfect=40, MaxProcBleedDamagePercent=50, PctTravelDistanceToDamageIn=80, PerfectDamage=65 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Apollo | 4 / ability_fencer_ultimate | 494112189 | 70 / Spirit | DelayedDamage:2.6, ImpactDamage:0.77 | BonusDamagePercent=60, DelayedDamage=200, ImpactDamage=70, IncomingDamageReductionPercent=70 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Rem | 1 / ability_familiar_ability02 | 3692321247 | 75 / Spirit | Damage:1.6 | Damage=75 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Rem | 2 / ability_familiar_attach | 1568845221 | 0 / Spirit |  |  | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Rem | 3 / ability_familiar_helpinghands | 536278136 | 20 / Spirit |  | DPSPerSprite=1, Damage=20, InfestDamageTakenPercent=30, PatrolDamageCooldown=10 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Rem | 4 / ability_familiar_ability01 | 2396527575 | 0 / Spirit | AwakeDamage:1.6 | AwakeDamage=120, DamageResistPctWhileChanneling=30, SleepDamageThreshold=100 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Silver | 1 / ability_werewolf_unloadgun | 1385849824 | 40 / Weapon |  | CurrentHealthDamagePercentage=2.5, Damage=40, NonPlayerBonusWeaponPower=100 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Silver | 2 / ability_werewolf_kickflip | 2717130900 | 0 / Spirit | BonusDamage:2.0 | BonusDamage=25 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Silver | 3 / ability_werewolf_netshot | 3983754897 | 40 / Spirit | Damage:1.6 | Damage=40 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Silver | 4 / ability_werewolf_transformation | 515791019 | 0 / Spirit | BonusFireRate:0.45 | RagePerDamage=0.255 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Celeste | 2 / ability_unicorn_prismaticguard | 1950738949 | 0 / Spirit | CombatBarrier:0.8 | BarrierDamagePercentage=50 | Basis 0 trotz positiver schadensbezogener Rohfelder: Bezug/Parser prüfen |
| Celeste | 3 / ability_unicorn_luminousstrike | 1011349580 | 55 / Spirit | ImpactDamage:0.63 | ImpactDamage=55 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Celeste | 4 / ability_unicorn_dazzlingorb | 2590796390 | 165 / Spirit | Damage:0.88 | Damage=165 | Kein offensichtlicher Null-Prüffall; keine fachliche Freigabe |
| Celeste | 1 / ability_unicorn_radiantblast | 0 | 15 / Spirit |  | Damage=15, FlareDamage=40 | Klassenfähigkeit ohne GC-ID: bisheriger Kampf-ID-Filter überspringt sie |
