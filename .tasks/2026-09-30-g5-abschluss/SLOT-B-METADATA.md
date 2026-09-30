status: erledigt, ausschließlich zugeteilte Metadatenauflösung
Datum: 2026-09-30

# Einzeln freigegebene V1-Lockauflösung

## Freigabe und Arbeitsort

Der Integrator gab ausdrücklich nur `cargo +1.97.1 metadata --offline --format-version 1` frei. Slots B bis E sind darüber hinaus nicht zugeteilt. Keine neue Nutzerfreigabe angefordert.

Worktree: /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930
Branch: fix/g5-replay-deferred-20260930
Ausgangshead: 0c290809bb259a06dfe3c5d0e8eac9c0008c6e7f
Arbeitsverzeichnis: /home/nathanael/.worktrees/brain-g5-replay-deferred-20260930/rust
Vor dem Lauf sauber; Sol war ready und die vorherige Abgabe gepusht. Die Hauptsession führte nur diese mechanische Auflösung aus, kein zweiter Quellimplementierer war aktiv.

## Tatsächlich ausgeführter Befehl und Ergebnis

`/home/nathanael/.cargo/bin/cargo +1.97.1 metadata --offline --format-version 1`

Exitcode 0, gemessene Dauer 562 ms, kein Fehlertext. Der vorhandene Cargo-Cache wurde benutzt. Ein `rust/target` existierte weder vor noch nach dem Lauf. Kein neuer Targetcache, kein Fetch, kein Compiler-/Build-/Testauftrag, keine Dienständerung.

Cargo-Ergebnis:

- Workspace-Root exakt der genannte eigene Quellworktree.
- 25 von Cargo gemeldete Workspace-Member. Die bisherige Zahl 24 bezeichnet nur die expliziten V1-Einträge im Rootmanifest; der zusätzliche lokale Pfadbaustein heißt uplink-infisical-transport. 24 ist nicht die vollständige von Cargo gemeldete Memberzahl.
- 386 Pakete und 386 Resolve-Nodes.
- Keine Gitquelle im aufgelösten V1-Graphen.
- Keine Pakete dbrain-replay, haste, valveprotos oder dungers im Ergebnis.
- SHA-256 der vollständig verarbeiteten Metadata-JSON-Ausgabe: f143b11277a256d4d66278318ff1e7d014b1521d3b01154ffa4aec5ffdd1cf71. Die Rohdatei wurde nicht als neuer Cache abgelegt.

## Tatsächlicher Lockdiff

Nur `rust/Cargo.lock` wurde durch Cargo geändert. 427 auf 386 Pakete reduziert, 41 Einträge entfernt, keine neuen Paketidentitäten und keine geänderten Checksums. Versionen und Quellen sämtlicher verbleibender Pakete sind erhalten. Cargo vereinfachte außerdem nicht mehr nötige Versionsqualifizierungen in Dependencyreferenzen. Ein semantischer Kantenvergleich fand zusätzlich genau eine Änderung an einem verbleibenden Paket: chrono0.4.45 referenziert serde nicht mehr. Ob V1-Code dieses früher transitiv aktivierte Feature benötigt, muss der Reviewer ausdrücklich statisch prüfen; kein Compilerbeweis behauptet. Diffstat: 22 Einfügungen, 436 Löschungen. `git diff --check` bestand.

Der getrennte Replay-Lockstand und die Parser-/Schema-Pins bleiben unverändert. Seine eigene spätere Auflösung ist weiterhin Replay-Arbeit, kein V1-Buildbedarf.

Die frühere Guardblockade betraf die komplexen statischen Schreibbefehle des Sol-Workers. Der ausdrücklich erlaubte Cargo-Aufruf mit korrektem Arbeitsverzeichnis konnte den Rootlock normal schreiben. Kein Schutzmechanismus wurde abgeschaltet oder umgangen.

## Nächste Prüfung, nicht ausgeführt

1. Unabhängiger vorhandener Reviewer prüft diesen echten Cargo-Lockdiff zusätzlich zum bereits beauftragten statischen Gesamtstand; SHA dieses Commits steht in der Branchhistorie und der Buildanfrage.
2. Nächste sinnvolle Integratorzuteilung: `cargo +1.97.1 metadata --locked --offline --format-version 1`, anschließend Vergleich, dass die Lockdatei unverändert bleibt. Dieser zweite Metadatenlauf wurde nicht eigenmächtig gestartet.
3. Danach erst konkret zugeteilte Format-/Compiler-/Testprüfung mit vorhandenem Cache, einem Cargo-Job und ohne neue ENV-Konfiguration. SQLx-Konfigurationsbedarf vorher klären.
4. Kein Buildgrün, kein frischer Netzwerkbezug, keine vollständige Prüfung ohne vorhandenen Cache und kein typed Serve-Livebeweis aus dieser Metadatenauflösung ableiten.
