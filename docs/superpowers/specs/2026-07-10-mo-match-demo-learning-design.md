# Mo & Krill Match-Demo-Lernen

Stand: 2026-07-10
Status: fachlich freigegeben

## Ziel

Deadlock Brain soll aus echten Matches nachvollziehbare Spielentscheidungen
lernen. Der erste vertikale Pilot nutzt alle auffindbaren Mo-&-Krill-Matches
des Spielers `kaptennn` (Account-ID `281768392`) als Beobachtungsmaterial.
Siege, Niederlagen und historische Patches werden gleichermassen aufgenommen.
Der Spieler ist eine Referenzquelle, nicht die alleinige Wahrheit.

Der erste Erfolgsbeweis ist ein vollstaendiger, von einem Menschen pruefbarer
Matchreport. Erst wenn die Reports kalibriert sind, duerfen daraus allgemeine
Brain-Regeln oder Builds entstehen.

## Pilotumfang

Der Pilot liefert in dieser Reihenfolge:

1. Match-History des Referenzspielers aus der Deadlock API katalogisieren und
   auf Mo & Krill filtern, ohne Ergebnis- oder Patchfilter.
2. Fuer ein Match mit verfuegbarer Demo die noetigen Ereignisse ueber die
   Demo-Query-API abrufen und als unveraenderte Rohquelle speichern.
3. Aus den Rohereignissen einen deterministischen Evidenzkontext bauen.
4. Einen strukturierten Vollreport erzeugen, validieren, speichern und
   menschenlesbar ausgeben.
5. Danach mindestens fuenf unterschiedliche Vollreports manuell kalibrieren.

Der erste Tracer endet nach einem geprueften Vollreport. Der automatische
Game-/Steam-Download-Fallback folgt vor dem Massen-Backfill, aber erst nachdem
die Reportqualitaet grundsaetzlich belegt ist. Ein eigener Source-2-Demo-Parser,
eine neue Datenbanktabelle, eine neue Queue und ein neuer Dienst sind nicht Teil
des ersten Tracers.

## Bestehende Bausteine

Der Pilot erweitert vorhandene Pfade:

- `dbrain-sources::deadlock_api` fuer HTTP-Abruf, Rohdateien,
  `source_documents`, `entity_snapshots` und `source_runs`.
- `deadlock_brain_core::http::HttpClient` fuer Timeout, Retry und Backoff.
- `dbrain-learn::player_decision_learning` fuer Matchkontext,
  Fireworks-Aufruf und `player_match_decision_notes`.
- Die bestehende Assets-Aufloesung fuer Hero-, Item- und Ability-Namen.

Neue Daten werden ueber neue `entity_type`-Werte in den vorhandenen Snapshots
gespeichert. Der Kontext-Hash und die Prompt-Version halten Reportlaeufe
reproduzierbar. Es wird keine weitere Abstraktionsschicht eingefuehrt.

## Datenfluss

### 1. Matchkatalog

Die Match-History wird fuer Account `281768392` abgerufen. Jeder zurueckgegebene
Mo-&-Krill-Eintrag wird mit Match-ID, Startzeit, Dauer, Ergebnis, Hero-ID und den
verfuegbaren Build-/Patchmetadaten gespeichert. Fehlende Patchdaten bleiben als
Datenluecke sichtbar und werden nicht aus dem Datum geraten.

Jedes Match des Spielers mit Mo & Krill bleibt im Katalog. Eine verfuegbare
Demo bestimmt, ob es analysiert werden kann. Abbruch oder unvollstaendige Daten
werden als Matchmerkmal ausgewiesen, nicht still herausgefiltert. Sieg und
Niederlage beeinflussen weder Auswahl noch Bewertung.

### 2. Demo-Evidenz

Die Deadlock API ist der erste Demo-Weg. Der Client reicht ein kleines,
versioniertes Standard-Query-Buendel ein und pollt den Job bis zu einem
terminalen Zustand. Es werden nur Ereignisse abgefragt, die der Vollreport
verwendet:

- Position, Lane, Rotationen und Aufenthaltsbereiche
- Souls, Farmquellen, Itemkaeufe und Skillreihenfolge
- Ability-Nutzung, Schaden, Heilung, Kills und Tode
- Objectives und relevante Teamfight-Ereignisse

