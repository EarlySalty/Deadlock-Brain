# Eingearbeitete Ergänzungen · Änderung v0.9 → v1.0

**EXT-01: Eingang und Planung geschlossen. Umsetzung weiterhin offen.**

## Tatsächlicher Quellenstand

| Kennung | Quelle | Behandlung |
|---|---|---|
| U1 | `Deadlock Brain Deep Research all in one Repo umstellung.md` | Konsolidierung, zentraler Answer Kernel, Jev, interne Daten, Docs und Bots |
| U2 | Nutzeraufträge in dieser Unterhaltung | Rust-Kern, alle Daten und Umbau gemeinsam, Performance, Aufteilung auf Chats |
| U3 | `deep-research-report Deadlock Brain und Deadlock Wikli.md` | Wiki-Discovery, Facts, Effect-/Mechanikmodell, Build-Regeln, Versionierung und Tests |
| U4 | `deep-research-report (5)(2).md` | Externe Repositories, Source Contract v2, API-Drift, historische Daten, Replay-/Population-Evidenz |

`deep-research-report (5)(1).md` und `(5)(2).md` haben denselben SHA-256:
`78eeee585f9dcae380b80a1214f8c81bb0423415870681163287f71ea7338e86`.
Sie sind kein doppelter Arbeitsauftrag. Originalquellen liegen unter `quellen/`; Hashes im Quellenmanifest.

## Was v0.9 tatsächlich fehlte

v0.9 enthielt nur U1 als Originalrecherche und hielt EXT-01 offen. Die allgemeinen Daten-/Performanceziele ersetzten nicht die konkreten Wiki- und Ökosystemanforderungen. v1.0 ergänzt deshalb nicht nur ein Quellenverzeichnis, sondern Verträge, Verantwortungen, Anforderungen, Tests, Release-Gates und eigene Arbeitsaufträge.

Neu sind Chat 12 (Wiki), 13 (externe Daten/Verträge/Schemaüberwachung) und 14 (Replays). Die bestehenden Chatnummern bleiben stabil; Chat 11 bleibt für den abschließenden Produktivwechsel zuständig, obwohl danach neue Nummern folgen. 03 verantwortet weiterhin alle Schemaänderungen; 05 weiterhin Fachlogik und Build-Engine; 04 weiterhin gemeinsame Worker-/Feeder-Infrastruktur.

## Aufgelöste Widersprüche der Recherchen

| Spannungsfeld | Verbindliche Planung |
|---|---|
| Historische Python-/Node-Pfade vs. Bericht über vorhandene Rust-Module | Chat 01 prüft den aktiven Code. Bestehendes gutes Rust wiederverwenden; nur fehlende/Legacy-Funktionen portieren. Keine Portierung allein aufgrund alter Notizen. |
| Vorgeschlagene neue `brain-*`-Ordner vs. vorhandene `dbrain-*`-Crates | Namen sind logische Vorschläge. 01/02 legen reale Pfad-/Owner-Zuordnung fest; keine Doppelmodule oder kosmetischer Komplett-Rewrite. |
| Wiki als Hauptimport vs. unterschiedliche Autorität von Patchnotes/Gamefiles/Telemetrie | Autorität pro Aussageart und Patch/Mode, kein blindes globales Mehrheitsvotum. Widersprüche bleiben prüfbar. |
| Mehrere scheinbar unabhängige Datenfeeds | Ableitungskette, Parserfamilie und gemeinsame Ursprungsartefakte erfassen; kopierte Gamefiles zählen nicht mehrfach als unabhängiger Beweis. |
| Einzelne Beispielwerte/Heldenzahlen im Wiki-Bericht | Historische Beispiele, keine aktuellen Konstanten. Manifest und Regeln bei Import dynamisch und versioniert erfassen. |
| Wiki-Modellkarte vs. normalisierte DB | Hero-Wissenskarte ist eine erzeugte, versionierte Sicht auf Facts und Evidenz; keine zweite handgepflegte Wahrheit. |
| Fremdparser in Python/.NET/JVM vs. Rust-Ziel | Produktions-, Rebuild- und regelmäßige Lernpfade bleiben Rust. Fremdtools nur optionale Offline-Referenzen mit versionierten Ergebnissen; keine Pflicht-Sidecars. |
| "Alle Daten" vs. private Inhalte oder ungeklärte Nutzungsrechte | Vollständiges Inventar und transparenter Status; keine öffentliche Veröffentlichung oder Mirror ohne Freigabe. Fehlende Pflichtdaten nicht still als optional umdeuten. |
| Alle Arbeiten in einem Zug vs. Sicherheit beim Wechsel | Ein Projekt/ein Zielmodell, aber getestete Zwischenstände und kontrollierter finaler Cutover. |

## Upgrade bei bereits begonnenem v0.9

00 sichert vorhandenen STATUS/ADRs und erfasst laufende Branches. Nur Planunterlagen ersetzen; keinen Code oder Arbeitsstand zurücksetzen. R26 ff. und Chats 12–14 zuordnen. Vorhandene Contracts gezielt erweitern; nicht parallel erneut erfinden. Falls G1 oder G2 bereits freigegeben war, die betroffenen Gates für neue Contract-/Knowledge-Versionen erneut prüfen. Bereits geprüfte unveränderte Komponenten dürfen mit ihrem Nachweis übernommen werden.

Eine dritte kleine Script-Ausnahme ist keine Freigabe für dauerhafte nicht-Rust-Backenddienste. Die in U1/U3 enthaltenen Python-/SQL-/YAML-Beispiele sind Forschungsentwürfe; verbindlich sind die hier festgelegten Rust-Verträge und Abnahmen.
