# Invite-Skill: verbindlicher Minimalvertrag

Stand 06.10.2026. Ergänzung aus direkter Hauptorchestrator-Nachricht und `../VON_HAUPT.md`, Klarstellungen 23:20 und 23:25. Gilt für die anschließende Umsetzung, auch wenn die erste Inventur noch die ältere Datenschutzgrenze referenziert.

## Wirkung

Der echte eigene Einladungsstatus wird über einen lesenden Skill des gemeinsamen Brains zeitnah beantwortet. Bots speichern Status und stellen die Brain-Antwort zu. Bestehende Audit-/Statusquelle verwenden, keinen zweiten Store, Connector oder Antwortweg bauen. B besitzt den Mechanikfix und darf die gemeldete fehlerhafte verspätete Lounge-Antwort sofort entfernen, ohne auf A zu warten. Andere gewollte funktionierende Antworten bleiben bis zum live belegten Ersatz erhalten.

## Datenfreigabe

Ausschließlich eigener Status der fragenden Person als Enum plus Zeitpunkt darf an den bestehenden Antwortweg und bestehenden Provider gehen. Beispiele sind verschickt, ausstehend, Freundschaft fehlt und Fehler. Tatsächliche Quellsemantik entscheidet die präzise Enumzuordnung. Unbekannt und Transport-/Lesefehler nie als Erfolg interpretieren.

Personenzuordnung, Steam-/Discord-IDs, Namen, Auditrohzeilen, fremde Zustände und zusätzliche personenbezogene Felder bleiben intern. Die erlaubte Projektion wird nach stabiler Identitäts- und Rechteprüfung gebildet. Keine Auswahl einer dritten Person anhand des Fragetexts. Auch der tatsächliche Modellpayload einschließlich Frage, Kontext und Belegen muss diese Grenze einhalten, nicht nur das neue JSON-Feld. Keine Annahme, die Spielwertefreigabe sei allgemeine Communityfreigabe. Kein Modell-/Zeitlimitwechsel.

## Abnahme

Eigener tatsächlicher Status, kein Fremdstatus; fehlende Identität, unbekannt, Fehler und alter Stand verständlich unterscheidbar. Gemeinsamer Request/Consumer/Sender statt Parallelroute, keine Bot-Statusformulierung. B-Mechanikdateien bleiben außerhalb des A-Eigentums. Bestehende Rechte-/Scopes deterministisch erhalten. Einen echten lesenden Skillpfad und den tatsächlich abgeschickten bereinigten Providerpayload isoliert prüfen; keine öffentlichen Community-Testnachrichten und keine personenbezogenen Inhalte in Workerberichte oder externe Reviewprompts. Code und Konfiguration dürfen regulär geprüft werden.

Read-only Inventur A-E2 liefert zuerst die Fundstellen und Wiederverwendung. Erst dann eigenes Implementierungseigentum vergeben. Keine zusätzliche Produktentscheidung oder Planfreigabe für den bereits ausdrücklich entschiedenen Vertrag nötig.
