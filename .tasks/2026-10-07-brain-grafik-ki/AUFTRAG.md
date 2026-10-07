# Brain: Grafik-/Webausgabe und gemeinsame KI-Schnittstelle

status: aktiv, 07.10.2026

INTENT[IA-1]: Stufe groß | Modell sol high | Thread a711a4d2-1cad-4120-97ac-8b648567172b | Register: REGISTER.md
ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt dispatch | Artefakt: .tasks/2026-10-07-brain-grafik-ki/AUFTRAG.md

## Freigabe und Ziel

Der Nutzer hat den Planungsbericht `.tasks/2026-10-07-brain-feature-audit/BERICHT.md` mit „jo do it“ zur Umsetzung freigegeben. Genau zwei Produktbereiche starten: Grafik-/Webausgabe und die erste gemeinsame KI-Anbindung mit Serverguide/Paten als Anschlussfällen. Keine Wiederholung der Inventur und kein Komplettumbau aller im Inventar genannten Bots in einer Welle.

Entitätsdaten und deterministische Berechnungen sind die Grundlage. Steckbrief, Antwort, Grafik und Webseite sind Ansichten derselben Daten, kein unabhängig gepflegter Profiltext und keine neue Wertedatenbank. API-Spiegel E und Rechenkern/Werkzeugvertrag G bleiben die einzigen Quellen aktueller Werte. Historische YouTube-Klassifikationen sind nicht greifbar: kein YT-Paket, keine neue Extraktion und keine ungeprüfte Wissensübernahme. Forum, Replay und weitere Importer sind ausgeschlossen.

## Verbindliche Nutzerkorrektur: Pate = Brain = Concierge

Der Nutzer präzisiert nach dem Dispatch: „Der Pate = Deadlock Brain = Consierge ne“. Pate und Concierge bezeichnen dieselbe persönliche Brain-Hilfe, KEIN separates menschliches Patenprogramm und KEINE zweite KI-Persona oder eigener Dienst. Serverguide ist eine Fähigkeit dieser einen Hilfe. Die Auditbefunde zum menschlichen Patenprogramm sind Bestandswissen, aber nicht Teil dieses Bauauftrags. Keine Vermittlung an menschliche Paten, keine Patenrollen, keine Änderungen am menschlichen Übernahme-/Anfrageprogramm und keine Aktivierung solcher Kontakte. K entfernt dies aus seiner Bauplanung; schon vorhandene fremde Mechanik bleibt unangetastet. Zustimmungs-, Datenschutz- und Ablehnungsregeln gelten weiterhin für die tatsächlich beauftragte eine Brain-Hilfe. Diese Klarstellung hat Vorrang vor früheren Formulierungen in Briefings und Auditbericht.

## H: Grafiken und kleine Webseiten

Erster vollständiger Nutzerfall: Zwei Helden vergleichen, eine von G wirklich unterstützte Kennzahl über Boons als Grafik sowie kleine Detailseite ausgeben. Namen, Werte, Bedingungen, Quellen und Versionsstand kommen aus derselben typisierten Ergebnisstruktur. Vorhandene Rust-Renderer und den vorhandenen Rust-Port der Brain-Site selektiv wiederverwenden, nicht den abgelösten Profilpublikationsweg reaktivieren. Kein zweiter Rechner, kein beliebiges Modell-HTML/JavaScript und keine modellbestimmten Zieladressen.

Die erste Strecke verwendet ausschließlich zur Veröffentlichung freigegebene öffentliche Spieldaten. Persönliche Profile und Chattexte gehören nicht auf öffentliche Seiten. Eine ausdrückliche Grafik-/Seitenanfrage gilt als einmaliger Auftrag für diese öffentliche Ausgabe, kein zweites Ja verlangen. Interne oder private Materialien müssen abgewiesen werden. Dienst erzeugt IDs/Links; Rechte-, Quellen- und Widerrufsprüfung auch beim Abruf und Cachetreffer. Geprüfte Link-/Anhangausgabe für Discord und Twitch, ohne deren allgemeine Schutzfilter auszuschalten. Bereits vorhandene Domain-/Brainroute bevorzugen, keine neue Domain kaufen und keinen weiteren Dauerdienst beginnen.

## K: Gemeinsame KI-Anbindung und Integration

Brain wird die gemeinsame Schnittstelle für Modellaufträge und freigegebenes Wissen. Eine gemeinsame Schnittstelle bedeutet nicht ein Modell für alle Aufgaben. Vorhandene Provider, genehmigte Modelle und Spezialregeln erhalten; insbesondere keine pauschale Umstellung auf Luna oder DeepSeek und keine Änderung der Patchnotes-sonar-pro-Ausnahme.

Erste Welle vollständig liefern: kompatibler typisierter Aufgaben-/Rechtevertrag im bestehenden Brain; einen vorhandenen Discord-FAQ/Serverguide-Antwortpfad und einen nichtkritischen Twitch-Textentwurf über bestehende Clientfassaden anschließen. Twitch bevorzugt einen vorhandenen Titelentwurf mit erlaubtem minimalem Kontext; kein automatisches Titelsetzen als neue Funktion. `tb-llm` bleibt Fassade, kein zweiter Connector. Wissensabruf ist optional, nicht jeder Auftrag ist RAG. Unmigrierte Fälle ausdrücklich inventarisiert lassen, nicht behaupten, alle KI sei bereits umgezogen.

