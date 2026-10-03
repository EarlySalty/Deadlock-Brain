# Gemeinsame Brain-Binaryinstallation

Dieser eigenständige Rust-Helfer ergänzt die vorhandenen Releaseverzeichnisse. Er ändert weder Units, Laufzeitkonfiguration, Datenbanken noch Wiki-Installation und startet keinen Dienst. G5 bleibt bis zur gemeinsamen Integration und Betriebsabnahme unverändert.

## Aufrufe

```text
brain-release plan /home/nathanael/.worktrees/<eigener-sauberer-Quellworktree>
brain-release build /home/nathanael/.worktrees/<eigener-sauberer-Quellworktree> /home/nathanael/.local/state/<neues-privates-Bundle>
brain-release verify /home/nathanael/.worktrees/<eigener-sauberer-Quellworktree> /home/nathanael/.local/state/<Bundle>
sudo -- /usr/local/libexec/brain-release install /home/nathanael/.worktrees/<eigener-sauberer-Quellworktree> /home/nathanael/.local/state/<Bundle>
sudo -- /usr/local/libexec/brain-release rollback <vollständiger-SHA>
sudo -- /usr/local/libexec/brain-release recover
```

`plan` liest den tatsächlichen Remote-main und die beiden vorhandenen Zeiger, danach prüft es die Quelle. Ein verschmutzter Arbeitsstand führt auch im Plan zum Exit 1. `verify` baut den geprüften Quellstand in einem neuen privaten Target vollständig neu und vergleicht die tatsächlich erzeugten Binaryhashes mit dem Bundle. Seine Standardausgabe enthält das geprüfte Manifest; Compiler- und Ressourcenmeldungen gehen auf die Fehlerausgabe. Ein frei verändertes Bundle mit passend geändertem Manifest genügt damit nicht als Herkunftsnachweis. Fehlende Pflichtbinaries, vertauschte Quellen und ein inzwischen veränderter Remote-main stoppen den Vorgang.

Quelle und Build laufen ausschließlich mit Betreiber-UID/GID 1000. Zulässig sind getrennte, vollständig saubere Worktrees unter `/home/nathanael/.worktrees/`. Auch ignorierte Dateien, Symlinks und fremd beschreibbare Pfade werden abgewiesen. Gruppen-Schreibrechte sind nur für die private Betreibergruppe 1000 erlaubt: Die frische Accountprüfung muss bestätigen, dass weder primäre noch zusätzliche Mitglieder eine andere UID haben. Bestehende Userverzeichnisse und Host-Sperrdateien werden dafür nicht umgestellt. Der feste Ursprung lautet `git@github.com:EarlySalty/Deadlock-Brain.git`. Alle verfolgten Quelldateien werden gegen die Git-Blobs geprüft, einschließlich Dateimodus; zusätzlich entsteht ein SHA-256-Fingerprint des vollständigen Quellbaums. `assume-unchanged` genügt deshalb nicht, um geänderte Bytes zu übersehen.

`build` erzeugt ein neues privates Bundle außerhalb der Quelle. Der tatsächliche Aufruf lautet:

```text
/home/nathanael/.cargo/bin/cargo build --locked --release --jobs 2 --workspace --bins --target-dir <Bundle>/target
```

Das Binaryinventar kommt aus `cargo metadata --format-version 1 --no-deps --manifest-path <Quelle>/rust/Cargo.toml`. Sämtliche Workspace-Binaries werden gemeinsam gebaut und gehasht. Pflicht sind `deadlock-brain`, `brain-serve`, `brain-maintain`, `brain-legacy-import`, `deadlock-brain-yt`, `brain-source-sync`, `brain-candidate-activate` und `brain-patchnotes-ingest`. Weitere integrierte P/S-Writer werden durch das Workspace-Inventar eingeschlossen. Publisher aus anderen Repositories gehören nicht zu diesem Brain-SHA.

