# Paket G: Register

status: aktiv, 07.10.2026

## Bereichsführung

- Auftrag: `.tasks/2026-10-06-brain-abschluss/BRIEFING-G.md`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`, Kommunikation über die Akte.
- G-Thread laut Steuerung `a867ef50`; native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Modell `gpt-6.1-sol[1m]`, bestätigte UltraCode-Workflowstarts mit Effort `xhigh`.
- Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`; Branch `feat/brain-v2-g-20261007`; Basis `bfda408cb988722ddceadb56bca5b72e12d12731`, HEAD `b4f4b866`. Dokumentcheckpoint `f129c91a` ist gepusht; der neue G0-Produktcheckpoint wird regulär geprüft.
- Statusproduzent: Bereichsführung G. Kein zusätzlich vergebener zentraler Versuch; keine erfundene Nummer. Keine weiteren T3-Threads.
- Wirkung: Featurearbeit und lokale Prüfungen. Release-Hold sperrt Main-Push, Release-Build, Install, Neustart und produktiven Tick. Kein Settle bei offener Arbeit.

## 07:13: Nutzerentscheidungen übernommen, G0-Produktcheckpoint

`VON_HAUPT.md`, Abschnitt 06:45, tatsächlich gelesen. Alle fünf Entscheidungen sind in `G/PLAN.md` übernommen: gemeinsames Wachstum auch für F-Builds, Hidden Mechanics in Profilen und `game_rules`, Rekonstruktion der fünf DNS-Blöcke und drei `#REF!`-Formeln, API-Meta mit Rangfilter und Aggregaten ohne Einzelmatches, Grafik-/Webseitenziel ausschließlich als spätere niedrige Priorität. Roadmapeintrag in `docs/brain-qa-roadmap.md`, kein Grafikbau. Der vorhandene Analytics-Baustein wurde nach Graphify-Vorabfrage nachgelesen; `item-stats`, Rangfilter und vollständiger Meta-Vergleich fehlen dort noch. Zeitlich exklusiver Änderungsbedarf ist an Hauptsteuerung gemeldet, keine fremde Datei geändert.

G0-R1 wurde nach dem gestoppten Workflow durch den frischen Vordergrundworker `af3b631553a29ed1f` abgeschlossen. Bereichsführung las Vertrag, tatsächliches Testlog, Compiler-/Clippy-/Verbraucherlogs und Exitdateien. 32 Unit- und 23 Integrationsfälle bestanden, 0 failed, 0 ignored; kein Doc-Testfall. Die drei getesteten Quelldateien stimmen per `sha256sum --check` mit `G/pruefungen/g0-r1/source.sha256` überein. Bericht `G/G0-VERTRAG.md`. Verbraucherkompilierung war gegen den damaligen gemeinsamen WIP; sie beweist keine E/F-Integration oder Live-Strecke. Für den Logcheckpoint wurde ausschließlich die letzte Leerzeile am Ende von `test.log` entfernt; sämtliche Prüfzeilen und Exitdateien blieben erhalten.

Produktcheckpoint `b4f4b866` enthält ausschließlich die drei Brain-Vertragsdateien und G0-Auftrag/Bericht; Featurepush nach origin bestätigt. Drei schreibende Git-Schritte einzeln: add, commit und Featurepush; zusätzlich getrennte read-only-Prüfungen. Kein Mainbezug. Regulärer Gate-Task `bu2svwkpn` bestand mit Exit 0, Base `f129c91a`, Head `b4f4b866`, Log `/tmp/brain-g0-gate-20261007.log`: `[gpt-6.1-sol] ALLOW: No merge-blocking defect found in the supplied diff and revision-specific snapshots.` Dieses Urteil betrifft genau den Sieben-Tool-Produktcheckpoint, nicht die noch offene Vertragsfortsetzung aus 06:45.

MERGEPROTOKOLL[MS-1]: 3 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW für b4f4b866; kein Main-Merge

