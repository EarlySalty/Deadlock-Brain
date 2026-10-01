status: aktiv
Datum: 2026-09-29

# E: unabhängiges Review Runde 1

Geprüft: Steam PR #82, Codehead 6ead49768008208aac6851dbe12706ddc6820d8c, Basis 8c3fc6e0e8f5ab1c33a43a181c1eee5667818837. Reviewer Hauptsession Astra, Autor Luna. Status und Diffstat vor Bericht geprüft. Ein Produktpfad, 152 zusätzliche Zeilen, davon überwiegend Tests. Keine eigene Ausführung nach diesem statischen Befund; Autorenlauf ist separat in E-REPORT.md.

## BLOCK E-R1: ungefilterte Store-Fehler landen im Log

`rust/crates/steam-web/src/routes/builds.rs:275-281,311-317,360-366` protokolliert jeweils `error = %error`. Die öffentliche Portgrenze liefert `String`; der DB-Adapter wandelt Fehler über `e.to_string()` um (Zeilen 63-80). Es gibt keine Begrenzung oder sichere Klassifikation. Der neue Test beweist sogar, dass beliebige Detailmarker aus dem Store im Log erscheinen.

Konkreter Fehlerfall: Ein Storefehler enthält einen Nutzwert oder eine Verbindungsadresse im Fehlertext. Alle drei Grenzen kopieren diesen Inhalt in das Serverlog. HTTP bleibt zwar generisch, aber das E-FIX-Briefing verbietet vollständige DSNs, SQL-Parameter, Payloads und Credentials ausdrücklich auch in der internen Diagnose. Der reine HTTP-Negativtest reicht deshalb nicht.

Minimaler Fix: sichere Operation und kontrollierte Fehlerklasse protokollieren; falls aus dem typisierten Ursprung verfügbar, validierten SQLSTATE ohne Freitext verwenden. Keine beliebige Display-/Debug-Darstellung und keine fragile Ersetzung bekannter Secretmuster. Unbekannter Fehler fällt auf eine konstante Klasse zurück. Den bestehenden Test erweitern, sodass Payload-/Adress-/Detailmarker weder in HTTP noch in den erfassten Logereignissen stehen, während die drei Operationen unterscheidbar bleiben. Keine neue Retry-/Tasklogik und kein API-Vertragswechsel.

Fertig N, Fix nötig J. Kein Merge/Deploy oder echter Publish. Nach Fix dieselbe Datei, passende fmt/clippy/Tests und lokale Selbstprüfung erneut; danach unabhängige Nachprüfung dieses Befunds.

## Runde 2: E-R1 behoben

Geprüfter Head: 923f6a8702e4f277a06f823d6b82bc8621c6ef70. Änderungen 205bcf1, f623516 und 923f6a8 gelesen. Der Port erhält jetzt den vorhandenen typisierten PersistenceError statt beliebiger Strings. Logfelder sind Operation, feste Fehlerklasse und ein auf fünf ASCII-Großbuchstaben/Ziffern beschränkter SQLSTATE. Fehlernachrichten, Display/Debug des Fehlerobjekts und Requestinhalte werden nicht geloggt. SQLx wird dafür von Test- zur normalen bestehenden Workspace-Abhängigkeit; keine neue Version oder neue Quelle.

Drei unabhängige Gegenproben in der Hauptsession bestanden jeweils mit Exit 0 auf exakt diesem Head: `store_failures_keep_generic_http_responses_and_log_distinct_operations`, `store_error_diagnostic_keeps_only_a_validated_sqlstate`, `sqlstate_must_be_five_uppercase_ascii_alphanumeric_bytes`. Aufruf aus dem Steam-rust-Verzeichnis: `cargo +1.97.1 test -p steam-web --features testing --locked --offline -- routes::builds::tests::<testname> --exact`. Bereinigte Kindprozessumgebung enthielt ausschließlich HOME, PATH, CARGO_BUILD_JOBS=2 und SQLX_OFFLINE=true. Je ein Test ausgeführt, 87 gefiltert, kein ignorierter oder fehlgeschlagener Test; kein DB-/GC-Aufruf dieser Fixturetests. Arbeitsbaum blieb sauber. Die 88 Tests der Autorensuite sind separate Evidenz.

Frische GitHub-Prüfung des Heads: GitGuardian SUCCESS. Acht fehlgeschlagene Job-Annotationen melden ausdrücklich nicht gestartete Jobs wegen Kontozahlungen/Ausgabenlimit; Rust-Build-Job SKIPPED. Daraus wird kein Rust-/Security-Codefehler abgeleitet und keine vollständig grüne CI behauptet. Keine Policy geändert.

Urteil für E-R1: behoben. Technischer Diagnosefix GO, Fix nötig N.

## Integration ohne Betriebsumschaltung

Nach frischem Fetch lief das lokale Gate separat gegen origin/main und den festgelegten Head 923f6a8. Exit 0: `ALLOW: No blocking defects found in the supplied diff; error logging is bounded and HTTP error responses remain unchanged.` Danach PR #82 regulär auf ready gesetzt und mit `gh pr merge --merge --match-head-commit 923f6a8702e4f277a06f823d6b82bc8621c6ef70` integriert. Kein Admin-Override, keine Policyänderung. GitHub bestätigt MERGED am 2026-09-29 um 12:51:12 UTC, Mergecommit f509f85e4ec32da589c0f46aedc10fa3953b21fc.

Das Main-Verbot des Nutzerauftrags gilt Deadlock Brain; die zusätzlich ausdrücklich gesperrten Consumer-PRs bleiben unberührt. Dieser ausschließlich technische Steam-Providerfix wurde nicht deployt, kein Service neu gestartet und kein echter Publish durchgeführt. Vor dem Merge geprüfte Steam-Workflows enthalten Tests/Scans, keinen Produktionsdeploy. Die eigene Aufgabenakte wurde um diese präzise Abgrenzung korrigiert, keine Nutzerfreigabe hinzuerfunden.

MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 1 | Gate: ALLOW, Fehlerlog begrenzt und HTTP unverändert

TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: bestätigter Fehler aus Runde 1 geschlossen
