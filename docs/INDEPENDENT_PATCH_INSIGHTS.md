# Eigenständige Patchanalyse und Quellenbelege

## Ziel und Wissensgrenzen

Das Brain entwickelt seine eigene Analyse aus dem offiziellen Patch und bereits
beobachteten Spieldaten. Ein Creator-Video ist weder Promptvorlage noch Wahrheit.
Es kann separat als Vergleichsquelle gespeichert werden. Eine abweichende,
belegte Schlussfolgerung ist zulässig und kein Fehler des Brains.

Der neue Rust-Befehl `patch-insights` verwendet den vorhandenen `AiClient` und die
bestehende Modellkonfiguration. Er führt keinen zusätzlichen Provider und keinen
STT-Dienst ein. Der bestehende Python-MCP-Server bleibt als Read-only-Brücke
kompatibel; neue Spielanalyse und Caption-Verarbeitung liegen in Rust.

Der erste implementierte Pfad erzeugt quellengebundene, gegengeprüfte Hypothesen
und ein Storyboard. Er ist kein vollständiger Spielsimulator und behauptet keine
empirisch bewiesene Meta. Eine Modellprüfung macht eine Hypothese nicht zur
Spielmechanik. Automatisches Veröffentlichen oder Steam-Build-Publish gibt es in
diesem Pfad nicht.

## Datenfluss

`patchnotes.changelog_posts` bleibt die Rohbasis des vorhandenen Imports.
`brain.patch_events` und `brain.patch_changes` bleiben die gemeinsame Historie.

`patch-insights analyze` liest alle indexierten Zeilen der angegebenen offiziellen
Steam-Patch-URL. Zusätzlich lädt es pro Hero, Item und Ability den letzten
Assets-Snapshot **vor Beginn des UTC-Patchtags**. Auch nicht direkt gepatchte
Heroes sind dadurch grundsätzlich im Kontext. Die Auswahl von Snapshotfeldern
ist ausdrücklich als unvollständig markiert; ausgelassene Felder werden genannt.
Fehlende oder veraltete Kit-Daten müssen als Wissenslücke behandelt werden.

Der Kontext wird nicht heimlich auf eine Zeichenanzahl gekürzt. Überschreitet er
das konfigurierte Budget, bricht der Auftrag vor einem Modellaufruf ab. Pro
Datenbank darf nur ein Analyseauftrag gleichzeitig laufen. Ein Advisory-Lock
wird an eine Transaktion gebunden und bei Abbruch freigegeben.

Die erste Modellstufe formuliert Wirkungsketten mit Quellen-IDs, Bedingungen,
Gegenbedingungen und Prüfplan. Eine zweite Stufe kontrolliert jede Hypothese gegen
denselben Quellenstand. Referenz-IDs und vollständige Prüfabdeckung werden
zusätzlich deterministisch kontrolliert. Nur `grounded_hypothesis` gelangt in das
Storyboard. Auch dort steht weiterhin `empirically_verified=false`.

Einfache Zahlenänderungen werden in Rust berechnet. Prozentpunkte werden von
relativen Prozenten unterschieden. Gemischte Einheiten, Staffelwerte, nicht
endliche Zahlen und unlesbare Formeln werden nicht geraten. Aus dem Vorzeichen
einer Zahl allein wird kein Buff oder Nerf abgeleitet. Komplexe Kampfsimulationen,
Breakpoints und experimentelle Verifikation bleiben ein eigener Ausbauschritt.

Der Kontext wird vor einer Speicherung erneut geladen. Ändert er sich während
der Modellaufrufe, wird das Ergebnis nicht gespeichert. Quellen- und Kontext-
Hashes stehen im Ergebnis. `patch_insight` prüft außerdem beim Lesen den Hash der
heutigen indexierten Fassung **dieses** Patch-Beitrags. Änderungen daran markieren
alte Analysen als `stale`. Ein späterer bekannter Patch wird getrennt gemeldet.
Das ist keine Garantie, dass der lokale Index den letzten Live-Patch kennt.

## Migration und Befehle

Migration durch den Schema-Betreiber ausführen:

```sh
psql -X -v ON_ERROR_STOP=1 -f scripts/migrations/2026-09-18-patch-insights-evidence.sql
cargo build --manifest-path rust/Cargo.toml --package deadlock-brain --bin patch-insights
cargo build --manifest-path rust/Cargo.toml --package deadlock-brain-yt
```

Die neuen Tabellen benötigen für die bestehenden Dienstrollen passende
Lese-/Schreibrechte. Die Migration vergibt absichtlich keine Rechte an PUBLIC.
Sie muss vor Verwendung des neuen Caption-Schreibpfads installiert sein.

Ohne Modellaufruf und ohne Schreibzugriff den Quellenkontext prüfen:

```sh
rust/target/debug/patch-insights analyze \
  --patch-url 'https://store.steampowered.com/news/app/1422450/view/698776157349216434' \
  --no-ai --out /tmp/patch-context.json
```

Eigene Analyse mit bestehender Modellkonfiguration:

```sh
rust/target/debug/patch-insights analyze \
  --patch-url 'https://store.steampowered.com/news/app/1422450/view/698776157349216434' \
  --write --out /tmp/patch-insights.json
```

Ohne `--write` wird analysiert, aber nicht in Postgres gespeichert. Das kann
Modellkosten verursachen. `--no-ai --write` wird abgelehnt. Es gibt keine
automatische Veröffentlichung. Ein fehlender Patch im Index ist ein Fehler;
Transkriptbehauptungen dienen nicht als Ersatz für den offiziellen Text.

Eine bereitgestellte Textdatei als separate Vergleichsquelle importieren:

