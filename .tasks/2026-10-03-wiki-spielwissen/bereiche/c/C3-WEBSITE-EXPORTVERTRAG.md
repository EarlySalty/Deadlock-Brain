status: Vertragsentscheidung, nicht implementiert
Datum: 2026-10-03

# C3: öffentliche Exportbindung der Website

Entscheidung: Den bestehenden atomischen Dateiexport weiterverwenden. Aus einem unveränderlichen CorpusRelease werden ausdrücklich öffentlich freigegebene Dokumentrevisionen fachlich projiziert. Kein zweiter Writer, keine parallele Datenhaltung und keine öffentliche Freigabe durch einen internen Pin.

Der WEBSITE-READER-ABGLEICH.md wurde vollständig gelesen. Anschließend wurden die unten genannten Quellen lesend geprüft. Die Reader-/Writer-Arbeitsdateien entsprechen weiterhin den dort dokumentierten SHA256. HEAD f7a03f9 ersetzt keinen Commitnachweis der uncommittierten C3-Integration.

## 1. Erlaubte Menge und fehlende Strecke

Für den neuen C3-Bestand ist in den vorliegenden Übergaben keine konkrete öffentlich freigegebene Dokumentrevision nachgewiesen. Das ist kein Urteil über sämtliche denkbaren Quellenrechte und keine Freigabe eines leeren Snapshots. A behält `unverified` und `redistribution_allowed=false`; diese Dokumente sind aus dem öffentlichen Export ausgeschlossen. MIT am Community-Repository erlaubt nicht automatisch enthaltene Valve-Assets oder Steam-Depotdateien. Originaltexte und Grafiken benötigen eigene Belege.

Bestehend: `knowledge_import.rs:151` schneidet Quellenfreigabe mit dokumentbezogener Lizenz; `knowledge_contract.rs:39` verweigert Veröffentlichung bei `unverified` oder fehlender Weitergabeerlaubnis. `knowledge_projection.rs:153` prüft diese Grenze erneut. Die ImportPolicy gewährt auf Quellenebene, nicht als fertige öffentliche Dokumentauswahl. Ein Quellenname im Websiteparser ist ebenfalls kein Rechtebeleg.

Geplant: Eine ausdrücklich belegte öffentliche Teilmenge aus dem vorhandenen Release bestimmen. Je exportiertem Dokument sind Freigabe, Lizenzbedingungen und Attribution an genau die gepinnte Store-Revision sowie die getrennt erhaltene Originalrevision gebunden. Nicht aus späteren Köpfen lesen. `CorpusRelease.source_revisions` enthält Store-Revisionsnummern (`brain-contracts/src/lib.rs:323`); Originalrevisionen sind damit nicht austauschbar.

Die technische Brücke fehlt weiterhin: Der CLI veröffentlicht den internen Release und prüft seinen Index (`brain-knowledge-import.rs:331`), während `rebuild_game_wiki` aus `brain.entity_snapshots` liest (`game_wiki.rs:58`, `:698`, `:741`). Keine implementierte öffentliche Release-zu-Dateiexport-Strecke wird behauptet. Die spätere Anbindung bleibt im bestehenden C3-Writerpfad, ohne Umweg über einen zweiten Bestand.

## 2. Gemeinsame fachliche Positivprojektion

Eine öffentliche Projektion entsteht vor Beschreibung, Tabelle, verschachtelten Werten, JSON-Ausklapper und Suche. Zulässige fachliche Gruppen sind Titel und öffentlicher Artikelname, Spielkategorie, Held-/Item-/Fähigkeitsnamen, belegte Spielwerte mit Einheiten und fachlichen Bedingungen, freigegebene Beschreibungen, freigegebener Originalquellenlink, erforderliche öffentliche Attribution sowie belegter Quellen-/Versionsstand. Unbekannte Felder werden nicht automatisch übernommen; pro Artikelart braucht es eine ausdrückliche Feldzuordnung. Zahlen und deren vorhandene Typen bleiben unverändert.

Hashes, lokale Pfade, interne IDs, Rohbelege und interne Provenienz gehören nicht in HTML, JSON, Suchtexte oder Suchtreffer. Ein öffentlicher Artikelname ist kein durchgereichter interner Dokumentschlüssel. Quellenlinks müssen ausdrücklich öffentlich freigegeben sein; HTTP(S) allein reicht dafür nicht. Originaltext und Bilder werden nicht aus der Zulassung strukturierter Spielwerte abgeleitet.

Die heutige rekursive Ausschlussliste ist nur ein Schutzansatz. `wiki.rs:713` rendert Rohdaten in Tabellen, während `:765` erst den JSON-Ausklapper bereinigt. Der Website-Owner korrigiert seine Projektion und verwendet dasselbe Ergebnis in sämtlichen öffentlichen Ausgabewegen. Er ändert keine C3-Import-, Writer-, Persistenz- oder Pin-Dateien.

