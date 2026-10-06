# Paket C: Erhalten ohne offene Remote-Branches

Ergänzender Auftrag vom 06.10.2026: Nicht laufende Erhaltungsstände werden als annotierte Tags `archiv/<branchname>` auf origin gesichert. Erst nach Prüfung von Tagobjekt und Ziel-SHA wird der entsprechende Remote-Branch gelöscht. Kein Main-Merge, keine Produktfreigabe. Der eigene Akten-Backup-Branch bleibt gemäß Briefing erhalten.

Vorher, zweite Runde: 76 echte Remote-Branches einschließlich main und 87 Worktrees. A hat 62 historische und 21 neue WIP-SHAs entschieden: 18 historische Stände verwerfen, 44 historische und 21 WIP-Stände erhalten. Aktuelle WIPs haben Vorrang vor ihren alten Elternständen.

## Vorgesehene Tag-Ziele

Jede Zeile ist `archiv/` plus Remote-Name. Vollständige Ziel-SHAs stehen in den ursprünglichen SHA-Backups; die Präfixe sind eindeutig. Verifikation und ausgeführte Löschungen folgen nach dem Push.

| Remote-Name | Ziel-SHA |
| --- | --- |
| backup/wip-functional-closeout-20261006 | 0e91b977 |
| backup/wip-pr61-review-20261006 | 28ac5c6f |
| backup/wip-pr61-scratch-20261006 | 7d60f287 |
| backup/wip-wiki-c-integration-20261006 | 74f6500e |
| codex/brain-deploy-completion-20260918 | 458d56d0 |
| codex/brain-luna-abo-20261003 | ee85ff14 |
| codex/brain-luna-dialogue-20261003 | 35c93de0 |
| codex/brain-pr61-pr9-integration-20261001 | 97515060 |
| codex/brain-release-20260918 | 9ead4617 |
| codex/core-completion-20260925 | e2bd0b22 |
| codex/fix-c10-real-replay-validation | 919c790f |
| codex/fix-pr61-publication-enforcement-20261001 | 27671409 |
| codex/luna-native/brain-pr4-evidence-20261001 | 1f4bb874 |
| codex/patch-understanding-evidence-20260918 | 9efeb1e4 |
| dependency/s00-planpaket-20260924 | 2734c2da |
| docs/reasoner-all-heroes | 8c98c63e |
| feat/brain-fertig-r-20261003 | e202beac |
| feat/brain-final-integration-20260921 | 7e4d5e6c |
| feat/brain-global-toml-20260920 | 38e4e559 |
| feat/brain-meta-publish-emojis | 36588add |
| feat/brain-release-completion-20260921 | 79f5a020 |
| feat/brain-runtime-release-20260921 | 457188a9 |
| feat/brain-s01-inventory-20260924 | 76205b5f |
| feat/brain-wiki-spielwissen-a | 80d1ca81 |
| feat/brain-wiki-spielwissen-b | b9be82ce |
| feat/brain-wiki-spielwissen-c | 209f1fbb |
| feat/brain-wiki-spielwissen-d | 60610fce |
| feat/serverguide-brain-20261003 | 849926b2 |
| fix/brain-rust-cutover-gate-20260930 | 3d74ae54 |
| fix/brain-spielwissen-c-liveprofile-20261005 | bb3ebf31 |
| fix/forum-xml-sitemap-20261003 | 6946269f |
| fix/paket-b-timer-20260930 | 8318690c |
| fix/patch-insights-evidence-20260918 | 089be38c |
| fix/reasoner-mechanics-completion | 3e7864a5 |
| fix/sheet-sync-secret-exec-20260924 | dcee4761 |
| integrate/serverguide-deploy-20261003 | 1d820bc2 |
| integration/brain-v1-closeout-20261001 | 4dbf3940 |
| integration/final-local-20260926 | 870c7a15 |
| integration/patch-analysis-evidence-20261001 | 49151073 |
| integration/pr61-luna-20261001 | 6c6dd8ba |
| luna/abschluss-brain-pr4-integrate-pr61-20261001 | aa2a051e |
| luna/abschluss-brain-pr9-20261001 | 7deebcb7 |
| luna/brain-codex-brain-deploy-completion-20260918-458d56d | 3d9098c6 |
| luna/brain-codex-brain-release-20260918-9ead461 | 79ec3857 |
| luna/finish-brain-core-completion-acl-20261001 | 60dec8f5 |
| luna/finish-brain-global-toml-20261001 | 40b43ac8 |
| luna/finish-brain-rust-cutover-20261001 | f34cd867 |
| sol/c9-brain/7bf0e0375ee34a00 | 357548d7 |
| worktree-wiki-spielwissen-status-s | 2f35c6b4 |
| codex/fix-pr61-publication-20261001 | 8d355fce |

