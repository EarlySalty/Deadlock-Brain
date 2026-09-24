# S08: erster E2E-Prüfplan

Status: `designed`, Ergebnis: `not_run`, Start der ausführbaren Umsetzung: nach integriertem G1 und S08-Pfadfreigabe. Dies ist ein lesbares Szenario, kein Ersatz-Wireformat, keine Mockimplementation und kein erfolgter API-Test.

Basis: `c00fc8935048bf490c1e4790f7c6195864ad49e2`. Contract, Policy, Schema und Knowledgeversion noch offen. Sämtliche Identitäten und Fakten unten sind ausdrücklich synthetische Testbezeichnungen, keine realen Spieler, Heroes, Accounts oder Spielwerte.

## Festes Szenario nach Übernahme der G1-Typen

Eine feste Testuhr, eine nachvollziehbare Trace-ID und zählbare Testports machen den Ablauf prüfbar. Die Versionsbezeichnungen `K_A`, `P_A`, `D_A` und `R_A` stehen nur für Testidentitäten. Sie werden in die durch 02 freigegebenen echten Typen übersetzt; S08 definiert weder ihre Serialisierung noch eigene Rust-Verträge.

| Bestandteil | Semantischer Fixtureinhalt |
| --- | --- |
| Principal | `principal_public_a`, ausschließlich öffentliche Leseberechtigung, kein Debugrecht |
| Zweiter Principal | `principal_internal_b`, eigener Tenant-/Conversationkontext und ausdrücklich freigegebene interne Objektberechtigung |
| Policy | `P_A`, serverseitig erstellt; externe Übermittlung für öffentliche Fixture erlaubt, interne Fixture nicht für den Antwortprovider freigegeben |
| Release | `current` zeigt beim ersten Auflösen auf das freigegebene `K_A`; zugehöriger synthetischer Patch und Mode sind bestimmt |
| Frage | „Wie lang ist die Abklingzeit der Testfähigkeit?“ |
| Exakter Fact | Stabile Fact-ID `fact_test_cooldown`; Dezimalwert `12`, Einheit Sekunden, Dataset `D_A`, Quelle `source_test_public` mit Revision `R_A`, gültig im Patch/Mode von `K_A` |
| Öffentliche Passage | `Testfähigkeit hat eine Abklingzeit von 12 Sekunden.` mit Document-/Chunkrevision und Locator auf genau `12 Sekunden` |
| Interne Passage | Eigenes synthetisches Dokument mit dem Marker `INTERNAL_TEST_MARKER`; getrennte ACL und kein Provider-Egress |
| Aktuelles Sperrregister | Anfangs leer, später Quelle/Conversation explizit entziehbar; vom historischen Knowledgezeiger unabhängig |
| Conversation | Zwei getrennte Conversations und zwei Revisionen desselben autorisierten Gesprächs als Cache-/Singleflight-Gegenprobe |
| Modell | Nur der freigegebene Testport; keine echten Credentials, keine Netzaufrufe; zählbare Aufrufe und Usage |

Locatorpositionen erst gemäß dem G1-Vertrag ableiten. Für einen Bytevertrag die UTF-8-Bytes verwenden und einen Zusatzfall mit Umlauten vor dem Zitat ausführen; niemals Zeichenoffsets still als Bytes interpretieren. Nur die öffentliche synthetische Quelle darf in der öffentlichen Ausgabe erscheinen. Das interne Fixture bleibt vollständig erfunden und enthält keine Produktionsdaten.

## Durchstich A: deterministischer Faktenpfad

Credentials des öffentlichen Testprincipals am API-Testtransport prüfen lassen. Eine zusätzliche reine Kernprüfung darf einen von der Test-Policy erzeugten festen AuthorizedContext verwenden; der HTTP-Test darf ihn nicht aus ungeprüftem Request-JSON übernehmen.

Der Request führt das freigegebene Antwortprofil sowie die synthetische Frage. `current` wird einmal aufgelöst. Store-/Domainport liefert den eindeutigen Fact mit vollständiger Provenienz. Vor Rückgabe werden aktuelle Rechte, Releasezuordnung, Einheit, Quelle und Profil geprüft.

Erwartung: fachlich beantwortet, Wert `12 Sekunden`, zugängliches Fact-Zitat und aufgelöste Dataset-/Patch-/Mode-/Knowledgeversion. Identische Trace-ID innerhalb dieses Requests. **Null Antwortmodell-, Embedding- und Jev-Aufrufe** für diesen eindeutigen Testfall; kein bloßes „Mock war schnell“ als Nachweis eines modellfreien Pfads. Ein Folgerequest kann nach erfolgreicher Speicherung ein Cachehit sein, muss aber erneut autorisiert werden.

Rote Gegenprobe nach Implementation: dem Fact die erforderliche Einheit oder erlaubte Quellrevision entziehen. Der positive Test darf nicht mehr bestehen; keine Zahl ohne Provenienz als Erfolg ausgeben. Die Mutation wird nur im isolierten Testfall verwendet, niemals eingecheckt oder in Produktion angewendet.

## Durchstich B: belegte Prosa

Dieselbe öffentliche Berechtigung, diesmal kein passender Exact-Fact-Treffer. Der Retrievaltestport liefert ein Evidence Pack aus öffentlicher Passage und Fact mit gemeinsamem Release. Die interne Passage darf nicht als Kandidat oder Modelleingabe auftauchen.

Der Modelltestport gibt eine passende Erklärung mit genau der gelieferten Evidence-ID und Revision zurück. Prüfen: Autorisierung vor Retrieval und Modell, genau ein freigegebener Modellaufruf, erlaubte Eingaben, zählbare Usage, Referenzprüfung und gesonderte Supportprüfung, anschließend Profil- und Rechtekontrolle.

