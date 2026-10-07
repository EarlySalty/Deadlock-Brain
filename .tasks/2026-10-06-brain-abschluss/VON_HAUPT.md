# Steuerung vom Haupt-Orchestrator (neueste oben)

## 07.10.2026, ca. 09:00: Release-Hold aufgehoben, Integration beginnt

Der Hold aus `A/RELEASEFENSTER.md` ist beendet. Grund: Er war eine Warteabsprache zwischen Sessions, das widerspricht der Arbeitsregel (keine Wartefenster auf fremde Builds; Serialisierung nur mechanisch über Worktree-Builds, flock im Deploy-Wrapper und Deploy vom aktuellen origin/main-SHA). Die parallele Session hat ihren Stand `f6f5cef6` um 08:52 deployt.

Ab jetzt gilt der normale Abschluss: Branch auf aktuellen origin/main bringen, Gate ALLOW, `git push origin HEAD:main` als Einzelschritt, Release aus eigenem Worktree vom origin/main-SHA über den regulären Deploy-Weg, Neustart, Live-Beweis. Wer als Zweiter kommt, deployt nach dem Merge einfach nochmal.

Reihenfolge wegen Abhängigkeiten: **E** (API-Spiegel) zuerst, dann erster Spiegel-Import, dann **F** (Publish ohne Matchgrenze, Warden mit `--publish`, `hero_build_id` melden), danach **G** sobald fertig. Integration von E und F übernimmt der Integrator I (`BRIEFING-I.md`). G mergt selbst nach eigenem ALLOW. A prüft danach die fünf G1-Fragen erneut.

## 07.10.2026, ca. 06:45: Nutzerentscheidungen zum Sheet-Modell (Paket G)

1. **Heldentabelle mit Wachstum ist Kern, auch für Builds.** Grund-DPS, Magazinschaden, HP, Wachstum je Boon, Werte bei beliebigen Boons und relatives Früh-zu-Spät-Wachstum je Held kommen aus der Rechenschicht und werden vom Build-Reasoner (F) über denselben Rechenkern genutzt, nicht doppelt gerechnet. Wachstumskurve über Boons als Kennzahl in `hero_compare` („ab wann überholt X Y“).
2. **Hidden Mechanics doppelt verfügbar:** als Abschnitt in den Steckbriefen der betroffenen Helden/Items und als eigene Wissenssammlung (Werkzeug `game_rules`): Kill-Bounty nach Spielzeit, Comeback, Urne, Midboss, Resist-Stacking, Ressourcenmechaniken. Werte aus versionsgebundenen API-/Spielkonfigdaten; wo keine Quelle existiert, als Lücke markieren, nichts erfinden.
3. **Kaputte Sheet-Stellen reparieren statt weglassen:** Absicht der fünf DNS-Abfrageblöcke und der drei `#REF!`-Formeln aus Umgebung, Beschriftung und Nachbarformeln rekonstruieren, in `G/SHEET-MODELL.md` dokumentieren und die gemeinte Rechnung korrekt in Rust abbilden.
4. **Ränge und Meta aus der Deadlock-API:** Werteränge (Platz unter allen aktiven Helden) aus dem Spiegel; dazu Meta-Ränge (Win-/Pickrate je Held, nach Rang filterbar) aus `/v1/analytics/hero-stats` und `item-stats`. Nur Aggregate je Patch/Tag, keine Einzelmatches speichern. Bestehenden Baustein `dbrain-sources::analytics_runtime` wiederverwenden.
5. **Grafiken/Webseiten: entfällt für G.** Der Nutzer lässt das bereits separat bauen (Nachtrag 06:55). G baut nichts dazu und trägt auch nichts in die Roadmap ein; die Werkzeugausgaben bleiben strukturiert (Zahlenreihen), damit eine Grafikschicht sie später nutzen kann.

## 07.10.2026, ca. 05:40: Wache, ein Leser für alle (E, F, G)