Jede gespeicherte Ereigniszeile erhaelt eine stabile Evidenz-ID sowie
Match-ID, Tick beziehungsweise Zeitpunkt, Quelltabelle und Query-Version. Die
API-Antwort bleibt zusaetzlich als Rohquelle erhalten.

### 3. Evidenzkontext

Der Kontext normalisiert nur Namen, Zeitpunkte und bekannte IDs. Er bewertet
nichts. Aus ihm entstehen:

- Match- und Patchkopf
- Datenvollstaendigkeits- und Datenlueckenliste
- chronologische Ereignisfolge
- Kauf- und Skilltimeline
- Kampf-, Objective-, Farm- und Rotationsereignisse
- Evidenzregister fuer spaetere Reportaussagen

Statische Brain-Daten duerfen Mechaniken erklaeren, aber keine nicht
beobachtete Matchaktion belegen. Aggregierte Builddaten duerfen nicht als
Ereignis dieses Matches ausgegeben werden.

### 4. Vollreport

Das Modell liefert ein strukturiertes JSON-Ergebnis; eine deterministische
Ausgabe rendert daraus den lesbaren Report. Dadurch koennen Pflichtfelder,
Evidenzverweise und Korrekturquote geprueft werden.

Der Report enthaelt:

- Matchkopf: Match, Spieler, Hero, Patch/Game-Build, Ergebnis, Dauer, Rolle
  oder Lane sowie Datenluecken.
- Phasen: Lane, Uebergang, Midgame und Late Game, soweit das Match diese Phasen
  erreicht.
- Oekonomie und Build: Farmweg, Kauf- und Skillfolge, Itemzweck, Powerspikes
  und erkennbare Anpassungen.
- Kampf: Initiierung, Zielwahl, Ability-/Active-Nutzung, Rueckzug, Ergebnis und
  Folgewirkung.
- Makro: Wellen, Camps, Rotationen, Druck, Objectives und Trades auf der Map.
- Starke Entscheidungen, Fehler, Wendepunkte und offene Datenluecken.
- Nur als Hypothesen markierte, matchuebergreifend zu pruefende Muster.

Jede wichtige Entscheidung verwendet denselben Vertrag:

1. Zeitpunkt oder Tick
2. beobachtbarer Zustand
3. ausgefuehrte Aktion
4. unmittelbare und spaetere Wirkung
5. moegliche Absicht, klar als Interpretation markiert
6. Bewertung mit Begruendung
7. realistische Alternative
8. Sicherheit
9. mindestens ein gueltiger Evidenzverweis

Fakt, Interpretation und Bewertung sind getrennte Felder. Das Matchergebnis
ist Kontext, kein Qualitaetslabel. Unbelegte Aussagen oder Verweise auf nicht
existierende Evidenz-IDs machen den Report ungueltig.

## Review und Kalibrierung

Die ersten fuenf Vollreports werden komplett geprueft. Korrekturen verwenden
die vereinbarten Labels:

- `falsch`
- `unbelegt`
- `Kontext fehlt`
- `wichtige Entscheidung uebersehen`

`falsch`, `unbelegt` und `wichtige Entscheidung uebersehen` sind kritische
Fehler. `Kontext fehlt` ist kritisch, wenn dadurch Bewertung oder Alternative
kippt. Die Korrekturquote ist die Zahl korrigierter Entscheidungsaussagen
geteilt durch alle geprueften Entscheidungsaussagen.

Das Kalibrierungsgate ist bestanden, wenn nach mindestens fuenf Vollreports
drei aufeinanderfolgende Reports keinen kritischen Fehler und jeweils weniger
als zehn Prozent Korrekturen haben. Vor diesem Gate bleibt jeder Report
`calibration_pending`; es werden keine allgemeinen Regeln, Builds oder
Coaching-Aussagen in aktives Brain-Wissen uebernommen.

Jedes Review wird als versionierter `player_match_report_review`-Snapshot mit
Report-ID, geprueften Entscheidungsaussagen, Fehlerlabel, Kommentar und
optionaler Korrektur gespeichert. Der Kontext fuer den naechsten Report enthaelt
die bisherigen, bestaetigten Fehlerregeln. Das Gate wird ausschliesslich aus
diesen gespeicherten Reviews berechnet, nicht aus einer Modell-Selbsteinschaetzung.