Erwartung: beantwortet mit belegtem Inhalt. Die Spur weist beide Validierungsschritte getrennt aus. Ein synthetischer Supporttest beweist nur die getestete Regel, keine allgemeine Qualität eines echten Modells.

Rote Gegenproben: eine nicht vorhandene Citation-ID, eine alte Revision, ein Locator außerhalb der Passage und eine semantisch widersprechende Aussage mit ansonsten gültiger ID. Jeder Fall muss den Erfolg verhindern. Eine strukturvalide Citation darf die Aussage „24 Sekunden“ nicht für den Fact `12 Sekunden` rechtfertigen.

## Durchstich C: Rechteänderung während laufender Arbeit

Die Prosaantwort mittels deterministischer Barriere unmittelbar vor der finalen Ausgabe anhalten. Quelle oder Conversation unter der aktuellen Policy entziehen; danach weiterlaufen lassen. Erwartung: keine geschützten Antwortbytes, Zitate, Debugdaten oder Cacheausgabe. Auch ein zuvor gepinnter historischer Release darf das aktuelle Verbot nicht zurücknehmen.

Dann einen warmen Cache mit unverändertem Fragetext und geändertem Principal, Tenant, ACL-Fingerprint, Egress, Profil, Conversationrevision oder Release abfragen. Jeweils einen Parameter gezielt verändern. Unzulässige Varianten dürfen weder Cachehit noch gemeinsame laufende Arbeit des ursprünglichen Requests verwenden.

Singleflight positiv mit mehreren tatsächlich identischen berechtigten Anfragen prüfen: eine gemeinsame teure Arbeit, aber getrennte Trace-IDs und abschließende Ausgabekontrollen. Abbruch eines Wartenden beendet nur dessen Antwort; Entzug der Berechtigung eines Wartenden darf nicht durch die Rechte eines anderen kompensiert werden. Abbruch aller Wartenden löst den vereinbarten best-effort Abbruch aus und hält keine unbegrenzten Jobs offen.

## Durchstich D: Versions- und Fehlerpfade

Nach erster Auflösung von `current` den aktiven Zeiger kontrolliert auf ein anderes freigegebenes Testrelease setzen. Alle Subschritte des bereits laufenden Requests bleiben auf `K_A`; der nächste Request sieht das neue Release. Eine gemischte Fact-/Index-/Rulegeneration ist ein Fehler, kein erfolgreicher aktueller Wissensstand.

Unzureichende Evidenz, veraltete Quelle, widersprüchliche Pflichtfakten, unbekannter Hero, fehlende Buildregel und nicht unterstützte Replayfelder getrennt einspeisen. Nullwerte dürfen fehlende Werte nicht ersetzen. Ein legaler Build bleibt ein Ergebnis von 05; ein Modell darf keine zusätzliche Rule erfinden.

Jev-Ausfall nutzt nur die freigegebene sichere Baseline. Provider-Ausfall, Budgetende und Gesamtdeadline dürfen keine unvalidierte Teilantwort freigeben. Höchstens zwei Retrievalpässe insgesamt; alle Retries und beide Pässe verbrauchen dasselbe Budget. Wartezeit und Finalvalidierung zählen zur Deadline. Vor Ablaufprüfung eines neuen Modellaufrufs muss das Restbudget reservierbar sein.

## Isolation und echte Integration

Nach G1: Tests mit synthetischen Daten, eigener temporärer Datenbank bzw. isoliertem freigegebenem Schema, festen Ports und ausgeschalteten externen Zugängen. Keine Infisical- oder Produktionscredentials laden. Keine echten Builds publizieren, Bots anschreiben, Dienste neu starten oder Quellen spiegeln.

Nach G2-Voraussetzungen: Testports schrittweise durch freigegebene 03-/04-/05-/06-/07-Module ersetzen. Je eine öffentliche und interne Quelle, ein Wiki-Hero mit Abilities/Items/Mechaniken, historischer Feed, Replayfall, Adapter sowie Delete-/ACL-/Konfliktfall müssen real zusammenpassen. Erlaubte Quellen und Parserfähigkeiten nicht durch synthetische Erfindungen ersetzen. Modell-/Provider-Vertragsprüfung und Egressfreigabe getrennt belegen.

Legacyadressen in der **isolierten Testumgebung** sperren und alle ausgehenden Verbindungen protokollfrei hinsichtlich sensibler Inhalte zählen. Zuerst eine absichtliche Probe auf eine gesperrte Adresse ausführen: sie muss tatsächlich scheitern. Anschließend E2E mit ausschließlich erlaubten Rustdiensten durchführen. Ein konfiguriertes, aber nicht wirksames Verbot ist kein Legacyfreiheitsnachweis. Keine Hostfirewall oder laufende Produktionsdienste verändern.

## Zu speichernder Testnachweis

Pro ausführbarem Test: echter Contract-/Schema-/Datenstand, getesteter Commit, Umgebung, Befehl, Testname, Ergebnis, Anzahl und Skips sowie Seed/Testuhr. Für die ersten positiven Fälle die tatsächlich ausgeführte rote Gegenprobe samt Fehlergrund und anschließendem grünen Ergebnis angeben. Synthetische Traces enthalten nur Fixture-IDs, Versionsreferenzen, Stufenstatus, Lauf-/Wartezeiten und freigegebene Usage.

Geplante Commands nach Pfadfreigabe aus dem echten Workspace ableiten. Keine nicht existierende Crate und keinen heute nicht vorhandenen E2E-Test als ausführbar dokumentieren. Die aktuellen Einträge in [TESTFAELLE.csv](TESTFAELLE.csv) bleiben bis dahin `not_run` und `blocked_G1`. [Übergabe](../handoffs/08-answer-kernel-api.md).
