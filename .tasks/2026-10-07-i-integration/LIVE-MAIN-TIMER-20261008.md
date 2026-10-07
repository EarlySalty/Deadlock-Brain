# Tatsächlicher Live-Main- und Timerabgleich

Auftrag NACHTRAG-I-LIVE-MAIN-TIMER.md vom 08.10.2026 regulär gelesen. Genau den ausdrücklich freigegebenen bisherigen Fremdbaum /home/nathanael/.worktrees/brain-live-main geprüft, keine Archivarbeit übernommen und keine fremde Sitzung verwaltet.

## Zulässiger Fast-forward

Vorher main auf 39710e3282c830ee9b47e90945deb71d1db44724, tracked und untracked sauber, 575 Commits zurück. Ignorierte Artefakte erhalten: data/cache mit einem Unterverzeichnis und zwei Symlinks, keine regulären Dateien; rust/target mit einem Unterverzeichnis und zwei Dateien, insgesamt 39.081.040 Bytes. Nur Metadaten gezählt, keine privaten Inhalte gelesen. Nachher dieselben Inventarzahlen, kein Cleanup.

Einzelne Git-Schritte mit literalem absoluten Worktreepfad: Status, HEAD, fetch origin, ancestry HEAD zu origin/main (Exit 0), Ziel-HEAD, merge --ff-only origin/main, Status, HEAD. Regulärer Fast-forward tatsächlich Exit 0, Hintergrundkennung bid3yhhn7. Frisch geholtes Ziel und tatsächlicher neuer Main-HEAD cf02c9a06d56aeab72cc1d685cc3256f00ed9b1a. Anschließend main gleich origin/main und weiterhin keine tracked/untracked Änderungen. Kein Reset, Stash, Force, neuer Build oder fremder WIP-Eingriff. Kein Main-Push erforderlich, übernommen wurde der schon veröffentlichte Mainstand. Keine ausdrückliche neue Codekritikerantwort aus der bloßen Exit-0-Meldung erfunden.

MERGEPROTOKOLL[MS-1]: 8 Git-Schritte einzeln | Anläufe: 1 | Gate: regulärer ff-only zugelassen, Exit 0; main tatsächlich auf cf02c9a0

## Geladener Timer verwendet bereits dauerhafte Runtime

Tatsächlich geladene deadlock-brain-build-data.service und Datei stimmen überein: WorkingDirectory /home/nathanael/.local/share/deadlock-brain/build-data-runtime-b7289d11, ExecStart /usr/bin/bash in deren scripts/run_build_data_with_infisical.sh. Einziger Drop-in /home/nathanael/.config/systemd/user/deadlock-brain-build-data.service.d/timeout.conf setzt TimeoutStartSec=2h; kein ExecStart-/Pfadoverride. LoadCredential bleibt unverändert, keine Environment-Einträge. Timer active/waiting, nächster Lauf 08.10.2026 03:30 CEST.

Dauerhafte rust/target/release/deadlock-brain-Bindung löst auf /opt/deadlock-brain/releases/b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2/bin/deadlock-brain auf. current und maintenance-current bleiben b7289d11. Tatsächlicher CLI-SHA256 ec24337a09a8e9fa5f699736cf2dd172098275edce4ec6ac9fa01e4038a79b9e, gleich zum früheren Releasebeweis. Vorhandene Skripthashes unverändert: run_build_data_with_infisical.sh b04d31e766f9e5efa514efb3d07a3f921226fa55d7fa06646a1c6ac929dff4da, export_infisical_env.py 5d2422c1336615f17f7bc691d9de92e685d46d6ba9d7fac896ed3566a57e5a78.

Die Annahme, der geladene Timer verwende noch den alten Worktree ohne Spiegel, trifft damit nicht zu. Der stale Maincheckout war real und ist jetzt aufgeholt; der operative Importpfad war schon zuvor unabhängig davon am gelieferten installierten Spiegel. Keine Rückumstellung oder neue Unit-/Skriptänderung. Die neue Checkout-SHA cf02c9a0 ist nicht die ausgeführte CLI-SHA b7289d11.

## Neuer regulärer Lauf: tatsächlicher Start

Vor Lauf MainPID=0, inactive/dead, Result=success, vorheriger vollständiger Abschluss 00:05:51 CEST mit Exit 0. Alte Assets-/Build-Runbelege 743/744 bleiben erhalten und werden nicht als neuer Lauf ausgegeben. Vorhandene rein lesende Rust-Receiptprobe über unveränderten regulären Credentialweg für aktuelle Ausgangsbindung tatsächlich Exit 0, 1min 55.807s, Hintergrundkennung be7lfpo1y, vollständiges Log /tmp/brain-i-timer-preflight-20261008.log mit normalem Read geprüft. Ausgang weiterhin Assets-Run 743 vollständig und Build-Run 744 ok, 40 Heldeneinträge; alle ursprünglichen Receipt-/Originalhashbindungen erneut bestanden. Dies war kein Import und kein neues Importwerkzeug.

