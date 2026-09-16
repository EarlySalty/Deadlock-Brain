# Item-Zweck und Helden-Skalierung – Gesamtstand

Stand: 16.09.2026. **NICHT abgeschlossen, keine fachliche Phasenfreigabe, kein neuer Release und kein neues veröffentlichtes Build.** Dieser Bericht trennt Implementierungsartefakte, tatsächlich belegte Tests und noch ausstehende Abnahme.

## Gesicherter Arbeitsstand

Produktcode-Basis `706b129d9c3af487d7ee2ef53bf43ed788e3b8ec`. Nutzerbefund, Arbeitsvertrag und erster Coder-Auftrag wurden mit `ee440f32c548f31c3a731b10d038e54b21429700` auf main gesichert. Inzwischen liegen auf main/origin/main auch die unabhängigen Preflight-Berichte in `reviews/codex-preflight/` (`424bc4d`, Klarstellung `b310830`). Diese Commits ändern keine Produktquellen.

Ein passender Opus-4.8-Implementierungsworker wurde bereits laufend vorgefunden, nicht doppelt gestartet:

- T3 `803d3e94-9b1d-42c5-9bb7-1905c8146acc`, Titel „Reasoner alle Helden (A Konversionsgraph)“, Modell `claude-opus-4-8`.
- Worktree `/home/nathanael/repos/wt/brain-purpose-a`, Branch `feat/reasoner-purpose-a`, Ausgangsbasis `706b129`.
- Dokumentationsbasis des Workers `60e8b63`.
- Erster nachprüfbarer Rust-Messwerkzeug-Commit `887e48d442bd0be39540edf2b4966d79bf0081d8`: Datei-Spiegel für PopulationPrior, technisch read-only Datenbankzugang der Messung, neuer Konversions-Abdeckungsbericht. Ausschließlich Examples; KEINE fertige Reparatur des produktiven Konversionsgraphen.

Die Anbindung bestätigte das laufende Einfrieren der Baseline und die Reparatur der zuvor live nachgeladenen Population. Bei der letzten direkten Dateiprüfung lagen jedoch noch keine PHASE-0.md, PHASE-A.md oder auswertbaren Messdateien unter `nachweise/` vor. Prozessstart oder Kompilierung ist kein gemessener Backtest.

## Messbasis – offen statt erfundener Zahlen

| Messgröße | Historische Dokumentation | Frische Phase 0 / Nachher |
|---|---|---|
| Warden-Referenzwaffen gegen 779996 | 6/9 | offen |
| Populations-Staples | 9/10, Gate nicht bestanden | offen |
| Populations-Kendall tau | 0,577 | offen |
| Jaccard@12 | 0,500 | offen |
| MR/Rusted Barrel/Healing Tempo als Kern | beanstandet | offen |
| HP-Downside und sinnvolle Ersatz-/Verkaufswahl | beanstandet | offen |
| Quantifizierte Mechaniken, nach Quelle/Parser/Sim getrennt | kein neuer Zähler | offen |
| Drei identische Rust-Replays / drei unabhängige KI-Läufe | kein neuer Nachweis | offen / offen |

Historische Zahlen stammen aus `.tasks/2026-09-16-population-baseline/N-MESSUNG.md`, nicht aus einem hier neu abgeschlossenen Lauf. Referenz- und Populationsmetriken nicht vermischen. FROZEN-V2 allein reicht nicht: der bisherige Replay lud PopulationPrior bei der Auswertung aus der laufenden DB nach. Vorher und Nachher brauchen dieselben eingefrorenen Asset-, Modell-, Referenz- UND Populationseingaben mit Hash/Versionsbeleg.

## Tatsächlich belegter Teststand

Der unabhängige Bericht `reviews/codex-preflight/REPORT.md` dokumentiert vier unterschiedliche bestandene vorhandene Rust-Tests auf Produktcode-Basis 706b129: zwei Spirit-Feuerraten-Tests (`item::tests::fix_e_`), den HP-Verlust-/Heilcap-Test und den Inventar-Compounding-Test für Spirit/Feuerrate/Magazin. Alle drei dort angegebenen Testaufrufe endeten mit Exit 0. Das ist ein gezielter Preflight, KEIN voller Testlauf und KEINE Abnahme von 887e48d oder Phase A. Keine neuen Testzahlen aus alten 170/386-Suitewerten ableiten.

