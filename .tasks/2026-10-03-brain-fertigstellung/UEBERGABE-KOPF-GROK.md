# Übergabe: Kopf-Rolle an Grok 4.7 (04.10.2026, ca. 21:10 Uhr)

Du übernimmst die Rolle der bisherigen Claude-Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2` (ab jetzt nicht mehr ansprechen, Limit erreicht). Du bist der **Kopf**: Du planst, priorisierst, prüfst und entscheidest. Du baust nicht selbst. Gebaut wird von Codex-Delegatoren (gpt-6.1-sol, high), die ihre eigenen Sol-Worker führen. Nur die Delegatoren sprechen mit dir.

Arbeitssprache mit dem Nutzer: Deutsch, echte Umlaute, keine Gedankenstriche, kurz und direkt. Erst das Ergebnis, dann das Nötige.

## Auftrag des Nutzers (wörtlich zusammengefasst)

- Das Deadlock Brain schnell und effizient fertigstellen. Der Kopf behält alles im Blick, verhindert Doppelbau und lässt **nur das Wichtige** bauen.
- **Nicht glauben, was Agenten berichten, sondern selbst prüfen** (Git-Stand auf origin/main, `readlink` der Release-Symlinks, `systemctl show`, Journal, Health-Endpunkte).
- Keine Bürokratie. Kurze Berichte, keine Zeremonie.
- Alle 50 Minuten eine Wache (siehe unten).

## Prioritäten (Stand Nutzer 04.10. nachmittags)

1. **Priorität 1: Spielwissen-Steckbriefe (D5).** Je Entität (Held, Fähigkeit, Item) ein Steckbrief mit aktuellen Werten plus Patch-Story (wann wurde was geändert). Quellen gebündelt: Patch-Historie (`brain.patch_changes`), Git-Spieldaten, später Wiki. Architektur: DB ist die Wahrheit (Fakten mit Gültigkeit Patch bis Patch). Steckbrief-Dokument im Brain und HTML auf der bestehenden Brain-Site `/brain` (Port 8087) werden daraus erzeugt. Rückwirkende Fragen laufen über die DB.
2. **Serverguide und Discord-Brain: live, Nutzertest offen.** Testfragen hat der Nutzer um 17:36 bekommen (Hilfekanal ohne Erwähnung, DM, Erwähnung, Twitch earlysalty). Wenn der Nutzer Screenshots schickt, Befunde an D1b geben.
3. **Build-Reasoner: mittlere Priorität.** Soll demnächst fertig werden (bedingte Item-Effekte gegen das echte Schadensprofil rechnen, keine handgetaggten Labels, Maßstab Warden-Build 779996, Befund `Deadlock-Brain/.tasks/2026-09-16-reasoner-item-zweck/BEFUND.md`, immer `--publish` und `hero_build_id` melden). Erst als neuen Delegator starten, wenn D5 Stufe 1 live ist (er braucht dessen Datenmodell).
4. **Zurückgestellt, nicht bauen lassen:** Replays und Demos, echte Steam-Spieldateien (Depot), Forum (`brain-forum-initial.service` bleibt failed), voller Serverguide über das MVP hinaus, Begrüßungsserien, Paten, Grafiken.

## Delegatoren (aktiv)

| Kürzel | T3-Thread | Bereich | Akte (Bericht `AN_HAUPT.md`, Steuerung `VON_HAUPT.md`, neueste Einträge oben) |
|---|---|---|---|
| D1b | `5efe9f27-ca6b-4713-94e0-9387f882bbf6` | Brain Welle 1: Integration, Deploy, Discord/Serverguide, Tageslauf. Sein W1 ist **alleiniger Brain-Integrator und Deployer** (Eingang `welle1/w1/EINGANG.md`) | `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/welle1/` |
| D5 | `6b670366-ae14-4fc8-bbfb-f6d1027a8371` | Steckbriefe mit Patch-Story (Priorität 1), Worker A/A3/A4/B/C/F-Fixer | `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-04-spielwissen-steckbriefe/` |

Fertig und gesettelt (nicht anschreiben): D2 `ea03e82a` (Community-Brücke), D3 `d4352296` (Harness), D4 `97de4d8b` (Betriebsfehler: Modell-Resolver, Wiki-Freshness, Branch-Watchdog, alle live grün). Alter D1 `707be38c` ist tot. Alte gestoppte Threads stehen in `KOPF.md`, nie wieder aufnehmen.

Regeln für Delegatoren: `DELEGATOR-REGELN.md` in diesem Ordner. Briefings: `welle1/DELEGATOR-BRAIN.md`, `../2026-10-04-spielwissen-steckbriefe/DELEGATOR-SPIELWISSEN.md`.

## Stand bei Übergabe (21:00 Uhr, selbst geprüft wo markiert)

- **Brain live:** `c0f4c3b` (CLI, serve, maintenance; Port 8788, healthz/readyz 200). Brain-main ist `022ed84` (D5-Grundstand, Gate ALLOW), W1 installiert ihn gerade per `sudo brain-release install`. Selbst prüfen: `git -C /home/nathanael/repos/Deadlock-Brain log origin/main --oneline -3` und brain-serve Release.
- **Bots live:** `/opt/deadlock/bots/current` → `b79862d4` (Serverguide-MVP seit 17:24, selbst geprüft). Discord-Zugriff des Brains läuft mit den Rechten der fragenden Person, ohne Identität nur Sicht verifizierter Mitglieder und nur lesend. Eigener enger Endpunkt, kein `api_call`, Export, Mitgliedersuche oder Moderation. Proaktiv nur Guild `1289721245281292288`, Kanal `1426220702054355077`.
- **Twitch:** Brain-Antwort im Chat vom Nutzer bestätigt (13:37). Provider Fireworks DeepSeek V4.1 Flash mit `reasoning_effort` none, Timeouts provider 55 s, request 60 s, Consumer 65 s.
- **D5:** Am 20:07 geschnitten. **Stufe 1** = Git-Werte aktueller Patch plus Patch-Historie, Steckbrief-Dokument im Brain, HTML auf `/brain`, drei echte Antworten (aktuelle Heldenzahl, Item-Frage, rückwirkend „Was wurde bei X im Patch vom 16.09. geändert?“), automatischer Durchlauf eines neuen Patches. **Stufe 2** = Wiki-Fakten, Wiki-Quittungen, A4, erweiterte Intervalle. Offen in Stufe 1: C registriert die Spielprofiltabellen im bestehenden Migrationsweg (`schema.rs`, `brain-migrate.rs`, `grants.sql`), Fixer F6 am letzten Fachbefund, dann Gate, Migration durch W1, Deploy, Antworten.
- **Entscheidungen, die schon gefallen sind:** Abgeleitete Git-Spielwerte dürfen öffentlich auf HTML; Spielzahlen, Patchänderungen und Wiki-Fakten dürfen an das bestehende Antwortmodell; Rohdateien, Pfade und wörtliche Wiki-Texte bleiben intern.
- **Tageslauf:** 04.10. 11:33 bis 05.10. 11:33. Danach (nicht vorher) die Legacy-Abschaltung vorschlagen lassen: Legacy-Shell `deadlock-brain-patchnotes-sync.timer` (User naniadm) erst abschalten, wenn der Rust-Weg `brain.patch_changes` belegt fortschreibt; `deadlock-brain-youtube-learning.service` ist failed (Altdienst, mit abschalten). Bisherige Tageslauf-Fehler stammen aus dem Twitch-/Bots-Betrieb (Raid 409, Auto-Raid ohne Ziel, Leave-Survey-DMs, Watchdog-Starttimeout 17:51), nicht vom Brain.

## Wache alle 50 Minuten

Ein systemd-Timer schickt dir alle 50 Minuten die Nachricht `[Wache]` in deinen Thread. Läuft gerade ein Turn von dir, fällt der Anstoß aus, dann beim nächsten Mal. Je Wache:
1. `date`, dann je Delegator `AN_HAUPT.md` (oberster Eintrag) lesen und `python3 ~/Documents/tools/t3-thread.py read --thread <id> | tail -c 2500`.
2. Selbst prüfen statt glauben: origin/main der betroffenen Repos (`git fetch`), `readlink /opt/deadlock/bots/current`, Brain-Release, `systemctl --user show <unit> -p Result -p ExecMainStatus`, Journal auf Fehler seit der letzten Wache.
3. Fragen beantworten: Eintrag oben in die passende `VON_HAUPT.md` schreiben, dann den Delegator kurz anstoßen (`t3-thread.py send --thread <id> "[Orchestrator] VON_HAUPT.md <Uhrzeit> lesen: ..."`, bei laufendem Turn `--force`; nur für Entscheidungen, nicht für Plauderei).
4. Steuern: Doppelbau verhindern (eine Hand je Pfad, W1 allein integriert und deployt Brain). Läuft etwas über mehrere Gate-Runden ohne Live-Stand, schneiden (kleinster brauchbarer Live-Stand zuerst). Toter Delegator (Kapazitätsfehler, keine Antwort über eine Wache) → Nachfolger mit gleicher Akte starten: `python3 ~/Documents/tools/t3-thread.py new --project Deadlock-Brain --model sol --effort high --title "..." "[Orchestrator] ... Akte lesen: <pfad>"`.
5. Dem Nutzer knapp berichten: was live ist, was offen ist, was er tun muss. Einen Eintrag `## Wache HH:MM Uhr` oben in `KOPF.md` ergänzen.
6. Fertige Delegatoren settlen: `t3-thread.py settle <id>`.

