# Deadlock API und mechanischer Prüfvertrag: Recherche durch ChatGPT

Stand 16.09.2026. Diese Recherche und Ableitung erfolgte direkt im ChatGPT-Chat, nicht durch einen neuen Claude-Agenten. Ergebnis ist ein Integrations- und Testvertrag, kein bereits implementierter Parser und kein frisch verifizierter Gameplay-Datensatz.

## Quellenstatus

Primärquelle S1: https://api.deadlock-api.com/openapi.json, am 16.09.2026 über Web gelesen. Insbesondere operationId list_client_versions, list_heroes, list_items, get_items_by_hero_id, item_stats, hero_build_stats und metadata.

Primärquelle S2: https://github.com/deadlock-api/deadlock-api-assets, README über Herkunft/Extraktion der Assets. Das Repository beschreibt dekompilierte Spieldaten, nicht manuell geschriebene Item-Zweckrollen.

Primärquelle S3: https://github.com/deadlock-api/deadlock-api, README zur bestehenden Rust-API und Rust-Datenverarbeitung.

Die API-Dokumentation ist gelesen. Live-Payloads für Helden/Items konnten in dieser Recherche NICHT zuverlässig abgerufen werden. Deshalb keine neuen behaupteten Item-Magnituden, kein behaupteter aktueller Clientversionswert und keine angeblich vollständig geprüfte Live-Roster-Abdeckung. Die lokalen Dateinamen heroes.*, raw_heroes.*, items.* und raw_items.* unter data/raw/deadlock_assets_api wurden gesichtet; Dateinamen beweisen weder Aktualität noch Payload-Inhalt. Fehlgeschlagene Abrufe sind kein Beweis für einen Ausfall des Anbieters. Kein umgangener MCP-Allowlist-Deny, keine zentrale Datenbankänderung.

## Aus S1 bestätigter Quellenvertrag

- Versionen: GET /v1/assets/client-versions. Hero-/Item-Endpunkte akzeptieren client_version; ohne Angabe gilt der neueste bekannte Stand.
- Roster: GET /v1/assets/heroes; only_active kann deaktivierte/Entwicklungshelden ausblenden. /v1/assets/items umfasst Fähigkeiten, Waffen und Upgrades. /items/by-hero-id/{id} schließt generische Bewegungsfähigkeiten aus.
- Population: /v1/analytics/item-stats bietet Filter für Held, Gegner, Lane, Kaufzeitpunkt, Kaufposition, Rang, Spielmodus und Zeitbereich. /hero-build-stats/{hero_id} bezieht sich auf das bei Spielbeginn gewählte Build, nicht auf spätere Änderungen. Eine Build-ID ist daher keine tatsächlich ausgeführte Kaufroute.

## Integrationsentscheidung aus diesen Befunden

Nicht die laufende Baseline auf neue Livewerte umstellen. Zuerst den vorhandenen Rust-Importer und seine Snapshot-Form weiterverwenden. Ein künftiger Importvertrag soll explizit den angefragten und tatsächlich belegten Clientversionsbezug, Sprache, vollständige Queryparameter, Abrufzeit, Payload-Hash und Parserrevision festhalten. Ein 404 auf eine Version ist eine Datenlücke, kein Anlass, still auf latest umzuschalten. Bereits verfügbare eigene Snapshots können einen historischen Stand sichern, auch wenn der Anbieter ihn nicht mehr bereitstellt.

Ein Audit soll einen aktiven Produktiv-Roster-Nenner und einen getrennten Entwicklungs-/Inaktiv-Roster ausweisen. Fehlender Hero, fehlende Ability und nicht modellierte Mechanik sind unterschiedliche Zustände. Nur vier mit by-hero-id gelieferte Fähigkeiten zu zählen würde generische Aktionen und andere abhängige Daten nicht automatisch abdecken. Tatsächliche Quellenverknüpfung über IDs/Klassen und referenzierte Daten prüfen, nicht Anzahl vier hart codieren.

Population bleibt Gegenprobe, nicht Mechanikersatz: gleicher Patch-/Zeitraum, Modus, Rang und Kaufzustand. Gegnerkonditionierung ist für Situationsitems nützlich; Gegnername oder Teamzugehörigkeit allein beweist jedoch noch keine Gun-/Spirit-Bedrohung. Hohe Kauf-Winrate ist kein kausaler Itemeffekt. Insbesondere nach endgültigem Vermögen ausgewählte Gruppen sind kein Ersatz für den Zustand zum Kaufzeitpunkt. Unterstützungszahl, fehlende Fälle und Unsicherheit gehören neben die Metrik.

## Eigene Sichtprüfung am Rust-Bestand