## 3. Generation und Statusschema 1

Vorhanden: `wiki_refresh.rs:176` schreibt `schema_version=1`, `state`, `generation`, `rendered_at`, `provenance`, `entries`, `counts`, `source_update_performed` und den internen `snapshot`-Pfad. Der Dateidigest entsteht vor `status.json` (`:248`). Vollständige Generation und Status werden vor dem atomischen `current`-Wechsel erstellt. Der Reader kanonisiert dieses Ziel einmal und cached nach Zielpfad (`wiki.rs:207`). Er prüft weder Digest noch öffentliche Releasebindung selbst.

Geplante Ergänzung, noch kein bestehendes Feld: ein versionierter Exportbindungsnachweis innerhalb derselben Generation. Er bindet Release, ausgewählte Store-/Originalrevisionen, dokumentbezogene öffentliche Freigaben, Projektionsversion und explizite Shardabdeckung. Dieser Nachweis wird vor dem bestehenden Dateidigest geschrieben und damit mit erfasst. Statusschema 1 bleibt erhalten; Status verweist intern auf diesen Nachweis. Interne Release-IDs, Hashes, Rechtebelege, Pfade und vollständige Provenienz werden nicht über eine öffentliche Statusausgabe durchgereicht.

`entries` und `counts` zählen ausschließlich die exportierte öffentliche Teilmenge, nicht den gesamten internen Corpus. Writer und Reader verwenden dieselbe deklarierte Shardmenge. Fehlende positive Shards, unbekannte Abdeckung oder Zählabweichungen sind Fehler. Nullmengen sind ausdrücklich beschrieben, nicht aus fehlenden Countfeldern geraten. Generationen bleiben unverändert und für laufende Leser verfügbar. Datenstand ist Quellenstand, nicht Renderzeit; historische oder unvollständige A/B-Abdeckung wird nicht als aktuell vollständig ausgegeben.

## 4. Fehler- und Leerzustände

Der gewünschte Vertrag ist bestätigt, ohne fachliche Abweichung:

| Zustand | HTTP und Ausgabe |
| --- | --- |
| Vollständig validierter, ausdrücklich öffentlich freigegebener Snapshot mit null Artikeln | 200, ehrliche leere Index-/Suchseite |
| Fehlender, defekter oder nicht öffentlich freigegebener Snapshot | 503, allgemeine Meldung ohne interne Details |
| Gültiger Snapshot, unbekannter Artikel | 404 |
| Ungültige Suchparameter | 400 |
| Gültige Suche ohne Treffer | 200, ehrliche Meldung ohne Treffer |

Die 200-Leermenge ist eine geplante Verhaltensänderung auf beiden Seiten: Der Writer lehnt derzeit null Einträge ab (`wiki_refresh.rs:249`), der Reader ebenso (`wiki.rs:243`). Ein Website-only-Fix reicht nicht. Fehlende öffentliche Freigabe darf nicht als erfolgreich freigegebene Leermenge erscheinen. Ein fehlgeschlagener neuer Export kann den alten, weiterhin gültigen öffentlichen Snapshot erhalten; dessen Quellenstand bleibt sichtbar.

## 5. Eigentum und aktueller Abschluss

Website: eigene Shards, passende variable JSON-Fences (`game_wiki.rs:1302`, `wiki.rs:185`) und gemeinsame öffentliche Render-/Suchprojektion. C3: spätere Release-/Revisions-/Freigabebindung im vorhandenen Export und Writer-Leerzustand. Keine geschützte Quelle geändert, kein Pin aktiviert, kein Snapshot veröffentlicht, kein Compiler oder neuer Thread gestartet. Es handelt sich um einen Quellen- und Vertragsbefund, nicht um Laufzeitabnahme.

Punkt 55 bleibt wirksam. Launcher-Root ist laut Auftraggeber jetzt 77552; 17073 ist überholt. Tatsächlicher Launcherabschluss und anschließender Root-vermittelter Relay-/Statistikabgleich stehen weiter vor neuer Bauarbeit. Die frühere laufende C3-Prüfung ist tatsächlich mit Exit 101 beendet, nicht mehr aktiv.

Quellenstand: Website `wiki.rs` SHA256 `b0bd927cb1c268c5d9803faf29a33c49e3492ba36219ae370bcf2d0edb1f1734`; Brain `wiki_refresh.rs` SHA256 `85d7e6a58c3594a34cfc5bd5dc6981b6fba40502eb935f8135b1c54744911449`. Ergänzend Bereich C `IMPORTRECHTE-C3.md:14` und vollständiger WEBSITE-READER-ABGLEICH.md.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 3 belegt | Senke: eigener Bereich C, C3-WEBSITE-EXPORTVERTRAG.md

Textprüfung per Sichtprüfung. Automatischer Textcheck durch Werkzeugfreigabe verweigert, nicht ersetzt. Keine Compiler-, Handler- oder Liveprüfung ausgeführt.
