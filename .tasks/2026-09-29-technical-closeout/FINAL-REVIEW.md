status: Zwischenprüfung, Schlussabnahme ausstehend
Datum: 2026-09-29

# Unabhängige Schlussabnahme: vorläufiger Nachweisabgleich

**Zwischenurteil zum Verifikationspaket: konsistent. Technischer Abschluss fertig: N. Produktfix nötig: N nach dem geprüften Nachweisstand. Keine finale Abgabe.**

Der abschließende Dokustand der sieben Architekturdateien ist noch nicht zur Prüfung übergeben. Dieses Zwischenurteil gibt weder die Dokumentationsabgabe noch G5, Produktion oder einen Merge frei. Das lokale Gate folgt separat durch die Hauptsession.

## Prüfgrundlage und Bindung

Auftrag: `FINAL-REVIEW-BRIEFING.md`, Intent `562a877b-0939-440a-964d-1145d9e9431a`, bestehender unabhängiger Review-Thread `52c34332`.

- Eingefrorener Koordinationshead: `54b1302ce7599ac083a9858d34dd2c3b4fc3cfbd`.
- Integrierter Produktcode: `022f8a981c2164f6d8d4302bae2194e100c4f65c`.
- Eigener Reviewbaum bleibt auf dem bestehenden Branch `review/pre-g5-core-abnahme-20260929`; kein lokaler Merge des Koordinationsheads.
- Zuerst `git diff --stat 022f8a9 54b1302` geprüft: 65 geänderte Dateien, ausschließlich unter `.tasks/2026-09-29-technical-closeout/`, keine Architektur- oder Produktänderung in diesem Delta. Auch die vollständige Pfadliste enthält keine Datei außerhalb dieses Taskordners.
- `FINAL-VERIFICATION.md` aus dem Koordinationsworktree stimmt bytegleich mit dem Blob an `54b1302` überein. Logauswertungen beziehen sich auf `git show 54b1302:<Pfad>`, nicht auf möglicherweise fortgeschriebene Arbeitsdateien.

Zusätzlich identische Git-Tree-IDs an beiden Heads:

| Verzeichnis | Tree-ID an 022f8a9 und 54b1302 |
| --- | --- |
| rust | `09569dd0fbb4b818871b084ce1dc3666c03638f5` |
| config | `8c5b64ccaaa8c685af439e786feb35b6f917772f` |
| scripts | `09d059a412861128e111b1e06113028531af02c4` |

Kein neuer Codeaudit, kein neuer Testlauf und keine neue Implementierung durch diesen Reviewer. Die gezielte Einsicht in bestehende Runner und Prozessassertionen dient ausschließlich dazu, die berichteten Zahlen und Aussagegrenzen einzuordnen. Graphify wurde davor befragt; der globale Graph lieferte keine belastbare Zielstelle für diese neuen Runner.

## Workspace-Nachweise

Die vier Befehle in `FINAL-VERIFICATION.md:16-23` sind mit bereinigtem Umfeld, Offline-/Lockfilebindung und getrennten Ausgaben dokumentiert. Die gemeldeten vier Exitcodes sind jeweils 0.

| Gate | Eigener Abgleich der vorliegenden Belege |
| --- | --- |
| Format | `fmt.log` ist erwartungsgemäß leer. Der Exitcode ist im Verifikationsbericht festgehalten; eine leere Datei allein beweist ihn nicht. |
| Clippy | `clippy.log` enthält den erfolgreichen Abschluss nach 51,95 s; keine `warning:`-/`error:`-Zeile. Aufruf mit `-D warnings` laut Befehlsnachweis. |
| Workspace-Tests | 99 Ergebnisblöcke maschinell aufsummiert: **1011 passed, 0 failed, 75 ignored, 0 measured, 0 filtered**. |
| Release | `release-build.log` endet mit erfolgreichem optimiertem Release-Profil nach 2 min 18 s; keine Compilerfehler im Log. |

Die 75 Namen aus `ignored-inventory.log` stimmen mit den 75 als ignoriert gemeldeten Workspacefällen überein. Die Inventarisierung ist kein Testlauf. Acht Namen sind außerdem in gezielten Runnerlogs tatsächlich mit `... ok` vorhanden: Match, zwei Legacy-Fälle, Serve-Prozess, Scratch-SCRAM, Core-PostgreSQL, Storage-Upgrade und Wiki-Runtime. Sie werden weder aus den 75 Workspace-Ignorierungen herausgerechnet noch als weitere Workspace-Ergebnisse addiert.

