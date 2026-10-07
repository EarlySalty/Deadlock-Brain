# Öffentliche YouTube-Quellproben und Claims-Lesezugang

Stand: 07.10.2026. Rechercheworker für Paket B; Grundlage sind `AUFTRAG.md` und `NACHTRAG-YT.md`. Der Hauptbaum wurde nur gelesen.

## Befund

Fünf vorhandene öffentliche Videoquellen liefern konkrete Beispiele für Combos, Mechanikbedingungen und Spielweisen. Ihre archivierten Feed-Einträge, Untertitelspannen und Datei-Hashes sind nachvollziehbar. Für diese Beispiele konnte ich jedoch keine gespeicherten Claims über einen belegten, zulässigen Claims-Lesezugang abrufen. Die Beispiele unten sind deshalb **Quellproben, keine nachgewiesen klassifizierten DB-Claims**.

Damit bleibt der zentrale Teil des Nachtrags offen: tatsächlichen gespeicherten Aussagewortlaut, Claim-ID, Klassifikation, kanonische Entitätszuordnung und Prüfstand dieser Quellen gemeinsam nachweisen. Ein heutiger Verifikationsbeleg liegt für keine der fünf Proben vor. Zahlenentfernung würde daran nichts ändern.

## Leseweg und Zugangsgrenze

Der verwendete Leseweg besteht ausschließlich aus vorhandenen Dateien unter `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/` und den öffentlichen Spielwissensdateien unter `/home/nathanael/repos/Deadlock-Brain/game-wiki/`. Die ausgewählten Videos stehen in archivierten öffentlichen Kanalfeeds von Deadlock Mythbusters beziehungsweise Natan. Gelesen wurden ihre Spielwissenspassagen, keine privaten Stream-, Mitglieder- oder Communityinhalte. Die heutige Erreichbarkeit der Videos wurde nicht geprüft.

Graphify wurde vor den Bestandssuchen abgefragt. Die Fundstellen führten zu `game_wiki.rs`, `youtube_learning.py`, `transcript_claims.rs`, `claims.rs`, `pg.rs` und dem vorhandenen Brain-MCP. Anschließend wurden die konkreten Dateien und öffentliche Korpuspfade geprüft.

Die vorhandenen Zugänge genügen für einen Claims-Abruf in diesem Auftrag nicht als Sicherheitsbeleg:

- `/home/nathanael/repos/Deadlock-Brain/mcp/README.md` dokumentiert ausschließlich `patch_history`, `patch_search`, `list_patches` und `entity_summary` für `brain.patch_changes`. Die generische SQL-Funktion wurde ausdrücklich entfernt. Dieser dokumentierte Zugang stellt keinen Claims-Leser bereit.
- Im gelesenen Checkout öffnet `Commands::Claims` in `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/main.rs:158` den gewöhnlichen `db::pg_pool()`. `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-yt/src/db.rs` delegiert an den allgemeinen Core-Pool, nicht an `pg_pool_read_only()`.
- Auch die lokale Referenz `origin/main` verwendet für `Claims` den gewöhnlichen Pool. Dort lädt `db.rs` den allgemeinen Laufzeit-Secretsbestand und verbindet ohne gesetztes `default_transaction_read_only`. Eine ausschließlich lesende Datenbankrolle ist aus diesen Fundstellen nicht belegt. Der Claims-Befehl wurde deshalb nicht gestartet.
- `/home/nathanael/repos/Deadlock-Brain/rust/crates/deadlock-brain-core/src/pg.rs:11` enthält zwar `pg_pool_read_only()`. Das belegt eine vorhandene Codefunktion, aber keinen bereits freigegebenen, ausführbaren Claims-Zugang mit nachgewiesener Rolle und Zielbindung. Für diese Recherche wurde kein neuer Aufrufer gebaut.

Die konkrete Zugangslücke ist ein vorhandener, nachweislich lesender Zugriff auf die öffentlichen Zeilen von `brain.youtube_learning_claims`, verbunden mit `brain.youtube_videos`, der Quellen- und Entitätsbezüge sowie Prüfmetadaten sichtbar macht. Die Rolle müsste ausschließlich lesend sein; das Ziel und der Ausschluss von Schreib-, Import- und Modellwirkung müssten am tatsächlichen Werkzeug belegbar sein. Das ist hier nicht nachgewiesen. Es gab keinen fehlgeschlagenen Verbindungsversuch und keine umgangene Sperre.

