# P-Register

- Intent-Thread: 481426fe-b477-42b3-91c6-901811fcba1d. Keine neuen T3-Threads oder Sessionnachrichten.
- Archiv: /home/nathanael/.worktrees/brain-p-patch-20261008, feat/brain-patch-discovery, gesichert dcb5d47e. Bleibt erhalten.
- Lieferung: /home/nathanael/.worktrees/brain-p-delivery-20261008, fix/brain-p-patch-delivery-20261008, Basis 400381e681a2e08283db46094d1bbbde037e2813.
- Readerfreigabe: ausdrücklicher Orchestratornachtrag und ENTSCHEIDUNG-WACHE-037.md Abschnitt 1. Nur bestehende 11 Zeilen first_post_html, keine fremden Kommentarlöschungen.
- Lieferumfang: bestehender Patchimport samt API-Modul, minimale CLI-/Timeranbindung, freigegebener Readerexport, unmittelbare Regressionen und eigene Akte. Keine Archivgesamtübernahme.
- Stand: gemeinsamer Patchtest einschließlich Lookup und echtem Scratch-Reimport gestartet. Noch kein aktuelles Gate, Mainmerge, Deploy oder Livebeweis.
- Scratch: eigener Unixsocket-only-Cluster /tmp/brain-p-patch-pg-20261008.UIcyXW/cluster, Port 55449. LookupDB brain_fixer12_patch_lookup getrennt von brain_p_patch_reimport. Nur Schema aus bestehenden Tabellen gelesen, keine Produktionsdaten kopiert.
- Historisch: 45 bestandene Featuretests, 1 ignorierter Lookupfall. Diese Zahlen gelten nicht als gemeinsame Mainprüfung.