Der verifizierte G0-Stand hat sieben Toolnamen. `game_rules`, typisierter Boonkurvenbereich und Analytics-Filter sind neue begrenzte Vertragsarbeit aus 06:45, noch nicht implementiert. G-P/G-K bleiben bis zum geregelten Anschluss ungestartet. G-M wurde mit dem vorhandenen Workflow wiederaufgenommen, nicht neu gebaut. Wache: Journal bestätigt den neuen nativen Agenten, zuletzt tatsächlicher Werkzeugaufruf; kein Abschlussrecord und kein Ersatzschreiber gestartet. Neue Mechanikanforderungen werden nach seiner Rückgabe gegen den vorhandenen Stand abgeglichen, ohne parallelen Reasonerschreiber.

TESTNACHWEIS[TW-1]: 55 passed, 0 ignored | Baseline: 0 rot

## Native Worker

| Worker | Schreibbereich | Belegter Start | Zustand |
| --- | --- | --- | --- |
| sheet-modell | Eigene `G/SHEET-MODELL.md`, `G/sheet/manifest.json`; Rohexport und Bilder im ursprünglich zugewiesenen gemeinsamen `G/sheet/` | Task `w6krhelmo`, Run `wf_f775f9c0-69b` | abgeschlossen, 13 Tabs, 5.307 Formelzellen |
| bestand:mechanik | Rückgabe zur eigenen `G/BESTAND-MECHANIK.md`; keine Produktänderung | Task `wyqxc1iva`, Run `wf_4e39a761-389`, Agent `a335322bab03367e9` | abgeschlossen, Bericht übernommen |
| bestand:antwort | Rückgabe zur eigenen `G/BESTAND-ANTWORT.md`; keine Produktänderung | Derselbe Workflow, Agent `a2e0399691fd75107` | abgeschlossen, Bericht übernommen |
| bestand:daten-abbau | Rückgabe zur eigenen `G/BESTAND-DATEN.md`; keine Produktänderung | Derselbe Workflow, Agent `a45d24611b1d8a737` | abgeschlossen, Bericht übernommen |
| baseline | Ursprünglich zugewiesene Rohbelege `G/pruefungen/baseline/`; eigene `G/BASELINE.md` aus Rückgabe | Task `wih3iu8uu`, Run `wf_ccd9af54-f71` | abgeschlossen, 474 passed und 38 eindeutige failed; 20 Fälle nicht abgedeckt |
| plan | Ausschließlich eigene `G/PLAN.md`; kein Produktcode, Git oder Runtime | Task `wdef9ng7t`, Run `wf_f79bff70-d4b`, Start 05:42 | abgeschlossen; Bereichsführung hat G0/G-M-Grenzen und finale Baseline eingetragen |
| G0:Werkzeugvertrag | `brain-contracts/src/{lib.rs,provider_input.rs,tools.rs}` und Cratetests | Task `wgid0g5cj`, Run `wf_45c9b56b-428`, Agent `a3a054b7281fbde9e` | abgeschlossen mit Vertragskonflikt; native Fortsetzung nach bestätigtem TaskStop beendet |
| G0-R1:Vertragfix | Ausschließlich dieselben drei Vertragsdateien und Cratetests; keine weiteren G0-Schreiber | Fixworkflow `w1ozge7jw` / `wf_563c24f7-221` gestoppt; Fortsetzung durch frischen Vordergrundworker `af3b631553a29ed1f` | abgeschlossen; 55 passed, 0 failed, 0 ignored; b4f4b866 mit Gate ALLOW gepusht |
| G0-06:45:Vertragsfortsetzung | Dieselben drei Brain-Vertragsdateien und unmittelbar zugehörige Tests; eigener neuer Bericht und Rohbelege | Task `w5dr3fr2i`, Run `wf_1cdc6a56-2a1`, Briefing `G/BRIEFING-G0-0645.md` | gestartet nach b4f4b866 ALLOW; achter Toolname, Boonbereich und Analytics-Auswahl; noch kein Abschlussbeweis |
| G-S:Sheetrekonstruktion | Ausschließlich `G/SHEET-MODELL.md`, eigener Rekonstruktionsbericht und schmale Belege; kein Produktcode | Task `w3rx1jcpj`, Run `wf_c7702e0e-8d1`, Briefing `G/BRIEFING-G-S.md` | gestartet; gezielte acht beschädigte Stellen, kein Wiederholen der Gesamtrecherche |
| G-M:Rechenkern | Reasoner-Modelle, reine Konverter, Exports, Mechanik und vorhandene Simulation gemäß `BRIEFING-G-M.md`; Fs Loader bleiben unverändert | Ursprünglicher Task `win5xzl3o` gestoppt; Resume `wthzcnb2d`, Run `wf_08b3462e-169`, Agent `af3dcfc2c09526a0b`, Start 06:36 | Wiederaufnahme im vorhandenen WIP bestätigt; noch kein Abschlussrecord, kein zweiter Schreiber |

