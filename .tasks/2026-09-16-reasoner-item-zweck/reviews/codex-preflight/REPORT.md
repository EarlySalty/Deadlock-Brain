# Unabhängiger Preflight: Messbasis und vorhandene Rust-Mechaniken

Datum: 16.09.2026. Geprüfter Hauptstand: `ee440f32c548f31c3a731b10d038e54b21429700` (Dokumentation auf Produktcode-Basis `706b129d9c3af487d7ee2ef53bf43ed788e3b8ec`).

**Urteil: BLOCK für eine behauptete vollständige Phase-0-Baseline, Phasenfreigabe oder Veröffentlichung.** Vier gezielte vorhandene Rust-Unit-Tests bestehen. Das ist weder ein vollständiger Backtest noch ein abgeschlossener Implementierungsauftrag. In diesem Review wurden keine Produktquellen verändert, keine zentralen DB-Aufrufe ausgeführt, keine Dienste geändert und kein Build veröffentlicht.

## 1. Zwei konkrete Blocker vor der Messung

### P0-1: `population stats` ist ein Schreibbefehl

Quelle: `rust/crates/dbrain-population/src/cli.rs:26-35,168-195`.

Der Unterbefehl beschreibt den Neuaufbau der Aggregate, ruft `db::assert_writable(pool)` und anschließend `aggregate::rebuild_hero(...)` auf. Er darf deshalb NICHT als lesender Baseline-Aufruf gegen die zentrale Datenbank benutzt werden. Der Aufruf wurde in diesem Review bewusst nicht ausgeführt.

Abhilfe innerhalb des vorhandenen Rust-Pfads: vorhandene Aggregate über einen technisch erzwungenen Read-only-Pool auslesen und als lokal unveränderliche Eingabe sichern. Ein notwendiger Neuaufbau gehört ausschließlich in die lokale Wegwerf-DB; Produktion erst durch den Delegator beim freigegebenen Deploy. Auch `population show` nicht unbesehen als erzwungen read-only bezeichnen: `run_population` verwendet derzeit `db::connect()` und lädt den Katalog online.

### P0-2: Die aktuelle Frozen-Messung lädt Population live nach

Quelle: `rust/crates/dbrain-reasoner/examples/build_evaluation.rs:437-456,554-585` und `rust/crates/deadlock-brain-core/src/pg.rs:11-30,45-58`.

Die Replay-Modi `evaluate`, `holdout`, `sensitivity` und `plan` rufen `load_populations()` auf. Diese Funktion verwendet `pg_pool()`, lädt Hero-IDs und `PopulationPrior` aus der laufenden DB und übergibt sie an die Auswertung. `pg_pool()` erzwingt kein Read-only; ein eigener `pg_pool_read_only()`-Pfad ist bereits vorhanden. Die hier untersuchten Abfragen sind SELECTs; es wird NICHT behauptet, dass dieser Helper selbst bereits geschrieben habe.

Damit sind die Eingaben trotz identischer `FROZEN-V2.json` nicht vollständig eingefroren. Der bestehende Messvertrag aus dem 13.09. beschreibt einen früheren Zustand und reicht als Beleg für den heutigen Aufrufpfad nicht aus. Das Ausweichen auf einen leeren PopulationPrior würde die Messung verändern und darf nicht als äquivalente Baseline ausgegeben werden.

Erforderliche Reparatur der Messstrecke vor A: Population gemeinsam mit dem verwendeten Modell-/Assetstand lokal einfrieren, Quellen/Versionen/Hashes und den Read-only-Nachweis sichern; Replay ausschließlich aus diesen Dateien, ohne Datenbank- oder Netzwerkinitialisierung. Abwesende Population als explizite Datenlücke ausweisen oder im beweisfähigen Modus ablehnen. Alte Vergleichsdateien niemals überschreiben. Tests müssen einen Replay ohne DSN/Netzzugang und eine Änderung des Population-Inputs nachweisen. Das betrifft die Messstrecke, nicht eine neue Produkt-Engine.

## 2. Der Ausgangsbefund muss gegen vorhandenen Code präzisiert werden

### A: Spirit → Feuerrate ist nicht vollständig unsichtbar

`WeaponProfile` besitzt tatsächlich kein eigenes `per_spirit`-Feld (`types.rs:114-124`). Daraus folgt aber NICHT, dass die Konversion in der Simulation fehlt:

