# S04: Schnittstellenanforderungen vor Implementierung

Antragsteller: Chat 04. Basis: `c00fc8935048bf490c1e4790f7c6195864ad49e2`. Plan 1.0. Contract-/Schema-Version: offen.

**Alle folgenden Anträge sind vorgeschlagen, nicht freigegeben oder implementiert.** Namen beschreiben Semantik aus dem Plan; es werden keine privaten Ersatztypen, Wireformate, Tabellen oder Migrationsnummern festgelegt. Chat 00 und die jeweiligen Besitzer entscheiden und integrieren.

## CR-S04-001: Source-, Policy- und HTTP-Vertrag

Besitzer: 02/03; HTTP-Abstimmung mit 07, Quellenfamilien mit 12/13/14. Requirements: R05, R06, R08, R09, R27, R38, R42.

Betroffene vorhandene Pfade: `rust/crates/deadlock-brain-core/**`, `rust/crates/dbrain-sources/src/store.rs`, `rust/crates/dbrain-sources/src/error.rs`. Gemeinsame neue Vertragsdateien bestimmt 02. S04 ändert diese hier nicht.

### Benötigte Änderung

Der derzeitige Dokument-Input und seine Hash-Deduplikation sind kein vollständiger Source Contract v2. Benötigt werden die gemeinsamen Typen für stabile Quell-/Recordidentität, Revision und Ereignisordnung, Raw-Hash/Locator, getrennte Abruf-/Quell-/Spielzeiten, Parser-/Schema-/Derivationsversionen, Elternreferenzen, Validation/Trust sowie serverseitige Zugriffs-, Lizenz- und Egressentscheidung. Unbekannte Daten bleiben unbekannt; Abrufzeit ist keine Spielversion.

Inhaltsgleichheit muss unabhängig von Metadaten-/ACL-/Tombstoneänderungen ausgewertet werden. Gleicher Inhalt mit strengeren Rechten verändert die effektive Sichtbarkeit, ohne deshalb erneut eingebettet werden zu müssen. Gleiche Bytes aus zwei Quellen behalten getrennte Identitäten und Rechte. Verspätete Revisionen dürfen weder aktuellere Inhalte überschreiben noch eine Löschung oder Sperre rückgängig machen. Wiederfreigabe benötigt ein ausdrücklich erlaubtes neueres Ereignis.

Fetch-Ergebnisse müssen Authfehler, Rate Limit, Retry-After, Timeout, Transportfehler, zu große Antwort, unvollständige Pagination und validierungsbedingte Quarantäne unterscheidbar machen. Grenzen gelten auch für Dekompression, Weiterleitungen und Wiederholungs-Gesamtdeadline. Ein 401/403 ist kein leeres erfolgreiches Quellmanifest und kein Anlass, Schutz zu umgehen. Der vorhandene spezielle `allow_forbidden`-Pfad wird nicht als Freigabe für neue interne Connectoren übernommen.

Für private Quellen darf der vorhandene URL-only-Cache nicht unverändert zwischen Principals/Scopes wiederverwendet werden. 02/07 entscheiden einen verifizierten Policy-/Auth-kontextgebundenen Cache oder einen explizit cachefreien sicheren Pfad; Logs enthalten keine Header, Tokens oder ungeprüften Quell-/Fehlerbodies. Lokale Quellen benötigen fest konfigurierte Wurzeln, Pfad-/Symlinkgrenzen und Größenlimits.

### Folgen und Übergang

Bestehende öffentliche Adapter bleiben erhalten. Unklassifizierte Altdaten werden durch fehlende Policyfelder nicht automatisch öffentlich. S04 implementiert nach Freigabe nur gegen die integrierten Typen. Fachparser bleiben bei 12/13/14.

Verpflichtende Regressionen: identischer Text mit ACL-Revoke; sourceübergreifende Hash-Dublette; alte Revision nach Delete; Cachegleichheit bei verschiedenen Authkontexten; verbotene Weiterleitung; 403-/429-/Timeout-/Paginationfehler; Pfadausbruch. Siehe Pilotfälle P03–P09 und P15–P17.

Entscheid Besitzer + 00: offen. Neue Version / Freigabe-ADR: offen.

## CR-S04-002: Dauerhafte Raw-, Job-, Checkpoint- und Publish-Semantik

Besitzer: 03 mit 02; Release-/Retrieval-Abstimmung mit 06/08/12. Requirements: R08, R17, R18, R22, R38, R40, R49.

