# Q: lokaler Quellen-Snapshot

Einmaliges Rust-Prüfwerkzeug für Paket Q, keine Produktpipeline und kein Antwortgenerator. Es verwendet den vorhandenen Discord-Verwaltungsadapter und den vorhandenen Postgres-CLI-Leser. Keine neue Provideranbindung, keine Modellprobe, keine Nachrichtenzustellung und keine DB-Schreibabfrage.

## Quellen

- `botlogs`: vorhandenes Discord-MCP, fünf Nachrichten pro Seite im beauftragten Kanal. Speichert nur als menschlich markierte Originalnachrichten. Ein menschlicher Beitrag wird dadurch noch nicht automatisch zum akzeptierten echten Evalfall.
- `dm`: `deadlock.bot.concierge_conversations`, ausschließlich Rolle `user`, bestehende Personen mit eingeschalteter Nichtverarbeitung ausgeschlossen. Das ist ein Gesprächsarchiv, kein vollständiger DM-Kanalverlauf.
- `twitch`: `twitch_analytics.public.twitch_chat_messages`, echte Nachrichtenreferenzen im UUID-Format, maximal 1000 zuletzt protokollierte Nichtkommandos. Bot-/Sprach-/Authentizitätsprüfung bleibt offen.
- `twitch-brain`: vorhandener Brain-Chat-Audit, Fragen mit UUID-Nachrichtenreferenz. Ein Auditdatensatz allein beweist keine erfolgreiche Zustellung.
- `public-heroes`, `public-items`: aktuelle öffentliche Endpunkte aus dem bestehenden Assets-Adapter. Keine privaten Eingaben im Request. Snapshots sind normalisierte JSON-Container, keine ungeänderten HTTP-Byte-Receipts.

## Datenschutz und Versionierung

`Q/private/` ist Git-ignoriert. Private Verzeichnisse müssen 0700 haben; neue Dateien werden mit 0600 und `create_new` geschrieben. Bestehende Snapshots werden nie überschrieben. Hashes binden die eingefrorenen Dateien. stdout enthält nur Mengen, technische Fehlercodes und Hashes. Originaltexte, Quellenpersonen und individuelle Nachrichtenreferenzen bleiben ausschließlich lokal.

Die einfachen Textmuster erzeugen **vorläufige Antwortarten**, keine Goldlabels. Unbekannte Fälle bleiben unbeschriftet. `accepted_gold_cases` bleibt deshalb 0. `plan` friert die vorläufigen Erwartungen mit Quellhash und Zeilenreferenz ein. `verify` prüft ausschließlich lokale Dateiintegrität und ist kein Brain-, Discord-, Twitch- oder P1-Funktionsbeweis.

Scheitert das Schreiben des Hashes nach dem JSON-Schreiben, bleibt die Datei absichtlich erhalten und die Version ungeprüft. `verify` bricht bei fehlendem Hash ab; derselbe Versionsname wird nicht überschrieben. Zur Wiederaufnahme zuerst den unvollständigen lokalen Stand erhalten, dann eine neue Quellenversion sammeln und deren Hashprüfung durchführen. Der alte Stand ist kein erfolgreich eingefrorenes Evalset. Diese Wiederaufnahme betrifft die Quellensammlung vor dem ersten Modelllauf. Ein später tatsächlich abgenommenes Evalset wird nach seinem ersten Lauf unverändert wiederverwendet, nicht durch eine neu gesammelte Version ersetzt. Private Originale werden zur Fehlerbehebung nicht gelöscht oder öffentlich übertragen.

## Befehle

Vom eigenen Q-Worktree:

```sh
/home/nathanael/.cargo/bin/cargo +1.97.1 build --locked --offline --jobs 3 --manifest-path /home/nathanael/.worktrees/brain-q-eval-20261007/.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml
```

Bestehenden privaten Teilstand unverändert prüfen, ohne Netzwerk oder Modell:

```sh
/home/nathanael/.worktrees/brain-q-eval-20261007/.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot verify q-partial-v1-20261007
```

Neue Quellenversionen erhalten jeweils einen neuen Versionsnamen. CLI: `<source> <version> [pages]`, anschließend `plan <version>`. Keine bereits existierende Version neu sammeln. Vierzig kleine Discordseiten wurden erfolgreich geprüft. Ein größerer Lauf mit 200 Seiten wurde vom äußeren Prüftimeout beendet und erzeugte keinen erfolgreich gemeldeten Snapshot.

Die vorhandene Python-Datei wird ausschließlich als unverändertes Verwaltungswerkzeug benutzt. Kein neuer Python-Code. Postgres-Abfragen sind statisch, laufen per bestehender Peer-Identität und mit `default_transaction_read_only=on`. Keine Tokens oder Passwörter in Argumenten, Dateien oder Konsolenausgaben.

## Aktuelle Sperre

Private Fragen dürfen nicht durch den konfigurierten Codex-Abo-Loopbackproxy geschickt werden. Dieses Werkzeug besitzt deshalb bewusst keinen Replay-/Modellbefehl. Die vollständige Ausführung gehört an den bestehenden Consumer, sobald die tatsächliche Datenflussgrenze freigegeben und belegt ist. Kein eigenmächtiger Anbieter-, Modell- oder Timeoutwechsel.
