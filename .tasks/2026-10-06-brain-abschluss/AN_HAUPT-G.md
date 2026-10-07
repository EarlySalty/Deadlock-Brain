# Paket G: Brain v2

status: aktiv, 07.10.2026

## 09:00: Hold aufgehoben, G schließt selbst ab

`VON_HAUPT.md`, Abschnitt 09:00, tatsächlich gelesen. Paket I integriert E/F; G schließt nach eigener Abnahme/Gate selbst auf dem dann aktuellen origin/main nach E/F ab, einschließlich regulärem Deploy, Neustart, Live-Beweis und Cleanup. Alte Holdberichte unten beschreiben frühere Zeitstände. Nach Fetch beobachteter Main `f6f5cef65f1f946113f0b8216c6475f6d38ec928`; E/F-Lieferungen daraus noch nicht von G bestätigt. Keine Sessionkoordination oder Wartefenster auf fremde Builds.

G-P-R1 ist abgeschlossen: gemeinsamer duplikatsicherer JSON-Eingang und kompatibler Source-Adapter vorhanden, beide ursprünglichen Wireproben lehnen doppelte Argumentnamen ab. 71 Vertrags- und 32 Providerfälle bestanden vor der letzten JSON-Ergänzung; deren Compiler-/Testschlussprüfung bleibt offen. Source-/Verbraucherprüfung traf noch fehlende Combathelfer im laufenden Reasoner-WIP. Keine fertige Produktabnahme behauptet.

Begrenzte Fehlerabrechnungsfortsetzung G-K-R1 tatsächlich gestartet: Task `w7kbtvxfg`, Run `wf_18a32653-098`, `G/BRIEFING-G-K-R1.md`. Einziger Vertrags-/Provider-/Kernel-Schreiber nach tatsächlichem JSON-Abschluss; bewahrt beobachtete Usage auch bei Fehlern, trennt konservative Reservierung und prüft die letzte JSON-Ergänzung vollständig. G-M-06:45 läuft disjunkt im Reasoner weiter. Gebaut: Teilstände. Reviewt: bisher G0 und Dokumentcheckpoints. Gemergt: nein. Live: nein.

## Abschluss-Hook vor 09:00 und aktive Fortsetzungen

Stop-Hook meldet offenen eigenen WIP und sieben Featurecommits außerhalb main. Tatsächlicher HEAD `ce21a457`, Status und Releasefenster erneut geprüft: keine Hold-Aufhebung. Der verlangte Main-Merge/Cleanup würde aktive Arbeit und die verbindliche Releasegrenze verletzen und wird nicht ausgeführt. JSON-Worker `wvlm6fn18` und Rechenfortsetzung `wq8uvf8ah` zeigen reale laufende Prüfaufrufe; kein Ersatzschreiber. `G/BRIEFING-G-K-R1.md` vorbereitet, noch nicht gestartet, da der Vertragsschreibbereich belegt ist. Kein verfrühter Produktcommit, kein Verwerfen oder Settle.

## Nachweischeckpoint ce21a457 auf origin

Rückgaben von G-M/G-P/G-K, konkrete Abnahmelücken und die exklusiven Folgeaufträge sind auf dem Featurebranch gesichert. Regulärer Dokumentgate gegen `1f5ed30f`, Exit 0: `[gpt-6.1-sol] ALLOW: no reviewable changes`, Log `/tmp/brain-g-worker-handoffs-gate-20261007.log`. Kein Produktreview daraus abgeleitet. Aktiv sind JSON-Anschluss `wvlm6fn18` und Wachstum/Sheet `wq8uvf8ah`, ohne gemeinsame Produktdateien. Fehlerabrechnung wartet auf freien Vertragsschreibbereich, nicht auf einen fremden Build. G noch nicht fertig; Hold bleibt verbindlich.

MERGEPROTOKOLL[MS-1]: 4 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW: no reviewable changes; kein Main-Merge

## G-M und G-K zurückgegeben, beide noch ohne Produktabnahme

