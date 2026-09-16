# Astra: eigene API-Recherche und Mechanikprüfung

Stand 16.09.2026. Diese Recherche wurde im ChatGPT-Chat ausgeführt, nicht an einen neuen Claude-Worker delegiert. Der bestehende Thread 803d3e94-9b1d-42c5-9bb7-1905c8146acc bleibt ausschließlich bei 0+A. Die neue Arbeitsteilung steht in ARBEITSTEILUNG.md; keine neuen Opus-Folgeworker automatisch starten.

## Was tatsächlich geprüft wurde

Die öffentliche OpenAPI-Spezifikation der Deadlock API wurde über den Webzugriff gelesen. Direkte aktuelle Hero-/Item-Payloads waren darüber nicht abrufbar; daraus folgt KEIN API-Ausfall auf dem Produktionshost. Dort war der Connector zeitweise nicht erreichbar. graphify und curl wurden von seiner Programm-Allowlist abgelehnt; diese Sperren wurden nicht über ein anderes Programm umgangen. Deshalb beruhen die Spielwerte unten ausdrücklich auf dem vorhandenen lokalen Modell-Snapshot, nicht auf einem frischen Rohdatenabruf.

Selbst gelesene Dateien: Worker-A-DESIGN.md; nachweise/PHASE0-COVERAGE.json; main rust/crates/dbrain-reasoner/src/data.rs (property_values, classify_condition, condition_from_properties, ability_model); combat.rs (active_with_text); planner.rs (Search::choices, Search::evaluate, plan_with_economy). Das ist eine Sichtprüfung, keine ausgeführte Suite oder gemessene Performanceanalyse. Fremde bzw. gleichzeitig angelegte Dokumente im Control-Worktree werden nicht überschrieben.

## Verifizierter API-Vertrag, keine erfundenen Livewerte

Quelle [1] dokumentiert versionierte Assets unter /v1/assets: client-versions, steam-info, heroes, items, generic-data. client_version bindet die Abfragen an eine Spielversion. Der Item-Katalog enthält Fähigkeiten, Waffen und Upgrades; items/by-hero-id schließt allgemeine Bewegungsfähigkeiten aus. ItemProperty bietet neben value auch conditional, provided_property_type, scale_function, display_units und usage_flags. Analytics enthält item-stats mit Gegner-, Kaufzeit- und Rangfiltern, item-flow-stats und item-permutation-stats. hero-build-stats verwendet das zu Spielbeginn ausgewählte Build, nicht spätere Wechsel. Das Rust-Client-Angebot ist in [2] belegt. Keine Abhängigkeit wurde hinzugefügt.

## Konsequenz für den vorhandenen Rust-Pfad

Dies sind eigene Implementierungsanforderungen aus dem Vertrag, keine Behauptung bereits vorhandener kompletter Mechanik:

1. Vor dem Abruf genau eine konkrete verfügbare Spielversion festlegen. Jede Hero-, Weapon-, Ability-, Upgrade- und Generic-Abfrage daran binden. Quelle, Abfrageparameter, Zeitpunkt und Inhalts-Hash sichern. Keine geschätzte Versionsnummer aus einem Schema-Beispiel übernehmen. Live-Aktualisierungen nie in eine laufende eingefrorene Vergleichsmessung mischen.
2. Vollständigkeit über Referenzen prüfen: alle aktiven Helden, ihre Primärwaffen, Slot-Fähigkeiten, verknüpfte Untereffekte und benötigte allgemeine Bewegungsmechaniken. Nur vier Fähigkeiten je Held aufzuzählen beweist kein vollständiges Modell. Roster-Abdeckung und Feld-/Mechanikabdeckung getrennt zählen.
3. Vorhandenen Loader erweitern, keinen parallelen API-Client oder eine zweite Engine bauen. Rohwerte plus Einheit, Zielstat, Skalierungsfunktion, Trigger, Zielbindung und Laufzeit bis zum quantifizierten Effekt erhalten. Unbekannte Werte nicht als bestätigte Null zählen. Darstellungs-Flags können Hinweise liefern, ersetzen aber keinen bewiesenen Ereignisauslöser.
4. Population nur als getrennte Gegenprobe: identisches Zeitfenster, Rang, Modus, Kaufphase und Gegnerkontext. Kaufhäufigkeit, Autorenvorschlag und tatsächlich gewähltes Start-Build sind unterschiedliche Beobachtungen. Weder rohe Siegquote noch häufige gemeinsame Käufe beweisen eine mechanische Synergie; bereits vorhandene Vorteile und spätes Kaufgeld können beide beeinflussen.
5. Aktuelle Rohdaten-Coverage als eigenen Diagnosebericht anlegen. Die laufende FROZEN-V2-Reproduktion mit neuer eingefrorener Population nicht nachträglich umetikettieren: wiederholbar, aber gemischter historischer Eingabestand und keine frische Gesamtbaseline.

