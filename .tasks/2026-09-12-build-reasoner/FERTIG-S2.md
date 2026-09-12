# Paket S, Fixrunde 2

Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`.
Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan-2`.
Branch: `fix/autoren-scan-live`, Basis `aba7636`.

## Ursachenbeleg vor dem Fix

1. `citadel_gcmessages_client.proto:1383` definiert `k_eInternalError = 0`.
   Das Antwortfeld hat diesen Default (`:1388`). Der bestehende Discovery-Request
   (`discovery.rs:111`) enthält Autor und Sprache `[0,0]`, aber keinen Helden.
   Der erfolgreiche exakte Lookup (`discovery.rs:256`) enthält `hero_id`.
   Fünf lesende GC-Suchanfragen über den vorhandenen Task-Handler auf dem
   laufenden zweiten Bot, seriell, ohne Retry:

   | Task | Anfrage neben Sprache `[0,0]` | Echte Antwort |
   | --- | --- | --- |
   | 4047536 | Autor 1650097169 | FAILED, `Response code: 0` |
   | 4047539 | gleicher Autor, Held 25 | DONE, response 1, result_count 40 |
   | 4047541 | gleicher Autor, Held 0 | DONE, response 1, result_count 0 |
   | 4047556 | gleicher Autor, Held 25, Build 779996 | DONE, response 1, result_count 1 |
   | 4047566 | gleicher Autor, Suchtext LIGHTBRINGERxSITUATION WARDEN BUILD | FAILED, `Response code: 0` |

   Damit ist ein konkreter Held für diese GC-Suche erforderlich. Held 0 ist
   kein brauchbarer Platzhalter für alle Helden. Sprache war bereits identisch;
   Suchtext ersetzt den Helden nicht. Der existierende API-Handler gibt bei
   Code 0 keine Roh-Ergebnisliste heraus. Deshalb ist deren Leerheit nicht
   behauptet; die erfolgreiche Gegenprobe benötigt keine Umdeutung von Code 0.

2. `catalog.rs:99-102` führt alle Autoren und anschließend alle konfigurierten
   Builds seriell in einem Task aus. Live enthält der Bestand 13 Autoren,
   38 Helden und 71 aktive Build-Konfigurationen über 36 Helden.
   `tasks.rs:334-346` und `:280-304` prüfen Stale anhand `started_at`, nicht
   `updated_at`. Ein Heartbeat nur auf `updated_at` hilft damit nicht.
   `runner.rs:130-140` hält das Lane-Permit bis zum tatsächlichen Handler-Ende;
   `:214-244` ändert beim Stale-Reap nur den DB-Status. Background hat
   Parallelität 1 (`lanes.rs:92-105`).

3. Journal bestätigt Start 19:26:32 UTC und Stale-Reap 19:36:33 UTC für
   Task 4047117. Eine weitere lesende DB-Prüfung während dieser Fixrunde zeigt:
   Erst um 19:41:03 UTC beendet der ursprüngliche Handler seinen Lauf und
   überschreibt den Stale-Fehler mit `BUILD_CATALOG_CYCLE: discovery failed`.
   Gleichzeitig wechselt Task 4047138 von PENDING zu RUNNING, started_at
   19:41:03 UTC. Ursache ist das belegte Lane-Permit, keine Dedup-Sperre und
   keine hängende DB-Lease. Er wurde somit schon vor dem Fix von selbst abgeholt.

4. Die Discovery hat im vorhandenen Code keinen Journal-Aufruf pro Autor;
   der Zyklus-Handler keinen End-Log. Statuswerte allein in der DB erklären
   den fehlenden Einblick im Journal.

## Bearbeitung

Ursachen dokumentiert, Implementierung und Tests folgen in dieser Session.
Kein Deploy, keine manuellen Korrekturen an Produktionsdaten, keine Unter-Agenten.
