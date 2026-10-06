status: erledigt
Datum: 2026-10-03
Stand der Abschlussprobe: 06:16:34 UTC

# B2: unabhängige VPK-Nachprüfung

Nativer Prüfer `ad378c14151ec7ca9`, geerbtes GPT 6.1 Sol high. B2 hält den Rückbericht hier fest. Prüfer hat weder Quellen noch Harness verändert und keine Compiler oder Tests gestartet.

Urteil: Primärfund P2/F6 behoben. Im untersuchten VPK-Allokationspfad ist kein weiterer Fix nötig. Das ist eine statische Nachprüfung des offenen Fundes, keine finale Eigenabnahme oder Compiler-/Testfreigabe.

`game_files/vpk.rs:202-237`: Eingebettete und Geschwister-Payloads werden gegen die gehaltene Datei geprüft, bevor Payloadspeicher reserviert wird. V2 schließt nachfolgende Prüfsummenabschnitte aus. Zeilen 251-256 prüfen das Budget vor `try_reserve_exact`; Allokationsfehler werden zurückgegeben. Bereichsrechnung verwendet `checked_add`. Die übrigen Summen sind durch u16-/u32-Eingabefelder überlaufsicher.

Baum und Preload sind ebenfalls vor Allokation begrenzt und budgetiert. Der öffentliche Aufrufer erhält Fehler als Inventarlücke. Die direkte Zwillingssuche ergab keinen weiteren Befund.

Der 51-Byte-Test deckt beide Archivvarianten, konkrete Bounds-Fehler und den öffentlichen Fehlerpfad bei hohem Optionslimit ab. `pruefharness-b/check-round2.sh:45-49` führt ihn zusätzlich unter 128 MiB Adressraum aus. Diese Schutzwirkung ist statisch nachvollziehbar; der Prüfer hat den Test nicht ausgeführt.

Hashbindung: VPK SHA-256 `feeec228eea64e008ba09dd70032245c32e366bcb7c4c2e891c3466310f9f368`. Alle elf Dateien aus SHA256SUMS-HANDOFF.txt bei Anfangs- und Abschlussprobe unverändert. Eine spätere Quelländerung entwertet diese Nachprüfung.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft
