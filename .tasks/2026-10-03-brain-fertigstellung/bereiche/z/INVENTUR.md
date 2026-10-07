status: aktiv
Datum: 2026-10-03

# Sicherungs- und Inventurstand Z

Sicherungsstand: 2026-10-03T14:39:43Z. Vollständige Inventur: `BACKUP-20261003T143943Z.txt`.

| Gegenstand | Nachgewiesener Umfang |
| --- | --- |
| Lokale Branches | 97, davon 30 Vorfahren von `origin/main` einschließlich main |
| Entfernte Branches | 83 serverseitig gegen `git ls-remote --heads origin` geprüft, davon 24 Vorfahren einschließlich main |
| Worktrees | 94, davon fünf locked; keiner prunable |
| Git-Referenzen | 251, jeweils mit exakt gleichem Namen und SHA im maßgeblichen Bundle |
| Bundle | 345 Heads, vollständige Historie, alle 94 Worktree-SHAs enthalten |

Maßgebliches Bundle: `/home/nathanael/.local/share/deadlock-brain/branch-backup-20261003T143900Z.bundle`, SHA-256 `3861c29fe579a82f74d577d11de0d936772e9fe3693d1f99aa6daf6046fce857`. Der native Sicherungsworker bestätigte keine Lücke gegenüber allen Referenzen, den serverseitigen Branches und den Worktree-SHAs. Die Hauptsession wiederholte `git bundle verify` unabhängig: Exit 0, 345 Heads, vollständige Historie.

Das ursprünglich vorhandene Bundle `branch-backup-20261003.bundle` blieb unverändert erhalten. Es war gültig, deckte aber fünf neue Sessionreferenzen nicht ab. Zusätzliche Vollsicherungen schließen diese Lücken und die anschließend hinzugekommenen zwei Referenzen.

## Bereinigung

Es wurde nichts gelöscht. Die erste Inventur belegt 27 unsaubere Arbeitsbäume und 62 mit ignorierten Artefakten. Commit-Historie ist gesichert, uncommittete, untracked und ignorierte Dateien sind durch ein Git-Bundle nicht gesichert.

Strikt geschützt bleiben Hauptcheckout und möglicher Live-main-Baum, Wiki-Auftrag, laufende Pakete P/S/Q/R/K/Z, locked Bäume sowie alle laufenden oder unklar zugeordneten Sessions. Die konkret gemergten, aber noch ungeklärten Kandidaten mit Vorschlag stehen in `UNKLAR.md`. Die Drafts #3/#4/#5/#6/#9 bleiben wegen Restwert erhalten; #46 wartet auf R.

## Fortsetzung

Die Prozesssicht wird im Workflow `wf_56dbf111-730` genauer geprüft. Der vorherige Lauf endete mit dem Harness ohne Ergebnis; das Journal enthielt nur `launched` und `started`. Der Lauf wurde am 03.10.2026 nach 15:12 UTC mit derselben Skriptdatei und Run-ID fortgesetzt. Keine zuvor abgeschlossene Sicherungsarbeit wird neu gebaut. Unmittelbar vor jedem späteren Löschschritt sind konkrete SHA-Deckung, Ancestry-Exitcode, Aktivität und Artefakte erneut zu prüfen.
