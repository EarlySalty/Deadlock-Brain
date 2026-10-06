# Fix A, Runde 2: abschließender Freeze

Datum: 03.10.2026
Auftraggeber: Teil-Orchestrator A, f01cce67-209b-468e-8abb-ec2070beeaa2
Worktree: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`
Branch und HEAD laut Briefing: `feat/brain-wiki-spielwissen-a`, `2734c2da4e814ff79953e8e825275b0216a6af16`
Modell: geerbtes GPT 6.1 Sol, keine Modellwechsel und keine Delegation.

Die ausdrückliche Schreibfreigabe aus dem aktuellen Auftrag ersetzt die vorsorgliche Schreibsperre des Briefings. Die vier Moduldateien sind nach dem letzten Formatlauf eingefroren. Ab jetzt keine weiteren Moduländerungen durch diesen Fixer. Der pausierte Datenworker kann genau diesen Stand prüfen. Keine eigenen Cargo-Aufgaben, Compiler, Hintergrundwrapper oder Hostlock-FDs gestartet oder gehalten. Harness und Originaldaten wurden nicht geändert.

Kompilierung bestätigt: N. Ausgeführte Regressionen bestätigt: N. Vollständige Echtdatenverarbeitung bestätigt: N. Grünes Laufzeit- oder Gesamturteil: N.

WIRKUNGSPRUEFUNG[WP-1]: 0 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 2/2 geprüft

Diese Zeile beschreibt ausschließlich die eigene statische Durchsicht der beiden zugewiesenen Korrekturen. Sie ist weder unabhängiger Review noch Gate-Freigabe. Die integrierte HTTP-Anbindungs- und Laufzeitprüfung bei C2 bleibt separat offen.

## Die zwei zugewiesenen Korrekturen

### 1. Quellenbindung vor dem ersten Dokumenteffekt

Der gemeinsame Spool erhält eine dauerhaft gesicherte, unveränderliche `source.json` mit Vertragsversion, tatsächlicher Quellenkennung und tatsächlicher HTTP-/HTTPS-Herkunft. Vor deren Anlage prüft `bind_source` vorhandenen Marker, tatsächlichen Checkpoint-Kontext, sämtliche vorhandenen Revisionsdokumente und Konfliktdateien. Eine fehlende Checkpoint-Datei liefert ausdrücklich keinen Beleg für die offizielle Quelle. Alte verwaiste Spools ohne Marker werden anhand ihrer erhaltenen Dokumente geprüft. Gemischte oder widersprüchliche Herkunft führt vor neuen Dokument-, Marker- oder Checkpointschreibungen und vor Live-Abrufen zum Fehler.

Beide Offline-Eingänge laufen weiterhin über `persist_offline`; dieser bindet den Spool vor `persist_document`. Der Live-Eingang bindet und prüft denselben Spool vor Zugriffssperren-Reset, Checkpointänderung und erstem Transportaufruf. Die ausdrücklich festgelegte Live-Zieladresse ist kein aus einem Checkpoint-Default abgeleiteter Herkunftsbeleg. Der Berichtseingang rekonstruiert vorhandene Herkunft ebenfalls über diesen Schutz. Direkte Dokument- und Checkpointschreibungen ohne passende vorherige Bindung werden zusätzlich im Speicherpfad abgewiesen.

Ein erster Offline-Lauf mit Inhaltskonflikt kann Dokument und Konflikt erhalten, aber nicht seine Quelle verlieren. Eine andere Quelle kann diesen Stand nicht umwidmen. Die ursprüngliche Quelle kann ihren unveränderten Originaltext wieder aufnehmen. Die vorhandenen Unterschiede zwischen Domains, HTTP-/HTTPS-Schema und Artikelpfaden bleiben erhalten.

### 2. Dauerhafte Ergänzung belegter Autorenherkunft

Identischer Inhalt derselben ID und Revision bleibt ein einzelnes Revisionsdokument. Das Original unter `documents/`, sein Text, Inhalts-SHA-256, Revisionsschlüssel und Erstbeobachtungszeit werden nicht überschrieben. Abweichende zusätzliche Herkunft wird unter `provenance/<Revisionsschlüsselhash>.json` als atomar gesicherte Ergänzung im bestehenden Spool gehalten, nicht in einer zusätzlichen Datenbank.

Jede Ergänzung enthält die Zuordnung zur Original-ID, Revision und Inhalts-SHA-256, einen SHA-256 ihrer Herkunftsaussage sowie die erste Beobachtung dieser Ergänzung. Die Aussage erhält Autor, Contributor, Attribution, Quellenadresse, Quellen-/Aufzeichnungshashes, belegte Abruf-/Lizenzzeit und das Eingabeformat. API-JSON und XML werden ausdrücklich unterschieden. Der Beobachtungszeitpunkt selbst gehört nicht zum Ergänzungsschlüssel: dieselbe Aussage an einem späteren Tag erzeugt keine zweite Ergänzung. Vorhandene Ergänzungen werden beim erneuten Öffnen in die bisherige Gesamtgrößengrenze eingerechnet; ein unzulässiger Zuwachs wird vor dem Speicherschritt abgewiesen.

`publish` liest die dauerhaften Ergänzungen und erzeugt das Vertragsdokument daraus erneut. Eindeutige zusätzliche Autoren-/Contributor-Werte können bislang leere Felder ergänzen. Widersprüchliche belegte Werte werden nicht durch einen ausgewählten Gewinner ersetzt. `metadata.provenance_original`, `metadata.provenance_supplements` und `metadata.provenance_conflicts` erhalten Original, einzelne Aussagen und sichtbare Widersprüche. Vorhandene singuläre Originalwerte bleiben erhalten; bei widersprüchlicher Herkunft werden leere singuläre Werte nicht automatisch befüllt.

Die ursprüngliche Attribution bleibt erhalten und weitere belegte Attribution wird angefügt. Lizenzname, Lizenzadresse und Nutzungsrechte des Originals bleiben unverändert. Eine spätere Beobachtung kann insbesondere `redistribution_allowed=false` nicht auf true hochstufen. Abweichende spätere Lizenzbehauptungen bleiben ausschließlich in ihrer Herkunftsergänzung nachvollziehbar. Wiederveröffentlichung nach einem Abbruch zwischen Ergänzungsschritt und Checkpoint/Veröffentlichung liest den bereits gesicherten Ergänzungsstand.

## Geschriebene Regressionen und erhaltene Fixes

Alle vorhandenen 22 Testfunktionen bleiben erhalten, ohne Abschwächung, Überspringen oder neue Ignore-Markierung. Sieben zusätzliche Testfunktionen ergeben statisch gezählte 29 `#[test]`-Vorkommen:

