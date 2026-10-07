# Neuer Schnitt Deadlock Brain (ab 03.10.2026, 21 Uhr)

Hauptsession: T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2` (Claude, Projekt Documents). Ersetzt die alte Orchestrierung mit P/Q/R/S/K/Z, Statusrolle und Betriebsabgleich. Alte Threads stehen nach dem Haltepunkt (`HALT.md`) und werden nicht wieder aufgenommen. Ihre Arbeit liegt gepusht auf ihren Branches, ihr Stand steht in `bereiche/<x>/STAND.md`.

## Ziel Welle 1: Brain live mit dem, was Nutzer sehen

1. Patchnotes laufen in Rust und landen im Brain.
2. Gebaute Builds werden über den Steam-Bot veröffentlicht, mit `hero_build_id`.
3. Docs, Second-Brain und Twitch-Chat (earlysalty) beantworten Fragen über das neue Brain.
4. Brain-CLI und brain-serve laufen vom selben neuen Release.

## Agenten Welle 1

| Agent | Repo(s) | Auftrag | Übernimmt |
|---|---|---|---|
| W1 Brain-Integration | Deadlock-Brain | Einziger Owner des Brain-Repos: Integrationsbranch von origin/main, Commits übernehmen, prüfen, Gate, Merge, Release installieren (CLI und serve), live prüfen | Z (Installer/Adapter), S-Brain (3 Commits), Q (nur C9: e48c189, 5c220a8), P-Brain (1a5b2b3) |
| W2 Patchnotes | Deadlock--Patchnotes-Bot | Rust-Patchnotes fertig, roten Test und Clippy fixen, Gate, Merge, Deploy, alte Units umstellen, live prüfen | P |
| W3 Steam-Publish | Deadlock-Steam-Bot | Steam-Branch mergen und deployen; nach W1-Deploy einen echten Build veröffentlichen und `hero_build_id` melden | S (Steam-Teil) |
| W4 Consumer | Deadlock-Docs, Deadlock-2nd-Brain, Deadlock-Twitch-Bot, Deadlock-Bots | Docs und Second-Brain mergen und deployen, Twitch-Antwort fertig, nach W1-Deploy echte Frage und Antwort in earlysalty belegen | K |

Abhängigkeit: W2, W3 und W4 dürfen ihren eigenen Repo-Teil sofort fertig machen. Was Brain-Code braucht, geht als Commit an W1 (Datei `welle1/w1/EINGANG.md`), nicht als eigener Merge ins Brain.

## Welle 2: später, nicht jetzt

- Replay (R): echte Demo scheitert an `UnknownStructure`, kein Import.
- Q-Rest: Retention, Writer, Sheet- und YouTube-Units, Provider-Shadow.
- Wiki und Spieldateien (A/B/C3/D): Import, Steam-Depot-Download.
- Forum: Import und `brain-forum-initial.service` (failed).
- Serverguide komplett (Guide-Producer, V4/V5, Persona, Grafiken, Testbetrieb).
- Legacy abschalten erst nach einem fehlerfreien Tageslauf von Welle 1.

## Arbeitsregeln für alle Agenten (Bürokratie raus)

- Ein Agent, ein Auftrag, keine Unter-Orchestratoren. Native Subagenten höchstens für eine klar abgegrenzte Teilaufgabe.
- Früh und oft committen, Branch nach jedem grünen Schritt pushen.
- Prüfen nur die betroffenen Crates, nach `HOSTPROBE.md` (3 Build-Slots). Keine eigenen Wrapper, Manifeste, Hashlisten oder FD-Nachweise.
- Status in `welle1/<agent>/AN_HAUPT.md`, je Meldung höchstens 5 Zeilen: erledigt, als Nächstes, Blocker. Keine PIDs, keine Beweisgrenzen-Prosa, keine Status-JSONs.
- Steuerung der Hauptsession kommt über `welle1/<agent>/VON_HAUPT.md`. Vor jeder Meldung lesen.
- Vor dem Merge einmal `gate_hook.py --review` gegen den eigenen Stand. Bei BLOCK den Befund fixen und erneut prüfen. Danach nach main mergen, `git push origin HEAD:main`, über den bestehenden Weg deployen, neu starten, mit einem echten Funktionsaufruf live prüfen.
- Selbst entscheiden. Die Hauptsession nur bei echten Produktfragen, Geld, Datenverlust oder einem Konflikt mit einem anderen Agenten fragen.
- Secrets nur über Infisical/FD, keine ENV-Konfiguration, Rust only, keine Code-Kommentare, Nutzertexte mit echten Umlauten.
