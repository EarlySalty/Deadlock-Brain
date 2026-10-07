# YouTube-Spielwissen: Code, Datenvertrag und Prüfkette

Stand: 07.10.2026, gelesen bis 06:32 UTC. Nachtrag für Teil-Orchestrator B. Keine Korpusbewertung, keine Produktfreigabe.

## Ergebnis

Vorhandene YouTube-Aussagen lassen sich als Quellenwissen wiederverwenden. Der gelesene Code liefert jedoch keinen Beleg, dass ein als `accepted` gespeicherter Claim heute für eine bestimmte API-Version oder einen Patch gilt. Die bisherigen Claim-Leser hängen überwiegend an Namen. Eine strukturierte Bindung an Gs Entitätsart und positive API-ID fehlt in diesen Lese- und Schreibverträgen.

Für den späteren Anschluss braucht es eine quellengebundene Aussage an der aufgelösten Entität, mit eigenem Prüf- und Zeitbezug. Ein Text-Steckbrief muss dafür nicht vorab veröffentlicht werden. YouTube darf weder als Wiki umetikettiert werden noch aktuelle Zahlen aus E/G ersetzen. Ob konkrete öffentliche Aussagen diese Anforderungen schon erfüllen, beurteilt der getrennte Korpusworker.

## 1. Lesestände und Beweisgrenzen

| Stand | Gelesener Bezug | Einordnung |
| --- | --- | --- |
| Lokales `origin/main` | `9711cb630aebacfe959ed4783595b071f479be36` | Produktcode separat mit `git show <SHA>:<Pfad>` gelesen. Keine Ableitung aus dem schmutzigen Hauptbaum. |
| Hauptbaum-HEAD | `2734c2da4e814ff79953e8e825275b0216a6af16` | Alter Feature-Stand, zusätzlich uncommittete Änderungen. Nur Vergleichsquelle. |
| Q-Worktree | `/home/nathanael/.worktrees/brain-fertig-q`, HEAD `96bf05c4ac61e9282847dc211d417a26f4cd8b46` | `claims.rs` und `transcript_claims.rs` unterscheiden sich im Commitvergleich nicht vom oben genannten main-SHA. Die eingegrenzte Statusabfrage zeigte dort keine Änderungen dieser Dateien. Kein Q-Abschluss geprüft oder behauptet. |
| G-Worktree | `/home/nathanael/.worktrees/brain-g-v2-20261007`, HEAD `ce21a4576444c2afc9f2412f090857a43f9a0e2e` | Aktiver schmutziger Arbeitsstand. Nur gezielt Plan und Anschlussvertrag gelesen. Plan ist Sollvertrag, keine Fertigmeldung. |

Die SHAs von Hauptbaum und `origin/main` waren am Ende der Codeprüfung unverändert. Es wurde nicht gefetcht, da Gitrefs nicht verändert werden dürfen. `origin/main` bezeichnet deshalb den lokal vorliegenden Remote-Tracking-Stand, nicht einen in dieser Recherche neu abgefragten Serverstand. Keine Aussage über das laufende Binary.

Arbeitsbaumdateien sind nicht durch ihren HEAD-SHA beschrieben. Gelesene Fingerabdrücke:

- `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/claims.rs`: SHA256 `c40d14748769f19eb7da702a66c31d8a37fb91f0b76419afb9c0e44c4db54e21`.
- `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/model.rs`: unversioniert, SHA256 `da77d667b31d712ddd36185e8144e70904b732a01d6ecea70aec13e1f9a54ef6`.
- `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/PLAN.md`: SHA256 `4c021191fdd5404e98b482290b3b1c0a33ea6fb2ebb1c779004520f1acb66af2`.

Graphify wurde vor den Codesuchen befragt. Seine Treffer dienten als Wegweiser; Zeilen und Inhalt wurden am jeweiligen Commit nachgelesen. Keine Datenbankverbindung, Korpusabfrage, Secret-Lektüre, Modellanfrage, Builds oder Testläufe. Python-Code wurde ausschließlich als historische Vertragsreferenz gelesen. Die SQL-Felder unten sind aus Rust-Abfragen rekonstruiert; das ist keine Prüfung der tatsächlichen Live-DDL.

## 2. Tatsächliche Klassifikation und gespeicherte Metadaten

### Zwei verschiedene Claim-Verträge