Der Build hält zuerst `host-checks.lock`, dann `/tmp/deadlock-cargo-release.lock`, prüft frisch auf nicht beendete Compiler einschließlich Rustfmt, RAM und Speicher und verwendet höchstens zwei Jobs. Fremde Compiler werden nicht beendet. Bei ihnen bleibt die Sperre gehalten und die Probe wird nach 30 Sekunden wiederholt. Es gibt kein Zeitlimit für das Sperrwarten und kein globales `CARGO_TARGET_DIR`. Die Kinder erhalten eine bereinigte Umgebung, keine geerbten Providerzugänge. `CARGO_INCREMENTAL=0` und eine Pfadumsetzung des frischen Targets verhindern die Wiederverwendung fremder Buildausgaben und reduzieren Unterschiede zwischen den frischen Targets. Nicht reproduzierbare Binarybytes führen beim Vergleich zum Fehler, nicht zur Installation.

Das Manifestformat 2 enthält Source-SHA, Git-Baum, Quellfingerprint, Quellpfad und Worktreeidentität, Werkzeugversionen, Buildargumente und alle Artefakthashes. Diese Angaben werden beim Verifizieren durch einen tatsächlichen neuen Build bestätigt. Die bestehende Betreiber-Toolchain und der fest installierte Helfer bleiben die Vertrauensbasis; es gibt keinen neuen Signierschlüssel oder frei importierbaren Buildbeleg. Ein Datei-Alter oder SHA-Verzeichnisname ersetzt den Build nicht. Cargoartefakte dürfen genau den belegten zweiten Hardlink in `release/deps/<Binary>-<Hash>` besitzen; zusätzliche oder außerhalb dieses Layouts liegende Hardlinks werden abgewiesen. Die kopierten Bundledateien müssen weiterhin einlinkig sein.

## Installation und Rückweg

Der privilegierte Teil läuft ausschließlich als separat eingerichtetes, root-eigenes `/usr/local/libexec/brain-release`, ohne setuid/setgid. Er akzeptiert kein Zielverzeichnis, keine Befehle und keine Umgebung vom Aufrufer. Git und Cargo laufen auch dort nie als root: derselbe vertrauenswürdig installierte Helfer führt die Herkunftsprüfung nach Entfernen der Zusatzgruppen und dauerhaftem Wechsel auf UID/GID 1000 aus. Vor dem Zeigerwechsel wird diese Prüfung erneut durchgeführt.

Eine gemeinsame Sperre `/opt/deadlock-brain/.deploy.lock` schützt Prüfung, Staging, Installation, Rückweg und beide Zeigerwechsel. Andere Binary-Deploywege müssen diese identische Sperrdatei verwenden. Ein fremder Prozess, der dieselbe Sperre nimmt, wird tatsächlich ausgeschlossen; ein bisheriger manueller Installer ohne diese Sperre ist kein sicherer paralleler Weg.

Jedes vollständige Release besitzt identische Manifeste und Binarybytes in beiden bestehenden Layouts:

```text
/opt/deadlock-brain/releases/<SHA>/bin/<Binary>
/opt/deadlock-brain/releases/<SHA>/manifest.json
/opt/deadlock-brain/maintenance-releases/<SHA>/<Binary>
/opt/deadlock-brain/maintenance-releases/<SHA>/manifest.json
```

Alte Releases bleiben erhalten. Reguläre Dateien werden mit `O_NOFOLLOW` geöffnet; Besitzer, Dateitypen, Schreibrechte, Hardlinks und Pfadbestandteile werden geprüft. Der Installer kopiert aus offenen Dateideskriptoren in root-eigenes Staging und prüft die resultierenden Hashes gegen den frisch kompilierten Stand. Bereits vorhandene Releaseverzeichnisse werden nach vollständiger Inhaltsprüfung wiederverwendet, nicht überschrieben. Fehlt nach einem Abbruch eines der beiden Layouts, ergänzt ein erneutes `install` das fehlende Layout unter derselben Sperre und erhält das bereits veröffentlichte Layout. Ein abweichendes vorhandenes Layout stoppt den Retry. Die Zeiger ändern sich erst nach vollständiger Prüfung beider Layouts.

