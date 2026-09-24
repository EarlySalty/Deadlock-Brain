# Wiki, Hero-Wissenskarten und deterministische Builds

**Grundlage:** U3, besonders „Ziele“, „Zielarchitektur“, „ETL“, „Build-Generierung“ und „Validierung“; ergänzt um U1/U4 und die verbindliche Rust-Vorgabe. Alle folgenden Punkte sind Umsetzungsaufträge, keine bereits importierten Daten.

## 1. Verantwortungen ohne zweite Facharchitektur

| Teil | Besitzer | Übergabe |
|---|---|---|
| Quellenmanifest und Wiki-Discovery/Parser | 12 | versionierte Raw-Records und strukturierte Extraktionskandidaten |
| Source Contract, Facts/Effects/Rules/Card-Wiretypen | 02, mit 03/05/12 | ein integrierter Contract |
| Canonical Store, Provenienz und Zeitmodell | 03 | persistierte/versionierte Fakten und Rechte |
| fachliche Normalisierung, Formeln, Mechaniken/Synergien und Planner | 05 | geprüfte Fakten, Beziehungs-/Berechnungsevidenz |
| Wissenskarten-Projektion | 12 | maschinenlesbare Karte aus freigegebenen 03/05-Daten |
| Worker, Delta, Feeder, Veröffentlichung | 04 | gemeinsame Jobs/Checkpoints, keine Wiki-Sonderpipeline |
| strukturierte/Graph-/Textsuche | 06 | ein Retrievalport mit konsistenten Ergebnissen |
| Antwort und Veröffentlichung | 08/09 | ein Kernel, freigegebene Ausgabe-/Exportprofile |
| Qualitätsabnahme | 10 | unabhängige Coverage-/Grounding-/Buildtests |

Bestehende passende Rust-Crates aus U4 nach Prüfung erweitern. Keine zweite „Wiki-Datenbank“ neben dem Brain-Store und kein zweiter Build-Reasoner nur für neue Karten.

## 2. Was „vollständig“ konkret heißt

Ein versioniertes Source Manifest erfasst alle im zugelassenen Wiki-Zugang entdeckten Seiten/Namespaces und ihre Kategorie: Heroes, Abilities, Items, Mechanics, Spielregeln, Patch-/Update-Historien, Lore, Guides, Localization, Redirects und Medienmetadaten. Kein fester Hero-/Itemzähler aus U3 wird in Code übernommen.

Vor G1 den Pflichtumfang festschreiben: alle entdeckten gameplayrelevanten Seiten sowie strukturierte Daten und ihre referenzierten Abhängigkeiten. Erklärtexte/Lore/Guides ebenfalls katalogisieren; Übernahme oder freigegebene Begrenzung je Klasse belegen. Bilder/Audio/Modelle zunächst als Metadaten/Verweise erfassen; Full-Mirror erst nach gesonderter Rechte-/Speicherentscheidung. Nicht erreichbare Seiten bleiben „unzugänglich“, nicht „nicht vorhanden“.

Coverage getrennt ausweisen: `discovered`, `fetched`, `parsed`, `normalized`, `validated`, `published`, `quarantined`, `unavailable`, `policy_blocked`, `approved_exclusion`. Eine vollständig befüllte Inventarliste ist nicht automatisch ein vollständig nutzbarer Knowledge Store. Für jede Pflichtklasse Denominator/Discovery-Stichtag sowie Änderungen seit dem Stichtag angeben.

U3 schlägt 100 % Manifestabdeckung und mindestens 99,5 % Parsererfolg auf bekannten strukturierten Seiten vor. Diese Werte sind Sollvorschläge. Für verpflichtende Build-Facts und Hard Constraints gilt zusätzlich: keine ungeklärte fehlende oder fehlerhafte Grundlage. Ein guter Gesamtdurchschnitt rechtfertigt keinen defekten Hero-Build.

## 3. Discovery und sichere Extraktion

Zuerst zulässigen API-/Exportzugang, Abrufregeln, Identifikation, Limits und Nutzungsfreigabe erfassen. Bevorzugt `Data:*` und andere strukturierte Datensätze, dann Templates/Module als Daten-/Regelreferenz und Wikitext/Revisionsexporte; gerendertes HTML nur als dokumentierter Fallback. Vorhandensein/Erreichbarkeit einzelner Endpoints nicht aus der Recherche voraussetzen.

Jeder Abruf bewahrt Page-ID, Revision, Titel/Namespace, kanonischen Verweis, Sprache, Quellzeit, Abrufzeit, Hash, Rechte-/Egressstatus und Abhängigkeiten. Auch Redirect-/Move-/Delete-Ereignisse und Template-/Moduleabhängigkeiten erfassen. Gleiche Seite/Revision idempotent verarbeiten; ältere Events dürfen keine neue Revision überschreiben.

