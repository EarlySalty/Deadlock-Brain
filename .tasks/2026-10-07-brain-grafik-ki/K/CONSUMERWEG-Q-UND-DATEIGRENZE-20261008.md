# K: konsumierbarer Consumerweg und Eigentumsgrenze

Wache 37 vollständig konsumiert, 8. Oktober 2026. Keine zweite Messpipeline, neuer Connector oder ungefragter Providerlauf.

## Konsumierbare gesicherte Commits

Brain-readgate baf981f9146e69c9a2d270c915f45a444a2ed490 ist Bestandteil des tatsächlich bestätigten main 400381e681a2e08283db46094d1bbbde037e2813. Botsconsumer 8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8 liegt tatsächlich auf main und dem eigenen Featurebackup. Beide Sourcegates gpt-6.1-sol ALLOW. Aktivierung beider Prozesse wird separat in PRIVATFIX-LIEFERUNG-20261008.md fortgeschrieben.

## Bereits erlaubter reproduzierbarer Consumerweg

Produktiv bestehende Kette: answer_discord_event, handle_discord_query_with_read_access, answer_discord_query_with_read_access, BrainApiAnswerer, AsyncBrainClient::answer_for_discord_with_read_access, bestehendes POST /v1/answer. Kein eigener Providerconnector. Private Anfrage setzt x-discord-read-access: disabled; öffentliche Anfrage behält den bisherigen Wire. Personen-/Requestbindung bleibt intern, Frage bleibt unverändert, eine Anfrage reserviert genau einmal aus derselben Tagesquote. Rückgabe ist bestehendes PublicAnswerResponse, kein neuer Tooltrace oder Accountingexport aus K behauptet.

Reproduzierbare lokale Consumerregression auf Botscommit 8e1b8f03, ohne Provider-, Discord- oder DB-Liveaufruf:

```text
SQLX_OFFLINE=true /home/nathanael/.local/bin/cargo-slot +1.97.1 test --manifest-path /home/nathanael/.worktrees/bots-k-live-20261007/rust/Cargo.toml -p dl-bot --bin dl-bot privatfix_consumer_staff_thread_dm_binden_lesesperre_im_echten_wire --locked --offline --jobs 3 --no-fail-fast -- --include-ignored
```

Der tatsächliche bereits ausgeführte vollständige modglue-Lauf enthält diesen Fall und insgesamt 61 bestandene Tests. Der Einzelbefehl oben ist der konsumierbare vorhandene Fall, kein zusätzlich ausgeführter Lauf. Er benutzt echten Callback, produktiven Consumer, gepinnten SDK und 17 neutrale HTTP-Loopbackanfragen. Er prüft Öffentlich-/Staff-/PrivateThread-/DM-Wire, ursprünglichen Antwortort, Leseflag, unveränderten Fragetext, frische Anfrage-/Gesprächsbindung, can_reply-Ablehnungen und Widerruf vor Zustellung. Zustellport und HTTP-Dienst sind Fixtures, keine tatsächliche Nutzerantwort oder Modellbewertung.

Servernachweis auf Brainmain: vorhandener Filter private_read_gate in brain-api, brain-client, brain-contracts, brain-kernel und brain-serve über cargo-slot +1.97.1 test --lib --locked --offline --jobs 3 --no-fail-fast -- --include-ignored private_read_gate. Tatsächlich 5 passed, 0 failed, 0 ignored. Liveabruf vor I/O sowie beide Discordlive-Evidencefreigaben gesperrt, gespeichertes Wissen offen; Flightbindung trennt Readpolicy und Person. Produktionsservice besitzt keinen Toolport. Gs Accounted-/ToolExecution-Messnachweis nicht durch diesen Consumer-Wiretest ersetzen.

Q führt seinen vorhandenen Runner und seine vorhandene Providerkonfiguration fort. Tatsächlichen Modell-/Kanalvergleich erst gegen die separat belegten aktiven G/K-Prozesse. Keine Goldlabels aus Fixtures erfinden, keine privaten Originale oder neue kostenpflichtige Läufe durch K.

## Geordnete Dateigrenze nach Wache 37

Die zugestellte erste disjunkte Übergabe wird respektiert: V besitzt Brain rust/crates/brain-contracts/src/invite.rs sowie im eigenen Botsbaum rust/bin/dl-bot/src/mcp/self_invite.rs und den engen self_invite_status-Zweig in mcp.rs. K beschreibt diese Dateien nicht; verweigerte MCP-Quelldateien werden nicht über Ersatzpfade gelesen.

Keine neue gemeinsame Schreibfreigabe: brain-contracts/lib.rs, provider_input.rs, brain-api/lib.rs, brain-serve/discord_live.rs und Orts-/mechanische Begleitdateien bleiben für den erhaltenen aktiven Orts-WIP exklusiv K. modglue/brain_api/Tagesquote sind durch den gesicherten Privatfixcommit konsumierbar, werden hier aber nicht an V übergeben. V besitzt laut Wache 37 weder diese Consumerdateien noch Cargo.toml/Cargo.lock. Der spätere Libexport und weitere gemeinsame Deltas warten auf die gesicherte integrierbare Ortsrunde und geordnete Übergabe durch den Delegator. Kein WIP verworfen, kein fremder Stand kopiert.
