# Paket C: Inventar und Löschgrenzen

Erhebung: 06.10.2026, 22:07 bis 22:14 Uhr CEST. Basis `origin/main`: `d6131cc52711a3e8b02d299704244f8d7dbdbce6`.

## Vollständige Namensliste

`BACKUP-SHAS.txt` enthält die Remote-Branches, lokalen Branches und Worktree-Pfade mit SHA und Commitdatum. Die Klassifikation unten gilt für jeden Namen der entsprechenden SHA-Zeile. Damit bleiben verschiedene lokale und entfernte Stände gleichnamiger Branches ausdrücklich getrennt. Insbesondere unterscheiden sich `codex/core-completion-20260925`, `codex/brain-functional-closeout-20260930`, `feat/wiki-hero-knowledge-20260924`, `feat/brain-rust-cutover-20260919` und `feat/brain-wiki-spielwissen-c-integration` lokal und remote.

Vorher: 106 Remote-Referenzen einschließlich `origin/HEAD`, also 105 echte Remote-Branches einschließlich main; 120 Worktrees; fünf offene Draft-PRs. Nach Anlegen des eigenen Worktrees und des parallel entstandenen A-Worktrees: 122 Worktrees. Der eigene Backup-Branch kommt zusätzlich auf origin hinzu.

## Klassifikation mit geprüftem Exit-Code

### Gemergt über Abstammung

Für diese SHA-Präfixe hat `git merge-base --is-ancestor <sha> origin/main` Exit 0 geliefert. Die Präfixe sind innerhalb des Backups eindeutig.

```
d6131cc5 54cfc032 0bb949a3 d020891b a85fd529 661b2a58 f0cd5add
dbd0e011 b687f613 f7fafa5f bc0100aa b6dc090c 740a35fb 511a347b
7da63018 022ed841 9da0449a 38c755aa 713ffb48 3bfe7216 ea82264f
e56e075d d440d8d4 fae8d26f 86d384c6 69015d92 a891ec30 e036fbde
6c985dad 04215897 89a72039 c5951b61 edd114db 3b86d3cb 72db8160
81d9fee7 084cdfc8 39710e32 be2aa6bd 64cbf3b0 1c362bca 96355922
```

### Gemergt über Patch-Identität

Hier ist der Abstammungstest Exit 1. `git cherry origin/main <sha>` hat keine `+`-Zeile geliefert. Zahl in der zweiten Spalte: Anzahl der `-`-Zeilen.

| SHA | cherry - |
| --- | --- |
| 633d15e2 | 4 |
| 1a5b2b33 | 1 |
| bd57e5cb | 21 |
| 0e208039 | 14 |
| 72f65294 | 20 |
| 31e003bb | 13 |
| 3aae8e0d | 1 |
| 1aff547a | 1 |
| 7d82ee32 | 1 |
| 2e9ade05 | 0 |
| fd00f24e | 2 |
| 5e64ca47 | 1 |
| 6cb3e6e8 | 1 |
| 934b8914 | 1 |
| bc97c7bd | 2 |
| 5c7e5b23 | 1 |
| c79c06d9 | 1 |

`2e9ade05` ist ein Merge-Stand ohne verbleibende Nicht-Merge-Patches; die beiden Worktrees haben aber noch ungesicherte Tests und bleiben bis zur WIP-Sicherung erhalten. Patch-Identität ist kein automatischer Nachweis für Konfliktauflösungen in Merge-Commits. Löschung dieses Stands wäre zusätzlich am Baum zu prüfen.

### Offen und ersetzt

Die übrigen SHA-Zeilen des Backups sind `offen`: Abstammungstest Exit 1 und mindestens eine `+`-Zeile. Inhalte, Diffgrößen und Entscheidungen stehen in `OFFEN.md`. Kein Stand wird allein aufgrund eines Committexts wie „superseded“ als `ersetzt` eingeordnet. In dieser ersten Runde ist kein eigenständiger Ersetzungsbeleg anerkannt.

Der eigene Backup-Branch ist geschützt. Sein nach dem Backup entstandener Commit `8aa4cdf8` ist absichtlich nicht auf main. Paket A, Paket B, main, der kanonische Checkout und heutige fremde Arbeit sind geschützt.

## Worktree-Zustand

