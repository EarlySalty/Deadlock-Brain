status: aktiv, fünf Arbeitsbäume für G5 erhalten

Ergänzung 30.09.2026: ausdrücklich erlaubter isolierter Snapshotzusatz im Worktree /home/nathanael/.worktrees/brain-g5-readonly-snapshot-20260930, Branch fix/g5-readonly-snapshot-20260930, Basis c5951b610aa2545d2c0b43b33b5fe1906198b292. Ebenfalls per git worktree lock geschützt. Kein neuer Cache. Bestehender Autor66adf9ee, Dispatch1183588. Alter Quellworktree brain-g5-replay-deferred-20260930 bleibt eingefrorenes PG2-Prüfziel. Diesen fünften Pfad ebenfalls vom Cleanup ausnehmen. Die folgende Viereraufnahme bleibt historischer Sperrbeleg.
Datum: 2026-09-30

# Aktiven G5-Bestand erhalten

Auf ausdrücklichen Nutzerauftrag wurden vier vorhandene Worktrees nach Lesen ihres Gitstatus per git worktree lock geschützt. Die Sperren tragen den Intent562a877b und den konkreten aktiven Zweck. Kein neuer Worktree, kein Checkoutwechsel, keine Quelldatei der Worker geändert.

| Worktree | Zweck | Stand bei Sperrung |
|---|---|---|
| /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930 | Aktive Autorquelle | b3523fa, Testfix8949198 enthalten |
| /home/nathanael/.worktrees/brain-pre-g5-core-review-20260929 | Aktives unabhängiges Nachreview | eceb14c, neuer Bericht in Arbeit |
| /home/nathanael/.worktrees/brain-pre-g5-harness-20260929 | Verbindlicher bestehender rust/target-Cache | 5c7e5b2, Targetverzeichnis vorhanden |
| /home/nathanael/.worktrees/brain-technical-closeout-20260929 | Eigene Koordinationsakte | 26ce3c0 mit eigener Dokumentationsarbeit |

Im vom Nutzer genannten lokalen Cleanup-Register /home/nathanael/Documents/claude-config/.tasks/2026-09-30-aufraeumen-gesamtstand/REGISTER.md wurde zusätzlich ein abgegrenzter Abschnitt mit genau diesen vier Pfaden, den Cachegrenzen und dem Verweis auf die zentrale Ressourcenakte eingetragen. Dieser Abschnitt ist eine lokale angeforderte Schutznotiz; diese Session committet keine fremde Cleanup-Arbeit und stellt deren Checkout nicht um. Der Schutzauftrag wird hier im eigenen Fachbranch gesichert. Auch /home/nathanael/.cargo bleibt für den Offline-Lauf erforderlich.

Git-Worktree-Locks verhindern reguläres worktree remove/prune, sind aber kein Schutz gegen ein direktes rm oder erzwungene Entfernung. Deshalb steht die ausdrückliche Ausnahme zusätzlich im Cleanup-Register, statt allein auf Prozess-cwd zu vertrauen. Keine neue Wache und keine Sessionnachricht. Nach tatsächlichem G5-Abschluss eigene Locks bewusst aufheben und erst dann den zugehörigen Cleanup ausführen.
