# Dauerhafte Verdrahtung des vorhandenen Importtimers

Der fremde brain-live-main-Arbeitsbaum wurde nicht verändert. Bestehende Userunit /home/nathanael/.config/systemd/user/deadlock-brain-build-data.service verwendet jetzt das dauerhafte Runtimeverzeichnis /home/nathanael/.local/share/deadlock-brain/build-data-runtime-b7289d11 statt eines später löschbaren Worktrees.

Die beiden vorhandenen administrativen Skripte wurden unverändert aus dem tatsächlichen Main-SHA b7289d115c0b64016fcfe6cbfe3c797cf9fa76e2 installiert, keine neue Pipeline oder Pythonfunktion:

- run_build_data_with_infisical.sh, SHA256 b04d31e766f9e5efa514efb3d07a3f921226fa55d7fa06646a1c6ac929dff4da.
- export_infisical_env.py, SHA256 5d2422c1336615f17f7bc691d9de92e685d46d6ba9d7fac896ed3566a57e5a78.

Beide tatsächlichen Bytes jeweils gegen den Git-Blob b7289d11 geprüft: gleich. ROOT_DIR wird durch den vorhandenen Skriptdefault aus dem eigenen Skriptpfad ermittelt. Der vorhandene BRAIN_BIN-Default rust/target/release/deadlock-brain wird über den neuen Runtime-Symlink rust/target/release -> /opt/deadlock-brain/current/bin an den regulären Installationszeiger gebunden. Dieser Symlink ist kein zusätzlicher Binärbuild oder Datenpfad. Der vorhandene vollständige assets-Aufruf läuft vor build-data --hero all; keine lokalen population-/Matchimporte.

R12 verweigerte den ersten Unittext wegen Environment-Einträgen. Diesen tatsächlichen Formfehler regulär behoben: sämtliche Environment-Einträge der Unit entfernt, keine alternative Schreibmethode oder Guardrailänderung. Die unveränderten vorhandenen Skripte verwenden ihre bestehenden Defaults; LoadCredential=infisical-token bleibt. Secrets werden zur Laufzeit im vorhandenen Infisical-Administrationsweg geladen, nicht als Unit-EnvironmentFile angelegt. Keine Secretwerte ausgegeben oder gespeichert.

systemd-analyze --user verify auf der tatsächlichen Unit: Exit 0. systemctl --user daemon-reload: Exit 0. Noch kein Start oder Erfolg des neuen Importlaufs aus diesen Konfigurationsprüfungen abgeleitet. Erst nach regulärem Releaseinstall mit gesichertem Main-SHA starten und tatsächlichen Run-/Receipt-/Originalhashbeweis führen. CLI pull assets --help am gebauten b7289d11 bestätigt den vollständigen Default und dauerhaften Standarddatenpfad /home/nathanael/.local/share/deadlock-brain.
