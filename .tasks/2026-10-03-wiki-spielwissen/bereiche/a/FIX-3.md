# Fix A, Runde 3: sicherer abschließender Freeze

Datum: 03.10.2026
Auftraggeber: Teil-Orchestrator A, f01cce67-209b-468e-8abb-ec2070beeaa2
Worktree: `/home/nathanael/.worktrees/brain-wiki-spielwissen-a`
Branch und HEAD laut Briefing: `feat/brain-wiki-spielwissen-a`, `2734c2da4e814ff79953e8e825275b0216a6af16`
Modell: geerbtes GPT 6.1 Sol. Keine Modellwechsel, zusätzlichen Agenten oder T3-Threads.

Die ausdrückliche Schreibfreigabe aus BRIEFING-FIX-3.md wurde übernommen: Der Datenworker bestätigte dort das Ende ausschließlich seines eigenen wartenden Wrappers um 05:50:10 UTC, ohne gestartetes Cargo und ohne verbleibende eigene Lock-FDs oder Compilerkinder. Dieser Fixer hat selbst keine Wrapper, Compiler, Hintergrundaufgaben oder Hostlock-FDs gestartet oder gehalten. Fremde Prozesse wurden nicht verändert. Der Datenworker blieb während der Änderungen gemäß Auftrag pausiert.

Die vier Moduldateien sind nach dem erfolgreichen Formatcheck und der abschließenden Hashmessung eingefroren. Ab jetzt keine weiteren Moduländerungen durch diesen Fixer. Dieser Bericht ist der einzige zusätzliche Schreibpfad. Harness, Originaldaten, Core, Manifeste, Register, Statusdateien und TODO.md blieben unangetastet.

Kompilierung bestätigt: N. Ausgeführte Regressionen bestätigt: N. Echtdatenverarbeitung bestätigt: N. Stromausfall- oder Kernel-Crash-Prüfung bestätigt: N. Integrierter Gate- oder Gesamtstatus: ausstehend.

WIRKUNGSPRUEFUNG[WP-1]: 2 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

Diese Zeile bezieht sich ausschließlich auf die beiden zugewiesenen statischen Speicherbefunde. Beide wurden im gemeinsamen Speicherpfad korrigiert. Sie bezeichnet keine unabhängige Abnahme und keine Laufzeitfreigabe. Im geänderten Speicherbereich gibt es keinen Fremddienstaufruf.

## 1. Konflikte zählen und vor dem Schreiben begrenzen

`WikiSpool::open` zählt jetzt alle JSON-Dateien aus `documents/`, `provenance/` und `conflicts/` zusammen. Die vorhandene Zählweise bleibt erhalten: tatsächliche Dateilänge plus ein Byte je Dokument, jeweils mit geprüfter Addition und unverändertem Gesamtlimit. Ein bereits zu großer Bestand führt sichtbar zum Fehler; vorhandene Dateien werden weder gelöscht noch überschrieben.

Vor einer neuen Konfliktdatei wird das vollständig normalisierte Vertragsdokument serialisiert und dessen Größe mit derselben geprüften Addition einschließlich des zusätzlichen Bytes gegen `stored_bytes` und `max_total_bytes` geprüft. Erst nach erfolgreichem unveränderlichem Schreiben wird `stored_bytes` fortgeschrieben. Ein bereits unter demselben Revisions- und Inhaltsschlüssel gesicherter Konflikt wird nicht erneut geschrieben oder gezählt. Das gilt auch bei ausgeschöpftem Budget und nach Wiederöffnen.

Die Ablehnung wegen Gesamtgrenze erfolgt vor Verzeichnisanlage und Dateischreiben. Sie meldet ausdrücklich „Konflikt nicht gespeichert“ und „Inventar bleibt unvollständig“, statt eine vollständige Konflikterfassung zu behaupten. Das unveränderte Original bleibt unter demselben Limit wiederaufnehmbar. Grenzen wurden nicht erhöht. Der gemeinsame Pfad bleibt für API, XML und Live maßgeblich; Quellenbindung und Original-/Herkunftstrennung wurden nicht verändert.

