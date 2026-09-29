status: aktiv
Datum: 2026-09-29

# Lokales Gesamtgate A: verbleibender Nachtrag

Prüfstand 6ddeb6c068d3997375f9e60a1bf2272272319e7f gegen origin/migration/rust-integration 4c962b83. Befehl `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo /home/nathanael/.worktrees/brain-pre-g5-finalize-20260929 --base origin/migration/rust-integration --head 6ddeb6c068d3997375f9e60a1bf2272272319e7f --codex-astra --effort high --timeout 600`. Exit 1, BLOCK. Kein Integrationsversuch von A danach.

R4 hatte den bekannten ersten Snapshotfehler unabhängig geschlossen. Das zusätzliche Gesamtgate nennt einen anderen Übergang derselben Fehlerfamilie: `brain-kernel/src/execution.rs:167` liefert Meta/Population als Answered nach validate_evidence, obwohl dabei ein zweiter synchroner Snapshot das verbleibende Budget verbrauchen kann. Das vorherige Budgetcheck vor der Evidenzvalidierung reicht nicht. A34 bekommt systematische Prüfung sämtlicher Erfolgs-/Validierungszweige und eine Gegenprobe mit schnellem ersten und langsamem zweitem Snapshot. Das äußere HTTP504 allein widerlegt kein verspätetes inneres Answered.

Weitere Gatehinweise:

1. NDJSON-Leerzeilen werden vor #L-Zählung entfernt. A12 korrigiert physische Quellenzeilen mit gezielter Fixture.
2. Config lässt Analytics-Fenster ohne volle effektive Upstream-Stunde zu, etwa [3601,7199]. A34 verwendet vorhandene effective_window-Prüfung schon beim Laden.
3. Fehlerhafte Retrievals verlieren weiterhin Usage. Bereits unabhängig als Anzeige-/Abrechnungsgrenze ohne nachgewiesene Budgetumgehung dokumentiert. Kein neuer Budget-/Portumbau allein aufgrund dieses NITs.

Der bisherige R4-Bericht bleibt als historischer positiver Nachweis der vorher bekannten Fälle erhalten. Das Gesamtgate wird weder durch dieses Urteil noch durch einen anderen Reviewer überstimmt. Nach Fix unabhängige Nachprüfung und erneutes Gesamtgate auf dem zusammengeführten Stand.
