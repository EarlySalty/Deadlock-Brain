# Beurteilung des Rust-Cutover-Altbranches

status: erledigt, 08.10.2026 (Beurteilung; Checkout-Abschluss noch blockiert)

## Urteil

Kein eigenständig übernehmbarer Produktionsanteil. Die heutigen Patch-, Antwort-, Release- und Betriebswege ersetzen den Großteil der Arbeit. Drei noch brauchbare Vertragsideen betreffen die laufenden Pakete I/G und werden unten als Übergaben festgehalten. Dafür entsteht kein weiterer Import-, Antwort- oder Veröffentlichungsweg. Der alte Branch wird nicht nach main gemergt.

**Beurteilungsbasis:** frisch geholtes `origin/main` = `b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2`. Graphify wurde zuerst befragt; dessen Fundstellen spiegeln zum Teil den alten Checkout. Die Aussagen unten wurden deshalb zusätzlich am Git-Baum dieses main-SHAs geprüft, nicht aus dem Graphen als heutiger Ist-Zustand übernommen.

**Kategorien:** (a) vorhanden oder fachlich durch Brain v2 ersetzt; (b) noch brauchbar, aber fehlend; (c) veraltet oder im heutigen Auftrag nicht übernehmbar. (a) bedeutet nicht automatisch einen identischen Cherry-pick. Die alten 21 Commit-SHAs liegen nicht als solche auf main.

## Sicherung und SHA-Backup

| Stand | Vollständiger SHA | Nachweis |
| --- | --- | --- |
| Lokaler Ausgangsbranch mit den genannten 21 Commits | `2734c2da4e814ff79953e8e825275b0216a6af16` | Vor der WIP-Sicherung; Paketinhalt gegen main verglichen |
| Lokaler WIP-Sicherungscommit | `f69f4d186736f51aef941e10ce5491d640e09f3b` | 578 Dateien: 12 Quell-/Betriebsdateien und 566 Markdown-Akten |
| Zusätzlich vorgefundener Remote-Stand | `c8ad3ef6bb715c2bc747f89174cd998ccd8513d1` | 14 zusätzliche Remote-Commits seit dem gemeinsamen Ausgangspunkt |
| Gemeinsamer Sicherungsstand | `c8ceb5d66a09fdfd7b1a99b60befc9f0f381c7a0` | Archiv-Merge mit beiden Ständen als Eltern; kein Produktionsmerge |
| Tag-Objekt `archiv/brain-rust-cutover-20260919` | `070a126831e87b14502cdb969c6cc0523a0a1a47` | Remote-Tag und aufgelöster Commit per `ls-remote --exit-code --tags`, Exit 0 |

Der erste Branch-Push wurde wegen des fortgeschrittenen Remotes mit `non-fast-forward`, Exit 1, abgelehnt. In einem eigenen isolierten Archiv-Worktree wurden beide Historien regulär verbunden. Bei zehn kollidierenden Laufzeitdateien gilt im gemeinsamen Archivbaum die spätere Remote-Fassung; die lokale WIP-Fassung bleibt über den Elterncommit vollständig erreichbar. Der anschließende Branch-Push und der Tag-Push waren erfolgreich. Kein Force-Push, Reset oder Stash.

`merge-base --is-ancestor` für den lokalen WIP, den vorgefundenen Remote-Stand sowie die lokalen und Remote-Branch-Refs gegen den Archiv-Tag: jeweils Exit 0. Direkt vor der Remote-Löschung zeigte `ls-remote --exit-code --heads` exakt auf `c8ceb5d66a09fdfd7b1a99b60befc9f0f381c7a0`, Exit 0. Der Altbranch wurde lokal und remote gelöscht. `branch -d` lehnte wegen des absichtlich nicht erfolgten main-Merges ab; erst nach dem Tag-/Vorfahrenbeweis wurde der ausdrücklich beauftragte Archivabschluss mit `branch -D` ausgeführt.

### Abweichung von der angenommenen WIP-Größe

