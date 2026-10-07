# Paket F: Build-Veröffentlichung ohne Match-Grenze

Rolle: Blatt-Worker (GPT 6.1 Sol). Keine weiteren Threads.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Regeln: `AUFTRAG.md`. Steuerung: `VON_HAUPT.md` (Abschnitt 07.10. 05:05 ist maßgeblich).

## Nutzerentscheidung

Die Publish-Grenze von mindestens 100 Nach-Patch-Matches (`rust/crates/dbrain-reasoner/src/publish.rs` um Zeile 109, `meta.rs` `min_matches`, eingeführt in 7966f85e) fällt. Wir wollen Builds bauen, die anders sind als das, was gespielt wird; dafür gibt es oft noch keine Spiele. Matchdaten werden nicht gespeichert.

## Auftrag

1. Mit Graphify (Skill `code-suche`) alle Stellen finden, an denen Veröffentlichung, Planung oder Confidence an Matchzahlen, Buildfamilien-Abnahme (`post_patch_player_matches`, `eligible_for_planning`) oder Populationsdaten als Pflicht hängt. Populationsdaten bleiben höchstens Nebensignal, nie Voraussetzung.
2. Neue Veröffentlichungsbedingung: Build beruht nachweislich auf den Spielwerten des aktiven Patches, ist strukturell gültig (Slots, Kategorien, Skills) und wurde aus der Mechanik abgeleitet. Keine hand-getaggten Labels, keine Sondergewichte je Held, keine Referenz-Itemnamen im Code. Ein `confidence=Low`, das nur aus fehlenden Matches kommt, darf nicht mehr blockieren.
3. Bestehende Tests anpassen, fmt, Clippy, eigener `gate_hook.py --review`. Eigener Worktree `~/.worktrees/brain-f-publish` von aktuellem origin/main, Branch `feat/brain-build-publish-ohne-matchgrenze`.
4. Beweis: Warden-Build mit dem Reasoner aus dem eigenen Worktree bauen und mit `--publish` im Spiel veröffentlichen (Weg siehe `steam.steam_tasks` BUILD_PUBLISH_ORIGINAL), `hero_build_id` melden. Kein Brain-Release, keine Installation, kein Neustart, kein Tick, das macht `live_strecke`.
5. Main-Push erst nach Aufhebung des Release-Holds (`A/RELEASEFENSTER.md`); bis dahin Featurecommit auf origin sichern.

## Bericht

`AN_HAUPT-F.md`: Commit, Testzahlen, Gate wörtlich, `hero_build_id`, was am Build anders ist als die häufigsten gespielten Builds. Deutsch, ohne Em-Dashes, keine Code-Kommentare, Secrets nie im Klartext. Danach `python3 ~/Documents/tools/t3-thread.py settle --selbst`, wenn gemergt und belegt; sonst stehen bleiben.
