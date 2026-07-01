# Architektur

Deadlock Brain ist als Daten-Fundament fuer einen spaeteren Discord-/Website-/Coaching-Bot gedacht.

## Prinzip

Die KI ist nicht die Datenbank. Die Datenbank ist das Gedaechtnis, die KI ist der Analyst.

## Source-Prioritaet

1. Patchnotes: hoechste Prioritaet fuer Veraenderungen ueber Zeit.
2. Deadlock Assets API: aktueller strukturierter Ist-Zustand.
3. Google Sheet: Community-Stats, Scaling, DPS/DPM, Boon-/Ratio-Kontext.
4. Wiki: Interaktionen und Edge Cases, aber schonend und versioniert.
5. Discord/Community spaeter: Hypothesen, nicht Fakt.

## Aktuelle Tabellen

- `source_documents`: jedes gezogene Rohdokument mit Hash und Pfad.
- `entity_snapshots`: strukturierte Snapshots aus Quellen, z.B. Hero, Item, Patchnote.
- `forum_claims`: historische Claims aus oeffentlichen Forum-Posts, inklusive
  Post-Link, Datum, Autorrolle, Vertrauensart, Gültigkeitsstatus und
  Quarantäne-Status; diese Claims überschreiben keine aktuellen Spieldaten.
- `entities`: kanonische Entities aus Snapshots, aktuell Hero, Hero/Internal,
  Item, Item/Special, Ability, Ability/Internal, Weapon/Internal und Rank.
- `entity_aliases`: Aliase je Entity, z.B. Anzeigename, API-ID und Classname.
- `source_runs`: Ingestion-Laeufe mit Summary.
- `patch_events`: einzelne Patchzeilen mit Source-Art, Patch-URL, Section, Entity,
  Change-Type, alter/neuer Wert soweit deterministisch erkennbar.
- `patch_event_enrichments`: deterministische Details pro Patch-Event, z.B.
  Stat-Name, alte/neue Werte, Einheit, Ability-Name, sekundaere Entity,
  Confidence und Parser-Flags.
- `entity_lineage`: Rename-/Rework-/Replacement-Beziehungen aus Patchnotes,
  inklusive alter und neuer Namen sowie Quellenzeile.
- `legacy_entities`: alte/entfernte Entity-Namen aus Patchnotes, die nicht in
  der aktuellen API und nicht schon ueber Lineage modelliert sind.
- `hero_stat_profiles`: normalisierte Google-Sheet-Zeilen je Hero-Snapshot.
- `hero_stat_values`: einzelne Sheet-Stats als Key/Value-Daten mit optionalem
  numerischem Wert.
- `analysis_notes`: persistierte Review-Kontexte und spaetere KI-Ergebnisse mit
  Kontext-Hash, Prompt-Version, Modellname, Confidence und Quellen.

## Naechste Ausbaustufe

- Echter Modellaufruf auf Basis von `review_context` und Speicherung des
  Ergebnisses in `analysis_notes`.
- Build-Module mit Patch-/Meta-Kontext.

## Entity-Normalisierung

`deadlock-brain normalize entities --rebuild` baut `entities` und
`entity_aliases` aus bestehenden `entity_snapshots`. Die erste Version nutzt
`deadlock_assets_api`: Heroes aus `/v2/heroes`, Items und Abilities aus
`/v2/items` plus Raw-Snapshots. Nur aktive, spielbare Heroes werden als `hero`
markiert; disabled/in-development/test Heroes werden `hero_internal`. `/v2/items`
enthaelt viele interne Eintraege; darum werden nur shopbare, nicht deaktivierte
Upgrades mit Item-Slot als `item` markiert. T5-/Sonder-/Modus-Items werden
`item_special`. `type=ability` wird nur dann `ability`, wenn es eine aktive
Hero-Faehigkeit mit lesbarem Namen ist; Future-/Test-/interne Faehigkeiten werden
`ability_internal`. Alles uebrige aus diesem Bereich bleibt konservativ
`weapon_or_internal`.

## Patchnotes-Parsing

Der Parser arbeitet bewusst quellenneutral. Steam-Announcements und Forum-Posts
werden aus derselben `patchnote`-Snapshot-Tabelle gelesen. `source_kind` wird aus
der URL abgeleitet:

- `steam`: Steam Community, Steam Store oder Akamai Steam-News-URLs
- `forum`: `forums.playdeadlock.com`
- `other`: alles andere

Alte Forum-Posts, die als eine lange Zeile mit vielen ` - `-Bullets gespeichert
wurden, werden vor dem Parsing in einzelne Bullet-Zeilen aufgeteilt.

Wenn `entities` vorhanden sind, nutzt der Parser die normalisierten Alias-Tabellen
zur Entity-Aufloesung. Dadurch werden Patch-Events nach `hero`, `item`,
`ability` oder `weapon_or_internal` klassifiziert, statt Item-/Ability-Zeilen nur
grob zusammenzufassen.

## Forum-Claims

`deadlock-brain pull forum` speichert öffentliche Thread-Seiten als Rohquellen
und Post-Snapshots. `deadlock-brain parse forum-claims` baut daraus eine
separate historische Claim-Schicht. Diese Claims sind absichtlich nicht Teil der
aktuellen Grundwahrheit: `currentness` steht auf `historical_quarantine`, alte
Community-Reports bleiben `historical_unverified`, und Entwicklerantworten wie
`fixed internally` werden als `fixed_or_obsolete` markiert. Jeder Claim enthält
einen konkreten Thread-/Post-Link in `source_url` und `source_references_json`,
damit die Originalstelle nachlesbar bleibt.