Es waren nicht lediglich rund 20 einzelne Dateien: `ls-files --others --exclude-standard` fand 9.220 Dateien mit 12.854.715.038 Bytes. Darunter liegen Cargo-Ausgaben, verschachtelte Git-Sicherungen, Binaries, Rohdaten und Prüfprotokolle. Diese wurden nicht ungeprüft nach GitHub übertragen.

Die Aufgabenbäume sind zusätzlich unverändert lokal gesichert unter:

`/home/nathanael/.worktrees/brain-cutover-lokalsicherung-20261008/.tasks/`

Die zwölf Quell-/Betriebsdateien und die Markdown-Akten wurden vor dem WIP-Commit geprüft. Der Quell-Diff hatte bei Gitleaks Exit 0. Die Textprüfung meldete drei `generic-api-key`-Treffer: einen technischen Markdown-Dateinamen und zwei zusammengezogene Hashzählungslabels, keine Zugangsdaten. Werte wurden nicht ausgegeben. Nicht veröffentlichte Artefakte bleiben in der Lokalsicherung und am bisherigen Aufgabenpfad erhalten. Ein abweichender lokaler Q-Status ist zusätzlich als `Q-STATUS-vor-main.json` gesichert. Eine seit dem 03.10. vorhandene leere `index.lock` hatte keinen Prozessbesitzer und wurde vor der Sicherung nach `index.lock-20261003` in der Lokalsicherung verschoben.

## Die ursprünglichen 21 Commits

