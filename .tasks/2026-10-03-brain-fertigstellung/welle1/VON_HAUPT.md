# Von der Hauptsession an D1

## 05.10.2026, 17:03 Uhr: Stopp bleibt, kein neuer Auftrag

Journal selbst gelesen. 16:35, „start operation timed out. Terminating.“, Status 15. Die Mengen sind danach dieselben: 378 Profile, 0 Quittungen, kein HTML. Keine höhere Grenze, keine Sourcefixrunde, kein eigener Tick.

Prozess `1607722` seit 16:35 ist der Timer, kein Auftrag. Nicht anfassen und nicht beobachten als Arbeit. Warten.

Legacy bleibt an. Nichts abschalten.

## 05.10.2026, 15:23 Uhr: Stopp gilt, der neue Automatiklauf bleibt nur Beobachtung

Journal selbst gelesen. 14:35, „start operation timed out. Terminating.“, Status 15. Wieder keine Quittungen und kein HTML. Keine höhere Grenze, keine Sourcefixrunde, kein eigener Tick.

Der automatische Prozess `1091650` seit 14:35 bleibt. Nicht abbrechen. Er ist kein neuer Auftrag. Wenn er endet, Mengen, Quittungen und HTML melden und warten.

Legacy bleibt an. Nichts abschalten.

## 05.10.2026, 14:33 Uhr: Die Statusabfrage hing, der Lauf lebt

`systemctl show` antwortet hier sofort. Derselbe Wartungsprozess `577256` läuft seit 12:35:12, Zustand aktiv, Grenze 2h. Der Usermanager wartet in `ep_poll`, nicht in `cgroup_lock`. Health 200, `/brain` 404. Seit 13:42 kein Journalfehler.

Das war eine hängende Abfrage, kein Produktfehler. Kein Kill, kein Neustart, kein `daemon-reload`. Denselben Lauf weiter nur lesen. Wenn er endet, Mengen, Quittungen und HTML melden. Endet er nach 120 Minuten ohne Quittungen, stoppen und warten.

## 05.10.2026, 12:52 Uhr: Folgelauf bleibt, keine höhere Grenze

Journal selbst gelesen. 12:35:12, „start operation timed out. Terminating.“, Status 15. Die 120 Minuten sind ohne Quittungen und ohne HTML zu Ende. Keine Sourcefixrunde und keine höhere Grenze.

Der automatische Folgelauf seit 12:35:12 bleibt. Nicht abbrechen, keinen zusätzlichen Tick. Wenn er endet, Profile, Quittungen und HTML melden. Endet auch er nach 120 Minuten ohne Quittungen, wieder stoppen und warten.

Legacy bleibt an. Nichts abschalten.

## 05.10.2026, 12:02 Uhr: Legacy bleibt, der Sheet-Sync bleibt liegen

Das Tagesfenster ist zu. Nichts abschalten. Der Rust-Schreiber hat seit dem Cutover kein neues Patchereignis, die Historie bleibt 33.209, der Legacy-Timer hat um 11:33:06 ausgelöst. Der Tag hatte den 55-Minuten-Abbruch und den Serve-Neustart um 09:44. Quittungen und HTML fehlen.

Die zwei Fireworks-Meldungen für Abrams und Infernus sind gelesen: HTTP 404, Modell nicht vorhanden. Der Exit 0 des Sheet-Sync ist kein Erfolg. Keine Modellprobe und keine Bereichsänderung aus diesem Auftrag.

Der Wartungslauf seit 10:35:12 läuft weiter. Die Zwei-Stunden-Grenze ist geladen. Nicht abbrechen, kein Code.

## 05.10.2026, 11:12 Uhr: 55 Minuten sind die Grenze, kein Code

Journal selbst gelesen. 10:35:12, „start operation timed out. Terminating.“, Status 15. `TimeoutStartUSec` der Wartung ist 55 Minuten. Der Folgelauf seit 10:35:12 läuft noch. Serve bleibt `e56e075d`, Health 200, `/brain` 404.

1. Den laufenden Durchgang nicht abbrechen.
2. Nur `TimeoutStartUSec` von `brain-maintenance.service` von 55 Minuten auf 120 Minuten setzen, sodass der nächste Start die neue Grenze nutzt. Kein Code, kein Guard, kein Scope, kein Kalender.
3. Danach den normalen Lauf abwarten. Melden, ob Quittungen und HTML entstehen. Endet auch der 120-Minuten-Lauf ohne Quittungen, stoppen und die gespeicherten Mengen nennen. Keine Sourcefixrunde.

## 05.10.2026, 09:35 Uhr: Zwei Quellfreigaben, dann ein normaler Lauf

D5/C-F16 hat keinen Produktfehler gefunden. Der Operator sieht keines der 112 Originale, weil `internal_doc_scopes` in `/etc/deadlock-brain/maintenance.json` nur `internal_docs` enthält. Die vorhandene Prüfung `entity_profile_source_inaccessible` bleibt. Kein Code, kein Guard, kein Schema.

In dieselbe Liste, `internal_docs` bleibt drin, genau diese zwei Einträge:

- `source.review:deadlock-wiki-deadlock-data`
- `source.review:steamtracking-gametracking-deadlock`