## Schutz und Worktree-Prüfung

Kanonischer Checkout, main, eigener Backup-Branch, aktive A-/B-Fix1-Arbeit, Q/Z und die gesperrten G5-Stände werden nicht gelöscht. Prozess- und Statusprüfung wurde neu erhoben. Worktrees mit unversionierten Daten, Archiven, Postgres-Clustern oder Prüfberichten bleiben bestehen. Der Tag sichert Git-Inhalte, nicht ignorierte Dateien. Lokale historische Branches bleiben als zusätzliche Rückholreferenzen bestehen.

Ein erster Tag-Push mit Wildcard-RefSpecs wurde vom Hook wegen fehlender Eindeutigkeit vor Ausführung abgewiesen. Kein Push erfolgte, keine Übersteuerung. Der folgende Push mit expliziten Tag-RefSpecs war erfolgreich.

## Ergebnis, Snapshot 06.10.2026 23:59:22 Uhr CEST

50 annotierte Tags auf origin geprüft: Typ `tag`, Tagobjekt-SHA und dereferenzierter Commit-SHA stimmen jeweils lokal und remote überein. 49 Tabellenzeilen hatten noch einen Remote-Branch; diese 49 Branches wurden einzeln nach Tag-Prüfung gelöscht. `codex/fix-pr61-publication-20261001` ist ein zusätzlicher lokaler Erhaltungsstand ohne entsprechenden Remote-Branch. Sein Tag wurde ebenfalls geprüft.

Alle 65 Erhaltungsentscheidungen sind abgedeckt: 59 SHA-Zeilen sind in den geprüften Tags oder deren Geschichte erreichbar, sechs gehören zu geschützter A-/Kanon-/G5-/Q-/Z-Arbeit und wurden nicht angefasst. Ein Tag auf einem neuen WIP erhält zugleich seinen alten Elternstand. Kein Tagname wurde überschrieben.

18 weitere Remote-Branches wurden nach A-Verwerfungsentscheidung für ihren exakten aktuellen SHA gelöscht:

```
codex/brain-maintenance-storage-20261002
codex/fix-c6-domain-kernel-wiring
codex/fix-pr61-build-publish-url-20261001
feat/brain-s05-domain-audit-20260924
feat/brain-wiki-spielwissen-c-integration
fix/brain-readonly-query-cli
fix/pr61-remaining-review-20261001
fix/reasoner-build-without-live-schema-20260925
migration/07-provider-jev-20260924
migration/s03-storage-preparation-20260924
migration/s04-ingestion-preparation-20260924
migration/s06-retrieval-preparation-20260924
migration/s08-answer-kernel-20260924
migration/s11-cutover-preparation-20260924
migration/s14-replay-preparation-20260924
review/brain-s01-s09-20260924
docs/pre-g5-closeout-20260929
migration/s10-quality-performance-20260924
```

Vor diesen Löschungen: `merge-base --is-ancestor` jeweils Exit 1 und aktueller Remote-SHA gleich A-Entscheidungs-SHA. Die Stände wurden nicht als Main-Vorfahren dargestellt. Für den neueren lokalen Wiki-Integrations-WIP gilt dagegen der geprüfte Tag `archiv/backup/wip-wiki-c-integration-20261006`, nicht die Verwerfungsentscheidung des alten Remote-SHAs.

22 zusätzliche Worktrees entfernt, jeweils ohne Sperre, ohne uncommittierten Quellstand, ohne Prozess-Treffer bei vollständiger `/proc`-Prüfung und `lsof +D`, Exit 1:

