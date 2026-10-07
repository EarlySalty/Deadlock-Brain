# G: Bestandsbefund Mechanik

status: erledigt, 07.10.2026. Recherche abgeschlossen, keine Implementierung oder Spielparität behauptet.

BESTAND[BS-1]: teilweise | Fundort: rust/crates/dbrain-reasoner/src/combat.rs:390 | Anknüpfung: bestehende Loader, Fortschritt, Spiritprojektion, Shopboni und Kampfauswertung gemeinsam nutzen

## Wiederverwendung

- `mechanics.rs:289`: `weapon_dps` rechnet den dauerhaften Magazinzyklus, nicht ein begrenztes Zeitfenster. `:344`, `:412` und `:448`: kanonische Spiritprojektion. `ERoundsPerSecond` hat Vorrang vor `EFireRate`, keine doppelte Addition.
- `progression.rs:58`: `at_souls` mit Seelenschwellen, Belohnungen und Upgradebuchungen. Quantifiziert Bullet Damage, HP und Spirit; andere Boon-Effekte bleiben bislang unbekannt.
- `data.rs:1495`, `:1530`: `ability_damage_units` und `refresh_ability_derived` für skalierte Ability-Werte, Dauer, Ticks, Projektile, Charges und Upgrades. Rohe Ability-Parser nicht parallel nachbauen.
- `combat.rs:390`, `:1982`, `:2029`: ausführlicher Inventarevaluator, Aggregation und kostenschwellenbasierte Shopboni. Komponenten und Verkäufe über bestehende Inventarübergänge, eindeutige gehaltene Items vorausgesetzt.
- `ability_interactions.rs`, `item_interactions.rs`, `defense.rs:9`: zeitabhängige Effekte, Schilde, Heilung und Regenerationsauslöser.

## Erforderliche Ergänzungen am gemeinsamen Kern

`types.rs:133` enthält kein vollständiges Waffenmodell: Burst, Pellets, Spin-up, Einzelnachladen, Falloff-Bias und Endskalierung fehlen. Der Loader übernimmt `bullets_per_second` als `shots_per_second`; Schuss- und Projektilraten dürfen nicht ungeprüft gleichgesetzt werden.

`types.rs:199` verwirft angeborene Regeneration und Resistenzen. Eine öffentliche Statprojektion muss die vorhandene private Aggregation zugänglich machen. Ziel-HP, Zielresistenzen, Entfernung, Trefferquote und Headshots sind als Szenarioeingaben zu führen. Bestehende Planer-Vergleichsszenarien können bleiben.

`simulate` ist privat und arbeitet mit 0,2-Sekunden-Schritten, anteiligen Schüssen und heuristischer Rotation. Der Gegner beginnt ohne Resistenz. Die Resistenzrechnung kombiniert Quellen multiplikativ, Shred dagegen additiv aus absoluten Properties. Das ist nicht dieselbe Rechnung wie der Sheet-Shred. Weder diese Differenz noch die TTK-Vereinfachung darf durch angepasstes Referenzmaterial verschwinden.

Eine lesende SQL-Reproduktion für Abrams ergab 32,9771 DPS aus der derzeit übernommenen Waffe gegenüber 46,4886 API-DPS mit Reload. Rohdaten enthalten neun Pellets, 3,6 Bullet Damage, 32,4 Schaden pro Schuss und Einzelnachladefelder. Kein Rust-Test oder Fehlerfix belegt; Einheiten und Zyklus zuerst klären.

## Daten- und Betriebsgrenze

Bestehende Loader unter `data.rs:649`, `:667`, `:732`, `:928` laden `deadlock_assets_api`, referenzierte Primärwaffen und Signaturen. Fehlende referenzierte Snapshots erzeugen Fehler. Itemmechanik stammt aus Snapshots; Katalog und `deadlock_data/item_card` ergänzen Metadaten.

Lesend gemessen: zentrale DB 66.849 Snapshots, 40 Kataloghelden, 251 Katalogitems. Dedizierte Brain-DB 55.561 Snapshots ohne Hero-/Item-Katalog. Für 39 Kataloghelden sind Primärwaffe und alle 156 Signaturen vorhanden. Baba, ID 88, fehlt als akzeptierter Asset-Heldensnapshot. Wardens verwendbarer Heldensnapshot stammt vom 2. Oktober, trotz neuerer globaler Assets. Kein Laufzeit-DSN oder vollständiger aktiver Spiegel nachgewiesen.

Die normalen Reasoner-Wege lesen weder `sheet_boons_ap` noch `sheet_shop_bonuses`. Sheet-Normalisierung `dbrain-normalize/src/sheet_tabs.rs:107` übernimmt Werte, rechnet keine Formeln. `lab.rs:129` ruft den Planer und ist keine unabhängige skalare Schnittstelle. Fast-Evaluatoren unterdrücken Diagnostik und reichen nicht als Abdeckungsnachweis.

F besitzt Planer, Confidence und Veröffentlichung. G ergänzt den gemeinsamen Rechenkern, baut keinen zweiten Simulator oder Importer.

## Herkunft

Nativer Rechercheagent `a335322bab03367e9`, Workflow `wf_4e39a761-389`, Abschluss vom Werkzeug bestätigt. Vollständiger Rohbericht: `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/030a7b6f-d25c-482d-b66c-68185cd05dbb/tasks/wyqxc1iva.output`, `result[0]`. Bereichsführung hat den Bericht in diese knappe Akte übernommen. Produktquellen gegenüber `bfda408c` unverändert; zwischenzeitlicher Dokumentcommit `96e6a8da`. Keine Tests, DB-Schreibzugriffe oder Runtimeänderungen durch diesen Rechercheagenten.
