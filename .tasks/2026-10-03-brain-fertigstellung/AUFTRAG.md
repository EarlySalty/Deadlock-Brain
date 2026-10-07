status: aktiv
Datum: 2026-10-03

ORCHESTRIERUNG[OR-1]: Stufe riesig | Schritt bau | Artefakt: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung

# Auftrag: Deadlock-Brain fertigstellen

Hauptorchestrator: Claude-Session `43a4886c-e135-484b-838a-0512d224a634` (T3, Projekt Documents).
Integrationsverantwortlicher für G5/G6: Paket Z.

## Nutzerziel

Der Nutzer will Deadlock-Brain nach wochenlanger Arbeit endlich fertig haben. Wörtlich: "alles muss fertig werden", ausdrücklich inklusive des Patchnotes-Bots in Rust. Fertig heißt produktiv: der Rust-Kern ist der laufende Pfad, die Consumer lesen aus ihm, die Python-Legacy-Writer sind abgeschaltet, Branches und Worktrees sind aufgeräumt.

## Ausgangslage (Stand 2026-10-03, belegt)

- `architecture/migration/STATUS.md` und `GATES.csv` auf origin/main: G1 und G4 lokal bestanden (1.011 Tests grün, Last 600/600), G0/G2/G3 teilweise, **G5 NEIN, G6 offen**. Der neue Kern ist nicht produktiv, der Bestandspfad läuft.
- 90 Worktrees, 59 ungemergte Remote-Branches, alte Draft-PRs #3 #4 #5 #6 #9 #46 in Deadlock-Brain.
- Patchnotes-Bot (`~/repos/Deadlock--Patchnotes-Bot`) ist Python (rund 13.500 Zeilen), läuft als User-Unit `deadlock-patchnotes.service` aus `~/.worktrees/patchnotes-live-main`. Dazu Brain-seitig `deadlock-brain-patchnotes-sync.timer` (Shellskript).
- Consumer: Bots #459 (Draft, Konflikt), Docs #4, 2nd-Brain #2 ungemergt; Twitch #984 gemergt.
- Laufender fremder Auftrag `.tasks/2026-10-03-wiki-spielwissen` (Codex /root, Pakete A bis D) baut Wiki- und Spieldatenimport. Nicht anfassen, Ergebnis wird von Z integriert.
- Fremde laufende Sol-Threads: Deadlock-Bots `a99dc9e9…` und `c0b1d111…`, Deadlock-Docs `14966995…` auf `codex/fix-c9-consumer-wiring`. Nicht hineinschreiben.

## Entscheidungen des Hauptorchestrators (gelten als Betreiberentscheidung)

1. **G5-Cutover: JA.** Der Nutzer hat "alles muss fertig werden" beauftragt. Cutover mit Rückweg (Legacy erst nach Live-Beweis abschalten, Rollback-Pfad dokumentiert).
2. **Provider:** Brain nutzt ausschließlich den bereits freigegebenen Provider: neuestes stabiles DeepSeek Flash bei Fireworks über den zentralen Weg. Kein neues Modell, kein neuer Anbieter. Shadow-Lauf damit fahren.
3. **Patchnotes-Übersetzung bleibt Perplexity `sonar-pro` mit dem alten Prompt aus `perplexity_requests.py` wortgleich** (Nutzervorgabe 01.10.). Kein Modellwechsel.
4. **Wiki:** Import im Rahmen der Wiki-Lizenz mit Attribution, Rechte- und Aufbewahrungsprüfung macht der Wiki-Auftrag. Z übernimmt dessen Ergebnis als Wiki-Pilot.
5. **Replay:** Teil von V1, soweit echte Demos über bestehende berechtigte Wege erreichbar sind. Sonst ehrliche Grenze im Abschlussbericht, das blockiert G5 nicht.
6. **Kein Python** für neuen Code. Python-Bestand nur als Verhaltensreferenz.

## Messbares Ende

- G5 und G6 in `GATES.csv` und `STATUS.md` auf bestanden, mit SHA und Live-Beweis.
- Patchnotes-Bot läuft als Rust-Binary, Python-Unit gestoppt und deaktiviert, ein echter Patch (oder Replay des letzten echten Patches im Vorschaumodus ohne Discord-Post) belegt identisches Verhalten.
- Steam-Build-Publish produktiv, mindestens ein echter Publish mit gemeldeter `hero_build_id`.
- Consumer (Bots, Docs, 2nd-Brain, Twitch) lesen live aus dem Rust-Kern.
- Legacy-Writer aus, Rückweg dokumentiert.
- Branches, Worktrees und Draft-PRs bereinigt (SHA-Backup vorher), nur noch Aktives offen.
- `ABSCHLUSSBERICHT.md` in diesem Ordner.

## Autorisiert

Eigene Commits, Merge nach main über den lokalen Merge-Gate (`gate_hook.py --review`), `git push origin HEAD:main`, Migrationen mit Bestandserhalt, bestehende Deploy-Wege, Dienstneustarts, Live-Prüfung, Löschen eigener und nachweislich gemergter oder überholter Branches/Worktrees nach SHA-Backup. Keine PRs neu anlegen, keine GitHub Actions, keine Käufe, keine Community-Posts (Discord-Posts des Patchnotes-Bots nur über den normalen Betrieb bei einem echten neuen Patch).