```
/home/nathanael/.worktrees/Deadlock-Brain-rust-cutover-gate-20260930
/home/nathanael/.worktrees/brain-luna-dialogue-20261003
/home/nathanael/.worktrees/brain-pr61-luna-integration-20261001
/home/nathanael/.worktrees/brain-pr61-pr9-integration-20261001
/home/nathanael/.worktrees/brain-pr61-remaining-review-20261001
/home/nathanael/.worktrees/brain-runtime-release-20260921
/home/nathanael/.worktrees/brain-s03-storage-20260924
/home/nathanael/.worktrees/brain-s04-ingestion-20260924
/home/nathanael/.worktrees/brain-s05-domain-audit-20260924
/home/nathanael/.worktrees/brain-s08-answer-kernel-20260924
/home/nathanael/.worktrees/brain-spielwissen-c-liveprofile-20261005
/home/nathanael/.worktrees/brain-v1-closeout-20261001
/home/nathanael/.worktrees/brain-wiki-spielwissen-c
/home/nathanael/.worktrees/luna-brain-rust-cutover-20261001
/home/nathanael/.worktrees/luna-native-brain-pr4-evidence-20261001
/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Brain
/home/nathanael/Documents/.worktrees/brain-s01-inventory-20260924
/home/nathanael/repos/Deadlock-Brain/.claude/worktrees/wiki-spielwissen-status-s
/home/nathanael/repos/wt/brain-purpose-control
/tmp/brain-pr61-review-9635592
/home/nathanael/.worktrees/brain-pre-g5-docs-20260929
/home/nathanael/.worktrees/brain-s10-quality-performance-20260924
```

Die ersten 21 waren einschließlich ignorierter Dateien leer; S10 enthielt ausschließlich einen nachbaubaren Cargo-target-Ordner mit vorhandenem Cargo.toml. Keine wertvollen ignorierten Daten entfernt. Historische lokale Branches bleiben als Rückholreferenzen erhalten.

## Nachher

11 echte Remote-Branches, 66 Worktrees. Zweite Runde: 76 → 11 Branches und 87 → 66 Worktrees, einschließlich neu entstandener A-Arbeit. Gesamtauftrag: 105 → 11 Branches und 120 → 66 Worktrees. Tatsächlich ausgeführt: 113 Remote-Löschungen und 61 Worktree-Entfernungen über beide Runden; gleichnamige neu angelegte WIP-Branches erklären, warum Löschungen nicht dem Nettoverlust entsprechen.

GitHub hat durch die Löschung ihrer Head-Branches die fünf Draft-PRs #3/#4/#5/#6/#9 geschlossen. Einzelprüfung mit `gh pr view` bestätigt jeweils `CLOSED`, nicht `MERGED`. Ihre exakten Heads bleiben als die entsprechenden annotierten Tags abrufbar. Kein PR wurde als erledigte Produktimplementierung dargestellt.

Verbleibende Remote-Branches:

```
main
chore/brain-aufraeumen-20261006
feat/brain-a-abschluss-20261006
fix/brain-a-sheet-20261006
feat/brain-rust-cutover-20260919
feat/brain-fertig-q-20261003
feat/brain-fertig-z-20261003
fix/g5-replay-deferred-20260930
integration/technical-closeout-20260929
review/pre-g5-core-abnahme-20260929
verification/pre-g5-final-20260929
```

Eigener Worktree bleibt als Ablageziel des gemeinsamen `C`-Verweises erhalten. Kanonischer Checkout, A/B-Arbeit und fünf explizite G5-Sperren bleiben unangetastet. Die übrigen historischen Worktrees enthalten geschützte Daten, Archive, Prüfbelege oder wurden aus Vorsicht wegen nicht vollständig leerem Status erhalten; ihre entfernten Remote-Branches werden nicht benötigt, um die lokalen Dateien zu behalten.

Wiederaufnahme eines Git-Stands: `git fetch origin tag archiv/<branchname>`, danach aus dem Tag einen eigenen Arbeitsbranch erzeugen. Archive und unversionierte Daten sind weiterhin lokal und nicht Bestandteil des Tags. Kein Merge nach main, Deploy oder Neustart durch C.
