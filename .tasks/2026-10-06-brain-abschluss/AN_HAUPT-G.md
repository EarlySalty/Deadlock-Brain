# Paket G: Brain v2

status: aktiv, 07.10.2026

## 07.10.2026, 05:23: Vollständige Sheet-Rechnung und benötigter Leseschnitt

G-Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, Basis `bfda408cb988722ddceadb56bca5b72e12d12731`. Diese Berichtsversion liegt im eigenen Worktree: Nach dessen Eintritt erlaubt das Harness keine Schreibzugriffe auf die gemeinsame Checkout-Akte. Die bereits gestarteten Rechercheworker liefern weiterhin in ihre vorher zugewiesenen getrennten Bereiche der gemeinsamen Akte. Kein alternativer Schreibweg wird benutzt; die Ergebnisse werden für den Featurecommit übernommen.

Der unveränderte XLSX-Export liegt vor: 13 sichtbare Tabs, darunter `BoonsAP` als Exportname für „Boons/AP“. Das Sheet setzt global 35 Boons und 38 Spirit. Seine „Base“-Werte können dadurch bereits Spirit enthalten: Wardens angezeigte Feuerrate ist `3.81 + (4 × AG25%)`, Hazes Magazin `25 + AG20`. G trennt echte Basiswerte von Szenariowerten. Vollständiges Modell und Abgleich sind noch in Arbeit.

Leseschnittbedarf an E, vor Implementierung gemeldet:

1. Hero-Rohpayload vollständig erhalten: `starting_stats`, `standard_level_up_upgrades`, `level_info`, `scaling_stats`, `purchase_bonuses`, `cost_bonuses`, Auswahl-/Entwicklungsflags und `items`. Damit werden Boons/AP, Max-Level und Shopboni aus Spielwerten statt Sheet-Konstanten berechnet. Die öffentliche API-Probe für Version 6759 belegt unter anderem Wardens 36 Level-Einträge und standardisierte Boon-Upgrades.
2. Hidden Mechanics benötigt zusätzlich versionierte Rohdaten aus `npc-units`, `misc-entities`, gegebenenfalls `modifiers` und `generic-data`. Die API benennt bei NPCs Widerstände, Backdoor-Modifier, Goldbelohnung, Wachstum und zeitabhängigen Schutz; bei Misc-Entities unter anderem Gold pro Minute. Nach dem Sheet-Modell meldet G die tatsächlich gelesenen Felder. G baut keinen eigenen Import oder Patchparser.
3. Benötigt werden Tabellen, JSON-Felder und der verbindliche Auswahlvertrag einer vollständig importierten aktiven `client_version` mit Patch-Datum und aktiven Helden. Vorläufige Anknüpfung sind `entity_snapshots`, `hero_catalog`, `item_catalog` und bestehende Fähigkeitsloader.

Öffentliche OpenAPI: `https://api.deadlock-api.com/openapi.json`. Die Lesemessungen beweisen weder einen vollständigen lokalen Spiegel noch die aktive Produktionsversion. E besitzt den Import. DB-Verdrahtung hängt am bestätigten Vertrag von E.

Versionsgebundene Referenzproben für Warden, Wraith und Haze liegen in `G/API-PROBEN.json`. Die API liefert exakte Waffen-DPS 66,0571, 59,6825 und 50,0952. Das Sheet zeigt 68,5426, 59,784 und 50,0752. Ursache sind unterschiedliche Spirit-Szenarien, vorgelagerte Rundung und bei Warden abweichende Rohwerte/Skalierung. Das ist noch kein Rechenkern- oder Antwortdienst-Test; die Rust-Abnahme muss dieselben Rohwerte und Szenarien rechnen.

## 07.10.2026: Start und Eigentum

Native Sol-Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, UltraCode im Harness aktiviert und tatsächliche Workflowstarts bestätigt. Keine weiteren T3-Threads. Vollständiges Sheet-Modell, Bestand und Baseline laufen in getrennten nativen Schreibbereichen. Bereichsregister: `G/REGISTER.md`.

F besitzt Publish-Abnahme, Planer und Confidence. G nutzt vorhandene Mechanik, Simulation und Build-Erzeugung gemeinsam. Eine Frage veröffentlicht keinen Build ohne vorhandene Freigabe. Geteilte Dateien werden vor Implementierung im G-Plan einzeln abgegrenzt.

A schließt die bisherige Steckbrief-Freischaltung ab. Bot-Anbindungen, Invites, Writerfence und Runtimekonfiguration bleiben unangetastet. G übernimmt danach die Spielwissens-Werkzeuge im bestehenden Antwortweg.

Release-Hold: kein Main-Push, Release-Build, Install, Neustart oder produktiver Tick. Featurearbeit und lokale Prüfungen laufen unabhängig weiter. Abbau erst nach belegtem Gleichstand und als separate Commits.

Gebaut: nein. Reviewt: nein. Gemergt: nein. Live: nein. Statusproduzent ist diese G-Session; ein zentraler Versuch wurde noch nicht gesondert vergeben. Nächster Vertrag: versionierter Leseschnitt von E.
