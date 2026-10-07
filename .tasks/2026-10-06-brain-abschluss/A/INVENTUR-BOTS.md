# Inventur A-I3: Discord-Brain, Serverguide und Twitch

Stand: 06.10.2026, 22:21 CEST. Nur lesende Prüfung, kein Review und keine Delegation. Kein Post, Versand, Konfigurationswechsel, Merge, Deploy oder Neustart. Game-Invite-Code unverändert. Secrets nicht ausgegeben; MCP-Lektüre über den bestehenden Infisical-/FD3-Weg, ohne Dateiexport oder zusätzlichen Modellaufruf.

BESTAND[BS-1]: teilweise | Fundort: /home/nathanael/.worktrees/brain-antwort-20261006/rust/crates/dl-brain/src/brain_api.rs:133 | Anknüpfung: vorhandenen Antwortfix b08d9366 und den bereits gemergten gemeinsamen Discord-/Serverguide-Handler weiterverwenden; Twitch-Adapter und enger Discord-Faktenzugang bestehen

## Stände und Dienste

Quellbelege beziehen sich auf `git show origin/main:<Datei>` im jeweiligen absoluten Repo, nicht auf dessen abweichenden Arbeitsbaum. Graphify wurde vor der Bestandssuche global befragt, danach wurden die Quellstellen verifiziert.

| Repo | Beobachteter origin/main-SHA | Laufender Binary-SHA |
|---|---|---|
| `/home/nathanael/repos/Deadlock-Brain` | `d6131cc52711a3e8b02d299704244f8d7dbdbce6` | `e56e075d486a75f83f4954b58d8113588082d3f1` |
| `/home/nathanael/repos/Deadlock-Bots` | `600b832ac90b314512c60bff470f78b6914509dd` | `600b832ac90b314512c60bff470f78b6914509dd` |
| `/home/nathanael/repos/Deadlock-Twitch-Bot` | `67786ba2c0b740f964a337dd0d75c19e7958ef66` | `67786ba2c0b740f964a337dd0d75c19e7958ef66` |

`systemctl show` meldet `brain-serve.service`, Userdienst `deadlock-bot-rust.service` und Systemdienst `deadlock-twitch-bot-rust.service` jeweils `active/running`. Tatsächliche Prozessdateien lesend über `/proc/1327506/exe`, `/proc/1405741/exe` und `/proc/1184203/exe` geprüft:

- `/opt/deadlock-brain/maintenance-releases/e56e075d486a75f83f4954b58d8113588082d3f1/brain-serve`, Start 20:37:33 CEST.
- `/opt/deadlock/bots/releases/600b832ac90b314512c60bff470f78b6914509dd/dl-bot`, Start 20:52:30 CEST.
- `/opt/deadlock/twitch/releases/67786ba2c0b740f964a337dd0d75c19e7958ef66/rust/target/release/tb-bot`, Start 20:13:09 CEST.

Brain `/healthz` und `/readyz` auf `http://127.0.0.1:8788`: HTTP 200. Ready meldet Wissens-/Releasebindung `maintenance-rebase-bfb28d8367f50c8766bf5fd027b9a2a35d30d7207046085b6d9038894144575e`, nicht den Binary-SHA. Laufender Brain-SHA ist Vorfahr von main, Exit 0, aber nicht dessen Kopf. Kern-/Releaseabgleich gehört A-I1.

## 1. Discord-DMs und Erwähnungen: halb, gültige Antwortfälle kaputt

**Soll:** W6-Briefing vom 03./04.10., Ereignisidentität, gemeinsamer Kern und `bot.public`, gebundene Zustellung und Personenrechte. Nebenfehler-Briefing vom 05.10., Links und Überlänge dürfen eine gültige Antwort nicht zum Aussetzer machen.

**Ist:** Bots-main `rust/bin/dl-bot/src/main.rs:260,1059` verdrahtet den vorhandenen `BrainApiAnswerer` mit eigenem Discord-Zugang. `rust/bin/dl-bot/src/modglue.rs:684` reicht den echten Event-Autor weiter und prüft Antwortrechte vor und nach der Anfrage. `/home/nathanael/.config/deadlock-bots/bot.toml:58-61`: direkter Anschluss an, Endpunkt `127.0.0.1:8788`, 65000 ms. Journal 06.10., 20:52:33 CEST: „Direkte Discord-Brain-Antworten angeschlossen“.

