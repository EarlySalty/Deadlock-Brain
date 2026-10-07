# Delegator Spielwissen und Steckbriefe (D5)

Zuerst lesen: `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/DELEGATOR-REGELN.md` (gilt vollständig). Kopf: Hauptsession T3 `92efdb66-6e7e-495c-873e-32f2912c2fa2`. Bericht nur über `AN_HAUPT.md` in diesem Ordner, Steuerung in `VON_HAUPT.md`.

## Ziel (Nutzerauftrag, Priorität 1)

Zu jeder Spiel-Entität (Held, Fähigkeit, Item, später Mechaniken) gibt es einen aktuellen Steckbrief mit allen Werten des aktuellen Patches. Dazu kommt seine Änderungsgeschichte („Patch-Story“: wann wurde was geändert). Das Brain beantwortet Fragen damit, auch rückwirkend („wie war X vor dem Patch Y?“).

Die Quellen werden nicht einzeln betrachtet, sondern je Entität zusammengeführt und gemeinsam gepflegt:
- Patch-Historie (`brain.patch_changes`, rund 170 Patches seit Mai 2024, heute nur über das MCP `dl-brain` lesbar)
- Spieldaten aus Git (660 Dokumente, 676.000 Fakten, Paket B, schon aufbereitet)
- Deadlock-Wiki (38.273 Dokumentversionen, 1,1 Mio. Fakten, Paket A, Importquelle `/home/nathanael/.local/share/deadlock-brain/wiki-spielwissen/2026-10-03/a/normalized-fix6/`)

Neue Patches und neue Spieldaten aktualisieren Steckbrief und Historie automatisch über den bestehenden Wartungsweg (`brain-maintenance`, Patchnotes-Rust-Feed). Es gibt keinen Handbetrieb.

## Architektur (Entscheidung der Hauptsession)

- **Wahrheit ist die Datenbank** (Brain-Postgres): Fakten je Entität mit Gültigkeit von Patch bis Patch (gültig ab, gültig bis, Quelle, Herkunft). Nur so gehen Historie und rückwirkende Fragen.
- **Der Steckbrief wird aus der DB erzeugt, nie von Hand gepflegt.** Bei jedem Import oder Patch wird er neu gerendert. Es gibt zwei Ausgaben aus derselben Erzeugung:
  1. ein kompaktes Steckbrief-Dokument je Entität im Wissensbestand des Brains. Das liest das Modell zuerst, es ist klein, aktuell und mit Quellenangabe.
  2. eine HTML-Seite je Entität für Menschen. Die bestehende Brain-Site (`deadlock-brain-site.service`, `/brain`, Port 8087) wiederverwenden, keine zweite Seite bauen.
- **Rückwirkende Fragen** beantwortet das Brain über eine Abfrage der Historie in der DB, nicht über alte HTML-Stände.
- Widersprechen sich die Quellen, gilt fest: Spieldaten aus Git für Zahlen des aktuellen Patches, Patchnotes für den Zeitpunkt einer Änderung, Wiki für Beschreibungen und Kontext. Der Widerspruch wird im Steckbrief sichtbar markiert, nicht still aufgelöst.
- Wiki-Rechte bleiben unverified/redistribution_allowed=false: Wiki-Inhalte nur intern für Antworten und Steckbrief-Fakten, keine wörtlichen Wiki-Texte auf der öffentlichen HTML-Seite.

## Bestand zuerst (Pflicht, nichts doppelt bauen)

Vor jedem Neubau mit Graphify und Lesen prüfen und wiederverwenden:
- MCP `dl-brain`: `entity_summary`, `patch_history`, `patch_search` (dort gibt es schon Entitäts- und Historienlogik).
- Wiki-Integration von gestern: Branch `feat/brain-wiki-spielwissen-c-integration` (Worktree `/home/nathanael/.worktrees/brain-wiki-spielwissen-c-integration`, HEAD `f7a03f9`, uncommittete Importarbeit), A-Commit `116f643` (`dbrain-sources/src/wiki_inventory*`), B-Commits `7168574`, `f7a03f9`. Akten unter `/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-wiki-spielwissen/` (HANDOFF). Offener Fund dort: Clippy-Fehler in `brain-knowledge-import.rs` Zeilen 11 und 14.
- Patchnotes-Kandidat von W2 und Brain-Kandidatenadapter von Z (bleibt inaktiv, Rechtefrage offen; die Patch-Historie selbst ist davon getrennt nutzbar).
- Brain-Site `/brain` (deadlock-brain-site).

## Zusammenarbeit

Ins Brain-Repo nach main merged und Brain deployt ausschließlich W1 von D1b (`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-03-brain-fertigstellung/welle1/w1/EINGANG.md`). Deine Worker liefern geprüfte Commits auf eigenen Branches und tragen sie dort ein. Brain-Site-Deploy darfst du selbst über den bestehenden Weg machen, wenn sie ein eigener Dienst ist. Es gibt keine Replay-, Steam-Depot- oder Forum-Arbeit.

## Fertig

Für alle Helden, Fähigkeiten und Items gibt es Steckbrief-Dokumente im Brain und HTML-Seiten auf `/brain`, mit aktuellen Werten und Patch-Story. Belegt durch drei echte Brain-Antworten: eine aktuelle Zahl eines Helden, eine Item-Frage und eine rückwirkende Frage („Was wurde bei X im Patch vom 16.09. geändert?“). Ein neuer Patch läuft ohne Handgriff durch.
