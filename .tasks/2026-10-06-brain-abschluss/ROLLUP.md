# Deadlock Brain: Wie es gebaut ist (Stand 07.10.2026, ca. 05:00)

## 1. Das Bild in einem Satz

Quellen werden in eine Postgres-Datenbank (Schema `brain`, eigene Instanz Port 5446) eingelesen, daraus entstehen geprüfte Fakten und Steckbriefe, ein Teil davon wird als „Korpus-Release“ eingefroren und aktiviert, und der Antwortdienst `brain-serve` (127.0.0.1:8788, `/v1/answer`) beantwortet Fragen von Discord und Twitch nur aus diesem aktiven Release.

## 2. Die vier Schichten

| Schicht | Was passiert | Wo im Code | Tabellen |
|---|---|---|---|
| 1. Quellen einlesen | Assets-API (Helden, Items, Fähigkeiten), Patchnotes (über den Patchnotes-Bot und Steam-News), Wiki (deadlock.wiki), Spieldateien (GameTracking-Git), Forum, Reddit, Statlocker, Google-Sheet, YouTube | `dbrain-sources` (9.150 Zeilen), `deadlock-brain-yt` | `source_documents`, `entity_snapshots`, `source_runs` |
| 2. Normalisieren und anreichern | Kanonische Entitäten mit Aliasen, Patchzeilen zerlegen (alt/neu/Einheit), Umbenennungen und Reworks, Sheet-Werte zuordnen. Bewusst ohne KI, deterministisch. | `dbrain-normalize`, `dbrain-enrich` | `entities`, `entity_aliases`, `patch_events`, `patch_event_enrichments`, `entity_lineage`, `legacy_entities`, `hero_stat_*`, View `patch_changes` |
| 3. Wissen ableiten und veröffentlichen | Steckbriefe je Held/Item mit Fakten und Quellenbindung, Patch-Story aus `patch_changes`, Quittungen, dann ein eingefrorener Korpus-Release | `deadlock-brain` (Wartung), `brain-storage::entity_profile`, `entity_derivation` | `entity_profile_entities_v1`, `entity_profile_facts_v1`, `entity_semantic_projections_v1`, `source_record_revisions/heads`, `entity_derived_receipts_v1`, `corpus_releases_v1` |
| 4. Antworten | `brain-serve` nimmt eine Frage, sucht lexikalisch die 6 besten Stellen im aktiven Release, lässt das Modell daraus formulieren und prüft, dass jede Aussage belegt ist. Ohne Beleg kommt `insufficient_evidence`. | `deadlock-brain-core`, `dbrain-retrieval` | liest nur den aktiven Release |

Daneben: `dbrain-reasoner` (17.700 Zeilen, Build-Reasoner), `dbrain-population` (Daten über gespielte Builds), `dbrain-builds`, `dbrain-learn` (8.700 Zeilen, Lernen aus Builds/Matches mit Fireworks).

## 3. Antwortweg heute

```
Discord (dl-bot: DM, Erwähnung, Hilfekanal)  ┐
Twitch (tb-bot: brain_chat_wiring)           ├─> brain-serve /v1/answer ─> aktiver Korpus-Release (lexikalisch, 6 Treffer)
                                              ┘                          └─> Modell gpt-6-luna über eigenen Codex-Abo-Proxy 127.0.0.1:18769
```

- Konfiguration: `~/.config/deadlock-brain/brain-serve.json`. Zeitlimit 60 s je Anfrage, Modell 55 s, höchstens 2 Modellrunden.
- Vertrauenswürdiger Discord-Consumer `dl-bot`/`discord` mit Rolle `bot.public`, eigener enger Faktenzugang `/mcp/public` (47 Kanäle, Voice-Zähler, Bot-Infotexte, keine Nachrichten).
- Twitch protokolliert jede Antwort in `twitch_analytics.tb_chat_brain_answers`.
- Aktiver Release heute: Doku-Release `docs-fe0fbb3c…`, also 61 Wartungsdokumente plus das Steam-Wartungswissen. **Keine Steckbriefe, keine Patchnotes, keine Patch-Story.** Deshalb beantwortet das Brain heute keine Spielfragen.

## 4. Was gebaut ist und wie gut es läuft

