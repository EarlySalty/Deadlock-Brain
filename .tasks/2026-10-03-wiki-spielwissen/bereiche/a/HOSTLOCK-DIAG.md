status: aktiv
Datum: 2026-10-03

# Nur lesende Hostlock-Beobachtung

A hat keine fremden Prozesse beendet, keine Lockdatei verändert und keine Sperre umgangen. Diese Beobachtung ersetzt keine eigene erfolgreiche Lockübernahme oder vorgeschriebene frische Compilerprobe.

Mit `fuser -v` wurden offene Handles auf beide bestehenden Lockdateien beobachtet. Für `/tmp/deadlock-cargo-release.lock` erschienen Bash PID 2159871 und ein kurzlebiger Sleep-Prozess PID 3209185. Bash 2159871 hielt außerdem einen offenen Handle auf host-checks.lock. Die weiteren dort gelisteten Bash-/flock-Paare sind offene Handles, nicht automatisch exklusive Inhaber; darunter können eigene und fremde Wartetasks sein.

Eine anschließende ausschließlich auf Prozessnamen/Status begrenzte Probe lieferte keine Nicht-Zombie-Zeile für cargo, rustc, clippy-driver, cargo-clippy, cc, cc1, cc1plus, gcc, g++, c++, clang, clang++, ld, ld.lld, lld, collect2, cmake, ninja oder make. Es wurden keine vollständigen Argumentlisten und keine Umgebungen gelesen oder ausgegeben.

Die Metadatenprobe von Bash 2159871 ergab PID 2159871, PPID 1358015, Status Ss, Alter 8991 Sekunden, einen Thread und 3596 KiB RSS. Sie beweist keine fachliche Sessionzugehörigkeit. A hat keinen eigenen Startnachweis für diesen Prozess und lässt ihn unverändert. `fuser` belegt offene Handles, nicht alleine die exklusive Lockzuständigkeit.

Der vorhandene A-Datenworker hatte drei eigene wartende Wrapper bereits vor neuen Moduländerungen sicher beendet. Ein neuer Versuch nach Fix-Freeze3 ist freigegeben; dessen tatsächlicher Compilerstart ist noch nicht bestätigt. Hauptorchestrator kann anhand seiner eigenen Startregister prüfen, ob der beobachtete Bash-/Sleeppfad ein eigener verbliebener Prüftask ist. Keine fremde Sitzung kontaktieren oder unbekannten Prozess beenden. A wartet weiterhin ausschließlich über die vorgegebenen Sperrmechanismen.
