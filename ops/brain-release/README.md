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

`plan` liest den tatsächlichen Remote-main und die beiden vorhandenen Zeiger, danach prüft es die Quelle. Ein verschmutzter Arbeitsstand führt auch im Plan zum Exit 1. `verify` prüft ein bestehendes Bundle und gibt ausschließlich sein Buildmanifest aus. Fehlende Herkunft, fehlende Pflichtbinaries, veränderte Artefakte, vertauschte Quellen und ein inzwischen veränderter Remote-main stoppen den Vorgang.

Quelle und Build laufen ausschließlich mit Betreiber-UID/GID 1000. Zulässig sind getrennte, vollständig saubere Worktrees unter `/home/nathanael/.worktrees/`. Auch ignorierte Dateien, Symlinks und fremd beschreibbare Pfade werden abgewiesen. Gruppen-Schreibrechte sind nur für die private Betreibergruppe 1000 erlaubt: Die frische Accountprüfung muss bestätigen, dass weder primäre noch zusätzliche Mitglieder eine andere UID haben. Bestehende Userverzeichnisse und Host-Sperrdateien werden dafür nicht umgestellt. Der feste Ursprung lautet `git@github.com:EarlySalty/Deadlock-Brain.git`. Alle verfolgten Quelldateien werden gegen die Git-Blobs geprüft, einschließlich Dateimodus; zusätzlich entsteht ein SHA-256-Fingerprint des vollständigen Quellbaums. `assume-unchanged` genügt deshalb nicht, um geänderte Bytes zu übersehen.

`build` erzeugt ein neues privates Bundle außerhalb der Quelle. Der tatsächliche Aufruf lautet:

```text
/home/nathanael/.cargo/bin/cargo build --locked --release --jobs 2 --workspace --bins --target-dir <Bundle>/target
```

Das Binaryinventar kommt aus `cargo metadata --format-version 1 --no-deps --manifest-path <Quelle>/rust/Cargo.toml`. Sämtliche Workspace-Binaries werden gemeinsam gebaut und gehasht. Pflicht sind `deadlock-brain`, `brain-serve`, `brain-maintain`, `brain-legacy-import`, `deadlock-brain-yt`, `brain-source-sync`, `brain-candidate-activate` und `brain-patchnotes-ingest`. Weitere integrierte P/S-Writer werden durch das Workspace-Inventar eingeschlossen. Publisher aus anderen Repositories gehören nicht zu diesem Brain-SHA.

Der Build hält zuerst `host-checks.lock`, dann `/tmp/deadlock-cargo-release.lock`, prüft frisch auf nicht beendete Compiler, RAM und Speicher und verwendet höchstens zwei Jobs. Fremde Compiler werden nicht beendet. Bei ihnen bleibt die Sperre gehalten und die Probe wird nach 30 Sekunden wiederholt. Es gibt kein Zeitlimit für das Sperrwarten und kein globales `CARGO_TARGET_DIR`. Die Kinder erhalten eine bereinigte Umgebung, keine geerbten Providerzugänge. Das Manifest enthält Source-SHA, Git-Baum, Quellfingerprint, Quellpfad, Werkzeugversionen, Buildargumente und alle Artefakthashes. Ein Datei-Alter oder SHA-Verzeichnisname ersetzt diese Nachweise nicht.

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

Alte Releases bleiben erhalten. Reguläre Dateien werden mit `O_NOFOLLOW` geöffnet; Besitzer, Dateitypen, Schreibrechte, Hardlinks und Pfadbestandteile werden geprüft. Der Installer kopiert aus offenen Dateideskriptoren in root-eigenes Staging und prüft die resultierenden Hashes. Bereits vorhandene Releaseverzeichnisse werden ausschließlich nach vollständiger Inhaltsprüfung wiederverwendet, niemals überschrieben. Ein einzelnes vorhandenes Layout stoppt die Installation.

`current` und `maintenance-current` wechseln jeweils über einen atomaren Symlink-Austausch. Zwei unterschiedliche Pfade lassen sich nicht in einem einzelnen Dateisystemaufruf gemeinsam austauschen. Das vorab synchronisierte `.deploy-pending.json` macht einen Abbruch sichtbar. Ein fehlgeschlagener zweiter Wechsel wird als Fehler gemeldet; der Rückweg stellt den Ausgangsstand wieder her. Scheitert auch dieser, bleibt das Journal bestehen und jede weitere Installation stoppt bis zu `recover`. Ein Fehler wird niemals als erfolgreiches gemeinsames Release gemeldet. Während eines G5-Schreibwechsels müssen die bestehenden Betriebsverträge die Writer pausieren und erst nach dem vollständig geprüften gemeinsamen Zeigerstand wieder starten; dieser Helfer führt keine Dienststeuerung aus.

`rollback` aktiviert ausschließlich einen bereits vollständig vorhandenen, hashgeprüften gemeinsamen Release-SHA. Historische Releases ohne Buildmanifest werden erhalten, aber nicht als nachgewiesenes Rollbackziel akzeptiert. `recover` nimmt einen unterbrochenen Zeigerwechsel anhand des root-eigenen Journals zurück und stoppt bei fremd veränderten Zeigern.

## Grenze dieses Arbeitsstands

Der Root-Helfer und seine eng begrenzte sudo-Freigabe müssen außerhalb dieses Workers nach Abnahme eingerichtet werden. Es gibt keine allgemeine sudo- oder Shell-Brücke und keine automatische Installation des Helfers. Der nächste Betriebsschritt ist die geprüfte Binaryinstallation nach finalem Merge, anschließend `plan` und `build` aus einem neuen sauberen Quellworktree des aktuellen Remote-main. Erst der gemeinsame G5-Ablauf führt Neustarts und Live-Abnahme aus.

Die isolierten Installationstests benutzen echte Dateien, Verzeichnisse, Symlinks, Hashes und flock mit einem unabhängigen Fremdprozess unter einem privaten Testverzeichnis. Ihre kleinen ELF-Platzhalter belegen die Installationsmechanik, weder die Herkunft echter Cargo-Releaseartefakte noch die Funktion laufender Brain-Dienste. In diesem Worker erfolgt kein Produktionswechsel und kein Releasebau.