Referenz ist jeder `worktree:`-Eintrag im Backup. Ohne Ausnahme unten: `git status --porcelain --untracked-files=normal` leer, kein zuordenbarer Prozess bei Erhebung. Auch saubere Worktrees können ignorierte Daten enthalten; diese werden vor einer Entfernung gesondert geprüft. Die Tabelle nennt Anzahl gemeldeter Statuspfade, nicht Anzahl einzelner Dateien innerhalb unversionierter Ordner.

| Worktree, normalerweise unter ~/.worktrees/ | Ungesicherte Statuspfade | Prozess oder Schutz |
| --- | --- | --- |
| /home/nathanael/repos/Deadlock-Brain | 17 | Kanonischer Checkout; PIDs 1784165, 1784218, 1814090, 2558697; nicht anfassen |
| Deadlock-Brain-meta-publish | 2 | Kein Treffer |
| Deadlock-Brain-paket-b-timer | 5 | Kein Treffer |
| Deadlock-Brain-rust-cutover-gate-20260930 | 10 | Kein Treffer |
| brain-a-abschluss-20261006 | 0 | Paket A; PID 1784106; nicht anfassen; nach Backup entstanden |
| brain-aufraeumen | 0 | Eigener Worktree; nicht löschen |
| brain-core-completion-20260925 | 2 | Kein Treffer |
| brain-fertig-q | 0 | PID 2075678; nicht anfassen |
| brain-fertig-r | 1 | Unversioniertes Cargo-target, kein Quellcode-WIP |
| brain-fertig-z | 1 | Unversioniertes Cargo-target, kein Quellcode-WIP |
| brain-final-integration-20260921 | 4 | Kein Treffer |
| brain-functional-closeout-20260930 | 6 | Kein Treffer |
| brain-g5-readonly-snapshot-20260930 | 2 | Kein Treffer |
| brain-global-toml-20260920 | 23 | Kein Treffer |
| brain-luna-abo-20261003 | 83 | Unter anderem FIFO-Nachweise und Snapshot; Datentypen vor WIP prüfen |
| brain-pr61-luna-integration-20261001 | 2 | Kein Treffer |
| brain-pr61-review-20260930 | 4 | Detached HEAD; WIP muss eigenen Sicherungsbranch bekommen |
| brain-release-completion-20260921 | 1 | Kein Treffer |
| brain-technical-closeout-20260929 | 6 | Kein Treffer |
| brain-v1-closeout-20261001 | 2 | Kein Treffer |
| brain-wiki-hero-knowledge-20260924 | 1 | Unversioniertes target, kein Quellcode-WIP |
| brain-wiki-spielwissen-a | 1 | Unversionierte Akte |
| brain-wiki-spielwissen-b | 1 | Unversionierte Akte |
| brain-wiki-spielwissen-c | 6 | Kein Treffer |
| brain-wiki-spielwissen-c-integration | 23 | Kein Treffer |
| brain-wiki-spielwissen-d | 1 | Unversionierte Akte |
| luna-brain-rust-cutover-20261001 | 2 | Kein Treffer |
| luna-native-brain-pr4-evidence-20261001 | 1 | Kein Treffer |
| /home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wiki-spielwissen-status-s | 1 | Unversionierte Akte |
| /tmp/brain-pr61-review-9635592 | 3 | Detached HEAD; WIP muss eigenen Sicherungsbranch bekommen |

Prozessprüfung: `/proc/<pid>/cwd`, `/proc/<pid>/exe` und offene Dateideskriptoren wurden auf den vollständigen Worktree-Pfad bzw. Unterpfade geprüft. Unlesbare Prozesse anderer Benutzer sind damit nicht ausgeschlossen. Vor Entfernung folgt `lsof +D` für den konkreten Kandidaten. Aktuelle Arbeit wird bei einem Treffer erhalten.

### Explizit gesperrte Worktrees

`git worktree list --porcelain` meldet Sperren für `brain-g5-readonly-snapshot-20260930`, `brain-g5-replay-deferred-20260930`, `brain-pre-g5-core-review-20260929`, `brain-pre-g5-harness-20260929` und `brain-technical-closeout-20260929`. Die Sperrgründe nennen den aktiven G5-Auftrag `562a877b` sowie eingefrorene Prüfstände. Worktrees und ihre Branches bleiben erhalten. Ein normaler Entfernungsversuch am Replay-Worktree wurde mit Exit 128 blockiert; kein Unlock und kein Force wurden versucht. Die Sperre ist stärker als ein fehlender Prozess-Treffer.

### Ignorierte Daten, die zunächst erhalten bleiben

