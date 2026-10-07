[Orchestrator]
# Q: wiederholbare Abnahme und Modellvergleich

status: vorbereitet, NOCH NICHT STARTEN. Erst nach belegtem Live-Gang von I, G und K. Kein neuer Worker oder Modelllauf allein aus dem Vorliegen dieses Briefings.

## Rolle, Auftrag und Vorrang

Frischer Worker für den offenen Q-Auftrag, nach tatsächlichem Start durch Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Alter Q-Thread 537049fe-ce77-4f95-ac54-32987c807df0 ist stopped, nicht wiederaufnehmen. Vorhandenen Collector, Quellenbindungen und Nachweise übernehmen, keine zweite Pipeline. Der frühere Haupt-Orchestrator d3a1741e bleibt unangetastet.

Verbindlich zuerst lesen: ENTSCHEIDUNG-DATENSCHUTZ-NUTZER-2045.md in aktueller korrigierter Fassung, ENTSCHEIDUNG-WEITERBAU-2015.md einschließlich Abschnitt 6, PAKETE.md, AUFTRAG.md sowie Q/WIEDERAUFNAHME.md, Q/BERICHT.md, Q/STATUS.json und Q/CLEANUP.json. Alle liegen unter .tasks/2026-10-07-brain-fertigstellung-astra/. Die neue Datenschutzentscheidung ersetzt für die Testphase frühere pauschale Sperrtexte; eine harte Kategoriesperre wurde ausdrücklich zurückgezogen.

## Feste Schicht und Datenschutz

- Die feste Schicht bestimmt Rechte, Werkzeuge, Rust-Rechnung und Belegprüfung. Ein Modell darf diese Grenzen nicht selbst erweitern. Discord-Kanalsicht bleibt rollenbasiert: nur Inhalte, welche die tatsächlich fragende Person selbst sehen darf. Rollen-/Identitätsbindung intern prüfen, bei fehlender Bindung keinen erweiterten Zugriff erlauben. Keine zusätzliche harte Ticket-/Mod-/Teamkategoriesperre erfinden.
- Der fremde Brückenfix-Thread 3e93aeea ist zurückgezogen. MUST NOT ihn verwalten, lesen, kontaktieren oder seine frühere Umsetzung übernehmen. Nicht auf einen dortigen Livefix warten.
- Nutzerfreigabe für TESTS: echte Discord-/Twitch-Fragen samt minimal nötigem Antwortkontext dürfen über den bestehenden gpt-6-luna/Codex-Abo-Provider laufen, einschließlich privater Antwortabnahme. Discord-/Steam-IDs, Mitgliederlisten und fremde Personendaten NEVER mitsenden. Nur nötige bereinigte Daten, keine pauschalen Chat-/Mitgliedschaftsexporte.
- Herkunft und Bereinigung lokal vor Provideraufrufen prüfen. Private Originale und Rohdaten MUST NOT in Git, öffentliche Berichte oder deine externe Codiermodellkonversation gelangen. Die Freigabe betrifft den bestehenden Antwortprovider, nicht deine eigene Modellkonversation. Secrets NEVER ausgeben. Auch Werkzeugargumente und Antworten nur lokal geschützt aufzeichnen; sichere Berichte ohne Kennungen oder Personendaten.
- Kostenpflichtige Anbieterläufe sind NICHT freigegeben. Erster Lauf ausschließlich gpt-6-luna über bestehenden Abo-Provider. Wählbarkeit nur über vorhandene zentrale Providerkonfiguration, kein neuer Connector, Sondermodell oder stiller produktiver Modellwechsel. Weitere kostenpflichtige Läufe erst nach ausdrücklicher Kostenfreigabe.

## Wiederverwendung und Eigentum

Geteilter schmutziger Produktcheckout /home/nathanael/repos/Deadlock-Brain bleibt unverändert. Keine fremden Produktdateien, Branchwechsel oder Commits dort. Vorhandener Rust-Collector Q/collector stammt aus geprüftem bf54e659814008c8db941e6ff6be9016314347d3. Lokale private Kopien, Integritätsprüfung und Rechte stehen in Q/WIEDERAUFNAHME.md. Diese zuerst lokal prüfen, ohne Originalinhalte auszugeben, neu zu exportieren oder zu überschreiben. 166 Kandidaten sind weiterhin keine 30 akzeptierten Goldfälle.

