# K: tatsächlicher Deploystand nach Prozessabbrüchen

Stand: 8. Oktober 2026. Privatfix weiter vor Ortsarbeit. Beide alten Anwendungsprozesse bleiben aktiv; Nutzerprobe noch nicht bereit. Keine Schutzumgehung, Helferänderung, Cachelöschung oder WIP-Verwerfung.

## Unterbrochene Läufe und wirkliche Befunde

Die früheren Hintergrundjobs hatten keinen vollständigen Abschlussbeleg und liefen bei Wiederaufnahme nicht mehr. Brain-install-2.log meldet STOPP: Unprivilegierte Herkunftsprüfung fehlgeschlagen (Exit None). Consumer-release-build-2.log meldet No space left on device beim dl-stats-Temporärverzeichnis. Gegenwärtige Messung: 243 GB frei, Inodes zu 10 Prozent belegt. Kein dauerhaft voller Datenträger behauptet und nichts gelöscht. Bestehende Consumerartefakte regulär weitergebaut.

Installationszustand über unveränderten brain-release recover regulär zurückgenommen, Dienste nicht neu gestartet. Brain-install-3, Lauf b0777z164, tatsächlich Exit 1. Verifikationsbau erfolgreich in 5m53s, danach wörtlich:

```text
STOPP: No such file or directory (os error 2)
STOPP: Unprivilegierte Herkunftsprüfung fehlgeschlagen (Exit Some(1))
```

Der konkrete fehlende Dateipfad wird vom Helfer nicht ausgegeben. Kein erfundener Ursachenbeleg. Vor dem Verifikationsbau reguläres Warten unter vorhandener Buildsperre beobachtet; keine fremde Sperre oder Arbeit verändert. Produktionszeiger weiterhin b7289d11. Keine weitere identische Wiederholung auf dem alten Bundle.

Gelesene unveränderte Helferstellen: ops/brain-release/src/main.rs operator_verify und install; source.rs verify; build.rs compile/copy_artifacts; fs_safe.rs cargo_artifact. Der Fehler tritt nach Cargoabschluss und vor dem ersten protokollierten Artefakttransfer auf. Source-/Artefaktlookup muss einen konkreten Pfadfehler liefern, bevor eine Helferreparatur behauptet werden kann. K verändert den Helfer nicht.

## Fertiger Consumer

Lauf bx7liyz3y tatsächlich Exit 0. Wörtlicher Befehl:

```text
SQLX_OFFLINE=true /home/nathanael/.local/bin/cargo-slot +1.97.1 build --manifest-path /home/nathanael/.worktrees/bots-k-live-20261007/rust/Cargo.toml -p dl-bot --bin dl-bot --release --locked --offline --jobs 2
```

Abschließender fortgesetzter Releasebau 17m11s. Source weiterhin sauber auf aktuellem Bots-main 8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8. dl-bot 79909384 Bytes, SHA256 df58014993ca1c2a9e9dc608f4b838cc119b979377a36d3b4bf9d71cc5541065, neuer Anker x-discord-read-access vorhanden.

Dokumentierten Stage-/Releaseweg aus AN_HAUPT-B.md Punkt 5 verwendet. Genau acht Manifestprüfungen OK, sieben Begleitbinaries unverändert mit ausdrücklich gemischter Provenance erhalten. Neuer Release /opt/deadlock/bots/releases/8e1b8f03cf0d84f4754eedc8fe848ef0fceffce8 unter regulärem flock installiert, root:root, Verzeichnis und dl-bot Modus 755, kein go-w. Neuer BUILD-PROVENANCE.toml und SHA256SUMS vorhanden. current bleibt bewusst auf 0fb873c6: keine Consumerfreischaltung vor tatsächlichem Brainreadgate.

## Neuer zwingender Brain-main

Unabhängiger plan nach Installationsfehler bestätigt Remote-main 2e0de01f0da4bbb7f6630832b52b0694ea566c6f statt 400381e6. Deshalb ist das alte Bundle unabhängig vom obigen Dateifehler nicht mehr installierbarer aktueller Source.

Fresh fetch, diff und sauberer ignoriert-inclusive Status geprüft. Genau 14 Q-Collector-/Auftragsdateien hinzugekommen, keine Produktionsänderung unter rust/. Eigenen sauberen Releaseworktree regulär per --ff-only auf dieses main vorgezogen; ungeprüften Orts-WIP nicht berührt. Plan auf neuem main Exit 0. Regulärer Build bp9uxtskp für neues privates Bundle /home/nathanael/.local/state/k-private-read-release-2e0de01f0da4bbb7f6630832b52b0694ea566c6f läuft. Das ist ein aktueller-Main-Release, kein neuer Sourcebau wegen eines früheren Gate-Denys und keine Doppelinstanz. Noch kein Build-/Install-/Restartresultat behauptet.

## Zweiter tatsächlicher Current-main-Abgleich

bp9uxtskp tatsächlich Exit 1: Cargo erfolgreich in 13m15s, danach regulärer STOPP: Quelle oder Remote-main während Build verändert. Retained target enthält die 17 kompilierten Binaries; diese werden nicht als gültiges Releasebundle oder ausweichender Installationsweg verwendet.