| Commit | Thema | Einordnung | Entscheidung und heutiger Bezug |
| --- | --- | --- | --- |
| `6c33ddc9` | Actions für Patch-Evidenz und Historienmigration | (c) | GitHub Actions sind abgeschafft; keine CI wiederherstellen. |
| `da3b55ea` | Eigenständige Patchentwürfe, Historie, Captions, Insight-Import | gemischt (a)/(b) | Revisionsspeicher durch v2 ersetzt; unabhängige begründete Entwürfe und Caption-Provenienz als Übergaben G bzw. I/G. Die alte CLI mit eigenen Legacy-Tabellen nicht übernehmen. |
| `37049973` | Inhaltsbezogene Erstbeobachtung und Caption-Text-Hash | (b), Übergabe I/G | Nützliche Zeit- und Provenienzverträge, auf main keine gleichnamige Patch-/Caption-Implementierung. Alter Trigger-/View-Vertrag hängt an Legacy-Tabellen. Kein isolierter Writer ohne passende Leser. |
| `6e4bdcae` | Actions mit Folgemigration und Caption-Integration | (c) | Assertions als Referenz erhalten; kein neuer Actions-Auftrag. |
| `d510c630` | Beschreibung der Zeit- und Caption-Verträge | (b), Referenz I/G | Fachliche Semantik nutzbar, Befehle und Tabellenbezüge veraltet. Dokumentation nicht als heutigen Betrieb ausgeben. |
| `2c57ff1d` | U0-Auftrag und damaliger Bericht | (c) | Historische Akte im Tag erhalten, kein aktueller Arbeitsauftrag. |
| `a168371c` | Kanonische Patch-ID, `item_or_ability`, Test-DSN | gemischt (a)/(b) | Identitätsidee nützlich; v2 nutzt stabile Quellen-/Revisionsidentitäten und API-Spiegel. `patch_<changelog_id>` und die Legacy-Entity-Klasse nicht als neues öffentliches Schema einführen. Mehrdeutigkeit und Erstbeobachtung an I/G übergeben. |
| `e72153a9` | Identitätsregressionen und CLI-Smoke | (b), Referenz I/G | Reimport, Alias und ID-Wechsel als Prüffälle geeignet; alte CLI und Scratch-Schemas nicht aktivieren. |
| `a061563e` | Dokumentation der Patch-ID und damaliger Blocker | (c) | Beschreibt den damaligen Pfad, nicht Brain v2. |
| `9efeb1e4` | Damalige Review-Korrekturen R1 bis R3 | (c) | Prüfgeschichte erhalten, kein Freigabebeleg für einen heutigen Diff. |
| `a2ad50e9` | Kontextauswahl und revisionssicherer Patch-Sync | gemischt (a)/(b) | Heutiger Import/Release durch I und v2; deterministische, belegte Mechanikauswahl als G-Referenz. `pg_patchnotes.rs` ist I-Schreibbereich. |
| `1528b1fc` | Release-Auftrag und Zwischenreport | (c) | Historischer Stand ohne heutige Freigabewirkung. |
| `dc941527` | ID-Umzug invalidiert alten und neuen Entwurf | (b), Übergabe I/G | Nützlicher Prüffall bei Quellenkorrekturen. Migration ist an `patch_review_runs` gebunden und darf nicht neben den v2-Revisionsspeicher gesetzt werden. |
| `2135e4b7` | Kompakte Mechanikprojektion und belegte Held/Fähigkeit-Links | (b), Übergabe G | Wiederverwendbare Auswahlprinzipien. Heutige Zahlen/Verknüpfungen aus I-Spiegel und G-Werkzeugen; keine zweite Snapshot-Rechenschicht. |
| `0eac0c48` | R6 Quellen-/Event-Konsistenz und jq-CI | gemischt (a)/(c) | Quellenkonsistenz gehört in den heutigen I-Import und v2-Releaseweg. jq-Actions-Anteil entfällt. |
| `eab314ed` | Patch-ID, offizieller Refresh, R6, verschiebbarer Repo-Pfad | gemischt (a)/(c) | I hat den API-Spiegel und heutigen Patchimport; v2 hat eigene Laufzeitkonfiguration. Alten ENV-/Manifestpfad nicht hinzufügen. |
| `a1fad5c7` | URL-Parsing, Trigger-Gate und Übergrößenprüfung | (a), Referenz I/G | Sicherheits- und Größenprinzipien behalten; heutiger I-Parser enthält eigene Quellenauflösung und Regressionen. Kein paralleler Refresh-Einstieg. |
| `9ead4617` | Historien-MCP in Python und Sync-Wrapper | gemischt (a)/(c) | Rust-`brain-mcp` leitet auf `brain_answer` weiter; `PatchHistory` ist im v2-Werkzeugvertrag vorgesehen und G zugeordnet. Kein Python-MCP und kein zweiter Sync-Wrapper. |
| `e48d58c4` | Veraltete Video-Claims sperren, Releaseverträge | gemischt (a)/(b) | v2 besitzt Patchgültigkeitsprüfungen. Spezifische Caption-/Claim-Revalidierung ist auf main nicht übernommen; Writer und G-Leser zusammen bearbeiten, nicht hier teilweise aktivieren. |
| `458d56d0` | Geprüfte Artefakte und damalige Deploy-Blocker | (c) | Die September-Artefakte sind kein aktuelles Release. Keine Installation aus Dateialter oder altem Reviewbericht. |
| `2734c2da` | Rust-Planpaket v1.0 | (a) | Der gesamte Verzeichnisbaum ist mit main identisch: `git diff --quiet` für `architecture/migration/deadlock-brain-rust-planpaket-v1.0`, Exit 0. |

## Zusätzlich entdeckte Remote-Fortsetzung

Diese Historie war in der Ausgangsbeschreibung nicht enthalten. Sie ist ebenfalls im gemeinsamen Archiv-Tag erreichbar.