Der Browser-/Modellparser akzeptiert genau sechs Zeichenketten: `build`, `item_timing`, `matchup`, `mechanic`, `combo`, `meta`. Das Rust-Modell besteht aus `entity`, `claim_type`, `assertion`, optionalem `patch_context` und `confidence`. `confidence` wird auf 0 bis 1 begrenzt. Eine Entitäts-ID, beteiligte Entitäten, strukturierte Bedingungen oder ein Gültigkeitsintervall gehören nicht zu diesem Modell.

Beleg: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/claims.rs:16-23,66-104` @ `9711cb63`.

Der separate Transkript-Claimimport ist offener. `claim_type` muss lediglich vorhanden und nicht leer sein. Er nimmt außerdem `entity_type`, `entity_name`, Aussage, Zitat, Videosekunde, Urteil, zwei Bewertungszahlen, `db_evidence`, `db_value` und `reasoning` an. Es gibt dort keine abgeschlossene Typenliste. Auch Entityname, Zitat und Datenbeleg dürfen fehlen. Die sechs Parserkategorien sind daher kein Beweis für sechs Kategorien im vorhandenen Korpus. Tatsächliche Korpustypen und ihre Häufigkeit bleiben beim anderen Worker.

Beleg: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/transcript_claims.rs:681-746,749-771` @ `9711cb63`.

`mechanic` und `combo` passen fachlich zum Nachtrag. Spielweise und Einsatzbedingung besitzen in diesen Verträgen kein eigenes geschlossenes Feld. Ein `build`- oder `item_timing`-Claim kann gleichzeitig Strategie, Zahlen und Bedingungen enthalten. Die Kategorie allein trennt qualitative Aussagen nicht von historischen Werten.

### Was die PostgreSQL-Schreibpfade tatsächlich ablegen

Beide Rust-Schreibpfade verwenden `brain.youtube_learning_claims` mit Claim-ID auf der Leseseite, `video_id`, `claim_hash`, `claim_index`, Entitytyp/-name, Claimtyp/-text, `evidence_quote`, `timestamp_seconds`, `model_confidence`, `verifier_confidence`, `status`, `model`, `prompt_version`, Prompt, Antwort, `provider_metadata`, `verifier`, `created_at` und `updated_at`.

- Der Browserpfad schreibt keinen Entitytyp, ein leeres Zitat, keine Videosekunde, `verifier_confidence=0`, `status=unverified` und `verifier=not_run`. Ein optionaler Patchtext steckt im JSON `verifier.patch_context`.
- Der Transkriptimport schreibt die mitgelieferten Daten und ein Verifier-JSON mit Urteil, Status, Bewertungszahl, `claude_db_crosscheck_v1`, `db_evidence`, `db_value` und Begründung. `provider_metadata` bleibt `{}`. `model_response_text` enthält hier die Begründung, nicht nachweislich eine vollständige ursprüngliche Modellantwort.

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/claims.rs:107-174` und `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/transcript_claims.rs:806-854` @ `9711cb63`.

Über `video_id` sind Titel, URL, Kanal und Veröffentlichungszeit anschließbar. Die Veröffentlichungszeit ist ein Quellenzeitpunkt; `timestamp_seconds` ist eine Stelle im Video. `created_at` und `updated_at` datieren Speicherung beziehungsweise Änderung. Keines dieser Felder ist schon eine fachliche letzte Prüfung oder eine belegte Patchgrenze.

Die Core-Modelle kennen durchaus Entitäts-ID, externes Primärkennzeichen und Aliase sowie Patchereignisse mit Snapshotbezug und Quelle. Diese Bausteine sind vorhanden, werden aber vom gelesenen Claimvertrag nicht als typisierte Bindung genutzt. Das Pg-Modul stellt Verbindungspools bereit, keinen fachlichen Claimprüfer.

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-core/src/models.rs:27-76` und `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-core/src/pg.rs:11-37,44-61` @ `9711cb63`.

## 3. Was die Prüfkennzeichen bedeuten