1. **F Spiegelmitgliedschaft:** Der Publish-Guard belegt aktuelle Spielwerte über Es gemeinsamen Leser `brain_storage::asset_mirror::{latest_mirrored_client_version, load_mirrored_assets}` (Originaldokumente je `client_version`, nur vollständige Runs), nicht über `entity_snapshots`. Kein eigener Wertespiegel, keine Snapshot-Erweiterung. Solange Es Branch nicht auf main ist, baut F auf Es Featurestand auf oder lässt die Prüfung als klar markierte Abhängigkeit offen; Integration nach Hold-Ende in der Reihenfolge E, dann F, dann G.
2. **G** liest ausschließlich über denselben Leser; Es Vertrag (AN_HAUPT-E im E-Worktree) ist verbindlich. Patchgeschichte nur mit belegter Zuordnung, Clientversion ist kein Balancepatch.
3. **Akten im Worktree:** Wer wegen Isolationsschutz nicht in die gemeinsame Akte schreiben kann, berichtet im eigenen Worktree unter `.tasks/2026-10-06-brain-abschluss/AN_HAUPT-<X>.md`. Der Haupt-Orchestrator liest dort.
4. **A:** E3g-Fixer erst nach G1-Abnahme ist richtig. Den Invite-Status beim Fixer-Start gemäß A/G-Abgrenzung als eine begrenzte Statusoperation im bestehenden Antwortweg planen.

## 07.10.2026, ca. 05:25: Architektur Brain v2 freigegeben (Paket G)

Nutzer hat entschieden: Spielwissen läuft künftig über strukturierte Entitäten, eine deterministische Rust-Rechenschicht (alles, was das Community-Sheet rechnet, plus Ränge gegenüber allen Helden) und Werkzeuge im Antwortdienst statt Dokumentpakete mit Textsuche. Details `BRIEFING-G.md`, Thread G `a867ef50`.

- A: laufende Steckbrief-Freischaltung als Übergang fertigstellen, danach keine neue Arbeit am Dokumentweg für Spielwissen. Discord-, Twitch- und Invite-Anbindung bleiben bei A.
- E: Der API-Spiegel ist die Datenbasis für G. Tabellenschnitt früh in `AN_HAUPT-E.md` festhalten, damit G darauf aufsetzt.
- F: Publish-Regel wie beauftragt; der Build-Skill wird später ein Werkzeug in G.
- Alle: kein Code auf Vorrat, nichts, was G ersetzt. Abbau nach belegtem Gleichstand.

## 07.10.2026, ca. 05:15: Nutzerantworten Modell und DSN

1. Antwortmodell `gpt-6-luna` über den Codex-Abo-Proxy ist vom Nutzer bestätigt.
2. DSN-Teilausschnitt: Rotation nur, wenn sie über den bestehenden Infisical-Weg automatisch und ohne Ausfall geht (alle Verbraucher lesen den neuen Wert ohne Handarbeit). Sonst nicht rotieren, nur den Fundort bereinigen.

## 07.10.2026, ca. 05:10: So schlank wie möglich zusammenbauen (gilt für A, D, E, F)

Nutzer: „so smart wie möglich zusammenbauen“. Daraus für alle:

1. Eine Datenbasis: Spielwerte kommen aus dem lokalen API-Spiegel (E). Steckbriefe, Patch-Story (A) und Reasoner (F) lesen genau diese Tabellen, keine eigenen Kopien, Parser oder Zwischenformate.
2. Ein Antwortweg: Discord und Twitch fragen `/v1/answer`; Builds sind ein Skill hinter demselben Weg, kein eigener Bot-Befehl mit eigener Logik.
3. Weniger statt mehr: Jeder Worker nennt in seinem Bericht, was er dadurch löschen konnte oder was als Abbau ansteht (ersetzte Parser, Matchablagen, Sondergates, tote Timer). Neuer Code nur, wo kein Baustein existiert.
4. Schnittstellen vor dem Bauen klären: Wer eine Tabelle oder Datei eines anderen Pakets braucht, meldet das in seiner AN_HAUPT-Datei, bevor er sie ändert.

## 07.10.2026, ca. 05:05: Nutzerentscheidung keine Matches speichern, keine 100er-Grenze