Keine Secrets, DSNs oder Umgebungswerte wurden gelesen oder ausgegeben. Keine Datenbankverbindung, kein Import, keine STT-, VLM- oder LLM-Verarbeitung, kein Build und kein Test wurden gestartet. Historische Datenbankbackups wurden nicht geöffnet.

## Messbasis und Reichweite

Die Auswahl umfasst fünf Videoquellen von zwei Urhebern. Vier sind Mechanik- beziehungsweise Combo-Shorts von Deadlock Mythbusters; eine weitere Probe stammt aus Natans öffentlicher Heldenübersicht. Die Auswahl ist thematisch gezielt und nicht repräsentativ für den gesamten Claimsbestand. Aus ihr folgt keine Anzahl gespeicherter oder brauchbarer Claims.

Alle 17 Markdown-Dateien unter `game-wiki/pages/` wurden hinsichtlich ihrer strukturierten `game-wiki-entry`-Quellenmarker untersucht. Die Marker nennen ausschließlich `deadlock_data` und `deadlock_wiki`, keinen YouTube-Quellmarker. Dieser Befund gilt für diese 17 Dateien, nicht für die Datenbank oder andere Bestände. Die Spielwissensdateien helfen hier bei historischen Namens- und Quellen-ID-Kandidaten; sie ersetzen keinen Claims-Abruf.

In den untersuchten Aufgaben-, Dokumentations-, Architektur- und Datenpfaden wurde bei einer begrenzten Dateinamensuche bis Tiefe sechs kein fachlich passender gespeicherter `verified_*.json`- oder Claims-Export gefunden. Das beweist nicht, dass außerhalb dieses Suchbereichs keine Exporte existieren. Die alten Mengenangaben in `docs/transcript-claims-pipeline.md` wurden nicht als heutige Bestandsmessung übernommen.

## Öffentliche Quellproben

Für alle folgenden Proben gilt: Der Quellbestand ist vorhanden. Die fachliche Einordnung in diesem Bericht stammt aus der gelesenen Passage; sie ist keine aus der DB gemessene Klassifikation. Claim-ID, Claim-Hash, tatsächlicher gespeicherter Aussagewortlaut, gespeicherte Entity-ID, Status, Prüfurteil, Prüferzeit, Modell-/Promptversion und geprüfter Patch sind nicht zugänglich. Die genannten externen Entitäts-IDs sind historische Zuordnungskandidaten aus dem Spielwissensarchiv, keine nachgewiesenen kanonischen E/G-IDs.

### 1. Lash: Combo nach einer erfolgreichen Parade

Quelle: Deadlock Mythbusters, Video `J2_mZjoqv7I`, „Maximize your combo damage after a parry!“, veröffentlicht am `2026-05-19T22:07:19+00:00`. Archivierte öffentliche URL: `https://www.youtube.com/shorts/J2_mZjoqv7I`.

Originalspanne: `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/transcript_J2_mZjoqv7I.7bb72528fbe4f9b5.vtt`, insbesondere `00:00:09.440` bis `00:00:17.840`. Kurzer Untertitelbeleg bei `00:00:11.070`: „add a flog in the middle of your fillers“.

Die Quelle beschreibt Flog zwischen normalen Schüssen und für eine anspruchsvollere Combo Schüsse vor dem ersten schweren Nahkampfangriff. Die Bedingung ist die Parade-Combo, nicht jede beliebige Kampfsituation. Im Untertitel stehen ursprünglich die Schadenswerte `354`, `336` und `468`. Diese Werte werden hier unverändert als historische Quellwerte festgehalten. Der Wechsel von 354 zu 336 darf weder als Extraktionsfehler still korrigiert noch als heutiger Schadensgewinn ausgegeben werden.

Historische Zuordnungskandidaten: `hero_lash` für Lash und `ability_lash_flog` für Flog. Beide Namen sind im Spielwissensarchiv eindeutig auffindbar. Ein gespeicherter Claim mit diesen Beziehungen ist nicht nachgewiesen. Ob Reihenfolge, Zeitfenster und Schaden heute gelten, bleibt offen; auch „max damage“ ist ohne aktuellen Vergleich nicht tragfähig.

### 2. Yamato: Crimson Slash zwischen normalen Schüssen

Quelle: Deadlock Mythbusters, Video `Z7N885Cz1u4`, „Level up your Yamato combos!“, veröffentlicht am `2026-05-20T20:40:32+00:00`. Archivierte öffentliche URL: `https://www.youtube.com/shorts/Z7N885Cz1u4`.