```sh
rust/target/debug/patch-insights import-reference \
  --file '/pfad/zum/transkript.txt' \
  --source-url 'https://www.youtube.com/watch?v=ZWm7ixeWjbQ' \
  --write --out /tmp/reference-import.json
```

Dieser Import erfindet keine Zeitmarken, korrigiert keine Namen und macht den Text
nicht zu Ground Truth. Er schreibt ausschließlich nach
`brain.patch_reference_documents`. Die Analyseladefunktion fragt diese Tabelle
nicht ab. Das private Originaltranskript gehört nicht automatisch ins öffentliche
Repository. Ohne `--write` ist dieser Import auch ohne Datenbank nutzbar.

## Caption-Fix

Neue Caption-Imports bewahren JSON3-Rohdaten und einzelne Segmente mit
`tStartMs`, `dDurationMs`, berechnetem Ende und Originalreihenfolge. Rohdaten
bewahren zusätzliche Token-Zeitmarken. Fehlende Zeitmarken bleiben `null`.
`timing_status` ist `available`, `partial` oder `missing`.

Korrigierte Zeitmarken erzeugen auch bei identischem Text einen neuen Evidence-
Hash. Frühere Rohfassungen bleiben in `brain.youtube_transcript_evidence`
erhalten. Der normale Text und sein bestehender Hash bleiben in
`brain.youtube_transcripts`. Beide Schreibschritte und der Status `ready` liegen
in derselben Transaktion. Ein Fehler rollt alles zurück.

Der Download nutzt keine undrainierten Ausgabepipes mehr. Logs landen in einer
Temporärdatei und werden begrenzt gelesen; Kindprozesse werden bei Timeout
beendet. Vorhandene yt-dlp-Konfiguration wird ignoriert. Die Ausgabe wird durch
Metadaten auf manuelle oder sprachlich zuordenbare automatische EN-/DE-Captions
begrenzt; erkannte maschinell übersetzte Auto-Tracks werden nicht übernommen.
Ohne Metadaten bleibt nur der bisherige englische Fallback aktiv.

Bereits vorhandene reine Fließtexte erhalten nicht nachträglich erfundene
Zeitmarken. Sie müssen bei Bedarf anhand tatsächlicher Caption-Rohdaten erneut
importiert werden. Der alte externe Transcript-Claims-Workflow erhält durch
diese Änderung nicht automatisch eine neue Extraktionslogik.

## Gemeinsame Read-only-Tools für Assistenten

Bestehende Tools bleiben erhalten. Hinzu kommen:

- `change_lookup(text, entity, since, until, limit, offset)`: wörtliche
  UND-Suche in Patchzeilen, Namen, Abilities und Stats, Aliasauflösung,
  Originalquelle, alte/neue Werte, Trefferzahl und Pagination.
- `patch_insight(patch_url)`: letzte eigene Analyse, Quellenrevision und Hinweis
  auf spätere indexierte Patches. Kein automatisches Hochstufen zu Ground Truth.
- `video_evidence(video_id, text, limit)`: Caption-Segmente der zum aktuellen
  Transkript-Hash passenden Rohfassung. Belegt Gesagtes, nicht dessen Richtigkeit.

`patch_date` ist das dokumentierte Patchdatum. `first_indexed_at` ist die früheste
noch erhaltene Parser-Indexierungszeit derselben Originalzeile, kein nachgewiesener
Live-Zeitpunkt und keine vollständige Beobachtungshistorie. Rebuilds können diese
Zeit verschieben. Die früheste gefundene Änderung gilt nur für den vorhandenen
Index und das gewählte Suchfenster.

Die MCP-Brücke bleibt Read-only. Die bestehende Konfiguration muss für andere
Assistenten auf diesen Server zeigen; eine zusätzliche Insel-Datenbank entsteht
nicht. Die Zugangsdaten werden erst bei der ersten tatsächlichen Abfrage geladen,
nicht bereits beim Import des Python-Moduls.

## Was ein Storyboard ist, und was noch fehlt

Die Ausgabe enthält deutsche Sprechertext-Entwürfe und Pläne für Quellenkarten
und Vorher/Nachher-Ansichten. Sie enthält kein gerendertes Video, keine synthetische
Stimme und keine abgeschriebenen Creator-Kapitel. Gameplay-Material und echte
Bildbelege müssen später ausdrücklich zugeordnet werden.

Der hier bereitgestellte Text enthält keine Bilder. Die Implementierung nennt
Caption-Text niemals eine visuelle Transkription. Ein visueller Lernpfad benötigt
reale Frames mit Videozeit, Herkunft und einer eigenständigen Prüfung. Weder
Frames noch Bildaussagen werden aus dem Transkript erfunden.

Weiterhin offen sind eine vollständige kausale Mechanikdatenbank, automatische
gezielte Neubewertung abhängiger Regeln nach jedem Patch, gemessene Spiel-
Experimente und ein verblindeter fachlicher Vergleich mit dem Referenzvideo.
Die Verbindung zu anderen Assistenzsystemen nutzt den bestehenden MCP-Zugang;
dessen Live-Konfiguration und Dienstrechte müssen im Deploy geprüft werden.

## Tests

```sh
python -m pytest -q mcp/test_history_tools.py
cd rust
SQLX_OFFLINE=true cargo test --package deadlock-brain --bin patch-insights
SQLX_OFFLINE=true cargo test --package deadlock-brain-yt transcripts::
cargo clippy --package deadlock-brain --bin patch-insights
cargo clippy --package deadlock-brain-yt
```

Der Caption-Postgres-Test ist absichtlich ignoriert, solange keine isolierte
Scratch-Datenbank mit Grundschema und neuer Migration vorhanden ist. Nicht gegen
den Produktionskorpus ausführen. Der Workflow `patch-insights.yml` führt nur
Vertragstests und Clippy aus, nicht Deployment oder Modellinferenz.
