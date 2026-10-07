status: Quellenvertrag geprüft, Senderlaufzeit offen
Datum: 2026-10-03

# Twitch: vorhandener Sender- und Antwortbeweisweg

Quelle: abgeschlossener lesender Workflow wf_a6a3e58f-d77, Werkzeug-ID w4altxi8d. 18 untersuchte Auth-/Helix-/Chatdateien entsprachen unveränderlichen Blobs am beobachteten Quellstand 84ce376c9ea7aab69116fc7399183032308d712e. Dieser Quellenabgleich ist kein vollständiger Bau-/Testnachweis für diesen Kopf. Keine Requests, Secrets, geschützten Laufzeitdateien, Änderungen, Compiler oder Nachrichten.

## Vorhandene Identitäts- und Sendebausteine

BotTokenManager::initialize in rust/crates/tb-chat/src/token.rs:293 validiert das Botcredential und nutzt bei Bedarf den authentifizierten Helix-/users-Weg. bot_user_id(), bot_login() und scopes() geben Identitätsmetadaten aus. try_build_api in rust/bin/tb-bot/src/chat_wiring.rs:612 verwendet DatabaseTokenStore und bindet die validierte Identität. Der bestehende Startlog bei Zeilen 717 bis 722 enthält nur Login, User-ID und Scopes. Fixturewerte sind keine produktive Identität.

HelixChatClient::resolve_user_id in rust/crates/tb-chat/src/moderation.rs:442 kann earlysalty über den vorhandenen botauthentifizierten Weg auflösen; alternativ besteht HelixClient::get_users in rust/crates/tb-transport-twitch/src/client.rs:343.

send_chat_message in rust/crates/tb-transport-twitch/src/chat.rs:295 unterstützt Usercredential und ausdrückliche Sender-ID mit user:write:chat. send_chat_reply ergänzt reply_parent_message_id. Die normale ChatApi und die Owner-Adminaktion aus rust/crates/tb-dashboard-api/src/handlers/admin_chat_action.rs:64 senden dagegen als Bot. Sie sind als Fragensteller ungeeignet, weil der Eingang eigene Botnachrichten ausschließt.

TokenProvider::get_valid_token_unrestricted_with_scope in rust/crates/tb-raid/src/token_provider.rs:66 unterstützt bereits gespeicherte Streamercredentials. Das vorhandene Uplink-Scopeprofil enthält user:write:chat; normale Basis-, Dashboard- und Titelprofile enthalten diese Berechtigung nicht, siehe rust/crates/tb-raid/src/scope_profiles.rs:37. Das belegt Quellunterstützung, keine tatsächlich verfügbare Senderidentität oder installierte geeignete Senderschnittstelle.

## Spätere enge Identitätsprüfung

Erst nach gemeinsamer Installation über den vorhandenen geschützten Authinhaber Kanal und vorgeschlagenen vorhandenen Sender auflösen. Sender-ID muss von der validierten Bot-ID abweichen. HelixClient::validate_user_token in rust/crates/tb-transport-twitch/src/user_token.rs:171 prüft Clientbindung, erwartete User-ID, Ablauf und tatsächlich erteilte Scopes. Credentials im vorhandenen geschützten Prozess halten; keinen credentialhaltigen Plattformtoken-Response ausgeben oder als interaktiven Ersatzweg abrufen.

Noch nicht produktiv belegt: earlysalty-User-ID, aktuelle Botidentität, vorhandener erlaubter Nicht-Bot-Sender mit gültigem user:write:chat, geeignete bestehende Senderschnittstelle, Partnerzulassung/Chatabonnement und gemeinsam installierte Brainkonfiguration. Falls das nicht belegbar ist, ohne neue Rechtebeschaffung oder Senderneubau stoppen und genau diesen Rest melden. TESTFREIGABE-TWITCH.md schafft keine Credentials.

## Minimaler späterer Test

Eine sachliche Frage genügt, erst nach gemeinsam geprüfter Installation und Identitätsprüfung:

`@<tatsächlicher-Bot-Login> Wo finde ich Informationen zu Community-Turnieren?`

Vorher den Platzhalter auflösen und prüfen, dass der gemeinsam importierte öffentliche Release eine passende Quelle enthält. Kein führendes !, Links, Flooding oder Moderationsexperiment. Keine Nachricht ist bisher gesendet.

ChatPipeline::handle in rust/crates/tb-chat/src/pipeline.rs:793 normalisiert Shared Chat, dedupliziert, schließt eigene und bekannte Bots aus, verlangt Partnerklassifikation und führt Moderations-/Spamprüfungen vor Brain aus. Zusätzlich gelten substanzielle begrenzte Erwähnung, nicht leere Message-ID, nicht stummer Kanal und verfügbares Kontingent. brain_chat.enabled mit typed-Modus aktiviert die Route global für berechtigte Partnerkanäle, nicht allein earlysalty.

## Korrelation und tatsächliche Grenzen

1. Tatsächliche EventSub-message_id, Sender und wirksamen Kanal erhalten. Der vorhandene Eingangsschreiber rust/crates/tb-chat/src/chatter_tracking.rs:264 persistiert in twitch_chat_messages.
2. rust/bin/tb-bot/src/brain_chat_wiring.rs:393 verwendet die Original-ID als Brain-Request-ID und Auditschlüssel. Frage und Antwort werden in public.tb_chat_brain_answers gespeichert, die Antwort vor dem Senden, danach der Zustellstatus.
3. Qs redigiertes Kernjournal anhand SHA256 der vollständigen ID korrelieren: twitch-bot, /v1/answer, passende gehashte ID, Status und Ergebnis. Das ist bislang veröffentlichter Vertrag, kein beobachteter Laufzeitbeweis.
4. Tatsächlichen Bot-Reply mit ursprünglicher parent-ID beobachten. Der vorhandene OBS-Ereignisweg in rust/bin/tb-bot/src/obs_dock.rs:193, :569 und :881 erhält Reply-ID und Elternreferenz in obs_dock_events. Wirksame Verdrahtung und tatsächliche Speicherung erst später belegen.

SendOutcome::Sent erhält Twitchs zurückgegebene Message-ID nicht. HTTP-Erfolg und Auditstatus Sent ersetzen deshalb den beobachteten Reply nicht. insufficient_evidence wird NoEvidence und erfüllt keinen fachlichen Erfolgsnachweis.

Weitere Grenze: rust/crates/tb-knowledge/src/brain.rs:121 projiziert Citationlabels, die Chatbrücke verwirft diese und entfernt Links. Der vorhandene Chataudit allein beweist damit keine auflösbare unterstützende Quelle. Dies ist an Haupt/Z zu übergeben, nicht durch einen fremden Kernpatch, neuen Logdienst oder die Behauptung eines noch nicht vorhandenen Belegs zu verdecken. Gemeinsame Abnahme muss einen zulässigen echten Quellenbeleg an genau der fachlichen Antwort ermöglichen.