`.core-test-pg/` sind mögliche Postgres-Cluster, `.core-test-logs/` und `.consumer-ci-reports/` Prüfartefakte. Ohne separaten Sicherungsbeleg werden diese nicht mit dem Worktree entfernt. Betroffen:

```
brain-c6-domain-kernel
brain-consumer-ci-completion-20260925
brain-core-completion-20260925
brain-db-pooling
brain-fix-c1-brain-serve
brain-fix-c11-storage-upgrade
brain-fix-c2-c3-retrieval
brain-fix-c7-c8-runtime-tooling
brain-functional-closeout-20260930
brain-pg-isolation-20260926
brain-pr61-review-20260930
brain-pre-g5-20260926
brain-pre-g5-harness-20260929
```

`brain-live-main` bleibt als main-Worktree erhalten und enthält außerdem `data/cache/`. `brain-pre-g5-finalize-20260929` enthält einen ungesicherten Graphify-Ordner und bleibt vorerst erhalten. Weitere Daten im kanonischen Checkout und in offenen Worktrees werden nicht entfernt. Cargo-target-Verzeichnisse mit zugehörigem Manifest gelten als nachbaubare Artefakte, nicht als Quellcode. Die im Status sichtbaren unversionierten target-Verzeichnisse werden nicht ungeprüft eingecheckt.

## Thread-Zuordnung aus den Akten

Kein Thread wurde kontaktiert oder fortgesetzt. Wo kein Aktenbeleg mit dem exakten Worktree-Pfad gefunden wurde, bleibt die Zuordnung `nicht belegt`. UUIDs eines ganzen Dokuments werden nicht als vermeintliche Worker-Zuordnung geraten.

