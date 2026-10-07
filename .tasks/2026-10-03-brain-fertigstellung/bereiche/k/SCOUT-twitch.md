status: erledigt
Datum: 2026-10-03

# Bestand Twitch-Consumer

Bestandsworker a65eb06eb6bbda5f5, Workflow wf_19c8af55-5fa, nach einmaligem Proxy-403 erfolgreich beendet. Die laufende Herkunft und der bestehende Verwaltungsweg wurden anschließend lesend geprüft.

## Bereits integrierter Stand

Eigener sauberer Kopf und beobachteter Release: 9a6356a679e05d0868f1178cf64a79c99b3e5105. PR #984 mit Merge 13321934f421b9a1cd81d0be8668c2e5a8fd925b ist darin enthalten. Zusätzlich liegt der echte Chat-Erwähnungspfad aus f3f00c7e bereits auf main. Beide behalten AsyncBrainClient mit Revision 3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef und bot.public bei. Kein Neubau des Consumers nötig.

## Verträge

- Dashboard: POST /twitch/api/v2/self-explainer/ask, Request-ID twitch-self-<pid>-<sequence>. typed fragt ausschließlich Brain; bei Fehlern sichere Unsicherheit statt lokalem Modellfallback. Der Aufruf protokolliert in der Datenbank und startet einen Discord-Logging-Relay. Deshalb kein wirkungsfreier Prüf-Endpunkt.
- Chat: sachliche Erwähnungen @<bot-login>, höchstens 500 Zeichen. Partnerprüfung, Moderation und Spamprüfung gehen voraus. Eigene Nachrichten und führende ! werden ausgeschlossen. Antwort ist eine Twitch-Reply; Request-ID ist die Twitch-Nachrichten-ID. Audit in public.tb_chat_brain_answers.
- Chat braucht bot.brain_chat.enabled und bot.brain_client.mode=typed. Dashboard verwendet dashboard.options.brain_client.mode=typed. Beide Endpunkte müssen zum laufenden Kern auf 127.0.0.1:8788 zeigen. Der globale Chatswitch gilt für alle zugelassenen Partnerkanäle; kein einzelner Brain-Kanalschalter belegt.
- Beide verwenden den bestehenden internen Dienstzugang TWITCH_INTERNAL_API_TOKEN aus Infisical. Kein Secretwert gelesen oder ausgegeben. Kernzuordnung bisher twitch-bot, Kanal twitch, bot.public.

## Beobachteter Betrieb

Dienste deadlock-twitch-bot-rust.service und deadlock-twitch-dashboard-rust.service laufen mit PIDs 1620085 und 1620199. /opt/deadlock/twitch/current zeigt auf den oben genannten Release. Installierte Artefakte haben passende .twitch_build-SHA und SHA256SUMS. /proc/<pid>/exe ist nicht lesbar; damit ist der vollständig geladene Prozessstand noch nicht bewiesen.

Die normale /var/lib/deadlock-twitch/config/bot.toml ist für diese Sitzung nicht lesbar. Aktive Modi sind deshalb unbekannt. Defaults und fehlende Logzeilen sind kein Beweis für legacy.

## Verwaltungsweg und Grenzen

Graphify zuerst, anschließend aktuelle Quellen geprüft: rust/bin/tb-bot/src/mcp.rs bietet nur Partner-, Pausen-, Ban- und Trennwerkzeuge. Es gibt keine Brain-Konfigurationssteuerung. admin_operating_config.rs nutzt tb_config::editor mit genau pool_max, acquire_timeout_ms und connect_timeout_seconds; fremde Felder werden abgewiesen. tb-config-check validiert die geschützte TOML, gibt aber keine Brain-Modi aus. Der vorhandene deploy-twitch-release-Wrapper serialisiert Releases und bietet keine belegt passende TOML-Umschaltung. Keine neue Diagnose-API oder Rechteumgehung gebaut.

Der bestehende In-App-Browser meldet keinen verfügbaren Automationshost. Kein verifiziertes Wegwerf- oder Testkonto für einen echten Twitch-Chatbeweis bekannt. Echte Streamerkonten bleiben unangetastet.

Offen: erlaubter enger Weg zum Lesen und Aktivieren der zwei Brain-Konfigurationsabschnitte, verifiziertes Testkonto, authentifiziertes Kern-Anfragejournal von Q/Z, tatsächliche Antwort und vollständiger Neustart-/Prozessbeweis.
