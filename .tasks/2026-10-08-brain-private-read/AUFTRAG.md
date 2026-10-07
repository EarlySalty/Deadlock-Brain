# K: private Discord-Anfragen ohne Nachrichtenreads

## Vertrag

ENTSCHEIDUNG-K-PRIVATFIX-0015.md und Wache 33 sind maßgeblich. Kleinster Schnitt: beide Öffentlichkeitssperren im Botsconsumer entfernen, can_reply und Antwort am ursprünglichen Eingang erhalten. Private Kanal-/Thread-/DM-Anfragen dürfen keine Discord-Nachrichten anderer Personen lesen, einschließlich Tool-/Evidencepfad. Eigene Frage, Spiel-/Server-/Dokuwissen und eigener erlaubter Invite-Status bleiben möglich. Öffentliche Anfragen unverändert. Keine neue Projektions-/Threadrechtevoraussetzung oder Game-only-Beschränkung.

## Enger gemeinsamer Anschluss

Bestehender SDK/HTTP/API/Context/Discordlivepfad muss die requestgebundene Reduktion transportieren und vor I/O sowie Evidence-/Toolfreigabe erzwingen. Vorschlag des bestehenden Workers: answer_for_discord_with_read_access(&Query, u64, bool), bei false x-discord-read-access: disabled. Tatsächliche Personen-/Actor-/Rechte-/Herkunftsbindung erhalten. Anfrage-/Cache-/Flight-Rückspielung darf die Restriktion nicht umgehen. Kein Query.answer_context oder pausierter Orts-WIP in diesen Kandidaten.

## Eigentum und Arbeitsstand

Einziger nativer Writer: a3d648bce719faf6c. Brainworktree /home/nathanael/.worktrees/brain-k-private-read-20261008, Branch fix/brain-private-read-access-20261008, Start b7289d11, frisch gefetchtes origin/main. Bestehende Client async_client.rs, API http.rs/lib.rs, interner DiscordRequestContext in contracts lib.rs, Serve discord_live.rs und passende Regressionen. Weitere mechanische interne Contextkonstruktoren oder zwingende vorhandene Cache-/Flight-Readpolicybindung konkret belegen und eng halten. Keine G-Port-/Deadline-/Reasoner-Semantik oder provider_input.rs-Projektionsänderung.

Eigene Primär-, Bots- und Twitchbäume samt pausiertem Orts-WIP bleiben erhalten. G darf ausschließlich die alte gesicherte provider_input.rs-Mergeauflösung in Gs eigenem Baum; später gesicherten geprüften Commit integrieren, kein fremder WIP. Keine Wartepflicht daraus.

## Beweis und Abschluss

Passende fmt-, clippy- und bestehende Testprüfungen über cargo-slot +1.97.1. Vollständige neue Logs von Anfang an im primären zulässigen K-Taskroot, nicht in gesperrten Logpfaden. Hauptsession-Botsbaseline bereits tatsächlich 21 passed, 0 failed, 0 ignored, Exit 0, unveränderte Botsquelle. Das ist kein Privatfix- oder Main-Hookbeweis.

Worker gibt zunächst den engen geprüften SDK-/Servicekandidaten ab, kein eigener Commit/Pin/Deploy. Primary fährt regulären Gate und Git einzeln mit absoluten Literalpfaden. Danach derselbe Worker im bestehenden Botsbaum auf gesicherter kompatibler SDKrevision: beide Guards weg, Readrestriktion durch bestehenden dl-brain-Pfad, Tagesquote und genau eine Reservation erhalten. Staff-/Thread-/DMregression muss Brainantwort am Eingang und 0 Discordlive-/read_messages-Zugriffe zeigen. Öffentlich unverändert. Regulärer Merge, aktueller origin/main-Deploy, Neustart und tatsächliche Nutzerprobe, kein verfrühter Gesamtabschluss.

## Routing und Grenzen

Native Hauptsession 47304059-5103-45b5-8e54-0fbb5f140555; Delegator 481426fe-b477-42b3-91c6-901811fcba1d. Keine neuen T3-Threads, Doppelwriter, eigenen Reviewer oder fremden Sessionkontakte. Keine Produktfrage wiederholen. Schutzablehnungen exakt melden, nicht umgehen; verweigertes mcp.rs und verweigerte Logs nicht anders lesen. Secrets NEVER lesen/ausgeben. Private Originale und Community-Rohdaten MUST NOT an Codiermodelle oder Git. Keine echten Testkonten oder produktiven DB-Schreibzugriffe. Moli statt Brave, persönlicher Browser und fremde Dienste unangetastet. Keine Modelle-/Timeoutwechsel, ai-coach oder Änderungen angewandter Migrationen.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-k-private-read-20261008