Danach ein normaler Wartungslauf. Melden, ob Profile, Bindungen, Projektionen und Quittungen gespeichert werden und ob HTML entsteht. Wird ein Scope abgelehnt, stoppen und den Fehlertext nennen. Keine Sourcefixrunde.

Einmal nachsehen, ob die lokale C-F16-Probe eine neue Datei im produktiven raw_dir hinterlassen hat. Gefundenes nur melden, nichts löschen. Die Datenbank hat C-F16 nicht beschrieben.

## 05.10.2026, 08:43 Uhr: Profilabbruch geht an D5

Der wiederholte Live-Abbruch `ENTITY_PROFILE_REFRESH_FAILED` geht an einen frischen D5-Fixer. W1 startet keine Sourcefixrunde und keine weitere Importserie. HTML und die drei Antworten warten auf diesen Stand.

Der Timer ohne nächsten Zeitstempel während des laufenden Wartungstick ist der vorhandene Fünf-Minuten-Kalender, keine ausgefallene Uhr. Keine Timeränderung.

Bots `dd58c0af` ist der Gesprächsstand, nicht von W1. Die Invite-Erkennung ist nicht live.

## 05.10.2026, 06:13 Uhr: Cutover ist live

Selbst geprüft um 06:13: `origin/main`, beide Releasezeiger und die laufende Binary sind `e56e075d486a75f83f4954b58d8113588082d3f1`. `brain-serve` seit 06:10:58, healthz und readyz 200. `/brain` bleibt 404. Das Gate `/home/nathanael/.cache/brain-c-f15-gate.log` ist ALLOW, der NIT bleibt ungebaut.

Der Wartungsdienst ist seit 06:10 noch aktiv. Der nächste Timertermin fehlt, solange dieser Befehl nicht endet. Wenn der Installationsbefehl endet, den Timer wieder mit nächstem Termin laufen lassen. Keine zweite Installation und keine neue Sourcefixrunde.

Danach der eine Durchgang: Profile, Patch, HTML, Aktivierung und die drei echten Antworten.

## 05.10.2026, 05:24 Uhr: Nutzerfrage zum 16.09. bleibt leer

Der Nutzer hat um 05:23 den Bot im Discord erwähnt: „Was hat sich am 16.09. geändert?“ Der Bot hat geantwortet: „Dazu hab ich gerade nichts Genaues. Frag am besten direkt im Discord nach.“ Die Erwähnung kommt an. Der Inhalt ist die leere Ausweichantwort.

Das ist der offene Stufe-1-Beleg, kein neuer Discord-Defekt und keine neue Fixrunde. C-F15 bleibt beim Importfehler. W1 startet keine weitere Importserie. HTML und die drei Antworten warten auf den geprüften Stand.

## 05.10.2026, 05:16 Uhr: Importfehler geht an einen frischen D5-Fixer

Der Befund von 05:07 ist angenommen. `runner.rs:659` auf `origin/main` verwirft den Fehlertext und setzt nur `ENTITY_PROFILE_REFRESH_FAILED`. Der Fixer sitzt bei D5.

W1 startet keine Sourcefixrunde und keine weitere Importserie. Grenzen bleiben, `canonical_raw_dir` bleibt aus. Live `e036fbde` und Bots `879c53d6` bleiben. HTML und die drei Antworten warten auf den geprüften Importstand.

## 05.10.2026, 04:36 Uhr: Patchobjekte gehören W1, der Dienst ist live

Selbst geprüft um 04:36: `origin/main` und beide Releasezeiger sind `e036fbdea7236cac41033f3b7dffda76f31fd12e`. `brain-serve` läuft seit 04:35:13 mit genau dieser Binary. healthz und readyz sind 200. Der Wartungstimer ist wieder aktiv. Sein Lauf von 04:35:13 bis 04:35:27 endete mit Exit 0. `/brain` auf Port 8087 bleibt 404. Bots bleiben `879c53d6`.

Die drei fehlenden Objekte in der Brain-Datenbank auf Port 5446 gehören W1. Nicht D5, nicht W2.

1. `brain.patch_changes`, `brain.patch_events` und `patchnotes.changelog_posts` aus der bereits gesicherten Definition des bestehenden Patchwegs ins Schema `brain` bringen. Keine neue Migrationsdatei, keine Änderung einer angewandten Migration, kein zweiter Feed, kein DDL durch D5. Eingefrorene GameTracking-Captures bleiben draußen. Patchhistorie, die in der gesicherten Definition steckt, kommt mit.
2. Fehlt diese gesicherte Definition, dort stoppen und den gesuchten Pfad in einer Zeile melden. Kein selbst erdachtes Schema.
3. Danach der normale Grantlauf, SELECT für `brain_service` auf den Profiltabellen, der eine bestehende Rust-Patchweg, HTML auf `/brain` und die drei echten Antworten. Die Legacy-Shell `deadlock-brain-patchnotes-sync.timer` bleibt bis nach 11:33 und bis der Rust-Weg belegt schreibt.
4. Eine Zeile in `AN_HAUPT.md`: wo die Objekte lagen und welcher Befehl mit welchem Exit sie ins Schema `brain` gebracht hat.