## Patchgueltigkeit

Jeder Matchreport ist an den beobachteten Game-Build oder Patch gebunden.
Historische Reports bleiben gueltige Beobachtungen. Ein neuer Patch deaktiviert
nicht pauschal das gelernte Wissen: Spaetere Wissensregeln muessen ihre
Quellreports und Patchgueltigkeit tragen; nur von einem Patch betroffene Regeln
werden historisch oder erneut pruefpflichtig.

Die Wissenspromotion selbst ist nicht Teil des ersten Tracers. Damit kann ein
schlechter erster Report keine dauerhafte Wissensverschmutzung verursachen.

## Fehlerbehandlung

- Rate-Limits und temporaere HTTP-/Jobfehler nutzen den vorhandenen Retry- und
  Backoff-Pfad. Ein fehlgeschlagener Abruf wird nie als analysiert markiert.
- Ist eine Demo in der API nicht verfuegbar, bleibt das Match offen. Fuer den
  ersten Report wird das naechste verfuegbare Match gewaehlt; der spaetere
  Game-/Steam-Fallback darf denselben Evidenzvertrag fuellen.
- Bricht eine API-Schemaaenderung das Query-Buendel, endet der Lauf mit dem
  konkreten Tabellen-/Spaltenfehler. Es gibt keinen Teilreport aus still
  fehlenden Ereignisklassen.
- Fehlende optionale Daten werden im Reportkopf und am betroffenen Urteil
  ausgewiesen.
- Ungueltiges Modell-JSON, unbekannte Evidenzverweise oder fehlende
  Pflichtfelder werden nicht gespeichert beziehungsweise nicht als fertiger
  Report markiert.

## Teststrategie

Die Umsetzung folgt TDD und beginnt mit kleinen gespeicherten Fixtures:

1. Match-History-Parser nimmt Mo-&-Krill-Siege und -Niederlagen aus mehreren
   Patches auf und verwirft andere Heroes.
2. Demo-Job-Polling behandelt Erfolg, Rate-Limit, temporaeren Fehler und
   terminal nicht verfuegbare Demo korrekt.
3. Evidenznormalisierung erzeugt stabile IDs und bewahrt Tick, Quelle und
   Match-ID.
4. Reportvalidierung akzeptiert einen vollstaendigen Report und verwirft
   fehlende Pflichtfelder sowie erfundene Evidenzverweise.
5. Der Promptvertrag fordert alle Reportsektionen und die Trennung von Fakt,
   Interpretation und Bewertung.

Ein Live-Smoke-Test ruft danach ein echtes verfuegbares Match ab. Der Pilot ist
erst belegt, wenn der gespeicherte Rohbeleg, der validierte Report und die
menschenlesbare Ausgabe dieselbe Match-ID und Patchinformation zeigen.

## Erfolgskriterien

- Alle von der API gelieferten Mo-&-Krill-Matches des Referenzspielers sind
  katalogisiert, unabhaengig von Patch und Ergebnis.
- Mindestens eine echte Demo wurde automatisch abgefragt und als Rohquelle plus
  normalisierte Evidenz gespeichert.
- Der erste Vollreport erfuellt den beschriebenen Vertrag und jede Bewertung
  verweist auf echte Evidenz.
- Ein Mensch kann den kompletten Report mit den vier Korrekturlabels pruefen;
  das Review wird versioniert gespeichert und ist fuer den Folgereport lesbar.
- Vor bestandenem Kalibrierungsgate ist kein neues aktives Brain-Wissen
  entstanden.

## Spaetere Schritte

Nach einem brauchbaren ersten Report folgen nur die bereits begruendeten
Erweiterungen:

1. vier weitere unterschiedliche Reports fuer die Mindestkalibrierung
2. Game-/Steam-Demo-Fallback fuer API-Luecken
3. schrittweiser Backfill aller verfuegbaren Mo-&-Krill-Demos
4. patchversionierte Wissenspromotion nach bestandenem Gate
5. weitere Referenzspieler und danach weitere Heroes