Originalspanne: `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/transcript_Z7N885Cz1u4.f2aa0e50d65b7579.vtt`, insbesondere `00:00:09.519` bis `00:00:14.559`. Bei `00:00:11.040` steht: „a crimson slash in between the left click fillers“.

Die Quelle beschreibt eine Parade-Combo mit Crimson Slash zwischen normalen Schüssen. Eine weitere Stufe verlangt einen Schuss vor dem ersten schweren Nahkampfangriff. Der ursprüngliche Untertitel nennt `386` Schaden für die einfache Combo. Dieser Wert ist kein aktueller DB-Wert. „Pillars“ in einer Untertitelzeile und „fillers“ in einer späteren Zeile zeigen außerdem, dass die Caption nicht wortgenau als Fachterminologie übernommen werden sollte.

Historische Zuordnungskandidaten: `hero_yamato` und `citadel_ability_healing_slash`. Die benannte Fähigkeit passt sprachlich eindeutig zu Crimson Slash. Ein aktueller kanonischer Join oder heutiger Combo-Beleg fehlt. Das qualitative Timing könnte ebenso durch Änderungen an Fähigkeiten, Nahkampf oder Paradefenstern veraltet sein wie die Zahl.

### 3. Unstoppable und Indomitable: ausdrücklich eingeschränkte Wechselwirkung

Quelle: Deadlock Mythbusters, Video `ORSIFmV5dIo`, „Can you stack Unstoppable with Indomitable?“, veröffentlicht am `2026-05-11T19:17:20+00:00`. Archivierte öffentliche URL: `https://www.youtube.com/shorts/ORSIFmV5dIo`.

Originalspanne: `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/transcript_ORSIFmV5dIo.7e5fbb6fb8745ef1.vtt`, `00:00:05.080` bis `00:00:15.440`. Die Quelle beginnt mit „In most cases“ und sagt anschließend, dass Indomitable trotz aktivem Unstoppable ausgelöst werde. Danach nennt sie abweichende Fähigkeitswechselwirkungen, bei denen Unstoppable den Cooldown von Indomitable verhindere.

Die Aussage ist ein bedingter Mechanikhinweis über zwei Items. Die betrachtete Passage enthält keinen Schadens-, Dauer- oder Cooldownzahlenwert. Ihre Einschränkungen sind trotzdem unverzichtbar: Die Passage allein begründet weder „immer auslösen“ noch eine vollständige Liste der Ausnahmen. Ein allgemeiner Stacking-Ratschlag wäre weiter gefasst als der Beleg.

Historische Zuordnungskandidaten: `upgrade_unstoppable` und `upgrade_auto_cleanse`. Beide Namen passen eindeutig zu den archivierten Item-Einträgen. Aktuelle Triggerregeln und die konkreten Ausnahmen sind nicht verifiziert. Das Fehlen einer Zahl macht die Empfehlung nicht aktuell.

### 4. Doorway und Paige: Nähe zur Tür als Einsatzbedingung

Quelle: Deadlock Mythbusters, Video `BLf3gn-mzXU`, „The doorway has a few hidden mechanics!“, veröffentlicht am `2026-05-26T20:29:27+00:00`. Archivierte öffentliche URL: `https://www.youtube.com/shorts/BLf3gn-mzXU`.

Originalspanne: `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/transcript_BLf3gn-mzXU.d6d954479fad5888.vtt`, `00:00:08.559` bis `00:00:17.349` und die anschließende Zeile zur Anwendung durch die Tür. Die Quelle nennt „the player's proximity to the door“ und beschreibt, dass Paige ihre Flächenfähigkeit normalerweise nicht durch Doorway einsetzen könne, wohl aber auf einer unsichtbaren Plattform an der Tür.

Die Probe verbindet eine Fähigkeit, eine andere Heldin und eine räumliche Bedingung. Eine isolierte Aussage „Paige kann durch die Tür wirken“ würde den ursprünglichen Vorbehalt verlieren. Der kurze Ausschnitt nennt keine aktuelle Distanz oder andere quantitative Grenze. Die Einleitung zählt drei Mechaniken auf; das ist keine gegenwärtige Spielwertangabe.

