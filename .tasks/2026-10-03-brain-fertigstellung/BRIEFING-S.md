status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/steam-publish-fertig

[Orchestrator] Paket S: Steam-Build-Publish produktiv

Zuerst `GEMEINSAM.md` in diesem Ordner lesen, sie gilt vollständig.

## Ziel

Der Steam-Build-Publish-Provider ist laut `architecture/migration/STATUS.md` integriert (Brain PR #73, Diagnosefix PR #82, `f509f85e`), aber nie ausgerollt und nie echt publiziert. Am Ende läuft der Publish-Endpunkt im Steam-Bot produktiv, Brain ruft ihn über den Vertrag `brain.build_publish.v1` auf, und ein echter Build ist über diesen Weg im Spiel veröffentlicht.

## Bestand

- Steam-Bot `~/repos/Deadlock-Steam-Bot`, Task-Typ `BUILD_PUBLISH_ORIGINAL` in `steam.steam_tasks` liefert `hero_build_id` (Memory `reasoner-build-publish-weg`). Deploy-Weg in Memory `steam-bot-deploy-und-test-weg`.
- Brain-Seite: `rust/crates/brain-feeds` Build-Publish-Teil, `deadlock-brain reason build --publish`.
- Offener G5-Blocker laut Memory: "Publish-Endpunkt im Steam-Bot". Prüfe per Graphify, ob der Endpunkt schon auf main ist oder auf einem Branch liegt, und übernimm ihn.

## Eigentum

Steam-Bot-Repo im eigenen Worktree; in Deadlock-Brain nur der Build-Publish-Teil von `brain-feeds` und die zugehörige CLI. Migrationen für Steam-Tabellen laufen zentral in Deadlock-Bots `dl-central-db/migrations`: dort nur neue Migration nachlegen, nie eine angewandte ändern.

## Arbeitsstand

Worktree `~/.worktrees/steam-publish-fertig`, Branch `feat/steam-publish-fertig-20261003` von Steam-Bot `origin/main`. Für den Brain-Teil zweiter Worktree `~/.worktrees/brain-fertig-s`, Branch `feat/brain-fertig-s-20261003`.

## Beweisziel

Ein echter Publish über Brain, Endpunkt und Steam-Bot mit Status DONE und gemeldeter `hero_build_id` (Held nach Wahl, z. B. Warden mit dem aktuellen Reasoner-Build). Dienste neu gestartet, Journal sauber, Fehlerpfad (Steam-GC gedrosselt) sichtbar statt still.

## Routing

Paket S, Versuch 1, Produzent `teil-s`. Rest siehe GEMEINSAM.md.