Vollständige abgeschlossene Workflow-Rückgaben liegen unter `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/030a7b6f-d25c-482d-b66c-68185cd05dbb/tasks/`: `w6krhelmo.output`, `wyqxc1iva.output`. Die anfänglich ausgegebenen, damals nicht vorhandenen Transkriptordner sind kein Ergebnisnachweis.

## Dokumentgate 06:15

Der gepushte reine Dokumentcommit `f129c91a` gegen `96e6a8da` wurde regulär geprüft. Erster Lauf Exit 2 ohne Urteil, Namespaceausfall bei `bwrap`/`unshare`. Ein unveränderter Retry Exit 0: `[gpt-6.1-sol] ALLOW: Documentation and reference manifest only; no blocking defect found in the supplied diff.` Logs `/tmp/brain-g-docs-gate-20261007.log` und `/tmp/brain-g-docs-gate-retry-20261007.log`. Keine Modell-, Hook-, Namespace- oder Berechtigungsänderung. Dieses ALLOW deckt keine uncommittierte Produktänderung.

## G0-Vertragskonflikt und Fixrunde

Der ursprüngliche G0-Workflow meldet inkompatible Typänderungen während seiner Prüfung: fmt Exit 1, Clippy/Test Exit 101, kein ausgeführter Test. Die native Nachricht startete eine zusätzliche Fortsetzung mit demselben Agent-ID-Kontext; dieser Schreibweg war gegenüber dem ursprünglichen Workflow nicht serialisiert. Kein fremder Session-Schreiber ist belegt. Bereichsführung beendete die Fortsetzung `a3a054b7281fbde9e` per TaskStop, Bestätigung und killed-Fertignachricht liegen vor. Der ursprüngliche Workflow war bereits abgeschlossen.

Frischer Worker G0-R1 besitzt jetzt allein die drei Vertragsdateien. Auftrag `G/BRIEFING-G0-R1.md`: vorhandene Formen kompatibel zusammenführen und komplett nachprüfen. Keine weitere aktive Workflow-Nachricht, keine Rücksetzung. G-P/G-K bleiben bis zum verifizierten Vertrag ungestartet. G-M kann in seinem getrennten Reasonerbereich weiterarbeiten. Noch keine Produktänderung committed oder gepusht.

Dokumentcheckpoint `f129c91a` ist auf `origin/feat/brain-v2-g-20261007` bestätigt. Drei schreibende Git-Schritte für diesen Checkpoint einzeln: add, commit und Featurepush; dazu getrennte read-only-Diffprüfungen. Keine Main-Aktion. Das spätere Dokumentgate steht im Abschnitt 06:15; der G0-Produktcheckpoint und sein Gate stehen im Abschnitt 07:13.

## Bauphase ab 06:00

Plan und Baseline haben bestätigte Fertignachrichten. Bereichsführung übernahm die Baseline-Rückgabe als `G/BASELINE.md`. G0 und G-M sind mit disjunkten Brain-/Reasoner-Schreibbereichen gestartet. Exklusive Briefings: `G/BRIEFING-G0.md`, `G/BRIEFING-G-M.md`. G-P und G-K starten nach verifiziertem G0-Vertrag; G-V bleibt an E-Receipt, Rechenkern und Fs reinen Buildvertrag gebunden.