Aktuelle API-, Patch- und Sheet-Daten haben Vorrang. Forum-Claims dürfen nur als
historischer Kontext, Regressionssignal, Dev-Beleg oder explizit angeforderte
Altfall-Recherche in Antworten einfließen.

## Deterministische Anreicherung

`deadlock-brain enrich patch-events --rebuild` baut `patch_event_enrichments`.
Der Schritt ist bewusst nicht KI-basiert. Er extrahiert robuste Standardmuster
wie `Cooldown increased from 24s to 28s`, Ability-Prefixe, Item-Upgrade-Bezuege
und Removed-/Added-Grant-Zeilen. Unbekannte Formen bekommen trotzdem eine Zeile
mit niedriger Confidence, damit spaetere Auswertung zwischen "nicht verarbeitet"
und "keine Daten vorhanden" unterscheiden kann.

`deadlock-brain normalize sheet-stats --rebuild` baut `hero_stat_profiles` und
`hero_stat_values` aus dem Google Sheet. Gematcht wird konservativ gegen echte
Hero-Anzeigenamen, nicht gegen interne Classname-Kuerzel. Wenn eine Sheet-Zeile
nicht eindeutig zur aktuellen API passt, bleibt `entity_id` leer und wird in der
Summary als `unmatched_heroes` sichtbar.

`deadlock-brain context <Name>` ist der erste Retrieval-Baustein. Er sucht die
beste Entity per Alias, zieht Patch-Historie, Enrichment-Zeilen und Sheet-Stats
zusammen und liefert JSON oder mit `--pretty` einen kompakten Menschen-Output.

## Rename- und Rework-Lineage

`deadlock-brain enrich lineage --rebuild` baut `entity_lineage` aus Patchnotes.
Das ist wichtig, weil die aktuelle Deadlock API alte Namen nicht zwingend noch
enthaelt. Beispiele aus den importierten Patchnotes:

- `Backstabber -> Stalker`
- `Debuff Remover -> Dispel Magic`
- `Curse -> Cursed Relic`
- Hero-Ability-Renames wie `Kudzu Bomb -> Entangling Thorns`

`context`, `timeline` und `review` verwenden diese Lineage als zusaetzliche
Lookup-Namen. Eine Timeline fuer `Stalker` zieht dadurch auch alte
`Backstabber`-Events, und eine Suche nach `Backstabber` bleibt historisch
nutzbar, obwohl der aktuelle API-Name anders ist.

`deadlock-brain enrich legacy-entities --rebuild` baut zusaetzlich
`legacy_entities`. Diese Tabelle sammelt alte/entfernte Namen, die in Patchnotes
auftauchen, aber in der aktuellen API fehlen und nicht bereits durch Rename-
Lineage erklaert sind. Offensichtlich kaputte Parser-Subjekte werden mit
`suspect_parser_subject` markiert.

## Timeline und Review-Kontext

`deadlock-brain timeline <Name>` baut eine chronologische Entity-Timeline aus
`patch_events` und optional `patch_event_enrichments`. Die Ausgabe ist nach
Patch gruppiert. Jedes Event bekommt eine deterministische Einschaetzung:
`numeric_buff`, `numeric_nerf`, `functional_change`, `rework`, `bugfix`,
`added`, `removed` oder `unknown`, plus Level `high`, `medium`, `low` oder
`unknown`.

`deadlock-brain review <Name>` ist die Schicht fuer spaetere KI-Antworten. Sie
ruft kein Modell auf, sondern baut einen kompakten Kontext mit Entity-Summary,
aktuellen Sheet-Hints, Timeline-Signalen, offenen Fragen, Quellen und einem
deutschen Prompt-Entwurf. Der Prompt schreibt vor, Namen von Items, Heroes und
Abilities auf Englisch zu behalten und Unsicherheiten klar zu markieren.

`deadlock-brain analysis save-review <Name>` persistiert den Review-Kontext in
`analysis_notes`. Dadurch kann spaeter nachvollzogen werden, welcher Kontext und
welche Prompt-Version zu einer KI-Antwort gehoert haben. Ein echter Modellaufruf
ist bewusst noch nicht Teil dieses Schritts.

## Qualitaetssicherung

`deadlock-brain quality` prueft die lokalen Daten gegen die normalisierten
Deadlock-API-Snapshots und abgeleiteten Tabellen. Aktuelle Checks:

- Pflicht-Tabellen vorhanden.
- Entity-Counts plausibel gegen lokale API-Snapshots.
- Alias-Kollisionen.
- Patch-Events mit unbekannten Entities.
- Fehlende oder low-confidence Enrichments.
- Fehlende oder leere Lineage.
- Fehlende oder leere Legacy-Entity-Ableitung.
- Sheet-Profile ohne Entity.
- `general`-Patchzeilen, die bekannte API-Entities nennen.

## Wiki-Policy

Kein Bulk-Crawl. Nur gezielte Seiten, Cache-TTL, Mindestabstand zwischen Requests.
Wenn spaeter viele Seiten benoetigt werden, erst nach Dumps, Spiegeln oder expliziten
Rate-Limit-Regeln suchen.
