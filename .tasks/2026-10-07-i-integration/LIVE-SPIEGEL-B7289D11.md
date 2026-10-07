# Tatsächlicher E-Livebeweis nach Mainlieferung

## Regulärer Release und Prozess

Gelieferter Main-SHA b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2. Eigener sauberer unveränderter Releasebaum /home/nathanael/.worktrees/brain-i-release-20261007. Bestehender root-eigener brain-release: initialer tatsächlicher Workspacebau 5m 06s, reguläres Install mit zwei unabhängigen vollständigen Neubauten 5m 26s und 5m 37s, jeweils 17 tatsächliche Binaries. Install Exit 0, Hintergrundkennung bsdffv6kk, vollständiges Original /tmp/brain-i-mirror-b7289d11-release-install.log mit normalem Read geprüft. Format-2-Manifeste identisch, alle 17 installierten Binaryhashes in jedem der beiden Layouts selbst erneut geprüft, 17/17 und 17/17 gleich. Beide Zeiger auf b7289d11. Kein eigener alternativer Installer.

Erster regulärer brain-serve-Neustart wurde vor Ausführung von R10 verweigert: Deploy-Verifiziererrolle nicht als geladen erkannt. Den ausdrücklich genannten Rollenpfad /home/naniadm/Documents/claude-config/skills/rolle-deploy-verifizierer/SKILL.md regulär mit Read geladen. Danach identischer normaler systemctl --user restart brain-serve.service tatsächlich Exit 0. Keine Rechte-, Unit- oder Wrapperänderung für diesen Neustart. K-Neustartweg damit nicht pauschal beurteilt; hier ist der konkrete reguläre I-Erfolg belegt.

PID 2388861 -> 3178539, NRestarts=0. /proc/3178539/exe ist /opt/deadlock-brain/maintenance-releases/b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2/brain-serve, ohne deleted. Tatsächlicher laufender Binaryhash 2d6523da45dd84efb7812214e9eef07fcd4958669dab6894624fc356eec84bd7 entspricht dem Manifest. Fehlerjournal mit -p err seit 1 Minute: 0 Einträge. /readyz HTTP 200, Content-Type application/json, status ready. release_id=maintenance-5819bf58f1b7aebf5dfce8533c182efb1e15770f59a08c213745a71b7098b92f ist der Wissens-/Laufzeitbezug dieser Antwort, nicht als Binary-SHA ausgegeben.

ASCII-Anker mirror_complete ist im tatsächlich installierten deadlock-brain-Binary vorhanden; SHA256 ec24337a09a8e9fa5f699736cf2dd172098275edce4ec6ac9fa01e4038a79b9e. Derselbe Anker ist in brain-serve nicht enthalten; dort belegen Manifesthash, Prozesspfad und tatsächliche Readyantwort die Herkunft, kein erfundener Ankertreffer.

## Erster tatsächlicher vollständiger Assetsimport

Bestehender dauerhaft verdrahteter deadlock-brain-build-data.service regulär gestartet, 07.10.2026 23:57:52 CEST; Hintergrundkennung bi0yozoey. Rust-Binary kommt über installierten current-Zeiger, nicht aus einer Debug- oder Worktreebinary. Vorhandener vollständiger Assetsaufruf, anschließend vorhandenes build-data --hero all. Keine lokalen population-/Matchimporte. Die Unit war beim letzten Vorcheck noch activating; Result=success beziehungsweise ExecMainStatus=0 im aktiven Job ist kein Gesamtabschluss.

Bestehende reine Rust-Readerprobe am gelieferten E-Code bindet reale SourceRun-/Manifest-/Endpoint-Receipts und prüft die unveränderten tatsächlichen Originaldateien. Erster direkter Launcheraufruf scheiterte an fehlendem regulärem Runtime Credential, Exit 1, keine Probe gelaufen. Danach denselben vorhandenen Launcher und dieselbe Probe über reguläres systemd-run --user mit LoadCredential und dem schon bestehenden FD-5-Muster ausgeführt. Kein Secretwert, keine Berechtigungsänderung oder künstliche Datenkorrektur. Probe tatsächlich Exit 0, Runtime 1min 45.746s. Original /tmp/brain-i-mirror-b7289d11-live-receipt-proof-credential.log normal Read geprüft; Ausgabe enthält ausschließlich IDs, Hashes, Zeitangaben, Größen und Pfade, keine Originalpayloads.

