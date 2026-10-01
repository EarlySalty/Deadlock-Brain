status: erledigt
Datum: 2026-09-29

# Unabhängige C1-Nachprüfung der Gate-Nachträge

Geprüfter Head: 52ef6c5 auf fix/pre-g5-harness-20260929, PR #60 nach migration/rust-integration. Autor Luna, Reviewer Hauptsession Astra. Diff ead4a791..52ef6c5 zuerst geprüft, sauberer Worktree bestätigt, danach Bericht gelesen. Graphify zuerst, anschließend konkreter Testdiff vollständig gelesen. Keine Produktänderung durch den Reviewer.

Die Alias-Konfliktprobe läuft jetzt vor dem Widerruf. Zwei getrennte normale Fragen belegen die zugänglichen Warden-/Abrams-Werte 770/650 mit je einer Citation, danach liefert der gemeinsame Alias bei Limit 1 InsufficientEvidence. Die Widerrufsfälle bleiben bestehen. Der SCRAM-Test ist ausschließlich ein Scratch-DB-Test mit synthetischem Laufzeitwert, ohne Passwort-ENV. Richtige Anmeldung und SQLSTATE 28P01 bei falschem Wert werden getrennt geprüft; bestehende Prozess-Redaktionstests laufen zusätzlich.

Eigener Aufruf: `/home/nathanael/.worktrees/brain-pre-g5-harness-20260929/scripts/test_brain_serve.sh > /tmp/brain-closeout-c1-independent-52ef6c5.log 2>&1`. Exit 0. Sechs Tests bestanden, null fehlgeschlagen oder ignoriert, 26 gefiltert. Enthalten: zwei Legacy-Scratchfälle, echter Serve-Prozess-E2E, Scratch-SCRAM und zwei Prozess-Redaktionsfälle. Keine Produktion und kein echter Provider.

| Worker | Requests | Beantwortet | Clientfehler | Dauer ms | Reader-Verbindungen |
|---:|---:|---:|---:|---:|---:|
| 8 | 600 | 600 | 0 | 956 | 4 |
| 16 | 600 | 600 | 0 | 853 | 4 |
| 32 | 600 | 600 | 0 | 964 | 4 |

Poolmaximum und Peak 4, fünf erzeugte Verbindungen einschließlich Recovery, 9115 wiederverwendete Verbindungen. Der eine absichtliche Sättigungs-Timeout gehört nicht zu den Lastrequests. Wartebudget unverändert 150 ms. Dies ist ein C-Paketnachweis, noch kein finaler integrierter A+C-Nachweis.

Einschränkung ausdrücklich erhalten: SCRAM läuft gegen den SQLx-DB-Client, nicht als brain-serve-Passwortstart, weil dessen bisheriger Pfad eine hier verbotene ENV-Passwortübergabe erwartet. Produktiver Start bleibt im beauftragten privaten Peer-Harness. Kein synthetischer DB-Test wird als Nachweis eines geänderten Service-Credentialpfads ausgegeben.

Urteil für die beiden Gate-Nachträge: fertig J, Fix nötig N. C1-Nachtrag technisch GO. Lokales Merge-Gate für den gesamten C-Head wird separat ausgeführt; keine G5-/Brain-main-Freigabe.

TESTNACHWEIS[TW-1]: 6 passed, 0 ignored | Baseline: nicht erhoben; 0 failed, 26 filtered

## Integration

Lokales Gesamtgate gegen frisch geholtes origin/migration/rust-integration auf 305df2d und Head 52ef6c5: Exit 0, `ALLOW: No merge-blocking defect is established by the supplied diff.` Der verbleibende NIT benennt ausdrücklich die oben dokumentierte Grenze: SQLx-SCRAM ersetzt keinen Service-Passwortstart. Diese Grenze bleibt offen sichtbar; der beauftragte passwortfreie Prozesspfad ist nachgewiesen.

PR #60 regulär bereitgestellt und ohne Admin-Override, Policyänderung oder Branchlöschung nach migration/rust-integration gemergt. GitHub bestätigt MERGED am 2026-09-29 um 13:53:25 UTC, Mergecommit 4c962b83cc3e17c5525e91f90da8ed3ca718d01f. Kein Brain-main-Merge, Deploy oder Neustart.

MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 1 | Gate: ALLOW, dokumentierter NIT zum nicht ausgeführten Service-Passwortpfad
