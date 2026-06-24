# Patchnotes-Forum Analyse

Quelle: https://forums.playdeadlock.com/forums/changelog.10/

## Ergebnis

Die bestehende Patchnotes-Pipeline deckt die Forum-Changelogs bereits ab. Es
sollte kein redundanter Forumscraper gebaut werden.

## Befund

- `src/deadlock_brain/sources/patchnotes_db.py` importiert aus der zentralen
  Bot-DB-Tabelle `changelog_posts` die Felder `title`, `url`, `posted_at`,
  `raw_content` und `translated_content`.
- `classify_source_kind()` markiert `forums.playdeadlock.com` bereits als
  `forum`.
- `src/deadlock_brain/patch_parser.py` liest `entity_snapshots` vom Typ
  `patchnote` und schreibt strukturierte `patch_events`.
- Die CLI ist bereits verdrahtet:
  `deadlock-brain pull patchnotes` importiert aus der zentralen DB,
  `deadlock-brain parse patchnotes` erzeugt daraus Events.
- Lokale DB-Pruefung gegen
  `/home/naniadm/Documents/Deadlock-Bots/data/deadlock.sqlite3`:
  117 Changelog-Posts mit Inhalt, davon 109 Forum-URLs und 8 Steam-URLs.
- Das Forum bietet zusaetzlich einen XenForo-RSS-Feed unter
  `/forums/changelog.10/index.rss`; der Feed ist fuer Discovery nutzbar, enthaelt
  aber teils nur Auszuege oder Steam-Unfurl-Karten. Die zentrale DB ist deshalb
  weiterhin die passendere Quelle fuer diese Pipeline.

## Scaffold

Minimaler, nicht verdrahteter Gemini-Textmodus-Scaffold:
`src/deadlock_brain/sources/patchnotes_gemini_scaffold.py`

Der Scaffold nimmt Changelog-Text entgegen, bereinigt HTML grob, ruft denselben
Gemini-Browser-Worker per `analyze-text` auf und parst die finale Antwort als
JSON. TODO: Integration erst nach Review entscheiden; die bestehende
deterministische `patch_parser`-Pipeline bleibt unveraendert.
