# B: eigene Architektur- und Betriebsnachweise

Messzeitraum: 07.10.2026, ab 07:47 CEST. Keine Runtimeoperation und keine neue Modellfrage.

## Revisionsgrenze

Die unten genannten `origin/main`-Stände wurden zunächst als lokale Remote-Refs gelesen und anschließend durch sechs rein lesende `git ls-remote origin refs/heads/main` mit dem Gitserver verglichen. Alle sechs stimmen zum Prüfzeitpunkt überein. Kein `fetch`, keine Gitrefmutation. Kanonische HEADs bleiben davon getrennt.

| Quelle | Kanonischer HEAD | Lokales origin/main |
| --- | --- | --- |
| Deadlock-Brain | `2734c2da4e814ff79953e8e825275b0216a6af16` | `9711cb630aebacfe959ed4783595b071f479be36` |
| Deadlock-Bots | `dbda52b81cf8e68ea2aa7351a0e7ef2df5c81dbc` | `e18f522226f8e2dec5a1c03fe97c2aba3200c8d1` |
| Deadlock-2nd-Brain | `28b4c078c7ccde8d0708f88f5bf1cc51b01a9eff` | identisch |
| Deadlock-Docs | `400231af7e8196e145111e4150c2e24c2ab79fb5` | `6fa4ca3758d6f259cc0ca128e5350834fe1b4cfe` |
| Website | `dbd2b347014dc733d926c6998d1c03681d61ab15` | identisch |
| Deadlock-Twitch-Bot | `d828481624d53408e0c0a4c3ed1a8e4a6d421c40` | `e0b0dbaf662d7680c4ceaa210bf15f1443693cd8` |

G-Worktree: `/home/nathanael/.worktrees/brain-g-v2-20261007`, beim eigenen Code-Nachlesen HEAD `3d6890c0ef69c910173563f140e17504ea8612a4`. Er wird parallel weiterbearbeitet. Aussagen beziehen sich auf die jeweils benannte Messung, nicht auf einen eingefrorenen Endstand.

## Aktuelle G-Grenze

`G/PLAN.md:17` wurde während des Audits korrigiert: Grafiken und Webseiten baut der Nutzer separat; G baut nichts dazu und trägt nichts in die Roadmap ein. Die Werkzeugausgaben bleiben strukturierte Zahlenreihen. Die ältere Fassung stellte dies als spätere niedrige Priorität dar. Für den Audit gilt die jüngere Abgrenzung. Der Rechercheauftrag priorisiert die Planung eines getrennten Grafik-/Webseitenpakets, nicht eine Erweiterung des laufenden G-Schreibbereichs.