## Belegte Hindernisse und Korrekturen

1. `population stats` ist laut Code und unabhängigem Preflight ein Aggregate-Schreibbefehl. Nicht gegen die zentrale DB für Phase 0 ausführen. Vorhandene Aggregate über technisch erzwungenen Read-only-Pool lesen; Neuaufbau ausschließlich lokal.
2. Spirit -> Feuerrate und prozentualer HP-Abzug sind schon teilweise vorhanden. Die Reparatur muss Rohdaten, Einheiten und alle Verbraucher vereinheitlichen, nicht parallel einen zweiten Bonus oder zweiten Abzug einführen. Details in PRUEFHINWEISE.md.
3. Unbekannte Bedingungen fallen teilweise auf None beziehungsweise pauschal 0,5; Inventarsimulation und flaches Scoring sind dabei nicht gleich. Unterschiedliche Effektbedingungen, echte Zeitfolgen/Distinct-Targets und unbekannte Mechanik müssen sauber getrennt werden.
4. Waffen- und Spirit-Schilde werden derzeit summiert; Gegner haben einen festen 50/50-Schadensmix. Relevanter Schutz und Anti-Gun-Kontext sind damit noch nicht ausreichend modelliert.
5. Der neue Messwerkzeug-Commit 887e48d ist noch nicht freigegeben: Er verwendet `FROZEN_POPULATIONS` als neue ENV-Konfiguration und lädt bei fehlender Variable weiter live nach. Für den beweisfähigen Modus sind explizite CLI-/Manifest-Eingaben ohne stillen Live-Fallback nötig. Ein Roundtrip-Test des Population-Spiegels und Eingabehash gehören dazu. Der neue Coverage-Example liest die bereits geladenen Modelle und zwei bekannte Rate-Keys; das ist nicht automatisch ein vollständiges Inventar aller Roh-Asset-Konversionen. Diese Punkte sind Sichtprüfungsbefunde, kein unabhängiges Abschlussurteil.
6. Nach den zunächst erfolgreichen T3-Abfragen/-Hinweisen wurden ein weiterer Leseaufruf und die Zustellung der letzten Korrekturhinweise vom Tool-Sicherheitsfilter blockiert. Die Hinweise aus Punkt 5 wurden damit NICHT an den Worker zugestellt. Keine Umgehung über andere Programme, Shells oder Sicherheitskonfiguration. Git-/Datei-Artefakte blieben prüfbar. Der aktuelle T3-Laufstatus ist über den letzten erfolgreichen Abruf hinaus nicht bestätigt.

## Phasen und Betrieb

0/A ist in Arbeit, A-Review fehlt. B, C, D und E haben konkrete getrennte Aufträge, aber noch KEINE neu gestarteten Worker und KEINE Freigabe. B-AUFTRAG.md, CDE-AUFTRAEGE.md und REGISTER.md halten Grenzen und nächsten Freigabepunkt fest. Keine zusätzliche bezahlte Parallelgruppe gestartet.

Keine zentralen DB-Writes, Migrationen, Dienste-/Timeränderungen, Releases oder Steam-Publishes durch diese Orchestrierung. Der Versuch, den Dienststatus direkt abzufragen, scheiterte an der Programm-Allowlist; daher wird auch kein neu geprüfter Live-Dienstzustand behauptet. 811202 bleibt die im Nutzerbefund genannte alte Build-ID, keine neue ID liegt vor.

Weiterarbeit ist erst ein Abschluss, wenn die offene Messstrecke korrigiert und die echte Baseline gesichert ist, danach A unabhängig geprüft und ohne Regression integriert wurde. B/C/D folgen jeweils mit frischem Coder und Reviewer; E prüft Gesamtergebnis und Release-/Publish-Beweise. Ein Artefakt-Commit ist keine Freigabe für die nächste Produktphase.

Eine 45-Minuten-Wache wurde nicht eingerichtet: die verfügbare Aufgabenplanung erlaubt höchstens stündliche Ausführung. Keine automatische spätere Phasenfortsetzung behauptet. Nicht integrierte oder noch aktive Arbeit wird nicht gelöscht; Aufräumen erst nach belegter Integration und SHA-Sicherung.
