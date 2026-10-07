# Paket A: erste Bestandsaufnahme

## Auftrag und Grenze

Auftraggeber ist Paket-A-Teil-Orchestrator im nativen Claude-Code-Harness, Modell GPT 6.1 Sol (`gpt-6.1-sol[1m]`). Hauptorchestrator: `3fcd8f71-443e-48ae-825c-527eb52fbe56`. UltraCode ist laut Runtime aktiv; der erste native Workflow belegt die tatsächliche Verfügbarkeit. Alle Agenten erben das Sitzungsmodell. Keine weiteren T3-Threads, keine Session-Nachrichten, keine eigenen Reviewer.

Zuerst `AUFTRAG.md` und `BRIEFING-A.md` im Elternordner lesen. Sollzustand aus den Akten vom 03., 04. und 05.10. sowie Draft-PRs 3 bis 9 ermitteln, gegen aktuellen Code und Laufzeit prüfen. Berichte sind Hinweise, keine Belege. Nicht an Game Invites arbeiten. Zurückgestellte Bereiche nicht bauen.

## Eigentum und Arbeitsstand

Dieser Durchgang ist read-only für Code, Git, Datenbanken und Dienste. Kein Merge, Deploy, Restart, Post, Löschen, Test mit Außenwirkung oder Änderung von Konfiguration. Einzige Schreibberechtigung: der pro Agent zugewiesene Bericht unter diesem A-Ordner. Geteilten Checkout und sämtliche früheren Worktrees unangetastet lassen. Git-Reads über literale absolute Pfade. `origin/main` wurde vor dem Start frisch geholt. Native Agenten teilen das Dateisystem.

| Agent | Zuständigkeit | Einziger Schreibpfad |
|---|---|---|
| A-I1 | Brain-Kern, Releases, Patch-Historie, Tagesläufe und Legacy-Abschaltung | `INVENTUR-KERN.md` |
| A-I2 | Spielwissen-Steckbriefe D5, Patch-Story und Wiki | `INVENTUR-WISSEN.md` |
| A-I3 | Discord-Brain, Serverguide und Twitch-Antworten | `INVENTUR-BOTS.md` |
| A-I4 | Build-Reasoner, Draft-PRs und unveröffentlichte Reasoner-Artefakte | `INVENTUR-REASONER.md` |

## Beweisziel

Graphify vor jeder Codebestandssuche. Danach relevante Fundstellen verifizieren. Nicht pauschal große Rohlogs oder Secrets ausgeben. Bericht: Bereich, Soll, Ist mit konkreten Code- und Live-Belegen, Zustand (`fertig`, `halb`, `kaputt`, `zurückgestellt`), vorhandener Wiederverwendungspunkt, kleinster nötiger Fix, Abhängigkeit und sinnvoller Schreibbereich. Aktuellen origin/main-SHA und laufenden Release-SHA getrennt nennen. Reale Antworten nur lesen, keine Testnachrichten an Community senden. Fehlender Zugang ist kein Beweis für kaputt.

Keine Dienste mit Geheimnissen oder Nutzerdaten an externe Modelle senden. Lokale Read-only-Diagnostik verwenden. Status- und Fachberichtproduzent je Agent ist der Agent selbst, Versuch 1. TODO.md und Haupt-REGISTER.md nicht ändern. Stop bei Schreibbedarf, widersprüchlichen Verträgen oder echter Produktentscheidung: präzisen Befund an Paket A im zugewiesenen Bericht festhalten. Native Worker delegieren nicht weiter.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: hauptbaum
