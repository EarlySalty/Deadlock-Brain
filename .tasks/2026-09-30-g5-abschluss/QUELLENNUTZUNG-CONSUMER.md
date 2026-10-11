status: aktiv, Vertragszuordnung statisch geklärt; positiver produktiver Consumerbeleg offen
Datum: 2026-09-30

# Quellenrechte und tatsächlicher Twitch-Erfolgsmaßstab

## Ergebnis

Mit den gegenwärtigen B2-Policies ist keine positive öffentliche Antwort des vorhandenen Twitch-Explain-Consumers aus diesen beiden Quellen belegbar. Interne technische Übernahme und Bereitstellung sind bereits beauftragt. Sie verleihen keine neuen Publikations- oder Providerrechte. Eine erneute pauschale Übernahmefreigabe ist deshalb nicht erforderlich; eine erfolgreiche Chatantwort lässt sich daraus aber ebenfalls nicht ableiten.

Diese Analyse verwendet den eingefrorenen Brain-Quellstand c5951b610aa2545d2c0b43b33b5fe1906198b292 und den statisch geprüften Consumerstand fdd7a5d2de07e718dcf5509a2d12e3db6770d276. Sie enthält keinen neuen Produktionszugriff. Der vom Nutzer gemeldete Twitch-Livestand b9b ist hier nicht erneut gegen diesen Consumerdiff geprüft. Keine Policy, Config oder Quelle geändert.

## Bereits geltender Auftrag

| Beleg | Tatsächlicher Vertragsinhalt | Nicht daraus ableiten |
| --- | --- | --- |
| AUFTRAG.md:12-16 | Gesamtabschluss und Aktivierung nach echten Nachweisen; Replay später | Zusätzliche Wiki-, Publikations- oder Providerrechte |
| B2-IMPORT-BRIEFING.md:8-14,29-37 | Bestehenden Importer für die beauftragte interne Archiv-zu-Core-Übernahme erweitern; vorhandene getrennte Rollen, private Grenzen und gebundener Snapshot | Historische Pilot-ACL ungeprüft zur heutigen Nutzungsfreigabe erklären |
| Nutzerpräzisierung nach U1/U2 | Aufgaben- und Quellenverträge als vorhandene Evidenz verwenden; keine erneute Formalfreigabe wegen leerer Vorlage | approval_ref oder vollständige Widerrufslisten erfinden |
| TYPED-CONSUMER-BINDUNG.md und bestehender Twitch-Adapter | Bestehendes typed POST /v1/answer, lokale Verbindung, freigegebene öffentliche Scopes, bestehende Dienstidentität, kein Legacyfallback | Einen verweigerten Zugriff oder Health200 als nützliche Antwort zählen |

Eine spätere approval_ref muss nachprüfbar auf den inhaltlich passenden bestehenden Entscheidungsbeleg verweisen und an den tatsächlich beobachteten Snapshot gebunden sein. Der Auftrag ist kein neuer vom Agenten ausgestellter Quellenrechtsbescheid.

## Quellen und Scopes sauber getrennt

| Quelle | Historischer Pilot | Gesperrte B2-Vorlage auf c5951b6 | Konsequenz für öffentlichen Twitch-Consumer |
| --- | --- | --- | --- |
| legacy-entities | 905 aktuelle Köpfe als public/game.public früher beobachtet | visibility=public, allowed_scopes=[game.public], publication_allowed=false, provider_egress_allowed=false | game.public ermöglicht unter weiteren ACL-Voraussetzungen internes Retrieval; weder Veröffentlichung noch Explain-Providerverarbeitung daraus freigegeben |
| legacy-patchnotes | 348 aktuelle Köpfe als private/brain.legacy.review früher beobachtet | visibility=private, allowed_scopes=[brain.legacy.review], publication_allowed=false, provider_egress_allowed=false | Kein öffentlicher Twitch-Scope. brain.legacy.review nicht als Lösung zum Twitch-Grant hinzufügen |
| API/Assets/History/Wiki/öffentliche Patchquellen | Quellenadapter und teilweise Repository-/Metadaten-Pins dokumentiert | Kein in diesen Belegen nachgewiesener passender produktiver Publikations- und Egressgrant | Keine Ersatzquelle allein aus öffentlicher Erreichbarkeit oder Code-Lizenz freigeben |
| Deadlock-Docs/2nd-Brain | Gemischt öffentliche/interne beziehungsweise interne Bestände im Register | Daten-/Egressfreigabe in den geprüften Registern nicht nachgewiesen | Keine pauschale Übernahme in öffentliche Twitch-Scopes |

