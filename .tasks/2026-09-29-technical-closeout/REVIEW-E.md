status: aktiv
Datum: 2026-09-29

# E: unabhängiges Review Runde 1

Geprüft: Steam PR #82, Codehead 6ead49768008208aac6851dbe12706ddc6820d8c, Basis 8c3fc6e0e8f5ab1c33a43a181c1eee5667818837. Reviewer Hauptsession Astra, Autor Luna. Status und Diffstat vor Bericht geprüft. Ein Produktpfad, 152 zusätzliche Zeilen, davon überwiegend Tests. Keine eigene Ausführung nach diesem statischen Befund; Autorenlauf ist separat in E-REPORT.md.

## BLOCK E-R1: ungefilterte Store-Fehler landen im Log

`rust/crates/steam-web/src/routes/builds.rs:275-281,311-317,360-366` protokolliert jeweils `error = %error`. Die öffentliche Portgrenze liefert `String`; der DB-Adapter wandelt Fehler über `e.to_string()` um (Zeilen 63-80). Es gibt keine Begrenzung oder sichere Klassifikation. Der neue Test beweist sogar, dass beliebige Detailmarker aus dem Store im Log erscheinen.

Konkreter Fehlerfall: Ein Storefehler enthält einen Nutzwert oder eine Verbindungsadresse im Fehlertext. Alle drei Grenzen kopieren diesen Inhalt in das Serverlog. HTTP bleibt zwar generisch, aber das E-FIX-Briefing verbietet vollständige DSNs, SQL-Parameter, Payloads und Credentials ausdrücklich auch in der internen Diagnose. Der reine HTTP-Negativtest reicht deshalb nicht.

Minimaler Fix: sichere Operation und kontrollierte Fehlerklasse protokollieren; falls aus dem typisierten Ursprung verfügbar, validierten SQLSTATE ohne Freitext verwenden. Keine beliebige Display-/Debug-Darstellung und keine fragile Ersetzung bekannter Secretmuster. Unbekannter Fehler fällt auf eine konstante Klasse zurück. Den bestehenden Test erweitern, sodass Payload-/Adress-/Detailmarker weder in HTTP noch in den erfassten Logereignissen stehen, während die drei Operationen unterscheidbar bleiben. Keine neue Retry-/Tasklogik und kein API-Vertragswechsel.

Fertig N, Fix nötig J. Kein Merge/Deploy oder echter Publish. Nach Fix dieselbe Datei, passende fmt/clippy/Tests und lokale Selbstprüfung erneut; danach unabhängige Nachprüfung dieses Befunds.