1. **Keine Matchdaten speichern.** Nutzer: „Datenmüll, der uns nix bringt.“ Punkt 4 der E-Freigabe (Nachholweg für Einzelmatches) und Punkt 3 aus 04:55 entfallen. Kein Import und keine Ablage von Einzelmatches. Braucht der Reasoner ein Populationssignal, fragt er die Analytics der API zur Laufzeit ab und speichert höchstens das kleine Ergebnis mit Zeitbezug, keine Rohmatches. Bestehende Matchablagen erfasst E in der Abbauliste.
2. **Die Publish-Grenze von 100 Nach-Patch-Matches fällt.** Nutzer: Wir wollen gerade Builds bauen, die anders sind; dafür gibt es zu dem Zeitpunkt vielleicht keine Spiele. Der Build entsteht aus Mechanik und aktuellen Spielwerten. Veröffentlicht wird, wenn der Build auf den Werten des aktiven Patches beruht und gültig ist; Populationsdaten sind nur ein Nebensignal und nie Voraussetzung. Umsetzung durch neuen Worker F (eigener Thread), nicht durch A, D oder E.

## 07.10.2026, ca. 04:55: Nutzerentscheidung API vor Eigenbau

Nutzer: Alles, was die Deadlock-API liefert, nehmen wir von dort statt es selbst zu bauen, weil das günstiger und langfristig wartbarer ist.

1. E: Spielwerte (Helden, Items, Fähigkeiten, Lokalisierung) 1:1 je `client_version` in die bestehenden Brain-Tabellen spiegeln, Abgleich bei neuer Clientversion plus täglich. Antworten lesen lokal, nie live pro Frage gegen die API. Grund: Werte ändern sich nur mit Patches, lokale Abfrage ist schneller, unabhängig von Ausfällen und Limits, und ältere Versionen fehlen teils bei der API (Warden 5044 404), die Patch-Story braucht aber die Historie.
2. E: Eigene Parser oder Scraper, die dieselben Spielwerte selbst aus Spieldateien, Wiki oder Seiten ziehen, nach dem API-Spiegel als ersetzt markieren und in `AN_HAUPT-E.md` als Abbauliste führen (Pfad, was ersetzt ihn, Abbau erst nach belegtem Gleichstand). Der offizielle Patchnotes-Parser bleibt, weil `/v2/patches` große Updates nur als Vorschau liefert.
3. Matches nicht komplett spiegeln (Limit 30/min, etwa 0,1 MB je Match), sondern gezielt je Held und aktivem Patch wie freigegeben.

## 07.10.2026, ca. 04:40: Paket E Umsetzung freigegeben

Freigabe für die Empfehlung aus `AN_HAUPT-E.md`, im eigenen Worktree `~/.worktrees/brain-e-deadlock-api` von aktuellem origin/main (bfda408c), Branch `feat/brain-deadlock-api-daten`:

1. Vorhandenen Assets-Ingest (`dbrain-sources/src/assets_api.rs`) versionsgebunden auf `api.deadlock-api.com/v1/assets` mit `client_version` und deutscher Lokalisierung, in die bestehenden Tabellen.
2. `/v2/patches` in den bestehenden Patch-Sync einhängen; Parser, `patch_events` und `patch_changes` behalten, offizielle Linkauflösung für große Updates bleibt.
3. Analytics- und Builddaten-Abfragen mit expliziter Matchzeit-Untergrenze ab dem aktiven Patch; das `|| echo`, das Timerfehler verdeckt, entfernen, damit Fehler sichtbar werden.
4. Gezielter Nachholweg für echte Einzelmatches ab aktivem Patch über den bestehenden Populationimport; 100er-Grenze und Publishbedingungen bleiben unverändert.

Tests, fmt, Clippy, eigener `gate_hook.py --review`. Main-Push erst, wenn der Release-Hold aufgehoben ist (A/RELEASEFENSTER.md); bis dahin Featurecommit auf origin sichern und in `AN_HAUPT-E.md` melden. Build, Install und Tick macht `live_strecke`. A bleibt Eigentümer von Steckbrief-/Patch-Story-Veröffentlichung und Korpusaktivierung; Dateiüberschneidung mit A vorher in `AN_HAUPT-E.md` melden.

## 07.10.2026, ca. 04:10: Fokus Spielwissen im aktiven Korpus