Unmittelbar erneut kein aktiver Dienstjob bestätigt und genau einen regulären systemctl --user start deadlock-brain-build-data.service gestartet, Hintergrundkennung bwgbl2t1g. Tatsächlicher Start 08.10.2026 01:37:11 CEST, MainPID 3731259, activating/start, noch kein ExecMainExitTimestamp. Result=success und ExecMainStatus=0 während dieser aktiven Phase sind kein Gesamtabschluss.

Tatsächlichen Kindprozess über /proc und ausschließlich Prozess-/Binarymetadaten geprüft, ohne Argumente oder Umgebung zu lesen: PID 3731293, exe /opt/deadlock-brain/releases/b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2/bin/deadlock-brain, ohne deleted. SHA256 ec24337a09a8e9fa5f699736cf2dd172098275edce4ec6ac9fa01e4038a79b9e entspricht geliefertem Release. Prüfung 07.10.2026 23:37:35 UTC. Zum Startzeitpunkt noch kein neuer Assets-/Build-Run oder Gesamtabschluss behauptet, keinen zweiten Import gestartet.

## Tatsächliches Ende und neue Runbindung

Genau dieser Dienstjob am 08.10.2026 01:44:12 CEST beendet, somit 7min 1s. Hintergrundmeldung bwgbl2t1g Exit 0 und tatsächliche systemd-Metadaten: ExecMainCode=1 (CLD_EXITED), ExecMainStatus=0, Result=success, MainPID=0, inactive/dead. Fehlerjournal ausschließlich 01:37:11 bis 01:44:12 CEST mit -p err: 0 Einträge; keine Journalinhalte ausgegeben. Timer active/waiting für 03:30, dauerhafte Binarybindung nachher unverändert b7289d11.

Vorhandene rein lesende Rust-Postrunprobe im regulären Credentialweg bkwl2twom tatsächlich Exit 0, 1min 56.010s. Vollständiges Original /tmp/brain-i-timer-postrun-20261008.log normal Read geprüft, keine Originalpayloads oder Secrets. Neue tatsächliche Assets-Run-ID 746, status ok, mirror_complete=true, Clientversion 6762, Parserrevision dbrain-assets/4. Manifestdokument 57265, tatsächlicher Originalhash 46c41542132ab6069ff1f34f1d7fa00d798ce5aee870f52f217cab0e8c9c0900. Alle 13 Endpointladungen und Manifestdatei gegen tatsächliche originale Dateibytes, Datenbankhash und Receipt geprüft; gemeinsame Readerladung gleich rungebundenem Payload. Dauerhafte Originalpfade unter /home/nathanael/.local/share/deadlock-brain/raw/deadlock_assets_api/.

Nichtleere Ladungen jeweils Englisch/Deutsch: items 746/746, heroes 40/40, heroes_all 65/65, generic_data 27/27, npc_units 93/93, misc_entities 105/105; modifiers 113 ohne Sprache. Anschließender tatsächlicher build_data-Run 747 status ok, 40 Heldeneinträge. Alte Runs 743/744 mit Version 6759 bleiben ihre getrennten historischen Belege. Neue Clientversion 6762 ist kein behaupteter historischer Balancepatch. Ausgeführt wurde die gelieferte CLI b7289d11, nicht die inzwischen aufgeholte Checkout-SHA cf02c9a0. Kein neuer Produktbau oder zweiter Import.

LIVEBEWEIS[DV-1]: PID 0->3731293 (Import beendet) | exe ohne (deleted) während Lauf | journal -p err leer | Anker "mirror_complete" im unveränderten installierten CLI | Funktion: vollständiger Assets-Run 746, Version 6762, 14 Originalhashbindungen geprüft; Build-Run 747 ok mit 40 Heldeneinträgen | Ort: brain.source_runs id=746 und id=747

Der begrenzte I-Betriebsnachtrag ist damit tatsächlich abgeschlossen. F/G-Abnahme, Mainlieferung und Warden-Publish bleiben als eigener offener I-Gesamtteil bestehen.

F arbeitet unverändert im eigenen Baum weiter, Discovery-Scopebereinigung aus Wache 34 bleibt verbindlich. Kein zweiter F-Writer, keine neue T3-Sitzung, kein Discoverymerge oder Archivimport. Private Originale/Community-Rohdaten MUST NOT an Codiermodelle oder Git gehen. Secrets NEVER ausgeben.
