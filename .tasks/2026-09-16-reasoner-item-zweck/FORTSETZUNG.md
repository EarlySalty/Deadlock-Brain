# Fortsetzung im bestehenden Integrationsbranch

ChatGPT führt die Arbeit selbst im Worktree brain-purpose-finish weiter. Ausgangsrevision 6238b3d9fa294a3f6b16577c9a6298c8ec397906. Keine neuen Claude-/Opus-Worker, keine weitere bezahlte Modellausführung, keine Änderung an zentraler Datenbank oder Diensten.

## Nachher-Messung des bereits vorhandenen Ladungsfixes

Tatsächlich gelesen mit `build_evaluation summary target/completion-charges-6238b3d.json`. Artefakt nennt exakt Revision 6238b3d und den eingefrorenen Stand vom 13.09.2026. Gegenüber PHASE0 bleibt Warden bei 6/9 Referenzwaffen und Jaccard@12 0,5, sein Populations-Kendall liegt bei 0,460255 statt 0,4706; Staple-Gate weiterhin 9/10 und falsch. Vindicta hat nach dem Ladungsfix wieder 9/9 Staples und Jaccard@12 0,6, aber Populations-Kendall 0,339118 statt 0,3799. Damit ist die verlorene Staple-Abdeckung repariert, nicht jede Metrik regressionsfrei. Keine Freigabe daraus abgeleitet.

## B: tatsächliche Spirit-Treffer, Refresh und nutzbare Regeneration

Quelle tatsächlich mit dem Rust-Inspector aus FROZEN-V2 gelesen: `upgrade_mystic_regeneration`, Snapshot 13117, fetched_at 30.06.2026, payload_hash ccc269ee2bec4ecdf8f864fdc86ed355c09c191c81c09afd60b91c332c9e9826. Der Snapshot enthält Regeneration=4 HP/s, RegenerationDuration=7 s sowie die Beschreibung: Spirit-Schaden an gegnerischen Helden aktiviert Regeneration, unterschiedliche Helden stapeln. Die ältere rohe Definition aus Mai hat Dauer 6; sie wurde NICHT mit der neueren Definition vermischt. Itemname und Klassenname werden nur zum Auffinden des Belegs verwendet, niemals als produktive Bewertungsregel.

Neu: ConditionKind::SpiritDamageToHeroes bewahrt Refreshdauer und optional belegtes Stapellimit. Ein enger Parser verbindet die numerischen Eigenschaften mit der normalisierten Mechanikbeschreibung. Fehlende/ungültige Zahlen, fehlender Ziel- oder Schadensbezug und nicht erkannte Formulierungen werden nicht anhand eines Namens geraten. Bereits normalisierte Bedingungen bleiben auch bei übersetzten/gelöschten Anzeigetexten erhalten. Das ist ein Parser für eine belegte Mechanikfamilie, kein vollständiges Sprachverständnis aller Tooltips.

Der vorhandene Simulator führt je Item Ablaufzeiten nach stabiler Zielidentität. Mehrere Treffer auf dasselbe Ziel erneuern dessen Ablauf statt neue unterschiedliche Ziele zu erfinden. Die Integration zählt echte Stapelsekunden einschließlich Ablauf mitten im Zeitschritt, künftige Treffer erzeugen keine rückwirkende Heilung. Ausschließlich tatsächlich verursachter zulässiger Spirit-Schaden gelangt hinein; proc-disabled-Fähigkeiten lösen nicht aus. Die Ausschlussregel wurde auch für den vorhandenen Waffen-Buildup-/Brandpfad nachgezogen.

Heilung wird auf fehlendes Leben begrenzt. Kein angespartes Overheal, keine Gleichsetzung mit einer lebenslangen passiven Regenerationsstatistik. Stackbelegung und tatsächlich nutzbare Regeneration werden in CombatScenarioEvaluation nach Item-ID ausgewiesen und in ItemScore.condition_factor sowie eine numerische Mechanikkette eingespeist. Der statische Score benutzt denselben begrenzten Einzelitem-Simulator; der Planner rechnet anschließend erneut im tatsächlichen Inventar. Kein weapon_share-, Hero-, Itemnamen- oder Populationsgewicht als Ersatz für den Ereignisstrom. Ohne passendes Item werden keine neuen ereignisbezogenen Vektorallokationen pro Tick vorgenommen.

Explizite Grenzen: Der Eigenbuff wird nach Verlust eines Ziels bis zum Ablauf erhalten; dies ist eine ausgewiesene Annahme, nicht durch die Tooltip-Zeile alleine experimentell bewiesen. Gleichzeitige Flächentreffer sind weiterhin nicht simuliert; die vorhandenen Szenarien können verschiedene Folgeziele treffen. Weitere Heil-/Dauermodifikatoren werden mit diesem Teilfix nicht automatisch als verstanden erklärt. Die insgesamt noch offene B-Quantifizierungsabdeckung bleibt offen.

