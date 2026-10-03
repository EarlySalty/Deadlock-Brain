# Finale Prüfung und gemeinsame Abnahme

Aktueller Vorrang: Der nachfolgende Nutzerbefund über blockierende FIFO-Öffnungen macht eine weitere enge Fixrunde erforderlich. Der aktuelle ungeprüfte Drei-Dateien-Stand und sein gesondert abzustimmender Prüfplan stehen in [FIFO-NACHSLOT.md](FIFO-NACHSLOT.md). Die unten genannte grüne Fixrunde 2 ist ein historischer Beleg für 940bb213; sie ist keine Freigabe des anschließenden FIFO-Fixes.

Fixrunde 2 ist compilerseitig vollständig geprüft: Format-, Compiler-, Tests und Clippy mit `-D warnings` sind grün, 657 Tests bestanden und 48 ignoriert. Der vollständige Quellinhalt steht in `FIXRUNDE-2-SNAPSHOT.tar` und `FIXRUNDE-2-MANIFEST.sha256`. Die folgenden noch offenen Schritte sind Gate, gemeinsame Integration und tatsächliche Laufzeitabnahme. Die erste Suite und Fixrunde 1 bleiben historische getrennte Artefakte.

## Nächster eigener Slot

Die verbindliche Reihenfolge ist Twitch, UI, Discordexport, Patchnotes, Discordnachprüfung, Docsnachprüfung, unsere Nachprüfung, gemeinsame C9-Nachsuite. Erst die ausdrückliche Docs-Übergabe eröffnet unseren Slot. Beide Hostlocks, frische vollständige Compilerprobe und maximal zwei Jobs bleiben erforderlich.

Die enge Nachprüfung betrifft `brain-providers`, `deadlock-brain-core` und `brain-maintenance`, deren Tests und Clippy mit `-D warnings`. Ein Compilercheck der drei Binaries sichert die Schnittstellen zu Serve, Maintenance und Legacy. Vor dem Start werden Manifest und Snapshot gegen die tatsächlichen Quelldateien geprüft. Änderungen während des Laufs sind ausgeschlossen. Bestehende Tests der unveränderten Komponenten haben ihren grünen ersten Lauf; eine erneute Vollsuite braucht einen konkreten Grund oder gehört zur gemeinsam integrierten C9-Nachsuite.

Nach grünen Quellprüfungen enthält der eigene Commit sämtliche eigenen Rust-, Cargo-, TOML- und Opsdateien. Das bestehende Gate prüft genau diesen HEAD gegen Basis `511a347b653beba13c2bf130f4bead7a7196cc2a`. Sein Ergebnis wird gesondert dokumentiert. Danach endet unser Slot mit vollständigem Prozessabbau und freien Lockgegenproben; der geprüfte Eigencommit geht unmittelbar an C9. Kein Einzelmerge oder eigenes Release.

## Tatsächliches Werkzeugschema

Die installierte CLI ist 0.160.0. Der Connector schaltet Shell, Unified Exec, Bildfunktionen, Browser, Computer Use, Apps, Plugins, MCP, Agenten, Skill-Suche, Schlafwerkzeug, Goals, Code Mode Host, Workspace Dependencies und Worktrees ausdrücklich ab. Die Featureliste bestätigt die letzten vier als vorhandene stabile Schalter. Das entfernte Feature `apply_patch_freeform` liefert keinen Nachweis für die heutige Patchwerkzeugregistrierung.

`codex debug prompt-input` und Appserver-Protokollschemata belegen nicht das tatsächliche Exec-Werkzeugarray. Die spätere synthetische Requestprobe braucht eine isolierte Sicht ohne vorhandene Authdateien und einen ausschließlich lokalen Transport ohne Authentifizierung. Sie darf ausschließlich Modellname, `tools` und `tool_choice` aus dem Requestbody erfassen. Header, Zugangsdaten, private Inhalte und fremde Konfiguration werden weder gespeichert noch geloggt. Kein anderer Anbieter oder reales Modell wird aufgerufen.