`current` und `maintenance-current` wechseln jeweils über einen atomaren Symlink-Austausch. Zwei unterschiedliche Pfade lassen sich nicht in einem einzelnen Dateisystemaufruf gemeinsam austauschen. Das Journal wird als `.deploy-pending.json.tmp` vollständig geschrieben und synchronisiert, dann atomar als `.deploy-pending.json` veröffentlicht und das Elternverzeichnis synchronisiert. Erst danach beginnt der Zeigerwechsel. Ein Abbruch während des Schreibens hinterlässt damit höchstens ein unvollständiges Zwischenjournal ohne Zeigerwechsel; Retry und `recover` entfernen dieses nach Dateisicherheitsprüfung. Ein veröffentlichtes Journal macht einen begonnenen Wechsel sichtbar. Ein fehlgeschlagener zweiter Wechsel wird als Fehler gemeldet; der Rückweg stellt den Ausgangsstand wieder her. Scheitert auch dieser, bleibt das Journal bestehen und jede weitere Installation stoppt bis zu `recover`. Ein Fehler wird nicht als erfolgreiches gemeinsames Release gemeldet. Während eines G5-Schreibwechsels müssen die bestehenden Betriebsverträge die Writer pausieren und erst nach dem vollständig geprüften gemeinsamen Zeigerstand wieder starten; dieser Helfer führt keine Dienststeuerung aus.

`rollback` aktiviert einen bereits vollständig vorhandenen, root-eigenen und hashgeprüften gemeinsamen Release-SHA im Manifestformat 2. Historische Releases ohne dieses neu geprüfte Buildmanifest werden erhalten, aber nicht als nachgewiesenes Rollbackziel akzeptiert. `recover` nimmt einen unterbrochenen Zeigerwechsel anhand des root-eigenen Journals zurück und stoppt bei fremd veränderten Zeigern. Ohne veröffentlichtes Journal bereinigt es ein geprüftes Zwischenjournal; fehlende Release-Layouts ergänzt anschließend ein erneutes `install`.

## Grenze dieses Arbeitsstands

Der Root-Helfer und seine eng begrenzte sudo-Freigabe müssen außerhalb dieses Workers nach Abnahme eingerichtet werden. Es gibt keine allgemeine sudo- oder Shell-Brücke und keine automatische Installation des Helfers. Der nächste Betriebsschritt ist die geprüfte Binaryinstallation nach finalem Merge, anschließend `plan` und `build` aus einem neuen sauberen Quellworktree des aktuellen Remote-main. Erst der gemeinsame G5-Ablauf führt Neustarts und Live-Abnahme aus.

Die Installationstests benutzen echte Dateien, Verzeichnisse, Symlinks, Hashes und flock mit einem unabhängigen Fremdprozess unter einem privaten Testverzeichnis. Kindprozesse werden an den Layout- und Journalveröffentlichungsgrenzen beendet; Retry, Rückweg und der Erhalt bereits veröffentlichter Releases werden anschließend an den hinterlassenen Dateien geprüft. Die kleinen ELF-Platzhalter belegen weiterhin nur die Installationsmechanik. Ein zusätzlicher echter Cargo-Releasebau eines kleinen Rust-Workspaces verwendet denselben Cargoaufruf und dieselbe Artefaktübernahme wie der Produktivpfad. Er prüft die tatsächlichen Cargo-Hardlinks, ausführbare kopierte Binaries und die Ablehnung veränderter Bundlehashes. Er ersetzt weder einen gemeinsamen Brain-Workspace-Build des endgültigen Remote-main noch die Funktion laufender Brain-Dienste. In diesem Worker erfolgt kein Produktionswechsel.
