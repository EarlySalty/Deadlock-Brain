# Admin-Build-Labor (experimentell)

Das Twitch-Admin-Dashboard kann `dbrain_reasoner::lab` als auf einen Commit
fixierte Rust-Abhängigkeit verwenden. Es gibt keinen zweiten Scorer, keine
Python-Bridge und keinen öffentlich zugänglichen Brain-Dienst.

`lab::catalog` liest den Heldenkatalog, den Datenbank-Patch und die neuesten
Asset-Importzeitpunkte. `lab::build` lädt die bestehenden Reasoner-Eingaben und
ruft `plan_build` auf. Kampffenster, Kanal-Uptime, eingehender Waffenanteil und
optionaler Schadensdruck sind validierte Szenarioannahmen. Patch ist immer
`current`, KI immer aus. Der Aufruf schreibt keine Builds und bietet keinerlei
Publish-Parameter an. Unbekannte Request-Felder werden abgelehnt.

Die Antwort enthält dominante Familie und getrennte Varianten, Item-Scores
je Familien-ID, die aufgelösten Heldenfähigkeiten, Imbue-Ziele, Kauf-/Verkaufs-
Hinweise, Skillorder, Patch-Anwendungen und deren Belege. Fehlende Familien-
Scores dürfen im Verbraucher nicht durch die Scores der dominanten Familie
ersetzt werden. Scores sind keine Winrate und nicht die fertige Kaufreihenfolge.

Quellalter und fehlende Zeitstempel werden explizit ausgewiesen. Weniger als
100 belegte Nach-Patch-Matches, unbekannter Patchbeginn und historische
Modellfelder sind Warnungen, keine implizite Freigabe. Der SHA-256-Fingerprint
bezieht sich auf geladene Helden-, Item-, Snapshot- und Szenariodaten, nicht
auf die gesamte Population. Die Daten werden pro Anfrage neu geladen; der
Endpoint löst selbst keinen Datenimport aus. Ein neuer Patch-Tag ist kein
Nachweis eines neuen vollständigen Asset-/Populationsimports.

Der Verbraucher muss Admin-Authentifizierung, CSRF-Schutz für POST und eine
read-only DB-Verbindung erzwingen. Das Dashboard nutzt bevorzugt
`DEADLOCK_BRAIN_READONLY_DSN`, andernfalls den bereits autorisiert zugeführten
`DEADLOCK_CENTRAL_DSN`, mit `default_transaction_read_only=on` und begrenzten
Abfragezeiten. Credentials werden weder an Browser ausgegeben noch geloggt.
Fehlende Konfiguration oder Rechte ergeben einen sichtbaren Fehler statt
erfundener Demo-Builds. Keine DB-Rechte oder Secrets werden durch die Library
angelegt oder verändert.

Die maximal zwei CPU-Rechenplätze werden mit dem normalen Reasoner geteilt.
Ein abgebrochener HTTP-Aufruf gibt einen noch rechnenden Platz nicht vorzeitig
frei. Diese Begrenzung ersetzt keine Prozessüberwachung für pathologische
Berechnungen; die bestehende Planung ist nicht während `spawn_blocking`
unterbrechbar.

## Abnahmeumfang

Der Laborzugang erlaubt Tests des neuesten Integrationsstands. Er erteilt
**keine fachliche Gesamtfreigabe**, verändert keine bestehenden Holdout-Gates
und hebt keine Steam-Publish-Sperre auf. Offene Befunde des Integrationsstands
in `.tasks/2026-09-16-reasoner-item-zweck/BUILD-FAMILIES-FORTSETZUNG-20260917.md`
bleiben gültig, bis neue Messungen sie tatsächlich widerlegen.

Verifikation am 17.09.2026: Reasoner-Lib 264 bestanden / 16 ignoriert;
Retrieval-Lib 22 bestanden / 12 ignoriert. Clippy für beide Libraries und
Examples mit `-D warnings` erfolgreich. Übersprungene DB-Tests sind kein
Live-Datenbanknachweis. Fünf neue Labortests prüfen unter anderem ungültige
Szenarien, Null-Grenzen und die Ablehnung von Publish-/KI-/Patch-Parametern.