source_runs id=743, source assets, status ok, mirror_complete=true, client_version=6759. Manifestdokument 56830, Original-SHA256 94ed4919c0a362e9ab726a111a6a3e1d71786c0ce9039de6f1b58f7dd3c7a3de, Parserrevision dbrain-assets/4. Dauerhafte Originalpfade unter /home/nathanael/.local/share/deadlock-brain/raw/deadlock_assets_api/, außerhalb von Worktrees.

Alle 13 Endpoint-Readerladungen gleich zum Receipt-bezogenen Payload und nicht leer; jeweils eigener tatsächlicher Datei-SHA256 gegen Datenbank und Receipt geprüft:

| Art | Englisch | Deutsch | andere Sprache |
| --- | --- | --- | --- |
| items | 746 | 746 | |
| heroes | 40 | 40 | |
| heroes_all | 65 | 65 | |
| generic_data | 27 | 27 | |
| npc_units | 93 | 93 | |
| misc_entities | 105 | 105 | |
| modifiers | | | 113, ohne Sprachkennung |

Gemeinsame Version und Runbindung sind tatsächlich belegt, kein behaupteter Balancepatch und kein Vollständigkeitsurteil aus HTTP 200. 14 Originalhashbelege einschließlich Manifest im lokalen Log erhalten. Originalbytes oder Community-Rohdaten nicht in Git aufgenommen.

LIVEBEWEIS[DV-1]: PID 2388861->3178539 | exe ohne (deleted) | journal -p err leer | Anker "mirror_complete" in installierter CLI | Funktion: vollständiger Assets-Run 743, Version 6759, alle Reader-/Receipt-/Originalhashbindungen geprüft | Ort: http://127.0.0.1:8788/readyz und brain.source_runs id=743

## Tatsächlicher Abschluss des Gesamtjobs und offenes F

Gesamtjob assets + build-data --hero all tatsächlich am 08.10.2026 00:05:51 CEST beendet, ExecMainCode=exited, ExecMainStatus=0, Result=success, MainPID=0, inactive/dead. Der bestehende Timer ist active/waiting, nächster Lauf 03:30 CEST. Nachgang der vorhandenen reinen Rust-Readerprobe enthält nur zusätzliche lesende Importmetadaten, keine neue Produktionsfunktion: build_source_run id=744 source=build_data status=ok, heroes_count=40. Vollständige Assets-/Originalhashprüfung erneut unverändert bestanden, regulärer Runtime-Credentiallauf Exit 0, 2min 4.241s. Lokales Original /tmp/brain-i-builddata-run-proof-live.log mit normalem Read geprüft. Eigene Probe mit cargo-slot fmt/build und strengem --no-deps-Clippy jeweils Exit 0; private Probe, kein neuer Produktrelease. Keine Raw-Buildpayloads oder Fehlerstrings aus der Importsummary ausgegeben.

analytics_runtime ist als konkrete Datei an G freigegeben, FREIGABE-ANALYTICS-RUNTIME-AN-G.md; Delegator hat die Übergabe bestätigt. Genau ein bestehender nativer F-Ausführer nach diesem tatsächlichen E-Importnachweis fortgesetzt. Gesicherter G-S3/S4-Vertrag inzwischen geordnet geliefert und weitergegeben; 38 Rechenfälle, keine behauptete grüne G-Vollsuite aus 342/12. Kein zweiter Rechner, noch kein F/G-Endbeweis oder Warden-Publish. Keine fremde Dienststeuerung, keine K-/G-Literalumstellung. Vorbestehend fehlgeschlagene brain-maintenance-Unit nicht als neu erfolgreich gestartet behauptet. Kein Cleanup oder Self-Settle bei offenem I-Gesamtauftrag.
