[Orchestrator] Paket A. Worktree /home/nathanael/.worktrees/brain-antwort-20261006, Branch fix/brain-antwort-nicht-verschlucken-20261006. Freigabe erst nach fremdem Prüfer. Kein Push, kein Deploy.

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/brain-antwort-20261006

# Paket A: Eine echte Brain-Antwort darf nicht als Aussetzer enden

Rolle: Blatt-Worker. Du startest keine weiteren Threads.

Auftraggeber: Kopf, T3-Thread 31575951-23e7-4dd9-b1af-8700f7ff45fe.
Bericht: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-05-nebenfehler/AN_HAUPT-A.md

## Ziel

Am 06.10.2026 um 01:52 Uhr hat der Live-Bot auf die Frage nach dem letzten Balance-Patch für Sinclair geantwortet: „Mein Hirn hakt grad. Probier's in ein paar Sekunden nochmal.“ Das Journal um 01:52:05 Uhr CEST sagt nur `Discord-Brain-Anfrage fehlgeschlagen`. Der Fehlerwert wird verworfen.

Fundstelle auf `origin/main`, `rust/crates/dl-brain/src/brain_api.rs`, Funktion `project`. Eine Antwort mit Status answered oder build_rejected wird nur durchgereicht, wenn der Text höchstens 3800 UTF-16-Einheiten hat und weder `http://` noch `https://` enthält. Jeder andere Fall, auch ein gültiger Text mit einem Link, wird zum Backend-Fehler. Der Discord-Satz dafür ist der Aussetzer. Der leere Wissenssatz bleibt für `InsufficientEvidence`.

Soll: Ein answered- oder build_rejected-Text wird gezeigt. Links werden aus dem gezeigten Text entfernt. Bleibt danach kein Text, ist das der leere Wissenssatz, nicht der Aussetzer. Ein Text über 3800 UTF-16-Einheiten wird gekürzt und gezeigt. Transportfehler, Zeitüberschreitung und ein Körper, der nicht der Antwortvertrag ist, bleiben der Aussetzer. Die Warnung nennt die Klasse: transport, vertrag, link oder laenge. Kein Fragetext, kein Token, keine Nutzerkennung.

## Eigentum

Worktree: `/home/nathanael/.worktrees/brain-antwort-20261006`
Branch: `fix/brain-antwort-nicht-verschlucken-20261006`
HEAD beim Start: `5e77e7ef744876c82cf5b2c530778ec1549a4e62`

Schreiben darfst du in `rust/crates/dl-brain/src/brain_api.rs` und den Tests direkt in dieser Datei.

Unangetastet: der laufende Bot, Invite-Erkennung, Knowledge-Start, Insights, Brain-Wartung, systemd, Fireworks.

## Beweis

Ein Fixture-Test zeigt vier Fälle. Antwort mit `https://` kommt ohne den Link als Antwort an. Zu langer Antworttext kommt gekürzt als Antwort an. `InsufficientEvidence` bleibt der leere Fall. Ein abgebrochener HTTP-Körper bleibt der Backend-Fehler. Bestehende Tests dieser Datei bleiben grün.

## Abschluss

Kein Push, kein Merge, kein Deploy, kein `systemctl`. Bericht mit Branch, SHA, Testbefehl und Ergebnis nach `AN_HAUPT-A.md`.
