status: aktiv
Datum: 2026-10-03

# Paket S: Arbeitsstand

Ziel: Einen echten Reasoner-Build über `brain.build_publish.v1`, den Steam-HTTP-Endpunkt und die Steam-GC-Warteschlange veröffentlichen. Abschlussnachweis ist `DONE` mit spielinterner `hero_build_id`.

## Eigentum und Start

- Steam: `/home/nathanael/.worktrees/steam-publish-fertig`, Branch `feat/steam-publish-fertig-20261003`, sauberer Start `4c5621763d5f01c96d7912400517c08aa1c40df1`.
- Brain: `/home/nathanael/.worktrees/brain-fertig-s`, Branch `feat/brain-fertig-s-20261003`, sauberer Start `511a347b653beba13c2bf130f4bead7a7196cc2a`. Schreibbereich ausschließlich Build-Publish in `brain-feeds` und zugehörige CLI.
- Auftraggeber: `43a4886c-e135-484b-838a-0512d224a634`. Statusproduzent ausschließlich `teil-s`, Paket `s`, Versuch `1`.
- Kein Schreiben in zentrale `REGISTER.md` oder `TODO.md`. Keine fremden Worktrees ändern.
- Merge nach Gate ALLOW, bestehender Deploy, Neustart, Live-Prüfung und eigenes Cleanup sind laut `AUFTRAG.md` autorisiert.

## Übernahme Versuch 2

Am 2026-10-03T15:46:53Z vorhandenen Stand übernommen. Beide HEADs entsprechen dem bisherigen Start. Steam enthält genau zwei geänderte Publish-Dateien, Brain genau fünf einschließlich Lockfile. Vorgängerprozess 2229378 hat bei zwei Startproben keine Nachkommen. Die fremde Session bleibt unangetastet.

Auftraggeber ist jetzt Codex /root, T3 `e6c19079-657e-4db9-80bd-8e1313e7f785`. Statusproduzent `teil-s2`, Paket `s`, Versuch `2`. GPT-6.1 Sol bleibt gewählt. UltraCode ist laut Session-Harness aktiv; die tatsächliche Workflow-Verfügbarkeit wird im nativen Lauf geprüft. Höchstens drei native Worker, gestaffelte Starts, keine neuen T3-Threads.

Eigentum umfasst die sieben Dateien des übernommenen Diffs. Seit Freigabe in VON_HAUPT.md um 16:21 UTC ist zusätzlich der nötige minimale Steam-Cargo.lock-Abgleich für die gebundene Prüfung erlaubt. Fremde Verknüpfung und Deadlock-Bots-Worktree bleiben unverändert. Keine eigene Umschaltung des Brain-Releasezeigers und kein main-Merge gekoppelter Änderungen. Z integriert und installiert gemeinsam. S liefert geprüfte Feature-SHAs, den belegten CLI-Installationsvertrag und nach Zs Deployment den echten HTTP-Publish-Nachweis. Eine erfolgreiche Prüfung aus Versuch 1 liegt nicht vor.

## Native Läufe Versuch 2

Native Session-ID aus dem Harness: `36b5f0c0-6349-4454-8cf4-c20d7a9e8fa0`. Workflow-Verfügbarkeit durch die tatsächlichen Läufe belegt. Die gespeicherten Assistant-Nachrichten aller fünf bisherigen Workflow-Agenten nennen `gpt-6.1-sol`; kein anderer produktiver Modellname wurde gefunden.

- Bestand und CLI-Installationsvertrag: `wf_de0b42a0-282`, beendet; kein passender SHA-Installer im benannten Bestand gefunden.
- Rust-/Security-Prüfung: `wf_e89b4844-2bd`, beendet; zwei konkrete HTTP-Befunde.
- Frische Intent-Abnahme: `wf_0114aa23-616`, beendet; drei weitere Publish-Abweichungen, lokale Abnahme verweigert.
- Erste HTTP-Fixrunde: `wf_806b96af-dd4`, beendet; Brain `148e1a58a485def587f763c76c9ec7a9ff06040f`, 29 Publishtests bestanden, Gate ALLOW, Featurebranch gepusht.
- Frischer Folgefix: `wf_67a90ad5-45a`, aktiv; drei Publish-Dateien geändert, eigener Prüfwrapper `3294685` mit wartendem `flock`-Kind `3294687`. Noch keine Nachlauftests.
- Steam-Lockprüfung: `wf_c3640305-ba5`, aktiver Nachlauf `a5971f3542909b788`; Prüfer `3120972` mit wartendem `flock`-Kind `3120985`. Beide Prüfer warten am 18:04-UTC-Snapshot auf denselben Hostlock-Inode `16006309`. Keine fremden Prozesse verändert.
- Unabhängige Lockdiff-Abnahme: `wf_2a889a29-6ad`, beendet; 35 nötige Einfügungen, keine Entfernungen oder bestehenden Versionssprünge. Abgenommener und aktuell gleicher SHA256: `85d00ffca8e41dee91a2c47a1142b827cb76fbb1707b473c9283645225da2357`. Das ist noch kein Steam-Testnachweis.

Sessiongebundene Statuswache `49e0c488`, alle 20 Minuten auf Minute 1, 21 und 41. Sie endet beim Sessionende, spätestens nach sieben Tagen, und wird bei der fachlichen Übergabe oder beim Abschluss gelöscht. Zwei Statusmeldungen waren verspätet; die Wache ergänzt deshalb die Phasenmeldungen.

Prüfdetails in `REVIEW.md`, Installationsvertrag in `DEPLOYVERTRAG.md`. Das erste rust-reviewer-Vorlaufproblem ist dort ausdrücklich als fehlender Prüfnachweis dokumentiert.

## Native Worker aus Versuch 1

Workflow `wf_f1638835-703`, Task `w9qq3076i`, Rolle Bestand, nativ im Claude-Code-Harness, geerbtes GPT-6.1 Sol, Effort high. Ultracode in der Hauptsession aktiv.

1. `bestand:steam`: nur lesende Steam-Recherche; Bericht `BESTAND-STEAM.md`.
2. `bestand:brain`: nur lesende Brain-Recherche; Bericht `BESTAND-BRAIN.md`.

Worker starten keine weiteren Agenten und ändern weder Produktivdaten noch Git-Stand. Schriftliche Briefings enthalten Ziel, Eigentum, Arbeitsstand, Beweisziel und Routing.

## Live-Ausgangsstand

Am 2026-10-03T14:14Z:

- `steam-bot`: MainPID `3423896`.
- `steam-core`: MainPID `3423858`.
- `steam-core-2`: MainPID `1329609`.
- Alle Steam-Units starten den bestehenden Infisical-Launcher und `/opt/deadlock/steam/current/steam-{bot,core}`. Die ältere Memory-Anleitung zum Build im Hauptcheckout ist überholt und wird nicht verwendet.

## Prüfvertrag

Vor Compilerstarts beide Hostsperren blockierend halten, frische NonZombie-Probe, höchstens zwei Jobs. Bestehende Publish-Tests mit tatsächlicher Anzahl protokollieren. Intent-Abnahme durch frischen nativen Subagenten, abschließend lokaler Merge-Gate. Kein zusätzlicher Review-Thread.