| Mitgeliefertes Urteil | Gespeicherter Status | Einordnung im bestehenden Antwortkontext |
| --- | --- | --- |
| `supported` | `accepted` | `creator_knowledge.verified` |
| `uncertain` | `needs_review` | `creator_knowledge.flagged` |
| `no_trusted_data` | `unverified` | `creator_knowledge.unverified` |
| `contradicted` | `rejected` | `creator_knowledge.refuted` |

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/transcript_claims.rs:764-771` und `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:5031-5067` @ `9711cb63`.

Der Transkriptimport führt den auf dem Etikett genannten DB-Abgleich nicht selbst aus. Er liest eine JSON-Datei, validiert Pflichtfelder und importiert das darin enthaltene Urteil. `db_evidence` und `db_value` können `null` sein; die vorhandene Prüfzahl wird nicht gegen einen zulässigen Bereich oder eine aktuelle Datenrevision geprüft. `supported` kann deshalb über diesen Import auch ohne Datenbeleg zu `accepted` werden. `PROMPT_TEXT` und `claude_db_crosscheck_v1` beschreiben den vorgesehenen beziehungsweise angegebenen Prüfweg, beweisen aber keine in diesem Lauf erfolgte Prüfung.

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/transcript_claims.rs:16-18,363-420,713-746,814-850` @ `9711cb63`.

Für ältere Bestände gibt es noch eine andere Bedeutung von `accepted`: Der lesbare Legacy-Verifier prüfte Textlänge, ein Zitat im Transkript, Modellbewertung, Entitätsauflösung und schwache Meinungsäußerungen. Er verglich keine Mechanik mit aktuellen Assets und band keinen Patch. Sein `entity_match` konnte eine interne `entities.id` im Verifier-JSON enthalten. Dieses verschachtelte Kennzeichen ist keine positive API-ID mit Entitätsart und kein relationaler Claim-Entity-Vertrag. Ob solche Datensätze tatsächlich im aktuellen Korpus liegen, wurde hier nicht abgefragt.

Beleg nur als historische Referenz: `/home/nathanael/repos/Deadlock-Brain/src/deadlock_brain/youtube_learning.py:1158-1208` @ `9711cb63`.

## 4. Entitätsbindung, Retrieval und Anreicherung

### Namen werden normalisiert, IDs gehen dabei nicht mit

`resolve_gaps` liest nicht aufgelöste Claims, bildet Kandidaten aus Namen, Slashlisten und Klammern und nimmt den ersten auflösbaren Kandidaten. Es schreibt `entity_type`, `entity_name` und das Verifier-JSON zurück. Es entfernt den Grund `entity_not_resolved` und markiert die erneute Namensauflösung. Es speichert dabei weder eine Entitäts-ID noch mehrere Teilnehmer einer Combo, deren Rollen oder einen versionsgebundenen Auflösungsbeleg.

Beleg: `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-normalize/src/resolve_gaps.rs:112-188,292-307,349-364` @ `9711cb63`.

### Drei Leser mit verschiedenen Filtern

