# Arbeitsstand: globale TOML-Konfiguration

Stand: 20.09.2026. **WIP, nicht produktiv abgenommen.**

## Gesicherte Ausgangslage

`origin/main` wurde frisch gelesen: `b306fa9`. Kanon auf `feat/brain-rust-cutover-20260919` mit HEAD `458d56d0845f83bb50fdb635e78c0367f3cff8a1` und fremden Änderungen. Eigener Worktree/Branch siehe AUFTRAG.md. Release-/Evidence-Branches sind nicht integriert und werden nicht still übernommen.

Die Graphify-Bestandssuche wurde ausgeführt und anschließend am Quelltext geprüft. Der bestehende zentrale Rust-Lader ist `deadlock-brain-core/src/config.rs`; zuvor keine globale TOML, nur `config/infisical.json` und `config/youtube_feeds.json`. `config.rs` liest ENV vor `.env` vor Kompilat-Default. `model_resolver.rs` liest dynamisch FIREWORKS_MODEL/FIREWORK_MODEL, hält einen prozessweiten Cache ohne Policy/Frist und löst erst nach HTTP 404 auf. `AiClient` kann das im Request angegebene Modell heimlich überschreiben.

### Tatsächlich gelesener Dienstzustand

| Dienst | Ausgangslage | Startpfad und Bedeutung |
|---|---|---|
| deadlock-brain-site | PID 1122, aktiv seit 10.09., NRestarts=0, `/usr/bin/python3.12` | Separater Build-Korpus-Site-Server für `/brain`, Portbeschreibung 8087. Nicht entfernen. Kein Rust-Cutover-Beweis. |
| deadlock-brain-build-data | failed, ExecMainStatus=1, zuletzt 19.09. 03:30 | Kanonischer `scripts/run_build_data_with_infisical.sh`, täglicher Timer 03:30. |
| deadlock-brain-sheet-sync | failed, ExecMainStatus=1, zuletzt 20.09. 00:49 | Wrapper-Drop-in ersetzt alten Python-ExecStart. Timer alle vier Stunden. |
| deadlock-brain-youtube-learning | failed, ExecMainStatus=1, zuletzt 19.09. 22:51 | `scripts/run_youtube_learning_with_infisical.sh`, Timer. |
| deadlock-brain-patchnotes-sync | letzter Lauf 20.09. 02:00 erfolgreich, NRestarts=0 | Externer `~/.local/bin/deadlock-brain-patchnotes-sync.sh`, fünfminütiger Timer. Ein No-op ist kein Inhaltsbeweis. |

Die lesende `/proc/*/exe`-Prüfung fand zu diesem Zeitpunkt keine Brain-/dbrain-Rust-Binary. Es wurden keine Prozessumgebungen und keine Secret-Werte gelesen oder ausgegeben. Keine Units, Timer, Produktionsdateien oder Datenbanken wurden verändert.

## Implementiert und lokal geprüft

- Strikter, typisierter TOML-Vertrag im bestehenden Core (`bot_config.rs`), Schema 1, unveränderliche Arc-Momentaufnahme.
- Begrenzte Dateigröße, absoluter Config-Pfad, arbeitsverzeichnisunabhängige interne Pfade einschließlich Symlink-Auflösung.
- Unbekannte Felder, fehlende Werte, falsche Typen, ungültige IDs, Zeitgrenzen, Samplingwerte, fremde/credentialhaltige Endpunkte und nicht freigegebene Pin-Syntax werden ohne Eingabeausschnitt abgelehnt.
- Redigierte Statusansicht und stabiler Fingerabdruck; frei beschreibbare Texte und Pfade werden nur gehasht ausgegeben.
- `config/bot.toml` dokumentiert die nachgewiesenen **Code-Defaults**, nicht vermeintlich ausgelesene Produktionswerte. Neue Auswahlgrenzen sind als neue Werte kommentiert.
- Core-Baseline: 16 Tests grün. Mit Lader: 31 Tests grün, keine ignorierten Tests in diesem Lauf.

## Noch offen, keine abgeschlossene Migration behaupten

1. Den bestehenden `config.rs` und alle Runtime-Einstiegspunkte wirklich auf die neue validierte Momentaufnahme umstellen. Der neue Parser allein ersetzt derzeit noch keine ENV-Verbraucher.
2. Sicherer, schlüsselspezifischer Vergleich tatsächlicher Betriebsvorgaben aus bisherigen Launchern/Infisical mit der TOML. Keine Environment-Dumps. Ohne diesen Abgleich keine Produktivschaltung der Datei.
3. Alle Rust-CLI-Pfade einschließlich Population, Reasoner, Build-Narration und YouTube anbinden; die alten Python-Pfade nur als Cutover-Abhängigkeit dokumentieren, nicht neu portieren. HTTP-Ziel und Rolle des separaten Site-Servers im Cutover klären.
4. Dynamische ENV-Leser und Wrapper inventarisieren und durch Regressionstest absichern. Betriebswerte und fachliche Request-Parameter unterscheiden.
5. Modell-Resolver ersetzen: offizieller paginierter Katalog, Metadaten, numerische Familienversion, Status/Fähigkeiten, begrenzte Retries/Timeouts, Sperre, synthetische und fachliche Proben, Frist/Policy-gebundener Postgres-Status, atomare Clientwahl. Der neue Pin-Syntaxcheck ist **keine Verfügbarkeitsprüfung** und keine Modellautomatik.
6. Postgres-Verbindungsgrenzen, Retrieval-/Indexvertrag, Quellendienste, Batch-Limits, Timer und Wrapper vollständig übernehmen. Keine neue SQLite- oder Python-Brücke.
7. Fachtests und Verbraucher-Nachweise, Rot-Gegenproben, Test-Gate und unabhängiger Merge-Kritiker; Fehler am Code beheben, Gates nicht umgehen.
8. Nach zulässiger Integration auf main mit `-j 2` bauen und über den vorhandenen gesperrten Deploy-Pfad ausrollen. PID, exe, Fehlerjournal, Binary-Anker, Inhaltsprüfung, Ort, Heartbeat und NRestarts belegen.
9. Entwicklerdokumentation nach Deadlock-Docs `internal/Deadlock-Brain/`, Betriebsbefunde nach bestehendem 2nd-Brain-Schema. Erst nach bestätigtem Merge und Live-Beweis Branch-/Worktree-Cleanup.

## Einordnung

Alle Konfigurationsänderungen erfordern zunächst Neustart. Der Test mit fehlgeschlagenem erneuten Laden beweist nur, dass eine bereits gehaltene Momentaufnahme unverändert bleibt; er behauptet weder Hot-Reload noch wiederaufgebaute Clients. Zugangsdaten bleiben in Infisical; Sessions, OAuth, Spielstände, Korpus-/Modellprüfstatus sind keine globale Config.