Die übrigen 67 sind weiterhin nicht als bestanden bezeichnet. Die berichtete Aufteilung 58 Datenbank-/Snapshotfälle, vier Pilotphasen, zwei Live-Wiki-Fälle, ein Live-Assets-Fall, ein Replay-Sandboxhelfer und ein separater Wiki-Manifest-DB-Fall ist mit dem Inventar vereinbar. Keine pauschale Aktivierung oder nachträgliche Umdeutung ignorierter Fälle.

## Gezielte Runner und Last

Ergebnisblöcke aus den committed Logs selbst gezählt:

| Log | Passed | Failed | Ignored | Filtered |
| --- | ---: | ---: | ---: | ---: |
| serve.log | 6 | 0 | 0 | 26 |
| core-postgres.log | 1 | 0 | 0 | 3 |
| storage-upgrade.log | 1 | 0 | 0 | 1 |
| match-store.log | 1 | 0 | 0 | 1 |
| assets-starting-stats.log | 2 | 0 | 0 | 0 |
| wiki-runtime.log | 295 | 0 | 9 | 7 |
| replay-offline.log | 57 | 0 | 1 | 0 |
| replay-audit.log | 56 | 0 | 0 | 0 |

Bei Wiki sind die 295 genau die berichteten 294 regulären plus ein gezielter Scratch-Fall. Der Serve-E2E bleibt ein Cargo-Test mit mehreren Szenarien, nicht 18 zusätzliche Testergebnisse. Wiederholungen werden nicht zu einer Gesamtzahl eindeutiger Tests aufaddiert.

Die drei JSON-Lastzeilen in `serve.log` stimmen vollständig mit dem Bericht überein:

| Worker | Anfragen | Answered | Clientfehler | Fehlerobjekt | Reader-Verbindungen | Dauer ms |
| ---: | ---: | ---: | ---: | --- | ---: | ---: |
| 8 | 600 | 600 | 0 | leer | 4 | 1148 |
| 16 | 600 | 600 | 0 | leer | 4 | 772 |
| 32 | 600 | 600 | 0 | leer | 4 | 800 |

Poolmaximum und Peak jeweils 4, insgesamt fünf erstellte Verbindungen, 9115 wiederverwendete Checkouts, `wait_count=5351`, `wait_max_micros=150070`, `wait_total_micros=20231810`, `wait_timeout_count=1`.

Die unveränderte Prozessfixture setzt vier Reader-Verbindungen und 150 ms Poolwartebudget (`process_e2e.rs:414`, `:419`). Nach den Laststufen hält sie absichtlich vier Verbindungen an einer exklusiv gesperrten Tabelle fest und erwartet für einen weiteren Request `Unavailable` mit begrenzter Wartezeit (`:746-786`). Nach Freigabe müssen die blockierten Requests und die Recoveryfrage erfolgreich sein. Das erklärt den einzelnen Sättigungstimeout getrennt von den fehlerfreien Laststufen. Keine angehobene Grenze im Abschlussdelta.

## Ergänzende Detailbelege und ihre Aufbewahrungsgrenze

Die committed Consumerlogs enthalten nur drei Exit-Zeilen je Modus, nicht die Testanzahl oder den Head. Daher zusätzlich read-only im Verifikationsworktree `/home/nathanael/.worktrees/brain-pre-g5-harness-20260929` geprüft:

- `.consumer-ci-reports/client/{provenance.txt,results.tsv,test.log}`: Head `022f8a9`, Format/Test/Clippy jeweils Exit 0, **14 passed**, keine Fehler oder Ignorierungen.
- Entsprechende Dateien für `quality`: derselbe Head, drei Exitcodes 0, **85 passed**, keine Fehler oder Ignorierungen.
- Entsprechende Dateien für `cutover`: derselbe Head, drei Exitcodes 0, **59 passed**, keine Fehler oder Ignorierungen.

Die Zahlen sind damit selbst nachgezählt, bleiben aber teilweise an flüchtige lokale Detailbelege gebunden. Der Bericht behauptet ausdrücklich nicht, diese Detaildateien seien committed. Ohne sie sind allein aus den drei kurzen Consumerlogs nur die gemeldeten Exitcodes nachvollziehbar. Das ist eine Aufbewahrungsgrenze, kein nachgewiesener fehlgeschlagener Lauf.

Der Wiki-Abschluss hat committed neun Exitcodes 0, Befehlsliste, `complete=true`, `success=1` sowie Umgebung und exakten Head `022f8a981c2164f6d8d4302bae2194e100c4f65c`. Die lokal referenzierten Detaildateien unter `rust/target/wiki-completion.maNj9f/` wurden zusätzlich gelesen: Offline 117 passed, Contracts/Sources 157 passed und 7 ignored, Hero-Filter 7 passed und 58 filtered. Sie passen zum Bericht. Auch diese Detailtestlogs sind flüchtig, während der Überblick committed ist.

