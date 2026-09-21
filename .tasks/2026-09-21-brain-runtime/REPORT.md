# Rust-Runtime als getrenntes Freigabepaket

Basis: `9b76ecd1ceab861999615d17873dc8673311cdf2`.

Übernommen aus dem vollständigen Integrationsstand `22c8ebe07fe90963e063508551b417b71bd50dc2`: native Konfigurationsauflösung, Postgres-Pools einschließlich technisch erzwungener lesender Verbindungen, Systemd-Credential-Auswahl und das zusätzliche Binary `deadlock-brain-secret-exec`.

Das Paket ändert keine Reasoner-Auswahl, keine Qualitätsgrenze, keine Patch-Migration und keine produktive Unit. Die Python-Löschung und die Umstellung der Aufrufer sind eigene, noch ausstehende Integrationsschritte. Dieser Zwischenstand ist nicht der vollständige Rust-Cutover.

Prüfung in diesem Worktree: **418 Rust-Tests bestanden, 0 fehlgeschlagen, 59 ignoriert**, kompletter Workspace mit allen Targets; Clippy ebenfalls Exit 0. Befehle: `cargo test --locked --workspace --all-targets -j 2` und `cargo clippy --locked --workspace --all-targets -j 2`, jeweils mit eigenem Target-Verzeichnis. Ausgaben und Abschlusscodes: `/home/nathanael/.local/share/deadlock-brain/releases/20260921-final/runtime-stage/`.

Die drei Credential-Regressionen prüfen die benannte Systemd-Bindung vor einem fremden offenen Deskriptor, den Schutz eines verdrängten FDs gegen Vererbung und die weiterhin unterstützte explizite FD-Bindung. Der vorgelagerte native Funktionsnachweis im Gesamtstand ist im Abschlussauftrag dokumentiert; er ersetzt keinen späteren Release-Nachweis dieses Pakets.

Merge-Freigabe und Main-Push werden erst anhand des regulären Gates protokolliert. Produktionsinstallation ist noch nicht erfolgt.
