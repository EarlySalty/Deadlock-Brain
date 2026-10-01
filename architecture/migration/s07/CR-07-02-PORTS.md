# CR-07-02 · Gemeinsame Providergrenzen vor Implementierung

Absender: S07. Empfänger: **00/02**, mit Abgrenzung zu 03, 04, 06, 08, 09, 10.
Status: vorgeschlagen, nicht angenommen.
Basis: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.
Anlass: G1 offen; `PFAD_OWNER.csv` reserviert S07 nur vorläufig im gemeinsamen Core.

## Benötigte Entscheidung

Keine neuen `brain-providers`-/`brain-jev`-Crates oder privaten Traits anlegen.
S02 soll in den vorhandenen gemeinsamen Verträgen Folgendes festlegen; S07
implementiert danach den Transport im bestätigten Pfadumfang:

| Thema | Benötigte Zusage | Besitzer |
|---|---|---|
| Modellkonfiguration | Explizite Settings-/Env-Präzedenz, genehmigte Familie, Alias-/Antwortrevision, Cache-Key und Invalidierung. Kein automatischer Wechsel zu fremder Familie. | 02/07 |
| Transportbudget | Eine Gesamtdauer einschließlich Warteschlange, Resolver, Retries und Bodylesen; Cancel-Signal und getrenntes Connect-/Read-Limit. Timeout-/Byte-/Retrywerte konfigurierbar; bestehende Userentscheidung nicht eigenmächtig ersetzen. | 02/07 |
| Fehler und Nutzungsdaten | Getrennte Policy-, Auth-, Limit-, Timeout-, Transport-, Schema- und ModelNotFound-Fehler; optional belegte Tokens/Kosten statt erfundener Nullen. | 02/07 |
| Rechte-/Egresskontext | Authentifiziert vom Aufrufer, nicht aus Prompt oder Modellantwort. Bereits erlaubte Datenklassen, Quellen, Zwecke, Provider und Budget als obere Grenze. | 02/08/09 |
| Jev-Ergebnisse | Noul, Choice, Score getrennt; rohe gültige Verteilung und Rubrikrevision erhalten. Entscheidung auf deterministische Baseline begrenzen. | 02/07/08 |
| Modellcache/Persistenz | Keine unversionierten globalen Aliaswerte; persistenter Cache nur über vorhandenen Postgres-Port und Schemaowner. Kein SQLite-/Dateistore-Ersatz. | 02/03/07 |
| Embeddings | Provider-/Modellrevision, Dimension, Normalisierung, Eingabereihenfolge und Corpus-/Indexrevision. Reindexierung ist keine implizite S07-Nebenwirkung. | 02/03/06/07 |
| Dateien/Tests | Genaue S07-Unterpfade im Core und Eigentum an Vertragstests bestätigen; generischen Quellen-HTTP-Cache nicht unbeabsichtigt ändern. | 00/02/04/07 |

## Verbindliche Grenzen für den Vorschlag

Auswahl eines erlaubten Providers setzt keine Datenfreigabe für diesen Provider
voraus. Beide Bedingungen müssen vor jeder Netzwerkstufe geprüft werden, auch
bei Resolver, Retry und Redirect. Keine automatisch gefolgten Redirects im
Providerprofil. Nur exakt erlaubte HTTPS-Ziele; Loopback-HTTP ausschließlich
im isolierten Testprofil ohne echte Credentials. Fremde/private Ziele,
URL-Credentials, DNS-Rebinding und Metadatenziele brauchen negative Tests.

Keine Credentials, vollständigen URLs mit Secrets, Requesttexte oder
ungefilterten Providerfehler in Logs. Metriklabels erhalten feste niedrige
Kardinalität statt Query-/User-IDs. Inhalte in einem Request dürfen die erlaubte
Quellen-/Providerliste nicht erweitern. Fallback ist lokal/deterministisch;
er darf keine zusätzliche Übermittlung an einen anderen Anbieter auslösen.

## Implementierung nach tatsächlicher Freigabe

1. Integrierten G1-Commit, Contracts und Besitzer neu lesen. Zuerst rote
   Rust-Tests am unveränderten zentralen Client; danach verifizierte Härten.
2. Persistenten begrenzten Transport mit gemeinsamer Deadline und Policy bauen.
   404-Neuauflösung und alle Wiederholungen zählen zum selben Budget. POST-
   Wiederholung nach unklarem Ausgang kann doppelt kosten und ist nicht
   stillschweigend „idempotent“.
3. Jev-Adapter und fünf getrennte Schalter hinter freigegebenen Ports ergänzen;
   kein Callerwechsel außerhalb der bestätigten Grenzen. Fixture-/Faulttests
   automatisieren, dann getrennt autorisierte Live-Vertragstests.
4. Erst mit S10-Labels Shadow bewerten; Aktivierung je Funktion an G4.

## Weitere Übergaben, keine Änderungen in anderen Paketen

S09/S11 erhalten den Codebefund `deadlock-brain-yt/src/gemini.rs`:
Python-Browserworker existiert im Basisstand. Ein S07-Port darf diesen Pfad
weder still entfernen noch den Gesamtbestand als Python-frei deklarieren.
S06 erhält die ungeklärte Embeddingkompatibilität; S10 liefert Rechteprüfung,
Entwicklungs-/Holdoutlabels und vorab festgelegte Abnahmekriterien.