## A: konkrete numerische Prüfspur aus dem vorhandenen Modell

PHASE0-COVERAGE.json nennt 38 normalisierte Heldenmodelle, davon einen Feuerraten-Konverter. Für Warden stehen dort shots_per_second=3.8095238095238098, ERoundsPerSecond.per_spirit=0.01 und EFireRate.per_spirit=0.25. Diese Angaben sind tatsächlich gelesen; sie belegen weder Rohdaten-Vollständigkeit noch die aktuelle Spielversion.

Eigene Rechnung: 3.8095238095238098 * 0.25 / 100 = 0.009523809523809525 Schuss/s pro Spirit. Der direkte Modellwert 0.01 liegt 5 Prozent über diesem rechnerischen Fallback. Das kann eine Rundung oder unterschiedliche Semantik sein; die Ursache ist NICHT nachgewiesen. Die beiden Kanten deshalb weder addieren noch ungeprüft als exakt gleich beschreiben. Rohquelle und Priorität belegen; Konflikte sichtbar ausweisen.

Der Entwurf A-DESIGN.md wollte damage_plan unverändert lassen und negative endliche Kanten wie fehlende behandeln. Beide Punkte wurden mit T3-Nachricht sequence 588212 zur Korrektur zurückgegeben. damage_plan braucht denselben expliziten Zustand wie die Konversionsberechnung. Bei null Basis-Spirit ist ein Null-Delta möglich; der Nachweis braucht zusätzlich einen kontrollierten Zustand mit Spirit. Endliche negative Kanten sind Downsides, nicht fehlende Daten. Eine unabhängige Endabnahme des Produktpatches steht weiterhin aus.

## B: Datenverlust an der eigenen/gegnerischen Lebensschwelle

data.rs::condition_from_properties macht aus EnemyLifeThreshold nur StateBound { threshold }. Dabei geht im Typ verloren, dass sich die Schwelle auf den Gegner bezieht. combat.rs::active_with_text erhält lediglich einen health_fraction-Skalar und entscheidet die Vergleichsrichtung anhand englischer Wörter im Beschreibungstext. Der geprüfte Aufrufpfad aus dem vorherigen Befund übergibt den simulierten health_fraction-Wert, nicht einen im Bedingungstyp identifizierten Akteur. Damit ist die Semantik jedenfalls nicht vollständig repräsentiert; ein eigener Lebenswert darf eine gegnerbezogene Bedingung nicht ersetzen.

Anforderung: Bedingung mit Actor/Target, Vergleichsoperator, Schwelle und Quelle ausdrücken; z.B. Enemy/HealthFraction/LessThan/0.3. Zahlen und Beschreibungssprache nicht als Ersatz für den Akteur verwenden. Eine Übersetzung oder Umbenennung darf bei identischem Mechanikgraphen das Ergebnis nicht ändern. Gegentests: Selbst 90 Prozent/Gegner 20 Prozent sowie Selbst 20 Prozent/Gegner 90 Prozent, gleiche gegnerbezogene Schwelle 30 Prozent. Nicht blind eine neue enum-Variante hinzufügen und den Consumer unverändert lassen.

## B: Sustain aus Ereignissen statt pauschalem weapon_share-Abschlag

