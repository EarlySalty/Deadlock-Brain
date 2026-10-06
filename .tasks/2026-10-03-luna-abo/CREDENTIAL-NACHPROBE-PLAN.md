# Korrigierte synthetische Credentialprobe, noch nicht ausgeführt

Die erste Probe scheiterte vor dem Loader, weil `systemd-run` das Argument `%d` wörtlich übergab. Der neue Plan benutzt einen vollständig aufgelösten normalen Pfad und keinen Specifier im bwrap-Aufruf. Keine neue Anwendungssource, kein Compiler, kein Modellaufruf und keine echte Infisical- oder DB-Verbindung.

Lokaler Metadatenbefund: `systemd-path user-runtime` meldet `/run/user/1000`, installierte Version ist systemd 255.4. Im offiziellen [systemd-v255-Quellcode](https://github.com/systemd/systemd/blob/v255/src/core/exec-credential.c#L70) bildet `get_credential_directory` den Pfad aus Runtimepräfix, `credentials` und vollständigem Unitnamen. Diese Quellregel begründet den Vorschlag; erst die echte transiente Metadatenprobe bestätigt ihn lokal.

Verwendetes Debugbinary: `/home/nathanael/.worktrees/brain-luna-abo-20261003/rust/target/debug/deadlock-brain`, belegter Buildhead `619dcb06d621f812699460427fa6d57fae056bb5`, SHA256 `6778674d0d016a76128c8e9d40403b6893a2558bfd4046d8ba32a5efecc5b649`. Vor dem Probeaufruf werden dessen vorhandenes Commitobjekt, historischer Snapshot und Binaryhash erneut gebunden. Der spätere Validatorhead ersetzt diesen bereits belegten Buildhead nicht. Ein eigener temporärer Fixtureordner enthält zwei echte Hardlinks dieses Binarys mit gleicher Inode und Hash, die normalen Job-TOMLs mit exakt den bisherigen Datenpfaden sowie ausschließlich synthetische Credentialfälle. Der Socketpfad wird auf `@AUDIT_DIR@/never-created-infisical.sock` gesetzt. Dieser eindeutig eigene Pfad wird niemals als Socket angelegt; sein Fehlen wird vor dem Aufruf als Metadatum geprüft.

Für den konkreten Fall Sheet-Sync/FIFO lautet der vollständige eigene Unitname `brain-abo-fifo-619-r2-sheet-fifo.service`. Damit ist der vorgeschlagene tatsächliche Manager-Credentialpfad `/run/user/1000/credentials/brain-abo-fifo-619-r2-sheet-fifo.service`. Der neue Aufruf enthält zusätzlich `ExecStartPre` mit dem vorhandenen `/usr/bin/stat`: Es gibt ausschließlich die Pfadmetadaten des vom Manager angelegten Verzeichnisses und seiner synthetischen regulären Credentialdatei aus. Keine Datei wird inhaltlich ausgegeben. Scheitert diese Prüfung, ist die ganze Probe BLOCK.

Der Exit von `stat` allein reicht nicht. Die äußere Probeauswertung verlangt exakt die zwei erwarteten Pfade, Dateityp Verzeichnis beziehungsweise reguläre Datei ohne Symlink, UID 1000 und eine Managerdateigröße entsprechend der eigenen regulären synthetischen Quelle. Geräte- und Inodefelder werden vollständig gebunden; sie müssen vorhanden sein und dürfen nicht mit der Quellinode gleichgesetzt werden, weil systemd die Datei kopiert. Die synthetische Quelle wird vorher ausschließlich auf Typ, Eigentümer und Größe geprüft. Jede fehlende oder widersprüchliche Metadatenzeile ergibt BLOCK.

```bash
timeout --kill-after=1s 10s systemd-run --user --wait --pipe \
  --unit=brain-abo-fifo-619-r2-sheet-fifo.service \
  --property=Type=oneshot --property=TimeoutStartSec=5s \
  --property=LoadCredential=infisical-token:@AUDIT_DIR@/manager-fixture \
  --property='ExecStartPre=/usr/bin/stat --terse /run/user/1000/credentials/brain-abo-fifo-619-r2-sheet-fifo.service /run/user/1000/credentials/brain-abo-fifo-619-r2-sheet-fifo.service/infisical-token' \
  /usr/bin/bwrap --die-with-parent --unshare-net --ro-bind / / \
  --proc /proc --dev /dev --tmpfs /opt/deadlock-brain \
  --ro-bind @AUDIT_DIR@/bundle /opt/deadlock-brain/fifo-audit/619dcb06d621f812699460427fa6d57fae056bb5 \
  --ro-bind @AUDIT_DIR@/credential-cases/fifo /run/user/1000/credentials/brain-abo-fifo-619-r2-sheet-fifo.service \
  /opt/deadlock-brain/fifo-audit/619dcb06d621f812699460427fa6d57fae056bb5/legacy/sheet-sync/deadlock-brain \
  entities --query synthetic-fixture

systemctl --user show brain-abo-fifo-619-r2-sheet-fifo.service \
  --property=Id --property=LoadCredential --property=Result --property=ExecMainStatus
```

`--collect` entfällt hier bewusst, damit die erwartete fehlgeschlagene eigene Unit bis zur anschließenden Metadatenabnahme geladen bleibt. Ihre tatsächliche `Id` muss exakt dem gerenderten eigenen Unitnamen entsprechen; die tatsächliche `LoadCredential`-Property muss genau `infisical-token:@AUDIT_DIR@/manager-fixture` binden. Es werden keine Environment- oder Credentialinhalte abgefragt. Fehlende Unitmetadaten ergeben BLOCK. Nach dieser Auswertung werden ausschließlich die eigene Unit gestoppt beziehungsweise zurückgesetzt und ihr vollständiger Abbau bestätigt.

Der Manager lädt immer dieselbe ausdrücklich synthetische, reguläre Datei. Die nicht geheimen FIFO-, Symlink- und sonstigen Negativfälle liegen nur im privaten bwrap-Bind. Der vollständige Unitname und beide Credentialpfadargumente werden pro Job/Fall vor dem Start konsistent gerendert. Die Manager-Credentialdirectory-Metadatenbindung bleibt dadurch unverändert, während ausschließlich die Sicht des jeweiligen bwrap-Namespace überlagert wird. Keine neue ENV-Konfiguration, kein Lesen von Prozess-ENV, keine Credentialvariable als eigener Configpfad und kein Shellprozess in der Unit.

Das Symlinkziel des Negativfalls ist ausschließlich `@AUDIT_DIR@/manager-fixture`, die eigene nicht geheime reguläre Datei. Es darf kein anderer Hostpfad als Ziel verwendet werden.

`--unshare-net` verhindert keine Verbindung zu einem per ro-bind sichtbaren pfadbasierten Unixsocket. Die normale synthetische Infisicalconfig benennt deshalb ausschließlich den oben belegten, eigenen und unerreichbaren Fixture-Socket. Kein echter Infisical- oder DB-Socket gehört zum adressierten Probeweg. Es wird keine Host-Credentialdirectory kopiert oder als Quelle benutzt: Die einzige Managerquelle ist die selbst geschriebene reguläre synthetische Datei, und der Runtimepfad gehört ausschließlich der jeweiligen eigenen transienten Unit. Die Anwendungsconfig enthält keine echte Credentialdatei und öffnet keine FD-Ersatzquelle.

Fälle je direktem Hardlink: fehlend, writerlose FIFO, Symlink, unlesbar, ungültiges UTF-8, übergroß und leer. Jeder Fall muss nach erfolgreicher `ExecStartPre`-Metadatenprüfung den tatsächlichen Infisical-Credentialloader erreichen und dort früh failclosed scheitern. Die bekannte generische Credentialfehlerklasse aus dem bestehenden Loader wird belegt; weder Tokeninhalte noch private Eingaben werden protokolliert. Exit 124/137, Managerfehler, Namespacefehler, fehlende Metadaten oder eine unerwartete Transportfehlermeldung sind niemals PASS.

Am exakt gebauten Commit `619dcb06d621f812699460427fa6d57fae056bb5` ist die Quellkette `entities → pg_pool_for_command → pg_pool_read_only → database_dsn → load_credential_from → read_credential_path → open_regular_file` gebunden. Der letzte Öffner nutzt bereits `O_NONBLOCK | O_NOFOLLOW | O_CLOEXEC` und prüft den geöffneten Descriptor vor dem Lesen auf reguläre Datei. Der ursprüngliche gegenteilige Reviewerbefund wurde nach Prüfung dieses exakten Commitobjekts zurückgezogen. Die neue Probe muss trotzdem die tatsächlichen Loaderfehler belegen und darf diesen statischen Befund nicht als Laufzeit-PASS ausgeben.

Die Ausführung benötigt die ausdrückliche Freigabe dieses korrigierten Probeplans. Danach gelten die aktuellen blockierenden Hostlocks, frische Probe nach beiden Erwerb und maximal der genehmigte Runtimeumfang. Es gibt keine zusätzliche Compilerprüfung. Nach den Proben werden ausschließlich die eigenen Units und der private Fixtureordner vollständig entfernt, eigene Sperrhalter regulär beendet und die tatsächlichen Gegenproben berichtet. Werkzeuge, Abo, Isolation/Persistenz und gemeinsamer Release bleiben getrennte Abnahmegrenzen.
