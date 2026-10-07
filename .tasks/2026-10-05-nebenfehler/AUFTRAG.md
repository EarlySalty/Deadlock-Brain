# Nebenfehler 05.10.2026

ORCHESTRIERUNG[OR-1]: Stufe mittel | Schritt dispatch | Artefakt: /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-05-nebenfehler/AUFTRAG.md

Drei getrennte Blatt-Worker. Je einer ein Schreibpfad. Kein gemeinsamer Deploy.

| Paket | Repo | Fehler | Live-Wirkung |
|---|---|---|---|
| S | Deadlock-Brain | Sheet-Sync endet mit Exit 0, obwohl Schritte fehlschlagen | Dienst läuft weiter auf dem Stand vom 24.09. |
| I | Deadlock-Bots | Insights-Handshake scheitert, Unit bleibt failed | Nächster Timer ist wöchentlich |
| K | Deadlock-Bots | Knowledge-Start sucht das Cargo-Binary | Dienst ist seit 21:22 aktiv |

Nicht in diesem Auftrag: Wartungstimer, Profil-Quellfix, D9, D6, Watchdog, Failure-Notify, Twitch, Intro-DM, YouTube, Forum, Wiki.

Deploy, Push, Merge und Dienstneustart bleiben beim Kopf, nach einem fremden ALLOW.
