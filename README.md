# Deadlock Brain

Ein neues, repo-taugliches Fundament fuer ein Deadlock-Wissenssystem.

Ziel: Quellen automatisch einsammeln, versioniert speichern und spaeter daraus
Patch-Historien, Item-/Hero-Timelines, Build-Analysen, Coaching-Antworten und
Patch-Reviews bauen.

## Quellen

- Deadlock Assets API: Heroes, Items und Raw-Daten von `assets.deadlock-api.com`
- Google Sheet: Hero-Stats/Scaling/DPS-Daten als CSV-Export
- Bestehende Patchnotes-DB: `changelog_posts` aus der zentralen Deadlock SQLite-DB
- Deadlock Wiki: bewusst vorsichtig, standardmaessig nicht als Bulk-Crawler
- Statlocker: optionale WPA-/Leaderboard-Meta-Signale, nicht in Bulk-Pulls

## Schnellstart

```bash
cd /home/naniadm/Documents/Deadlock-Brain
python3 -m venv .venv
. .venv/bin/activate
pip install -e .
deadlock-brain status
deadlock-brain pull assets
deadlock-brain pull sheet
deadlock-brain pull patchnotes
deadlock-brain pull forum --limit 100
deadlock-brain parse forum-claims --rebuild
deadlock-brain pull statlocker --kind wpa-items --patch patch_129989 --hero "Mo & Krill" --min-sample-size 50
deadlock-brain normalize entities --rebuild
deadlock-brain parse patchnotes --rebuild
deadlock-brain enrich patch-events --rebuild
deadlock-brain enrich lineage --rebuild
deadlock-brain enrich legacy-entities --rebuild
deadlock-brain normalize sheet-stats --rebuild
deadlock-brain context Shiv --pretty
deadlock-brain timeline Indomitable --descending --pretty
deadlock-brain review Indomitable --pretty
deadlock-brain quality --pretty
deadlock-brain lineage Stalker --pretty
deadlock-brain legacy --pretty
deadlock-brain item Refresher --pretty
deadlock-brain build "Mo & Krill" --pretty
deadlock-brain analysis save-review Stalker --pretty
deadlock-brain analysis run-minimax Indomitable --dry-run --pretty
deadlock-brain entities --type item --query Indomitable
deadlock-brain events --entity "Trophy Collector"
```

Ohne Installation geht es auch:

```bash
cd /home/naniadm/Documents/Deadlock-Brain
PYTHONPATH=src python3 -m deadlock_brain.cli status
PYTHONPATH=src python3 -m deadlock_brain.cli pull assets
```

## Forum schonend importieren

Das öffentliche Deadlock-Forum kann über die Sitemap in kleinen Wellen importiert
werden. Der Import läuft von alten zu neuen Thread-IDs, speichert die komplette
Thread-HTML-Seite als Rohquelle und legt zusätzlich strukturierte Thread- und
Post-Snapshots ab:

```bash
cd /home/naniadm/Documents/Deadlock-Brain
deadlock-brain pull forum --limit 100 --delay-seconds 1
```

Bereits gespeicherte Thread-IDs werden standardmäßig übersprungen. Dadurch kann
der Backfill beliebig oft mit kleinen Limits fortgesetzt werden. Mit
`--refresh-existing` lassen sich vorhandene Threads gezielt erneut abrufen.

Der Import hält sich an die Forum-Robots-Regeln: `/search/`, `/posts/`,
`/attachments/`, Login-/Account-Bereiche und private Inhalte werden nicht
gecrawlt. Attachment- und Bild-URLs, die in öffentlichen Thread-Seiten sichtbar
sind, werden nur als Referenz in den Post-Snapshots gespeichert; der Download
und die Bildanalyse sind ein separater, späterer Schritt.

Aus den gespeicherten Forum-Posts kann danach eine historische Claim-Schicht
gebaut werden:

```bash
deadlock-brain parse forum-claims --rebuild
```

Diese Claims sind absichtlich quarantined: Sie enthalten konkrete
Thread-/Post-Links, Datum, Autor-/Rollenhinweis, Vertrauensart und
Gültigkeitsstatus, gelten aber nicht als aktuelle Spieldaten. Alte Reports,
Bugfix-Antworten und Exploit-Risiken bleiben dadurch nachlesbar, ohne aktuelle
API-/Patch-/Sheet-Daten zu überschreiben oder Default-Antworten zu vergiften.