Exklusiver neuer Berichtsort Q2/ unter dem zentralen Aufgabenordner: STATUS.json, BERICHT.md, Methodik und sichere technische Nachweise. Alte Q-Belege unverändert halten. Zentrale TODO/REGISTER/PAKETE bleiben beim Delegator; I/G/K-Produktdateien gehören ihren ausführenden Paketen. Produktbefunde mit Ort, Szenario und Beweis zurückgeben, nicht selbst produktiv fixen.

Falls Anpassung des vorhandenen Collectors wirklich nötig: zuerst code-suche/Graphify und bestehende Evaluations-/Providerwege inventarisieren. Dann eigener regulärer Worktree unter ~/.worktrees und Branch von frisch geprüftem main, enges Rust-Delta statt neuer Pipeline. Git-Einzelschritte, nur eigene sichere Dateien, keine privaten Artefakte stagen. Kein Entwicklungsbranch nur als Warteplatz.

## Abnahmevertrag

1. Startbedingung: echte I/G/K-Livelieferung einschließlich Prozess-/SHA-/Consumerbelegen, nicht bloß Featurepush oder ALLOW. Fehlende Bedingung melden, keinen Vergleich auf einer synthetischen Ersatzschicht starten.
2. Mindestens 30 echte Fragen aus den beauftragten Quellen, feste erwartete Antwortarten und unabhängige Originalfakten VOR erstem Lauf. Herkunft, Dubletten und Datenschutz lokal prüfen. Keine erfundenen Ersatzfragen oder ungesicherten automatischen Labels als Gold. Pocket/Haze-Fakten und fünf vorbereitete Patchstichproben übernehmen und gegen tatsächlichen Wissensstand prüfen.
3. Dasselbe versionierte Set gegen dieselbe feste Werkzeug-/Rechte-/Rechenschicht führen. Je Lauf Modell und Providerkonfiguration, Code-/Wissens-/Quellstand, tatsächliche Werkzeugaufrufe samt lokal geschützten Argumenten, Übereinstimmung der Zahlen mit Rust-Rechnung, Belegtreue, Antwortzeit und Tokenverbrauch erfassen. Fehlende Verbrauchswerte als nicht erfasst ausweisen, nicht als null. Gemessene, reservierte und unbekannte Abrechnung nicht vermischen.
4. Erster Lauf Luna. Bei später genehmigtem Modellwechsel nur Providerkonfiguration ändern; Fragensatz, erwartete Antwortarten, Rechte, Werkzeugstand und Rechenmaßstab unverändert. Unterschiede der Datenbasis dokumentieren, nicht passend machen. Messurteile dokumentieren, keine eigenmächtige Modellwahl für Produktion.
5. Echte Discord-Erwähnungs-/DM- und Twitch-Antworten über freigegebene Testkonten belegen, insbesondere P1 unter 20 Sekunden. Private und öffentliche Fälle getrennt. Rollen-Sichtgrenzen und Datenminimierung nachweisen. Keine fremden Kanäle oder Produktionsdienste für Ausfalltests stoppen, keine Invite-Sendewirkung oder Veröffentlichung fremder Builds. A-Fragen danach erneut, P0 bis P11 ehrlich nachhalten. Nach späteren Deploys dasselbe feste Set wiederverwenden.

Historische Healthproben, synthetische HTTP-Fixtures und Compilerläufe sind kein neuer Kanal-/Zahlenbeweis. Scheitert eine Voraussetzung, exakten Teilblocker melden und vorhandene Ergebnisse erhalten. Kein Polling fremder Sessions oder neuer Reviewthread.

## Verbindliche Fälle aus dem echten Nutzertest

`BEFUND-NUTZERTEST-2000.md` im zentralen Aufgabenordner lesen. Vier echte Nutzerfragen aufnehmen: Armor Piercing Rounds gegen Plated Armor; Haze und Magic-/Spirit-Schaden; Hilfe für einen schlechten/neuen Spieler; Pocket kontern. Keine Antworten vor Originalfaktenprüfung als Soll erfinden. Die drei letzten Fragen kamen zusammen mit drei Erwähnungen in EINER Nachricht. Diese Transportform erhalten und nicht ausschließlich als drei unabhängige Happy-Path-Anfragen prüfen.

