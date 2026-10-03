# Enger FIFO-Fix und nächste Prüfung

Der historische Head `940bb2138d694ada783c2d7d8b4abecb4d20fbed` hatte 657 bestandene und 48 ignorierte Tests sowie Gate Exit 0 ALLOW. Der anschließend vom Nutzer benannte FIFO-Befund blockiert seine abschließende Übergabe. Wrapper und Kinder sind beendet, beide Locks sind frei. Der neue Drei-Dateien-Fix ist bisher ausschließlich statisch geprüft, ohne neue P1/P2 laut bestehendem Rust-Reviewer.

## Quellumfang

`config.rs` öffnet normale Configdateien mit `O_NONBLOCK | O_NOFOLLOW | O_CLOEXEC`, prüft den geöffneten Descriptor auf reguläre Datei und begrenzt die Lektüre auf 256 KiB plus ein Erkennungsbyte. Eine vorgelagerte Größenprüfung spart die Lektüre bereits zu großer Dateien; die zusätzliche Grenze greift auch bei Wachstum. `pg_secrets.rs` liest TOML nicht mehr redundant vor und benutzt für Credentialpfade denselben sicheren Öffner. Die vorhandene `read_at`-Grenze von 8193 Bytes, der Offsetvertrag und die Credentialdirectory-/FD-Priorität bleiben erhalten. `ai.rs` respektiert das kleinere von Aufrufer- und normalem Configtimeout.

Die neuen synthetischen Regressionen prüfen writerlose FIFOs mit einer begrenzten Ergebniswartezeit, Symlinks, nicht reguläre Dateien, Configgröße und die geöffneten NONBLOCK-/CLOEXEC-Flags. Bestehende Credential-Offset- und Prioritätstests bleiben enthalten. Alle Fixtures sind künstliche, nicht geheime Daten.

## Begrenzt abgestimmte Compilerprüfung

Erst nach Freigabe des konkreten neuen Heads: beide bestehenden Hostlocks erwerben, vollständige frische Non-Zombie-Compilerprobe, maximal zwei Jobs. Der neue Sourcefreeze bleibt während der Suite unverändert.

```bash
cargo test --locked --offline -j 2 -p deadlock-brain-core --lib
cargo clippy --locked --offline -j 2 -p deadlock-brain-core --all-targets -- -D warnings
cargo check --locked --offline -j 2 -p deadlock-brain --all-targets
```

Für die tatsächlichen Binaryproben muss anschließend das passende Debugbinary aus demselben Head vorliegen. Der dafür nötige enge `cargo build --locked --offline -j 2 -p deadlock-brain` ist vor Start ausdrücklich mit abzustimmen. Kein Releasebuild und keine erneute Vollsuite aller 657 unveränderten Tests. Danach prüft das bestehende Gate den neuen vollständigen Eigenhead gegen Basis 511a347.

## Isolierte Hardlinkproben, noch nicht zur Ausführung freigegeben

`@AUDIT_DIR@` ist ein eigener temporärer Fixtureordner, `@FIFO_HEAD@` der tatsächliche neue Source-SHA und `@TEST_BINARY@` dessen belegtes Debugbinary. Zwei echte Hardlinks liegen in `@AUDIT_DIR@/bundle/legacy/{sheet-sync,build-data}/deadlock-brain`. Je Job wird ausschließlich seine normale TOML-Vorlage kopiert. Die belegten `settings.data_dir` bleiben unverändert; die Infisicalmetadaten werden in der künstlichen Credentialprobe auf eine nicht vorhandene lokale Fixture-Socketdatei gesetzt. Es gibt keine Verbindung zu Infisical oder Postgres und keinen Modellaufruf.

Im bwrap-Namespace wird `/opt/deadlock-brain` mit einem privaten tmpfs überlagert und das Fixturebundle unter `/opt/deadlock-brain/fifo-audit/@FIFO_HEAD@` nur lesbar eingebunden. Der Host-Releasepfad und seine Dateien bleiben unverändert. Die zwei logischen direkten Hardlinkaufrufe verwenden dieselbe installierte Rootregel wie das spätere Release. Binaryhash und Inode werden vor der Probe für beide Fixturehardlinks dokumentiert.