## Wiki bewusst schonend nutzen

Das Wiki wird nicht automatisch gecrawlt. Einzelne Seiten koennen gezielt gezogen
werden. Netzwerkzugriff fuer Wiki muss explizit erlaubt werden:

```bash
DEADLOCK_BRAIN_WIKI_ENABLED=1 deadlock-brain pull wiki Indomitable
```

Die Wiki-Quelle nutzt lokalen Cache und einen Mindestabstand zwischen Requests.
Fuer groessere Backfills sollten wir spaeter erst mit den Wiki-Betreibern klaeren,
welche Frequenz okay ist oder ob Dumps/API-Spiegel existieren.

## Statlocker gezielt nutzen

Statlocker wird bewusst nicht in `pull all` gezogen. Einzelne WPA-/Leaderboard-
Requests koennen gezielt importiert werden:

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind wpa-patches
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind wpa-items --patch patch_129989 --hero "Mo & Krill" --min-sample-size 50
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind leaderboard --leaderboard-page-size 100
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind leaderboard-player-matches --players-from-leaderboard 3 --matches-per-player 4
```

Der Build-Optimizer nutzt `statlocker_wpa_item` als schwaches Meta-Signal:
hilfreich fuer Item-Priorisierung, aber nicht als Ersatz fuer Hero-Verstaendnis,
Shop-Oekonomie und Item-Mechanik.

Fuer Match-Entscheidungen kann Statlocker gezielt Player-Profile, Recent
Matches, einzelne Matchdetails und Player-Build-Analysis speichern. Das ist
bewusst limitiert, damit kein Bulk-Crawl entsteht:

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli pull statlocker --kind leaderboard-player-matches --players-from-leaderboard 2 --matches-per-player 3 --include-match-details --include-build-analysis
PYTHONPATH=src python3 -m deadlock_brain.cli player list-matches --pretty --limit 10
PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-match <account_id> <match_id> --dry-run --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-next --dry-run --pretty --limit 3
DRY_RUN=1 PLAYERS_FROM_LEADERBOARD=2 MATCHES_PER_PLAYER=2 ./scripts/run_player_match_learning.sh
infisical run -- PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-match <account_id> <match_id> --pretty
infisical run -- ./scripts/run_player_match_learning.sh
```

`player analyze-match` baut einen MiniMax-Kontext fuer ein konkretes
Player-Match: Hero-Aufgabe, Item-Timing, Standard-Core vs. Adaptation,
Game-State-Hinweise, Risiken und wiederverwendbare Coaching-Regeln.
`player analyze-next` arbeitet die naechsten importierten Player-Matches als
Queue ab. `scripts/run_player_match_learning.sh` zieht vorher vorsichtig neue
Statlocker-Samples und startet danach diese Queue.
Das Script nutzt denselben Infisical-Zugriff wie `Deadlock-Bots`: Config aus
`/home/naniadm/.config/deadlock-bots/infisical.env` sourcen, Secrets per
Infisical-HTTP-API exportieren und dann MiniMax mit `MINIMAX_TOKEN_PLAN_KEY`
ueber den token-plan Endpoint ausfuehren. Ein lokales `infisical` Binary ist
dafuer nicht noetig.
Das Script setzt keine niedrige Output-Grenze; `MINIMAX_MAX_COMPLETION_TOKENS`
ist nur der technische API-Ceiling. Als Haenge-Schutz gibt es
`ANALYSIS_TIMEOUT_SECONDS` (Default 600 Sekunden pro Aufgabe, `0` deaktiviert).

## Datenablage

- `data/deadlock_brain.sqlite3`: lokale Staging-DB
- `data/raw/`: Rohdaten je Quelle
- `data/cache/`: HTTP-/Wiki-Cache

Die Staging-DB ist absichtlich noch generisch. Sie speichert Source Documents und
Entity Snapshots. Der Patchnotes-Parser erzeugt daraus strukturierte
`patch_events`. Die Entity-Normalisierung erzeugt daraus zusaetzlich
kanonische `entities` und `entity_aliases`. Weitere deterministische
Verarbeitungsschritte bauen `patch_event_enrichments`, `hero_stat_profiles` und
`hero_stat_values`.