Neueste verbindliche Präzisierung im Befund (21:15) hat Vorrang: einzige fachliche Grenze 50 Brain-Fragen je Nutzer und Kalendertag Europe/Berlin, Wert aus bot.toml. Keine Sekunden-, Stunden-/Kanal- oder globale Tagesgrenze. Folgefragen sofort, ebenfalls gegen dieselbe Tagesgrenze. K liefert diesen kleinen Schritt vorgezogen. Q prüft nach I/G/K-Livegang erste Spielfrage und direkte Folgefrage ohne Schweigen oder Zeitfenster, anschließend die Dreifachfrage als eine vollständige Brain-Anfrage mit Antworten auf alle Teilfragen. Grenzfall 50/51, nutzergetrennte Zählung und Berliner Tageswechsel einschließlich Sommer-/Winterzeit isoliert prüfen; keine produktive Spamserie oder manuelle Produktivzähleränderung. Bei erreichter Grenze genau eine sichtbare Rückmeldung, dass es morgen weitergeht. Fachliche Fragen, Nachrichtenereignisse und Modellaufrufe getrennt zählen. Kein „Frag Nani“ als Wissensfallback für Spielfragen; Coaching-/Patenweg anhand P3 prüfen. K behebt den Consumer, Q ändert keinen Botcode.

## Zusätzlicher echter Ortskontextfall, Nutzerbefund 21:30

Die reale Frage „wie kann ich auch deadlock spielen“ wurde im dafür zuständigen Discord-Bereich gestellt. Als weiteren echten Fall aufnehmen, mit lokal belegtem, minimalem und rollenbasiertem Frageort. Erwartete Antwortart: konkrete Hilfe beziehungsweise nächster Schritt, kein Verweis zurück in genau denselben Bereich. Den genauen Einladungsablauf vor dem Lauf an der gültigen Quelle prüfen, nicht aus dem früheren Fehltext übernehmen.

K liefert dafür bestehenden Antwortvertrag mit Kanal, Kategorie, Thema beziehungsweise Zweck, Thread-/DM-Kontext und Eingangsart; Twitch mit Kanal/Partnerstatus. In derselben festen Vergleichsschicht prüfen, dass Ortskontext tatsächlich verwendet wird und keine IDs oder unsichtbaren Kanalinformationen an den Provider gehen. DM-/Thread-/Twitch-Kontext und fehlende Ortsdaten ergänzend prüfen, künstliche Kontrollfälle nicht als echte Goldfragen zählen. Kein Q-Start vor I/G/K-Livegang; fremden Docs-Thread 59740e62 nicht lesen oder verwalten.

## Werkzeuge und Abschluss

Cargo ausschließlich über den freigegebenen `cargo-slot <cargo-argumente>`, keine FD/flock-Schleife oder verschachtelten Ersatzrunner. Zugewiesenen Worktree als regulären Arbeitsroot verwenden. Bei neuer Schutzablehnung genaue Ursache/fehlenden Nachweis melden, nicht umgehen. Neue Werkzeuge Rust, Persistenz Postgres. Bestehende Suites erhalten, passende Tests/Format/Clippy für eigene Änderungen. Einziger Reviewer lokaler Merge-Gate; bei BLOCK frischer nativer Fixer, gleiches Urteilmodell, spätestens nach fünf erfolglosen Runden qualifizierte Rückgabe. Keine weitere T3-Orchestrierungsebene.

Vor Browserarbeit /home/nathanael/Documents/claude-config/wissen/agent-browser.md lesen. Nur Moli, MUST NOT Brave oder persönlichen Browser verwenden. Fremde laufende Dienste unangetastet lassen.

Gebaut/reviewt/gemergt/live getrennt berichten. Eigene Artefakte regulär sichern, private Kopien erhalten. Cleanup und Self-Settle erst nach tatsächlichem eigenen Abschluss. Deutsch mit Umlauten, keine Gedankenstriche. Nur echte Blocker oder vollständige Abnahme melden, keine Rundennachrichten.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 25 min | Worktree: hauptbaum
