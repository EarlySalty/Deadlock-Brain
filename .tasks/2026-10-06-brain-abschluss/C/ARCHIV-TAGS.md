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

## Schutz und Worktree-Prüfung

Kanonischer Checkout, main, eigener Backup-Branch, aktive A-/B-Fix1-Arbeit, Q/Z und die gesperrten G5-Stände werden nicht gelöscht. Prozess- und Statusprüfung wurde neu erhoben. Worktrees mit unversionierten Daten, Archiven, Postgres-Clustern oder Prüfberichten bleiben bestehen. Der Tag sichert Git-Inhalte, nicht ignorierte Dateien. Lokale historische Branches bleiben als zusätzliche Rückholreferenzen bestehen.

Ein erster Tag-Push mit Wildcard-RefSpecs wurde vom Hook wegen fehlender Eindeutigkeit vor Ausführung abgewiesen. Kein Push erfolgte, keine Übersteuerung. Es folgen explizite Tag-RefSpecs.
