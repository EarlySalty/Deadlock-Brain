# Paket D: SHA-Sicherung und Cleanup

07.10.2026. Cleanup der drei eigenen Featurebranches auf Stop-Hook-Auftrag. Kein Main-Push, kein Releasebau und keine Liveoperation. Der Auslieferungs-Hold bleibt bestehen.

## Vor jeder Löschung geprüft

Frisches `git fetch origin` erfolgreich. Für jeden der folgenden Branches eigener Aufruf `git merge-base --is-ancestor <branch> origin/main`, jeweils Exit 0. SHA-Sicherung vor den Löschungen:

| Branch | Gesicherter SHA | Worktree |
| --- | --- | --- |
| `feat/brain-ernte-sicherheit-20261007` | `baca936e7d121a9b44f4a81aa914fbe91093b3e6` | `/home/nathanael/.worktrees/brain-ernte-sicherheit` |
| `feat/brain-ernte-spielstil-20261007` | `0ade1a3dc881e466a15d81550a560dc512598f2f` | `/home/nathanael/.worktrees/brain-ernte-spielstil` |
| `feat/brain-ernte-patchbelege-20261007` | `fde910f6a0199c00f44083e73fc8f4c5e4f80b86` | `/home/nathanael/.worktrees/brain-ernte-patchbelege` |

Worktree-Status: keine geänderten oder unversionierten Dateien; Sicherheit und Spielstil enthalten jeweils ein ignoriertes `rust/target`, Patchbelege ist vollständig sauber. Keine Prozesse mit Arbeitsverzeichnis, Binary oder Argumenten in diesen drei Worktrees gefunden. Die beiden Targets sind normale Verzeichnisse, keine Symlinks. Sie werden vor Worktreeentfernung unverändert unter `/home/nathanael/.local/state/brain-ernte-pruefartefakte-20261007/` erhalten. Keine Artefakte löschen und kein `worktree remove --force` verwenden.

## Ausführung

Abgeschlossen und um 03:37 CEST geprüft. Beide Targets wurden unverändert nach `sicherheit-target` und `spielstil-target` im genannten Erhaltungsverzeichnis verschoben. Alle drei Worktrees regulär ohne Force entfernt; alle drei lokalen Branches mit `branch -d` gelöscht. Beide gepushten Featurebranches regulär auf origin gelöscht. `ls-remote --heads` liefert für alle drei Namen keine Remote-Branches, lokale/Remote-Tracking-Referenzen ebenfalls nicht mehr vorhanden. Die Worktreeverwaltung enthält von D nur noch die detached Integrationsquelle.

Die Warnung von `branch -d`, die Stände seien nicht im HEAD des fremden kanonischen Featurebranches enthalten, wurde nicht umgangen: Entscheidend ist der zuvor für jeden SHA geprüfte Exit 0 gegen frisch geholtes `origin/main`. Keine Änderung am kanonischen HEAD oder fremden Dateien.

Der eigene detached Integrationsworktree `/home/nathanael/.worktrees/brain-ernte-integration` auf `0ade1a3dc881e466a15d81550a560dc512598f2f` und das unfertige Releasebundle bleiben als Übergabequelle erhalten. Auslieferungs-Hold unverändert; keine vollständige Betriebsabnahme oder Self-Settle behaupten.

MERGEPROTOKOLL[MS-1]: 19 Git-Schritte einzeln | Anläufe: 1 | Gate: nicht ausgelöst (nur Cleanup; frühere Ports ALLOW)

Umfang: Fetch, drei Statusprüfungen, drei Ancestorprüfungen, SHA-Sicherung per show-ref, drei Worktreeentfernungen, drei lokale Branchlöschungen, zwei Remote-Branchlöschungen, Referenzprüfung, Remoteprüfung und Worktreeprüfung. Keine neuen Commits oder Main-Pushes.