| Leser | Auswahl | Was fehlt für eine aktuelle Entitätsansicht |
| --- | --- | --- |
| YT-CLI `query_claims` | Exakter Name ohne Groß-/Kleinschreibung, festes Paar `youtube_claims_de_v2` und `gemini-web`, neueste Veröffentlichungszeit zuerst | Kein Statusfilter, kein Patch-/Versionsfilter, kein Revalidierungsfilter im gelesenen main. Rückgabe enthält nur Aussage, Kategorie, Modellbewertung und Quellenangaben. Transkriptimport mit eigener Promptversion ist über diesen Leser nicht abgedeckt. |
| `ask-context` | Entitynamen, Text-/Zitat- und Keywordtreffer, Relevanzbewertung und Statusgruppen | Keine API-ID-Bindung, kein Claim-Patchintervall, keine Prüfung einer Quellenrevision. `accepted` wird direkt `verified`. |
| Reasoner `load_claims` | API-Hero-ID wird zuerst in Namen übersetzt; dann exakter oder Teilstringtreffer in YT-/Forum-Entitynamen, maximal 100 je Tabelle | Keine Claimstatus- oder Patchprüfung an dieser Ladestelle. Vollständige Zeile wird als JSON in den Meta-Index gegeben. |

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/claims.rs:178-223`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:4140-4177,4180-4288,5031-5067`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-reasoner/src/data.rs:1378-1401` @ `9711cb63`.

Der ausführliche `ask-context`-Claim behält Text, Zitat, Kategorie, Entityname, Status, Verifierbewertung, Datenbelege und Quellvideo-ID/-titel. Er verliert gegenüber der Tabelle insbesondere Claim-ID in der JSON-Ausgabe, Entitytyp, URL, Veröffentlichungszeit, Videosekunde, Erstellungs-/Änderungszeit und vollständige Prüfbegründung. Die Promptverdichtung reduziert weiter auf Claimtext, Statusgruppe und Videotitel; nur bei `flagged` oder `refuted` kommt ein gekürzter Datenbeleg mit. Das ist für eine nachprüfbare Steckbriefaussage unvollständig.

Beleg: `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:4439-4465,5249-5264,5450-5475` @ `9711cb63`.

Positiv: Der Antwortkontext trennt bestätigte, fragliche, widerlegte und ungeprüfte Aussagen. Widerlegte Aussagen erhalten eine eigene Gruppe und reservierten Platz. Ungeprüfte Aussagen können bei dünnem Bestand als ausdrücklich gekennzeichneter Notbehelf erscheinen, auch wenn `include_unverified=false` ist. Diese Sichtbarkeit ist kein Gültigkeitsbeleg und darf in einer Ansicht nicht still als Freigabe interpretiert werden.

Beleg: `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:5031-5067,5117-5151,5208-5235` @ `9711cb63`.

## 5. Zeitbezug, überholte Aussagen und Arbeitsstände

### Keine durchgängige Claim-Lebensdauer

In den gelesenen Rust-Claimpfaden gibt es die vier genannten Statuswerte, aber keinen eigenen Übergang für `stale` oder `superseded`, keinen Ersatzclaim-Verweis und kein belegtes Gültigkeitsintervall. Unbekannte Statuswerte landen im Antwortkontext in keiner der vier Gruppen. `superseded` in anderen Brain-Bereichen betrifft unter anderem Wartungsjobs und darf nicht als Claim-Lebensdauer ausgegeben werden.

Ein neuer Abruf oder eine neue Speicherung bestätigt die Aussage nicht. Auch ein frei formulierter `patch_context` wird weder zu einer Patch-ID aufgelöst noch mit Gs technischer Clientversion abgeglichen.

Die beiden Schreibpfade behandeln Wiederholungen unterschiedlich:

- `save_claims` bildet den Hash aus Video, Claimtext, leerem Zitat und Promptversion. Bei gleichem Hash überschreibt es unter anderem Status und Verifierdaten wieder mit dem aktuellen Extraktionszustand, also `unverified`/`not_run`. Es führt keine getrennte Prüfungshistorie.
- Der Transkriptimport hasht Video, Claimtext und Zitat. Er überspringt vorhandene Hashes und verwendet `ON CONFLICT DO NOTHING`. Eine neuere Bewertung derselben Aussage wird über diesen Importpfad nicht als Revalidierung gespeichert.

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/claims.rs:115-169`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/transcript_claims.rs:398-405,788-803,825-832` @ `9711cb63`.

### Der alte Hauptbaum enthält einen Guard, main nicht

Am alten Hauptbaum-HEAD `2734c2da` schloss `query_claims` Videos mit `metadata.needs_claim_revalidation=true` aus. Dieser Filter ist auch im gelesenen uncommitteten `claims.rs` enthalten, dort Zeile 223. Am main-SHA `9711cb63` und Q-SHA `96bf05c4` fehlt er in dieser Funktion. Der Commitvergleich zeigt außerdem den Wegfall des dazugehörigen Vertragstests. Der Graph nennt den alten Test noch; daraus folgt keine heutige Schutzwirkung.

Im schmutzigen Hauptbaum erweitert WIP den Leser außerdem auf `youtube_claims_de_v3`/`fireworks-rust-transcript` plus das ältere Paar. `model.rs` enthält einen Transkript-Extraktor mit JSON-Vorgabe und Fehlerklassen. Er ist keine fachliche Patchprüfung und kein Beleg für eine laufende Pipeline. Keine der Änderungen wurde gebaut, getestet oder als aktiv eingeordnet.

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/claims.rs:203-245` im Arbeitsbaum mit dem oben genannten Dateihash; Vergleich `2734c2da..9711cb63` derselben Datei; `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/model.rs:12-18,52-111` als unversionierter WIP.

## 6. Präzise Anschlusslücken

1. **Prüfstatus beweist keine heutige Wahrheit.** Der Transkriptimport übernimmt Urteile ohne verpflichtenden Datenbeleg. Der Legacy-Verifier prüfte Transkriptbeleg und Namensauflösung. Beide Bedeutungen können hinter `accepted` stehen. `ask-context` bezeichnet diese Gruppe dennoch pauschal als gegen Spieldaten geprüft. Zwilling geprüft: Browserpfad schreibt ausdrücklich `unverified`, CLI- und Reasonerleser prüfen den Status dagegen überhaupt nicht. Belege in Abschnitten 3 und 4 sowie `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:72-90,5040-5045` @ `9711cb63`.

