status: erledigt
Datum: 2026-10-03
Belegstand: origin/main 511a347b653beba13c2bf130f4bead7a7196cc2a

# Vorhandene begrenzte Wiki-HTTP-Schnittstelle

A meldet in UEBERGABE.md einen unbegrenzt gelesenen Body über den alten `get_no_redirect`-Pfad. Das trifft diesen alten Pfad zu. Auf C-Integrationsbasis existiert bereits `HttpClient::get_bounded`; kein neuer Core-Helfer ist nötig. Der historische A-Ausgangsstand enthält diese neuere Schnittstelle nicht. Die tatsächliche A-Anbindung an den vorhandenen begrenzten Leser steht noch aus; die reine Existenz der Methode ist kein Fixnachweis für As bisherigen Aufruf.

Konkrete Fundstellen im C-Integrationsworktree:

- `rust/crates/deadlock-brain-core/src/http.rs:1-2`: Modul und öffentliche Optionen/Antwort reexportiert.
- `rust/crates/deadlock-brain-core/src/http.rs:108-117`: gemeinsamer Client ohne Redirects und automatische Dekompression.
- `rust/crates/deadlock-brain-core/src/http/bounded.rs:18-26`: `SourceHttpOptions`.
- `rust/crates/deadlock-brain-core/src/http/bounded.rs:81-83`: `get_bounded(&self, url: &str, options: SourceHttpOptions) -> Result<SourceHttpResponse>`.
- `rust/crates/deadlock-brain-core/src/http/bounded.rs:196-205` und `224-232`: Content-Length-Vorprüfung und tatsächliches Lesen von höchstens max_bytes plus einem Erkennungsbyte. Überschreitung wird Fehler, nicht abgeschnittener Erfolg.

A bindet diese vorhandene Methode bei der gemeinsamen C-Integration an. Für einen Wiki-Abruf `max_bytes = options.max_response_bytes`, `attempts = 1`, `request_timeout = Duration::from_secs(options.timeout_seconds)` und entsprechend begrenztes `total_timeout` setzen. Globale Obergrenzen bleiben 64 MiB pro HTTP-Body und 300 Sekunden Gesamtzeit; Optionen darüber sind ausdrücklich ungültig. `Accept-Encoding: identity` wird gesetzt. Komprimierte Antwortentitäten bleiben unverändert; nicht als JSON ausgeben oder unbeschränkt dekomprimieren.

`SourceHttpResponse` enthält `status: u16`, exakte `content: Vec<u8>`, ausgewählte sichere Header, echte `observed_at`, `attempts`, `url` und `content_type()`. A muss HTTP 401/403 sowie HTML statt JSON vor Normalisierung als Zugangsgrenze behandeln. Ein Versuch, keine Redirects oder automatische Wiederholung. Die bereits belegte Managed Challenge wird nicht erneut umgangen.

Für As lokalen Prüfharness ist ein ausdrücklicher Pfad auf den vorhandenen Core im aktuellen C-Integrationstree zulässig, ohne das produktive A-Manifest zu ändern. Produktive gemeinsame Registrierung und tatsächlich gemeinsame Compilerprüfung bleiben bei C. Die Methode verändert keine bestehenden Legacy-Aufrufer. Neue Core-Dateien oder Core-Änderungen sind für diesen Auftrag nicht erforderlich.

Nachweisgrenze: vorhandener Main-Code nach Graphify gezielt gelesen. Bestehende Tests für Content-Length und Chunked-Body, sichere Header, Redirects und Fehler sind vorhanden, in dieser C-Sitzung noch nicht ausgeführt. Keine Wiki-Anfrage durch diese Prüfung.
