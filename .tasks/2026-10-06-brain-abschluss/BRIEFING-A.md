# Paket A: Brain-Bestandsaufnahme und Fertigbau

Rolle: Teil-Orchestrator (Claude-Code-Hauptsession mit GPT 6.1 Sol). Du darfst native Subagenten mit getrennten Schreibzuständigkeiten starten. Keine weiteren T3-Threads ohne Rückfrage beim Haupt-Orchestrator.
Auftraggeber und Haupt-Orchestrator: Claude-Session `3fcd8f71-443e-48ae-825c-527eb52fbe56`. Erst `AUFTRAG.md` in diesem Ordner lesen.

## Schritt 1: Ehrliche Bestandsaufnahme (zuerst, schnell)

Nicht den alten Berichten glauben, sondern am System prüfen: `origin/main` von Deadlock-Brain, `readlink` der Release-Symlinks, `systemctl show`/Journal der Brain-Dienste (brain-serve, maintenance, Timer), Health-Endpunkte, tatsächliche Antworten im Discord und Twitch. Grundlage für die Soll-Liste: `.tasks/2026-10-03-brain-fertigstellung/` (KOPF.md, UEBERGABE-KOPF-GROK.md, PLAN-NEU.md, welle1/), `.tasks/2026-10-04-spielwissen-steckbriefe/`, `.tasks/2026-10-05-nebenfehler/`, Draft-PRs #3 bis #9.

Ergebnis als `A/STAND.md`: Tabelle Bereich | Soll | Ist (Beleg) | fertig/halb/kaputt/zurückgestellt | nächster Schritt. Bereiche mindestens: Brain-Kern und Release, Patch-Historie und Patchnotes-Weg (Legacy-Timer abgelöst?), Spielwissen-Steckbriefe (D5 Stufe 1 und 2), Discord-Brain und Serverguide, Twitch-Antworten, Wiki-Anbindung, Build-Reasoner, Legacy-Abschaltung. Danach eine Zeile an `AN_HAUPT-A.md`.

## Schritt 2: Fertigbauen

Prioritäten des Nutzers (Stand 04.10., weiter gültig, wenn nichts anderes belegt ist):
1. Spielwissen-Steckbriefe mit Patch-Story (Stufe 1 live, dann Stufe 2).
2. Alles, was live kaputt ist oder „halb“ hängt (Discord-Brain, Serverguide, Tageslauf-Fehler, Legacy-Abschaltung nach belegtem Rust-Weg).
3. Build-Reasoner nach `.tasks/2026-09-16-reasoner-item-zweck/BEFUND.md` (keine handgetaggten Labels, Maßstab Warden 779996, immer `--publish`, `hero_build_id` melden).
Zurückgestellt, nicht bauen: Replays/Demos, Steam-Depot, Forum, Begrüßungsserien, Grafiken.

Vorhandene Branches und Worktrees mit eigenem Inhalt übernehmen statt neu zu bauen. Paket C listet in `C/OFFEN.md` die nicht gemergten Branches; entscheide dort je Zeile „übernehmen“ oder „verwerfen“ mit kurzem Grund, C setzt das um.

Nicht anfassen: Game-Invite-Bug (Paket B). Fremde T3-Threads nicht anschreiben.

## Abschluss

Je fertigem Teil: Gate-Selbstprüfung, Merge, `brain-release install`, Neustart, Live-Beweis, aufräumen. Am Ende `A/STAND.md` aktualisieren (was jetzt fertig ist, was bewusst offen bleibt und warum) und `AN_HAUPT-A.md` mit Kurzfazit. Bei echten Produktentscheidungen (Kosten, Datenschutz, Abschaltung von Nutzerfunktionen) im `AN_HAUPT-A.md` fragen und an anderem Teil weiterarbeiten.
