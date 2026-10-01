# Schnittstellen-/Freigabeanforderung CR-S06-001

Antragsteller: S06. Status: **vorgeschlagen, nicht freigegeben**.
Basis: `c00fc8935048bf490c1e4790f7c6195864ad49e2`.
Contract-/Schema-Version: offen. Plan: 1.0.
Requirements: primär R11, R17, R30, R35, R38; zusätzlich Schnittstellen zu R02, R06, R08, R14, R19, R23, R24, R32, R34, R40, R42, R43, R51, R53, R58, R60.

## Anlass und kleinste kompatible Lösung

Die vorhandene Retrieval-Crate bleibt erhalten. Die Startprüfung in `README.md` zeigt einen unimplementierten Rust-Vektorpfad, bestehende heuristische Textsuche und Einstiegssignaturen ohne die neuen gemeinsamen Auth-/Knowledge-Verträge. G0/G1 sind offen. S06 erfindet deshalb weder neue öffentliche Typen noch ein unabhängiges ACL-/Release-/Embeddingmodell.

Die Namen aus dem Plan sind hier nur semantische Referenzen. S02 veröffentlicht die tatsächlichen kompatiblen Typen/Ports, S03 die Stores und Schema-Version. Kein hier beschriebenes JSON ist ein Laufzeit-Wireformat.

## Benötigte Übergaben nach Besitzer

| Besitzer | Benötigtes Ergebnis | Warum S06 sonst blockiert bleibt |
|---|---|---|
| S00 | integrierte G0-/G1-Nachweise, explizite S06-Arbeitsfreigabe, verbindlicher Basis-Commit; Überschneidung `dbrain-retrieval/src/game_wiki*` mit S12 auflösen | keine eigenmächtige Gate-/Ownerentscheidung |
| S01/S10 | aktiver Referenzpfad, reproduzierbarer Corpus-/Rechtebestand, Lastprofil; unabhängige Queries/Labels und Qualitätsmargen | keine belastbare Suchentscheidung oder Vergleichbarkeit |
| S02 | verifizierter Auth-Kontext; Source-/Object-ACL und Provider-Egress; Budget/Deadline; gemeinsames Evidence-/Retrieval-/Fehlermodell; kompatibler Embeddingport | keine Authentifizierung durch Anfragefelder oder Modellurteil; keine privaten Ersatzports |
| S02/S03 | ein Knowledge-Release für Text, Vektoren, Facts, Graph und Karten; gesondert aktuelle Widerrufe/Tombstones; vollständige Versions-/Provenienzreferenzen | keine Vermischung alter und neuer Generationsstände; Rollback darf keine Rechte reaktivieren |
| S03 | kanonische Snapshot-/Chunk-/Factlesewege, geregelter Index-Staging-/Publish-/Rebuildzugriff, isolierbare Testdaten; erforderliche SQL-Migrationen allein durch S03 | abgeleitete Indizes sind keine zweite Wahrheit; kein konkurrierender Writer |
| S04 | versionierte Index-/Delta-/Tombstonejobs und gezielte Invalidierung; begrenzte Ressourcen und kontrollierte Veröffentlichung | kein S06-eigener Worker oder Releasezeiger |
| S05 | strukturierte Fact-/Berechnungsevidenz mit Einheiten, Inputs, Algorithmusversion; begrenzte Mechaniknachbarn mit korrekter Herkunft | exakte Zahlen und Mechanik nicht aus semantischer Ähnlichkeit ableiten |
| S07 | feste Embeddingrevision und Vorverarbeitung, Dimension/Distanz/Normalisierung; erlaubte Query-/Dokument-Egresswege; getrennt abschaltbares Jev-Relevanzangebot | reproduzierbare Dense-Baseline und sichere externe Datenweitergabe |
| S08 | tatsächlicher Retrievalaufruf, finale Rechte-/Zitatprüfung, Budgetübergabe und sichere Degradierung; Paritätsentscheid für `analysis_run_ai` mit S07 | bestehende Funktionen erhalten, aber keinen zweiten Answer Kernel fortschreiben |
| S12 | abschnittsbezogene Wiki-/Kartenevidenz mit stabilen Entities, Patch/Mode, Locale und Fact-/Quellrevisionen | keine Mega-Kartenembeddings oder ungeprüfte Versionsmischung |

## Zwingende semantische Grenzen

Der gemeinsame Vertrag muss eine fachlich leere autorisierte Suche von fehlendem Index, inkompatibler Generation, abgelaufener Deadline, erschöpftem Kandidatenbudget, verweigertem Egress und fehlgeschlagener ACL-Prüfung unterscheiden können. Die genaue Fehlerdarstellung gehört S02/S08; kein von S06 eingeführtes öffentliches Fehlerschema.

Aktuelle Rechteprüfungen sind bei direkter Suche, Nachbarschaftserweiterung, Cachehits, Jev, Antwortmodell und Zitaten erforderlich. Ein historisches Release ersetzt die aktuelle Policy nicht. Unzulässige Scope-Anfragen werden nach der gemeinsamen API-Regel abgelehnt oder eingeschränkt, nie durch S06 oder Jev erweitert. Auch Query-Embeddings dürfen keine vertraulichen Fragen an nicht zugelassene Anbieter senden.

## Auswirkungen, Übergang und Rückwärtskompatibilität

Nach Freigabe bleiben vorhandene Entity-, Timeline-, Review-, Quality-, Item-, Build-, Claim- und Wiki-Funktionen in der Paritätsmatrix erhalten. S06 entfernt weder `analysis_run_ai` noch den Altbestand stillschweigend. Die Kernel-/Providertrennung erfolgt über getrennt besessene Folgeänderungen mit S07/S08 und Consumerprüfung.

Die vorliegende Vorbereitung verursacht keine Schemaänderung, keinen Backfill, kein Reembedding, keine neue Abhängigkeit und keine Datenübermittlung. Ein konkreter SQL-/Indexbedarf wird nach den gemeinsamen Verträgen an S03 übergeben, nicht in fremden Migrationspfaden implementiert. Root-/Lockfileänderungen bleiben S02 vorbehalten.

Regressionen nach Implementierungsfreigabe: `CASES.csv`, bestehende Retrievaltests, gemeinsame Contract-/ACL-/Generationstests, unabhängig gelabelte Recall-/Latenzmessung und echter S03–S08-Durchstich. Mock-/Fixtureerfolge bleiben von echter Integration getrennt.

## Entscheidung von Besitzern und S00

Freigabe/ADR: offen.
Integrierter G1-Commit: offen.
Neue Contract-/Schema-Version: offen.
Bestätigte Pfade: offen.
Corpus-/Label-/Rechtefreigabe: offen.
Nächste freigegebene Arbeitswelle: keine durch S06 erteilt.