| Commits | Thema | Einordnung | Entscheidung |
| --- | --- | --- | --- |
| `8ac107f8`, `6550a5c5`, `585fed25`, `285a35c4` | Rust-Secret-Loader, Betriebswrapper, Dotenv-Ablösung, Tokio-Fix | (c) für diesen Auftrag | Neuer ENV-basierter Legacy-Betrieb widerspricht der Grenze ohne neue ENV-Konfiguration und dem heutigen v2-Betriebsweg. WIP nicht ausrollen. |
| `692da20a`, `9cfec67e` | Eigenes Rust-MCP und vollständiger Legacy-Ask | (a)/(c) | Auf main bestehen `brain-mcp` und der eine typisierte `brain-serve`-Antwortweg. Kein weiteres Legacy-Ask/MCP aktivieren. Zuständigkeit G/K. |
| `b2008c62`, `63ec17f5`, `e7a5f3a9` | YouTube-Rust-Cutover und Abhängigkeiten | (c) für diesen Auftrag | Der fortgeschrittene Remote verändert den gesamten alten YouTube-Betrieb und seine Abhängigkeiten. Keine losgelöste Übernahme in den parallel fertiggestellten Antwortweg; Provenienzvertrag separat an I/G. |
| `b3b56002`, `2a0d472a` | Rust-Cutover-CI und Formatter-Baseline | (c) | Keine Wiederbelebung von Actions. |
| `efa99251`, `bc353345` | Pauschale Python-/Legacy-Entfernung | (c) | Entfernt unter anderem vorhandene Daten-/Match-/Antwortpfade pauschal. Heutiger Ersatz und Löschauftrag müssen je Pfad belegt sein; kein pauschaler Cutover aus dem Altbranch. |
| `c8ad3ef6` | Zweite Commitfassung desselben Planpakets | (a) | Der Planpaket-Baum ist bereits auf main; kein zusätzlicher Inhalt. |

## Nachgelesene heutige Ersatzwege

Die Zeilen beziehen sich auf den oben gepinnten main-SHA.

- `rust/crates/brain-storage/src/pg_release.rs:31-53`: Release-Validierung gegen vorhandene `source_record_revisions`, anschließend Speicherung in `corpus_releases_v1`. `:56-103` bewahrt bei importierten Wiederholungen die gespeicherte Releasezeit. Das ist kein Beleg für eine schon vorhandene spezielle Caption-Historie.
- `rust/crates/brain-feeds/src/patchnotes.rs:17-24`: typisierter, validierter Patchfeed. `rust/crates/deadlock-brain/src/pg_patchnotes.rs:216-225` bereitet den heutigen Import vor und startet seine Transaktion; `:1379-1575` ersetzt/materialisiert Events. Die alten Patchtrigger würden diesen I-Schreibpfad verändern.
- `rust/crates/brain-contracts/src/source_patch.rs:14-46`: kanonische Patchgültigkeit darf nicht mit einem widersprechenden Legacy-Patch überschrieben werden. Unbekannte Gültigkeit bleibt unbekannt. Das ersetzt nicht die spezielle Video-Evidenzmarkierung.
- `rust/crates/deadlock-brain/src/bin/brain-mcp.rs:247-291`: angeboten wird `brain_answer`, aufgerufen wird der bestehende typisierte Brain-Client. `rust/crates/brain-contracts/src/tools.rs:295` enthält den `PatchHistoryRequest`-Vertrag. G baut laut seinem Briefing die Patchgeschichte als Werkzeug im einen Antwortdienst.
- `rust/crates/dbrain-retrieval/src/lib.rs:4146-4299` liest Video-Claims. Die Altbranchänderung an diesem Leser überschneidet sich mit G; Caption-Writer allein wäre kein vollständiger Revalidierungsvertrag.

Zusätzliche Suche am main-Baum nach `patch_review_runs`, `patch_history_v1`, `youtube_caption_segments_v1`, `needs_claim_revalidation` und `transcript_evidence_hash` fand keine dieser Legacy-Implementierungen im heutigen Rust-/SQL-/Betriebsbaum. Die beiden Treffer für `first_observed_at` liegen in Populationsevidenz, nicht in Patch-/Caption-Historie. Die Revisionstabellen und Releaseverträge wurden dagegen in Storage, Feeds, Wartung und Readern gefunden. Vorhandene v2-Revisionsbausteine wiederverwenden, keine zweite Persistenz bauen.

## Übergaben statt paralleler Umsetzung

Dies sind fachliche Referenzen und Prüffälle, keine Erweiterung der bereits beauftragten I/G/K-Pakete durch diesen Worker. Die Hauptsession kann sie im jeweiligen bestehenden Paket einordnen.