**Offen:** `rust/crates/dl-brain/src/brain_api.rs:121-132` auf Bots-main verwirft `answered`/`build_rejected` weiterhin bei HTTP-Link oder mehr als 3800 UTF-16-Einheiten als Backend-Fehler. Das aktuelle Binary stammt genau von diesem main. Journal bestätigt echte Discord-Anfragefehler am 06.10. um 01:52:05 und 03:54:07 CEST, aber ohne Fehlerklasse oder Antworttext. Die konkrete Ursache des Sinclair-Falls ist damit nicht bewiesen.

**Vorhandener Fertigbau:** sauberer Worktree `/home/nathanael/.worktrees/brain-antwort-20261006`, HEAD `b08d9366b06360fa5eae87fbdf3d118154f42930`. `brain_api.rs:133` enthält Linkentfernung, zeichensichere Kürzung, leeren Wissensfall und klassifizierte Warnungen samt Fixtures. Vorfahrprüfung gegen Bots-main: Exit 1. Noch nicht gemergt oder live.

**Minimal und Schreibbereich:** vorhandenen Fix auf aktuellem main integrieren, reguläres Gate und Auslieferung. Nur `/home/nathanael/.worktrees/brain-antwort-20261006/rust/crates/dl-brain/src/brain_api.rs` beziehungsweise dieselbe Datei im eigenen Integrationsworktree. Kein Brain-Neubau oder Modellwechsel. Danach echte DM-/Erwähnungsantwort nachweisen; diese Inventur hat keine neue Frage gesendet.

## 2. Enger Discord-Faktenzugang: fertig verdrahtet

**Soll:** W5b/W6, eigener `/mcp/public`-Zugang und Personenrechte; unbekannte Identität nur eingeschränkte lesende Sicht.

**Ist:** Bots-main `rust/bin/dl-bot/src/mcp.rs:155-232` trennt öffentlichen Zugang und Ereignisheader vom allgemeinen MCP. `/home/nathanael/.config/deadlock-brain/brain-serve.json` enthält den Discord-Live-Zugang und vertrauenswürdigen Consumer `dl-bot`/`discord` mit `bot.public`. Netzbudget 2, Anfragezeitlimit 60000 ms, Providerzeitlimit 55000 ms. Modell unverändert `accounts/fireworks/models/deepseek-v4p1-flash`.

**Live-Beleg:** lesender `public_server_facts`-Aufruf mit dediziertem Zugang, ohne behauptete Nutzeridentität: HTTP 200, 47 erlaubte Kanäle, 15 Voice-Zähler, fünf Bot-Infotexte, null Nachrichten. Belegt die laufende eingeschränkte Sicht, nicht sämtliche individuellen Rechtefälle. Allgemeines MCP wurde nur als Verwaltungsleser für die Inventur genutzt, nicht im Bot-Antwortpfad.

**Minimal:** wiederverwenden, kein Schreibbereich nötig.

## 3. Serverguide-MVP: Code ausgeliefert, echte Antwortabnahme offen

**Soll:** MVP-Briefing, konkrete Hilfsfragen nur in Guild `1289721245281292288`, Kanal `1426220702054355077`, gemeinsamer Consumer; menschliche Hilfe und Gesprächsende respektieren. Keine Begrüßungsserien, Profile oder neue Modellrunde.

**Ist:** Bots-main `rust/bin/dl-bot/src/modglue.rs:395-434,616-655,684-749` enthält Kanal-/Frageauswahl, flüchtige Reservierung und denselben Consumer. `rust/crates/dl-brain/src/lib.rs:50-73` enthält 60 Sekunden pro Person, 20 pro Kanal/Stunde und 500 pro UTC-Tag. Lange Antworten werden bereits vollständig als Embed dargestellt (`modglue.rs:436`), nicht auf 2000 Einheiten abgeschnitten.

**Vorhandene Arbeit:** MVP-Commit `b79862d4d241be1e5a9efdf16831ad04798fd39c` aus `/home/nathanael/.worktrees/serverguide-mvp-20261004` steckt im aktiven Bots-main, Vorfahrprüfung Exit 0. Alten Komplettstand `/home/nathanael/.worktrees/serverguide-deploy-20261003/Deadlock-Bots` nicht zurückspielen. Der frühe W6-BLOCK-Bericht beschreibt ebenfalls nicht den heutigen Handler.