## Tatsächlich ausgeführte Prüfungen

- Neue öffentliche Combat-Tests zuerst rot: 0/2 bestanden. Fehlende Ableitung der Bedingung und kein Regenerationsnutzen bei zulässigem Burst nachgewiesen.
- Danach `cargo test -q -p dbrain-reasoner --lib`: 199 bestanden, 16 datenbankabhängige Tests ignoriert, 0 fehlgeschlagen. Die 7 neuen Tests prüfen Parser, Zeit-/Zielbindung, Caps, Ablauf, Ereignisreihenfolge, tatsächlichen Combat-Nutzen/Proc-Ausschluss sowie Condition-Faktor/Sprachinvarianz/schnellen und erklärten Pfad.
- `cargo clippy -q -p dbrain-reasoner --lib --examples -- -D warnings`: Exit 0.
- Fokussierter Formatter über ignoriertes target/focused-format/Cargo.toml, keine repoweite Neuformatierung.

Die Zahlen sind lokale Rust-Gegenproben, keine Live-KI-Läufe oder Spielclient-Beweise. Vollständiger gleicher Backtest dieses neuen B-Teilfixes und unabhängige Abnahme sind noch separat erforderlich. Kein main-Merge, Deploy oder Publish durch dieses Dokument.

## C: schadenskanalgerechte Kapazität und konsistenter Item-Score

Neue Gegenproben waren vor der Änderung rot: Ein unpassender Schild gab 700 statt 600 effektives Leben; ein reiner Anti-Waffen-Debuff gab gegen reinen Spirit-Schaden 800 statt 600. Das wurde nicht mit Item- oder Hero-Ausnahmen repariert, sondern über einen expliziten eingehenden Schadensmix und getrennte Ressourcenkanäle.

ReasonerConfig enthält incoming_weapon_share. Alte eingefrorene Configs behalten über serde exakt 0,5 als bisherigen Vergleichsdefault; 0 und 1 bilden die reinen Gegenkanäle ab. Der neue gemeinsame Rust-Helfer löst die stückweise Gleichung max(Waffenrate*x-Waffenschild,0)+max(Spiritrate*x-Spiritschild,0)=HP+Universalbarriere. Resistenz und gegnerbezogene Reduktionen verändern nur die zugehörige Rate. Ein riesiger, noch nicht verbrauchter Schild des falschen Kanals darf den HP-Verlust nicht bezahlen.

Combat, Schildnutzen bei Fähigkeitswahl und die bekannten defensiven Item-Statwerte nutzen denselben Kapazitätsrechner. Der statische WeaponPowerDebuff-Wert hing zuvor an den eigenen Waffen-DPS; jetzt hängt er am geschützten HP-Pool und gegnerischen Schadensmix. Der alte 10-Punkte-Test erwartete die falsche Bezugsgröße. Er ist begründet auf (600/0,875-600)/40=2,142857 umgestellt und zusätzlich gegen reinen Spirit-Schaden und millionenfach geänderte eigene DPS abgesichert. Das ist keine Änderung, um eine Referenz-Itemliste zu treffen.

Grenze bleibt ausdrücklich: Kapazität gegen einen konstanten Mix ist keine vollständig simulierte gegnerische Rotation. Die bisherige Druckkurve, fehlende detaillierte Schildverbrauchs-/Wiederaufladehistorie und unquantifizierte Gegnerreaktionen werden nicht als gelöst ausgegeben. Dieser Teilfix allein erzwingt daher auch kein gewünschtes Glass-Cannon-Urteil.

## Nachprüfung B: keine zweite flache Regenerationsgutschrift

Eine zusätzliche öffentliche Score-Gegenprobe fand nach dem ersten B-Commit noch 4 Punkte Nutzen ohne jedes Spirit-Ereignis. Ursache war die alte allgemeine Regeneration-Property-Heuristik neben dem neuen Ereignisrechner. Zeitgebundene Regeneration wird nun aus diesem flachen Pfad ausgeschlossen. Score und Simulator zählen sie nur einmal; sowohl reine properties als auch gespiegelte passive_properties sind geprüft. Der neue Test war rot (4 statt 0) und ist danach grün. Kein bereits veröffentlichter Stand wurde dafür angepasst; sämtliche Arbeiten bleiben im Integrationsbranch.

## C-Fortsetzung: tatsächlicher Lebensdruck und endliche Schilde