Eine Ergänzung ging an den tatsächlich belegten nativen G0-Agenten: geschlossener Toolport, serverseitiger Kontext und unteranfragegebundene Belegabhängigkeiten. Zustellung bestätigt, keine andere Session angeschrieben. Gate-CLI wurde über `--review --help` geprüft; der erste Aufruf ohne `--review` las Hook-STDIN und scheiterte, ohne eine Review auszuführen. Kontextmodus durfte die externe Gate-Skriptdatei nicht lesen; keine Settings- oder Berechtigungsänderung.

## Wache 05:55

Sheet und Bestandsrecherche sind fertig. Der Planworker läuft seit 05:42 und verwendet die abgeschlossenen Artefakte. Baseline-Artefakte zeigen Prüfungen und Integritätsauswertung bis 05:50; kein toter Worker belegt und kein Ersatz gestartet.

Baseline bisher: fmt und Compiler Exit 0. Striktes Retrieval-Clippy Exit 101 wegen `items after a test module`. Reasoner: 287 passed, 4 failed, 0 ignored, 3 filtered. Retrieval: Bibliothek 59 passed, 0 failed, 15 filtered; `chunked_retrieval` 3 passed, 15 failed. Weitere getrennte Kernel-, Serve-, Integrations- und Wiederholungsläufe sind protokolliert, werden nicht zu einer erfundenen Gesamtsumme addiert. Schlussbericht mit Ursachen steht aus. Kein grüner Gesamtbeweis.

Produktdateien gegenüber `bfda408c` unverändert. Dokumentcommit `96e6a8da` wurde auf origin gesichert. Sheet- und Bestandsberichte sowie diese Registeraktualisierung sind noch uncommittiert. Kein Gate, Main-Merge, Deploy oder Livebeleg.

## Ablage und Schutzbefunde

Hauptsteuerung 05:40 bestätigt eigene Worktree-Akten bei Isolationsschutz. Elternsession schreibt keine gemeinsame Checkout-Akte. Zuvor gestartete Worker behalten ihre ursprünglichen getrennten Bereiche; Rückgaben wurden mit nativen Editierwerkzeugen in die eigene Akte übernommen. TODO bleibt unangetastet.

Ein früher Stop-Hook hielt den frisch angelegten Branch mit bloßem Main-Ausgangsstand für erledigt. Aktive Arbeit wurde erhalten; seit `96e6a8da` besitzt der Branch einen eigenen gepushten Dokumentcommit. Kein Cleanup aktiver Arbeit. Ein Nachrichtenzustellversuch an den angenommenen Namen `sheet-modell` scheiterte; keine Zustellung, kein `ListAgents`, keine fremde Sessionkoordination.

## Verbindliche Verträge

E besitzt Import und gemeinsamen `brain_storage::asset_mirror`-Leser. Aktuelle Spielwerte kommen aus vollständigen erfolgreichen Originaldokument-Runs je Clientversion, nicht aus Snapshots oder Katalogprojektionen. Es zusätzlicher globaler Mechanikdaten-Leseschnitt fehlt bisher. F besitzt Planer, Publish-Abnahme, Confidence und `dbrain-reasoner/src/data.rs` samt Manifest; keine uncommittierte Fremdarbeit kopieren. A besitzt Übergangsfreischaltung, Bot-Consumer und Invites.

G teilt Rechenkern und Provider-/Kernel-Werkzeugloop in getrennte Schreibpakete. Gemeinsame Typverträge zuerst, Ansicht und Integration nach bestätigten Abhängigkeiten. Ein Schreiber je Produktdatei. Native Worker delegieren nicht weiter. Bei Gate-BLOCK erhält ein frischer Fixer die Funde, nicht der Implementierer.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-g-v2-20261007