## 04.10.2026, 23:32 Uhr: BLOCK bleibt bei D5, W1 wartet

Der BLOCK ist gelesen und an D5 gegeben. Frischer Fixer dort, nicht bei W1. W1 startet keine eigene Fixrunde und fasst die sechs Konfliktpfade nicht an.

Vor ALLOW des zusammengeführten Stands kein Main, kein Deploy, keine produktive Migration, keine Grants. Live bleibt `022ed841`.

Bots `879c53d6` seit 22:46 ist die getrennte D6-Coaching-Auslieferung, kein W1-Deploy. Nichts zurückrollen.

## 04.10.2026, 22:43 Uhr: C-Paket gegen aktuelles Main prüfen

Ja. W1 übernimmt `3bfe72166d68521bb172c79e966ce5db4cd7b34e` und führt das normale Gate gegen aktuelles `origin/main` `022ed8415e2b3437f4bd40e29ddbf4de630c97af`. Vollständiger Diff, keine Teilprüfung, keine dritte Runde gegen `8a88767`, kein zusätzlicher Reviewer.

Beide Exit-2-Läufe sind belegt: das Gate sah 1.412.087 Zeichen. Der vollständige Diff gegen das aktuelle Main ist 576.344 Byte und passt unter das Limit. Der Diff gegen die alte Basis ist 1.139.585 Byte und passt nicht.

Vor ALLOW kein Main, kein Deploy, keine produktive Migration, keine Grants. Danach der bestehende Weg: Migration, Grants, Deploy, drei echte Antworten. D5 legt denselben Stand in `w1/EINGANG.md`.

## 04.10.2026, 15:55 Uhr: Patchdurchlauf und Legacy-Patchnotes-Sync

D5 meldet: `deadlock-brain-patchnotes-sync.timer` (naniadm) startet weiter die Legacy-Shell, entgegen W2s Bericht. W1/P bestätigt bitte schriftlich in AN_HAUPT, welcher Weg heute `brain.patch_changes` fortschreibt (Rust-Feed `c256b22` oder Legacy-Shell) und hängt den D5-Rendereraufruf an genau diesen Rust-Weg. Kein zweiter Feed. Die Legacy-Shell wird nicht jetzt abgeschaltet, sondern zusammen mit der Legacy-Abschaltung nach dem Tageslauf (05.10., 11:33), sofern der Rust-Weg dann belegt schreibt. Reihenfolge sonst unverändert: Bots-Deploy 92e0b3ee, Brain-Deploy, dann Testfragen an mich, dann Serverguide-MVP.

## 04.10.2026, 14:40 Uhr: Neue Prioritäten des Nutzers

1. Der Serverguide ist sehr wichtig und soll zeitnah online gehen, damit das Brain grundlegend mit Discord getestet werden kann. Nach W5b und W6 baut D1b daraus ein **Serverguide-MVP**: W6 (DM und Erwähnung) plus proaktive Hilfe nur im bereits freigegebenen Kanal (Guild `1289721245281292288`, Kanal `1426220702054355077`), über denselben Kern und mit den Rechten der fragenden Person (Abschnitt 14:10). Den vorhandenen Guide-Code aus `/home/nathanael/.worktrees/serverguide-deploy-20261003/` wiederverwenden, soweit er passt; das Handoff dazu liegt in `/home/nathanael/Documents/.tasks/2026-10-03-serverguide/HANDOFF-OFFEN-2026-10-03.md`. Nicht dazu gehören dauerhafte Profile, Begrüßung bestehender Mitglieder, Kontaktserien, Grafiken und Paten-Vermittlung. Das bleibt später.
2. Neuer Delegator D5 „Spielwissen und Steckbriefe“ (Ordner `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-04-spielwissen-steckbriefe/`). Er liefert geprüfte Brain-Commits über `w1/EINGANG.md`. W1 bleibt alleiniger Brain-Integrator. Bitte D5-Eingänge zügig integrieren, sie haben Priorität 1.
3. Replays, Steam-Spieldateien und Forum sind zurückgestellt.

## 04.10.2026, 14:10 Uhr: Korrektur des Nutzers zu 13:55, Rechte der fragenden Person (ersetzt 13:55 Punkte 1 bis 3)

Der Abschnitt 13:55 war zu eng. Das Brain soll in Discord lesen und schreiben können, aber immer nur mit den Rechten der Person, die fragt. Neues Modell:

1. **Rechte je Frage:** Jede Anfrage trägt die Discord-User-ID der fragenden Person. dl-bot berechnet deren effektive Kanalrechte (Kanal sehen, Verlauf lesen, Nachrichten senden) aus Rollen und Overwrites und gibt nur frei, was diese Person selbst sehen bzw. tun darf. Das Modell erhält nie mehr als die Person.
2. **Ohne bekannte Discord-Identität** (Twitch-Zuschauer ohne verknüpftes Discord, unbekannt, Fehler): Sicht der normalen verifizierten Mitgliederrolle, nur lesend. Im Zweifel weniger, nie mehr.
3. **Lesen:** Kanalstruktur, Topics, Voice-Belegung und Nachrichten aus Kanälen, die die Person lesen darf. Die Nutzung dieser Inhalte über den bestehenden Fireworks-Antwortweg hat der Nutzer damit freigegeben. Gelesene Nachrichten fließen nur in die eine Antwort und werden nicht als Wissen gespeichert.
4. **Schreiben:** nur als Antwort auf die konkrete Frage und nur in Kanäle, in denen die Person selbst schreiben darf. Kein eigenständiges Posten.
5. **Technisch eingegrenzt, nicht per Prompt:** Das Brain spricht den Discord-Bot über einen eigenen Endpunkt an, der genau diese Werkzeuge mit Rechteprüfung anbietet. Der allgemeine MCP-Bearer mit `api_call`, `export_category`, `search_members` und Moderations- bzw. Löschfunktionen bleibt für das Brain unerreichbar.
6. **Tests:** Ein Mitglied ohne Mod-Rolle kann keinen Mod-Kanal lesen, ein Mod kann es. Eine unbekannte Person bekommt nur die Mitgliedersicht. Senden ohne Schreibrecht ergibt 403. Ein Ticket ist nur für die Person sichtbar, die es auch selbst sieht.

W5b (Sichtbarkeit, TempVoice-Lanes) und W6 (DM und Erwähnung in Discord) bauen auf diesem Modell auf. Die übrigen Punkte aus 13:45 und 14:00 gelten weiter.

## 04.10.2026, 14:00 Uhr: Neues Paket W6 Discord antwortet über das Brain

Nutzertest 13:41 Uhr: DM an den Discord-Bot „Kannst du mir helfen mich auf dem Server zurecht zu finden“, keine Antwort. Ursache: dl-bot ist nicht ans Brain angeschlossen, und die alte Concierge-/Passivhilfe wurde im Serverguide-Strang abgeschaltet.

W6, frischer Worker, nach oder parallel zu W5b, falls sich die Schreibpfade in Deadlock-Bots nicht überschneiden. Sonst nacheinander im selben Bots-Stand:
1. dl-bot beantwortet DMs und direkte Erwähnungen im Server über das Brain, mit demselben öffentlichen Bestand (`bot.public`) und denselben Discord-Live-Fakten wie Twitch. Bestehenden Brain-Client und die vorhandene Consumerbindung im Brain nutzen, keinen zweiten LLM-Weg im Bot. Falls für dl-bot ein Grant fehlt: W1 legt ihn über den ConfigWriter an.
2. Nicht dazu gehören: proaktive Hilfe, Begrüßungen, Profile, Paten-Vermittlung, Persona, Ticketzugriff. Das bleibt Serverguide, Welle 2. Der Inhalt von DMs geht nur als Frage ans Brain, wird nicht als Wissen gespeichert und nicht in öffentlichen Antworten verwendet.
3. Kurze, natürliche Antworten in Nanis Ton, gleiche Regeln wie bei Twitch. Limits pro Nutzer wie bei Twitch.
4. Bots-Deploy über den bekannten D2-Weg, danach eine Testfrage für DM und eine für Erwähnung an mich.

## 04.10.2026, 13:55 Uhr: Vorrang für W5b, Zugriff technisch eingrenzen (Nutzerentscheidung)

Der Nutzer will, dass das Modell gar nicht erst die Möglichkeit hat, Tickets, Moderation oder Ähnliches zu sehen. Ein Filter allein reicht nicht. Es muss durch die Zugriffsrechte unmöglich sein. Das gehört zu W5b, vor der Sichtbarkeitserweiterung:

1. **Eigener enger Zugang:** Das Brain darf beim Discord-Bot nur einen einzigen lesenden Endpunkt erreichen, der ausschließlich die öffentlichen Fakten liefert. Der Zugang des Brains darf serverseitig keine anderen Werkzeuge erlauben, also weder `api_call`, `read_messages`, `send_message`, `export_category`, `search_members` noch sonst eines. Heute nutzt das Brain laut W5 den allgemeinen MCP-Bearer (`TWITCH_INTERNAL_API_TOKEN`). Damit hat es Zugang zu allem. Lösung zum Beispiel: eigene Route wie `/mcp/public` mit fester Werkzeugliste im dl-bot und eigenem Zugang für das Brain. Ein neues Secret, falls nötig, direkt in Infisical anlegen, den allgemeinen Bearer aus der Brain-Konfiguration entfernen.
2. **Fail-closed im dl-bot, bevor Daten den Prozess verlassen:** sichtbar für die verifizierte Mitgliederrolle UND nicht in einer harten Sperrliste (Ticket-, Moderations-, Team-, Admin-Kategorien, per ID in der normalen TOML). Unbekannt oder nicht eindeutig heißt ausgeschlossen.
3. **Typ statt Disziplin:** Die Antwortstruktur des Endpunkts hat gar kein Feld für Nachrichtentexte von Nutzern, Nutzernamen oder Nutzer-IDs. Bot-Infotexte nur aus der bestehenden Liste eigener Publisher-Referenzen.
4. **Tests, die das beweisen:** Der Brain-Zugang auf `api_call` und `read_messages` ergibt 403. Ein Ticket-Kanal und ein Mod-Kanal sind ausgeschlossen, auch wenn die Mitgliederrolle sie sehen könnte. Ein Kanal nur für Verifizierte ist enthalten.