Die erste C-Korrektur hat die falsche Kanalbewertung beseitigt. Anschließend ist auch die noch offene prozentuale Druckkurve entfernt: Das Druckszenario nimmt nun einen expliziten incoming_pressure_dps-Wert oder den bisherigen Vergleichsumfang von 85 Prozent des Helden-Basislebens pro Fenster als FESTES Schadensbudget. Eigene Item-HP, Bonusleben oder Max-HP-Verlust verringern nicht länger automatisch den eintreffenden Schaden. Der Gegnerkontext bleibt eine offengelegte Modellannahme, keine gemessene gegnerische Fähigkeitsrotation.

DamageLedger verbraucht typgebundene Schilde, anschließend universelle Barrieren und dann Leben. Unveränderte Schildkapazitäten füllen verbrauchte Ressourcen nicht im nächsten Tick wieder auf. Kapazitätserhöhung gewährt nur den Zuwachs; Ablauf begrenzt den Rest. Wiederholte item-spezifische Refresh-/Aufladeregeln bei unveränderter Gesamtkapazität sind weiterhin nicht belegt und werden nicht erfunden. Regen/Lifesteal dürfen nur bereits entstandenen Lebensverlust auffüllen, nicht künftigen Schaden vorwegheilen. Score, frühzeitiger Tod und Diagnosefelder incoming_health_damage/remaining_health/remaining_shields verwenden diesen Zustand. Die Auswertungszeit bleibt auf 0,2 Sekunden aufgelöst.

Neue öffentliche Gegenprobe zunächst rot: 600 Basis-HP, 13 Prozent Max-HP-Verlust, 16 Sekunden Fenster und 37,5 reine Waffen-DPS führten im alten Modell NICHT zum Tod. Danach tatsächlich grün: ohne passende Deckung Tod bei 14,0 Sekunden (522/37,5=13,92, auf den Schritt aufgerundet). Ein 200er Waffenschild trägt bis zum Fensterende mit 122 HP Rest; ein 200er Spiritschild bleibt beim früheren Tod ungenutzt. Keine Item-/Hero-Namenregel und kein pauschaler 600-HP-Abzug.

Drei ältere Tests wurden an die belegte Semantik angepasst: Der 700-HP-Pool nach 13 Prozent Verlust bleibt im gesunden Szenario exakt 609, nach festen 510 eingehenden Schadenspunkten unter Druck aber nur noch 99. Zeitmittel bei 25 Schrittanfängen: 364,2; über alle drei Szenarien 527,4 statt einer ständig angenommenen vollen Kapazität von 609. Der Lifesteal-Test vergleicht nun tatsächlich verbliebenes Leben mit/ohne zulässigen Spirit-Heal, nicht ein künstliches EHP-Plus oberhalb vollen Lebens. Die bestehende Meldung „Leben aufgebraucht“ bleibt als Ereignis erhalten. Die Testanpassungen sind physikalisch hergeleitet, nicht aus der gewünschten Build-Liste abgelesen.

Aktueller vollständiger Reasoner-Testlauf danach: 211 Library-Tests und 22 Example-Testausführungen bestanden, 16 DB-Tests ignoriert, 0 fehlgeschlagen. Clippy der Library/Examples ebenfalls Exit 0. Die folgenden Prüfzahlen dokumentieren den davor liegenden C-Kapazitätsteil.

## Prüfung des vorherigen C-Kapazitätsstands

- cargo test -q -p dbrain-reasoner --lib --examples: 207 Library-Tests und 22 Example-Testausführungen bestanden, 16 DB-Tests ignoriert, 0 fehlgeschlagen.
- cargo check -q --workspace --all-targets: Exit 0; dies ist ein Workspace-Kompilationsnachweis, kein ausgeführter Workspace-/DB-Testlauf.
- cargo clippy -q -p dbrain-reasoner --lib --examples -- -D warnings: Exit 0.
- Fokussiertes fmt für eigene Dateien einschließlich defense.rs: Exit 0.

Die Replays unter target/completion-regen-2cc57ac.json und target/completion-regen-2cc57ac-repeat.json wurden als bereits vorhandene Artefakte angetroffen; Wiederholungsaufrufe lehnten ein Überschreiben mit Exit 1 ab. Die tatsächlich gelesene Summary des ersten Artefakts nennt Revision 2cc57ac und mode=replay, mit unveränderten ausgewiesenen Warden-/Vindicta-Metriken gegenüber 6238b3d. Ein erfolgreich frisch ausgeführter kompletter Mehrheldenlauf wird aus diesen Exit-1-Aufrufen nicht behauptet. Der neue CLI-Modus replay nimmt beide Dateien explizit, lädt niemals eine DB und bricht bei fehlender Heldenpopulation ab; seine fünf Example-Tests bestehen.
