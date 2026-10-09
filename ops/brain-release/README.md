# Gemeinsame Brain-Binaryinstallation

Der Rust-Helfer baut ein gemeinsames Release und prüft dessen Herkunft ohne weiteren Cargo-Bau. Er ändert keine Units, Laufzeitkonfiguration, Datenbanken oder Wiki-Dateien und startet keinen Dienst.

## Aufrufe

```text
brain-release plan /home/nathanael/.worktrees/<saubere-Quelle>
brain-release build /home/nathanael/.worktrees/<saubere-Quelle> /home/nathanael/.local/state/<neues-privates-Bundle>
brain-release verify /home/nathanael/.worktrees/<saubere-Quelle> /home/nathanael/.local/state/<Bundle>
sudo -n -- /usr/local/libexec/brain-release install /home/nathanael/.worktrees/<saubere-Quelle> /home/nathanael/.local/state/<Bundle>
sudo -n -- /usr/local/libexec/brain-release rollback <vollständiger-SHA>
sudo -n -- /usr/local/libexec/brain-release recover
```

`build` ruft den fest installierten Root-Helfer über `sudo -n` auf. Dieser hält die vorhandene Deploy-Sperre, startet denselben vertrauenswürdigen Helfer mit entfernten Zusatzgruppen und UID/GID 1000 für den Cargo-Bau und übernimmt dessen Manifest. Compiler-Ausgaben gehen auf stderr, das Manifest auf stdout. Der Root-Prozess führt keinen Compiler und kein Git aus.

Der geprüfte Build wird anschließend in die beiden root-eigenen Release-Layouts kopiert. Dabei werden die kopierten Bytes gegen die vom Buildkind gelieferten Hashes geprüft. Die Releasezeiger ändern sich bei `build` nicht. Das bisher erst bei `install` erfolgte Staging dient jetzt bereits als dauerhafter Buildbeleg. Es gibt weder einen zusätzlichen Signierschlüssel noch einen vom Betreiber frei beschreibbaren Herkunftsbeleg.

`verify` prüft Quelle, Remote-main, Inventar und Bundlehashes und vergleicht das vollständige Manifest mit den vorhandenen root-eigenen, erneut hashgeprüften Layouts. Nach einem Abbruch genügt ein bereits vollständig veröffentlichtes Layout als Buildbeleg; vor Aktivierung ergänzt `install` das fehlende zweite Layout. Ein verändertes Bundle mit passend verändertem Manifest genügt nicht. `install` wiederholt die Herkunftsprüfung vor dem Staging und unmittelbar vor dem Zeigerwechsel, ohne Cargo zu starten. Ein zweites `build` desselben bestätigten Commits kopiert die bestätigten Binaries in ein neues Bundle, ebenfalls ohne Cargo. Auch ein anderer sauberer Worktree ist zulässig, wenn Commit, Git-Baum und vollständiger Quellfingerprint identisch sind. Seine Identität wird während jeder Quellprüfung auf Stabilität geprüft; die Identität der ursprünglichen Buildquelle bleibt unverändert im root-eigenen Manifest dokumentiert. Bei einem anderen Quellbaum, abweichenden oder beschädigten bestätigten Layouts wird gestoppt.

## Quelle, Cache und Zwischenartefakte

Die Quelle muss ein vollständig sauberer eigener Worktree unter `/home/nathanael/.worktrees/` auf dem aktuellen Remote-main sein. Auch ignorierte Dateien, Symlinks und fremd beschreibbare Pfade werden abgewiesen. Gruppen-Schreibrechte sind nur für die nach Accountprüfung private Betreibergruppe 1000 erlaubt. Der feste Ursprung ist `git@github.com:EarlySalty/Deadlock-Brain.git`. Verfolgte Dateien werden einschließlich Dateimodus gegen die Git-Blobs geprüft; zusätzlich wird der vollständige Quellbaum mit SHA-256 erfasst. Worktree-Inode und Git-Zeiger werden ebenfalls geprüft. `assume-unchanged` ersetzt die Byteprüfung nicht.

Der Build belegt einen der drei Host-Build-Slots und danach `/tmp/deadlock-cargo-release.lock`. Die feste Build-Ablage wird unter dieser Sperre vor dem Bau entfernt, frisch angelegt und nach Übernahme der Binaries wieder entfernt, auch bei einem Buildfehler. Es gibt keine Wiederverwendung eines Cargo-Targets von einem anderen Commit oder Worktree. Nach einem harten Prozessabbruch räumt der nächste gesperrte Lauf die Ablage vor dem Bau auf.

```text
CARGO_INCREMENTAL=0
RUSTFLAGS=--remap-path-prefix=/home/nathanael=/brain-operator
RUSTC_WRAPPER=/home/nathanael/.cargo/bin/sccache
SCCACHE_DIR=/home/nathanael/.cache/sccache
cargo build --locked --release --jobs 4 --workspace --bins --target-dir /home/nathanael/.cache/brain-release-target --config 'build.build-dir="/home/nathanael/.cache/brain-release-target"'
```