## 2. Neue Verzeichnisse und Eltern vor Bestätigung synchronisieren

Ein gemeinsamer Helfer legt das benötigte Verzeichnis an und synchronisiert anschließend dessen gesamten absoluten Elternpfad bis zur Wurzel. Damit werden sowohl das Verzeichnis selbst als auch seine Einträge in den nötigen Eltern synchronisiert. Bereits existierende Verzeichnisse werden ebenfalls synchronisiert, damit ein nach einer fehlgeschlagenen Synchronisierung wiederholter Versuch nicht allein aus ihrer Existenz eine Dauerhaftigkeitsbestätigung ableitet. Jeder Fehler wird weitergegeben.

`WikiSpool::open` verwendet diesen Helfer für die Spoolwurzel und `documents/`. Beide Dateischreiber, `write_atomic` und `write_immutable`, verwenden ihn für den Dateielternpfad vor dem temporären Dateischreiben. Damit sind auch die erste `provenance/`- und `conflicts/`-Anlage erfasst. Die bisherigen Schritte Dateisynchronisierung, atomarer Rename beziehungsweise unveränderlicher Hardlink und anschließende Synchronisierung des unmittelbaren Dateielternverzeichnisses bleiben erhalten. Die Bestätigung hängt nicht mehr von einem späteren Checkpoint ab. Der Veröffentlichungsweg verwendet die bereits beim Öffnen gesicherte Spoolwurzel und behält seine eigene Datei-/Elternsynchronisierung.

Dies ist eine Korrektur der Synchronisierungsmechanik. Es wurde weder ein Stromausfall noch ein Kernel-Crash ausgelöst oder simuliert. Eine echte Crash-Dauerhaftigkeitsprüfung wird nicht behauptet.

## Regressionen und Bestandserhalt

Die 29 vorhandenen Tests bleiben erhalten. Vier gezielte neue Testfunktionen ergeben statisch gezählte 33 `#[test]`-Vorkommen:

1. Exaktes Budget aus Original und Konflikt: erstes Konfliktschreiben, idempotente Wiederholung ohne Doppelzählung, weiterer Konflikt vor Schreibeffekt abgewiesen; Wiederöffnen mit einem Byte weniger verweigert, mit exaktem Budget möglich; erneuter Konflikt und neue Revision bleiben am Gesamtbudget begrenzt, das Original bleibt unverändert veröffentlichbar.
2. Eine API-Aufzeichnung passt in die Antwortgrenze, ihr angereichertes Vertragsdokument überschreitet aber die Gesamtgrenze: sichtbare Ablehnung ohne Konfliktdatei oder veränderten Spool, anschließend Originalwiederaufnahme und Berichtswiederöffnung unter denselben Grenzen.
3. Verzeichnis-/Elternsynchronisierung mit nachvollziehbarer Aufrufreihenfolge, injiziertem Elternsynchronisierungsfehler und erneuter vollständiger Synchronisierung bereits angelegter Verzeichnisse. Die Injektion prüft die Mechanik und Fehlerweitergabe, keinen echten Stromausfall.
4. Neue verschachtelte Spoolwurzel, erste Herkunftsergänzung und erster Konflikt ohne anschließenden Checkpoint: gewöhnliches Wiederöffnen, idempotente Wiederholung und Veröffentlichung erhalten Original, Erstbeobachtung, Autorenherkunft und Konflikt.

Diese Regressionen wurden geschrieben, nicht ausgeführt. Frühere Quellenbindungs-, Revisions-, Inventar-, Checkpoint-, Domain-, Artikelpfad- und Herkunftsfixes bleiben erhalten. `wiki_inventory.rs` und `normalize.rs` sind nach dem Formatlauf bytegleich zum Eingangs-Freeze. Inhaltliche Änderungen betreffen ausschließlich `storage.rs` und `tests.rs` innerhalb der vier erlaubten Modulpfade.