| Worktree oder Gruppe | Beleg / Thread |
| --- | --- |
| Kanonischer Checkout | `.tasks/2026-10-04-spielwissen-steckbriefe/REGISTER.md`, D5 `6b670366-ae14-4fc8-bbfb-f6d1027a8371`; mehrere weitere historische Nutzer |
| brain-deploy-completion-20260918 | `.tasks/2026-09-18-brain-release/REGISTER.md`, `57ed8534-ea8b-41bb-8822-f9a310739238` |
| brain-release-20260918 | `.tasks/2026-09-18-brain-release/AUFTRAG.md`; keine eindeutige UUID belegt |
| brain-fertig-q, brain-fertig-r, brain-fertig-s, brain-fertig-z | `.tasks/2026-10-03-brain-fertigstellung/BRIEFING-Q.md`, `BRIEFING-R.md`, `BRIEFING-S2.md`, `BRIEFING-Z.md`; Zuordnung über Paketnamen, UUID nicht eindeutig |
| brain-patchnotes-rust-sync | `.tasks/2026-10-03-brain-fertigstellung/bereiche/p/BRAIN-KANDIDAT.md` |
| brain-independent-cleanroom-20260930-continued, brain-live-main, brain-pr61-review-20260930, /tmp/brain-pr61-review-9635592 | `.tasks/2026-10-03-brain-fertigstellung/bereiche/z/UNKLAR.md`; historische Übergabe, keine eindeutige Worker-UUID |
| brain-spielwissen-db-20261004 | Steckbrief-REGISTER, zuletzt A-F6 `de466002-34d7-4a37-be59-0928490d7adb` |
| brain-spielwissen-ausgabe-20261004 | Steckbrief-REGISTER, zuletzt B2 `c3d4590c-9284-4c97-9b6a-811b87739173` |
| brain-spielwissen-anschluss-20261004 | Steckbrief-REGISTER, C `d203d0ae-dac5-41fc-a89c-fb09f9c14f28` |
| brain-spielwissen-zuordnung-20261004 | Steckbrief-REGISTER, zuletzt A3-F7 `e204092e-a175-4553-9bd2-20d4a3392b80` |
| brain-spielwissen-stufe1-20261004 | Steckbrief-REGISTER, A3-F8 `4efcf649-ee6a-4a75-9b3d-a1f38579a1b6` |
| brain-spielwissen-wiki-20261004 | Steckbrief-REGISTER, A4 `5d5271da-b416-484a-a3a7-c2f79975f880`; historisch lesend wartend |
| brain-spielwissen-a-grundfix-20261004 | Steckbrief-REGISTER, A3-F5 `1b04cae9-d523-4ce0-99a1-9fb1349dbf96` |
| brain-spielwissen-c-gate1-20261004 | Steckbrief-REGISTER, C-F1 `ce7d523f-8dc4-4ec5-80a2-0882e66c4f9d` |
| brain-spielwissen-c-mainfix-20261004 | Steckbrief-REGISTER, C-F2 `8bfc043f-9d37-4d18-b27c-21b1f2f7ef91` |
| brain-spielwissen-c-refreshfix-20261004 | Steckbrief-REGISTER, C-F3 `8c47758b-2fa2-4ea1-982f-20af01d2e2aa` |
| brain-spielwissen-c-runtimefix-20261005 | Steckbrief-REGISTER, C-F4 `1c9c26a6-a1ad-46a1-b869-49609f4769bd` |
| brain-spielwissen-c-rightsfix-20261005 | Steckbrief-REGISTER, C-F5 `fdf59a02-5e5c-4f0b-84bf-98c0331be69f` |
| brain-spielwissen-c-fdstart-20261005 | Steckbrief-REGISTER, C-F6 `3a890b84-389d-4bad-9095-3e20595d6f1d` |
| brain-spielwissen-c-patchrole-20261005 | Steckbrief-REGISTER, C-F7 `5b797bfb-f2f3-4c8c-b175-21966b24bc6b` |
| brain-spielwissen-c-unitguard-20261005 | Steckbrief-REGISTER, C-F8 `8a3b0a80-b336-4f47-94aa-c3825a4828ab` |
| brain-spielwissen-c-htmlwithdraw-20261005 | Steckbrief-REGISTER, C-F9 `015b1bad-37ab-444f-a589-43eb5a7beaf2` |
| brain-spielwissen-c-rawwithdraw-20261005 | Steckbrief-REGISTER, C-F10 `2280c551-f4e5-4f76-be45-4f3631e92714` |
| brain-spielwissen-c-publishrestore-20261005 | Steckbrief-REGISTER, C-F11 `f156ebc7-6dee-4236-a8a3-a7ff73678c89` |
| brain-spielwissen-c-bindingscope-20261005 | Steckbrief-REGISTER, C-F12 `9f0a0d6a-a179-486d-b2b1-2b19e1c13be1` |
| brain-spielwissen-c-blobfacts-20261005 | Steckbrief-REGISTER, C-F13 `e25b95ea-ed88-4612-aa94-61bb36666f61` |
| brain-spielwissen-c-retrievalfixture-20261005 | Steckbrief-REGISTER, C-F14 `f6068203-09dd-433f-9ad7-7a9dd2bf7523` |
| brain-spielwissen-c-importerror-20261005 | Steckbrief-REGISTER, C-F15 `c8cbb50a-97b6-4142-803e-fc29a6bf2104` |
| brain-spielwissen-c-liveprofile-20261005 | Steckbrief-REGISTER, C-F16 `f16f5e0a-5976-46a0-80bd-e487978bb728` |
| brain-w5b-integration-20261004 | `.tasks/2026-10-03-brain-fertigstellung/welle1/REGISTER.md` |
| brain-d5-c-f14-release-e036fbde, brain-d5-c-f15-release-e56e075d | `.tasks/2026-10-03-brain-fertigstellung/welle1/w1/DOCS-TON-AN_D1.md`; Detached-Releasebäume |
| brain-discord-members-live-20261004 | `.tasks/2026-10-03-brain-fertigstellung/welle1/w5b/BRIEFING.md` |
| brain-wiki-spielwissen-a, -b, -c, -c-integration, -d | `.tasks/2026-10-03-wiki-spielwissen/HANDOFF-ROOT-20261003.md` und jeweilige Bereichsbriefings; mehrere historische Threads, kein eindeutiger aktueller Besitzer |
| wiki-spielwissen-status-s | `.tasks/2026-10-03-wiki-spielwissen/ORCHESTRATOR-KURZ.md` |
| brain-wiki-completion-20260925 | `.tasks/2026-10-03-wiki-spielwissen/bereiche/a/RECHERCHE.md`; keine UUID belegt |
| sheet-sync-exit-20261005 | `.tasks/2026-10-05-nebenfehler/REGISTER.md` und `BRIEFING-S.md` |

## Pull Requests

#3, #4, #5, #6 und #9 sind Drafts mit offenen SHA-Ständen aus `OFFEN.md`. In dieser Runde wird keiner geschlossen. Alle fünf wurden zur Nachverfolgung mit diesem T3-Thread verknüpft. Erst ein konkreter Ersetzungsbeleg oder eine Entscheidung aus Paket A erlaubt die nächste Aktion.