- `data.rs:308-330` liest `scaling_stats`, einschließlich `scaling_stat = ETechPower` und `scale`, in `HeroModel.scaling`.
- `item.rs:150-216` bewertet `ERoundsPerSecond` beziehungsweise `EFireRate` als Prozent-Fallback und berücksichtigt Spirit bei Waffen-DPS und Item-Score.
- `combat.rs:639-652` lädt dieselbe Konversionsachse; `combat.rs:827-829` rechnet sie in die tatsächliche Waffenrate des Inventars ein.
- Bereits vorhandene Tests prüfen, dass direkte Schussrate und Prozent-Fallback nicht doppelt addiert werden.

Phase A muss deshalb die tatsächlichen Assetpfade, Einheiten, vollständigen Zielstats und Downstream-Verbraucher prüfen und vorhandene Pfade vereinheitlichen/ergänzen. Ein zweiter additiver Spirit-Feuerratenpfad allein wegen eines neuen WeaponProfile-Feldes wäre ein Doppelzählungsrisiko. Die unten ausgeführten Tests sind synthetische mechanische Fixtures; sie beweisen KEINE vollständige Live-Assetabdeckung, keinen 2–3-Helden-Screenshotabgleich und keine korrekte aktuelle Warden-Buildauswahl.

### C: Prozentualer Max-HP-Abzug ist bereits teilweise implementiert

`combat.rs:126-127` verarbeitet `MaxHealthLossPercent`; `combat.rs:838-842` zieht ihn vom berechneten Pool aus Basisleben, Prozentbonus und flachem Bonusleben ab. Der vorhandene Test `max_health_loss_applies_to_bonus_health_and_healing_cap` bestätigt für 600 Basisleben + 100 Bonusleben und 13 % Verlust einen Wert von 609.

Die offene Arbeit ist damit nicht nur das Einführen des Prozentabzugs, sondern die belastbare Risikobewertung im jeweiligen Kauf-/Gegner-/Zeitkontext. In der aktuellen Simulation ist der Druckverlauf unter anderem vorgegeben (`health_fraction` in `combat.rs:729-733`); daraus darf keine vollständige Simulation gegnerischer Bedrohung behauptet werden. Ein grüner HP-Rechentest ist kein Beweis, dass Glass Cannon ohne ausreichende Defensive korrekt verworfen wird.

Arithmetische Präzisierung: 1599 → 1391 bedeutet 208 HP Verlust. 13 % entsprechen erst bei einem Pool von ungefähr 4615 HP etwa 600 HP. Keine fixe 600-HP-Strafe oder erfundenen Maxlevelwerte. CC-Immunität nicht als Schadensimmunität und Shields nicht als kontextunabhängige HP-Erstattung behandeln.

### D: Ein Teil des Compounding existiert bereits

Der vorhandene Inventartest `spirit_changes_whole_weapon_rate_and_magazine_value` besteht: Spirit erhöht den simulierten Waffenschaden; ein zusätzliches Magazin erhöht ihn weiter und reduziert Reloads. Das ist eine konkrete vorhandene Synergie, aber noch kein Beweis für korrekte Kauf-/Upgrade-/Verkaufsentscheidungen, kausale Begründungsketten oder Konvergenz gegen 779996.

### B: Schadenanteile alleine identifizieren keinen Dauer-Trigger

`DamagePlan` enthält aggregierte DPS und Anteile, aber keine vollständige zeitliche Ereignisfolge. Gleiche DPS/weapon_share können aus einem seltenen Burst oder kontinuierlichen kleinen Ticks entstehen. Eine Unterdrückung von Sustain allein nach weapon_share würde daher neue Fehlurteile hart codieren.

Vorhandene Simulationsstrukturen für `spirit_events`, `pending_damage`, `pending_hits` und Zielwechsel sind in `combat.rs:615-633` sichtbar. Trigger, Dauer/Refresh, Distinct-Targets, Stacks und interne Cooldowns sollten an die tatsächlich vorhandenen Ereignisse angeschlossen werden. Namen, Held-IDs und handgepflegte Zweck-Labels bleiben ausgeschlossen.

Rein mathematisch bewirkt das Aufteilen desselben Schadens in kleinere Ticks bei konstantem prozentualem Lifesteal noch keinen geringeren Gesamt-Heal. Abweichungen brauchen einen Datenbeleg für Triggerregeln, Caps, Rundung, Schadens-/Zielmodifikatoren oder Timing. Die konkrete aktuelle Itemmechanik wurde in diesem Preflight nicht neu aus Live-Assets verifiziert; entsprechend keine neue Tatsachenbehauptung dazu.

## 3. Tatsächlich ausgeführte Tests