1. Erster API- oder XML-Batch mit gleichem Versionsschlüssel und verschiedenen Texten: dauerhafte Bindung, Fremdquellenablehnung über beide Offline-Eingänge und Live, keine zusätzlichen Dateiänderungen, ursprüngliche Quelle wiederaufnehmbar.
2. Unterbrechung direkt nach Originalspeicherung, sowohl mit Marker als auch als älterer Spool ohne Marker: Fremdquelle vor Netz-/Schreibeffekten abweisen, Originalquelle wiederaufnehmen.
3. Bereits gemischte ältere verwaiste Dokumente: weder Quellenmarker noch zusätzliche Daten schreiben und keinen Transportaufruf ausführen.
4. API ohne Autor, danach XML mit Actual author und Contributor-ID 9: dauerhaft sichtbare Ergänzung, unveränderte Originaldateien, Text, Hash, Revision, Erstbeobachtung und ursprüngliche Lizenz; Wiederholung mit späterer Beobachtungszeit ohne Duplikat; erneutes Öffnen und Veröffentlichen.
5. Widersprüchliche API-/XML-Autoren: ursprünglichen Wert und ergänzende Gegenbelege erhalten, Widerspruch sichtbar, keine automatische Contributor-Auswahl, Wiederholung idempotent.
6. Abbruch nach dauerhafter Ergänzung vor Veröffentlichung: Ergänzung beim Wiederöffnen erhalten; beliebige spätere Behauptung freier Rechte hebt die ursprünglichen Rechte nicht an; erneute Speicherung erzeugt kein Duplikat.
7. Direkt geöffneter, aber ungebundener Spool: Dokument- und Checkpointschreibungen verweigern.

Diese Szenarien sind geschriebene Rust-Regressionen, keine ausgeführten Reproduktionen. Die vorherigen Revisionsauswahl-, lesbare-Historie-, Checkpoint-Lesebegrenzungs-, Domain- und Artikelpfadfixes wurden nicht zurückgenommen. Beide Inventar-Einfügepfade verwenden weiterhin den gemeinsamen revisionsbewussten Merge.

## Eigene tatsächlich ausgeführte Prüfungen

