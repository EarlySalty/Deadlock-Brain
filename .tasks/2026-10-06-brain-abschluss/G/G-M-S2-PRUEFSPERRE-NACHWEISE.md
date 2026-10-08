# G-S2-Restkern: tatsächliche Prüfsperre vor Umsetzung

status: blockiert vor Quelländerung, 07.10.2026

## Tatsächliche Rückgabe und unabhängige Bindung

Genau ein frischer nativer Fixer im Workflow wncl0gm5w / wf_c0bc5cf3-ded abgeschlossen. Kein Testprozess, Fix, Commit, Push oder Gate-Lauf. Beide freigegebenen combat.rs-Restkerne bleiben offen. Bereichsführung bestätigte nach Rückgabe weiterhin HEAD 8feb8b6ec0bf3dac7a8e180bfacc59ed001d3206, gleichen Produktstatus und alle zwölf Quellfingerprints des Endmanifests mit Exit 0. Fünf vorhandene Fixcommits sowie S3/S4-WIP und Originalfixtures erhalten. Wache 3c9d3ecd gelöscht.

## Präzise Sperre

Der in HOSTPROBE.md veröffentlichte FD/flock-Slotloop wurde vor Ausführung abgewiesen. Worktree-Isolation klassifiziert exec im Schleifenkonstrukt als nicht hinreichend prüfbar: aus übergebenem Shelltext könne Git außerhalb des eigenen Worktrees nicht ausgeschlossen werden. Die Fehlermeldung fordert einfache getrennte Befehle. Keine Wiederholung, kein anderer Tool-/Workerweg, kein Cargo ohne Slot, keine Änderung an Hook oder Settings.

Originaler unveränderter Wortlaut steht in der tatsächlichen Rückgabe:

/home/nathanael/.claude/projects/-home-nathanael--worktrees-brain-g-v2-20261007/030a7b6f-d25c-482d-b66c-68185cd05dbb/subagents/workflows/wf_c0bc5cf3-ded/journal.jsonl:3

Bereichsführung extrahierte aus dem vorhandenen nativen Transkript ausschließlich den verweigerten Werkzeugaufruf samt Fehler, keine neue Ausführung. Genau ein entsprechendes verweigertes tool_result, Zeit 2026-10-07T15:47:02.658Z. Exakter verweigerter Befehl unverändert abgelegt als G/pruefungen/g-m-s2-restkern/slot-denied-command.txt. Kein ausführbares Ersatzskript. Der Versuch, den originalen Fehler zusätzlich unverändert als Rohtext abzulegen, wurde wegen seines Original-Gedankenstrichs von R13 abgewiesen; nicht wiederholt oder durch anderen Schreibweg umgangen. Der originale Wortlaut bleibt im unveränderten Transkript erhalten.

Der abgewiesene Test sollte combat::tests im Primärworktree mit locked/offline/jobs 3 und dem vorhandenen Target prüfen. Kein Cargo-Exitcode und keine passed/failed/ignored/filtered-Zahl vorhanden. Der angefragte Befehl enthielt include-ignored und nocapture, aber nicht test-threads=1; diese fehlende Pflichtoption ist kein Grund des Denys und kein ausgeführter Testfehler. Für einen künftig ausdrücklich zugelassenen Prüflauf muss sie berücksichtigt werden.

## Fehlendes Beweisziel und Zuständigkeit

Nicht erbracht: numerischer Stackbonus mit und ohne Rüstungsverringerung in den gemeinsamen Pfaden, explizites Szenario ohne doppelten Shred, doppelte Item-IDs über beide öffentlichen Simulationseingänge mit identischer Stat-/Shop-/Effektpopulation. Alte Compilerbelege ersetzen diese Prüfung nicht. Keine neue S2-Freigabe und keine Sicherung oder S3/S4-Fortsetzung daraus.

Empfehlung an den zuständigen Harness-/Regel-Eigentümer: den veröffentlichten bestehenden Slotweg und die Worktree-Befehlsprüfung miteinander vereinbar machen, ohne Git-Isolation oder Prüfsperren zu umgehen. G verändert keinen Hook, baut keinen Wrapper und überträgt die verweigerte Ausführung nicht an einen anderen Worker. Facheskalation in AN_HAUPT-G.md gemäß der Fortsetzungsentscheidung; zentrale Akte bleibt beim Delegator.

Kein Main, Release, Deploy, Neustart, Runtime, Cleanup oder Settle. Zwei lesende Git-Schritte des Fixers, vier lesende Elternprüfungen vor und nach dem Lauf; keine schreibende Gitwirkung dieses Versuchs.

MERGEPROTOKOLL[MS-1]: 6 Git-Schritte einzeln | Anläufe: 0 | Gate: nicht ausgeführt; Prüfsperre
