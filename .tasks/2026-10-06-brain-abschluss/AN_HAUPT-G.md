# Paket G: Brain v2

status: aktiv, 07.10.2026

## 07.10.2026, 06:00: Bau gestartet, Liefergrenzen vor Änderung gemeldet

G0 baut den gemeinsamen Werkzeugturn-/Toolport-Vertrag in `brain-contracts/src/{lib.rs,provider_input.rs,tools.rs}`. Native Workflowkennung `wgid0g5cj`, Run `wf_45c9b56b-428`. Produktarbeit bleibt im eigenen Worktree; keine weitere T3-Session.

Der Plan liegt unter `G/PLAN.md` vor. Baseline ist abgeschlossen: 474 eindeutige bestandene und 38 fehlgeschlagene Fälle; 20 weitere Fälle ausdrücklich ausgeschlossen. Bericht `G/BASELINE.md`, Rohbelege im ursprünglichen Baselinebereich. Kein grüner Gesamtbeweis.

**Weitere G-Schreibgrenze vor Bearbeitung:** reine öffentliche Modell-/Szenarioverträge, Payloadkonverter und Exports in `dbrain-reasoner/src/{types.rs,data.rs,lib.rs}`; danach gemeinsame Mechanik in `mechanics.rs`, `progression.rs`, `combat.rs`, `defense.rs` und bestehenden Interaktionsmodulen. In `data.rs` verändert G die reinen Konverter, nicht Fs aktuelle Modelllade-/SQL-/Spiegelauswahl. Kein zweiter Parser. F bleibt Eigentümer seiner Loader, Publish-Regel und Confidence. Die gemeinsamen Konverter-/Exportänderungen werden als begrenzter eigener Diff übergeben und nach E/F integriert. G-V schreibt diese Reasoner-Dateien nicht parallel.

**Zusätzliche notwendige Lieferung E:** Der bestehende Leser gibt Originalpayload als `Value`, aber noch keinen Beleg des tatsächlich gelesenen Source-Runs/Dokuments zurück. Für Rechte-, Herkunfts- und Cache-Neuprüfung benötigt G einen kompatiblen gemeinsamen Receipt-Zugang: Clientversion, Run-ID, Manifest- und Endpoint-Dokument-ID, Art/Sprache, Original-URL/-Hash, Parserrevision sowie Spiegel-/Prüfuhrzeit. Details und konsistente Payloadbindung: `G/PLAN.md`, C1. G baut keinen eigenen SQL-Werteleser. Der Bedarf für globale Mechanikdaten steht in Planabschnitt 5; `modifiers` bietet laut Originalschema keine Sprachvariante.

**Zusätzliche notwendige Lieferung F:** reiner Plan-/BuildObject-Eingang für bereits geladene versionsgebundene Modelle, mit tatsächlich angewandtem Spielstil, AI aus, Persistenz aus und ohne Analytics-Netzwerk oder Veröffentlichung. Bestehenden Planer/Composer verwenden. G bindet diesen Eingang als `build_plan` an; kein eigener Composer und keine erfundene Build-ID.

G-V ist an diese Lieferungen gebunden. G0 sowie reine Modelle/Rechnung und Provider-/Kernel-Erweiterung können lokal unabhängig weiterbauen. Release-Hold und Integration E, F, G bleiben unverändert.

## 07.10.2026, 05:55: Recherche abgeschlossen, gemeinsamer Leser bestätigt

G arbeitet in `/home/nathanael/.worktrees/brain-g-v2-20261007`, Branch `feat/brain-v2-g-20261007`, HEAD `96e6a8da`. Der Dokumentcommit ist auf dem Featurebranch gesichert. Produktquellen gegenüber der Basis `bfda408c` sind unverändert. Hauptsteuerung 05:40 bestätigt die eigene Worktree-Akte als Übergabeort.

Sheet und drei Bestandsrecherchen sind abgeschlossen: `G/SHEET-MODELL.md`, `G/BESTAND-MECHANIK.md`, `G/BESTAND-ANTWORT.md`, `G/BESTAND-DATEN.md`. Der unveränderte XLSX-Export umfasst 13 sichtbare Tabs und 5.307 Formelzellen. Alle Tabs wurden visuell nachvollzogen; zwei Roh-/Abfragetabs nur als unformatierter Ausschnitt A1:T20. Grenzen, Fehlercaches und historische Tabellen sind im Modell ausgewiesen. DPM bedeutet Schaden pro Magazin. Persönliche Meta-Rankings werden nicht als Fakten übernommen; die Sheet-TTK lässt notwendige Reload-Zeiten weg.