- Gelesen: BRIEFING-FIX-2.md, zentraler AUFTRAG.md, PAKETE.md, CONTRACT.md und AN_BEREICHE.md, REVIEW-LOCAL-2.md, FIX-1.md, BAU.md sowie `/home/nathanael/Documents/.tasks/2026-10-02-offene-branches/HOSTPROBE.md`.
- `code-suche` vor Codefragen geladen. Die lokale Graph-Abfrage scheiterte am nicht vorhandenen Worktree-Graphen; die anschließende globale Graphify-Abfrage lief. Ihre Treffer waren für diese noch nicht registrierten neuen Module nicht belastbar. Keine Graphneuerstellung.
- Die Kontextwerkzeug-Ausführung wurde vom aktuellen Berechtigungsmodus verweigert. Die vorhandenen nativen Read-, Edit-, Grep- und Bash-Werkzeuge wurden verwendet, ohne Settingsänderung oder Schutzumgehung.
- Ausschließlich die vier eigenen Module mit `/home/nathanael/.cargo/bin/rustfmt --edition 2021` formatiert. Der anschließende gezielte `--check` auf denselben vier Dateien endete erfolgreich mit Exit 0. Ein vorheriger Aufruf ohne absoluten Formatterpfad scheiterte mit Exit 127; dies war keine Compilerprüfung.
- Gezielte Grep-Zwillingssuche belegt beide Dokument-/Checkpointpfade, drei Aufrufstellen des gemeinsamen Bindungsschutzes einschließlich Berichtseingang, den gemeinsamen Ergänzungsschritt, die Veröffentlichung daraus und beide Eingabeformatmarkierungen. Testanzahl separat per Grep als 29 gezählt.
- Abschließende vier SHA-256-Werte nach dem letzten Formatlauf mit `sha256sum` gemessen. Die Eingangs-Freeze-Werte wurden aus dem bestätigten Rundendokument übernommen, nicht vor der Änderung nochmals eigenständig gemessen.

Kein Cargo-, rustc-, Clippy-, Test-, Daten- oder Live-Netzlauf. Keine Secrets, ENV-Dateien, fremde Prozessverwaltung, weiteren Agenten oder T3-Threads. Keine Core-, Manifest-, Schema-, Harness-, Originaldaten-, Register-, Status- oder Übergabeänderungen. Keine Commit-, Push-, Merge- oder Deployschritte. Die echten Compiler-/Datenläufe verbleiben beim pausierten Datenworker, mit beiden HOSTPROBE-Sperren in vorgeschriebener Reihenfolge, frischer NonZombie-Probe und höchstens zwei Jobs.

## Sichere Übergabe: vier neue SHA-256-Werte

| Eingefrorene Datei | SHA-256 |
| --- | --- |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory.rs` | `2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs` | `27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/storage.rs` | `88ff521c77032215ba2aef8b4dc4abee20fd43bedbcf5ab218620acbb43e7498` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/tests.rs` | `82356a793f8981f021ef6ab9fcf420bfc529fa445243dcff04df2ed2749e7d91` |

Vor und nach den tatsächlichen Datenworker-Läufen diesen konkreten Freeze prüfen. Nur dieser abschließende Stand wird übergeben.

## Offene integrierte Prüfung und Gate

Die während der Arbeit übermittelte verbindliche Präzisierung des Hauptorchestrators zu AN_BEREICHE.md Punkt 26 ist berücksichtigt: Die frische Main-Integrationsbasis besitzt bereits `HttpClient::get_bounded` in `deadlock-brain-core/src/http/bounded.rs` mit Content-Length-Prüfung und `Read::take(max_bytes + 1)`. Keine neue Coreimplementierung und kein zweiter Netzpfad werden von A benötigt oder gebaut. C2 prüft die Anbindung des vorhandenen Readers und dessen Größen-/Laufzeitprobe; unsere ältere A-Basis wird dafür hier nicht verändert. Die Grenze wird ausdrücklich als offene integrierte Anbindungs-/Laufzeitprüfung bei C2 geführt.

`gate_hook.py --review`: ausstehend. Kein in dieser Runde belegter vorhandener Sol-only-Auswahlweg mit Effort höchstens high; keine Astra-/Opus-Defaults gestartet. Echte Kompilierung, ausgeführte 29 Regressionen, vollständige Archive, reale Crash-/Wiederaufnahmeprüfung, C2-Integration und unabhängiger Schluss-Gate bleiben ausstehend. Dieser Bericht erteilt keine Laufzeitfreigabe.
