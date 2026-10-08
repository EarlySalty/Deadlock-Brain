# V: erste Dateiübergabe und Teilbau

Freigabe vom Delegator nach ENTSCHEIDUNG-WACHE-037.md, Abschnitt 3. Auftraggeber: 481426fe-b477-42b3-91c6-901811fcba1d. Die Eigentumsinfo ist laut Auftrag an K/G zugestellt. Keine weitere Nutzerfreigabe erforderlich.

## Freigegeben

- Brain: rust/crates/brain-contracts/src/invite.rs. Vorhandenen A/E3f-Vertrag verwenden, zentralen Matcher korrigieren, direkte Modultests erhalten und ergänzen.
- Bots im eigenen Worktree: rust/bin/dl-bot/src/mcp/self_invite.rs und der enge self_invite_status-Zweig in mcp.rs, einschließlich zwingender direkter Tests in diesen beiden Dateien. Bestehenden authentifizierten Personen-/Requestpfad und lesenden Postgreszugriff verwenden.

## Nicht freigegeben

Brain lib.rs, provider_input.rs, API, discord_live.rs, hardening.rs, Cargo.toml/Cargo.lock und gemeinsame K/G-Dateien. V darf dafür genaue Integrationsdeltas beschreiben, aber keine Produktänderung vornehmen. HTTP/Client/Flight/service.rs bleiben ohne belegte Lücke unverändert. Aktuelle K-Consumer und 50-Fragen-Quote pro Berliner Tag unverändert erhalten.

## Tatsächliche Basen

Brain frisch gefetcht und eigener Planstand regulär mit geliefertem K-Main verbunden: 22561cb2fe542c98fc383b12ca576c581d31056d. Plancommit d49983d0f8906352910f69f2113622579694786e bleibt Vorfahr. Keine Änderung an fremdem WIP.

Bots nach frischem Fetch eigener Worktree /home/nathanael/.worktrees/brain-v-invite-consumer-20261008, Branch feat/brain-v-invite-consumer-20261008, Start 8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8. Dieser Stand enthält Ks private Consumerarbeit. Keine alten Botscommits blind übernehmen.

## Beweisgrenze

Ohne Export aus dem noch K-eigenen contracts/lib.rs ist invite.rs noch nicht im Brain ausführbar oder abgenommen. Kein Ersatz-Export, keine schreibende temporäre Produktverdrahtung und kein Fake-Testharness als Funktionsnachweis. Bots-Modul passend kompilieren und bestehende sichere Tests prüfen, ohne produktive Freischaltung oder reale Personendaten.

Branchbackup nach Prüfung erlaubt. Kein Mainmerge, Deploy, Restart, Cleanup oder Settle für diesen Teilstand. Stop-Hook nicht umgehen. Restübergabe folgt über den Delegator nach gesichertem K/G-Commit. Teilbau ist keine fertige Livefunktion.