G-M (`wthzcnb2d`) ist abgeschlossen. Neue Rechenfälle: 17 passed; isolierte Suite: 303 passed, 5 failed, 0 ignored, gegenüber Baseline 287 passed, 4 failed. Bereichsführung prüfte die tatsächlichen Testmarker und Fehlernamen. Neue Regression ist eine Planer-Testfixture ohne Grenznutzen des zweiten Kaufs bei diskreten Schüssen. **Vor Änderung gemeldete Testgrenze:** `dbrain-reasoner/src/planner.rs` ausschließlich dieser Test und nötige Fixtureeingaben; keine Planer-Produktlogik oder Publish-Regel. Danach wächst derselbe reine Kern gemäß Entscheidung 06:45 und Sheetrekonstruktion. Fortsetzung gestartet: Task `wq8uvf8ah`, Run `wf_fca62072-53f`, `G/BRIEFING-G-M-0645.md`; ursprünglicher G-M-Schreibweg beendet. Vier alte DB-Fixturefehler, drei ausgeschlossene Produktionsfälle und echte E/F-Laufzeitbindung bleiben getrennte Grenzen. Keine vollständige Zahlenabnahme behauptet.

G-K (`w120utj39`) ist ebenfalls abgeschlossen. Rückgabe: `Kernel::with_tools(port, resolver, provider_identity)` und verpflichtender `GameContextResolver::{resolve,validate}`; Pin vor Cache-/Flight-Schlüssel. Gemeldet 49 passed, 18 gleiche Baselinefehler, 0 ignored; neue Fachfälle 15. Bereichsführung bestätigte tatsächliche Suitezahlen, dieselben 18 Baseline-Fehlernamen und grüne Format-/Clippy-/Compiler-/Verbraucherexits. Die fünf Kerneldateien bleiben fingerprintgleich; inzwischen erweiterter gemeinsamer JSON-Eingang braucht erneute Verbraucherprüfung. Anschluss `G/G-K-NACHWEISE.md`. Konkrete Abnahmelücke: Fehler von Provider und Toolport tragen noch keine gemessene Usage. Diese begrenzte Vertragsfortsetzung wird erst nach Abschluss des laufenden gemeinsamen JSON-Workers zugeteilt; kein paralleler Brain-Vertragsschreiber. Kein Produktgate, Main oder Liveabschluss.

## G-P: 29 Fälle bestanden, JSON-Eingang noch nicht abgenommen

Providerworker abgeschlossen, Rohbelege `G/pruefungen/g-p/`. Bereichsführung bestätigte vier grüne Abschluss-Exits, 29 passed, 0 failed, 0 ignored sowie sieben unveränderte Quellfingerprints. Baseline 19 passed, 0 failed. Beide Wireformen tragen den Acht-Tool-Vertrag, ursprüngliche Deadline, kumulierte Usage und gebundene Ergebnisse. Noch kein Gate oder echter Luna-Beweis.

Konkreter Blocker: Beide Providerparser akzeptieren doppelte JSON-Argumentnamen und behalten den letzten Wert. `json-probe.log` bestätigt native und OpenAI-kompatible Form. Der grüne Suiteabschluss ist dafür kein Sicherheitsbeweis. Kein Produktcheckpoint vor Korrektur.

**Begrenzte Anschlussgrenze vor Änderung:** vorhandenen duplikatsicheren Parser aus `dbrain-sources/src/external/strict_json.rs` einmal in den bereits von Quellen und Providern benutzten `brain-contracts`-Bereich verlegen. Der bestehende private Source-Eingang behält Signatur und delegiert an dieselbe Implementierung. Provider verwendet diesen Eingang für Antwortobjekt, Argumentstrings und finales JSON. Kein zweiter Parser, keine neue Crate-/Manifestabhängigkeit und keine Änderung an Es Import, API-Pins oder Analytics. Neuer nativer Vertrag-/Providerworker gestartet: Task `wvlm6fn18`, Run `wf_89eb7717-56e`, `G/BRIEFING-G-P-R1.md`, ausschließlich diese Grenze; G-K schreibt weiter getrennt im Kernel. G0 und ursprünglicher G-P-Schreibweg sind abgeschlossen.