Die Compiler-Flags und der Targetpfad sind laufunabhängig. Der vorhandene gemeinsame sccache bleibt die einzige Compiler-Cache-Ablage. Ein globales `CARGO_TARGET_DIR` ist nicht erlaubt. Die Kinder erhalten eine bereinigte Umgebung ohne geerbte Providerzugänge. Prüfungen der Worker verwenden dasselbe Release-Profil und dieselben Flags, beispielsweise `cargo-slot test --locked --release` oder `cargo-slot clippy --locked --release --all-targets` mit den oben genannten Umgebungswerten. Testprogramme und Clippy-Metadaten sind weiterhin eigene Cargo-Targets, kein Nachweis eines Produktionsbinaries.

Das Inventar kommt aus `cargo metadata --format-version 1 --no-deps`. Sämtliche Workspace-Binaries werden gemeinsam gebaut. Pflichtbinaries sind `deadlock-brain`, `brain-serve`, `brain-maintain`, `brain-legacy-import`, `deadlock-brain-yt`, `brain-candidate-activate` und `brain-patchnotes-ingest`. Cargoartefakte dürfen den belegten zweiten Hardlink in `release/deps/<Binary>-<Hash>` haben; weitere Hardlinks werden abgewiesen. Kopierte Bundledateien müssen einlinkig sein.

Manifestformat 3 enthält Source-SHA, Git-Baum, Quellfingerprint, Quellpfad und Worktreeidentität, Werkzeugversionen, Buildargumente und Artefakthashes. Das root-eigene Staging belegt den tatsächlichen unprivilegierten Build statt eines späteren unabhängigen Neubaus. Die Betreiber-Toolchain und der fest installierte Helfer bleiben die Vertrauensbasis. Alte Bundles im Format 2 werden von `verify` nicht bestätigt; vorhandene root-eigene Releases im Format 2 bleiben als geprüfte Rollbackziele nutzbar.

## Installation und Rückweg

Der privilegierte Teil läuft aus dem root-eigenen `/usr/local/libexec/brain-release`, ohne setuid/setgid. Eine eng begrenzte sudo-Freigabe muss `build`, `install`, `rollback` und `recover` erlauben. Der Helfer akzeptiert keinen Zielpfad und keinen beliebigen Kindbefehl. `compile` ist der unprivilegierte interne Buildaufruf; allein erzeugt er keinen root-eigenen Buildbeleg.

`/opt/deadlock-brain/.deploy.lock` schützt Buildbestätigung, Staging, Installation und beide Zeigerwechsel. Andere Deploywege müssen dieselbe Sperrdatei verwenden. Vorhandene Releases werden vollständig geprüft, nicht überschrieben. Der Helfer erhält diese Layouts:

```text
/opt/deadlock-brain/releases/<SHA>/bin/<Binary>
/opt/deadlock-brain/releases/<SHA>/manifest.json
/opt/deadlock-brain/maintenance-releases/<SHA>/<Binary>
/opt/deadlock-brain/maintenance-releases/<SHA>/manifest.json
```

Reguläre Dateien werden mit `O_NOFOLLOW` geöffnet; Besitzer, Dateitypen, Schreibrechte, Hardlinks und Pfadbestandteile werden geprüft. Root-Staging kopiert aus offenen Dateideskriptoren und prüft die kopierten Bytes. Beide vollständigen Layouts müssen dasselbe Manifest und dieselben Binaryhashes haben. Ein teilweise vorhandenes Layout kann bei `install` ergänzt werden, wenn bereits ein vollständiger Buildbeleg vorliegt; ein unvollständiger erster Buildbeleg stoppt die Bestätigung.

Die beiden Releasezeiger wechseln einzeln atomar. Vorher wird `.deploy-pending.json.tmp` vollständig geschrieben und synchronisiert, danach als `.deploy-pending.json` veröffentlicht. Schlägt ein Zeigerwechsel fehl, stellt der Helfer beide alten Zeiger wieder her. Scheitert der Rückweg, bleibt das Journal erhalten und weitere Installationen stoppen bis zu `recover`. Ohne veröffentlichtes Journal entfernt `recover` ein geprüftes Zwischenjournal. Vorhandene Releaseverzeichnisse und Rückfallstände bleiben erhalten.

Der Helfer entfernt nur seine Build-Zwischenartefakte und fehlgeschlagene eigene Bundles. Historische Bundles werden nicht anhand ihres Alters gelöscht: Laufende und mögliche Rückfallstände bleiben erhalten. Worktree- und allgemeines Cargo-Cache-Aufräumen liegen außerhalb dieses Helfers. Ein neues Helferbinary darf erst installiert werden, wenn `pgrep -x brain-release` keinen Prozess findet.

Die Tests prüfen echte Dateien, Symlinks, Hardlinks, Hashes, Prozessabbrüche und flock sowie einen kleinen echten Cargo-Releasebau. Die Herkunftsprüfung wird gegen veränderte Bundlebytes und passend gefälschte Hashes geprüft. Der vollständige Brain-Release und die laufenden Dienste müssen nach dem Merge separat geprüft werden.
