# FIFO-Nachprüfung auf 619dcb06

Geprüfter eigener Codehead: `619dcb06d621f812699460427fa6d57fae056bb5`, Worktree `/home/nathanael/.worktrees/brain-luna-abo-20261003`. Historischer Eigenanteil darunter: `940bb2138d694ada783c2d7d8b4abecb4d20fbed`. Kein eigener Merge, Release oder Deploy. Keine Gesamtfreigabe für die gemeinsame Integration.

Der Freeze blieb während des gesamten Nachlaufs unverändert. Vollsnapshot `FIFO-SNAPSHOT.tar`: SHA256 `bf573670decd9d5269828a7030f980172af3c3f635a56914dc25d6064bf20a79`; Quellmanifest `FIFO-MANIFEST.sha256`: SHA256 `ea2ed5c8b167260df0d739caa111e5a5fb6b1043f39c9c3e1088202718770de9`. Beide Manifestprüfungen vor und nach der Compilerphase waren erfolgreich.

| Prüfung, jeweils --locked --offline -j 2 | Tatsächliches Ergebnis |
| --- | --- |
| Formatprüfung der drei FIFO-Fixdateien | Exit 0 |
| Core --lib Tests | Exit 0, 31 bestanden, 0 ignoriert |
| Core all-targets Clippy -D warnings | Exit 0 |
| deadlock-brain all-targets Interfacecheck | Exit 0 |
| deadlock-brain Debugbau | Exit 0 |
| bestehendes Gate auf 619dcb06 gegen Basis 511a347 | Exit 0, ALLOW, gpt-6.1-sol |

Gateprotokoll `fifo-gate.log`: SHA256 `0bcabc996eb51fa3369fe96cd5369c461d0142b3f43eaeeb66c92786f7b215dc`. Das Gate nennt zwei NITs: tatsächliches `tools[]` bleibt unbewiesen; der normale Validator akzeptiert `max_response_bytes=1..127`, während der Connector mindestens 128 verlangt. Die zweite Einschränkung ist noch nicht korrigiert oder nachgeprüft. Es gab keine automatische Quelländerung oder zusätzliche Kompilierung.

## Tatsächliche Hardlink- und Configbelege

Zwei echte Hardlinks des Debugbinarys hatten denselben Descriptor-Dateiidentifikator `2820716161:8134500`, Linkzahl 2 und SHA256 `6778674d0d016a76128c8e9d40403b6893a2558bfd4046d8ba32a5efecc5b649`. `fifo-hardlinks.log` hat SHA256 `bb04897ca7f4495879e2bbc553881d7354e0801adb58ad26ef057cfb0cdfb996`.

Im privaten bwrap-Namespace unter `/opt/deadlock-brain/fifo-audit/619dcb06d621f812699460427fa6d57fae056bb5/legacy/{sheet-sync,build-data}` lief jeder direkte Hardlinkaufruf `ai-model` mit Exit 0. Beide Ausgaben belegen tatsächlichen Exepfad, eigene physische Jobroot und ausschließlich die jeweilige `config/bot.toml` für AI, Infisical und Settings. Sheet-Sync meldet exakt `/home/nathanael/repos/Deadlock-Brain/data`, Build-Data exakt `/home/nathanael/.worktrees/brain-live-main/data`. Modell und Provider sind `gpt-6-luna` und `openai_chatgpt_subscription`; dies sind Configbelege, kein echter Modellaufruf.

Je Job scheiterten fehlende, writerlose FIFO-, Symlink-, unlesbare, ungültige und übergroße normale TOML früh mit Exit 1. Damit liegen zwölf negative Configbelege vor. FIFO meldete die reguläre-Datei-Prüfung, unlesbar `Permission denied`. Kein Exit 124/137 und kein Namespacefehler in diesen Configproben.

## Credentialprobe weiterhin blockiert

