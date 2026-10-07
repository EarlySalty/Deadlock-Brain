# Entscheidung K-Liveabschluss (07.10.2026, ca. 22:35 CEST, Nutzerauftrag „alles fertig bekommen“)

Antwort auf `K-LIVE-RESTBLOCKER.md`:

1. **K frisch im eigenen Worktree fortsetzen** (ausdrücklich beauftragt): wie bei I und G per `t3-harness new --worktree <K-Worktree> --branch <K-Branch>`, Bots- und Twitch-Artefakte übernehmen, alten Thread 79c97ab5 regulär stilllegen und als „nicht wieder aufnehmen“ registrieren.
2. **Brain-Release:** `/usr/local/libexec/brain-release` ist der reguläre Weg und bleibt unverändert nutzbar. Seine interne Release-Sperre ist dieselbe, die `cargo-slot` nimmt; `cargo-slot` betrifft nur Prüf- und Testläufe der Agenten. Keine Änderung am Releasewerkzeug nötig.
3. **Bots-Deploy:** Der belegte reguläre Weg steht in `.tasks/2026-10-06-brain-abschluss/AN_HAUPT-B.md` Punkt 5 (zuletzt so ausgeliefert: e1f11614 am 07.10. 01:23): Release-Stage `~/.local/state/bots-release-stage-<SHA>` nach `/opt/deadlock/bots/releases/<SHA>`, Rechte härten, temporären Symlink per `mv -T` atomar auf `current`, Neustart mit `bot-restart dl-bot web`, danach exe/SHA, Journal, Funktion prüfen. Genau diesen Weg nutzen, keinen neuen bauen. Gebautes Release 0fb873c6 (SHA256 700d9ab5…) jetzt so ausliefern.
4. **Discord-Livebeweis:** Kein Secret suchen. Bis der Nutzer ein Testkonto nennt, gilt als Beweis: Bot-Journal mit `Discord-Brain-Antwort empfangen` samt Status für echte Anfragen nach dem Deploy, Prozess-/SHA-Abgleich und eine Nutzerprobe (der Nutzer testet selbst im Discord). Offenen Testkontobeweis getrennt im Bericht führen.
5. Ortskontext danach direkt im frischen K-Thread.