Die Probe soll dieselben Produktionsflags und Modellmetadaten verwenden. Jede erforderliche Abweichung, etwa ein anderer Providername oder geänderte Authkonfiguration, wird benannt; daraus folgt keine automatisch bewiesene Gleichheit zum Abo-Aufruf. Abnahme verlangt ein tatsächlich leeres `tools`-Array. Bleiben Plan-, Patch- oder andere Werkzeuge, ist Werkzeugfreiheit nicht erfüllt. Fehlende Werkzeugereignisse in einer Antwort genügen nicht. Der echte Abo-Aufruf wird erst später mit synthetischem Inhalt unter `gpt-6-luna` und erzwungener ChatGPT-Anmeldung geprüft.

## Gemeinsame Laufzeitbelege durch C9

| Beleg | Erforderliches Ergebnis |
| --- | --- |
| Beide direkten vollständigen SHA-Hardlinkpfade | Tatsächlicher Exepfad und Jobwurzel jeweils im richtigen Releasejobstamm |
| Binarydateien | Gleicher SHA256 und gleiche Inode beider Hardlinks auf das einmal kompilierte `deadlock-brain` |
| Sheet-TOML | `config/bot.toml`; effektives `data_dir` exakt `/home/nathanael/repos/Deadlock-Brain/data` |
| Builddata-TOML | `config/bot.toml`; effektives `data_dir` exakt `/home/nathanael/.worktrees/brain-live-main/data` |
| TOML fehlt, unlesbar oder ungültig, jeweils beide Startpfade | Fehler vor Nutzung von Ersatzconfig oder Datenstand; kein CWD-, JSON- oder ENV-Fallback |
| Installierte Settings unvollständig oder Datenpfad relativ | Fehler auch beim direkten Population-Datenbankpfad |
| Benanntes systemd Credential fehlt oder unlesbar | Fehler ohne FD- oder andere Credentialausweichquelle |
| Tatsächlicher FD-Aufrufer ohne Runtime-Credentialdirectory | Vorhandener sicherer FD-Vertrag bleibt erhalten |
| Serve und Maintenance | Gemeinsame Bot-TOML, gpt-6-luna, vorhandener Abo-Zugang; kein API- oder DeepSeek-Fallback |
| Prozessrunner | Timeout, gesamte Prozessgruppe, beide Ausgabebytegrenzen und Reasoning-Weitergabe nachgewiesen |

`ai-model` ist die lesende Root-/Config-/Datenpfadprobe. Sie lädt keine Infisical-Secrets und öffnet keine Datenbankverbindung. Negative Runtimeproben werden gemeinsam so isoliert, dass sie weder produktive Daten schreiben noch den laufenden Dienst verändern. Erst C9 rendert `@RELEASE_ROOT@` in den Unitvorlagen zu vollständigen SHA-Pfaden; der gerenderte Diff gehört ins Gruppengate.

Die gemeinsame Servicekonfiguration enthält die bisherigen Maintenancefelder direkt unter `[brain.maintenance]` und die bisherigen Runtimefelder unter `[brain.maintenance.runtime]`. `maintenance_config`, `serve_config` und `infisical_config` zeigen dort auf exakt dieselbe absolute vollständige SHA-Release-Datei `config/bot.toml`. Serve liest `[brain.serve]`, sein Zeitlimit `brain.serve.timeouts.provider_ms`; der sichere Secretloader erhält `[brain.infisical]` im Speicher. Die private Operatorregistry bleibt unter `[brain.operator]` mit eigener Credential-Untertable. Keine erzeugte JSON-Zwischendatei und keine zusätzliche Secretquelle.

Es gibt keinen belegten harten CLI-Tokendeckel. Nullbudget stoppt vor Prozessstart; Vorfilter und Verbrauchprüfung begrenzen akzeptierte Antworten, garantieren aber keinen maximalen Gesamtverbrauch eines bereits gestarteten Abo-Aufrufs. Timeout und Bytegrenzen werden nicht als Tokengrenze dargestellt.