Raw zuerst speichern, danach reine Parsertransformierung. Seitentypen: Hero, Ability, Item, Mechanic, PatchHistory, DataNamespace und generische Prosa. Diagnose/Quellposition muss jeden nicht verstandenen Ausdruck sichtbar machen. Keine dynamische Ausführung fremder Lua-/Template-/Pythonfragmente; Formeln nur nach Übersetzung in eine begrenzte, typisierte Ausdruckssprache.

## 4. Kanonisches Wissensmodell

Ein gemeinsamer Entitätsraum für Hero, Ability, Item, Mechanic, StatDefinition, Patch/Mode, LocalizedText, Alias und Lineage. IDs kommen aus stabilen Quell-IDs oder einer persistenten Mappingtabelle, niemals allein aus dem aktuellen Anzeigenamen. Slugs bleiben adressierbare Aliase; Umbenennung erzeugt nicht automatisch eine neue Entity. Aufspaltungen/Zusammenführungen historisch modellieren.

Facts tragen Subject-/Predicate, typisierten Wert, Einheit, Operation, Bedingungen, Mode/Variante, Game-Gültigkeit, Source-Revisionen, Ableitung, Prüfstatus und Knowledge-Release. Prozentanteil, Prozentpunkt und Multiplikator getrennt halten. Dezimalpräzision/Rundung vor Berechnungen definieren. Fehlendes Feld ist unbekannt, nicht 0; ein unbekannter boolescher Wert ist nicht automatisch false.

Effects von Items und Abilities nutzen dasselbe Modell: Trigger, Bedingungen, Damage-/Heal-/Status-/Bewegungseffekte, Ziel, Dauer, Intervall, Reichweite, Skalierung, Stacking und Cooldown. Nicht unterstützte Semantik darf nicht durch plausiblen Freitext zu einer berechenbaren Regel werden.

Ein Mechanikgraph wird zunächst im vorhandenen Store abbildbar entworfen. Extra Graph-Datenbank nur nach belegtem Bedarf. Kanten wie `SCALES_WITH`, `MODIFIES_STAT`, `USES_MECHANIC`, `TRIGGERS`, `COUNTERS` besitzen dieselbe zeitliche/provenienzbezogene Prüfung wie Facts.

## 5. Patchsicherheit, Herkunft und Konflikte

Wiki-Revision, Upstream-Game-Build und Game-Patch sind getrennte Größen. `observed_at` oder ein Gitcommit-Datum beweisen keine Spielgültigkeit. Unbekannte Zuordnung bleibt unbekannt. Rückwirkende Korrekturen werden historisiert, nicht destruktiv überschrieben.

Patchnotes erklären beabsichtigte/angekündigte Änderungen; Gamefiles zeigen einen beobachteten Datenstand; Wiki erläutert; Replays zeigen beobachtetes Verhalten; Population beschreibt eine Stichprobe. Autorität hängt von Aussageart und zeitlicher Zuordnung ab. Abweichungen nicht per globaler „höchster Trust gewinnt immer“-Regel beseitigen. Alle betroffenen Quellenrevisionen bleiben im Reconciliation-Fall erhalten.

Synergien unterscheiden `explicit_source`, `mechanical`, `rule_derived`, `empirical_observation` und `model_hypothesis`. Quellenstatus/Unsicherheit nicht allein aufgrund dieser Klasse als kalibrierten Prozentwert ausgeben. Modellhypothesen können geprüft werden, aber weder harte Spielregeln noch kanonische Zahlen ersetzen. Gemeinsame Upstream-Gamefiles nicht als mehrere unabhängige Bestätigungen zählen.

## 6. Hero-Wissenskarte: konkrete geforderte Ausgabe

`HeroKnowledgeCard` ist eine reproduzierbar erzeugte JSON-Sicht, optional zusätzlich lesbares Markdown. Sie ist eine **Wissenskarte des Spielcharakters**, keine trainierte Modellgewichtsdatei und kein Ersatz für die Faktenbank.

| Kartenbereich | Inhalt / Schutz |
|---|---|
| Identität | stabile Hero-ID, Namen/Aliase, Sprache, Mode, aufgelöster Patch, Knowledge-Version |
| Faktenbasis | Basiswerte, Einheiten, Waffen-/Abilityzuordnung, Skalierungen, Upgrade-/Levelregeln soweit belegt |
| Fähigkeiten | Effekte, Bedingungen, Kosten/Cooldowns, Mechaniken, Quellen- und Fact-IDs |
| Spielverständnis | Stärken, Schwächen, Rollen, Ausführungsmuster; jeweils als Quellenbehauptung oder Ableitung markiert |
| Buildprofil | mechanische Bedürfnisse, Synergien/Antisynergien, relevante Regeln, Gegenkontext |
| Empirische Hinweise | Patch, Stichprobe, Kontext und Observation-/Population-Referenzen; keine Kausalitätsbehauptung aus Winrate |
| Unsicherheit | unbekannte Werte, ungeklärte Konflikte, begrenzte Parserabdeckung und veraltete Evidenz |
| Provenienz | Source-/Fact-/Rule-/Algorithmusrevisionen, Generierungsversion und Sichtbarkeit |