```bash
timeout --kill-after=1s 5s /usr/bin/bwrap --die-with-parent --unshare-net \
  --ro-bind / / --proc /proc --dev /dev --tmpfs /opt/deadlock-brain \
  --ro-bind @AUDIT_DIR@/bundle /opt/deadlock-brain/fifo-audit/@FIFO_HEAD@ \
  /opt/deadlock-brain/fifo-audit/@FIFO_HEAD@/legacy/sheet-sync/deadlock-brain ai-model
```

Der gleiche Aufruf wird für `build-data` vorbereitet. Gültige TOML muss den jeweiligen bisherigen absoluten Datenpfad zeigen. Danach fehlen die Fixture-TOMLs oder werden FIFO, Symlink, unlesbare Datei, ungültige TOML und übergroße Datei. Erwartet wird jeweils ein früher Fehler; Exit 124 oder 137 ist kein bestandener Nachweis. Die zeitlich begrenzten Aufrufe und ihre Namespacefähigkeit müssen vor Ausführung bestätigt werden.

Die Credentialprobe muss den tatsächlichen Loader erreichen. Dazu ist `entities --query synthetic-fixture` geeignet: Nach normaler Settingsprüfung öffnet dieser bestehende Lesepfad direkt den read-only Pool, ohne Datenverzeichnisse anzulegen oder vorher einen Katalog aus dem Internet zu laden. `population stats` wäre ungeeignet, weil es vor der Datenbank einen externen Katalog lädt. `ai-model` allein prüft keine Credentials.

Eine transiente systemd-User-Unit erhält ausschließlich eine bekannte nicht geheime reguläre Fixturedatei über `LoadCredential=infisical-token`. systemd liefert seine echte `CREDENTIALS_DIRECTORY`-Metadatenbindung. Der `%d`-Specifier bezeichnet laut installierter systemd.unit-Dokumentation dieses Verzeichnis; im bwrap-Namespace wird dort ausschließlich das jeweilige künstliche Credential-Fallverzeichnis überlagert. Kein eigener ENV-Konfigpfad wird gesetzt. Der genaue Aufruf ist vor Ausführung durch die Orchestrierung zu bestätigen:

```bash
systemd-run --user --wait --pipe --collect \
  --unit=brain-abo-fifo-audit-@HEAD_KURZ@-sheet-@FALL@ \
  --property=Type=oneshot --property=TimeoutStartSec=5s \
  --property=LoadCredential=infisical-token:@AUDIT_DIR@/manager-fixture \
  /usr/bin/bwrap --die-with-parent --unshare-net --ro-bind / / \
  --proc /proc --dev /dev --tmpfs /opt/deadlock-brain \
  --ro-bind @AUDIT_DIR@/bundle /opt/deadlock-brain/fifo-audit/@FIFO_HEAD@ \
  --ro-bind @AUDIT_DIR@/credential-cases/@FALL@ %d \
  /opt/deadlock-brain/fifo-audit/@FIFO_HEAD@/legacy/sheet-sync/deadlock-brain \
  entities --query synthetic-fixture
```

Für beide Jobhardlinks werden fehlende, writerlose FIFO-, Symlink-, unlesbare und ungültige künstliche Credentialdateien vorbereitet. Der reguläre `manager-fixture`-Quellpfad ist nie eine FIFO. Eine Namespace- oder Managerfehlermeldung ist kein bestandener Loadernachweis. Es werden ausschließlich Exitstatus, Laufzeit, tatsächliche Root-/Datenpfadfelder und redigierte Fehlerklassen gespeichert. Header, Tokenwerte, private Inhalte und Credentialdateiinhalte werden weder gelesen als Agentenausgabe noch kopiert oder protokolliert.

Nach abgestimmter Suite, Gate und begrenzten Laufzeitproben endet der Slot mit vollständigem Prozess-/Unitabbau und freien Lockgegenproben. Die noch offene tatsächliche `tools=[]`- und gemeinsame Abo-/Releaseabnahme bleibt davon getrennt.