Betroffene vorhandene Pfade: `rust/crates/dbrain-sources/src/store.rs`, `rust/crates/deadlock-brain/src/wiki_refresh.rs`. Reale Schema-/Migrationspfade und deren Ownerfreigabe liefert 03; kein DDL in diesem PR.

### Benötigte Änderung

S04 benötigt einen gemeinsamen dauerhaften Port für Auftragsannahme, Claim/Lease mit Schutz gegen abgelaufene Worker, begrenztes Retry und Quarantäne, monotones Checkpoint-Update sowie einen wiederholbaren Publish-/Invalidierungsauftrag. Ein Run-Status oder eine lokale zweite Checkpointdatei genügt nicht.

Die minimal erforderliche Zusage lautet: Bevor eine Eingangsseite bestätigt wird, müssen ihre validierten Raw-Referenzen dauerhaft verfügbar sowie Revision/Änderung, Checkpoint und noch nötige Folgearbeit nach der von 03 definierten Transaktionsgrenze dauerhaft gesichert sein. Kein Ack darf ausschließlich auf einem In-Memory-Erfolg beruhen. Crash nach Commit vor Ack führt zu Wiederholung ohne doppelte Aktivierung. Eine dauerhafte Quarantäne darf quittiert werden, falls Sperre und Wiederaufnahmemöglichkeit erhalten bleiben; sie darf niemals als veröffentlichter Erfolg erscheinen.

Raw-Datei und PostgreSQL werden nicht ohne Nachweis als gemeinsame ACID-Transaktion bezeichnet. 03 legt Staging, Hash-/Längenprüfung, atomare Finalisierung, Referenzierbarkeit und Bereinigung verwaister Artefakte fest. Ein beim Schreiben abgebrochenes vorhandenes Raw-File darf bei Retry nicht allein wegen seiner Existenz akzeptiert werden.

Löschungen und Rechteentzüge sperren effektive Sichtbarkeit unabhängig von asynchronem Reembedding und historischen Releases. Nachfolgende Index-/Cache-/Karten-Invalidierung ist dauerhaft und idempotent. Fehlgeschlagene Teilabrufe dürfen keine Massenlöschung aus scheinbar fehlenden Mitgliedern erzeugen. Aufbewahrung/GC benötigen eine freigegebene Regel; es gibt in S04 keine ungefragte Datenlöschung.

Staging wird validiert und danach über genau einen gemeinsamen Knowledge-Release-Vertrag aktiviert. Der vorhandene Wiki-Publikationspfad wird integriert, nicht durch einen zweiten konkurrierenden Zeiger überlagert. Rebuild nutzt freigegebene Raw-Revisionen und aktuelle Rechte; ein älterer Snapshot kann Rechteentzüge nicht zurücknehmen.

### Folgen und Übergang

Migrationen und Schema-Nummern ausschließlich durch 03. S04 benötigt eine reproduzierbare isolierte Testdatenbank, Test-Rechte und Cleanup-Regeln sowie einen leeren Store-Rebuildpfad. Produktive Writer bleiben bis 11 unverändert. Keine Exactly-once-Zusage allein aus Retries oder Deduplikation.

Verpflichtende Regressionen: Crash vor Raw-Finalisierung, vor Transaktionscommit, nach Commit vor Ack und während Publish; Leaseverlust mit zweitem Worker; Partial Crawl; Delete/ACL gegen alten Release; Rebuild und Deltaabgleich. Siehe P09–P14 und P19.

Entscheid Besitzer + 00: offen. Neue Version / Freigabe-ADR: offen.

## CR-S04-003: Ressourcen- und Feedergrenzen

Besitzer: 00/02 für Budgetkonfiguration, 06/07 für Chunk-/Embedding-/Decision-/Answer-Ports, 10 für messbare Abnahme; Modulbeiträge von 12/13/14. Requirements: R09, R13, R14, R20, R27, R38, R59.

### Benötigte Änderung

Vor Implementierung ein integriertes Budgetprofil bereitstellen: begrenzte Queue, Speicher-/Bytegrenzen, CPU-/I/O-Slots, Gesamtdeadline, Retry/Backoff/Jitter, Abbruchverhalten und getrennte Kapazitäten für reguläre Deltas, Bulk, Replay und Embedding. Konkrete Zahlen kommen aus 01/10 und einer Entscheidung von 00; keine erfundenen p95-/p99-Ziele. Delete/Revoke muss auch bei ausgelasteten Bulkqueues wirksam werden. Blocking-Arbeit darf Live-Tokio-Tasks nicht unkontrolliert belegen.

