# A-R1: gemeinsamen aktuellen Brain-Main ausliefern

Native Build-/Deploy-Blattrolle. Auftraggeber Paket A `2c7de4c9-bac4-43ad-b91a-f8ac889f09b4`. Keine weiteren Agenten, T3-Threads, fremden Sessionnachrichten oder Reviewer. Kein Produktcodebau, Root-Akten nur A. Oberste Priorität Grundding, VON_HAUPT.md 07.10.01:45.

## Quelle und tatsächlich bestandene Prüfung

A hat die kombinierte Integration mit aktuellem Main fde910f6 normal geprüft: Source 1325/1325 Gitdateien identisch, Format/Compiler/striktes Clippy Exit 0, 126 passed/0 failed/0 ignored/0 filtered auf echter frischer privater PG; alle Jobs abgeschlossen und Scratch gestoppt. Geprüfter Commit 1e925d5f, nach reinem Commitnachrichten-/Attributionsabschluss 8d61a949. Gitbaum vorher/nachher exakt 500040ac54a6e5d2e1a60cbe7ea6f52bf1ad9ff6, kein Sourcebyte verändert. Regulärer Main-Push durch A Exit 0, kein Forcepush oder Hookbypass. Voller SHA zunächst über tatsächlichen HEAD messen. Belege /tmp/brain-a-integration-proof-20261007/main-fde-results.json, main-fde-source-final.json und main-fde-commands.json.

Eigene vollständig saubere detached Releasequelle `/home/nathanael/.worktrees/brain-a-release-20261007`, von aktuellem origin/main 8d61a949 angelegt. Nur diese Quelle für Releases verwenden, keine Sourceänderung, Gitmutation, fremden Artefakte oder geteilten Kanon. Integration-worktree bleibt A. Eigenes neues Bundle unter `/home/nathanael/.local/state/brain-a-release-20261007-8d61a949` verwenden. Keine bestehenden Bundles überschreiben. A startete vorhandenen `brain-release plan` bereits, Beleg /tmp/brain-a-integration-proof-20261007/main-release-plan.json, Task bvpetrz2u. Erst dessen tatsächliches Ergebnis prüfen, keinen Doppelplan/Build auf demselben Ziel.

## Vorhandener mechanischer Weg

Ops-Vertrag in eigenem `ops/brain-release/README.md` lesen. Vertrauenswürdiger vorhandener Rust-Helfer `/usr/local/libexec/brain-release`: plan, build, verify, anschließend eng erlaubtes sudo -- brain-release install. Er bindet Source/Gitbaum/Fingerprint, alle Workspace-Binaries, Werkzeugversionen und echte Hashes, serialisiert mit bestehender Sperre. Keine eigene Installation, keine Hook-/Allowlist-/Roothelferänderung, kein Herkunftsnachweis anhand Datei-Alter. Vorgeschriebene tatsächliche Verifikationsbuilds nicht überspringen. Buildargumente des unveränderten Helfers verwenden, keine eigenen kürzeren Budgets, keine neuen Targets außerhalb dessen Vertrags und keinen globalen CARGO_TARGET_DIR.

Vor jedem Install prüft der Helfer aktuelle Remote-main. Bei weitergelaufenem Main keine stale Installation und keine SHA-Absprachen mit fremden Sessions: gebundenes Bundle erhalten, exakte Grenze an A zurück. Ohne Standänderung normales hashgeprüftes Install und Neustart der tatsächlich betroffenen laufenden Brainservices über vorhandene Mechanik. Vorher genau PIDs/Exes/Units erfassen. Keine fremden PIDs, Timerabschaltung oder Restartschleife. Andere nicht auf Grundding einzahlende Sheet-/Site-/Modellarbeiten nicht starten. Unklare oder verweigerte privilegierte Schritte melden, nicht umgehen.

## Ziel und Livebeweis

Aktuell läuft brain-serve auf 10ebbb20, PID 2932843; F1b belegt ENTITY_PROFILE_REFRESH_FAILED, null Ableitungsquittungen/Git-Steckbriefdokumente. Der neue Main enthält den zusätzlichen vorhandenen Veröffentlichungs-/Fehlerklassifikationsfix fde910f6. Kein zweiter Profilfix nötig, bevor der neue reguläre Runtimepfad geprüft ist.

Nach Installation Manifest-/Binaryhash, beide Releasezeiger, tatsächliche PIDs/Exes ohne deleted, Fehlerjournal/NRestarts und normalen Antwortdienst prüfen. Installed brain-migrate check und check-entity-profiles dürfen nur lesend prüfen, keine neuen Migrationen ohne Scope. Danach bestehenden normalen priorisierten Stage-1-Maintenancepfad mit vorhandenen Credentials/ConfigWriter nutzen, damit der bereits beauftragte Profilpfad genau einmal auf neuer Runtime laufen kann. Keine manuelle DB-Zustandskorrektur oder gefälschten Quittungen. Fehler vollständig mit normaler tatsächlicher Klassifikation zurückgeben, nicht Exit 0 allein als Tick-/Profil-Erfolg ausgeben. Keine Modelle/Timeouts/Datenfreigaben verändern, Communityrohtexte oder Secrets ausgeben, keine öffentlichen Testnachrichten.

Neue eigene Belege /tmp/brain-a-main-release-proof-20261007/. Rückgabe mit exaktem Source-/Runtime-/Remote-SHA, Build-/Verifikationsexits und Dauer, Manifesthashes, PIDs/Exes, normalen Tick- und Profilquittungs-/Dokumentzuständen. Build, Install, Grunddingantwort und Zustellung getrennt. Branch-/Worktreecleanup führt A nach tatsächlichem Livebeweis aus.

Wache 20 Minuten, spätestens 30. Kein Abbruchbudget, warme länger laufende Compiler nicht stoppen oder kalt ersetzen. Aufträge vollständig bis tatsächlichem Exit nachhalten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-a-release-20261007