Arbeitsverzeichnis: `/home/nathanael/repos/Deadlock-Brain/rust`. Toolchain: `/home/nathanael/.cargo/bin/cargo`, Ausgabe `cargo 1.97.1 (c980f4866 2026-06-30)`. Nur Debug-Testprofil, `--offline --locked`; keine Release-Binary ersetzt.

| Aufruf/Filter | Exit | Tatsächliches Ergebnis |
|---|---:|---|
| `cargo test --offline --locked -p dbrain-reasoner --lib item::tests::fix_e_` | 0 | 2 bestanden, 0 fehlgeschlagen, 0 ignoriert, 187 herausgefiltert |
| `cargo test --offline --locked -p dbrain-reasoner --lib combat::tests::max_health_loss_applies_to_bonus_health_and_healing_cap -- --exact` | 0 | 1 bestanden, 0 fehlgeschlagen, 0 ignoriert, 188 herausgefiltert |
| `cargo test --offline --locked -p dbrain-reasoner --lib combat::tests::spirit_changes_whole_weapon_rate_and_magazine_value -- --exact` | 0 | 1 bestanden, 0 fehlgeschlagen, 0 ignoriert, 188 herausgefiltert |

Vier unterschiedliche Tests sind bestanden. Andere Tests wurden damit nicht ausgeführt. Kein Clippy-Gesamtbeleg, kein kompletter Rust-Testlauf, kein frischer Warden-/Populationsbacktest und keine drei KI-Läufe.

## 4. Mess- und Workerstatus

Frische Referenzwaffen, Staples, Kendall tau, Jaccard@12, Fehlurteile und unquantifizierte Mechanik-Abdeckung: **noch nicht gemessen**. Historische Angaben aus BEFUND/ORCHESTRIERUNG sind keine neue Phase-0-Baseline. Die beiden P0-Blocker müssen vorher geschlossen werden.

Im Repository wurde ein bereits angelegter fremder Worktree `/home/nathanael/repos/wt/brain-purpose-a`, Branch `feat/reasoner-purpose-a`, Basis `706b129d9c3af487d7ee2ef53bf43ed788e3b8ec` vorgefunden. Bei der ersten Statusprüfung enthielt er nur unversionierte Auftragsdokumente, keine Produktquelländerungen. Er wurde in diesem Review nicht verändert, umgeschaltet oder gelöscht.

Während der Vorprüfung erschien im Hauptcheckout `PRUEFHINWEISE.md`. Diese Orchestrierungsakte nennt einen aktiven Opus-4.8-Implementierer mit T3-Thread-ID `803d3e94-9b1d-42c5-9bb7-1905c8146acc` für den vorhandenen A-Worktree und dokumentiert, dass der Hinweis zur bereits bestehenden Konversion im Thread bestätigt wurde. Das ist ein Dokumentationsbeleg für die separate Beauftragung; dieser Review hat den Thread nicht selbst gestartet und seinen Laufstatus nicht direkt über T3 abgefragt. Keine abgeschlossene Phase oder fertiggestellten Änderungen allein aus diesem Eintrag ableiten. Kein Doppelworker und kein Eingriff in die andere Orchestrierung.

Agentenprogramme `hermes`, `claude` und `t3` wurden bei reinen Hilfeaufrufen durch die codex-mcp-Programm-Allowlist abgewiesen. Keine Umgehung über Shell, alternative Programmpfade, Runtime-Subprozesse oder Sicherheitskonfigurationsänderungen. In diesem Chat wurde deshalb kein Opus-Worker gestartet; kein unterstellter Agentenfortschritt und keine spätere automatische Fortsetzung. Dieser Reviewer-Worktree enthält ausschließlich Dokumentation.

## 5. Freigabebedingung / Übergabe

Zuerst P0-1/P0-2 mit unveränderlicher, vollständig dokumentierter Eingabe schließen und echte Baseline sichern. Dann Phase A auf den bereits vorhandenen Skalierungspfaden durchführen, mit echten Assetbelegen und Doppelzählungs-/Einheiten-/Vorzeichen-/Umbenennungstests. Danach frischer unabhängiger Review und nicht schlechtere identisch eingefrorene Vergleichsmetriken. Die Phasen B–E sowie zentrale Migration, Merge der Implementierungsphasen, Deploy und Veröffentlichung bleiben gesperrt, solange ihre jeweiligen Gates fehlen.

Dieser Bericht ist ein belegter Preflight, keine Erledigtmeldung für den Gesamtauftrag und keine Freigabe des gegenwärtigen Warden-Builds.