## Patch-Events

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli parse patchnotes --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli events --entity Indomitable
PYTHONPATH=src python3 -m deadlock_brain.cli events --entity Shiv --source-kind steam
```

Forum-Posts und Steam-Announcements werden gleichwertig verarbeitet. Die URL wird
nur als Source-Metadatum klassifiziert (`forum`, `steam`, `other`), damit die
Timeline nicht an eine einzelne Plattform gekoppelt ist. Wenn vorher
`normalize entities` gelaufen ist, nutzt der Parser die Alias-Tabelle fuer
genauere Event-Typen wie `item` und `ability`.

## Enrichment, Sheet-Stats und Kontext

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli enrich patch-events --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli enrich lineage --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli enrich legacy-entities --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli normalize sheet-stats --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli context Indomitable --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli context Shiv --limit-events 8
PYTHONPATH=src python3 -m deadlock_brain.cli lineage Stalker --pretty
```

`enrich patch-events` zerlegt Patchzeilen weiter in Stat-Namen, alte/neue Werte,
Einheiten, Ability-Kontext und einfache Confidence-/Flag-Daten. `normalize
sheet-stats` nimmt die Google-Sheet-Snapshots und legt Hero-Stats als einzelne
Key/Value-Zeilen ab. `context` fuehrt Entity, Aliase, Patch-Historie,
Enrichments und Sheet-Stats zu einem kompakten Datenpaket zusammen, das spaeter
direkt in Review-, Coaching- oder Build-Antworten gehen kann.

`enrich lineage` extrahiert Rename-/Rework-Beziehungen aus Patchnotes, z.B.
`Backstabber -> Stalker`, `Curse -> Cursed Relic` oder Ability-Renames wie
`Kudzu Bomb -> Entangling Thorns`. `context`, `timeline` und `review` nutzen
diese alten und neuen Namen als gemeinsame Lookup-Kette.

`enrich legacy-entities` modelliert alte oder entfernte Namen, die nicht mehr in
der aktuellen API vorkommen und nicht zwingend ein Rename sind, z.B. alte
Sonderitems oder Tippfehler aus historischen Patchnotes. Fragwuerdige Parser-
Subjekte werden markiert statt blind als echte Entity behandelt.