Gesamter Planpfad: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/PLAN.md`.

### Vorhandene WIP-Verträge

Direkt im G-Code nachgelesen:

- `rust/crates/brain-contracts/src/tools.rs:11`: Werkzeugnamen für Entitätssuche, Profil, Heldenvergleich, Schadensrechnung, Patchhistorie und Builds. Der erweiterte Vertrag enthält `game_rules`.
- `tools.rs:183`: `ToolScenario` mit Fortschritt, Level, Fähigkeitspunkten, Spirit und Item-IDs.
- `tools.rs:210`: `ToolAnalyticsSelection` mit Badge- und Zeitgrenzen. Diese filtern den durchschnittlichen Team-Badge, nicht den individuellen Rang einer Person; siehe `G/G0-0645-VERTRAG.md:51`.
- `tools.rs:256` und `:272`: Boonbereich und `HeroCompareRequest` mit Szenario und Vergleichsmetriken.
- `tools.rs:344`: geschlossener `GameRulesRequest`.
- `tools.rs:636`: Werkzeugergebnis mit Call-ID, Ergebnis, Beleg-IDs und Fehlerstatus. `result` ist ein JSON-Wert, kein schon fertiger Rendervertrag.
- `tools.rs:774`: serverseitiger `PinnedGameContext` mit Clientversion, Sprache und Mechanikrevision.

Der Pfad `rust/crates/brain-contracts/src/tools.rs` existiert im gemessenen Brain-`origin/main` `9711cb63` noch nicht. Die Verträge sind WIP, kein heutiger Main-/Livebeweis. `G/G0-0645-VERTRAG.md:53` hält zudem fest, dass der bestehende Analytics-Fachport Rangfilter und Item-Meta noch nicht liefert. Der Vertrag allein beweist keine spielregel- oder filterrichtige Antwort.

### Anschluss statt Doppelbau

`G/PLAN.md:43`, `:58` und `:87` binden Rechenauftrag und Ausgabe an Spiegelversion, Herkunft und freigegebene Belege. Ein Grafikpaket soll diese fertigen Ergebnisse als Ansichten verarbeiten. Es soll keine Formeln, Wertequelle, Sheetsync, Matchablage oder eigenes Modell zum Nachrechnen anlegen.

`G/PLAN.md:75` und `:189` halten Actor, Scopes und URLs aus Modellargumenten heraus. `:194` bindet Cache und Single-Flight an Release, Rechtekontext, Anfrage-Pin, Szenario, Mechanik-/Modellrevision und vollständige Quellenabhängigkeiten. Vor Cachetreffern werden Freigaben erneut geprüft. `:195` verbietet private Mitgliederdaten und Discord-Rohtexte im externen Modellkontext.

Für Grafikseiten muss ein eigener enger Rendervertrag aus diesen geprüften Ergebnissen folgen. Der sichtbare Titel, Beschreibung, Zahlen und Belege dürfen Daten sein; frei ausführbares HTML, JavaScript, CSS und URLs aus einer Modellantwort sollten keine Renderanweisung werden. Das ist eine Empfehlung dieses Audits, kein bereits vorhandenes Feature.

## Gemeinsamer Brain-Anschluss des Serverguides

Direkt aus Brain-`origin/main` `9711cb63` nachgelesen: `rust/crates/brain-contracts/src/lib.rs:75` definiert `Query` mit Gesprächs-ID, Text, optionalem typisiertem Domainauftrag, angefragten Scopes und Antwortprofil. `:155` definiert den serverseitig berechtigten Kontext mit Gesprächsbindung, Principal, Release und Budget. `:170` trennt öffentliche, interne, private und nur für die einzelne Anfrage gültige Quellen; `:57` benennt das Verbot, anfragegebundene Inhalte als Wissensquelle zu speichern. `public_api.rs` liefert bereits beleggebundene öffentliche Retrieval- und Antwortverträge.

Empfehlung: öffentliches Serverwissen und Spielwissen hinter dem gemeinsamen Antwortweg wiederverwenden. Kontaktzeitpunkte, Patenzustand, Nein/Stopp und personenbezogene Profile bleiben Zustandsaufgaben des Bots mit lokaler Persistenz. Ein zentraler Antwortdienst darf daraus nicht automatisch eine öffentliche Wissensquelle oder ein externes Personenprofil machen. Ob ein lokaler personabezogener Generierungsweg zentral angebunden werden soll, braucht den jeweiligen fachlichen Kontextvertrag und Datenschutzbeleg; Paket A inventarisiert die KI-Wege. Eine bloße Gesprächs-ID im allgemeinen Query ist kein belegter vollständiger Persona-/Kontaktserienanschluss.

## A/B-Überschneidung und Release-Halt

- Brain-Abschluss-A: zentrale Antwortwege, Profile, Steam-/Serverwissen, Discord-/Twitch-Consumer, Invite-Skill und Website-Port. Quellen: `.tasks/2026-10-06-brain-abschluss/BRIEFING-A.md`, `AN_HAUPT-A.md`.
- Brain-Abschluss-B: verspätete Game-Invite-Antworten und Versandmechanik. Quelle: `BRIEFING-B.md:1` sowie `AN_HAUPT-B.md`. Das ist nicht Audit B und kein Serverguide-Patenauftrag.
- `A/RELEASEFENSTER.md:7`, `:25` und `:49`: exklusiver Liveverantwortlicher, keine parallelen Main-Pushes, Builds, Installationen, Neustarts oder Ticks. Quellen-/Featurefreigaben sind keine Runtimefreigabe für diesen Audit. Ältere SHA- und Runtimeabschnitte dieser Datei sind historische Meldungen.

## Sichere aktuelle Betriebsbeobachtung

Beobachtet um `2026-10-07T05:52:52.491Z`, entsprechend 07:52:52 CEST. Ausschließlich `systemctl show` für Id, MainPID, ActiveState und SubState, außerdem `/proc/<pid>/exe` und ein vorhandener Release-Symlink.

| Dienst | Beobachtung | Aussagegrenze |
| --- | --- | --- |
| `brain-serve.service`, User-Service | active/running, PID `4062119`; Executable unter `/opt/deadlock-brain/maintenance-releases/bfda408cb988722ddceadb56bca5b72e12d12731/brain-serve` | Prozess läuft aus diesem Pfad. Keine neue Antwortprobe, kein Artefakthash geprüft. |
| Brain-Releasezeiger | `/opt/deadlock-brain/current` zeigt `/opt/deadlock-brain/releases/bfda408cb988722ddceadb56bca5b72e12d12731` | Pfadbeobachtung, nicht vollständige Herkunftsprüfung. |
| `deadlock-brain-site.service`, User-Service | active/running, PID `2002603`, Executable `/usr/bin/python3.12` | Laufende Site ist im gemessenen Moment Python, nicht der WIP-Rust-Port. Keine produktive Pythonänderung beauftragt. |
| `deadlock-bot-rust.service`, User-Service | active/running, PID `2848836`; Executable nicht lesbar | Prozessstatus beweist keine Serverguide-/Patenfunktion. |
| `deadlock-twitch-bot-rust.service`, System-Service | active/running, PID `1878256`; Executable nicht lesbar | Prozessstatus beweist keinen Twitch-Featurepfad. |

Ein `ctx_execute_file`-Zugriff auf das Release-Manifest außerhalb des Workspace wurde verweigert. Keine Berechtigungsänderung und kein alternativer Zugriff zur Umgehung. Deshalb keine neue Hash-/Manifestverifikation behauptet. Betriebsproben mit Nutzerkonten, LLM-Anfragen und Nachrichten waren ausgeschlossen. Die gesicherte Aussage ist Prozess-/Pfadstatus, nicht vollständige Funktionsabnahme.
