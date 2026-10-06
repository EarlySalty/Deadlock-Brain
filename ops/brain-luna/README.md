# GPT Luna mit dem vorhandenen Abo

`deadlock-brain-luna-proxy.service` startet die bereits installierte Rust-Brücke
`claude-code-proxy` 0.1.43 als eigenen Benutzer-Dienst auf `127.0.0.1:18769`.
Die bestehende Codex-Anmeldung mit ChatGPT-Abo bleibt der einzige Auth-Weg.
Modell, Denktiefe und Werkzeugverbot kommen aus dem Brain-Adapter.

Die geprüfte Brückenrevision ist
`49b2c85d61d675217982f85b6ab982b4ae531bc1` des Tags `v0.1.43`.
`server.rs` bindet ohne Konfigurationsoverride an Loopback.
`providers/codex/model_allowlist.rs` würde geerbte Modellvorgaben vorziehen.
Die Unit entfernt diese Vorgaben und alle weiteren Anbieter-, Konfigurations-
und Transport-Overrides. Die normale gemeinsame Brückenkonfiguration darf
keine abweichenden Vorgaben enthalten. Vor Installation ihre Abwesenheit
prüfen, ohne native Auth-Dateien zu lesen.

`registry.rs` ordnet `gpt-6-luna` dem Codex-Anbieter zu.
`providers/codex/translate/request.rs` überträgt `output_config.effort=low`
und `tool_choice.type=none`. Der Brain sendet zusätzlich `tools=[]`.
Dadurch gibt es auch kein Websearch-Werkzeug, das einen Modellwechsel auslösen
würde. Die Antwort nennt nur das angefragte Modell; als Laufzeitnachweis dienen
die festen Felder Modell, Ereignis und HTTP-Status des nativen Proxylogs.
Keine Anfragen, Antworten, Header oder Auth-Werte aus diesem Log ausgeben.

Nach Installation `systemctl --user daemon-reload` und
`systemctl --user restart deadlock-brain-luna-proxy.service` ausführen.
Den tatsächlichen Listener auf `127.0.0.1:18769` und `/healthz` prüfen.
Anschließend den Providerblock aus
`config/codex-subscription-provider.example.json` mit dem vorhandenen
ConfigWriter übernehmen, Brain neu starten und echte Spielantworten mit
Quellen sowie einen unbelegten Negativfall prüfen.

Andere Brückeninstanzen und ihre Modelle behalten ihre eigene Konfiguration.