Historische Zuordnungskandidaten: `ability_doorman_doorway` und `hero_bookworm` für Paige. Die konkret gemeinte Paige-Fähigkeit wird in der Passage nicht benannt und kann daraus nicht zuverlässig auf eine Fähigkeits-ID aufgelöst werden. Das Referenzwort „this“ und die unsichtbare Plattform sind visuell abhängig. Ohne damalige Darstellung und heutigen Mechanikbeleg bleibt die Ausführbarkeit offen. Es wurde keine Videoanalyse nachgeholt.

### 5. Lash: Roaming, Ganks und Neuplatzierung auf erhöhten Positionen

Quelle: Natan, Video `wihHFTapEGk`, „EVERY Deadlock Hero Explained in 30 SECONDS! Playstyles & Builds & Difficulty“, veröffentlicht am `2026-02-15T09:39:16+00:00`. Öffentlicher Videobezug: `https://www.youtube.com/watch?v=wihHFTapEGk`.

Originalspanne: `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/transcript_wihHFTapEGk.f5dd1627563f2986.vtt`, `00:06:23.880` bis `00:06:44.800`. Bei `00:06:25.640` steht „roaming and ganking, looking for kills“. Später beschreibt die Quelle Slam von erhöhten Positionen und Grapple, um sich im Kampf wieder nach oben zu versetzen.

Die Quelle empfiehlt Lash als mobilen Spirit-Burst-Spieler und grenzt diese Spielweise gegenüber dauerhaftem Frontline-Kampf ab. Zur Begründung nennt sie unter anderem höhenabhängigen Slam-Schaden sowie relative Gesundheits- und Cooldowneigenschaften. Auch ohne konkrete Zahl hängen diese Begründungen an Spielwerten und Mechanikstand.

Historischer Zuordnungskandidat: `hero_lash`; Flog ist als `ability_lash_flog` auffindbar. Die konkrete kanonische Zuordnung von Slam und Grapple wurde hier nicht nachgewiesen. Eine wichtige Schnittgrenze liegt unmittelbar davor: Bei `00:06:19.640` bis `00:06:22.400` steht noch „10 permanent spirit power on kills“, bevor der Text zu Lash wechselt. Dieser Zahlenrest darf nicht durch bloße Nähe zum Wort Lash dem neuen Heldenabschnitt zugeschlagen werden.

Die Passage belegt eine damalige Empfehlung. Ein heutiger Beleg für ihre Mechanikbegründung oder ihre strategische Tragfähigkeit fehlt.

## Zeit, Version und Prüfstand

Die Veröffentlichungszeiten stammen aus den archivierten Feed-Einträgen. Die dortigen `updated`-Zeiten lauten für Probe 1 `2026-05-23T01:36:38+00:00`, für Probe 2 `2026-05-23T21:45:34+00:00`, für Probe 3 `2026-05-14T17:03:10+00:00`, für Probe 4 `2026-05-27T08:29:45+00:00` und für Probe 5 `2026-05-24T04:44:39+00:00`. Diese Zeiten belegen keinen erneuten fachlichen Check. Untertitelzeiten geben nur die Position im Video an.

Der historische Lash-Eintrag im Spielwissensarchiv nennt Abrufzeit `2026-07-09T19:36:22.351021+00:00` und die Datenrevision `e3fb36ebbb046e73c44a37ecb31e3e096bfc7a06` von `deadlock-wiki/deadlock-data`. Diese Revision ist ein Beleg für den damaligen Entitätsdaten-Snapshot, kein geprüfter Patch der YouTube-Aussagen und keine aktuelle E/G-Zahlenbasis.

Die Python-Legacyfunktion `_save_verified_claims` und der Rust-Insert in `transcript_claims.rs` lokalisieren den alten Aussagenbestand in `youtube_learning_claims` beziehungsweise `brain.youtube_learning_claims`. Sie zeigen Felder für `video_id`, `claim_hash`, `claim_index`, `entity_type`, `entity_name`, `claim_type`, `claim_text`, `evidence_quote`, `timestamp_seconds`, Status, Verifier, Modell, Promptversion und Speicherzeiten. Das ist ein Speichervertrag, kein Beweis, dass eine bestimmte Probe als Claim vorliegt oder dass ihr früheres Urteil heute gilt. Es wurden keine Claim-Hashes aus den Quellproben erfunden.