2. **Kanonische Entitätsbindung fehlt im Claimvertrag.** Altnamen, interne Entity-IDs in beliebigem Verifier-JSON und API-IDs sind unterschiedliche Dinge. Die erste auflösbare Komponente einer Slashliste reicht für eine Combo nicht. Zwilling geprüft: Import, Normalisierung, CLI, Antwortkontext und Reasoner bleiben an Namen beziehungsweise untypisiertem JSON. Belege in Abschnitten 2 und 4.

3. **Patchgültigkeit und Quellenänderung sind nicht durchgängig abgesichert.** Weder die Namens-/Keywordabfragen noch der Reasonerclaimleser binden einen Patch oder eine technische Datenversion. Der Revalidierungsfilter ist nur im alten Hauptbaum belegt, nicht in der gelesenen main-Funktion. Beleg in Abschnitt 5; Zwilling geprüft: beide `ask-context`-SQL-Pfade und `load_claims`.

4. **Erneute Prüfung hat keinen einheitlichen Speicherweg.** Ein Pfad überschreibt Verifierdaten, der andere überspringt vorhandene Hashes. Beides ersetzt keine revisionsgebundene Prüfhistorie und keinen nachvollziehbaren Übergang zu überholt oder ersetzt. Belege in Abschnitt 5; beide Schreibpfade geprüft.

5. **Bedingungen und Prüfbarkeit gehen auf dem Lesepfad verloren.** Es fehlen geschlossene Bedingungen, Rollen bei Mehrfachentitäten und Qualitativ-/Zahlentrennung. Vorhandene Zitat-/Zeit-/Herkunftsfelder werden im Antwortkontext teilweise entfernt. Zwilling geprüft: ausführliche Claim-JSON-Ausgabe, Promptverdichtung und CLI-Rückgabe. Belege in Abschnitten 2 und 4.

6. **Der bestehende Prompt ist für die neue Zahlenregel zu weit.** Er erlaubt konkrete Zahlen aus strukturierten Spieldaten „oder verified“. Gleichzeitig entsteht `verified` nur aus dem gespeicherten Status. Für den Nachtrag darf dieser Weg alte Creator-Zahlen nicht als aktuelle Zahlen legitimieren. Das Promptverbot für ungeprüfte Zahlen ist sinnvoll, ersetzt aber keine deterministische Trennung vor der Ansicht oder Berechnung. Beleg: `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:72-90,5031-5045` @ `9711cb63`. Zwilling geprüft: Importurteil und Browserstatus.

7. **Der vorhandene Metapfad kann untypisierte IDs als Itembezug zählen.** `load_claims` gibt vollständige Claimzeilen an `build_meta_index`. Dessen rekursiver `item_ids`-Sammler behandelt auch jedes Feld namens `id` und sogar `ability_id` als möglichen Itembezug. Somit kann eine Claimzeilen-ID oder Verifier-Entity-ID bei numerischer Kollision als `claim_hit` für ein Item zählen. `claim_hits` enthält keinen Statusfilter; der Itemscore gewichtet den Treffer mit 0,02. Das ist eine Codeeigenschaft, kein in dieser Recherche beobachteter Produktionsfall. Dieser Zählweg taugt nicht als Gültigkeits- oder Zuordnungsbeleg für die neue Ansicht. Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-reasoner/src/data.rs:1385-1398`, `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-reasoner/src/lib.rs:379-380,730-731`, `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-reasoner/src/meta.rs:536-563`, `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-reasoner/src/item.rs:344-349` @ `9711cb63`. Zwillingssuche: beide Reasoneraufrufer und sämtliche `claim_hits`-Fundstellen gelesen.

8. **Der aktuelle Profilquellenvertrag hat noch keinen YT-Anschluss.** `ProfileSourceKind` kennt nur `GameFile` und `Wiki`. `EntityDocumentContext` und `project_knowledge` akzeptieren diese beiden Quellenarten; der Kontext projiziert Patchgültigkeit ausdrücklich als `Unknown`. YT-Claims in dieses Schema unter `wiki` einzufügen wäre eine falsche Herkunftsbehauptung. Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-contracts/src/entity_profile.rs:15-34,46-67`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/brain-storage/src/entity_profile.rs:43-64,118-142,150-152`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/knowledge_projection.rs:78-119` @ `9711cb63`. Beide Quellenartprüfungen als Zwilling geprüft.

