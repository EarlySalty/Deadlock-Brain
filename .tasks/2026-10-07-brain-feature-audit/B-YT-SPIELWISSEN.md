# Audit B: vorhandenes YouTube-Spielwissen an Entitäten anschließen

Stand: 07.10.2026. Recherchebericht zum bestätigten Nachtrag, kein Bauauftrag und keine Releasefreigabe. Maßgeblich sind `AUFTRAG.md` und `NACHTRAG-YT.md`. Die pauschale YouTube-Ausnahme in der historischen Fassung von `B-FEATURE-BESTAND.md` ist damit ersetzt. Grafik-, Guide- und Patenbefunde wurden nicht erneut recherchiert.

## Urteil

Der Code enthält Speicher-, Klassifikations- und Prüfverträge für Aussagen. Der durch den Hauptorchestrator benannte lokale Peer-Zugang funktioniert secretsfrei in lesenden Transaktionen. B bestätigte: **Beide geprüften Claimsablagen enthalten aktuell 0 Zeilen, beide Videoablagen jeweils 195.** Die 51 `insight_records` haben in den geprüften strukturierten Quellen Patchherkunft, keine YT-Herkunft. Ein früherer öffentlicher YT-Klassifikationsexport wurde an den dokumentierten/geprüften Orten nicht lokalisiert.

Fünf archivierte öffentliche Videoquellen belegen reale Mechanik-, Combo- und Spielweiseaussagen, jedoch keine gespeicherten Klassifikationen. Keine Probe ist als heute gültig verifiziert. Der frühere Datenzugangsblocker ist aufgelöst; die verbleibende Grenze betrifft den fehlenden nachgewiesenen historischen Klassifikationsbestand, nicht den Zugang zur genannten DB.

Ein Anschluss an den bestehenden G-Entitätsabruf ist fachlich sinnvoll. Dafür fehlen ein durchgehender kanonischer ID-Bezug, ein belegter aktueller Prüfstand und ein typisierter Wissensabschnitt. `accepted` oder daraus abgeleitetes `verified` genügt nicht als heutiger Gültigkeitsbeweis. Der Steckbrief bleibt eine Ansicht der Entitätsdaten; es entsteht weder eine zweite Profiltextwahrheit noch eine neue YouTube-Pipeline.

## 1. Belegter Bestand und Reichweite

| Gegenstand | Nachgewiesen | Nicht nachgewiesen |
| --- | --- | --- |
| Speicherung | Dedizierte DB: `brain_legacy.youtube_learning_claims` 0, Videos 195; zentrale DB: `brain.youtube_learning_claims` 0, Videos 195, Attempts 0; von B in lesenden Transaktionen bestätigt | Früherer Bestand außerhalb dieser Tabellen, konkreter historischer Claimwortlaut und Claim-ID |
| Klassifikation | Browserparser mit sechs Typen; Transkriptimport mit freiem nicht leerem Typ; 51 patchbezogene `insight_records` | Tatsächliche gespeicherte YT-Klassifikation der fünf öffentlichen Proben |
| Quellen | Vorhandene öffentliche Feedarchive, VTT-Dateien, Zeitspannen und Datei-Hashes | Heutige Videoerreichbarkeit, Verknüpfung dieser Datei-Revision mit einem gespeicherten Claim |
| Entitäten | Namen, historische Archivkennungen, Aliasse und vorhandene Resolver | Aktuelle kanonische API-IDs und vollständige Rollenbindung jeder Aussage |
| Prüfung | Status-/Verifierfelder und frühere Prüflogik im Code | Letzte fachliche Prüfung gegen den heutigen E/G-Stand; heutige Gültigkeit einer realen Aussage |

Die fünf Quellen sind eine gezielte Stichprobe aus zwei öffentlichen Kanälen. Sie repräsentieren weder den vollständigen Korpus noch dessen Klassifikationsverteilung. Die fachliche Einordnung unten stammt aus den gelesenen Passagen und ist ausdrücklich keine gemessene DB-Klassifikation.

### Quellenstände