**Verbindliche Datenbasis:** `brain_storage::asset_mirror::{latest_mirrored_client_version, load_mirrored_assets}` aus E. Original-JSON aus vollständigen erfolgreichen lokalen Runs je Clientversion; kein HTTP-Fallback. `entity_snapshots`, `hero_catalog` und `item_catalog` sind keine aktuelle G-Spielwertequelle. Die Clientversion ist kein bestätigter Balancepatch. Ränge nutzen die aktiven Helden desselben vollständigen Spiegelstands.

Es Bericht nennt inzwischen die Featurecommits `5e70da3a` und `e65efae2`. Der zweite enthält den gemeinsamen Leser. E meldet 513 bestandene Tests und 24 ignorierte Tests, zusätzliche echte öffentliche Contractproben sowie einen technischen Gate-Ausfall ohne Urteil. Diese Angaben sind Es Nachweise, keine G-Verifikation oder Mergefreigabe. F stellt seine Loader auf denselben Leser um und besitzt `dbrain-reasoner/src/data.rs` sowie dessen Manifest. G kopiert weder Es Import noch Fs uncommittierten Arbeitsstand.

**Offener Datenvertrag vor Neubau:** Es Leser bietet bisher `items`, `heroes`, `heroes_all` auf Englisch und Deutsch. Teile von Hidden Mechanics benötigen zusätzlich versionsgebundene `npc-units`, `misc-entities`, gegebenenfalls `modifiers` und `generic-data`. Öffentliche Originalproben liegen vor, aber kein gemeinsamer lokaler Leseschnitt dieser Arten. G meldet die Lücke statt einen eigenen Import oder ungepinnte Frageabrufe zu bauen. Die konkreten Feldgruppen stehen im Sheet-Modell.

**Abobrücke:** Port 18769 gehört dem bestehenden `claude-code-proxy` 0.1.43. Der öffentliche Quellstand dieses Tags übersetzt native `tool_use` und `tool_result` in Codex-Funktionsaufrufe und zurück. Der Brain-Transport setzt dagegen bisher `tools=[]`, `tool_choice=none` und verwirft `tool_use`. G erweitert den bestehenden Brain-Provider; kein neuer Connector, Modellwechsel oder Proxyupdate. Quellprüfung ist noch kein echter Luna-Werkzeuglauf.

`G/PLAN.md` entsteht durch einen nativen Planworker aus den abgeschlossenen Berichten. Baselineworker hat fmt und Compiler mit Exit 0 sowie rote Bestandssuites und Retrieval-Clippy belegt. Schlussbericht und exakte getrennte Testzahlen stehen noch aus. Keine Gesamtprüfung als grün ausgegeben.

Gebaut: nein. Reviewt: nein. Gemergt: nein. Live: nein. Native Worker und Nachweisorte: `G/REGISTER.md`.

## Schnittstellen und Betriebsgrenzen

F besitzt Publish-Abnahme, Planer, Confidence und aktuelle Modellloader. G erweitert die gemeinsame Mechanik und nutzt den bestehenden Planer als lesendes Werkzeug: AI aus, Persistenz aus, keine Veröffentlichung. A besitzt die Übergangsfreischaltung, Bot-Consumer und Invites. Deren Runtime, Rechte und Writerfence bleiben unangetastet.

Release-Hold gilt: kein Main-Push, Release-Build, Install, Neustart oder produktiver Tick durch G. Nach Hold-Ende Integration E, F, G; Installation und Live-Strecke nach Eigentümerregel. Abbau erst nach belegtem Gleichstand in eigenen geprüften Commits.

## Frühere Referenzmessung

`G/API-PROBEN.json` bindet öffentliche Proben an Clientversion 6759, nicht an die Produktionsversion. Waffen-DPS für Warden, Wraith und Haze: 66,0571, 59,6825 und 50,0952. Sheet: 68,5426, 59,784 und 50,0752. Unterschiedliche Spirit-Szenarien, vorgelagerte Rundung und Wardens abweichende Ausgangswerte erklären die Abweichungen. Der Rust-Abgleich muss dieselben Rohwerte und Szenarien rechnen; diese Proben sind noch kein Rechenkern- oder Antwortdienst-Test.

TEXTNACHWEIS[DR-1]: Gedankenstriche 0 | ae/oe/ue/ss-Ersatz 0 | Absolutwörter 0 belegt | Senke: eigene G-Aufgabenakte