Vierzehn ausdrücklich synthetische LoadCredential-Aufrufe für beide Jobhardlinks endeten mit Exit 1 vor dem Loader: bwrap meldete `Can't mkdir %d: Read-only file system`. `systemd-run` reichte den vorbereiteten Specifier hier wörtlich durch. Diese Aufrufe sind Namespacefehler und KEIN PASS für die Credentialgrenze. Die Fälle waren fehlend, FIFO, Symlink, unlesbar, ungültig, übergroß und leer. Kein tatsächlicher Credentialloadernachweis, kein Zugriff auf echte Tokens und kein Infisical-, DB- oder Modellaufruf.

Ein korrigierter systemd-Aufruf benötigt eine getrennt abgestimmte tatsächliche Runtime-Credentialpfadbindung. Er wurde nicht automatisch wiederholt. Probenstatuses `fifo-runtime-results.log`: SHA256 `44958e20853bc1b77652f8c84033bf194d6a7b3a7ec1d3e50db9e62265b7b693`.

## Externe vorhandene Sourceabhängigkeit

Die Suite verwendete die bereits vorhandenen Pfadabhängigkeiten im Worktree `/home/nathanael/.worktrees/Deadlock-Bots`, Githead `b379cdf49fe1c54dc31610bd73362cee2608787d`. `uplink-infisical-transport` war dort sauber. `dl-token-secrets/Cargo.toml` und `src/lib.rs` waren bereits staged added und gehören nicht zum reinen Githead oder unserem Eigencommit. Ihre bei der Interface-/Debugphase gebundenen Hashes blieben bis zum Abbau unverändert:

- Cargo.toml: `78a37ef4924cd7d2415b55f3269399348bdeb116db0e20991e4043d64ebe8330`.
- src/lib.rs: `43d2d3096e09d7a4145024a92e5ff649f02260ee0f1a017cf931630f550a641a`.

`fifo-external-dependencies.sha256`: SHA256 `7f07e1037193048f36f25dd99fc14737ae5dbd652fdff6f49f104cd975abe780`. Diese fremden Dateien wurden weder geändert noch in unseren Commit übernommen. Die gemeinsame Quelle muss diese externe Blobgrenze separat revisionsgebunden integrieren.

## Tatsächlicher Hostabbau

Wrapper PID 831418 hielt FD8 und FD9 mit Inodes 16006309 und 142952. Beide Gegenversuche waren während der gehaltenen Sperren tatsächlich Exit 1. Unmittelbar vor jedem Compiler wartete die vollständige Non-Zombieprobe auf einen freien Host.

Nach den Proben waren alle ausschließlich eigenen transienten Units bereits durch `--collect` entfernt. Der private Fixtureordner `/tmp/brain-fifo-619-vZKsPx` wurde vollständig entfernt. Beim vorbereiteten regulären Abschlussschritt war Wrapper 831418 bereits verschwunden; Session 30208 meldete tatsächlich Exit 1 ohne Abschlussausgabe. Eine Ursache ist nicht belegt. Es wird weder ein regulärer Exit 0 noch ein externer Abbruch behauptet. Eigener Wrapper, seine FD8/FD9 und direkte Kinder waren nicht mehr vorhanden.

Die anschließende Hostprobe war tatsächlich Exit 75 mit fremdem Cargo 1166388. Beide nachträglichen Lockgegenversuche waren Exit 1. Sie belegen belegte Sperren, keine freie Hostlage und keinen eigenen Lockrest. Fremde Nachfolger wurden nicht angefasst.

## Getrennter Dialogstand

Der separate Dialog-Eigenkopf `35c93de0cf7bd10225925b766f5b6a98733db296` betrifft ausschließlich zwei Providerdateien plus eigene Akte. Er ist statisch geprüft, aber noch nicht kompiliert, getestet oder gegatet. Scratch-Vertragsabhängigkeit `e7061c8` wird ausdrücklich nicht als Guide-Portersatz übernommen. Späterer Testkontext muss den tatsächlichen gemeinsamen Guide-/Servevertrag binden. PrivateDm bleibt vor Prozessstart gesperrt. Harte CLI-Tokenbegrenzung, tatsächliches tools[], Isolation und Persistenz bleiben offene gemeinsame Abnahmegrenzen.
