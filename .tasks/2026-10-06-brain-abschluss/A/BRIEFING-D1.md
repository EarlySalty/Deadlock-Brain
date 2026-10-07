# A-D1: vorhandenen Discord-Grundding-Consumer als Main-Release bauen

Native Build-/Deploy-Blattrolle, Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Keine weiteren Agenten/Threads oder fremden Sessionnachrichten, keine Reviewer, kein Produktcodebau. Root-Akten schreibt A. Neue Nutzerpriorität aus VON_HAUPT.md 07.10.01:45 gilt.

## Verifizierter Stand und Eigentum

Eigener vorhandener Worktree `/home/nathanael/.worktrees/brain-a-discord-20261006`, Branch fix/brain-a-discord-20261006, HEAD e18f522226f8e2dec5a1c03fe97c2aba3200c8d1. Arbeitsbaum sauber, A hat Main e1f11614 normal integriert und e18f5222 regulär auf origin/main gepusht, Exit 0. B-Invitefix bleibt erhalten. 38 echte Bibliothekstests mit privater PG, vollständiger dl-bot-Compiler und normales Clippy Exit 0. Strikter Clippy an dl-central-db/platform_connections.rs:30 rot, keine Warnungsunterdrückung. Regulärer Kombinationsgate ALLOW nach technischem automatischem Rückfall auf Opus. Belege A/F2C-RUECKGABE.md und /tmp/brain-a-discord-integration-proof-20261007/.

Kein Worker bearbeitet diesen Worktree weiter. Nur Release-/Beleg-/Stagingarbeit hier, keine Sourceedits, Commits, Main-Pushs oder Migrationen. A-E4 gehört der andere Invite-Bots-Worktree, nicht anfassen. Kanon, B-Worktrees und andere Quellen tabu.

## Vorhandener Betriebsweg

Live nach B-Auslieferung: deadlock-bot-rust.service, dl-bot PID 2848890, Release e1f11614. Nicht den fehlenden Alias dl-bot.service als Ausfall melden. Vor jedem Eingriff tatsächliche Exe/Hashes/Unit messen.

Aktuellen bestehenden Weg aus B-LIVE.md und AN_HAUPT-B.md im Aufgabenordner gezielt wiederverwenden. B hat belegt: alle acht SHA256SUMS-Einträge prüfen, Stage unter ~/.local/state/bots-release-stage-<SHA>, Releasewurzel /opt/deadlock/bots/releases/<SHA>, vorhandene atomare Aktivierung unter /run/lock/deploy-deadlock-bots-release.lock, Restarthelper /usr/local/bin/bot-restart. Kein vorhandener aktueller Wrapper in scripts/ops/usr-local-Verzeichnissen gefunden, keinen alten target/release-Helfer reaktivieren oder zweite Mechanik erfinden. Falls ein vorgeschriebener Schritt tatsächlich nicht über diesen belegten zulässigen Weg erreichbar ist, konkrete Grenze melden, keine Allowlist/sudo-/Hookumgehung.

## Auftrag

Jetzt benötigtes dl-bot-Release und alle tatsächlich von dl-brain transitiv betroffenen Workspace-Binaries aus diesem eigenen Main-Quellstand bauen. Abhängigkeiten vor dem Inventarentscheid über Graphify und tatsächliche Cargo-Metadaten prüfen. Keine Annahme, dass dl-web betroffen oder unbetroffen ist. Unveränderte installierte Artefakte nur übernehmen, wenn transitive Quell-/Lock-/Buildvertragidentität gegen den aktuellen Commit und echte Hashes belegt sind. Datei-Alter genügt nicht. Bestehenden eigenen warmen Target verwenden, sofern der Quellstand gesichert ist. Kein fremder Target/Build und kein globales CARGO_TARGET_DIR.

Angesichts Hostlast höchstens ein Cargo-Job, normaler Releasebau im Hintergrund bis zum tatsächlichen Exit. Keine Kurzlimits, 20-/30-Minuten-Wache ist kein Abbruchbudget. Keine lebenden kalten Compiler abbrechen, keinen Target löschen. Source muss während des Baus unverändert bleiben.

Vor jeder Aktivierung aktuellen origin/main erneut prüfen. Hat Main inzwischen einen neueren Stand, keine stale Binary installieren: gebundenen Bau/Target erhalten, konkrete SHA-Grenze an A zurückgeben. Kein Warten auf fremde Builds oder SHA-Absprachen. Keine ungeprüfte Main-Weiterentwicklung während eines Baus.

Wenn der belegte vorhandene Releaseweg eindeutig zulässig und die Quelle weiterhin aktuelles origin/main ist, normale SHA-/Hash-/flock-geprüfte Installation, betroffene Dienste über vorhandenen Restarthelper neu starten und tatsächlich live prüfen. Keine Community-Testnachricht, keine Daten-/Modell-/Konfigänderung. Bei unklarem oder verweigertem privilegiertem Schritt stoppen und gebundenes Bundle melden, nicht eigene Deploywerkzeuge bauen.

Eigene neue Belege /tmp/brain-a-discord-release-proof-20261007/. Abschluss mit exaktem Source-/Remote-/Runtime-SHA, Builddauer/Exit, tatsächlichen Artefakthashes, PID vorher/nachher, Exe ohne deleted, Fehlerjournal/NRestarts, Gateway/MCP-Anschluss und funktionalem vorhandenen Endzustand. Build, Installation und Zustellung getrennt melden. Cleanup und Root-Protokoll führt A nach regulärem Nachweis aus.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-discord-20261006