Hauptkoordinator 3fcd8f71 ist erreichbar über diese Datei und den Thread, nicht per Sessionnachricht. G1 zeigt den Kern des Problems: 4 von 5 Abnahmefragen enden in insufficient_evidence, weil im aktiven Korpus keine Spielprofil-, Patchnotes- oder Patch-Story-Quelle aktiv ist. Nur das Steam-Wartungswissen antwortet.

1. A: Nach der Recovery durch live_strecke ist die einzige Priorität G1-Lücke 1, also Steckbriefe des aktuellen Patches und Patch-Story über den bestehenden Veröffentlichungsweg in den aktiven Korpus bringen. Die Aktivierung übernimmt live_strecke. Danach die fünf Abnahmefragen erneut stellen; Ziel ist answered mit Beleg bei Haze, Mystic Burst, Bullet Dance und Patch-Story.
2. A: G1-Lücke 2 (boolesche Twitch-Profilfreigabe) und Lücke 3 (Request-ID bis zum Discord-Sender, E4) direkt danach. Der ENV-Testfix R2 läuft nebenher und blockiert nichts.
3. A: Den gemeldeten Teilausschnitt von TWITCH_ANALYTICS_DSN in G1-Tooloutput als Secret-Befund mit Fundort (Datei oder Log, ohne Wert) in AN_HAUPT-A.md nennen, damit über eine Rotation entschieden werden kann.
4. Releasefenster, Tests und Gates bleiben, aber Belegprosa kurz halten. Gemessen wird am Ergebnis: echte Antworten in Discord und Twitch.

## 07.10.2026, ca. 04:00: Release-Hold bestätigt, kein Fremd-Gesamtmerge

1. Der gemeinsame Release-Hold mit `live_strecke` als einzigem Build-, Install- und Tickeigentümer gilt auch für A und D. Main nur nach gemeinsamer Freigabe fortschreiben.
2. D: Den vom Stop-Hook verlangten Gesamtmerge von `feat/brain-rust-cutover-20260919` nicht ausführen. Fremder Branch auf toter Basis, bleibt unberührt. Der Hook-Fund gilt als erledigt mit dieser Entscheidung.
3. Nach der Schlussabnahme bleibt das Ziel unverändert: Grundding live an echten Fragen in Discord und Twitch belegen, danach Builds als Skill.

## 06.10.2026, ca. 23:10: Nutzerentscheidung „ein Brain“

Wörtlich sinngemäß vom Nutzer: Es soll ein gemeinsames Deadlock Brain sein, mit verschiedenen Skills und Wissen über alles, nicht zehn verschiedene. Antworten an Nutzer (auch zum Game-Invite) sollen über das Brain kommen, nicht als eigene Bot-Logik.

Folgen:

1. **Grundsatz für alle Pakete:** Bots (Discord, Twitch, Steam) führen nur Mechanik aus (Invite verschicken, Status speichern, Nachricht zustellen). Was ein Nutzer als Antwort liest, formuliert das Brain über seinen gemeinsamen Antwortweg. Daten wie der Invite-Status sind ein Skill bzw. eine Wissensquelle des Brains, kein eigener Textbaustein im Bot.
2. **B-Fix1:** Die beiden Mechanikfehler (Rückblick verschluckt Live-Bitten, Frische direkt vor dem Fremdaufruf prüfen) weiter beheben. Die eigene öffentliche Statusantwort der Invite-Lounge (`invite_lounge.rs`, Text „schon eingeladen“ usw.) nicht reparieren, sondern entfernen bzw. abschalten; der Bot speichert nur den echten Status. Keinen neuen eigenen Antworttext im Bot bauen.
3. **A:** Neue Priorität 1 neben den Steckbriefen: Bestandsaufnahme „wer antwortet heute Nutzern außerhalb des Brains“ (Concierge, Serverguide, Invite-Lounge, Twitch-Antworten, Patchnotes-Bot, Pitches, eigene LLM-Aufrufe oder feste Antworttexte in dl-bot und tb-bot). Ergebnis als `A/EIN-BRAIN.md`: Pfad, was er tut, bereits über Brain ja/nein, Umbauweg auf den gemeinsamen Antwortweg. Dann den Invite-Status als Brain-Skill anbinden (Lesen aus `steam.beta_invite_audit` bzw. vorhandener Quelle, Antwort zeitnah über den gemeinsamen Discord-Antwortweg) und die übrigen Pfade nach Nutzen umziehen. Vorhandene Bausteine nutzen (gemeinsamer Brain-Consumer, `.tasks/2026-09-19-unified-community-game-ai` falls einschlägig), keinen zweiten Antwortweg bauen. Bevor ein sichtbares Bot-Verhalten wegfällt, muss der Brain-Ersatz live sein.