06 liefert Parser-/Chunk-/Nachbarschafts- und Embeddingrevisionen mit der No-op-/Invalidierungssemantik. Gleiche Raw-Revision allein genügt nicht für No-op, falls Policy, Parser, Template, Schema oder nachgelagerte Konfiguration geändert wurde. Unveränderte gültige Ableitungen werden nicht grundlos neu eingebettet. Ein Parserupgrade ist kein neuer Spielpatch.

Der Feeder braucht getrennte Decision- und Answer-Ports aus 07. Typisierte Relevanzentscheidungen starten im Shadowmodus; sie ändern weder Quellrechte noch autorisieren sie Egress. Generatives Schreiben benötigt erlaubte Eltern, konservative Rechte und eindeutige Kennzeichnung als abgeleitet. Digest-Nachkommen dürfen nicht als neue Originalereignisse erneut zur Generierung zugelassen werden. Revoke, Quarantäne, Redaction und Retention der Eltern wirken auch auf Ableitungen.

### Folgen und Übergang

Keine neuen direkten Providerclients, fest kodierten Modelle, allgemeinen Zahlenlimits oder dauerhaften Pythonhelfer in S04. Der zuerst abzunehmende Datei-Pilot benötigt kein generatives Modell. Feeder-A/B und gemischte Live-Last sind spätere echte Messungen, keine Fixture-Erfolgsmeldungen.

Verpflichtende Regressionen: langsamer Bulk-/Replayjob unter Liveverkehr, Queuevollstand, Abbruch/Leasefreigabe, gezielte Dependencyinvalidierung, Decision-Ausfall, unerlaubter Egress, gemischt berechtigte Eltern und Digestrekursion. Siehe P04, P08, P18–P20.

Entscheid Besitzer + 00: offen. Neue Version / Freigabe-ADR: offen.

## CR-S04-004: Eindeutige Pfade und Legacy-Feeder-Übergabe

Besitzer: 00 mit 01/04/09/11/12/13/14. Requirements: R03, R04, R05, R23, R24, R58, R60.

### Benötigte Änderung

Das vorhandene Ownerregister überlappt für CLI, Sources und Wiki-/API-/Replaydateien. 00 soll die tatsächlichen S04-Dateien nach G1 explizit freigeben und die anderen Besitzer behalten lassen. Vorgeschlagen ist Erweiterung vorhandener CLI-/Sources-Strukturen; keine Vorgabe einer neuen Crate und kein Root-/Lockfile-Edit durch S04.

01/09 liefern für den bisherigen Docs-Feeder den integrierten Code-Commit, Entrypoints, Writer, Nachrichten-/Ackverhalten, deduplizierte Ereignistypen, aktive Quellrechte, Redaction/Retention und erlaubte Testbelege. Dieser S04-Lauf hat den Legacy-Feeder nicht bis zur Implementierung verifiziert. Keine privaten Dokumente oder Sessioninhalte als öffentliche Fixture kopieren.

Auf der integrierten Basis liegt das Gesamtpaket nur als versioniertes Root-ZIP vor. 00 soll vor der Implementierungsfreigabe auch die im Plan geforderte entpackte Paketablage unter `architecture/migration/` integrieren. S04 verändert diese gemeinsame Ablage nicht.

Der auf der Basis belegte Python-Secrets-Wrapper ist eine Übergabe an 11, nicht Erlaubnis zum Abschalten oder Ändern eines aktiven Timers. Späterer Rust-Betriebsnachweis muss normale Versorgung und leeren Rebuild ohne Pflicht-Python zeigen. Existierende Rust-Helfer oder offene andere PRs sind ohne Integration kein Deploymentnachweis.

### Folgen und Übergang

S04 beginnt nach integriertem G1 und Pfadfreigabe mit genau einem deterministischen Datei-/Dokument-Pilot. Docs-, GitHub-Event- und Sessionquellen folgen nur mit belegter Herkunft und Freigabe. Externe Fachadapter und Replaydecoder werden nicht von S04 neu gebaut.

Entscheid Besitzer + 00: offen. Freigegebene Pfade / Basis-Commit / nächste Welle: offen.
