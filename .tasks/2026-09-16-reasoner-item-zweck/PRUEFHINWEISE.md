# Integrationsprüfung – belegte Ausgangslücken

Stand: Sichtprüfung des Rust-Quellstands 706b129/ee440f3 am 16.09.2026. Dies ist KEINE Worker-Freigabe und KEIN ausgeführter Backtest. Der aktive Implementierungsworker ist T3 803d3e94-9b1d-42c5-9bb7-1905c8146acc, Opus 4.8, Worktree /home/nathanael/repos/wt/brain-purpose-a, Basis 706b129. Kein Doppelworker gestartet. Zusätzlicher Hinweis zur bestehenden Konversion wurde im Thread bestätigt.

## A: vorhandenen Pfad vereinheitlichen, nicht duplizieren

`dbrain-reasoner/src/combat.rs` enthält vor `base_stats` bereits `spirit_rate`: `hero.scaling` wird nach `ERoundsPerSecond` beziehungsweise `EFireRate` mit `per_spirit` durchsucht; EFireRate wird über die Basis-Schussrate und /100 umgerechnet. Das Fehlen von `WeaponProfile.per_spirit` allein beweist daher noch nicht, dass jede Konversion in Combat unsichtbar ist. Loader-Quellfelder/Einheiten und sämtliche Verbraucher sind der eigentliche Nachweis. `hero.rs` berechnet `weapon.sustained_dps` aus dem Waffenprofil, `damage_plan` wiederum über mechanics. Reviewer muss sicherstellen, dass weder eine doppelte Konversion noch unterschiedliche Basis-/Inventar-/Progression-Werte entstehen.

## B: unbekannt ist nicht unbedingt bedingungslos

`data.rs::classify_condition` kombiniert numerische Eigenschaften und breite Texttreffer. Der letzte Rückfall ist `ConditionKind::None`, auch wenn der Text eine nicht erkannte Auslösung beschreibt. `mechanics.rs::condition_factor_for_hero` vergibt bei unbekannter Action teilweise 0,5; `condition_factor` für ActionBound ebenfalls 0,5. `combat.rs::active_with_text` gibt für unbekannte Actions dagegen false. Diese Abweichung zwischen flachem Scoring und Inventarsimulation gehört in die Vereinheitlichung, nicht in eine neue Namensliste.

Das Modell hat eine einzelne `ItemModel.condition` und eine Menge bedingter Propertynamen. Mehrere Effekte mit verschiedenen Auslösern müssen abbildbar bleiben (z.B. bedingter Regen plus unbedingtes Basisleben), ohne einen Faktor pauschal auf das gesamte Item zu legen. Erwartete Distinct-Target-Stapel aus realem Tick-/Refresh-/ICD-Verlauf, nicht ausschließlich aus `weapon_share`. Ein Burst-Held mit derselben Gesamt-DPS wie ein DoT-Held benötigt im Test andere Zeitreihen.

## B/C: Kontext tatsächlich durchreichen

`combat.rs::Stats` führt BulletShieldMaxHealth, TechShieldMaxHealth und CombatBarrier in EIN `shield`-Feld zusammen. Die Annahmen von `evaluate_core` setzen gegnerischen Schaden pauschal auf 50 % Waffen und 50 % Spirit und rechnen genau einen gegnerischen Helden in Aura-Nähe. Der vorhandene Drei-Szenarien-Vergleich ist also noch kein Anti-Gun-/Anti-Spirit-/Mehrziel-Kontext. Ein guter Counter darf nicht allein dadurch zum universellen Kern werden, dass jedes Szenario denselben festen Schadensmix bekommt.

`MaxHealthLossPercent` wird in `apply` bereits multiplicativ als health_loss berücksichtigt. Phase C ist deshalb eine Prüfung des tatsächlichen Pools, Kauf-/Levelzustands, Schadenskanals, Zeitverlaufs und relevanter Deckung, nicht einfach ein zusätzlicher pauschaler HP-Malus. Aktive CC-Immunität nicht in Schild-HP umdeuten.

## Abdeckungsbericht nicht durch Weglassen verbessern

`combat.rs::metadata` blendet unter anderem MaxStacks, TickRate, ProcCooldown, DamageThreshold und DamageThresholdDuration aus der unquantifizierten Liste aus. Das ist nur legitim, wenn die zugehörige Mechanik diese Metadaten tatsächlich verbraucht. Die verringerte Länge einer Textliste beweist keine größere Mechanikabdeckung. Parser-Erfolg, Modellrepräsentation, tatsächlicher Sim-Verbrauch und Testbeleg getrennt zählen; vorher unbekannte Wirkung nicht einfach als Metadatum klassifizieren.

## Tests: Zeitfenster und deterministische Replays

`ReasonerConfig::default` setzt 40 Sekunden; Combat simuliert Duell, Druck und bewegliches Ziel und kann bei Zieltod früh enden. Es gibt keinen universellen festcodierten 3-Sekunden-Default. Kurze reale TTK kann trotzdem genau die beanstandete Verzerrung erzeugen. Tests müssen mindestens Lane-Trade, längeren Schaden/Refresh und Zielwechsel abdecken und bei gleichen Eingaben dieselben Metriken ausweisen. Keine Behauptung '40 Sekunden simuliert', wenn ein Szenario nach 3 Sekunden tatsächlich endet.

## Datenbank-Sicherheit bereits vorhanden nutzen

`deadlock-brain-core/src/pg.rs::pg_pool_read_only` setzt `default_transaction_read_only=on` über `PgConnectOptions::options` für jede Verbindung. Die Read-only-Examples zeigen zusätzlich `SHOW default_transaction_read_only`. Dieser Weg ist belastbarer als die historische Shell-Angabe PGOPTIONS, deren Wirkung vom verwendeten Treiber abhängt. Nur vorhandene freigegebene Infisical-Zugänge verwenden, keine Secrets ausgeben. `--no-persist` allein ersetzt keine serverseitige Read-only-Verbindung; implizite CLI-Migrationen vorher prüfen.

Diese Funde gehen in B/C/D und die unabhängige Abnahme ein; A bleibt auf seinen vereinbarten Scope begrenzt. Produktcode wurde durch diese Sichtprüfung nicht verändert.
