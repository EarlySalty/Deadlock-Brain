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

## Neuer regulärer Lauf aktiv

Vor Lauf MainPID=0, inactive/dead, Result=success, vorheriger vollständiger Abschluss 00:05:51 CEST mit Exit 0. Alte Assets-/Build-Runbelege 743/744 bleiben erhalten und werden nicht als neuer Lauf ausgegeben. Vorhandene rein lesende Rust-Receiptprobe über unveränderten regulären Credentialweg für aktuelle Ausgangsbindung tatsächlich Exit 0, 1min 55.807s, Hintergrundkennung be7lfpo1y, vollständiges Log /tmp/brain-i-timer-preflight-20261008.log mit normalem Read geprüft. Ausgang weiterhin Assets-Run 743 vollständig und Build-Run 744 ok, 40 Heldeneinträge; alle ursprünglichen Receipt-/Originalhashbindungen erneut bestanden. Dies war kein Import und kein neues Importwerkzeug.

Unmittelbar erneut kein aktiver Dienstjob bestätigt und genau einen regulären systemctl --user start deadlock-brain-build-data.service gestartet, Hintergrundkennung bwgbl2t1g. Tatsächlicher Start 08.10.2026 01:37:11 CEST, MainPID 3731259, activating/start, noch kein ExecMainExitTimestamp. Result=success und ExecMainStatus=0 während dieser aktiven Phase sind kein Gesamtabschluss.

Tatsächlichen Kindprozess über /proc und ausschließlich Prozess-/Binarymetadaten geprüft, ohne Argumente oder Umgebung zu lesen: PID 3731293, exe /opt/deadlock-brain/releases/b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2/bin/deadlock-brain, ohne deleted. SHA256 ec24337a09a8e9fa5f699736cf2dd172098275edce4ec6ac9fa01e4038a79b9e entspricht geliefertem Release. Prüfung 07.10.2026 23:37:35 UTC. Kein neuer Assets-/Build-Run oder Gesamtabschluss behauptet; genau diesen bestehenden Job bis Ende nachhalten, keinen zweiten Import starten.

F arbeitet unverändert im eigenen Baum weiter, Discovery-Scopebereinigung aus Wache 34 bleibt verbindlich. Kein zweiter F-Writer, keine neue T3-Sitzung, kein Discoverymerge oder Archivimport. Private Originale/Community-Rohdaten MUST NOT an Codiermodelle oder Git gehen. Secrets NEVER ausgeben.
