# Paket G: Register

status: aktiv, 07.10.2026

## Bereichsführung

- Auftrag: `.tasks/2026-10-06-brain-abschluss/BRIEFING-G.md`; Haupt-Orchestrator `3fcd8f71-443e-48ae-825c-527eb52fbe56`, Kommunikation über die Akte.
- G-Thread laut Steuerung `a867ef50`; native Session `030a7b6f-d25c-482d-b66c-68185cd05dbb`, Modell `gpt-6.1-sol[1m]`, bestätigte UltraCode-Workflowstarts mit Effort `xhigh`.
- Worktree `/home/nathanael/.worktrees/brain-g-v2-20261007`; Branch `feat/brain-v2-g-20261007`; Basis `bfda408cb988722ddceadb56bca5b72e12d12731`, HEAD `96e6a8da`.
- Statusproduzent: Bereichsführung G. Kein zusätzlich vergebener zentraler Versuch; keine erfundene Nummer. Keine weiteren T3-Threads.
- Wirkung: Featurearbeit und lokale Prüfungen. Release-Hold sperrt Main-Push, Release-Build, Install, Neustart und produktiven Tick. Kein Settle bei offener Arbeit.

## Native Worker

| Worker | Schreibbereich | Belegter Start | Zustand |
| --- | --- | --- | --- |
| sheet-modell | Eigene `G/SHEET-MODELL.md`, `G/sheet/manifest.json`; Rohexport und Bilder im ursprünglich zugewiesenen gemeinsamen `G/sheet/` | Task `w6krhelmo`, Run `wf_f775f9c0-69b` | abgeschlossen, 13 Tabs, 5.307 Formelzellen |
| bestand:mechanik | Rückgabe zur eigenen `G/BESTAND-MECHANIK.md`; keine Produktänderung | Task `wyqxc1iva`, Run `wf_4e39a761-389`, Agent `a335322bab03367e9` | abgeschlossen, Bericht übernommen |
| bestand:antwort | Rückgabe zur eigenen `G/BESTAND-ANTWORT.md`; keine Produktänderung | Derselbe Workflow, Agent `a2e0399691fd75107` | abgeschlossen, Bericht übernommen |
| bestand:daten-abbau | Rückgabe zur eigenen `G/BESTAND-DATEN.md`; keine Produktänderung | Derselbe Workflow, Agent `a45d24611b1d8a737` | abgeschlossen, Bericht übernommen |
| baseline | Ursprünglich zugewiesene Rohbelege `G/pruefungen/baseline/`; eigene `G/BASELINE.md` aus Rückgabe | Task `wih3iu8uu`, Run `wf_ccd9af54-f71` | abgeschlossen, 474 passed und 38 eindeutige failed; 20 Fälle nicht abgedeckt |
| plan | Ausschließlich eigene `G/PLAN.md`; kein Produktcode, Git oder Runtime | Task `wdef9ng7t`, Run `wf_f79bff70-d4b`, Start 05:42 | abgeschlossen; Bereichsführung hat G0/G-M-Grenzen und finale Baseline eingetragen |
| G0:Werkzeugvertrag | Ausschließlich `brain-contracts/src/{lib.rs,provider_input.rs,tools.rs}` und Cratetests; eigener G0-Vertrag | Task `wgid0g5cj`, Run `wf_45c9b56b-428`, Agent `a3a054b7281fbde9e`, Start 05:56 | aktiv, kompatible Turns und Toolport/Abhängigkeiten |
| G-M:Rechenkern | Reasoner-Modelle, reine Konverter, Exports, Mechanik und vorhandene Simulation gemäß `BRIEFING-G-M.md`; Fs Loader bleiben unverändert | Task `win5xzl3o`, Run `wf_08b3462e-169`, Start 06:00 | aktiv, zuerst eigene Modellverträge, dann Rechnung |

Vollständige abgeschlossene Workflow-Rückgaben liegen unter `/tmp/claude-1000/-home-nathanael-repos-Deadlock-Brain/030a7b6f-d25c-482d-b66c-68185cd05dbb/tasks/`: `w6krhelmo.output`, `wyqxc1iva.output`. Die anfänglich ausgegebenen, damals nicht vorhandenen Transkriptordner sind kein Ergebnisnachweis.

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