## Feste Regeln (nicht verhandelbar)

- Secrets nie im Klartext lesen, ausgeben oder speichern. Keine ENV-Dateien, Secrets nur über Infisical/FD.
- Produktiver Code nur Rust. Keine Code-Kommentare. Kein Modellwechsel in Bots/Diensten ohne Freigabe des Nutzers.
- Keine angewandte Migration ändern. Kein Force-Push, immer `git push origin HEAD:main`. Keine fremden Prozesse beenden, keine fremden Worktrees anfassen.
- Merge-Gate: `python3 /home/nathanael/Documents/.claude/gpt-workers/gate_hook.py --review --repo <wt> --base <origin/main> --head <HEAD>` (0 ALLOW, 1 BLOCK, 2 kein Urteil). Bei BLOCK geht die Liste an einen **frischen** Fixer-Thread, nie an den Autor.
- Host: Build-Slots nach `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md` (3 Slots, Release zusätzlich `/tmp/deadlock-cargo-release.lock`). Keine Build-Warteschlange per Nachricht.
- Community-Ankündigungen nur mit Go des Nutzers. Nutzer-/Community-Daten nicht an externe Anbieter, Ausnahme: lesbare Discord-Nachrichten über den bestehenden Fireworks-Antwortweg (vom Nutzer freigegeben).
- Kontingent-Sperren in `t3-thread.py kontingent` sind oft falsch: `kontingent freigeben` und das Modell direkt starten.
- Bei Rückfragen des Nutzers selbst entscheiden, wenn es keine echte Produkt- oder Architekturfrage ist, und die Entscheidung im Bericht nennen.

## Weitere Aufgabe: Branch- und PR-Durchgang (D6)

Das Übergabeprotokoll liegt in `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/UEBERGABE-GPT-BRANCHES-20261004.md`. **Nicht starten:** Den GPT-Lauf startet der Nutzer selbst. Du spawnst dafür nichts.

## Erste Schritte jetzt

1. Diese Datei, `DELEGATOR-REGELN.md`, die obersten Einträge beider `AN_HAUPT.md` und beider `VON_HAUPT.md` lesen.
2. D1b und D5 je eine Zeile schicken: „[Orchestrator] Der Kopf wechselt zu Grok-Thread <deine ID>. Akten und Regeln bleiben gleich, Bericht weiter in AN_HAUPT.md.“ (bei laufendem Turn `--force`).
3. Kein D6 starten (macht der Nutzer selbst).
4. Erste Wache machen und dem Nutzer den Stand berichten.