Kurze Profilkarte plus separat ladbare Ability-/Mechanik-/Buildabschnitte statt immer eine riesige Karte in den Prompt. Numerische Facts direkt bereitstellen. LLM-Zusammenfassungen sind gekennzeichnete, invalidierbare Zusatzansichten; nach einem Fact-/Rule-/Patchwechsel nur betroffene Karten/Abschnitte neu erzeugen.

`current` wird einmal pro Request auf eine geprüfte Knowledge-/Patch-Version aufgelöst. Fehlende Patchzuordnung darf nicht als sicher „aktuell“ ausgegeben werden. Deutsch/Englisch teilen dieselben Entities und Zahlen; Texte/Übersetzungsherkunft separat.

## 7. Deterministischer Build Planner in Rust

Context Resolution → legale Kandidaten → numerische Features → mechanische/kontextuelle Synergien → begrenzte Suche/Optimierung → überprüfte Auswahl → quellengestützte Erklärung.

Anfrage führt Hero, Patch/Knowledge-Version, Mode, Budget, Spielphase, freigeschaltete Kapazitäten, bereits gekaufte Items/Upgrades, Ziele und optional Gegnerkontext. Relevante Grenzen aus **versionierten Regeln** beziehen, nicht aus den historischen Beispielzahlen in U3. Regeln zu Inventar, Active Items, Modus, Budget, Upgradepfaden und Ausschlüssen werden vom Code geprüft; unbekannte notwendige Regeln führen zu einer klar begrenzten/abgelehnten Anfrage.

Formeln über eine beschränkte AST evaluieren. Einheiten prüfen, NaN/Overflow/Division durch null behandeln, deterministische Sortierung und Seeds dokumentieren. Searchbudgets begrenzen CPU/RAM und Laufzeit; Abbruch kann ein legales best-so-far-Ergebnis mit ausgewiesenem Suchlimit liefern, aber kein behauptetes globales Optimum. Kleine Fälle gegen vollständige Enumeration testen, größere mit fixierten Vergleichen.

Plannerresultat enthält Items, berechnete Eigenschaften, erfüllte/verletzte Constraints, verwendete Rule-/Fact-IDs, Algorithmus-/Datensatzversion und Alternativen. Scores sind kalibrierbare Heuristiken, keine Wiki-Fakten. Ein Antwortmodell erklärt erst das validierte Resultat und darf keine anderen Zahlen/Items/Regeln erfinden.

## 8. Qualität und Gates

G1: Entity-/Fact-/Effect-/Rule-/Kartenvertrag, Zeitachsen, Einheiten und Source-Mapping fixiert.

G2: ein Hero mit belegten Abilities, mehreren Items und mindestens zwei unterscheidbaren Mechaniken, Varianten/Bedingung, Alias und zwei Revisionen bis Karte/Build/API durchlaufen. Die in U3 genannten Seiten sind Kandidaten für Fixtures, keine garantiert aktuell passenden Werte. Unbekannte Entity und absichtlich illegaler Build müssen sicher erkannt werden.

G3: Pflichtmanifest abgeglichen, keine ungeklärten buildkritischen Lücken, Historien erhalten, Mechanik-/Synergiegraph ohne verwaiste Referenzen, Karten aus Facts rebuildbar. Lore/Guides/Medienstatus separat transparent. Änderung einer Template-/Data-Abhängigkeit invalidiert alle betroffenen Facts/Karten, nicht alles pauschal.

G4: numerische Aussagen in Golden-Tests zu 100 % auf erlaubte Fakten/Berechnungsevidenz rückführbar; alle modellierten Hard Constraints eingehalten; richtige Mode-/Patchversion; unbekannte Entities/Regeln werden nicht erfunden. Menschliche Paarvergleiche und getrennte Holdouts prüfen Buildqualität. Tests beweisen keine absolute Allwissenheit oder global optimale Builds.

Verpflichtende Fälle: Prozent vs. Prozentpunkt, Einheit, Rundung, Bedingungen/Variante, Alias/Rename, Build-Up vs. Stack als getrennte IDs, Patch-Unbekannt, Zukunftsdaten, korrelierte Quellen, Quarantäne, Delete/ACL-Revoke, gezielte Reprojektion und fehlende Build-Legalitätsdaten.
