# Eigener künftiger Probevertrag

Die aktualisierten zentralen Akten `HOSTPROBE.md` und `SLOTS.md` wurden gelesen. Nur die eigenen künftigen Prüfaufrufe verwenden jetzt `hostprobe.sh`. Beide bestehenden Locks müssen zuvor blockierend erworben und während des Prüfschritts gehalten werden. `brain_wait_for_compilers` wiederholt bei BLOCK die vollständige Probe nach 30 Sekunden. Compilerstart und Quellfreeze bleiben Aufgabe des jeweiligen freigegebenen Wrappers; dieser Helfer startet selbst keine Prüfung.

Die einzige Ausnahme verlangt genau sieben NUL-getrennte Argumentfelder: Cargo, `metadata`, `--format-version`, `1`, `--no-deps`, `--manifest-path` und einen absoluten Pfad mit Dateiname `Cargo.toml`. Die Arrays werden mit Bash-Builtins ausschließlich im Speicher gelesen und nochmals verglichen. Nicht lesbare Prozessdaten, geänderte Prozessidentität, abweichender Prozessname oder unbekannte Argumente führen zu BLOCK. Die Ausgabe enthält höchstens PID, PPID, Prozessname und Klassifikation. Prozessumgebungen werden nicht gelesen.

`bash -n` für beide eigenen Dateien und `bash hostprobe-classifier-check.sh` ergaben Exit 0. Geprüft wurden vier Allowfälle einschließlich NUL-Array und Leerzeichen im Manifestpfad sowie 17 Blockfälle: relative oder falsche Pfade, zusätzliche, doppelte, fehlende und umgestellte Argumente, falsche Formatversion, anderer Programmname, Build-/Test-/Check-/Clippy-/Rustc-/Rustdoc-Unterbefehle, leerer Aufruf und fehlende Prozessdaten. Es wurde keine tatsächliche Hostprobe, Kompilierung, Credential- oder Dialogprobe ausgeführt.

| Eigene Datei | SHA256 |
| --- | --- |
| hostprobe.sh | 5433e6ba5a797541dd1e9e264359b8b88174b1bdc72030a153905e58264a72d1 |
| hostprobe-classifier-check.sh | cf5ae0963e4c15ec193727ca70b9f28b3f120710ce5ecf86ec7311183c001f80 |

Die Dateien sind eigene lokale Prüfartefakte. Zentrale Akten, fremde Prozesse und Repos wurden nicht verändert. Der tracked Quellstand bleibt `9d236af5ead09a221f290fe437e14ccaf77ba220`. Die historischen Validatorproben, ihre Exitcodes und Ergebnisakte bleiben unverändert.