Danach geht es mit der Sichtbarkeit und den TempVoice-Lanes aus Abschnitt 13:45 weiter, über Gate und den bekannten Deployweg.

## 04.10.2026, 13:45 Uhr: Nutzertest bestanden, aber Live-Sicht zu klein (neuer Auftrag W5b)

Beleg vom Nutzer (Screenshot 13:37 Uhr, earlysalty): Der Bot antwortet als Reply: „Also laut Doku gibt's Casual-Lanes, Ranked-Lanes und Street Brawl als Lanearten. Aktuell siehst du auf dem Server außerdem die 🆕Neue Spieler Lane und die 🏆Coaching Lane als Voice-Kanäle, aber das sind nur die momentan sichtbaren Kanäle, keine Aufzählung der Lanearten.“ Damit ist der Twitch-Chatbeleg erbracht, W1 kann seinen Worktree nach SHA-Sicherung aufräumen.

Nutzerurteil: Das Brain hat noch keinen echten Zugriff auf den Discord-Server. Ursache: Der W5-Filter berechnet „öffentlich“ nur über @everyone. Wegen des Verify-Gates sieht @everyone fast nichts. Im W5-Briefing stand ausdrücklich „@everyone bzw. die normale Mitgliederrolle nach Verify“.

Auftrag W5b, frischer Worker:
1. Sichtbarkeit über die normale verifizierte Mitgliederrolle berechnen (Rolle aus der bestehenden Bot-Konfiguration bzw. dem Verify-Panel ermitteln, nicht raten). Ausgeschlossen bleiben alles, was diese Rolle nicht sieht, sowie Ticket-, Moderations-, Team- und Admin-Kategorien. Der Test aus W5 (Ticket- und Mod-Kanal ausgeschlossen) bleibt und bekommt einen Fall dazu: Kanal nur für Verifizierte ist enthalten.
2. Dynamische Lanes (TempVoice, Join-to-create) mitliefern: Kategorie, die Join-Kanäle und die gerade offenen Lanes mit Anzahl ohne Namen.
3. Antwortton: keine Meta-Sätze wie „das sind nur die momentan sichtbaren Kanäle, keine Aufzählung“. Einfach sagen, was es gibt und was gerade los ist.
4. Bots-Deploy über den bekannten D2-Weg, danach dieselbe Lanes-Frage intern prüfen und eine neue Testfrage an mich. Für den Tageslauf ist dieser eine Deploy in Ordnung, er wird im Tageslauf-Protokoll vermerkt.

## 04.10.2026, 11:10 Uhr: Abnahme gilt, Nutzer testet jetzt

Eine Antwort, die echte Lanes aus Live-Fakten nennt (Neue Spieler, Coaching, Ranked), ist eine brauchbare Antwort. Die Abnahme von 05:25 ist damit erfüllt, eine exakte Liste wird nicht gefordert. Der Nutzer stellt jetzt die Testfrage in earlysalty. W9 darf die Passagenauswahl eng fertig machen, das blockiert aber nichts mehr und bekommt keinen weiteren Umfang. Nach W9: Tageslauf starten.

## 04.10.2026, 10:20 Uhr: provider_error nach 56,9 Sekunden

56,9 s liegt genau über `timeouts.provider_ms = 55000` in brain-serve. Sehr wahrscheinlich also ein Provider-Timeout und kein HTTP-Fehler. Bekannte Falle aus `Documents/AGENTS.md`: DeepSeek bei Fireworks antwortet standardmäßig im Denkmodus. Mit großem Kontext (50 Dokumente, Live-Fakten, zwei Runden) dauert das sehr lange und kann das Ausgabebudget auffressen. Bitte zuerst belegen, ob brain-serve beim Provideraufruf `reasoning_effort` setzt und wie lange eine einzelne Runde dauert. Bestätigt sich das, ist freigegeben: Denken für die Antwortaufrufe des Brains abschalten (`reasoning_effort: none`, wie bei allen Judge-Aufrufen im Twitch-Bot). Das ist kein Modellwechsel. Modell, Budgets und Netzrunden bleiben. Danach dieselbe Lanes-Probe.

## 04.10.2026, 09:30 Uhr: 30-Sekunden-Abbruch bei der Lanes-Frage

Auffällig: brain-serve hat `timeouts.provider_ms = 55000`, `timeouts.request_ms = 60000` und jetzt `max_network_rounds = 2`. Die Consumer brechen nach 30000 ms ab. Mit zwei Providerrunden plus Live-Fakten kann eine ehrliche Antwort länger als 30 Sekunden dauern. Erst lesend belegen (Dauer der Lanes-Anfrage serverseitig, falls messbar). Bestätigt sich das, ist freigegeben: Consumer-Timeouts von Docs, Second-Brain und Twitch über die normale Config auf 65000 ms setzen, also knapp über `request_ms` des Servers. Das ist kein erfundenes Limit, sondern die Angleichung an den Server. Budgets, Modelle und Netzrunden bleiben, wie sie sind. Für Twitch ist eine Antwort nach rund einer Minute im Chat noch in Ordnung.

## 04.10.2026, 06:10 Uhr: D1 ersetzt, Bots-Deployweg

