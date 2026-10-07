# Paket G: Brain v2

status: aktiv, 07.10.2026

## 07.10.2026: Start und Schnittstellenbedarf

G startet im eigenen Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, Basis `bfda408cb988722ddceadb56bca5b72e12d12731`. Native Sol-Session, UltraCode im Harness aktiviert. Keine weiteren T3-Threads. Zuerst vollständiges Sheet-Modell, danach Bestand, Plan und Bau in getrennten Schreibbereichen.

1. E: G braucht den verbindlichen Leseschnitt des lokalen API-Spiegels: Tabellen, JSON-Felder, Auswahl einer vollständigen aktiven `client_version`, aktive Helden und Patch-Datum. G baut keinen Import, keine zusätzlichen Rohdatenkopien und keinen Patchparser. Vorläufige Anknüpfung sind `entity_snapshots`, `hero_catalog`, `item_catalog` und bestehende Fähigkeitsloader. E bitte den finalen Schnitt in `AN_HAUPT-E.md` festhalten.
2. F: G nutzt Mechanik und Simulation im `dbrain-reasoner` gemeinsam. Bis zur Dateiabgrenzung keine Änderungen an F-Publishlogik, Confidence oder Planer. G braucht die vorhandene Build-Erzeugung als lesenden Werkzeugaufruf; eine Frage veröffentlicht keinen Build ohne vorhandene Freigabe.
3. A: G besitzt künftig die Spielwissens-Werkzeuge im Antwortdienst. Bestehende Freischaltung, Bot-Anbindungen, Invites, Writerfence und Runtimekonfiguration bleiben unangetastet. Geteilte Antwortdienstdateien werden vor Bau im G-Plan einzeln abgegrenzt.
4. Hold: kein Main-Push, Release-Build, Install, Neustart oder produktiver Tick. Featurearbeit und lokale Prüfungen laufen unabhängig weiter. Abbau erst nach belegtem Gleichstand und als separate Commits.

Gebaut: nein. Reviewt: nein. Gemergt: nein. Live: nein. Nachweise folgen unter `G/`; Statusproduzent ist diese G-Session. Nächster Vertrag: versionierter Leseschnitt von E.
