status: aktiv
Datum: 2026-09-30

# Statische Absicherung des nächsten Workspace-Testlaufs

Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a. Du bist der einzige Thread für dieses Paket. Keine Unter-Threads oder Unter-Agenten spawnen. Bestehender Sol-Thread 66adf9ee-bc03-4ff3-91da-73cd8efc5e72.

## Exakte Bindung

Eigener Quellworktree: /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930
Branch: fix/g5-replay-deferred-20260930
Produkt-/Lockhead: 9a29b81d230c01e5c03423cc34ba34c1074eab69. Letzter geprüfter Zustand sauber. Prüfe Bindung vor Arbeitsbeginn; kein Checkoutwechsel. Spätere reine Berichtscommits ändern diese Produktbasis nicht.

Dieser Head enthält inzwischen die echte Offline-Lockauflösung. Beide Metadatenläufe bestanden; der ausdrücklich einzeln zugeteilte Clippy-Lauf mit Workspace und allen Targets bestand mit Exit 0, ohne Konfigurationsänderung. Cargo-Slot ist bereits zurückgegeben. Nicht deine frühere Basis 0c290809 prüfen.

## Auftrag und Grenze

Nutzerauftrag: „Vor Ausführung klären, dass der Standardlauf keine Produktmodelle oder produktiven Dienste aufruft und keinen ENV-/Secret-Setup verlangt. Ignorierte DB-/Livefälle später getrennt mit eigenen echten Nachweisen führen; ein bestandener Unitlauf ersetzt sie nicht.“

Prüfe statisch den genauen Standardlauf, noch NICHT ausführen:
/home/nathanael/.cargo/bin/cargo +1.97.1 test --workspace --locked --offline --jobs 1 --target-dir /home/nathanael/.worktrees/brain-pre-g5-harness-20260929/rust/target -- --test-threads=1
cwd: genannter Quellworktree/rust.

1. Graphify vor Bestandssuche. Gesamten tatsächlich aufgelösten V1-Workspace erfassen, inklusive lokalem uplink-infisical-transport und Doc-/Integrationstests. Bekannte Zahl 25 Cargo-Member, kein Replay. Keine neue Metadatenaktion.
2. Nicht ignorierte Testpfade auf reale Netzwerk-/LLM-/Secret-/DB-Aufrufe, Prozessstarts, globale ENV-Manipulation, Dateisystem-/Configvoraussetzungen und bedingtes stilles Überspringen prüfen. Lokale Mockserver/Wegwerfprozesse von Produktdiensten unterscheiden und benötigte Laufzeitklasse benennen. Tracing bis zum Helper, nicht nur Testnamen zählen.
3. Genau feststellen, ob der vollständige Standardlauf innerhalb einer künftigen reinen Unit-/Testzuteilung sicher ausführbar ist. Wenn er lokale Prozess-/DB-Harnesses enthält, konkret nennen. Kein Test entfernen, ignorieren oder abschwächen; keinen fehlenden Nachweis als bestanden behandeln.
4. Ignorierte DB-, lokale Prozess-, Live-/Produktmodell- und Replayfälle getrennt inventarisieren: genauer Paket-/Testname, Skipbedingung, tatsächlicher späterer Ressourcen-/Configbedarf und zulässiger Nachweisweg. Zahlen als statisch ermittelt kennzeichnen; keine ausgeführten Testzahlen behaupten.
5. Exakte nächste Testanforderung und Laufzeitklassen mit vorhandenen Caches liefern. Bei nicht erfüllbarer Safety keine blinde Workspacefreigabe, sondern konkret benannte Blocker und sichere Prüfsequenz vorschlagen, ohne Produktcode umzubauen.

Nur statische Quellprüfung und git diff --check. KEIN Cargo, Compiler, Test, Fetch, Modellaufruf, Testprozess, Dienstwechsel oder neue ENV-/Secretkonfiguration. Hauptsession übernimmt die ausdrücklich erlaubte Formatprüfung. Keine Code-Kommentare, keine Produktänderung oder neue Skripte. Bestehende Python-Anwendung nur lesbare Referenz.

Bericht ausschließlich .tasks/2026-09-30-g5-abschluss/WORKSPACE-TESTVORBEREITUNG.md auf deinem eigenen Branch, mit Pfad:Zeile-Belegen. Eigene Berichtsdatei committen und eigenen Branch pushen, kein Merge. Bestehende Berichte als historisch belassen. Hauptsession prüft parallel den Serve-/Cutoververtrag, das ist nicht dein Paket.

Melde Ergebnis und SHA unmittelbar. Bei echtem Blocker: [Bump-up] Paket Tests: Grund: ... Erledigt: ... Worktree: ... Offen: ... an den Intent-Thread, dann stoppen. Kein fremder Worktree und kein neuer Worker.