TESTNACHWEIS[TW-1]: 29 passed, 0 ignored | Baseline: 0 rot

## Dokumentcheckpoint 1f5ed30f gesichert

Sheetrekonstruktion, aktueller Vertrags-/Workerstand und Korrektur zu Punkt 5 sind auf `origin/feat/brain-v2-g-20261007` gesichert. Regulärer Dokumentgate gegen `3d6890c0`, Exit 0: `[gpt-6.1-sol] ALLOW: no reviewable changes`. Dieses Urteil liefert keinen Produktreview; der Commit enthält acht Dokumentdateien. Log `/tmp/brain-g-sheet-scope-gate-20261007.log`. Laufender Rechenkern-/Provider-/Kernel-WIP blieb unstaged. Gemeinsame Steuerung nennt die Grafik-Korrektur nun ausdrücklich als Nachtrag 06:55. E/F-Akten liefern weiterhin keinen zusätzlichen Receipt-, globalen Daten- oder reinen Buildadaptervertrag. Hold unverändert.

MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW: no reviewable changes; kein Main-Merge

## Korrektur zu 06:45, Punkt 5

Grafiken und Webseiten baut der Nutzer separat. G baut nichts dazu und trägt nichts in die Roadmap ein; Werkzeugausgaben bleiben strukturierte Zahlenreihen. Der eigene Grafikabschnitt aus `f81e2ae2` wurde aus `docs/brain-qa-roadmap.md` entfernt. `G/PLAN.md` und Register folgen der Korrektur. Die früheren Zeitstandsabschnitte unten dokumentieren die ursprüngliche, inzwischen zurückgenommene Entscheidung.

## 07.10.2026: Acht-Tool-Vertrag gesichert, acht Sheetstellen geprüft

G0-06:45 ist als `3d6890c0` auf origin gesichert: `game_rules`, typisierter Boonbereich sowie API-Rang-/Zeitfilter für Profile und Vergleiche. 67 passed, 0 failed, 0 ignored; Compiler, Format, striktes Clippy und damalige Verbraucherkompilierung bestanden. Bereichsführung prüfte tatsächliche Logs/Exits und unveränderte Quell-/Referenzfingerprints. Regulärer Gate gegen `f81e2ae2`, Exit 0: `[gpt-6.1-sol] ALLOW: No grounded blocking defects found in the supplied diff.` Log `/tmp/brain-g0-0645-gate-20261007.log`, Anschluss `G/G0-0645-VERTRAG.md`. Die generischen Portsignaturen blieben unverändert. API-Rangfilter bezeichnet ausdrücklich den durchschnittlichen Rang beider Teams, keine individuelle Spielerrangklasse.

`G/SHEET-MODELL.md`, Abschnitt 14, dokumentiert alle acht beschädigten Stellen. Fünf DNS-Blöcke liefern Melee und vier Signaturfähigkeiten. Flying Slash braucht echte Light-Melee-Skalierung und erhält den gültigen Null-Basiswert. Drei Scratchpadformeln sind über erhaltene Haze-Zwillinge strukturell rekonstruierbar; gelöschter Zusatzschaden und inzwischen falsch verkabelte Bonus-/Ratenzellen verhindern eine belegte eindeutige Originalzahl. Die bekannten Zwillingseingaben ergeben 78,91 DPS. Keine erfundenen Ersatzwerte und kein Rustbeweis behauptet. Original-XLSX und beide API-Payloadhashes durch Bereichsführung nachgeprüft; bytegleiche E-Probe bindet sie an 6759, nicht an den aktiven Produktionspatch.

G-P (`w5pqkkyxv`, `wf_9c5666f8-dc6`) und G-K (`w120utj39`, `wf_bade480e-cb2`) sind in disjunkten Provider-/Kernelbereichen gestartet. Beide verwenden die geprüfte generische API und müssen die tatsächliche Acht-Tool-Fassung abschließend konsumieren. G-M läuft im bisherigen WIP weiter. Sheet-Anschlussbedarf wird nach seiner Rückgabe an den einzigen Reasoner-Eigentümer gegeben; kein paralleler Schreiber. E-Receipt/globale Daten, exklusiver Analytics-Anschluss und F-Integration bleiben offen. Kein Main-/Runtime-/Liveabschluss.