## Tatsächlich ausgeführte Prüfungen

- Gelesen: BRIEFING-FIX-3.md, zentraler AUFTRAG.md, CONTRACT.md und AN_BEREICHE.md einschließlich Punkt 26, REVIEW-LOCAL-3.md, FIX-2.md, HOSTPROBE.md sowie die betroffenen Speicher- und Testpfade und ihre Aufrufstellen in `wiki_inventory.rs`.
- Eingangs-Freeze aller vier Module unabhängig mit `sha256sum` gemessen; alle vier Werte stimmten mit Briefing und unabhängigem Review überein.
- `code-suche` geladen. Die erste lokale Graphify-Abfrage scheiterte am fehlenden Worktree-Graphen. Globale Graphify-Abfragen wurden ausgeführt; die konkrete Symbolabfrage lieferte keine passenden Knoten. Kein Graph neu aufgebaut.
- Das Kontextwerkzeug wurde im aktuellen Berechtigungsmodus verweigert. Native Werkzeuge wurden ohne Settingsänderung oder Schutzumgehung verwendet.
- Gezielte Grep-Zwillingssuche auf Zählung, geprüfte Addition, Verzeichnisanlage, beide Dateischreiber, Veröffentlichung und Synchronisierung ausgeführt. Statische Testanzahl separat per Grep als 33 gemessen.
- Tatsächlicher Formatlauf ausschließlich auf den vier erlaubten Moduldateien mit `/home/nathanael/.cargo/bin/rustfmt --edition 2021 --config skip_children=true`. Danach tatsächlicher `--check` auf exakt denselben vier Dateien: Exit 0. Kein Cargoformatlauf und keine weiteren Modulpfade.
- Abschließende vier SHA-256-Werte nach dem Formatcheck mit `sha256sum` gemessen.

Keine Cargo-, rustc-, Clippy-, Test-, Release-, Daten-, Netzwerk-, Git- oder Deployschritte. Keine Secrets oder ENV-Dateien gelesen. Keine eigenen Hostlocks erworben. Die späteren echten Compiler-/Datenprüfungen bleiben beim separaten Datenworker mit den beiden blockierenden HOSTPROBE-Sperren in vorgeschriebener Reihenfolge, frischer NonZombie-Probe und höchstens zwei Jobs.

## Endgültige vier SHA-256-Werte

| Eingefrorene Datei | SHA-256 |
| --- | --- |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory.rs` | `2507c127aaf8e366f7bb7cf22564074adbaa546b344328d6fdbd593e5b38ef80` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/normalize.rs` | `27e63412a7a57c956dc82e602df4f93683adb3eb06866b22381157f85b96dcdc` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/storage.rs` | `a16a36d1880a5eb3c07856c39e4c7f6b18b9a168aa4cac0de93961ae34f6827b` |
| `/home/nathanael/.worktrees/brain-wiki-spielwissen-a/rust/crates/dbrain-sources/src/wiki_inventory/tests.rs` | `5e71ade9d106a9adffc5935a9d141741b58805a3d4cf15eb01d324901eb9bef4` |

Vor und nach späteren Datenworker-Läufen exakt diesen Freeze prüfen. Keine weiteren Modulschreibzugriffe durch diesen Fixer nach dieser Messung.

## Offene Nachweise

Ausführung der jetzt 33 Regressionen, Kompilierung, vollständige Echtdatenverarbeitung und eine echte Crash-Dauerhaftigkeitsprüfung sind nicht belegt. C2 prüft separat die tatsächliche Anbindung und Laufzeitgrenze des bereits auf der frischen Integrationsbasis vorhandenen `HttpClient::get_bounded`; kein neuer Corepfad wurde gebaut.

`gate_hook.py --review` bleibt ausstehend. Kein verifizierter Sol-only-Gate-Aufruf wurde in diesem Auftrag ausgeführt; keine Modell- oder Gate-Defaults gestartet. Übergabe ausschließlich an A, Paket a/Versuch 1. A bleibt alleiniger Statusproduzent teil-a.