Gelesene Dateien: rust/crates/dbrain-reasoner/src/data.rs, item.rs und die bereits im Auftrag dokumentierten combat.rs-Stellen. Exakte Ausgangsrevisionen siehe Worker-Basis 706b129 und A-DESIGN-REVIEW-CHATGPT.md; kein Produktcode in dieser Recherche verändert.

1. data.rs::scaling_stats bewahrt per_spirit/per_level, leitet scale aber nur bei scaling_stat=ETechPower als Spirit ab. ability_model liest zusätzlich scale_function; das ist kein allgemeiner Nachweis einer mehrdimensionalen Konversionssprache. Roh-Inputachsen, Units, caps und Verbindungen zu anderen Stats müssen beim Audit getrennt betrachtet werden. Keine Behauptung, welche zusätzliche Kante ein bestimmter aktueller Held besitzt, solange der Rohbeleg fehlt.
2. item.rs::spirit_fire_rate_value filtert negative Skalen aus und erkennt Spiritnamen enger als combat.rs::apply. Unterschiedliche Aliasabdeckung ist ein generischer Paritätstest, kein Anlass für weitere Itemnamen-Ausnahmen.
3. item.rs::meta_value verwendet abs() für winrate_pp und lift_pp, danach einen positiven additiven Beitrag zum Score. Damit werden bei sonst gleichen Eingaben gleich große positive und negative Werte gleich behandelt. Das ist ein konkreter Prüfpunkt für die vorhandene Bedeutung dieser Felder: Evidenzstärke und Nutzenrichtung dürfen nicht verwechselt werden. Nicht nebenbei in Phase A ändern; die Scorewirkung und bestehende Feldsemantik zunächst isoliert prüfen.
4. ability_model leitet item_proc_disabled aus einem englischen Beschreibungsfragment ab. Parser-/Sprachwechsel dürfen dies nicht unbemerkt ändern. Soweit strukturierte Flags vorhanden sind, diese mit Provenienz bevorzugen; ohne Rohbeleg nicht behaupten, ein bestimmtes Flag sei bereits geliefert.

## Nachgelesene lokale Modellspur und zusätzlicher B-Befund

Inzwischen direkt im A-Worktree gelesen: .tasks/2026-09-16-reasoner-item-zweck/nachweise/PHASE0-COVERAGE.json. Der Bericht bezeichnet sich korrekt als Modell-Abdeckung, nennt 38 Helden und einen Konverter. Im gelesenen Warden-Eintrag stehen shots_per_second=3.8095238095238098, ERoundsPerSecond.per_spirit=0.01 und EFireRate.per_spirit=0.25. Die ersten gelesenen Vergleichseinträge Infernus und Seven haben beide Spirit-Felder null. Das sind lokale normalisierte Modellwerte, kein aktueller Roh-Asset-Nachweis.

Rechenprüfung: 3.8095238095238098 * 0.25 / 100 = 0.009523809523809525. Der direkte Wert 0.01 liegt etwa 5 Prozent über diesem aus dem Modell berechneten Fallback. Rundung, anderer Basisbezug oder andere Semantik sind mögliche Erklärungen, aber nicht bewiesen. Aliaspriorität und Einheit daher datenbelegt klären; weder beide addieren noch ohne Beleg Gleichheit behaupten.

Zusätzlich data.rs::condition_from_properties direkt gelesen: EnemyLifeThreshold wird zu StateBound { threshold }, ohne den Gegner als Bezugsakteur im Typ zu erhalten. Der bekannte combat.rs::active_with_text-Pfad verwendet einen einzelnen health_fraction-Skalar und englische Richtungswörter. Damit enthält die Bedingung nicht mehr ausreichend Information, um eigenes und gegnerisches Leben sicher auseinanderzuhalten. B braucht Akteur, Stat, Vergleichsoperator und Schwelle, nicht nur einen Float. Gegenprobe: eigene HP=90 Prozent/Gegner=20 Prozent und anschließend vertauscht, gegnerbezogene Schwelle=30 Prozent. Bei gleicher gegnerbezogener Mechanik müssen diese Fälle unterschiedlich auslösen, unabhängig von Beschreibungssprache. Das ist Code-Sichtprüfung, kein bereits durchgeführter Gameplaytest.

Beim abschließenden Git-Status wurden außerdem bereits separat gestagte ASTRA-API-PRUEFUNG.md und ASTRA-PRUEFFAELLE.json vorgefunden. Der Bericht ASTRA-API-PRUEFUNG.md wurde gelesen und enthält ergänzende Vorarbeit. Diese zwei Dateien wurden in dieser Bearbeitung nicht angelegt, verändert oder als eigene Änderungen zur Aufnahme ausgewählt. Keine doppelte Implementierung aus paralleler Vorarbeit ableiten.

## B: präzise Ereignismechanik statt Zweck-Labels

