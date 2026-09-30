status: aktiv
Datum: 2026-09-30

# Brain vollständig abschließen und nach Nachweisen produktiv umstellen

Intent-Thread: 562a877b-0939-440a-964d-1145d9e9431a
Vorgänger: .tasks/2026-09-29-technical-closeout/
Ausgangsstand: migration/rust-integration 1c362bca6d35e7fec10125b2b159e7513a299243, Produktcode 022f8a981c2164f6d8d4302bae2194e100c4f65c.

## Neue ausdrückliche Nutzerentscheidungen

Nutzerauftrag: „Mach alles fertig“.
Auf die Frage „Gilt ‚alles fertig‘ ausdrücklich als Freigabe für Brain-Main-Merge, Deployment und produktive Consumer-Aktivierung, sobald die offenen Voraussetzungen nachgewiesen und die lokalen Gates bestanden sind?“ antwortete der Nutzer: „Ja, nach allen Nachweisen“.
Auf die Frage „Muss die erste produktive Version echte Replays verarbeiten können?“ antwortete der Nutzer: „Nein, Replay später“.

Damit ist die bisherige Main-/Produktionssperre bedingt aufgehoben, nicht sofort. Replay gehört ausdrücklich nicht zur ersten produktiven Version. Daraus folgt kein Recht, fehlende Lizenzen, Echtnachweise oder noch ungeklärte Wiki-/Provider-Freigaben als vorhanden zu melden. Keine ungefragten Discord-/Twitch-Nachrichten oder echten Steam-Build-Veröffentlichungen. Keine Replaydownloads.

## Arbeitsumfang

1. Replay aus der ersten produktiven Version sauber abtrennen. Bestand und Abhängigkeiten über Graphify prüfen. Keine nur optional markierte, aber weiterhin frisch aufzulösende private Git-Abhängigkeit als Lösung ausgeben. Frischen Build ohne bisherige Cargo-Gitobjekte nachweisen. Bestehende Replay-Arbeit erhalten, keine Tests zur Täuschung abschwächen und keinen Erfolg für ausgeschlossene Replaytests behaupten.
2. Alle noch offenen G5-Voraussetzungen anhand der maßgeblichen Dokumentation und Live-Konfiguration ohne Secret-Ausgabe prüfen. Insbesondere Wiki-Quelle/Lizenz/Aufbewahrung, freigegebener zentraler Modell-/Providerpfad, Budget, Egress, PostgreSQL-Service-Start, Restore/Rollback und tatsächlicher Deploy-/Consumer-Weg. Tatsächliche Betreiberentscheidungen gesammelt melden, technische Fragen selbst lösen.
3. GitGuardian-Vorfall 37635766 über den normalen zugänglichen Prüfweg behandeln; keine History-Umschreibung oder Policyumgehung. GitHub Actions sind kein Merge-Gate; echte Compiler-, Sicherheits- und Abhängigkeitsfehler bleiben technische Arbeit.
4. Consumer-PRs Bots #459, Docs #4, 2nd-Brain #2 gemeinsam mit dem finalen Brain-Stand unabhängig abnehmen. Neue bedingte Freigabe umfasst deren Integration und Aktivierung erst nach erfüllten Voraussetzungen. Twitch #984 und Steam #82 sind bereits gemergt; tatsächliche Laufzeit nicht daraus ableiten.
5. Finale lokale Prüfungen auf dem endgültigen Stand, unabhängige Intent-Abnahme, lokales Merge-Gate. Danach Brain #40 und nötige Consumer-PRs regulär integrieren, kontrolliertes Deployment mit Rollback, Dienststart und nicht destruktiver Live-Beweis. Keine Echt-Nachrichten oder Publishes als Test.
6. Dokumentation und Register mit tatsächlichen Nachweisen abschließen, eigene Worker settlen, Überwachung löschen. Fremde oder ausdrücklich geschützte alte Arbeitsbäume und Branches erhalten; kein Reset, Merge-Abbruch, Force-Push oder Massen-Cleanup.

## Arbeitsregeln

Hauptsession integriert; Worker committen und pushen nur eigene Branches. Bestehende registrierte T3-Threads fortsetzen, ein schreibender Worker je Worktree, keine Unteragenten. Kleine Pakete Luna, größere Sol, unabhängige Abnahme Astra. Rust für neuen Code und Werkzeuge, keine neuen Code-Kommentare. Secrets nur über den bestehenden Infisical-Pfad in den Prozess, niemals ausgeben oder in Dateien schreiben. Normale Konfiguration in Dateien, keine neue Environment-Konfiguration. Bestehende Schutz-Hooks nicht umgehen.

## Fertigkriterium

Entweder nachgewiesene produktive erste Version ohne Replay mit überprüften Consumerpfaden und erfüllten Freigaben, oder ein konkreter, nicht technisch auflösbarer Blocker mit belegtem Ort und genau benötigter Entscheidung. Ein bloßer Dispatch oder erneuter Statusbericht ist kein Abschluss.

## Ergänzung zur Ressourcenkoordination vom 30.09.2026

Der Nutzer hat die vorhandenen Freigaben ausdrücklich bestätigt und die gezielte Fortsetzung beauftragt. Keine erneute Freigabefrage. Der Twitch-/DL-Integrator unter `/home/nathanael/Documents/.tasks/2026-09-30-twitch-alles-live/` hält die Build- und Deploykoordination. Vor jedem Cargo-Build/Compilerlauf oder Dienstwechsel müssen genauer Befehl, vorhandener Cache, erwartete Ressourcen und Dienste in `BRAIN-G5-BUILD-REQUEST.txt` dort stehen und in diesem Thread gemeldet sein. Bis zur konkreten Slotzuteilung keine Builds, Benchmarks, Modellaufrufe oder Restarts. Höchstens ein Releasebuild, keine neuen Caches, kein paralleler Produktivwechsel zum Twitch-/DL-Cutover.

Statische Bestandsaufnahme und Änderungsvorbereitung bleiben erlaubt. Das Slotverbot umfasst auch Implementierer-/Revieweraufrufe und Hook-Aufrufe mit Modellreview. Die Hauptsession umgeht es weder mit einem anderen Modell noch durch eigenen Compilerstart. Bestätigte Arbeitsorte und konkreter Prüf-/Serveplan stehen in `REGISTER.md` und `PRUEF-SERVE-CUTOVERPLAN.md`.