## 06.10.2026, ca. 23:20: Klarstellung zu Punkt 3

Die verspätete öffentliche Invite-Lounge-Antwort ist der gemeldete Fehler und darf sofort entfallen. Die Regel „erst Brain-Ersatz live“ gilt nur für gewollte, funktionierende Bot-Antworten.

## 06.10.2026, ca. 23:25: Datenschutz Invite-Skill

Der Skill liefert nur der fragenden Person ihren eigenen Status (Enum plus Zeitpunkt), keine Steam-IDs, Namen oder Daten Dritter. In dieser Minimalform über den bestehenden Antwortweg und Provider zulässig.

## 07.10.2026, ca. 00:30: Betreiberwissen aus Discord

Nutzer zeigt Beispiel (#allgemein, 07.10. 00:11/00:21, Bild `A/beispiel-betreiberwissen-steam-wartung.png`): Frage „sind die steam server dead?“, Antwort von Nani/EarlySalty: „Di auf Mi ist immer Patch Day, da starten die die Server neu usw, 10 20 min idR“. Solches Wissen soll ins Brain.

1. Diesen Fakt sofort über den bestehenden kuratierten Wissensweg aufnehmen (Thema Steam-Wartung: nachts Dienstag auf Mittwoch, Neustart, meist 10 bis 20 Minuten Ausfall; Quelle Betreiber).
2. Prüfen, ob es schon eine Quelle gibt, die Antworten des Betreibers (Discord-ID von Nani/EarlySalty) auf Fragen in Discord als Wissen erntet (z. B. Community-Brücke D2). Wenn ja anschließen, wenn nein als Skill „Betreiberwissen“ über den bestehenden Ingest bauen: Frage plus Antwort des Betreibers als Kandidat, zunächst nur Nani, keine Nachrichten anderer Nutzer als Fakt, Rohtexte intern, Antwort im Brain in eigenen Worten.

## 07.10.2026, ca. 01:45: Grundding zuerst, Builds als Skill, Ernte freigegeben

Nutzer: Gelöschte gemergte und verworfene Branches sind in Ordnung. Die 44 erhaltenen Stände sollen umgesetzt werden, „das Grundding muss bald funktionieren“. Builds bauen soll ebenfalls ein Brain-Skill sein.

1. **Definition „Grundding“ (A, oberste Priorität vor allem anderen):** Eine Frage in Discord (DM, Erwähnung, Hilfekanal) und im Twitch-Chat bekommt zeitnah eine Antwort aus dem einen Brain, mit aktuellem Spielwissen (Steckbriefe aktueller Patch plus Patch-Story) und Serverwissen. Das muss live und an echten Fragen belegt sein. Alles, was nicht darauf einzahlt, wartet.
2. **Builds als Skill (A, direkt nach dem Grundding):** Frage wie „Build für Warden“ in Discord oder Twitch führt über den Build-Reasoner zu einem veröffentlichten Build mit `hero_build_id`, die Antwort nennt die Build-ID. Publish ist derzeit wegen 47 statt 100 Nach-Patch-Matches gesperrt: Qualitätsgrenze nicht senken, sondern prüfen, ob mehr Matchdaten aus den vorhandenen Quellen (Deadlock-API) geholt werden können, und ehrlich antworten, wenn die Datenlage noch nicht reicht.
3. **D, Ernte:** Die Portierliste ist vorab freigegeben für alles mit Nutzen „hoch“ oder „mittel“, das auf Punkt 1 oder 2 einzahlt (z. B. Spielstilplanung, Rechtefix, Sicherheitsregressionen, Caption-/Patch-Review). Reihenfolge nach Nutzen für Punkt 1, dann Punkt 2. Bei Überschneidung mit A hat A Vortritt; D stimmt sich über `AN_HAUPT-D.md` ab und baut nichts doppelt. „Niedrig“ und „keiner“ bleiben archiviert.
