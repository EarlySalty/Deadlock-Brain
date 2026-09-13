# Brain-Release 13.09.2026

Produktiver Quellstand: main und origin/main `2c28fda2204483d57bf788e6938050edb236156f`.
Technische unabhängige Freigabe auf Code `9bcc588`, reguläres Astra-Gate auf
2c28fda mit Exit 0/ALLOW, danach regulärer Merge- und Main-Push-Gate.

Ein erster Merge-Aufruf wurde technisch vor Ausführung blockiert, weil der
Hook den Branch im falschen Arbeitsverzeichnis aufzulösen versuchte. Der
reguläre Aufruf mit explizitem `git -C` wurde anschließend freigegeben; kein
Gatezustand wurde geändert und kein Blocker umgangen.

Release aus sauberem aktuellen main mit absolutem Cargo-Pfad, ohne zusätzliche
ENV-Konfiguration. Vor Start kein anderer Host-Releaseprozess. Build erfolgreich
in 1m 11s. Binary: `/home/nathanael/repos/Deadlock-Brain/rust/target/release/deadlock-brain`.

- Vorher SHA256: `b93072bdd7183931c8fb13477e39ce25fec850fe5e021431c162a852aaa700cb`.
- Nachher SHA256: `73495b6d4ac208653f00bee38dc5ae5145548dd0d7b486212b86cc9f0de640e4`.
- Rollbackkopie: `/home/nathanael/.local/share/deadlock-brain/releases/b93072bdd7183931/deadlock-brain`.
- Neuer Release akzeptiert `reason --help`; Ausgabe neben diesem Bericht.

Der tatsächliche Rust-Verbraucher ist ein Timer-Oneshot, kein dauerhaft
laufender Brain-Rust-Daemon. `deadlock-brain-build-data.service` war vorher
inactive, MainPID 0, letzter Exit 0. Regulärer Unitstart am 13.09.2026 um
17:19:27 CEST, Invocation `68bd9507031b43c6ac6e7a5518ab5ead`. Der Kindprozess
1456309 führte den neuen Releasepfad aus; SHA256 über `/proc/1456309/exe`
entspricht exakt dem neuen Binary. Startstatus und Prozesshash sind separat
gesichert. Regulärer Abschluss um 17:29:09 CEST nach 9m 42s:
Result=success, ExecMainStatus=0, MainPID=0, ActiveState=inactive.
Die Zusammenfassung dieser Invocation belegt 38 Helden, 6328 Itemstatistiken,
608 Lift-Zeilen, 38 Fähigkeitsfolgen und 114000 Synergiezeilen. Der Timer
bleibt aktiv; nächster regulärer Lauf am 14.09.2026 um 03:30 CEST.

Der normale vorhandene Unitpfad wurde gestartet; Secrets wurden weder
ausgegeben noch manuell in ENV gesetzt. Bestehende Legacy-Secretwrapper wurden
nicht als neue Messstrecke wiederverwendet. Die echte Reasoner-/Ask-/Publish-
Messung verwendet die Crates desselben Quellstands über separat gebaute
Read-only-Examples mit Infisical-FD. Der CLI-Oneshot belegt den produktiven
Releaseverbrauch; er ist kein behaupteter Steam-Upload oder Qualitätsnachweis.

Die separate Python-Corpus-Site und die schon vorher fehlgeschlagene
YouTube-Lernunit wurden nicht verändert. Das fachliche Meta-Qualitätsziel
bleibt offen, siehe ursprüngliche AUFTRAG.md und ABNAHME-FINAL.md.

Nach dem produktiven Sync wurde der Read-only-Ask-/Payload-Smoke erneut
erfolgreich ausgeführt (`ask-publish-postdeploy.json`, SHA256
`f674c70b6b2261d2848e71b78db0fd1df1cb3b481cbf166d911ed303a6f8d99a`).
Das vollständige BuildObject entspricht der direkten Reasoner-Ausgabe;
16 Käufe, unveränderte Kern-IDs gegenüber der Abnahme, weiterhin 2/9
Referenzwaffen, 3 Imbues, 5 Verkaufsprioritäten, 14.030 Promptzeichen.
Read-only=on, upload_performed=false. Der Datensync ändert das negative
fachliche Urteil nicht. Kategorieabmessungen sind Payload-Prüfung, kein
Nachweis der Darstellung im Spielclient.

F-/G-Worktrees und lokale Branches sind nach SHA-Backup und nachgewiesener
Integration entfernt. Der abschließende Doku-Commit versioniert diesen
Bericht und die unabhängige Abnahme; er verändert keine Rust-Quellen und
erfordert keinen zweiten Releasebau. Der verbleibende Abschlussworktree wird
nach dessen regulärem Merge ebenfalls entfernt.