Die persönliche Brain-Hilfe übernimmt die Guide-/Concierge-Funktion; „Pate“ ist eine Bezeichnung dafür. Erhaltene Guidearbeit selektiv übernehmen, nicht alte Branches vollständig mergen. Für den anfragenden Nutzer einen durchgängigen, zustellbaren Guidefall herstellen. Ablehnungen der persönlichen Hilfe respektieren und gegebenenfalls dauerhaft speichern, erledigte Interaktionskarten verbrauchen und dieselbe Frage nicht wiederholen. Keine privaten Profile weitergeben. Rollen, Fristen, Consent, Versand und Wiederholungsschutz bleiben im Bot. Keine neuen breiten Kontaktserien, alten Mitglieder anschreiben oder pauschales Concierge-Aktivieren. Persönliches Gedächtnis mit nicht festgelegten Speicher-/Löschregeln bleibt aus; keine neue Profilablage. Offene echte Produktentscheidungen mit Empfehlung melden, nicht durch stilles Aktivieren ersetzen.

## Schutz und Betrieb

- Runtimecode in Rust, Persistenz Postgres, keine neuen Code-Kommentare. Bestehende Pythonpfade sind lesbare Referenz, produktiver Fix im Rustpfad.
- NEVER read, print or write plaintext secrets. MUST NOT send private user/community data to remote models. Infisical für Secrets, normale Configdateien für alles andere. Keine ENV-Dateien oder neue ENV-Konfiguration.
- Kein Modellwechsel, keine neuen Anbieter und kein kostenpflichtiger Rückfall ohne Freigabe. Brain-Ausfall lässt deterministische Bot-Schutzregeln aktiv; kein heimlicher Direktmodellfallback, keine Sanktion ohne Urteil.
- Bestehende OAuth- und Clientwege nutzen. Rechte über Plattform-ID und serverseitige Bindung, nie über Modelltext, Logins oder frei wählbare fremde IDs.
- Keine fremden Threads kontaktieren, stoppen oder aufräumen. Kein ListAgents/SendMessage. Fremde schmutzige Checkouts bleiben unberührt. Worktrees unter ~/.worktrees, nur eigene Änderungen committen.
- Der im Audit erwähnte Release-Halt ist nachweislich aufgehoben: `.tasks/2026-10-06-brain-abschluss/VON_HAUPT.md`, Abschnitt 07.10. ca. 09:00, und `A/RELEASEFENSTER.md`, Kopfzeile. Normaler Gate-/Deployabschluss erlaubt. Kein neuer Session-Warteslot; Builds/Deploys mechanisch serialisieren. Aktuelles origin/main deployen.
- G arbeitet weiterhin an contracts/tools, provider/kernel und Rechnung. Diese aktiven Dateien nicht parallel ändern. Zunächst disjunkte Module/Adapter bauen; integrieren erst auf geprüftem G-Vertragsstand. Das ist eine Codeabhängigkeit, kein Warten auf fremde Builds. Keine Kopie des G-WIP, keine zweite G-Implementierung. Abhängigkeit mit konkretem Pfad/Vertrag im Status melden.

## Abschluss

K ist einziger Integrator der neuen H/K-Arbeit und aller gemeinsamen Bot-/API-/Contractänderungen. Nach gemeinsamer Intent-Abnahme und regulärem Merge-Gate-ALLOW autonom mergen, pushen, regulär deployen, Dienste neu starten, reale Strecke mit öffentlichem Material/Wegwerfkonten prüfen und eigene Branches/Worktrees aufräumen. Keine PRs oder Actions. Kein Versand im Namen des Nutzers und keine Ankündigung ohne gesondertes Go. Keine produktiven Konten trennen oder deren Adressen rotieren.

Rustprüfungen passend zum Diff: Format, Compiler, Clippy und vorhandene Suites, Baselinefehler ehrlich ausweisen und neue Regressionen beheben. Keine Pflicht für zusätzliche Tests, aber tatsächliche Wirkungsbelege. Grafik-/Webseite im Browser prüfen, Desktop/Mobil und Screenshot; kein Zahlenmock als Livebeweis. Bei Gate-BLOCK frischer nativer Fixer je Runde, nach spätestens fünf erfolglosen Runden echte Blockade melden. Review und Livebeweis an den gelieferten SHA binden.

Modellwahl: GPT 6.1 Sol, Effort high, Claude-Code-Hauptsessions mit eigenen nativen Subagenten und UltraCode-Workflow. High bleibt high, nicht über `--effort ultracode` auf xhigh wechseln. `enable_workflows=true` ist vorhanden; tatsächlichen Workflow-Agentenlauf belegen, keine Aktivierung aus Konfiguration allein behaupten. Fehlende Laufzeitfähigkeit melden, native Sol-high-Agenten verwenden statt Modell/Settings eigenmächtig zu ändern.