Fresh fetch bestätigt origin/main ef2f1f95257c37d50e1ffd6d4da7c14b3202c30c. Gegen 2e0de01f genau drei Q-Status-/Berichtsdateien geändert, weiterhin keine Produktionsänderung. Eigenen sauberen Releasebaum regulär fast-forwarded; plan Exit 0. Neues erforderliches Current-main-Bundle über unveränderten brain-release build läuft als b1w3swz9t unter /home/nathanael/.local/state/k-private-read-release-ef2f1f95257c37d50e1ffd6d4da7c14b3202c30c. Keine alte Manifestumetikettierung, keine Herkunftsprüfung übersprungen und kein Haltefenster für fremde Mainlieferungen verlangt.

Der bestehende Parentthread wurde nach Statusprüfung ready über den normalen T3-Weg mit dem gesicherten Consumerrelease und der vorherigen konkreten Installationsgrenze informiert, sequence 1928804. Kein neuer Thread, Worker oder fremder Zugriff.

## Konkrete gemeinsame Layoutursache empirisch belegt

b1w3swz9t tatsächlich Exit 1: Cargo erfolgreich in 12m25s, danach STOPP: No such file or directory (os error 2). Im retained ef2f1f95-Target sind die fertig kompilierten Binaries vorhanden, beispielhaft brain-browser-consent 4227696 Bytes, 2 Hardlinks. target/release/deps und target/release/build fehlen tatsächlich.

Nur Buildlayout-Schlüssel gelesen, keine Zugangsdaten: /home/nathanael/.cargo/config.toml:11 setzt build-dir = "/home/nathanael/.cache/rust-build/{workspace-path-hash}". Bestehendes cargo metadata --no-deps bestätigt für den Releasequellworktree build_directory /home/nathanael/.cache/rust-build/4c/9a5269b3bb883d statt target/release/deps. Cargo und Rustup bleiben unverändert, Cargo 1.99.0.

Unveränderter Helfer fs_safe.rs:133-138 verlangt dagegen target/release/deps und kontrolliert dort die Cargo-Hardlinkaliases. Nach erfolgreichem Cargoabschluss bricht build.rs:54 bereits beim ersten cargo_artifact-Aufruf ab; deshalb kein Artefakttransferprotokoll. Die getrennte aktuelle Cargo-Zwischenablage und diese Lookupannahme passen nicht zusammen. Das erklärt den erneut reproduzierten Dateifehler ohne Privilegienwechsel; die zwischenzeitlichen Mainwechsel sind ein zusätzlicher unabhängiger Guardbefund.

Der Helfer erstellt Kommandos mit env_clear (source.rs:61-72); ein vorgeschobenes CARGO_BUILD_BUILD_DIR am äußeren Aufruf repariert diesen inneren Build nicht. Keine solche Scheinreparatur verwendet. Keine falschen Aliases hergestellt, keine globalen Cachepfade verschoben oder gelöscht, keine gemeinsame Cargo-Konfiguration verändert.

Konkrete Lösungsrichtung für den zuständigen Helferauftrag: run_cargo im bestehenden Rust-Helfer an die kontrollierte eigene Buildablage binden, zum Beispiel CARGO_BUILD_BUILD_DIR auf denselben bereits geprüften Targetpfad setzen. Hardlink-, Besitzer-, Mode-, Source-/Main- und Doppelverify-Grenzen erhalten, bestehenden echten Cargo-/forged-bundle-Test auf die konfigurierte getrennte Zwischenablage nachziehen; Gate und reguläre Installation des Helfers. K hat dafür keine Freigabe zur Änderung des fest installierten Helfers und verlangt eine geordnete zuständige Reparatur, keinen alternativen Deploymentpfad. Keine weitere identische Releasewiederholung vor dieser Kompatibilitätskorrektur.

Der aktuelle Consumerrelease bleibt fertig und hashgeprüft, aber vor dem Brainreadgate nicht aktiviert. Alle eigenen Releasejobs sind tatsächlich beendet; kein aktiver Doppelbuild oder neuer Sourceworker.

Konkrete Layoutursache über normalen bestehenden Parentthread zugestellt, sequence 1937797. Vorheriges read meldete ready, normaler send traf running-Race; anschließend vorgesehene bewusste --force-Option für dieselbe Statusmeldung verwendet. Keine neue Session oder Modellwahl. Helfer-Kompatibilitätsreparatur geordnet angefordert, kein Freigabe- oder Fertigergebnis daraus behauptet. K bleibt für den bestehenden Auftrag verfügbar; kein Cleanup oder settle vor Aktivierung und Livebeweis.

## Livegrenze

Zuletzt tatsächlich: brain-serve PID 3178539, exe auf b7289d11 ohne deleted; dl-bot 2766584 und dl-web 2766645 auf 0fb873c6 ohne deleted. User-Units deadlock-bot-rust und deadlock-web-rust aktiv, NRestarts 0. Gleichnamige System-Units sind nicht die produktiven User-Units und wurden nicht gestartet. Start-/Journal-/Funktionsbeweis für neue Quellen steht aus.

Nach erfolgreichem regulären Braininstall: tatsächlichen brain-serve-Prozess neu starten und belegen, erst danach neuen Bots-current unter bestehender Sperre aktivieren und bot-restart dl-bot web. Nutzer prüft anschließend selbst die private Pocket-Frage. Keine echte Nutzerantwort oder neue Invitefunktion aus Fixtures behaupten. Orts-WIP und V-Dateigrenzen bleiben wie in CONSUMERWEG-Q-UND-DATEIGRENZE-20261008.md erhalten.