D1 `707be38c` ist um etwa 06:00 Uhr mit „Selected model is at capacity“ ausgefallen und wird nicht wieder aufgenommen. Der Nachfolger D1b übernimmt denselben Bereich, dieselben Dateien und die laufenden Worker W1-F2 `5c831e60` und W5 `4582e35a`.

Bots-Deployweg für W5: Es gibt keinen Wrapper in `/usr/local/bin`. Der bestehende Weg ist der, den D2 heute Nacht für Bots `b87f5d5a` benutzt hat: frischer Release-Build, root-eigen nach `/opt/deadlock/bots/releases/<sha>`, Symlink `/opt/deadlock/bots/current` umstellen, FD3-Units neu starten (`systemctl --user restart` für Bot und Web). Belege und genaue Befehle stehen in `/home/nathanael/repos/Deadlock-Twitch-Bot/.tasks/2026-10-03-community-abschluss-neu/` (`AN_D2.md` 01:02 Uhr, `bots-main-push.log` und die übrigen `bots-*.log`), die ausführenden Worker stehen dort in `REGISTER.md`. Genau diesen Weg wiederholen, keine neue Deployarchitektur.

## 04.10.2026, 05:35 Uhr: Nachtrag des Nutzers zu 05:25

- Ton (Punkt 2 von 05:25) nur leicht anpassen: Der Nutzer findet die Antwort „schon okay, nicht ultra bad“. Nur die Wörter „Belege“ und „Evidenz“ vermeiden und den Nichts-gefunden-Fall kurz halten. Kein großer Umbau.
- Punkt 1 (Discord-Doku in den öffentlichen Bestand) bleibt.
- Neu: Paket W5 Discord-Live-Fakten, Briefing `w5/BRIEFING.md`. Lesender Zugriff auf öffentliche Kanäle (Struktur, Topics, Voice-Zahlen ohne Namen, Infotexte unseres Bots), ausdrücklich ohne Tickets, Moderation, Team-Kanäle und Nutzernachrichten. Bitte einen frischen Worker starten, parallel zu Punkt 1, sofern sich die Schreibpfade nicht überschneiden.

## 04.10.2026, 05:25 Uhr: Twitch antwortet, aber die Antwort ist unbrauchbar (Vorrang)

Nutzertest 05:20 Uhr in earlysalty mit Release `4728e2f9`: Frage „Welche Lanes gibt es auf dem Discord-Server?“. Antwort kam als Reply, also ist der Chatpfad durch. Antworttext: „Die vorliegenden Belege enthalten keine Aufzählung der Lanes auf dem Discord-Server. ‚Lanes‘ werden nur beiläufig erwähnt … ist in der Evidenz nicht enthalten.“

Zwei Aufträge, beide Welle 1, sonst nichts:

1. Wissen: Der `bot.public`-Bestand hat nur sechs öffentliche Dokumente. Die öffentliche Discord-Server-Doku aus Deadlock-Docs (`public/discord-server/`, z. B. Lanes, Router, Rollen, Paten) gehört in den öffentlichen Bestand, über den bestehenden `brain-maintenance`-Weg für freigegebene Dokumentationsfassungen. Keine privaten Inhalte, keine Rechte umdeklarieren. Wenn die Seiten dort schon freigegeben sind und nur nicht übernommen werden, die Ursache finden und beheben. Danach neuen Release live, Docs, Second-Brain und Twitch auf denselben Release.
2. Ton: Antworten an Nutzer dürfen nie „Belege“, „Evidenz“, „Quellen liegen nicht vor“ oder Ähnliches sagen. Findet das Brain nichts Passendes, kommt eine kurze natürliche Antwort, z. B. „Dazu hab ich gerade nichts Genaues, frag am besten direkt im Discord nach.“ Der Code hat schon `no_evidence_reply`. Das Brain muss diesen Fall eindeutig signalisieren, statt eine Erklärung als `Answered` zu liefern, und sonst kurz und locker in Nanis Ton antworten (Bot-Text-Regeln aus AGENTS.md).

Abnahme: dieselbe Lanes-Frage ergibt eine echte Aufzählung der Lanes aus der Doku. Danach die Testfrage in `AN_HAUPT.md`.

## 04.10.2026, 04:48 Uhr: Tagesgrenze geklärt, Nutzer testet jetzt

Selbst gelesen: `/var/lib/deadlock-twitch/config/bot.toml` hat keinen Abschnitt `[bot.brain_chat]`, nur `[bot.brain_client]` mit typed, Endpoint und `bot.public`. Es gelten also die Code-Defaults: 60 s Nutzer-Cooldown, 20 je Kanal und Stunde, 500 je Tag. Nichts davon blockiert. Release `4728e2f9` ist live. Der Nutzer stellt jetzt die neue Testfrage in earlysalty. W4-F4 prüft danach die Bot-Antwort im Chat und die passende Zeile in `tb_chat_brain_answers` und meldet das Ergebnis.

## 04.10.2026, 04:20 Uhr: Eingrenzung für W4-F4 (bitte sofort weitergeben)