Ein fester weapon_share beschreibt nicht, wie oft, wie lange oder auf wie vielen unterschiedlichen Gegnern Spirit-Schaden entsteht. Ein waffenlastiger Held mit wiederkehrendem kleinem Proc kann einen Refresh trotzdem halten. Ein einzelner Burst kann wiederum eine lange Nachwirkung anstoßen. Daher keine Ersatzregel 'viel Waffe = Regen nutzlos'.

Für eine aus Rohdaten belegte zielgebundene Regeneration: pro Gegner stabile Identität und Ablaufzeit führen; erneute gültige Treffer refreshen nach der belegten Regel; verschiedene Ziele und Stacks begrenzen; Tod, Zielwechsel, Tickabstände und Proc-Ausschlüsse beachten. Heilung nur bis zum fehlenden Leben und mit den tatsächlichen Modifikatoren anrechnen. Für Lifesteal zählt der gesamte berechtigte Schaden, nicht die bloße Größe eines einzelnen Ticks. Unterschiedliche Tickgrößen allein rechtfertigen kein schlechteres Ergebnis.

Die Datei ASTRA-PRUEFFAELLE.json enthält explizit SYNTHETISCHE Rechen-/Gegenproben, keine Warden- oder Itemgewichte. Sie sind Testanforderungen mit erwarteten Zahlen, noch keine integrierten oder ausgeführten Rust-Tests.

## Performance: konkrete Prüfstelle, Ursache noch offen

Der Worker meldete am 16.09.2026 15:24 UTC fehlenden Fortschritt des Gesamtfreezes bei hoher CPU und beendete seinen eigenen Messprozess. Die Identität des blockierenden Helden und eine Stack-/CPU-Profilierung sind nicht belegt; 'nach Celeste' ist kein hinreichender Heldennachweis.

Eigene Sichtprüfung: Search::choices evaluiert alle passenden Kandidaten und gegebenenfalls deren Einzelverkaufsalternativen, bevor choices.truncate(BEAM_WIDTH) greift. Die vier Beam-Kandidaten begrenzen also nicht die gesamte Zahl teurer Simulationen. plan_with_economy wiederholt dies an Checkpoints und im Folgeschritt. Zugleich reduzieren erfolgreiche Käufe die verbleibenden Kostenbandplätze und setzen used-IDs; eine Endlosschleife oder unbeschränkte Rekursion ist durch den gelesenen Code NICHT bewiesen.

Gezielte Messung vor einem Fix: Held-ID VOR Berechnung loggen; Phase/Checkpoint, Kataloggröße, Simulationen, Cache-Hits/-Misses, Einzel-Simulationszeit und Gesamtzeit messen. Erst so Planner-Auswahl, teure Hero-Mechanik und langsamen ausführlichen Erklärungsmodus unterscheiden. Kein Kandidatenlimit, kürzeres Kampffenster oder Herofilter heimlich als Performancefix setzen, weil das Ergebnisse verändert. Der langsame Fall bleibt ein separater Abnahme-Blocker. Die bereits eingefrorene Warden-Vergleichsmessung darf daneben fortgesetzt werden.

## Quellen

[1] Deadlock API, direkt gelesene OpenAPI-Spezifikation, Abruf 16.09.2026: https://api.deadlock-api.com/openapi.json . Relevante operationIds: list_client_versions, get_steam_info, list_heroes, list_items, get_items_by_hero_id, get_generic_data, item_stats, item_flow_stats, item_permutation_stats, hero_build_stats. Die Spezifikation ist kein Live-Payload-/Wirkungsbeweis.

[2] Offizielles Client-Repository des API-Betreibers, Abruf 16.09.2026: https://github.com/deadlock-api/openapi-clients . Vorhandenes Rust-Client-Angebot, nicht ungeprüft als neue Abhängigkeit eingebaut.

[3] Eigener Repository-Code und Worker-Artefakte an den oben genannten Pfaden. Diese Sichtprüfung verändert keine Produktlogik, keine zentrale DB und keinen Dienst. Kein Deploy, kein Publish, keine neue hero_build_id, kein Gesamt-ALLOW.