Der Verifikationsbranch steht inzwischen auf `5c7e5b23550cb90b9d525240c052d9641feac18d`; dessen Diff zu `022f8a9` enthält ausschließlich den Verifikationsbericht und seine Logs. Das spätere Committen der Nachweise wird nicht mit einem anderen getesteten Produktcode verwechselt.

## Grenzen: Replay, Betrieb und CI

`replay-audit.log` enthält ausdrücklich `status=blocked`, `manifest_rows=0`, `real_matches=0`, `integration_verified=false`. Der erfolgreiche Vorbereitungswrapper ist kein erfolgreicher realer Korpustest. Die acht Scratch-PG-Fälle sind kein Produktionsdatenbanknachweis; SQLx-SCRAM ist kein Service-Passwortstart. Wiki-Offlineabschluss ist kein Live-Capture, lokale Providerverträge sind kein Provider-Shadow.

`FINAL-CI.md` wird als datierter externer Beobachtungsbericht eingeordnet, nicht als eigener neuer GitHub-Lauf dieses Reviewers. Er nennt den finalen Codehead, den fehlgeschlagenen Erwerb des exakten `haste_core`-Pins `bfb292d4798031350861ad297aa26753267a1ea6`, fehlgeschlagene Consumer-CI und offene GitGuardian-Prüfung. Erfolgreiche lokale Cache-/Offlineprüfungen widerlegen diese Probleme einer frischen Fremdumgebung nicht. Keine vollständig grüne CI, kein Check-/Policy-Bypass, kein Pinwechsel oder Security-Abschluss behauptet.

Die bekannten nichtblockierenden Claim-Lease-/Usage-Grenzen bleiben aus dem unabhängigen R5-Bericht und dem Gesamtgate erhalten. Hier wurden sie weder als behoben erklärt noch ohne neuen Befund wieder als Produktblocker eröffnet.

## Noch ausstehende Schlussprüfung

Nach Übergabe des eingefrorenen finalen Dokustands werden ausschließlich die sieben Architekturdateien und deren Bindung an diese Nachweise geprüft. Offen sind insbesondere die sachliche Erledigung von `F2-NACHPRUEFUNG.md`, eindeutige historische gegenüber aktuellen Markern, implementierte Provider gegenüber ausstehender Betriebsaktivierung und bereits vor diesem Auftrag gemergtes Twitch #984 gegenüber den weiterhin offenen Consumer-PRs. Auf `54b1302` wurden diese finalen Korrekturen nicht als abgeschlossen bewertet.

**Zwischenstand: kein Widerspruch in den geprüften Verifikationszahlen; keine nachgewiesene Produktänderung; keine finale technische Schlussabnahme vor dem Doku-Nachtrag.**

### Belegfingerprints

SHA256 der eingefrorenen committed Logs:

```text
d45d37c5f4a08e1ff13d33e597cf50f7326095b4890ca62e968eb10262f89a37  workspace-test.log
6eb39739bcd58ca9cda04983107b1334a09b6433270e801b1aec5e49a3c3d472  serve.log
dd0e451077ad94605f9659a25a2f548d12184a5b82d7089e0f529d237e0c99ed  ignored-inventory.log
3047c79a82937b9d295e2be94804457eeca0c014373cea41416babb1afdb7855  wiki-completion-environment.txt
```

SHA256 der ergänzend gelesenen flüchtigen Consumerbelege:

```text
7d8a42d6b4b1276587c726f03cda9d6130c91a012f6a00d9c49979f9294147a9  provenance.txt in allen drei Modi
68fd700d284d7a779cee740f0740ae0034ad0fe1e33c85dc166ff66d4aa41e9b  results.tsv in allen drei Modi
c6c4e1438597918d3097de98cf2ad2ecd5fe952d0420c6d2f3da3d388245ef45  client/test.log
9d8e1c32be179354d51dbfd9ceb164781c74190ed25d7296fadb85681a03da96  quality/test.log
5f64242c26457ef3618a2276e9f514c4ad0d0e190f40b7d19edf80b5958daf62  cutover/test.log
```

TESTNACHWEIS[TW-1]: 1011 passed, 75 ignored | Baseline: keine Altfehlerbehauptung; vorhandenen Fremdlauf selbst nachgezählt, 0 neue eigene Testläufe; 8 gezielte Fälle separat belegt
TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: interner vorläufiger Reviewbericht
