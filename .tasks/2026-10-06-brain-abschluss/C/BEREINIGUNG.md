# Paket C: ausgeführte Bereinigung

Stand: 06.10.2026 22:48 Uhr CEST. Vor dem ersten Löschen war `BACKUP-SHAS.txt` auf origin im Commit `8aa4cdf8` vorhanden. Die SHA- und Namensabdeckung der ursprünglichen Remote-Referenzen und Worktree-HEADs wurde programmgesteuert verglichen. Nach dem Snapshot entstandene A-Arbeit war geschützt.

46 einzelne Remote-Löschungen, 39 erfolgreiche Worktree-Entfernungen, ein wegen Sperre verweigerter Entfernungsversuch. Kein Force, kein Unlock, kein Reset, kein Stash. Lokale historische Branches wurden bewusst nicht gelöscht: Sie erhalten die ursprünglichen Commitobjekte und erlauben Wiederaufnahme aus dem SHA-Backup.

## Gelöschte ursprüngliche Remote-Stände

```
codex/consumer-ci-completion-20260925
codex/core-completion-20260925
codex/fix-brain-db-pooling
codex/fix-c1-brain-serve
codex/fix-c11-storage-upgrade
codex/fix-c2-c3-retrieval
codex/fix-c7-c8-runtime-tooling
codex/wiki-completion-20260925
feat/brain-fertig-s-20261003
feat/brain-patchnotes-rust-sync-20261003
feat/brain-spielwissen-anschluss-20261004
feat/brain-spielwissen-ausgabe-20261004
feat/brain-spielwissen-db-20261004
feat/brain-spielwissen-stufe1-20261004
feat/brain-spielwissen-zuordnung-20261004
feat/forum-wissen-20261003
feat/wiki-hero-knowledge-20260924
fix/brain-discord-members-live-20261004
fix/brain-spielwissen-a-grundfix-20261004
fix/brain-spielwissen-c-bindingscope-20261005
fix/brain-spielwissen-c-blobfacts-20261005
fix/brain-spielwissen-c-fdstart-20261005
fix/brain-spielwissen-c-gate1-20261004
fix/brain-spielwissen-c-htmlwithdraw-20261005
fix/brain-spielwissen-c-importerror-20261005
fix/brain-spielwissen-c-mainfix-20261004
fix/brain-spielwissen-c-patchrole-20261005
fix/brain-spielwissen-c-publishrestore-20261005
fix/brain-spielwissen-c-rawwithdraw-20261005
fix/brain-spielwissen-c-refreshfix-20261004
fix/brain-spielwissen-c-retrievalfixture-20261005
fix/brain-spielwissen-c-rightsfix-20261005
fix/brain-spielwissen-c-runtimefix-20261005
fix/brain-spielwissen-c-unitguard-20261005
fix/pre-g5-locked-dependencies-20260929
fix/token-db-local-20260930
integrate/brain-w5b-20261004
integrate/forum-wissen-20261003
integration/pre-g5-finalize-20260929
integration/pre-g5-review-20260926
migration/s12-wiki-preparation-20260924
migration/s13-external-sources-preparation-20260924
review/pre-g5-consumer-abnahme-20260929
review/pre-g5-consumers-20260929
review/pre-g5-providers-20260929
codex/brain-functional-closeout-20260930
```

`codex/core-completion-20260925` wurde danach mit einem anderen, offen gebliebenen lokalen WIP neu angelegt: `e2bd0b22`. Der gelöschte ursprüngliche Remote-SHA `64cbf3b0` liegt auf main. Deshalb sind im Nachvergleich 45 ursprüngliche Remote-Namen abwesend, nicht 46. Der neue Branch ist nicht gemergt und wird nicht als erledigt gewertet.

`codex/brain-functional-closeout-20260930` hatte ebenfalls verschiedene lokale und entfernte Stände. Der gemergte Remote-SHA `be2aa6bd` wurde erst nach Sicherung des lokalen WIP als `backup/wip-functional-closeout-20261006` entfernt.

## Entfernte Worktrees

Die Pfade beginnen mit `/home/nathanael/.worktrees/`, soweit nicht anders angegeben. Vor der Erhebung jeder Zeile: geprüftes SHA-Backup, Abstammungstest Exit 0 bzw. Exit 1 mit `git cherry` ohne `+`, leerer nicht ignorierter Status und `lsof -nP -t +D <pfad>` ohne Prozess-Treffer, Exit 1. Ignorierte wertvolle Daten wurden ausgesondert. Die Entfernungen liefen einzeln ohne Force.

```
brain-d5-c-f14-release-e036fbde
brain-d5-c-f15-release-e56e075d
brain-discord-members-live-20261004
brain-fertig-s
brain-forum-wissen
brain-independent-cleanroom-20260930-continued
brain-patchnotes-rust-sync
brain-pre-g5-consumer-review-20260929
brain-pre-g5-consumers-20260929
brain-pre-g5-dependencies-20260929
brain-pre-g5-providers-20260929
brain-s12-wiki-20260924
brain-s13-external-sources-20260924
brain-spielwissen-a-grundfix-20261004
brain-spielwissen-anschluss-20261004
brain-spielwissen-ausgabe-20261004
brain-spielwissen-c-bindingscope-20261005
brain-spielwissen-c-blobfacts-20261005
brain-spielwissen-c-fdstart-20261005
brain-spielwissen-c-gate1-20261004
brain-spielwissen-c-htmlwithdraw-20261005
brain-spielwissen-c-importerror-20261005
brain-spielwissen-c-mainfix-20261004
brain-spielwissen-c-patchrole-20261005
brain-spielwissen-c-publishrestore-20261005
brain-spielwissen-c-rawwithdraw-20261005
brain-spielwissen-c-refreshfix-20261004
brain-spielwissen-c-retrievalfixture-20261005
brain-spielwissen-c-rightsfix-20261005
brain-spielwissen-c-runtimefix-20261005
brain-spielwissen-c-unitguard-20261005
brain-spielwissen-db-20261004
brain-spielwissen-stufe1-20261004
brain-spielwissen-wiki-20261004
brain-spielwissen-zuordnung-20261004
brain-w5b-integration-20261004
brain-wiki-completion-20260925
sheet-sync-exit-20261005
token-db-local-20260930/Deadlock-Brain
```

## Nachher, erste Runde

105 echte ursprüngliche Remote-Branches einschließlich main, entsprechend 106 Referenzen mit `origin/HEAD`, werden zu 76 echten Remote-Branches. Darin stecken der neue Backup-Branch, neu gesicherte WIPs und parallel entstandene Arbeit. Von 120 ursprünglichen Worktrees wurden 39 entfernt; durch eigenen und parallel entstandene Worktrees werden aktuell 86 angezeigt. Fünf offene Draft-PRs bleiben offen.

21 WIP-Commits sind auf origin bestätigt. Sie sind Quellstand- und Aktenrettung, keine neue Implementierung. Keine Compiler-, Test- oder Produktfreigabe behauptet, kein Dienst geändert oder neu gestartet. Ignorierte Postgres-Cluster, Prüfberichte, nicht geprüfte Archive und offene Arbeit bleiben erhalten. `OFFEN.md` nennt Entscheidungen pro SHA; die Kontrollrunde läuft bis 07.10.2026 01:18 Uhr CEST.
