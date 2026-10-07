status: aktiv
Datum: 2026-10-03

BRIEFING[WB-1]: Pflichtteile 5/5 | Timer 20 min | Worktree: /home/nathanael/.worktrees/patchnotes-rust-fertig

[Orchestrator] Paket P: Patchnotes-Bot komplett nach Rust

Zuerst `GEMEINSAM.md` in diesem Ordner lesen, sie gilt vollständig.

## Ziel

Der Patchnotes-Bot (`~/repos/Deadlock--Patchnotes-Bot`, rund 13.500 Zeilen Python: `main.py`, `bot_runtime.py`, `perplexity_requests.py`, `changelog_*_fetcher.py`, `patchnotes_db.py`, `brain_feed.py`, `brain_feed_server.py`, `web_publish.py`, `patch_view.py`, `entity_emojis.py`, `reconcile_patchnotes.py`, `web/`) läuft am Ende vollständig als Rust-Binary. Die Python-Unit ist gestoppt und deaktiviert, der Python-Code ist aus dem Repo entfernt (Git-Historie bleibt Referenz).

## Pflicht-Verhalten (Nutzervorgaben, nicht verhandelbar)

- Übersetzung über Perplexity `sonar-pro` mit dem Prompt aus `perplexity_requests.py` (Stand bff168d) **wortgleich** übernommen. Kein Modell- oder Providerwechsel.
- Deutsche Steam-Fassung direkt übernehmen, englische Quellen immer übersetzen, nie englischen Text roh posten.
- Bilder und Medien aus Steam-Artikel, offizieller Update-Seite und Forum-Post bei jedem Patch mitnehmen.
- Alle bestehenden Funktionen erhalten: Erkennung neuer Patches, Discord-Post, Web-Publish, Katalog und Application-Emojis im Brain, Brain-Feed `brain.feed.patchnotes.v1` (Vertrag auf Deadlock-Brain main), Vorschau schreibfrei, Reconcile, Yoshi/DevFeed-Teile soweit sie in diesem Repo leben.
- Offene Aufträge im Repo beachten: `.tasks/2026-10-01-patchnotes-perplexity-bilder-zweitserver/` (Zweitserver, Bilder). Was dort beauftragt ist, gehört zum Port.

## Bestand zuerst

Im Repo gibt es schon Rust (`rust/`, u. a. `patch-sources`, Branch `codex/luna-dispatch/.../feat-patchnotes-rust-presentation-20260928-…`). Per Graphify und `git log` erfassen, was bereits portiert ist, und darauf aufbauen. Ebenso den Brain-Teil `deadlock-brain-patchnotes-sync.service` (Shellskript `~/.local/bin/deadlock-brain-patchnotes-sync.sh`): auf den Rust-Feed umstellen, kein Shell-/Python-Pfad mehr.

## Eigentum

Ganzes Repo Deadlock--Patchnotes-Bot im eigenen Worktree. In Deadlock-Brain nur, was der Patchnotes-Feed-Verbraucher zwingend braucht; jede Änderung dort in `AN_HAUPT.md` ankündigen. Die Unit-Dateien `~/.config/systemd/user/deadlock-patchnotes.service*` und `deadlock-brain-patchnotes-sync.*`. Der laufende Live-Worktree `~/.worktrees/patchnotes-live-main` wird erst nach Live-Beweis des Rust-Binaries umgestellt. Der fremde T3-Thread `0ab3e0f8…` (Katalog) ist nicht deiner.

## Arbeitsstand

Repo-Default-Branch `main`. Worktree `~/.worktrees/patchnotes-rust-fertig`, Branch `feat/patchnotes-rust-fertig-20261003` von `origin/main`. Der Hauptcheckout dort steht auf einem fremden Feature-Branch mit unversionierten Dateien: nicht anfassen.

## Beweisziel

1. Paritätstest: für die letzten mindestens drei echten Patches (aus `patchnotes_db`/Brain) erzeugt das Rust-Binary im Vorschaumodus dieselbe Struktur wie Python (Abschnitte, Bilder, Emojis, Links); Übersetzung über den echten Perplexity-Aufruf für einen Patch.
2. Rust-Unit live, Python-Unit `systemctl --user disable --now`, Journal sauber über mindestens einen Poll-Zyklus, Brain-Feed liefert über den Rust-Pfad.
3. Rückweg dokumentiert (alte Unit-Datei als `.disabled` sichern).
Kein Test-Post in echte Discord-Kanäle. Der erste echte Post kommt mit dem nächsten echten Patch; prüfe dann Bilder und Sprache.

## Routing

Paket P, Versuch 1, Produzent `teil-p`. Rest siehe GEMEINSAM.md.