TESTNACHWEIS[TW-1]: 67 passed, 0 ignored | Baseline: 0 rot

MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW für 3d6890c0; kein Main-Merge

## 07.10.2026: Entscheidungscheckpoint auf origin

`f81e2ae2` ist auf `origin/feat/brain-v2-g-20261007` gesichert. Er enthält aktualisierten Plan, Roadmapeintrag, Aufgaben-/Workerakten und die archivierten G0-Prüfbelege. Regulärer Dokumentgate gegen `b4f4b866`, Exit 0: `[gpt-6.1-sol] ALLOW: Documentation and archived verification logs only; no merge-blocking defect found.` Log `/tmp/brain-g-decisions-gate-20261007.log`. Dieses ALLOW ist kein Abnahmebeweis für die laufenden neuen Produktänderungen.

**G-K-Dateigrenze vor Start präzisiert:** Zusätzlich zu `lib.rs`, `execution.rs` und `flight.rs` gehören die bestehenden `brain-kernel/src/cache.rs` und `outcome.rs` zum einzigen Kernelworker. Nach Graphify-Abfrage geprüft: Outcome hält nur Belege, Cache prüft sie ohne typisierte Tool-Unteranfrage. Erweiterung derselben Bausteine ist nötig für vollständige Toolabhängigkeiten; kein weiterer Cache oder fremder Schreiber. Serverseitiger Anfrage-Pin vor Cache-/Flight-Schlüsselbildung ist ebenfalls im Kernelbriefing festgehalten; ein fehlender Portzugang wird als begrenzter Vertragsbedarf behandelt.

MERGEPROTOKOLL[MS-1]: 4 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW für f81e2ae2; kein Main-Merge

## 07.10.2026, 07:13: Entscheidungen 06:45 übernommen, G0 gesichert

Die fünf Nutzerentscheidungen sind in `G/PLAN.md` einschließlich Rechenverträgen, Werkzeugen, Abnahme und Baufolge übernommen. `docs/brain-qa-roadmap.md` enthält das spätere Grafik-/Webseitenziel mit niedriger Priorität; kein Grafikbau. Wachstum wird von G und F gemeinsam gerechnet. Hidden Mechanics werden als Profilabschnitt und `game_rules` geplant. DNS-/`#REF!`-Rekonstruktion ist verbindliche weitere Abnahme. Meta-Ränge kommen aus API-Aggregaten mit Rang-/Zeitfilter, ohne Einzelmatchablage.

G0-Sieben-Tool-Vertrag lokal verifiziert und als `b4f4b866` auf dem Featurebranch gesichert. Bereichsführung prüfte Rohlogs/Exits und identische Quellfingerprints: 55 passed, 0 failed, 0 ignored; Compiler, Format, striktes Clippy und Verbraucherkompilierung bestanden. Regulärer Gate gegen `f129c91a`, Exit 0: `[gpt-6.1-sol] ALLOW: No merge-blocking defect found in the supplied diff and revision-specific snapshots.` Vertrag `G/G0-VERTRAG.md`, Rohbelege `G/pruefungen/g0-r1/`, Gatelog `/tmp/brain-g0-gate-20261007.log`. Kein E/F-, Luna- oder Livebeweis daraus abgeleitet.

`game_rules`, Boonkurvenparameter und Analytics-Filter sind eine begrenzte neue Vertragsfortsetzung, nicht durch dieses ALLOW geprüft. Sie ist im disjunkten Brain-Vertragsbereich gestartet: Task `w5dr3fr2i`, Run `wf_1cdc6a56-2a1`, Briefing `G/BRIEFING-G0-0645.md`. Die gezielte Rekonstruktion der acht Sheetstellen läuft dokumentseitig getrennt: Task `w3rx1jcpj`, Run `wf_c7702e0e-8d1`, Briefing `G/BRIEFING-G-S.md`, noch kein Produktbeweis. G-M wurde nach Sessionabbruch im vorhandenen Workflow/WIP wiederaufgenommen, Task `wthzcnb2d`, Run `wf_08b3462e-169`; noch kein Abschlussbericht. Kein doppelter Reasoner-Schreiber. G-P/G-K noch ungestartet.

