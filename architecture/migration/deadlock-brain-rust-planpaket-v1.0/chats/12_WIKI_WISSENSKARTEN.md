# Chat 12 · Wiki-Integration und Hero-Wissenskarten


**Startbedingung:** Nach G0 Discovery/zulässige Fixtures vorbereiten; Implementierung nach G1. Pilot G2, Pflicht-Coverage G3, Qualitätsabnahme G4.

**Zusätzlicher Kontext:** U3 vollständig; 11_WIKI_WISSEN_UND_BUILDS.md; Source-/Fact-/Kartenverträge von 02, Store von 03, Workerport von 04 und Fachnormalisierung aus 05.

**Ziel:** Alle freigegebenen Wiki-Inhalte nachvollziehbar katalogisieren und gameplayrelevantes Wissen als versionierte Facts/Mechaniken/Regeln plus Prosa nutzbar machen. Hero-Karten sind generierte Projektionen, keine neue Wahrheitsquelle.

**Eigentümerschaft:** Wiki-Discovery/-Client/-Parser, wiki-spezifische Fixtures, Coverageberichte und HeroCardProjector. Bestehendes wiki.rs nach Inventar erweitern; 00/02 reservieren konkrete Unterpfade. DB-Migrationen 03, gemeinsame Contracts 02, fachlicher Rule-/Synergie-/Buildcode 05, allgemeine Worker 04.

1. Erhebe zulässigen API-/Exportweg, Rechte und alle relevanten Namespaces/Revisionen. Vollmanifest einschließlich Redirects, Abhängigkeiten, Lore/Guides und Medienstatus; keine fest codierten Hero-/Itemzahlen.
2. Speichere Raw/Page-/Revision-/Hash-/Zeit-/Sprach-/Policy-Metadaten über gemeinsame Ports. Strukturierte Datenseiten bevorzugen, Wikitext danach, HTMLfallback explizit.
3. Implementiere seitentypspezifische Parser mit Quellpositionen und diagnostizierter unbekannter Semantik. Effekte/Einheiten/Bedingungen/Varianten an die gemeinsame IR übergeben; fremde Templates/Lua nicht ausführen.
4. Mit 05 stabile Entity-/Ability-/Mechanikbeziehungen und zeitlich saubere Facts prüfen. Alias/Locale trennen; Wiki-Revision nie als erfundenen Patch behandeln. Konflikte/quarantinierte Pflichtfakten nicht still veröffentlichen.
5. Erzeuge HeroKnowledgeCard aus freigegebenen Facts/Rules/Synergien und Quellenreferenzen: kurz abrufbare Profil-/Ability-/Mechanik-/Buildabschnitte, strategische Ableitungen und Unknowns sichtbar. Keine handgepflegten doppelten Stats.
6. Delta-/Template-/Dataabhängigkeitsänderungen selektiv nachführen; unveränderte Inhalte nicht neu embedden. Full Reconcile mit 04/03, Coveragezustände statt bloß importierter Dateizahl.
7. Einen Hero samt Abilities, Items, zwei Mechaniken, Bedingung/Variante, Alias und zwei Revisionen bis Karte/Build/API mit 05/08 integrieren. Danach den freigegebenen Gesamtkorpus verarbeiten.

**Liefergegenstände:** Rust-Adapter/Parser/Projektor, versioniertes Source-/Abhängigkeitsmanifest, WIKI_COVERAGE, Hero-Kartenfixture, Golden-/Delta-/Unknown-/Groundingtests und Abgleichbericht.

**Abnahme:** Pflichtmanifest und buildkritische Facts vollständig geprüft; alle Beziehungen/Einheiten/Zeiten/Quellen stimmen in den festgelegten Tests; Karten rebuildbar, keine zweite Factquelle; R26–R41/R60 soweit zugeordnet belegt.

**Erster Schritt:** Kleinsten erlaubten Wiki-Quellbestand und dessen tatsächliches Format feststellen; Parser-/Contractanforderungen an 02/03/05 liefern. Kein Massencrawl vor Freigabe.



**Verbindlich:** Eigene reguläre Backend-, Worker-, Parser-, Learning- und Rebuildpfade sind Rust. Kein notwendiger Python-/JVM-/.NET-Sidecar, kein Legacy-HTTP-Kern. Externe fremdverwaltete Datenfeeds dürfen konsumiert werden; die eigene Implementierung/Steuerung bleibt Rust. Optionale Offline-Referenzwerkzeuge und kleine Hilfsskripte nur dokumentiert. Rechte/Egress setzt Code durch.

**Arbeitsregeln:** Lies 00_START_HIER.md, 01_MASTERPLAN.md, 02_GEMEINSAME_REGELN.md, 06_VERTRAEGE_UND_GRENZEN.md, 08_ERGAENZUNGEN_INTEGRIERT.md und 10_REIHENFOLGE_UND_PARALLELITAET.md sowie STATUS/ADRs/Übergaben unter architecture/migration/. Vor Änderungen Basis-Commit, Contract-/Schemaversion, reale freigegebene Pfade und Arbeitsmodus nennen. Vor G1 nur vorbereiten. Kein zweiter Store, Scheduler, Vertragsentwurf als heimlicher Runtimevertrag oder eigener LLMpfad. Änderungen an fremden Modulen per CHANGE_REQUEST.

**Nachweis/Übergabe:** Eigene Tests und relevante Integrationstests tatsächlich ausführen; Command, Commit, Ergebnis und nicht ausgeführte Prüfungen trennen. Golden-/Mocknachweis ersetzt keine echte Daten-/Providerintegration. Ausgefüllte vorlagen/UEBERGABE.md mit Commit/PR, Grenzen und next-owner liefern. Ohne Codezugang keine Änderungen oder Tests behaupten. Erst integrierter Commit und bestätigte Checks machen den Auftrag fertig.