| Paket | Brauchbarer Kern | Archiv-Fundstelle | Zusammengehörige Umsetzung / Nachweis |
| --- | --- | --- | --- |
| I + G | Inhaltsbezogene erste Beobachtung, stabile Patchidentität, Invalidierung bei ID-/Quellenwechsel | `37049973`, `a168371c`, `dc941527`; alte `scripts/migrations/2026-09-18-patch-evidence*.sql` und `tests/patch-understanding/*identity*`, `*r4*` | In den vorhandenen v2-Quellen-/Revisions- und Releaseverträgen abbilden. Reimport, Löschen/Reimport, Rückkehr zu gleichem Inhalt, mehrdeutige ID und Zuordnungswechsel prüfen. Quelldatum nicht als Beobachtungszeit behaupten. |
| G, Daten aus I | Eigenständige begründete Patchanalyse ohne Creator-Transkript als Wahrheit; kompakte belegte Mechanikauswahl | `da3b55ea`, `a2ad50e9`, `2135e4b7`; `deadlock-brain-patch-review.rs`, `docs/AUTONOMOUS_PATCH_REVIEW.md` im Tag | In vorhandene Werkzeuge und `/v1/answer` einordnen. Zahlen aus dem API-Spiegel, Verknüpfungen nur mit Beleg, fehlende historische Mechaniken als unbekannt. Kein Legacy-CLI-Veröffentlichungsdienst. |
| G, gegebenenfalls I für Quellenvertrag | Caption-Roh-/Text-Hash, Zeitsegmente und Sperre veralteter Video-Claims | `da3b55ea`, `37049973`, `e48d58c4`; `deadlock-brain-yt/src/transcripts.rs`, `claims.rs`, `dbrain-retrieval/src/lib.rs` im Tag | Writer, Claim-Speicher und verwendete Leser gemeinsam abnehmen. Gleicher Text mit neuer Zeitzuordnung ist neue Evidenz; geänderter Text darf keine alten Segmente/Claims als aktuell liefern. Die Legacy-Migration bündelt Patch und Caption und darf nicht unverändert übernommen werden. |

## Checkout-Abschluss: echte Blockade

`git switch main` wurde abgelehnt, weil `main` bereits im fremden Worktree `/home/nathanael/.worktrees/brain-live-main` ausgecheckt ist. Dieser Worktree wurde nicht verändert oder entfernt. Auch der gemeinsam benutzte lokale main-Zeiger wurde nicht verschoben.

Der Haupt-Checkout wurde stattdessen sauber auf den aktuellen main-Inhalt mit detached HEAD gesetzt. Die vorherigen Aufgabenpfade wurden ohne Überschreiben aktueller main-Dateien wiederhergestellt. Damit diese lokalen Altakten und Prüfartefakte nicht einen scheinbar schmutzigen Checkout erzeugen, wird ausschließlich dort eine worktree-lokale Git-Ausschlussdatei verwendet: `.git/info/cutover-local-excludes`. Die bestehenden globalen Ausschlüsse bleiben enthalten. Der neue Ernte-Worktree nutzt nachweislich weiter `/home/nathanael/.config/git/ignore`; fremde Worktrees erhalten diese Ausschlüsse nicht. Die lokalen Dateien sind dadurch nicht gelöscht und nicht automatisch zu einem main-Commit geworden.

**ABWEICHUNG:** Die wörtlich geforderte Rückkehr auf den Branch `main` ist offen, obwohl dessen aktueller Inhalt sauber vorliegt. Die Hauptsession muss zuerst die Belegung von `main` im vorhandenen Live-Worktree geordnet auflösen. Kein `--ignore-other-worktrees`, keine Branchumbenennung und kein Ref-Update unter einem fremden Checkout.

## Prüf- und Lieferumfang

Produktions-Cherry-picks: 0. Neue Laufzeitdateien, Migrationen und ENV-Konfiguration: 0. Keine Matchdaten erhoben oder in einen neuen Commit übernommen. I/G/K-Produktionsbranches und deren Worktrees unverändert.

Rust-Compiler-, Clippy- und Testläufe wurden nicht ausgeführt: Der Ernte-Diff besteht aus Aufgaben-Dokumentation. Die archivalischen WIP-Commits sind ausdrücklich kein getestetes Release. Deploy, Dienstneustart und Live-Funktionstest entfallen ohne Produktionsdiff. Das Archiv wird nicht als Nachweis für fertiggestellte Brain-Funktionen ausgegeben.