Für einen genauen Abgleich fehlen die jeweiligen öffentlichen Claim-Zeilen, ihr Quellenrevisionsbezug und ihr heutiger Prüfbeleg. Ohne diese Daten ist eine Aussage über bereits klassifiziertes, kanonisch gebundenes und aktuell verifiziertes Wissen nicht möglich. Aktuelle Zahlen bleiben ausschließlich Sache von E/G. Aus dieser Recherche entsteht weder ein Profiltextbestand noch eine zweite Profilpublikation.

## Nachprüfbare Dateien und Identitäten

Checkout: `feat/brain-rust-cutover-20260919`, HEAD `2734c2da4e814ff79953e8e825275b0216a6af16`. Zusätzlich wurde der Leserpfad per `git show` gegen die lokale Referenz `origin/main` mit SHA `9711cb630aebacfe959ed4783595b071f479be36` geprüft. Die Referenz wurde nicht frisch geholt; sie ist kein Live-Deploymentbeleg. Die folgenden SHA-256-Werte identifizieren die gelesenen Dateien unabhängig vom schmutzigen Checkout.

Untertiteldateien, jeweils unter `/home/nathanael/repos/Deadlock-Brain/data/raw/youtube/`:

| Probe | Datei | SHA-256 |
| --- | --- | --- |
| 1 | `transcript_J2_mZjoqv7I.7bb72528fbe4f9b5.vtt` | `7bb72528fbe4f9b59badc4412c59695d49a6c8ff2aa13797617708d0299e7088` |
| 2 | `transcript_Z7N885Cz1u4.f2aa0e50d65b7579.vtt` | `f2aa0e50d65b75790dd59460946328ff7aea8637b5cc95b786dc9fead3c2e638` |
| 3 | `transcript_ORSIFmV5dIo.7e5fbb6fb8745ef1.vtt` | `7e5fbb6fb8745ef12832169fc6313f3429d03b8da1c764f33be4e5c03df91417` |
| 4 | `transcript_BLf3gn-mzXU.d6d954479fad5888.vtt` | `d6d954479fad5888a70e7e0cd2b3d9978ffbea68c271921982d0f3ad05c96515` |
| 5 | `transcript_wihHFTapEGk.f5dd1627563f2986.vtt` | `f5dd1627563f298641c2bc1e8f2c8a8552d0b3e083f8a03265984d25e9218c03` |

Feeddateien im selben Verzeichnis:

| Proben | Datei | SHA-256 |
| --- | --- | --- |
| 1, 2, 4 | `feed_channel_dlmythbusters.028ac7236b918910.xml` | `028ac7236b9189109528be622702e4754ca38c4e4d8ae3f3c6291d5c42587da2` |
| 3 | `feed_channel_dlmythbusters.0e6f65145c39615e.xml` | `0e6f65145c39615ee2b6c1e0d83f596c9a4886f71a332dd891717b5ff02075e2` |
| 5 | `feed_channel_natandeadlock.017837dd88b5d41f.xml` | `017837dd88b5d41fb31299833b357ca3a7cd8a864e145db244ff292d5a56ebad` |

Historische Entitätsfundstellen:

- `/home/nathanael/repos/Deadlock-Brain/game-wiki/pages/deadlock-data/hero.md`: Lash ab Zeile 2591, Paige ab 3397, Yamato ab 6210. Genannte Snapshot-IDs: `39359`, `39337`, `39391`; Source-Dokument jeweils `7069`. SHA-256 der Datei: `b858f168036f202da8cdbce9945b131d89721d0f40c23e3313f83d3862c06000`.
- `/home/nathanael/repos/Deadlock-Brain/game-wiki/pages/deadlock-data/item.md`: Indomitable ab 6773, Unstoppable ab 15464. Snapshot-IDs `39999` und `40214`; Source-Dokument `7072`. SHA-256: `aceb2a2d6bc934ba19b372b2cd200e8344de47f9e0a8123a3f17e72bda4dab4c`.
- `/home/nathanael/repos/Deadlock-Brain/game-wiki/pages/deadlock-data/ability.md`: Crimson Slash ab 6366, Doorway ab 7499, Flog ab 9855. Snapshot-IDs `39613`, `39420`, `39470`; Source-Dokument `7070`. SHA-256: `a08987b3afc15bc11cb515340be4ada20ad77cbe7bf6dbfa74eed1bb8dd06889`.

Snapshot- und Source-Dokument-IDs dieser Liste gehören zu den archivierten Entitätsdaten. Sie sind weder YouTube-Claim-IDs noch ein Beleg für eine gespeicherte Beziehung zwischen einem YouTube-Claim und der Entität.
