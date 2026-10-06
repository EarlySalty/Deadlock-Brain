# Ergebnis des Validator-Nachlaufs

Geprüfter Eigenstand: `9d236af5ead09a221f290fe437e14ccaf77ba220`, Basis `619dcb06d621f812699460427fa6d57fae056bb5`. Der tracked Quellstand blieb während des Laufs unverändert. Der Eigenhunk in `rust/crates/deadlock-brain-core/src/config.rs` verlangt jetzt mindestens 128 Antwortbytes, entsprechend dem bereits bestehenden Connectorminimum. Der vorhandene Validatorregressionsfall prüft die Grenzen 0 bis 127 und 128. Es wurden keine Defaults geändert.

Der Wrapper 1242168, Exec-Session 14303, hielt beide vereinbarten Hostlocks durchgehend. Er wartete bei fremden Cargo-Prozessen mit beiden Sperren und startete den eigenen Cargo-Prozess 1722906 erst nach einer freien vollständigen Non-Zombie-Probe. Der Test verwendete `--locked --offline -j2`. Die externe Pfadquelle wurde lesend gebunden, nicht übernommen: Deadlock-Bots-HEAD `b379cdf49fe1c54dc31610bd73362cee2608787d`. Die erfassten Blobhashes stehen in `validator-external-dependencies.sha256`.

## Einzelprüfungen

- `rustfmt --edition 2021 --check rust/crates/deadlock-brain-core/src/config.rs`: Exit 0.
- `cargo test --locked --offline -j2 -p deadlock-brain-core --lib config::tests::ai_configuration_selects_subscription_luna -- --exact`: Exit 0, ein Test bestanden, keine Fehler, 30 herausgefiltert. Kompilierung 33,16 Sekunden.
- Bestehendes `gate_hook.py --review`, Basis 619dcb06, Head 9d236af: Exit 0, ALLOW. Die 128-Byte-Grenze und der Grenztest sind laut Gate konsistent; keine blockierenden Defekte.

## Hashbindung

| Beleg | SHA256 |
| --- | --- |
| validator-fmt.log | e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 |
| validator-test.log | cb3286b2f2c55a7a5f6f17e54fc4b2462f088d5907b7224cf7442828d63547ee |
| validator-gate.log | c29ee12bde1e37faaa1a936edd6914eeb893162ed0143ff69cb167daf362c1b1 |
| validator-external-dependencies.sha256 | caf7f72c2d7105d53a71bb39f85479f1d26e01adabc8c23041fe81f9a5961bab |

## Abbau und Grenzen

Der Wrapper endete regulär mit Exit 0. Seine Abschlussausgabe bestätigt das Schließen der Sperren. Wrapper und direkte Kinder waren bei der anschließenden Probe nicht mehr vorhanden. Die Compilerprobe danach ergab Exit 75 wegen des fremden Cargo-Prozesses 1757961, PPID 1403431. Beide nachträglichen Lockgegenversuche ergaben tatsächlich Exit 1; freie Hostlocks werden deshalb nicht behauptet. Fremde Prozesse wurden nicht verändert.

Dieser Nachlauf ergänzt ausschließlich die Validatorprüfung. Die historischen Ergebnisse des FIFO-Heads 619dcb06 bleiben separat. Die korrigierte Credentialprobe gemäß `CREDENTIAL-NACHPROBE-PLAN.md`, SHA256 `b809af01b30dd11c53c0594e49398c2cd703dfe24f4b9f8b174f70cc3dd7e508`, wurde noch nicht ausgeführt. Die früheren 14 Credentialversuche bleiben wegen Namespacefehlern BLOCK.

Der tatsächliche produktive `tools[]`-Nachweis, CLI-Isolation und Persistenzabnahme sind weiterhin offen. Private CLI-Aufrufe bleiben gesperrt. Ein harter CLI-Tokendeckel ist nicht belegt. Der separate Dialogstand 35c93de0 ist noch nicht gegen den tatsächlichen gemeinsamen Guide-/Serve-Kontext geprüft. Es gab keinen Merge, Releasebuild, Deploy oder produktiven Neustart durch diesen Nachlauf. Eine Gruppenfreigabe wird nicht erteilt.