## 7. Technische Mindestanforderung für Bs Empfehlung

Gs Plan verlangt `EntityRef` als Art plus positive API-ID. `entity_profile` ist eine Ansicht aus Originalfakten und gemeinsamer Projektion; ein veröffentlichter Text-Steckbrief ist keine Voraussetzung. Dieser Vertrag bleibt maßgeblich.

Beleg: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/PLAN.md:62,151-159`, gelesener Arbeitsstand mit HEAD `ce21a457` und oben genanntem Dateihash.

Für eine ergänzende qualitative Aussage wären mindestens erforderlich:

- stabile Claimkennung und eigene, korrekt bezeichnete YT-Herkunft;
- aufgelöste Entitätsart/API-ID je beteiligter Entität, gegebenenfalls Rollen und Mehrdeutigkeitsgrund;
- Aussageart, separat erhaltene Einsatzbedingungen und erkennbare historische Zahlenteile;
- Video-/Quellenrevision, Veröffentlichungszeit und genaue Belegstelle, soweit vorhanden;
- ursprüngliches Prüfurteil getrennt von aktueller Verwendbarkeit, fachliche letzte Prüfung mit Daten-/Regelrevision und begründetem Ergebnis;
- belegte Patchbindung oder ausdrücklich unbekannte Gültigkeit; auch qualitative Mechanik und Spielweise werden nicht automatisch dauerhaft gültig;
- Konflikt-, Überholungs- und Ersatzbezug, ohne abgelehnte Aussagen zu löschen oder als gültige Fakten zurückzuholen.

Aktuelle Zahlen bleiben bei Es versionsgebundenen Daten und Gs Berechnung. Quellenpopularität, Modellbewertung, Videotitel, Abrufdatum und Namensauflösung ersetzen keinen mechanischen Gegenbeleg. Wo API/Mechanikdaten eine qualitative Aussage nicht entscheiden können und eine belastbare Patchhistorie fehlt, bleibt die aktuelle Gültigkeit offen. Eine Modellzustimmung schließt diese Lücke nicht.

Die frühere Wiki-Spec beschreibt aktuelle Snapshotseiten, externe IDs, Quellenhash und zusätzliche `ground_truth.game_knowledge`. Sie liefert weder einen YT-Claimvertrag noch eine Erlaubnis für eine zweite Profilwahrheit. Sie ist hier nur Herkunfts- und Kontextreferenz.

Beleg: `/home/nathanael/repos/Deadlock-Brain/docs/superpowers/specs/2026-07-21-llm-wiki-game-knowledge-design.md:15-38` @ `9711cb63`.

Keine neue Pipeline, kein Q-Writerabschluss und keine zweite Profilpublikation werden vorgeschlagen. B bewertet den kleinsten nutzbaren Anschluss nach den echten Korpusproben; G entscheidet den Entitäts- und Ansichtsvertrag.

## 8. Prüfnachweise richtig einordnen

Vorhandene Parser-, Statusgruppen-, Import- und Metahit-Tests wurden als Verträge im Code gelesen. Beispielsweise prüft der Importtest die vier Statusgruppen und verwendet auch `macro`; das belegt die offene Importkategorie, kein echtes öffentliches Spielwissen. Konkrete Spielsätze in Fixtures sind keine Korpusproben und werden hier nicht als solche übernommen.

Belege: `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/transcript_claims.rs:1044-1049,1108-1116`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-retrieval/src/lib.rs:8265-8312,8442-8465`; `/home/nathanael/repos/Deadlock-Brain/rust/crates/dbrain-reasoner/src/meta.rs:849-872` @ `9711cb63`.

Kein Test wurde ausgeführt. Ein vorhandener Testname, ein gespeichertes Prüfetiquett oder ein älterer grüner Bericht beweist weder den heutigen Korpusstand noch die aktuelle Patchgültigkeit. Öffentliche Originalaussagen, Mengen und Quellenqualität sind bewusst nicht Bestandteil dieses Pakets.

WIRKUNGSPRUEFUNG[WP-1]: 8 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

Die letzte Zahl bezieht sich ausschließlich auf externe Laufzeitproben dieses Pakets: keine vorgesehen, keine ausgeführt. Providertransport, laufende Dienste und Live-Daten wurden nicht geprüft; die Zeile ist keine Freigabe dieser Bereiche.
