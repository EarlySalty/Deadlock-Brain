status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-fertig-q

# Worker Q3: Anforderungen und G0 belastbar einordnen

## Wiederaufnahme und neue bestätigte Voraussetzungen

Der frühere native Lauf wurde beim Sitzungsende gestoppt. Die bereits befüllte `ANFORDERUNGSSTATUS.csv` und `evals/q-g0/messdefinition.json` bleiben erhalten. Prüfe und vervollständige diesen Stand, statt neu anzufangen. Q1 ist geliefert; die Hauptsession integriert gerade die autorisierten C9-Commits sowie ein redigiertes Anfrageereignis als neue Quelle für G0 und Consumer-Live-Nachweise. Der Nachweis muss echte Consumer-Kennzahlen enthalten, nicht Rohtexte.

`VON_HAUPT.md:32-33` bestätigt: interne Übernahme Sheet/YouTube, Lizenz unbekannt, keine Veröffentlichung oder externe Modelle, keine kanonischen Fakten, aktuelle Fassung plus 12 Monate Historie, Entfernung bei verschwundener Quelle, Gemini bleibt aus. Shadowfreigabe gilt für mindestens 100 Fragen aus öffentlichen FAQ/Patchnotes/Wiki und gekennzeichneten synthetischen Fragen; keine privaten Nutzer- oder Communityfragen. Werte diese bestätigten Entscheidungen korrekt im Rechteinventar unter deinem `q-g0/`-Schreibbereich aus. Priorität der Hauptsession ist C9-Operatortransport und Auditereignis vor G0.

## Ziel und Vertrag

Fülle jede der 60 Anforderungen in `architecture/migration/ANFORDERUNGSSTATUS.csv` mit ehrlichem Status und konkretem Nachweis. Definitionen in `architecture/migration/deadlock-brain-rust-planpaket-v1.0/07_ANFORDERUNGEN.md`; Paketgrenzen in `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/{AUFTRAG.md,PAKETE.md,BRIEFING-Q.md,GEMEINSAM.md}`. Was zu P/S/R/K/Wiki/Z gehört, verweist auf das Paket und bleibt bis zu dessen Beweis offen beziehungsweise teilweise. Keine Anforderung als erledigt markieren, weil sie einem Paket zugeteilt ist.

G0: vorhandene SLO-/Lastwerkzeuge und Rechteinventare ermitteln, dann eine präzise Q-Messvorlage mit vorhandenen Quellen erstellen. Reale Consumerdaten bleiben lokal; nur Kennzahlen ins Git. Kein synthetischer Pilot als echter Verkehr. Der erste read-only G0-Scout ist vor Beginn mit WebSocket-403 ausgefallen; sein Ergebnis existiert nicht. Du ersetzt genau diesen fehlenden Teil.

Bereits unabhängig festgestellt: `brain-serve.service` läuft mit HEAD `511a347`, Postgres `brain` auf Port 5446, aktuelle Reader-Konfig 60s Request/55s Provider. `brain-serve` besitzt Z. Journald des User-Discorddienstes `deadlock-bot-rust.service` enthält vom 01.10. bis 03.10. über 13000 Zeilen, aber keine Treffer auf brain/retrieval/knowledge; das belegt keine Nullnutzung. Twitch ist ein Systemdienst `deadlock-twitch-bot-rust.service` unter `/opt/deadlock/twitch/current`; die normale Benutzerabfrage seines Systemjournals lieferte keine Zeilen, daher nicht als Nullverkehr ausgeben. 2nd-Brain läuft als `deadlock-brain-site.service`. Weitere Logs/Beweismöglichkeiten empirisch finden. Quellenrechte für Sheet/YouTube sind unbestätigt, Entscheidung wurde in `bereiche/q/AN_HAUPT.md` angefragt. Keine Lizenz erfinden.

## Eigentum

Du schreibst ausschließlich `architecture/migration/ANFORDERUNGSSTATUS.csv` und `architecture/migration/evals/q-g0/` für Befund/Messdefinitionen. Keine `STATUS.md`, `GATES.csv`, keine fremden Inventory-Dateien, keine Rust-Crate, kein Cargo-Manifest oder Lock, keine Units/DB-Schreibzugriffe. Code- und Datensuche zuerst Skill `code-suche` und Graphify. Nutze den vorhandenen Brain-Graph `/home/nathanael/.graphify/projects/deadlock-brain/graphify-out/graph.json`, global bei Consumerbeziehungen.

## Arbeitsstand

Eigener Worktree `/home/nathanael/.worktrees/brain-fertig-q`, Branch `feat/brain-fertig-q-20261003`, HEAD `511a347b653beba13c2bf130f4bead7a7196cc2a`. Provider- und Writer-Worker schreiben parallel in ihre disjunkten Crates. Baseline-Q-Datei der Hauptsession nicht verändern. Kein Commit/Push/Merge, keine Compilerstarts. Native Workerrolle ohne weitere Delegation.

## Beweisziel

CSV hat genau 60 eindeutige R-IDs, keinen leeren Status/Nachweis. Nachweise existieren und passen zur jeweiligen Anforderung. Keine vollständige Green-Behauptung, solange wirkliche Live-/Rechtegrenzen fehlen. Q-G0-Bericht trennt vorhandene Messwerte, abgeleitete Sollwerte, fehlende Eingangsdaten und praktikablen Messbefehl mit bestehendem Rust-Werkzeug. Bestehende Suites nicht ändern. Rust-only gilt auch für neue Messwerkzeuge; in diesem Worker keine Werkzeuge bauen.

## Routing

Auftraggeber Teil-Orchestrator Q, Session `c671588c-6192-4bf5-8206-28bb30163666`; Hauptorchestrator `43a4886c-e135-484b-838a-0512d224a634`. Status allein durch `teil-q`, Versuch 1. Nie `TODO.md` oder `REGISTER.md` schreiben. Ergebnisdatei `bereiche/q/Q3-G0-ANFORDERUNGEN.md` im gemeinsamen Auftragsordner `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung`. Kompakter Bericht mit Grenzen und fundierten nächsten Prüfschritten, keine eigene Bugreview-Rolle.
