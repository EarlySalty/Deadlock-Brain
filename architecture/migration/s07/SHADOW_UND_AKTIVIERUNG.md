# S07 · Shadowplan und Aktivierungsentscheidungen

Status: **keine Shadowmessung, keine Aktivierung**. Die JSON-Datei
`AKTIVIERUNGSENTSCHEIDE.json` ist ein überprüfbares Entscheidungsprotokoll,
keine Runtimekonfiguration. Die Requestfixtures sind Rubrikentwürfe v0.1,
keine kalibrierten Produktionsprompts.

| Funktion | Vorschlag für Bewertung | Sichere Baseline bei Ausfall oder ausgeschaltetem Schalter | Entscheidung |
|---|---|---|---|
| Relevanz | Score auf vollständiger erlaubter Evidenzpassage | Bestehende deterministische Auswahl beibehalten; relevante Evidenz nicht aufgrund eines fehlgeschlagenen Modells verwerfen | aus |
| Feederklassifikation | Choice aus freigegebenen Quellentypen | Bestehendes Quellen-/Parserrouting erhalten, unklare Inhalte zur bestehenden Prüfung; keine Rechteausweitung | aus |
| Queryrouting | Choice: strukturierter Lookup, Erklärung, Enthaltung | Vorhandener deterministischer Router; strukturierte Datenabfrage benötigt keinen Pflichtmodellcall | aus |
| Answerability | Noul zur Beantwortbarkeit mit bereitgestellter Evidenz | Vorhandene Evidenzprüfung und sichere Enthaltung bei fehlenden Belegen | aus |
| Modellrouting | Choice ausschließlich aus bereits erlaubten Optionen | Konfigurierte erlaubte Baseline; kein Providerwechsel und keine Erhöhung von Budget oder Datenfreigabe | aus |

Die Tabellenbaseline ist eine Implementierungsanforderung an die freigegebenen
Ports. Sie wird durch diese Dokumentationsänderung nicht in die Runtime eingebaut.
Insbesondere ist „aus“ hier kein Beweis über die Konfiguration eines anderen
laufenden Dienstes, sondern der Freigabestatus für dieses S07-Vorhaben.

## Versuchsdesign für S10 und S07 nach G1

Vorab eine freigegebene, deduplizierte Datenmenge mit separaten Entwicklungs-
und Testlabels bereitstellen. Patch, Sprache, Quellentyp, Aufgabenart und
Berechtigungsklasse erfassen. Eng verwandte Ausschnitte derselben Quelle,
Replay-/Matchdaten oder fast gleiche Fragen nicht über beide Splits verteilen.
Thresholds ausschließlich auf Entwicklung kalibrieren; danach Modell-/Rubrik-
und Konfigurationsrevision einfrieren. Eine Revision oder Quellenänderung
macht betroffene Freigaben erneut prüfpflichtig.

Baseline und Shadow erhalten dieselben erlaubten Eingaben. Shadow verändert
keine Antwort, Auswahl, Veröffentlichung oder Feederaktion. Privacyprüfung
und Kostenbudget gelten trotzdem vor einem Shadowaufruf: „nicht wirksam“
bedeutet nicht „darf Daten übertragen“. Synthetische Fixtures ersetzen keine
fachlichen Labels.

## Messschema, noch ohne Werte

Je Funktion und Schicht mindestens erfassen: relevante Evidenz vor/nach
Vorschlagsfilter, falsch verlorene relevante Evidenz, Fehlklassifikationen,
unberechtigte Routen, unbelegte Antworten, Enthaltungen sowie Aufrufanzahl.
Für Vorher/Nachher-Proben Rechte- und Quellenstand konstant halten.

Kostenbilanz = Jev-Aufrufe + Resolver/Retry-Netzwerkrunden + verbleibende
Antwortmodellaufrufe + Fallbacks. Eingabe-/Ausgabetokens, Cacheangaben,
Latenzverteilung einschließlich Warteschlange und Fehlerquote getrennt
speichern. Fehlende Provider-Usage oder fehlender Tarifstand ergibt
`unbekannt`, nicht Null. Preise nur mit gültigem Tarifzeitpunkt; Anbieter-
Benchmarks sind keine eigenen Performanceergebnisse.

S10 legt Zielwerte, Mindeststichprobe und tolerierbare Unsicherheit vor dem
Holdout fest. Ohne diese Werte und einen gemessenen Nettonutzen bleibt die
jeweilige Funktion aus. Keine generische Schwelle wie 0,5/0,8/0,9 als Default
übernehmen. Rechte und harte Regeln haben unabhängig vom Messwert Vorrang.

## G4-Entscheidung je Funktion

Erforderlich: aktuelle Rust-Fault-/Contracttests, autorisierter realer
HTTP-Vertrag, getrennte Holdoutauswertung, kein unvertretbarer Evidenzverlust,
belegte Gesamtkosten und Latenz, Rückschalter zur Regelbaseline sowie
benannte Freigabe durch 00. Ein positiver Entscheid für Relevanz aktiviert
weder Modellrouting noch eine andere Funktion. Alle fünf Entscheidungen
bleiben hier mangels Nachweisen blockiert.
