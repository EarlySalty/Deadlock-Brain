# Fix-Briefing: build-reasoner (Paket S, Fixrunde 2, Live-Befund)

[Orchestrator] Fixer für Paket S nach dem Live-Deploy. Der Merge aba7636 ist
in Steam-Bot main, steam-core und steam-core-2 laufen seit 2026-09-12 21:26
damit. Der erste echte Lauf zeigt zwei Fehler, die im Test nicht sichtbar
waren (Tests nutzen simulierte GC-Antworten).

- Worktree: `/home/nathanael/.worktrees/steam-bot-autoren-scan-2` (existiert,
  Branch `fix/autoren-scan-live` ab main aba7636, ausgecheckt)
- Intent-Thread: `33a32f58-476b-4a67-99cc-8f6c1e8f7001`
- Du bist der einzige Thread für diese Fixrunde. Keine Unter-Threads oder
  Unter-Agenten spawnen.
- Kontext: `BRIEFING-S.md`, `FERTIG-S.md`, `REVIEW-S.md` im Ordner
  `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-09-12-build-reasoner/`.

## Live-Befund, wörtlich aus DB und Journal

1. Der Scheduler hat um 19:26:32 UTC `BUILD_CATALOG_CYCLE` Task 4047117
   eingeplant (Journal: "BUILD_CATALOG_CYCLE eingeplant task_id=4047117").
   Um 19:36:33 UTC hat der Runner ihn abgeräumt: "Hängenden Task bereinigt
   (FAILED)", `error = task stale: keine Antwort innerhalb 600s`
   (`task/runner.rs:34` `DEFAULT_STALE_AFTER_S = 600`). Ein zweiter, per API
   eingereihter `BUILD_CATALOG_CYCLE` (Task 4047138, 19:26:54 UTC) steht
   seitdem auf PENDING und wird nicht aufgenommen.
2. `tierlist.watched_build_authors`: alle 13 Autoren haben `last_checked_at`
   zwischen 19:26:35 und 19:27:45 UTC (also rund 6 Sekunden je Autor, die
   Rückschreibung aus Fixrunde 1 funktioniert), aber jeder Autor steht auf
   `error` mit `0 builds from 0 heroes; 0 new, 0 updated, 1 errors;
   {"authorAccountId":<id>,"error":"Response code: 0"}`. Auch für
   1650097169, dessen Warden-Build 779996 laut deadlock-api existiert.
3. `tierlist.hero_build_sources`: bestehende Warden-Builds (357243, 419580)
   haben `last_seen_at` 19:34 UTC, der Helden-Teil des Zyklus lief also
   weiter und hat die 600 Sekunden aufgebraucht; Build 779996 ist nicht in
   der Tabelle.
4. Im Journal gibt es zwischen 19:26:32 und 19:36:33 UTC keine einzige
   Zeile je Autor oder je Held: der Lauf ist von außen unsichtbar.

## Was du tust (Ursache, nicht Symptom)

1. `Response code: 0` bei `CMsgClientToGCFindHeroBuilds`: Klären, was der GC
   mit Ergebnis 0 meint (Proto `citadel_gcmessages_client.proto`, Enum der
   Antwort; 0 ist weder `k_eSuccess = 1` noch `k_eTooBusy = 2`). Prüfe die
   Anfrage gegen das, was der bestehende Helden-Scan (der Builds findet)
   sendet: fehlt bei der Autoren-Suche ein Pflichtfeld (`hero_id`,
   `language`, `search_text`), oder wird die Antwort falsch ausgewertet
   (Feld `result` nicht gesetzt, obwohl `results` gefüllt ist)? Ein
   gezielter Test gegen den echten GC über den laufenden steam-core ist
   erlaubt, wenn er lesend ist und maximal eine Handvoll Anfragen stellt
   (Rate-Limit Code 5 beachten).
2. Laufzeit: der Zyklus muss unter dem Stale-Limit bleiben oder in Einheiten
   zerfallen, die es einhalten. Vorschlag: je Autor ein eigener Task
   (`DISCOVER_WATCHED_BUILDS` mit `author_account_id` im Payload), der Zyklus
   plant sie nur ein; alternativ Fortschritt per Heartbeat (`updated_at`)
   melden, wenn der Runner das als Lebenszeichen wertet (prüfe
   `runner.rs:214-244`, was genau als stale zählt). Kein pauschales
   Hochsetzen von `DEFAULT_STALE_AFTER_S`.
3. Sichtbarkeit: eine Journal-Zeile je Autor (Ergebniscode, Zahl der Builds)
   und eine je Zyklus-Ende, damit der nächste Lauf ohne DB-Abfrage lesbar
   ist. Keine Zeile je Held.
4. Der hängende PENDING-Task 4047138: erkläre, warum er nicht aufgenommen
   wird (Dedup, Lane, Lease), und ob er nach dem Fix von selbst läuft.

## Regeln

- Nur Katalog-, Discovery- und Persistenzpfad. Keine Lobby-, Rank-,
  Invite-, Publish-Änderungen, keine Migration, keine neuen Poll-Schleifen,
  kein Python. Keine Code-Kommentare.
- Toolchain `export PATH=/home/nathanael/.cargo/bin:$PATH`. Tests:
  `SQLX_OFFLINE=true cargo test -p steam-core -p steam-persistence
  --manifest-path rust/Cargo.toml` (Baseline 174 bestanden) und die
  Docker-Katalogtests über `rust/scripts/central_test_db.sh` mit
  `--features testing -- task::handlers::builds::catalog::tests
  --include-ignored` (Baseline 26 bestanden). `.sqlx`-Cache nachziehen bei
  neuen Queries. Kein `--release`.
- Selbstprüfung vor der Fertigmeldung. Nur `fix/autoren-scan-live`
  committen und pushen, nie main. Commit-Trailer `Co-authored-by: <dein
  Modell> <modell@local>`. Echte Umlaute, keine Gedankenstriche.

## Fertigmeldung

In diesem Thread und als `FERTIG-S2.md` im Task-Ordner: Ursache je Befund
mit Beleg (Proto-Zeile, Code-Zeile, GC-Antwort), Fix mit Datei:Zeile,
Commit-SHA, Testzahlen (Baseline und Endstand), was der Delegator nach dem
Deploy sehen muss (Journal-Zeilen, DB-Abfrage für Build 779996).