## Timelines, Reviews und Qualitaet

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli timeline Shiv --descending --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli review Indomitable --limit-events 80
PYTHONPATH=src python3 -m deadlock_brain.cli review Indomitable --prompt-only
PYTHONPATH=src python3 -m deadlock_brain.cli item Refresher --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli build "Mo & Krill" --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli analysis save-review Indomitable --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Indomitable --dry-run --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli analysis list --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli learn import-steam-builds --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli learn list-builds --hero Haze --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli learn analyze-build 28 --dry-run --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli learn analyze-next --dry-run --pretty --limit 3
PYTHONPATH=src python3 -m deadlock_brain.cli player list-matches --pretty --limit 10
PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-match <account_id> <match_id> --dry-run --pretty
PYTHONPATH=src python3 -m deadlock_brain.cli player analyze-next --dry-run --pretty --limit 3
PYTHONPATH=src python3 -m deadlock_brain.cli quality --pretty
```

`timeline` gruppiert Patch-Events nach Patch und haengt pro Zeile eine
deterministische Impact-Einschaetzung an, z.B. `numeric_buff`, `numeric_nerf`,
`rework`, `functional_change`, `added`, `removed` oder `bugfix`.

`review` baut noch keinen Discord-Post und ruft auch kein KI-Modell auf. Es
verdichtet Entity, aktuelle Sheet-Hints, Timeline-Signale, Quellen und offene
Fragen in ein KI-taugliches Datenpaket plus deutschen Prompt-Entwurf.

`item` zeigt strukturierte Item-Daten aus der Deadlock Assets API: Cost, Tier,
Shop-Slot, Aktiv/Passiv, Beschreibung, relevante Properties und erkannte
Item-Archetypen wie `lane_farm`, `kill_setup`, `core_scaling`, `support` oder
`luxury`.

`build` erzeugt einen ersten deterministischen Build-Vorschlag fuer Heroes. Der
Scorer nutzt Deadlock-API-Itemdaten, Hero-`item_draft_bucketing`,
Shop-Bonus-Kurven bis zum wichtigen 4800-Souls-Fenster, Sheet-Hints,
Patchsignale und optional gecachte Wiki-Zusammenfassungen. Die Ausgabe zeigt
den erkannten Damage-Plan (`weapon`, `spirit`, `hybrid`, `utility`),
priorisierte Scaling-Stats, fruehe Economy-Kaeufe, Core-Impact, Situational
Counter und Late/Luxury, damit ein Item wie `Refresher` nicht ohne harte
Begruendung im Core landet. Die Build-Ausgabe ist aktuell ein erklaerbarer
Kandidaten-Plan, keine finale Copy-Paste-Buyorder.

`analysis save-review` speichert diesen Review-Kontext persistiert in
`analysis_notes`, inklusive Kontext-Hash, Prompt-Version, Quellen und optionalem
fertigem Analyse-Text.

`analysis run-minimax` baut aus dem Review-Kontext einen MiniMax-Request,
ruft das OpenAI-kompatible Chat-Completions-Endpoint auf und speichert die
Antwort wieder in `analysis_notes`. `--dry-run` zeigt nur den Request-Aufbau
ohne Netzwerkaufruf. Der Token wird aus `MINIMAX_API_KEY` oder
`MINIMAX_TOKEN_PLAN_KEY` gelesen und nie ausgegeben.

`learn import-steam-builds` importiert vorhandene Steam/GC-Hero-Builds aus der
zentralen Bot-DB in `learned_builds`. Top-Rank-Builds werden als schwache
positive Trainingslabels gespeichert. `learn analyze-build` baut daraus einen
MiniMax-Analyseauftrag, damit das Brain lernt, warum ein Build gut,
situativ oder fragwuerdig ist. `learn analyze-next` nimmt automatisch die
naechsten noch nicht mit MiniMax analysierten Builds.

`player analyze-match` ist die Match-Entscheidungs-Schicht dazu. Es nutzt
Statlocker-Player-Matches als beobachtete Samples und speichert MiniMax-Notizen
in `player_match_decision_notes`, damit das Brain nicht nur Build-Listen lernt,
sondern auch Timing, Adaption und Match-Kontext.

```bash
DRY_RUN=1 PLAYERS_FROM_LEADERBOARD=2 MATCHES_PER_PLAYER=2 ./scripts/run_player_match_learning.sh
infisical run -- ./scripts/run_player_match_learning.sh
```

```bash
DRY_RUN=1 LIMIT=3 ./scripts/run_build_learning.sh
infisical run -- ./scripts/run_build_learning.sh
HERO=Haze LIMIT=2 infisical run -- ./scripts/run_build_learning.sh
```

`scripts/run_build_learning.sh` synchronisiert zuerst Steam/GC-Builds aus der
Bot-DB und startet danach den Analyse-Queue-Lauf. `DRY_RUN=1` speichert nur den
Kontext ohne MiniMax-Aufruf.

Automatisch geht es z.B. per cron in der Umgebung, die
`MINIMAX_TOKEN_PLAN_KEY` aus Infisical bereitstellt:

```cron
0 */2 * * * cd /home/naniadm/Documents/Deadlock-Brain && infisical run -- ./scripts/run_build_learning.sh >> /home/naniadm/Documents/Deadlock-Brain/data/build_learning.log 2>&1
```

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Indomitable --dry-run --pretty
infisical run -- PYTHONPATH=src python3 -m deadlock_brain.cli analysis run-minimax Indomitable --pretty
```

`quality` prueft lokale API-/Patch-/Sheet-Daten auf Alias-Kollisionen, fehlende
Entity-Matches, Low-Confidence-Enrichments, Sheet-Zeilen ohne Entity und
general-Patchzeilen, die bekannte API-Entities erwaehnen.

## Smoke Check

```bash
./scripts/smoke_check.sh
```

Der Smoke-Check kompiliert die Python-Module und prueft die wichtigsten JSON-
Ausgaben fuer `context`, `timeline`, `review`, `lineage`, `legacy`, `item`,
`build` und `quality`.

## Entity-Normalisierung

```bash
PYTHONPATH=src python3 -m deadlock_brain.cli normalize entities --rebuild
PYTHONPATH=src python3 -m deadlock_brain.cli entities --type item --query Indomitable
```

Die Normalisierung liest die bestehenden `entity_snapshots` der
Deadlock Assets API. Aktive, spielbare Heroes werden als `hero` klassifiziert;
disabled/in-development/test Heroes landen in `hero_internal`. Normale
Shop-Upgrades werden als `item` klassifiziert, T5-/Sonder-/Modus-Items als
`item_special`. Aktive Hero-Faehigkeiten landen in `ability`; Future-/interne
Faehigkeiten in `ability_internal`. Waffen und weitere technische Assets landen
konservativ in `weapon_or_internal`.