- Brain-main: `9711cb630aebacfe959ed4783595b071f479be36`, erneut rein lesend per `git ls-remote` bestätigt.
- Alter Hauptbaum: `feat/brain-rust-cutover-20260919`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`, mit fremdem WIP. Kein aktueller Main- oder Livebeweis.
- Q-Worktree: `/home/nathanael/.worktrees/brain-fertig-q`, HEAD `96bf05c4ac61e9282847dc211d417a26f4cd8b46`. Die geprüften Claimdateien unterscheiden sich im Commitvergleich nicht vom genannten main. Kein Q-Abschluss vorgenommen.
- G-Worktree: `/home/nathanael/.worktrees/brain-g-v2-20261007`, geprüfter HEAD `ce21a4576444c2afc9f2412f090857a43f9a0e2e`. Aktive Arbeit, keine behauptete Livefunktion.

Graphify wurde vor der Codebestandssuche verwendet. Aktuelle Fundstellen wurden im Code nachgelesen. Die statischen Befunde sind keine beobachteten Produktionsfälle.

## 2. Vorhandene Aussageverträge und Prüfkennzeichen

### Zwei unterschiedliche Klassifikationsverträge

Der Browser-/Modellparser kennt genau `build`, `item_timing`, `matchup`, `mechanic`, `combo` und `meta`. Er liest `entity`, `claim_type`, `assertion`, optional `patch_context` und `confidence`; die Bewertung wird auf 0 bis 1 begrenzt. Eigene Typen für Spielweise oder Bedingungen fehlen. Diese Inhalte können im Aussagewortlaut stecken, ohne strukturiert erfasst zu sein. Der Vertrag enthält keine kanonische API-ID, keine Liste aller Combobeteiligten mit Rollen und keine typisierten Einsatzbedingungen oder Gültigkeitsintervalle.

Beleg: `rust/crates/deadlock-brain-yt/src/claims.rs:16-23,66-104` @ main.

Der Transkriptimport fordert dagegen nur einen vorhandenen, nicht leeren `claim_type`. Eine im Parser bekannte Kategorienliste beweist daher keine einheitliche Klassifikation des Gesamtbestands. Entityname, Zitat und Datenbeleg können fehlen.

Beleg: `rust/crates/deadlock-brain-yt/src/transcript_claims.rs:681-771` @ main.

### Speicherung und Bedeutung der Felder

Der Speichervertrag enthält unter anderem Claim-ID auf der Leseseite, `video_id`, `claim_hash`, `claim_index`, `entity_type`, `entity_name`, `claim_type`, `claim_text`, `evidence_quote`, `timestamp_seconds`, Modell-/Verifierbewertungen, Status, Modell, Promptversion, Prompt/Antwort, Provider-/Verifiermetadaten sowie `created_at` und `updated_at`. Zum Quellvideo gehören Titel, URL, Kanal und `published_at`.

Belege: `claims.rs:107-174,178-223`; `transcript_claims.rs:806-854` @ main. Details stehen in `b/YT-CODE-PRUEFUNG.md`.

Diese Zeiten haben verschiedene Aufgaben: Veröffentlichung des Videos, Position im Video und Speicherung des Datensatzes. Sie belegen weder die genaue Aussageentstehung noch die letzte fachliche Prüfung. Ein Feed-`updated` ist ebenfalls kein Prüfdatum. Unbekannte Zeiten müssen unbekannt bleiben.

### Was die bisherigen Urteile tragen

| Vorhandener Weg | Kennzeichen | Tragfähige Aussage |
| --- | --- | --- |
| Browserimport | `unverified`, `not_run`, leeres Zitat, keine Videosekunde, Verifierbewertung 0 | Importiert, noch nicht fachlich geprüft |
| Transkriptimport | `supported` wird `accepted`; `uncertain` wird `needs_review`; `no_trusted_data` wird `unverified`; `contradicted` wird `rejected` | Übernommenes Urteil; der Import führt den angekündigten DB-Abgleich nicht selbst aus |
| Legacy-Verifier | Textlänge, Zitat im Transkript, Modellbewertung, Namensauflösung, schwache Meinungsäußerungen | Frühere Plausibilitäts-/Quellenprüfung, keine belegte aktuelle Assets-/Mechanik-/Patchprüfung |
| Retrieval | `accepted` wird `creator_knowledge.verified` | Abgeleitetes Speicherkennzeichen, kein zusätzlicher aktueller Prüfbeweis |

Beim Transkriptimport können `db_evidence` und `db_value` null sein; `supported` kann ohne verpflichtenden aktuellen Datenbeleg zu `accepted` werden. `claude_db_crosscheck_v1` und Prompttext sind Kennzeichen, kein Beweis eines tatsächlich ausgeführten Abgleichs. Gleiche Hashes werden übersprungen beziehungsweise mit `ON CONFLICT DO NOTHING` behandelt. Das speichert kein neueres Urteil derselben Aussage als durchgehende Revalidierungshistorie.

Belege: `transcript_claims.rs:363-420,713-771,788-854`; Legacy ausschließlich lesend `src/deadlock_brain/youtube_learning.py:1158-1208`; `rust/crates/dbrain-retrieval/src/lib.rs:5031-5067` @ main. Python wird nicht produktiv erweitert oder korrigiert.

## 3. Fünf echte öffentliche Quellproben

Alle Proben stammen aus vorhandenen Dateien unter `data/raw/youtube/`. Veröffentlichungsdaten und URLs stammen aus öffentlichen Feedarchiven. Quellenpfade, vollständige SHA-256 und historische Entitätskandidaten stehen in `b/YT-KORPUSBEISPIELE.md`. B hat die fünf VTT-Hashes und jeweils eine zentrale Textstelle unabhängig nachgeprüft; die Hashes stimmen mit dem Workerbericht überein.

**Für jede Probe fehlen gespeicherte Claim-ID, Claim-Hash, DB-Klassifikation, kanonischer Join und aktueller Prüfbeleg.** Historische Archivkennungen sind keine bestätigten E/G-API-IDs.

| Öffentliche Quelle | Passage und tatsächlicher Inhalt | Bedingungen, Zahlen und Grenze |
| --- | --- | --- |
| Deadlock Mythbusters, [J2_mZjoqv7I](https://www.youtube.com/shorts/J2_mZjoqv7I), 19.05.2026, „Maximize your combo damage after a parry!“ | `00:00:09.440` bis `00:00:17.840`: Flog zwischen normalen Schüssen; anspruchsvollere Reihenfolge mit Schüssen vor dem ersten schweren Nahkampfangriff | Gilt laut Quelle für eine Parade-Combo. Originalzahlen `354`, `336`, `468` sind historische Quellwerte. Den Wechsel 354 zu 336 nicht still korrigieren. „Max damage“ ist ohne heutigen Vergleich unbelegt. Kandidaten Lash/Flog sind auffindbar, aktueller Join und Zeitfensterprüfung fehlen. |
| Deadlock Mythbusters, [Z7N885Cz1u4](https://www.youtube.com/shorts/Z7N885Cz1u4), 20.05.2026, „Level up your Yamato combos!“ | `00:00:09.519` bis `00:00:14.559`: Crimson Slash zwischen normalen Schüssen; weiterer Schuss vor dem ersten schweren Nahkampfangriff | Parade-Combo, ursprünglicher Wert `386` Schaden. Caption wechselt zwischen „pillars“ und „fillers“. Yamato/Crimson Slash sind historische Zuordnungskandidaten; Fähigkeit, Nahkampf und Paradefenster können auch die qualitative Reihenfolge veralten lassen. |
| Deadlock Mythbusters, [ORSIFmV5dIo](https://www.youtube.com/shorts/ORSIFmV5dIo), 11.05.2026, „Can you stack Unstoppable with Indomitable?“ | `00:00:05.080` bis `00:00:15.440`: Indomitable löse trotz aktivem Unstoppable in den meisten Fällen aus; bestimmte Fähigkeitswechselwirkungen verhinderten den Cooldown | Kein Zahlenwert in dieser Passage. „In most cases“ und Ausnahmen müssen erhalten bleiben. Die Passage trägt keine Aussage „immer“ und keine vollständige Ausnahmenliste. Aktuelle Triggerregeln nicht verifiziert. |
| Deadlock Mythbusters, [BLf3gn-mzXU](https://www.youtube.com/shorts/BLf3gn-mzXU), 26.05.2026, „The doorway has a few hidden mechanics!“ | `00:00:08.559` bis `00:00:17.349` und Folgezeile: Nähe zur Tür, unsichtbare Plattform und eine Anwendung durch Doorway mit Paige | Räumliche Bedingung unverzichtbar; kein aktueller Distanzwert. Die konkrete Paige-Fähigkeit wird nicht benannt. „This“ und die Plattform hängen von der damaligen visuellen Darstellung ab. Kein zuverlässiger vollständiger Fähigkeitsjoin aus Text allein; keine Videoanalyse nachgeholt. |
| Natan, [wihHFTapEGk](https://www.youtube.com/watch?v=wihHFTapEGk), 15.02.2026, „EVERY Deadlock Hero Explained in 30 SECONDS! Playstyles & Builds & Difficulty“ | `00:06:23.880` bis `00:06:44.800`: Lash für Roaming/Ganks, Slam von erhöhten Positionen, Grapple zur Neuplatzierung statt dauerhaftem Frontline-Kampf | Damalige Spielweiseempfehlung, begründet mit Höhenmechanik und relativen Gesundheits-/Cooldownwerten. Die unmittelbar vorherige Aussage „10 permanent spirit power on kills“ gehört vor den Lash-Abschnitt und darf nicht durch Textnähe Lash zugeordnet werden. Heutige Mechanik und strategische Tragfähigkeit offen. |

Die Beispiele zeigen konkrete Anschlussanforderungen: mehrere Entitäten, Rollen, Reihenfolge, situative Einschränkungen und klare Abschnittsgrenzen. Auch eine reine Textaussage ohne Zahl kann veraltet oder unvollständig sein. Eine plausible Entitätszuordnung beantwortet diese Gültigkeitsfrage nicht.

Der historische Lash-Datensnapshot nennt Abrufzeit `2026-07-09T19:36:22.351021+00:00` und Revision `e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06` von `deadlock-wiki/deadlock-data`. Das ist ein damaliger Entitätsdatenbeleg, kein verifizierter Patch der Videoaussagen und keine heutige E/G-Zahlenbasis.

## 4. Empirischer Nachtrag: Zugang vorhanden, aktuelle Claims leer

Der Hauptorchestrator benannte zwei bestehende lokale Peer-Verbindungen. B bestätigte ihre Identität und `transaction_read_only=on` selbst mit `psql -X -w` in vier Transaktionen, jeweils `BEGIN READ ONLY; ... ROLLBACK;`. Keine Credentials oder Secrets wurden gelesen, keine Konfiguration verändert. Historische Backups blieben ungeöffnet. Der früher behauptete Zugangsblocker ist damit ersetzt.

| Verbindung | Aktueller gemessener Bestand |
| --- | --- |
| `/run/deadlock-brain-postgresql`, Port 5446, Rolle `brain_migrate`, DB `brain` | `brain.youtube_learning_claims` nicht vorhanden; `brain_legacy.youtube_learning_claims` 0 Zeilen, `brain_legacy.youtube_videos` 195 |
| Lokale zentrale DB `deadlock`, Rolle `nathanael` | `brain.youtube_learning_claims` 0 Zeilen, `brain.youtube_videos` 195, `brain.youtube_transcript_claim_attempts` 0 |

Die transaktionale Lesesperre ist belegt; ausschließlich lesende Rollenrechte werden nicht behauptet. Die Videozahlen aus beiden Datenbanken nicht addieren. Zusätzlicher Befund des Hauptorchestrators: 549 `youtube_video`-Snapshots und keine Claimtyp-Treffer in dessen Abfrage. Das ist Video-/Quellbestand, kein nachgewiesener Aussagebestand.

### Insight-Herkunft geprüft

B prüfte ausschließlich Quellen-/Typaggregate und strukturierte Herkunft, keine ungefilterten möglicherweise privaten Texte. Alle 51 `insight_records` haben Patchdaten in ihren Payloads; 38 besitzen Patchevent-Referenzen mit `forums.playdeadlock.com`, 13 den Importbatch `steam_0630` und Steam-/Assetquellen. Quellenhosts auch in Referenzen und Payloads sind ausschließlich `forums.playdeadlock.com`, `store.steampowered.com` und `steamstore-a.akamaihd.net`. Keine YT-Quelle in diesen Feldern.

Die beiden vorhandenen Dateien `data/insights/2026-07-01-spark-patch-insights.json` (38 Einträge) und `2026-07-01-steam-0630-insights.json` (13) enthalten ebenfalls Patchklassifikationen. Typen, Umfang und Herkunft passen zu den DB-Aggregaten; zeilenweise Gleichheit wurde nicht behauptet. Gespeicherte Kennzeichen wie `current_until_superseded` sind kein neuer fachlicher Aktualitätsbeweis. Diese Daten sind kein Ersatzbestand klassifizierter YT-Claims.

### Frühere Klassifikationsartefakte gezielt gesucht

`docs/TODO.md` dokumentiert für Juni 2026 frühere Creator-Claimmengen und den Kampagnenpfad `~/.cache/deadlock_brain_campaign/`. `docs/transcript-claims-pipeline.md` beschreibt `verified_<id>.json` und gebündelte Ingestdateien. Der konkrete Kampagnenordner existiert heute nicht am dokumentierten Pfad. In den gezielt geprüften Dokumentations-/Aufgabenpfaden wurde keine weitere konkrete Exportablage lokalisiert. Dateinamenprüfung in `data/raw/youtube/`, `data/youtube_transcripts/`, `data/insights/` und im Gitindex lieferte keinen YT-Claims-JSON-Export. Keine vollständige Festplattensuche und kein Schreiben in Archive.

Damit wurde kein echter historischer YT-Claim mit Modellurteil, Quellenrevision und Entitybindung wiedergefunden. Anders benannte Artefakte und nicht dokumentierte Ablagen sind nicht ausgeschlossen. Historische Mengenangaben sind keine heutigen Messwerte. Aus leeren aktuellen Tabellen und einem fehlenden Kampagnenordner folgt keine Datenverlustbehauptung.

Exakte Verbindungen, Aggregate, Datei-Hashes, geprüfte Orte und Suchgrenzen stehen in `b/YT-EMPIRISCHER-BESTAND.md`. Die frühere Zugangsaussage in `b/YT-KORPUSBEISPIELE.md` ist historisch und durch diesen Nachtrag ersetzt; dessen öffentliche Quellproben bleiben unverändert gültige Quellenbelege.

## 5. Zuordnung, Aktualität und Widersprüche

### Vorhandene Zuordnung reicht nicht als kanonischer Join

Core besitzt interne IDs, externe Primärkennungen und Aliasse (`deadlock-brain-core/src/models.rs:27-76`). Die geprüften Claims binden diese nicht durchgängig als typisierte kanonische API-Referenzen. Der Gapresolver sucht Namen, Slashlisten und Klammern und verwendet den ersten auflösbaren Kandidaten. Er aktualisiert Typ/Name und Verifier; damit entsteht weder eine vollständige Combobindung noch ein versionsgebundener Identitätsbeleg.

Beleg: `rust/crates/dbrain-normalize/src/resolve_gaps.rs:112-188,292-307,349-364` @ main. Eine interne `entities.id` im Legacy-Verifier ist ebenfalls kein kanonischer API-ID-Vertrag.

### Leser verlieren Prüfkontext oder lassen ungeprüfte Aussagen durch

- `claims.rs:178-223` filtert auf main nach Namen und Modell-/Promptpaar, nicht durchgehend nach Status, Patch oder Revalidierung. Der alte HEAD beziehungsweise Hauptbaum-WIP enthält zusätzlich den Guard `needs_claim_revalidation`; main und der geprüfte Q-Commit haben ihn in dieser Funktion nicht. B hat diesen Unterschied unabhängig geprüft. Eine heutige Livewirkung ist nicht gemessen.
- Retrieval sucht Namen/Keywords/Zitate und leitet `verified` aus `accepted` ab. Auch die ausführliche Ausgabe erhält nicht alle Claim-/Entitäts-/Quell-/Zeitbezüge; Verdichtung reduziert weiter. Der Prompt erlaubt Zahlen aus Spieldaten oder „verified“. Das darf alte Creator-Zahlen künftig nicht zu aktuellen Werten aufwerten. Belege: `dbrain-retrieval/src/lib.rs:72-90,4140-4288,5031-5067` @ main.
- Der alte Reasoner liest Claim-JSON nach Heldennamen/Teilstring ohne Status-/Patchfilter (`dbrain-reasoner/src/data.rs:1378-1402`). Die rekursive Itemheuristik berücksichtigt unter anderem jedes Feld namens `id` und zählt Treffer ohne Claimstatusprüfung (`meta.rs:536-563`, Gewichtung `item.rs:344-349`). Dadurch sind numerische Kollisionen zwischen Claim-/Verifier-IDs und Item-IDs im Code möglich. Kein beobachteter Produktionsfall; kein tragfähiger kanonischer Join und kein Aktualitätsbeleg. Fs laufende Bereinigung nicht doppelt beauftragen.

### Verwendbarkeit braucht einen eigenen belegten Prüfstand

Eine Aussage muss getrennt ausweisen, was die Quelle damals behauptete, wie sie damals bewertet wurde und ob sie heute für die Anfrage verwendbar ist. Mindestens folgende Fälle unterscheiden:

| Zustand | Umgang im Entitätsabruf |
| --- | --- |
| Aktuell belegt für den angefragten Kontext | Mit Quelle, Bedingungen und passendem Versions-/Prüfbeleg nutzbar |
| Ungeprüft oder Gültigkeit unbekannt | Als offene/historische Aussage kennzeichnen; keine aktuelle Empfehlung daraus ableiten |
| Widersprochen oder überholt | Nicht als gültigen Ratschlag verwenden; Gegenbeleg und gegebenenfalls Ersatzbezug erhalten |
| Zuordnung mehrdeutig oder visuell unvollständig | Keine erzwungene API-ID; Lücke und fehlende Beteiligte sichtbar lassen |

Das Entfernen alter Zahlanteile macht den Rest nicht aktuell. Der ursprüngliche Wortlaut und seine Zahlen bleiben als Quellenbeleg erhalten. Aktuelle Werte stammen ausschließlich aus E/G. Für Mechaniken braucht es eine fachliche Gegenprüfung gegen den passenden Daten-/Mechanikstand; strategische Wirksamkeit oder „maximaler Schaden“ ist nicht allein durch Namensauflösung oder Zahlenkompatibilität bewiesen. Widersprüche nicht durch Quellenpriorität oder Modellbewertung unsichtbar machen.

## 6. Passender Anschluss an G

G bleibt Eigentümer des Entity-/Ansichtsvertrags. Sein Plan verlangt Art plus positive API-ID und den Abruf als Datenansicht. Im geprüften Commit sind `ToolEntityRef`, `EntityProfileRequest`, `PinnedGameContext`, `ToolEvidenceDependency` und `ToolExecution` vorhanden. Ein vollständiger YT-Wissensabschnitt beziehungsweise Fachadapter ist dadurch noch nicht belegt.

Belege: G `rust/crates/brain-contracts/src/tools.rs:90-93,240-246,773-800`; `G/PLAN.md:45-68,151-162,189-210`. Mainprofilvertrag und Details stehen in `b/YT-G-VERTRAG.md`.

Der bestehende Mainprofilvertrag kennt nur `GameFile` und `Wiki`. Er besitzt bereits Fakten, Qualifizierer, Herkunft, unbekannte/belegte Patchgültigkeit, Konflikte und Lücken. Storage und Retrievalprojektion akzeptieren entsprechend nur vorhandene Quellenarten. YouTube nicht als Wiki umetikettieren. Bei expliziter Patchanfrage werden Fakten ohne passende Gültigkeit ausgeschlossen; ein `source_statement` im Kontext wird dadurch nicht fachlich verifiziert.

Belege @ main: `brain-contracts/src/entity_profile.rs:17-89`; `brain-storage/src/entity_profile.rs:60-63,126-166,288-324,554-558`; `dbrain-retrieval/src/knowledge_projection.rs:78-119`.

### Kleinster brauchbarer Anschlussumfang

1. Einen qualifizierten Wissensabschnitt im bestehenden Entitätsabruf festlegen. Auf bestehende Claim-ID und Originalhash verweisen, keinen unabhängigen Profiltextbestand anlegen. Aktuelle E/G-Zahlen bleiben getrennt von historischen Quellbehauptungen.
2. Jede beteiligte Entität typisiert an Art plus kanonische API-ID binden, mit Rolle und belegter Zuordnungsrevision. Mehrdeutige oder fehlende Zuordnung bleibt eine sichtbare Lücke. Bedingungen und Comboreihenfolge strukturiert erhalten.
3. Quelle, Originalspanne/Videozeit, Veröffentlichungszeit, Aussagezeit oder deren Unbekanntheit, früheres Urteil, letzte fachliche Prüfung mit Beleg sowie Versions-/Patchgültigkeit mitführen. `client_version` ist keine automatisch belegte Balancepatchzuordnung.
4. Unsicherheit, Widerspruch, Überholung und Ersatzbezug erhalten. Rechte und Quellenfreigaben über dieselben Toolabhängigkeiten vor Modellweitergabe, Ausgabe und Cachetreffer prüfen. Öffentliche Quelle bedeutet nicht automatisch eine bestätigte Weiterverwendungsfreigabe.
5. Den Anschluss erst an tatsächlichen öffentlichen Claim-Zeilen abnehmen: Quelle und gespeicherte Aussage stimmen überein, IDs/Rollen sind belegt, aktuelle Zahlen ausschließlich E/G, ungeprüfte Empfehlungen und widersprochene Aussagen gelangen nicht als gültiges Wissen in Antwort oder Renderer.

Das ist ein Vertrags- und Integrationsvorschlag innerhalb der gemeinsamen Entitätsansicht. Mit den aktuell leeren Claimsablagen und ohne lokalisierten historischen Klassifikationsexport ist der YT-Wissensanschluss daraus nicht aktivierbar. Die fehlenden Aussagen in diesem Audit nicht neu erzeugen oder importieren. Keine neue Extraktion, STT/VLM, Modellprobe, YouTube-Verarbeitung, Forumfunktion, zweite Profilpublikation oder Q-Writerfertigstellung. Kein zusätzlicher Provider und keine Einzelmatchablage. Laufende E/F/G-Schreibbereiche bleiben unberührt; der bestehende Release-Halt bleibt bestehen.

## 7. Abschluss und offene Abnahme

Der Nachtrag ist einschließlich empirischer Abnahmeergänzung abgelegt. Die aktuelle leere Claimsablage und die Patchherkunft der untersuchten Insightquellen sind belegt. Ein früherer öffentlicher YT-Klassifikationsexport wurde in den dokumentierten/geprüften Orten nicht lokalisiert; konkrete klassifizierte Entitybeispiele mit Modellurteil bleiben deshalb unbelegt. Kein Zugangsblocker mehr und keine Datenverlustbehauptung. Keine reale Probe wurde als heute gültig freigegeben.

Fachnachweise:

- `b/YT-CODE-PRUEFUNG.md`: Parser, Speicher-/Prüfverträge, Leser, Main/WIP und acht Anschlussbefunde.
- `b/YT-KORPUSBEISPIELE.md`: fünf öffentliche Quellen, Originalspannen, historische Zahlen und vollständige Datei-/Feed-Hashes; frühere Zugangseinschätzung ersetzt.
- `b/YT-G-VERTRAG.md`: eigener Main-/G-Vertragsvergleich und ausdrücklich nicht implementierter Anschlussvorschlag.
- `b/YT-EMPIRISCHER-BESTAND.md`: Peer-Lesezugang, heutige Tabellenzahlen, Patchprovenienz der Insights und dokumentierte Export-Suchgrenzen.

Nur eigene Auditdateien wurden geschrieben. Keine Produkt-, Config-, Git-, DB- oder Runtimeänderung, kein Build, keine Modellprobe und kein Versand. Keine Secrets oder privaten Communitydaten gelesen oder exportiert. B bleibt nach Übergabe zur Abnahme stehen.