**Schnittstellenbedarf vor Änderung:** `dbrain-sources/src/analytics_runtime.rs` wurde nach Graphify-Vorabfrage nachgelesen. Der bestehende Client liefert Helden-Meta und itemgefilterte Heldenpopulation; noch kein `item-stats`, Rangfilter oder vollständiger Meta-Rangvergleich. G benötigt die begrenzte Erweiterung dieses vorhandenen Moduls samt direkt betroffenem Schema-/Testvertrag nach zeitlich exklusiver Eigentumsklärung mit E. Kein neuer Analytics-Client oder Importer. Die bestehende Herkunft markiert Patchmitgliedschaft als `Unverified`; diese Grenze wird nicht durch ein frei gesetztes Patchlabel beseitigt. Details in `G/PLAN.md`, C6. E-Receipt und globale Mechanikdaten sowie Fs reiner Eingang bleiben offen; F soll dieselbe Wachstumsprojektion verwenden.

Gebaut: G0 lokal. Reviewt: G0 ALLOW. Gemergt: nein. Live: nein. Hold und Integration E, F, G unverändert.

TESTNACHWEIS[TW-1]: 55 passed, 0 ignored | Baseline: 0 rot

MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW für b4f4b866; kein Main-Merge

## 07.10.2026, 06:15: Dokumentgate ALLOW nach technischem Retry

Commit `f129c91a` gegen `96e6a8da` regulär geprüft. Erster Aufruf Exit 2 ohne Modellurteil: `bwrap`/`unshare` konnten keinen Namespace anlegen (`Cannot allocate memory`). Genau ein unveränderter Retry bestand mit Exit 0:

`[gpt-6.1-sol] ALLOW: Documentation and reference manifest only; no blocking defect found in the supplied diff.`

Rohlogs: `/tmp/brain-g-docs-gate-20261007.log`, `/tmp/brain-g-docs-gate-retry-20261007.log`. Keine Modellwahl, Hook-, Namespace- oder Berechtigungsänderung. Der Retry betrifft den Dokumentcommit, keine uncommittierte Produktänderung. Main und Runtime unverändert.

MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW für f129c91a; kein Main-Merge

## 07.10.2026: G0-Vertragsfix, Recherchecheckpoint gesichert

Dokumentcheckpoint `f129c91a` ist auf `origin/feat/brain-v2-g-20261007` bestätigt. Sheetmodell, drei Bestandsberichte, vollständige Baselinegrenzen, Plan und die ersten Bauaufträge sind damit gesichert. Keine Produktänderung committed oder gepusht.

G0 meldete konkurrierende Vertragsformen und Compilerfehler: fmt Exit 1, Clippy/Test Exit 101, kein ausgeführter Test. Die native Ergänzungsnachricht startete eine zusätzliche Fortsetzung, deren Schreibweg gegenüber dem ursprünglichen Workflow nicht serialisiert war. Bereichsführung hat diese Fortsetzung mit bestätigtem TaskStop beendet; ursprünglicher G0-Workflow ebenfalls abgeschlossen. Kein fremder Session-Schreiber belegt und keine Änderungen zurückgesetzt.

Frischer G0-R1-Worker führt die vorhandenen drei Vertragsdateien jetzt exklusiv zusammen: Task `w1ozge7jw`, Run `wf_563c24f7-221`, `G/BRIEFING-G0-R1.md`. G-P/G-K bleiben bis zum verifizierten Vertrag ungestartet. G-M läuft in seinem disjunkten Reasonerbereich weiter, Task `win5xzl3o`. Der Konflikt ist ein eigener Baufehler, nicht durch die 38 Baselinefehler erklärt.

E-Receipt, globale Mechanikdaten und Fs reiner Buildvertrag bleiben gemeldete Integrationsabhängigkeiten. Release-Hold, keine Runtimewirkung und Reihenfolge E, F, G unverändert.

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