**Live-Lücke:** read-only MCP `read_messages`, letzte 100 Nachrichten im freigegebenen Hilfekanal, jüngste Nachricht 06.10., 20:04:52 CEST: kein fachlicher Guide-Antwortbeleg gefunden. Das beweist weder Defekt noch fehlenden Bedarf; Auswahlbedingungen, echter Eingang und Wirkung bleiben nachzuweisen. Historischer MVP-Bericht lässt die Nutzerabnahme ausdrücklich offen.

**Abgrenzung:** Journal „Automatische Brain-Hilfe inaktiv: Zielkanal oder zentraler KI-Connector fehlt“ betrifft den anderen Mitspieler-Suche-Pfad (`rust/bin/dl-bot/src/main.rs:1796-1801`), nicht diesen MVP. Daraus keinen neuen Guide-Schalter bauen.

**Minimal und Schreibbereich:** bestehenden MVP behalten und nächste echte passende Anfrage samt gebundener Antwort belegen. Erst bei nachgewiesenem Wiringfehler `/home/nathanael/repos/Deadlock-Bots/rust/bin/dl-bot/src/modglue.rs` im eigenen Worktree ändern. Derzeit kein belegter zusätzlicher Codebedarf. Erweiterte Guide-Serien bleiben zurückgestellt.

## 4. Twitch: bestehender Antwortpfad funktioniert

**Soll:** Welle 1, typisierter Consumer am gemeinsamen Brain, `bot.public`, bestehende Limits und natürliches Nichts-Ergebnis statt zweitem LLM-Weg.

**Ist:** Twitch-main `rust/bin/tb-bot/src/brain_chat_wiring.rs:493-549` und `rust/crates/tb-knowledge/src/brain.rs:92-125` enthalten Adapter und Zustellungsprotokoll. Bestehender Rust-Befehl `tb-config-check --config /var/lib/deadlock-twitch/config/bot.toml --brain-inspect`, Exit 0: Bot `typed`, Brain-Chat an, Endpunkt `127.0.0.1:8788`, 65000 ms. Dashboard `shadow` ist vom aktiven Chat getrennt, hier kein Aktivierungsauftrag.

**Jüngste echte Antworten:** Journal liefert keine Brain-Antworttexte. Ergänzend wurde ausschließlich lesend die aktive lokale PG-Tabelle `twitch_analytics.public.tb_chat_brain_answers` geprüft, ohne Fragen, Nutzerkennungen oder Kanalnamen auszugeben. Alle fünf Ereigniskennungen haben UUID-Form; keine neuen Proben erzeugt.

- ID 4, 04.10., 13:37:06 CEST: `Answered/Sent`, 6208 ms. Antwort nennt Casual, Ranked und Street Brawl sowie sichtbare Neue-Spieler-/Coaching-Kanäle.
- ID 3, 04.10., 12:22:38 CEST: `Answered/Sent`, 4894 ms. Antwort nennt Casual-Lane, Ranked-Lane und Street Brawl.
- Jüngster Eintrag ID 5, 05.10., 21:01:02 CEST: `NoEvidence/Sent`, 320 ms, „Dazu finde ich gerade keine sichere Antwort. Frag lieber im Discord nach.“ Zugestellter leerer Wissensfall, kein Transportausfall.
- ID 1 seit 04.10., 03:26:17 CEST bleibt `Pending/Pending`. Ursache nicht untersucht, keine manuelle Tabellenkorrektur. Nach dem aktuellen Neustart am 06.10. ist noch keine neue Brain-Anfrage protokolliert.

**Minimal und Schreibbereich:** keinen zweiten Antwortweg bauen. Wissensabdeckung gesondert mit Paket A prüfen; aktuelle Antwortwirkung bei nächster echter Anfrage belegen. Kein nachgewiesener nötiger Twitch-Sourcefix. Falls die alte offene Reservierung einen echten Wiederanlauffehler zeigt, bestehendes `/home/nathanael/repos/Deadlock-Twitch-Bot/rust/bin/tb-bot/src/brain_chat_wiring.rs` im eigenen Worktree untersuchen, keine DB-Korrektur von Hand.

## Übergabe

Unmittelbarer Fertigbau ist vorhandener Bots-Antwortfix `b08d9366…`. W5b und Serverguide-MVP nicht neu bauen. Erfolgreiche Twitch-Zustellungen und öffentliche Discord-Fakten sind belegt; Discord-DM-/Erwähnungsantwort und proaktive Guide-Antwort brauchen nach dem Fix noch einen echten Nutzerbeleg. Keine Compiler-/Testläufe oder Gate-Prüfung in dieser Inventur; frühere Testberichte sind keine neuen Prüfnachweise.