Wichtige Korrektur: Die vorhandene B2-Vorlage setzt nicht beide Sichtbarkeiten auf private. Nur Patchnotes sind dort private; Entities behalten public/game.public. Beide Quellen haben jedoch Publikation und Egress ausgeschaltet. Die B2-Prüfung erzwingt private/brain.legacy.review für Patchnotes, nicht private für Entities. Die gesperrte Vorlage mit production_binding=null ist zudem keine tatsächlich ausgeführte Produktionsbindung.

Quellen: ops/brain-postgres/legacy-core-cutover.json; rust/crates/brain-legacy-import/src/cutover.rs:67-164; architecture/migration/LEGACY_CORE_MIGRATION.md:81-82; frühere echte Metadatenbelege in ARCHIV-METADATEN-1347.md. Frühere Counts sind keine aktuelle Zustandsinventur.

## Was die vorhandenen Quellenbelege leisten

- architecture/migration/inventory/QUELLENREGISTER.csv weist mehrere Adapter als verifiziert aus, Lizenz-/Daten-/Egressfelder aber als not_verified. Dies ist eine Bestandsaufnahme, kein produktiver Nutzungsgrant.
- architecture/migration/handoffs/s13/QUELLENREGISTER.csv trennt beobachtete öffentliche Repository-/OpenAPI-Metadaten ausdrücklich von not_approved, code_license_is_not_data_license und data_egress_not_approved. Öffentliche Metadaten legitimieren nicht automatisch das Verarbeiten ihrer Inhalte durch den Antwortprovider.
- architecture/migration/s12/SOURCE_CHECK.md dokumentiert historisch HTTP403 bei der Wiki-Rechteabfrage. Keine belastbare Live-Rechteantwort daraus vorhanden.
- architecture/migration/s12/reports/SOURCE_MANIFEST.json:2-12 bezeichnet ausschließlich selbst erstellte synthetische Testdaten, publication_allowed=false und provider_egress_allowed=false. Dies sind keine echten Wiki-Rechte.

Damit ist in den tatsächlich geprüften Belegen keine zusätzliche vorhandene Quelle mit vollständigem passendem Grant für den aktuellen öffentlichen Explain-Pfad nachgewiesen. Das behauptet nicht, dass weltweit keine solchen Rechte existieren. Fehlende Belege bleiben konkret offen, statt eine neue allgemeine Nutzerentscheidung zu unterstellen.

## Tatsächlicher Antwortpfad

1. Twitch rust/crates/tb-knowledge/src/brain.rs erstellt domain=None, profile=AnswerProfile::Explain und requested_scopes aus der vertrauenswürdigen Konfiguration. Er wechselt nicht automatisch zu Fact.
2. Brain rust/crates/brain-kernel/src/execution.rs prüft zunächst Retrieval und ACL. Ohne Treffer folgt InsufficientEvidence, bei unzulässiger Evidenz UnauthorizedEvidence. Providerfreie Kurzpfade für Fact oder explizite typisierte Domänen passen nicht zur gegenwärtigen Twitch-Abfrage.
3. Für Explain prüft execution.rs:263-301 Budget, Principal-Egress und Evidenz erneut mit provider=true, bevor provider.answer aufgerufen wird.
4. rust/crates/brain-contracts/src/store.rs:209-237 verweigert versionierte Records mit origin.policy.provider_egress_allowed=false bei provider=true. dbrain-retrieval/src/release_port.rs:286ff,381ff prüft gepinnte Revision und aktuellen Kopf einschließlich Tombstones und aktuellen Rechten.
5. Der Twitch-Adapter behandelt UnauthorizedEvidence/Unavailable/ProviderError/BudgetExceeded als Backendfehler und InsufficientEvidence als NoEvidence. Das ist kein positiver Antwortbeleg.

