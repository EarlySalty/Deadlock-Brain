# Zentrale TOML-Konfiguration für Deadlock-Brain

Stand: 20.09.2026. Auftraggeber: Nutzer dieser ChatGPT-Sitzung. Durchführung direkt, kein Implementierungs-Worker und keine erfundene T3-ID.

## Verbindlicher Vertrag

Eine globale `config/bot.toml` für Deadlock-Brain, nicht Deadlock-2nd-Brain. Globale nicht geheime Betriebswerte nicht mehr aus ENV lesen; Zugangsdaten bleiben im bestehenden geschützten Infisical-Verfahren. Keine TOML-zu-ENV-Brücke, keine neue Python-Brücke, keine zweite Portierung und keine neue SQLite-Persistenz. Bestehende dynamische Daten, Sessions, OAuth und Modellprüfstatus bleiben außerhalb der TOML beziehungsweise in Postgres.

Konfiguration vor Nebenwirkungen vollständig und typisiert validieren, Schema-Version und unbekannte Felder prüfen, keine Eingabewerte oder Parserausschnitte in Fehlern. Ein absoluter `--config`-Pfad für alle produktiven Rust-Einstiegspunkte. Relative interne Pfade beziehen sich auf das Verzeichnis der Config, nicht auf das Arbeitsverzeichnis. Kein automatisches Überschreiben der Datei. Zunächst explizite Neustartpflicht statt behauptetem Hot-Reload.

Bestehende Anbieter- und Datenschutzgrenzen erhalten. Automatische Fireworks/DeepSeek-Flash-Auswahl nur anhand offizieller Metadaten, numerischer Familienversion, Verfügbarkeit und erfolgreicher Funktionsproben. Keine Pro-, Preview-, Vision-, Distill- oder Fremdanbieter-Ausweichpfade. Ein expliziter Pin gewinnt. Letztes geprüftes Modell nur innerhalb aktueller Policy und Gültigkeitsfrist; Status gehört nicht in die TOML. Tatsächlich verwendetes und protokolliertes Modell müssen übereinstimmen.

## Ausgangslage und Isolation

- Kanon: `/home/nathanael/repos/Deadlock-Brain`; `Documents/Deadlock-Brain` verweist darauf.
- Remote: `git@github.com:EarlySalty/Deadlock-Brain.git`; `origin/HEAD` zeigt auf `main`.
- Frisch gefetchtes `origin/main`: `b306fa9` (vollständige gebundene Heldenfähigkeiten und sichere Namensauflösung).
- Kanon-HEAD: `458d56d0845f83bb50fdb635e78c0367f3cff8a1`, Branch `feat/brain-rust-cutover-20260919`, mit fremden uncommitteten Änderungen an pg/pg_secrets, YouTube und Wrappern. Nicht verändert und nicht in diesen Auftrag übernommen.
- Eigener Worktree: `/home/nathanael/.worktrees/brain-global-toml-20260920`, Branch `feat/brain-global-toml-20260920`, von `origin/main`.
- Unintegrierte Release-/Evidence-Arbeit existiert. Keine Behauptung eines abgeschlossenen Cutovers.

## Abnahme

Parser-, Grenzen-, Pfad-, Redaktions-, ENV- und Verbrauchertests; Modellpolicy- und Transportfehler-Tests; bestehende Fachtests. Sicherer Vorher-/Nachher-Abgleich. Feature-Commits verifizieren und sofort pushen. Test-Gate und unabhängigen Merge-Kritiker gemäß Rolle Merge-Schleuse ausführen. Erst danach Integration auf bestätigtes main, Build mit `-j 2`, Deployment mit PID-/exe-/Journal-/Binary-/Funktionsnachweis und Heartbeats. Unintegrierte Arbeit bleibt erhalten.

Kein Community-Post und keine neue CHANGELOG.md. Entwicklerdokumentation gehört nach Deadlock-Docs `internal/Deadlock-Brain/`, operative Erkenntnisse in das bestehende 2nd-Brain-Schema. Ohne Merge und Live-Beweis ist dieser Auftrag nicht abgeschlossen.