Ein reiner weapon_share-Wert enthält weder Schadenszeitpunkte noch Ziele. Für stackende Regeneration ist der relevante Eingang ein Strom zulässiger Treffer mit Zeit, stabiler Zielidentität, Schadenskanal, Quellenfähigkeit, Proc-Berechtigung und Zielart. Zu einer angegebenen Dauer T lässt sich eine Distinct-Target-Regel als Anzahl unterschiedlicher noch aktiver Zielschlüssel formulieren, danach Cap und belegte Refresh-/ICD-Regeln anwenden. Die genaue Spielregel muss aus dem eingefrorenen Rohbeleg kommen, nicht aus dieser Formel geraten werden.

Wichtige Gegenprobe: Ein Burst kann einen Buff auslösen, der nach dem Burst weiterläuft. Burst darf deshalb nicht pauschal auf Nullnutzen abgebildet werden. Synthetisches Beispiel, KEINE behaupteten aktuellen Itemwerte: 8 Sekunden Refresh-Dauer, ein Ziel, Treffer nur bei t=0 ergibt bis t=8 aktive Dauer; Treffer bei t=0,2,4,6 können bis t=14 verlängern. Wiederholte Treffer auf dasselbe Ziel dürfen bei einer Distinct-Target-Regel keine zusätzlichen unterschiedlichen Ziele erfinden. Ob Schaden, tatsächliche Heilung, Overheal oder Buff-Aktivzeit zählt, ist getrennt auszuweisen.

Prozentualer Lifesteal ergibt ohne weitere Regeln p mal Gesamtschaden. Bei 100 Gesamtschaden liefern ein Treffer und zehn kleine Treffer denselben Bruttowert. Unterschiede entstehen erst durch tatsächliche zeitliche Verwundung, Modifikatoren, Caps oder dokumentierte Auslösebedingungen. Keine Tests schreiben, die kleine Ticks allein als Lifesteal-Nachteil vorschreiben. Alle Fähigkeiten eines Helden zusammen auswerten; ein pauschales Burst-/DoT-Label kann eine zeitweise kanalierte oder passive Schadensquelle verdecken.

## C/D: Risiko und Grenznutzen aus demselben Zustand

HP-Downside aus aktuellem HP-Pool berechnen. Defensive wirkt nur gegen passende Kanäle und während ihrer realen Dauer; CC-Immunität ist keine pauschale Erstattung verlorener HP. Sustain auf fehlendes Leben und lebende/erreichbare Ziele begrenzen. Kauf, Komponentenaustausch und Verkauf müssen denselben Inventarzustand inklusive verlorener Verstärkerkanten vergleichen.

Synthetische Waffenrechnung zur Testableitung: Schaden pro Schuss d, Magazin n, Feuerrate r, Reloadzeit R. Im stationären Zyklus ist DPS = d*n*r/(n+R*r). Zusätzliche Feuerrate hat bei R>0 sinkenden Grenznutzen, bleibt für sich genommen aber positiv. Ein Fire-Rate-Item ist daher nicht allein wegen vorhandener Fire Rate schlecht; Opportunitätskosten, andere Effekte, Kosten, Slot und reales Kampffenster entscheiden. Dieses stationäre Modell ersetzt nicht den diskreten Simulator oder die In-Game-Einheitenprüfung.

## Konkrete, noch auszuführende Gegenproben

A: direkte/Fallback-Kante nie doppelt; explizite Null kein Fallback; negative Kante erhalten; gleiche SP-Aliase in allen Konsumenten; unbekannte Einheit nicht als sicheren Bonus zählen; zustandsgleiches damage_plan/Combat/Item-Ergebnis.
B: Burst löst zeitlich begrenzten Buff aus; DoT verlängert ihn nur gemäß Refreshregel; dasselbe Ziel bleibt ein Ziel; mehrere Ziele und Cap; Ablauf/ICD; Proc-disabled; PvE gegen hero-only; gleicher Gesamtschaden bei unterschiedlicher Tickteilung.
C/D: passende gegen unpassende Schilde; Ablauf vor Folge-Burst; kein Overheal-Nutzen; Basis/Upgrade/Ersetzen/Verkauf aus gleichem Zustand; marginale Sättigung ohne Namensregel.
Messung: identisches Asset-/Populations-Dateipaar auf beiden Codefassungen, ohne erreichbare DB; Invalid/fehlende Eingabe muss kontrolliert fehlschlagen statt live nachzuladen; Replay-Vergleich getrennt von historischer Veröffentlichung und von Live-KI-Determinismus.

Diese Liste ist Testentwurf. Keine dieser neuen Gegenproben wird hier als bereits implementiert oder bestanden ausgegeben. Der unabhängige A-Entwurfsbefund ist separat dokumentiert und wurde vom bestehenden Worker in T3 inhaltlich bestätigt.