Selbst geprüft: `--brain-inspect` zeigt Bot `typed`, `bot_brain_chat_enabled: true`, Endpoint gesetzt. Damit baut `brain_chat_wiring::build` den Dienst, und die Pipeline erreicht Schritt 8 (Sus-Invite-Log um 01:26:17Z belegt). Der Abbruch liegt also in `BrainChatService::maybe_respond` (`rust/bin/tb-bot/src/brain_chat_wiring.rs` ab Zeile 368). Alle Ausstiege dort sind stumm:

1. `question_for_bot` liefert None, wenn `token_manager.bot_login()` nicht exakt `deutschedeadlockcommunity` ist (leer oder anderer Login).
2. `timeout_guard.is_muted("earlysalty")`.
3. `answerer` None (leerer Dienst-Token), Warnung läuft über `warning_budget` und kann gedrosselt sein.
4. `limit.reserve` None.
5. `log.begin` liefert `Ok(None)` (`user_count > 0` oder Kanal- bzw. Tageslimit) oder `Err` (gedrosselte Warnung).

Weg: je Ausstieg genau eine INFO-Zeile mit Grund (ohne Nachrichtentext, ohne Secret), Regressionstest mit dem exakten Nutzertext `@deutschedeadlockcommunity Wie erstelle und verwalte ich eine Lane auf dem Discord-Server?`, dann über Gate und den normalen Twitch-Deploy live. Danach eine neue Testfrage in `AN_HAUPT.md`. Zusätzlich `timeout_ms` im Bot-Client auf 30000 setzen oder den Default von 8000 erhöhen, weil das Brain für öffentliche Antworten rund 18 Sekunden braucht.

## 04.10.2026, 03:40 Uhr: Nutzer-Chatprobe ist gelaufen, keine Antwort (Vorrang)

Der Nutzer hat um 03:26 Uhr in earlysalty geschrieben: `@deutschedeadlockcommunity Wie erstelle und verwalte ich eine Lane auf dem Discord-Server?` Der Bot hat nicht geantwortet. Im Journal von `deadlock-twitch-bot-rust` steht nur `tb_chat::sus_invite ... result="no_action"` um 01:26:17Z, danach kein Brain-Aufruf. brain-serve hat in dem Zeitraum keine Anfrage bekommen. Nebenbei wurde brain-serve um 03:32 und 03:33 Uhr für die W1-Installation neu gestartet.

Auftrag an W4-F3 bzw. einen frischen Worker, jetzt vor allem anderen in Welle 1: herausfinden, warum der Chatpfad bei dieser Erwähnung nicht ins Brain geht (Modus legacy statt typed im Bot, fehlende Kanalzulassung für earlysalty, Erwähnungserkennung, Gate davor). Ursache beheben, über Gate und den normalen Twitch-Deploy live bringen und mir in `AN_HAUPT.md` eine neue Testfrage nennen. Nur eine Testfrage auf einmal, keine Rate-Runden mit dem Nutzer.

Discord gehört nicht zu Welle 1 (Bots sind zurückgestellt). Dort keine Arbeit.

## 04.10.2026, 01:50 Uhr: ein Twitch-Deploy

Der Abschnitt 01:45 stammt nicht von der Hauptsession und gilt nicht. D1 schreibt nicht in diese Datei.

Ein Deploy ist frei, genau `36d5157fa4fa99b8b2b12896c2d3d43a3bc56dec`, über den bereits installierten `/usr/local/bin/deploy-twitch-release`. Dieses Skript nicht ersetzen. Sein Hash bleibt `9bacf64f60394c96b0d09558817a55cfe08d6d8c8825ee2dcb488f9a1394788e`. Kein weiterer Commit und kein zweiter Cherry-pick. Die acht Migrationen nicht erneut anwenden, die beiden fremden Credential-Versionen bleiben unangewandt. Kein Rollback. Danach die bestätigten Brain-Felder nur über den vorhandenen Editor setzen. Die Nutzer-Chatprobe nur als Frage in `AN_D1.md`, kein eigener Chat.

Damit ist der Satz von 01:35, W4 deployt Twitch nicht erneut, auf diesen einen Release beschränkt aufgehoben. Der übrige Satz von 01:35 bleibt: kein Rollback, keine erneute Migration.

## 04.10.2026, 01:45 Uhr: ungültig

Nicht von der Hauptsession. Gilt der Abschnitt 01:50.

## 04.10.2026, 01:37 Uhr: Mechanik

Kein zusätzlicher Mechanikumfang in Welle 1. Der enge Zeitwert-Fix geht nicht hinein, er hebt die Low-Sperre nicht auf. Steam bleibt ohne Publish. Keine Grenzsenkung.

## 04.10.2026, 01:35 Uhr: Aktivierung, Scopes, Twitch

Patchnotes-Kandidat nicht aktivieren. `publication_allowed` bleibt false, `authorization_ref` bleibt unknown, `provider_egress_allowed` bleibt false. Der Adapter hat richtig abgelehnt. Keine Flagänderung und kein anderer Aktivierungsweg. Der live Rust-Dienst `c256b22` bleibt die Patchnotes-Oberfläche. Der Brain-Serve bleibt auf dem Maintenance-Bestand.

Docs und Second-Brain dürfen denselben öffentlichen Maintenance-Bestand über den vorhandenen Scope `bot.public` lesen. `docs.public` nicht anlegen, nichts importieren, keine private Fassung. Second-Brain bekommt keinen eigenen Datenstand.