| Teil | Zustand | Kern des Problems |
|---|---|---|
| Antwortdienst und Transport | läuft | Twitch belegt zugestellt. Discord: Fix für Antworten mit Link oder Überlänge auf Bots-main, noch nicht deployt. |
| Spielwissen (Steckbriefe) | halb | 378 Profile und 64.342 Fakten liegen in der DB, sind aber nie in einen aktiven Release gekommen. Wartungslauf scheiterte im Dienst an der Git-Dateiliste. |
| Patch-Historie | halb | 33.209 Patchereignisse vorhanden, alter Python-Sync läuft noch alle 5 Minuten parallel zum Rust-Weg, Brain hängt eine Patch-ID hinterher. |
| Wiki | halb | 1,1 Mio. Wiki-Fakten vorhanden, ohne Quittungen nicht veröffentlichbar. |
| Build-Reasoner | halb | Rechnet aus Mechanik, Publish war an 100 Nach-Patch-Matches gebunden. Diese Grenze wird gerade entfernt (F). |
| Wartung/Tageslauf | kaputt | Timer-Fehler, Abbrüche, Fehler werden teils als Erfolg gemeldet (`|| echo`, Batch `failed > 0` mit Exit 0). |
| Sheet-Sync | kaputt | Fireworks-404, läuft aus einem fremden Checkout statt aus dem Release. |
| YouTube-Lernen, Forum | pausiert bzw. fehlgeschlagen | nicht für das Grundding nötig. |

## 5. Wo es unnötig kompliziert ist

1. **Zu viele Quellen für dieselben Spielwerte:** Assets-API, Spieldateien-Git, Wiki und Sheet liefern teils dieselben Zahlen. Entscheidung 07.10.: Spielwerte kommen aus dem lokalen 1:1-Spiegel der Deadlock-API je `client_version`; doppelte Parser werden abgebaut (E).
2. **Drei Patch-Importwege:** Patchnotes-Bot (`pg_patchnotes`), direkter Steam-News-Import (`pg_steam_news`) und Python-Legacy-Sync. Ziel: einer, mit dem API-Patchfeed als Auslöser.
3. **Matchdaten und Lernpfade:** Match-Lernen, Statlocker-Samples, Population. Entscheidung 07.10.: keine Matchdaten speichern.
4. **Viele Timer** (Wartung, Builddaten, Patchnotes, Wiki 4× täglich, Sheet, YouTube, Feeder, Forum) mit eigenen Fehlerbildern.
5. **Releaseprozess** mit Manifesten, Rootlayouts, Fences und exklusivem Besitz. Sicher, aber langsam; aktuell blockiert er sichtbar den Fortschritt.
6. **Antworten außerhalb des Brains:** Patchnotes-Bot formuliert selbst, dazu Concierge, Pitches, Moderationsmodelle. Liste in `A/EIN-BRAIN.md`.

## 6. Hebel zum Verbessern (Reihenfolge)

1. Steckbriefe und Patch-Story in den aktiven Release bringen (A, nach Recovery). Danach die fünf Prüffragen.
2. API-Spiegel als einzige Spielwerte-Quelle (E), danach Abbau doppelter Parser und des Legacy-Patchsyncs.
3. Retrieval verbessern: heute rein lexikalisch mit 6 Treffern. Für Fragen wie „wie stark ist Haze gerade“ besser direkt den Steckbrief der erkannten Entität laden statt Textsuche.
4. Timer zusammenlegen auf einen Tageslauf plus Patch-Auslöser, Fehler ehrlich melden.
5. Build als Skill hinter `/v1/answer` (F, dann A), ohne Matchgrenze.
6. Bot-eigene Antwortwege nach `A/EIN-BRAIN.md` auf das Brain umziehen.

## 7. Offene Punkte für den Nutzer

- Modell im Antwortdienst ist jetzt `gpt-6-luna` über den Codex-Abo-Proxy (vorher Fireworks DeepSeek V4.1 Flash). Umgestellt von der parallelen Sitzung. Bitte bestätigen, dass das so gewollt ist.
- Teilausschnitt von `TWITCH_ANALYTICS_DSN` in einem Werkzeug-Output (Fundort `AN_HAUPT-A.md:7`): Rotation entscheiden.