publication_allowed wird in den geprüften Storepfaden mitgeführt und bei aktuellen Rechten verschärft, ist aber in record_allowed kein allgemeiner Chat-Sende-Guard. Deshalb nicht behaupten, dieses Flag allein verhindere jeden denkbaren öffentlichen Faktpfad. Für den konkreten aktuellen Explain-Pfad ist das sourcebezogene Egressverbot bereits eine wirksame Sperre. Ein providerfreier interner Fact-Test würde internes Retrieval belegen, aber weder die öffentliche Nutzung legitimieren noch den bestehenden Twitch-Explain-Consumer abnehmen.

## Konkreter positiver Consumerbeleg

Nach gesonderter Laufzuteilung und echter, rechtlich sowie technisch passender Quellenbindung muss eine interne Probe durch den tatsächlichen Twitch-Adapter Folgendes zeigen, ohne eine Twitch-Nachricht zu senden:

1. Tatsächlicher Consumercommit, laufendes Brain-Artefakt, lokale Zieladresse, bestehender Dienstgrant, öffentliche Scopes und reale Release-/Knowledgeversion sind gebunden. Kein Ersatzclient und kein alternatives Profil.
2. Eine konkrete Frage wird aus einer im echten Release vorhandenen und für diesen Zweck zulässigen Quelle beantwortet. Erwartetes Ergebnis ist Answered mit inhaltlich passender Antwort und überprüfbarer Quellen-/Revisionszuordnung, nicht Health200, bloße HTTP200, NoEvidence oder ein erfundener Stubtext.
3. Dieselbe öffentliche Identität erhält keine privaten Patchnotesevidenzen. Tombstone, Scopewiderruf und Egressverbot müssen auch gegenüber älteren gepinnten Releases wirksam sein. Ein negatives Ergebnis zählt als Sicherheitsbeleg, nicht als Ersatz für Punkt2.
4. Vorhandener Provider und dessen freigegebene Konfiguration/Budget bleiben erhalten. Kein neues Modell, kein Weg um Source-Egress und keine Änderung auf Fact, um die Prüfung scheinbar grün zu machen.
5. Aufruf mit Wegwerfconversation, ohne echte Chat-/Communitynachricht; Besitzbindung kann Datenbankwrites auslösen und ist daher keine reine Read-only-Probe. Logs, Exit, Antwortstatus und sichere Provenienz sichern, ohne Geheimnisse oder unzulässige Inhalte zu veröffentlichen.

## Konsequenz für den Abschluss

Interne B2-Übernahme mit unveränderten Rechtegrenzen und öffentlicher Consumererfolg sind zwei getrennte Nachweise. Die erste bleibt beauftragt. Für den zweiten fehlt in den geprüften Belegen eine tatsächlich nutzbare Quelle für den bestehenden Explain-Vertrag. Empfehlung: interne Snapshot-/Zustandsbindung und technische Beweise weiterführen; öffentliche Consumerabnahme offen lassen, bis ein vorhandener passender Quellenvertrag konkret belegt ist. Weder Scopeerweiterung noch Egressumschaltung aus dem Gesamtauftrag ableiten.

Während des vom Nutzer gemeldeten echten Twitch-Clip-Sprach-/Lernlaufs bleiben PG2, Last, Medien, Produktionszugriffe und nicht zugeteilte Compilerläufe zurückgehalten. Die isolierte Quellarbeit am lesenden Snapshotmodus läuft im bestehenden Autorthread weiter; sie löst die Beobachtungslücke, nicht die Publikationsfrage.

ORCHESTRIERUNG[OR-1]: Stufe groß | Schritt review | Artefakt: .tasks/2026-09-30-g5-abschluss/QUELLENNUTZUNG-CONSUMER.md