Twitch live ist `2169533539557964e1d21fc71f2bc3cfabe97c1a`. Die acht Migrationen sind dort schon angewandt. W4 deployt Twitch nicht erneut und wendet sie nicht noch einmal an. Kein Rollback dieses Stands.

Steam bleibt ohne Publish, solange alle Builds Low sind. Keine Grenzsenkung.

## 03.10.2026, 23:28 Uhr: Gate nach dem Fix

Die Spanne bis `ba326b76fcd33dfd4a83f25653940ac389ff88b4` nicht erneut prüfen. Sie bleibt das alte BLOCK und kann den Fix nicht enthalten.

Nach dem Fixcommit genau ein Gate: `--base ba326b76fcd33dfd4a83f25653940ac389ff88b4 --head <Fix-HEAD>`. Wird die Eingabe wieder zu lang, an `2b20d4d154ef4b457640836aa7bf9ab30231bbb0` teilen und beide Teilstücke mit demselben Gate prüfen. Die volle Spanne ab `859852c` nicht schicken. ALLOW auf dieser neuen Spanne reicht. Der Fixer `4767ad74` macht das, kein zweiter Worker auf demselben Worktree.

## 03.10.2026, 23:00 Uhr: Patchnotes-Gate für den großen Stand

Die Claude-Sitzung der Hauptsession ist bis 01:30 Uhr am Limit. Weitere Antworten stehen in dieser Datei, nicht im Thread. Auftrag und Schnitt bleiben.

Das volle Gate von `859852c51316aa4cbf9b3a7018de62f363945d1e` bis `47654a6d6cea3d2b8e1ee37e6ead81982d8ab09a` hat 1.531.618 Zeichen geliefert. Codex nimmt höchstens 1.048.576 an. Dieselbe Spanne noch einmal zu schicken endet wieder mit Exit 2. Das Gate-Programm bleibt. Kein Einzelmodell, keine Umgehung, kein neuer Transport.

1. Ein Commit auf dem bestehenden Branch nimmt den Ordner `.tasks/2026-10-03-paket-p-rust/` vollständig heraus. Darin liegen Agentenakten, Logs und `host-compiler-probe.cjs`. Fonts, Rust, Cargo.lock, `docs/patchnotes-rust-betrieb.md` und die Unit-Drop-in bleiben.
2. Danach zwei Aufrufe des bestehenden Gates, nacheinander. Beide müssen ALLOW sein.
   - `--base 859852c51316aa4cbf9b3a7018de62f363945d1e --head ba326b76fcd33dfd4a83f25653940ac389ff88b4`
   - `--base ba326b76fcd33dfd4a83f25653940ac389ff88b4 --head <HEAD nach dem Aufräumcommit>`
3. Liefert der zweite Aufruf wieder nur wegen der Eingabelänge Exit 2, diese Spanne an `2538b41f9a738a979068e4767e3684c0baef0d8e` teilen und beide Teilstücke mit demselben Gate prüfen. Die volle Spanne nicht ein drittes Mal schicken.
4. Erst wenn beide Urteile ALLOW sind, den Kandidaten ohne Aktivierung an W1 übergeben. Die Brain-Aktivierung bleibt bei W1.

Den laufenden Releasebau auf `2913bf1` nicht anfassen. Live zeigt weiter den Release `maintenance-6edd92…` und nur den Twitch-Grant. Die Release-Antwort von 21:58 Uhr gilt weiter. D2 und D3 liegen außerhalb dieses Bereichs.

## 03.10.2026, 21:58 Uhr: Antwort zu Release-IDs für Docs und Second-Brain

1. Docs: Der aktuell live gebundene, freigegebene Release steht in `/home/nathanael/.config/deadlock-brain/brain-serve.json` unter `release.id = maintenance-6edd9236639f9b06067a3b6b67fa410e5e668aa264db1b2baf28b7e1733940de` mit `knowledge_version = docs-6edd9236…`. Er kommt aus dem bestehenden `brain-maintenance`-Weg („freigegebene Dokumentationsfassungen übernehmen“). Diesen Release für den Docs-Grant (`docs.public`) binden, keinen neuen Kandidaten bauen.
2. Second-Brain: Einen eigenen Second-Brain-Datenstand gibt es in Welle 1 nicht, die Writer dafür (Sheet, YouTube) sind Welle 2. Bindet `credentials[second-brain/internal].release` und `internal_operator.release` atomar auf denselben Release wie Docs. Damit beantwortet Second-Brain vorerst Fragen aus dem öffentlichen Wissensstand. Das reicht für Welle 1, bitte nicht mehr bauen.
3. Nach dem W1-Deploy zeigt brain-serve vermutlich auf einen neuen Release mit Patchnotes. Dann beide Consumer auf diesen neuen Release ziehen, sofern er die Docs-Inhalte enthält; sonst bei 1. bleiben.
4. W2-Sprachgate: Der native deutsche Steam-Pfad plus Übersetzungsfallback ist richtig, weil das die bestehende Regel ist (deutsche Steam-Fassung direkt, Englisch nie roh). Nur das fixen, nichts drumherum.

D3 ist fertig. Danke für die knappen Meldungen, so bleibt es.
