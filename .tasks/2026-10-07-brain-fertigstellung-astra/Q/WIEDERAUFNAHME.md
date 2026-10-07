# Q: Wiederaufnahme desselben Evaluationsauftrags

Stand der lokalen Abnahme: 2026-10-07, 13:24:02 UTC. Der Collector-/Methodikteil ist auf main und abgeschlossen. Der beauftragte eigene Q-Branchabschluss ist erfolgt: Branch und Worktree entfernt, zentrale Integritätsprobe danach erfolgreich. Der Thread bleibt bereit für die echte private Providerentscheidung und I/G/K-Lieferungen, ohne Self-Settle und ohne Entwicklungsbranch als Warteplatz.

## Dauerhafte Ablage

Zentraler eigener Aufgabenordner:

`/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/Q/`

Dort liegen Report, STATUS, Originalfakten, fünf vorbereitete Patchstichproben, Sicherungsnachweise und der unveränderte Rust-Collector. TODO, REGISTER, Produktdateien und fremde Worktrees wurden nicht bearbeitet. Vorhandene Ziele wurden geprüft; der zentrale Q-Ordner existierte vorher nicht.

Private Arbeitskopie: `Q/private/`. Unabhängige lokale Sicherung außerhalb des ehemaligen Worktrees:

`/home/nathanael/.local/share/brain-q-private-20261007-bf54e659`

Am 12:49:44 UTC stimmten die 17 aktuellen privaten Dateien aus dem Q-Worktree mit dieser bestehenden Sicherung überein. Acht JSON-/Digestbindungen waren korrekt. Um 12:55:41 UTC war zusätzlich die zentrale private Arbeitskopie pfad- und byteidentisch. Je Kopie: vier Verzeichnisse mit 0700, 17 Dateien mit 0600, keine Symlinks, abweichenden Modi oder falschen Eigentümer. Inventarhash: `30c0a264b782d8d9f6260de5c929597e77f49b1ce56cd520f901d2eab96d0caf`. Die zusätzliche zentrale Kopie ermöglicht den bisherigen Collector ohne Codeänderung; die unabhängige Sicherung wurde dabei nicht verändert.

Private Originale und Teilplan MUST NOT in die Modellkonversation, externe Dienste oder Git-Uploads gelangen. Secrets NEVER ausgeben. `private/` und `collector/target/` sind lokal Git-ignoriert. Die Sicherungsnachweise enthalten technische Metadaten, keine Originaltexte oder Personenkennungen.

## Geprüfter Code und tatsächlich nutzbarer Befehl

Unveränderter Collector-Code aus dem geprüften Commit:

`bf54e659814008c8db941e6ff6be9016314347d3`

SHA256 von `collector/src/main.rs`:

`c7628ed475f5a5dfe86639bd2a3140ea3ba56c808be65f30b6922a0d78229735`

Die zentrale Quelldatei wurde gegen das Git-Objekt dieses Commits und die ursprüngliche Worktree-Datei geprüft. Sie stimmt überein. Das Binary wurde anschließend mit dem zentralen Manifest neu gebaut, nicht aus dem bisherigen Worktree übernommen. Damit verweist seine eingebettete Quellenwurzel auf den zentralen Q-Ordner.

```sh
RUSTUP_TOOLCHAIN=1.97.1 /home/nathanael/.cargo/bin/cargo build --locked --offline --jobs 3 --manifest-path /home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/Cargo.toml
/home/nathanael/repos/Deadlock-Brain/.tasks/2026-10-07-brain-fertigstellung-astra/Q/collector/target/debug/brain-q-source-snapshot verify q-partial-v1-20261007
```

Beide Befehle wurden ausgeführt, Exit 0. Build: 6,02 Sekunden. Zentrales Binary-SHA256: `98010d7e87519f45c7244d846d3a60e44327a44a34d5801591563d98a519bb07`. Die Integritätsprobe meldete vier gebundene Quellen, 166 vorläufige Kandidateneinträge, 0 akzeptierte Goldfälle und 0 Modellaufrufe. Teilplan-SHA256: `c2332949206a0773b604391d7f540c9520598a66409053f94defb4076447b0ed`.

Zusätzlich am zentralen Manifest ausgeführt: fünf Tests mit `--include-ignored`, Clippy mit `--all-targets -- -D warnings`, `fmt --check`, jeweils Exit 0. Kein Release-Build, Produktdeploy oder Dienstrestart.

TESTNACHWEIS[TW-1]: 5 passed, 0 ignored | Baseline: nicht gemessen, keine Altfehler als rot behauptet

## Offener fachlicher Zustand

- Quellenversion und vorläufiger Teilplan: `q-partial-v1-20261007`. **166 Kandidateneinträge sind keine 30 akzeptierten Goldfragen.** Herkunft, Überschneidungen, vollständige Kategorien und Sollfakten je Fall sind offen; unbeschriftet bedeutet nicht Unsinn.
- Öffentliche Assets: `q-original-v1-20261007`. Patchstichproben: `q-patches-v1-20261007`. Faktenberichte sind vorbereitet, keine echten Antworten oder Zustellungen abgenommen.
- Baseline- und Wiederholungsläufe mit privaten Fragen: 0. P1-Ende-zu-Ende-Zeit: keine. Historische Health-/Prozessproben und acht isolierte HTTP-Clienttests zählen nicht als zugestellte Discord-/Twitch-Antworten.
- Private Antwortprobe bleibt gesperrt. Kein eigenmächtiger Anbieter-, Modell- oder Timeoutwechsel, kein neuer Connector. Loopbackadresse allein belegt keine lokale Inferenz. Die eng begrenzte eigene Invite-Status-Ausnahme wird nicht auf Rohfragen oder fremde Personen erweitert.
- Gesamt-Q und P0/P1 bleiben offen. Der Thread wird nicht gesettelt. Kein Polling fremder Sessions, keine neuen Review- oder Implementierungsthreads und keine produktiven Eingriffe während des Wartens.

## Wiederaufnahme nach tatsächlicher Entscheidung und Lieferung

1. Orchestratorentscheidung und I/G/K-Artefakte lesen. Den dann tatsächlich laufenden Provider und seine Datenflussgrenze prüfen; eine Code- oder Wissensrevision ist keine automatische Freigabe privater Fragen.
2. Den obigen lokalen Integritätsbefehl ausführen. Bei fehlender oder abweichender Datei abbrechen, die unabhängige Sicherung pfad-/hash-/rechtegeprüft wiederherstellen und Originale erhalten. Keine private Datei zur Fehlerbehebung veröffentlichen oder überschreiben.
3. Herkunft und Goldfälle lokal vervollständigen. Antwortarten vor dem ersten Modelllauf festhalten. Kein heuristisches Label als fertiges Gold und keine synthetische Ersatzfrage als echte Herkunft zählen. Ein später abgenommenes Evalset nach seinem ersten Lauf unverändert wiederverwenden.
4. Reale Antworten ausschließlich über den bestehenden freigegebenen Consumer abnehmen. Zulässige Zielkanäle, Identität, tatsächliche Zustellung und Ende-zu-Ende-Zeit prüfen. Ausfallprobe isoliert, ohne Produktionsdienste anzuhalten. Keine Invite-Sendewirkung oder Veröffentlichung fremder Builds.
5. Dasselbe feste Set pro tatsächlich beobachtetem Deploy verwenden. Code-SHA, Wissens-/Quellbindung, Zeiten und offene Kriterien getrennt protokollieren. Produktkorrekturen und Produktdeploys bleiben bei I/G/K. Neues Git-WIP nur für tatsächlich beauftragte Änderungen, nicht zum Offenhalten eines Wartethreads.

## Branchabschluss

Der sichere Dokumentstand `3ceb504d6cebc8dda6e7438a40acd8e7570128be` erhielt vom bestehenden Gate ALLOW, Exit 0, und wurde mit `HEAD:main` gepusht, Exit 0. Frischer Fetch und beide Ancestryprüfungen bestanden. Status einschließlich ignorierter Dateien wurde geprüft. Der eigene Branch wurde regulär gelöscht; Worktreepfad und Registrierung fehlen nach Entfernung ohne Force. Der Werkzeugaufruf meldete wegen des anschließend gelöschten Shell-Arbeitsverzeichnisses Exit 1; die erfolgreiche Entfernung ist durch die separate Nachprüfung belegt.

Um 13:24:02 UTC waren beide verbleibenden privaten Kopien unverändert: je 17 Dateien, vier Verzeichnisse, acht korrekte Digestbindungen, Inventarhash wie oben, Rechte 0600/0700, keine Symlinks oder falschen Eigentümer. Der zentrale Collector lief nach Entfernung erfolgreich mit dem obigen Verifybefehl, Exit 0. `STATUS.json` und `CLEANUP.json` dokumentieren den tatsächlichen Abschluss. Die nachträglichen lokalen Abschlussbelege gehören nicht zum zuvor gepushten Commit `3ceb504d`; der kanonische Produktcheckout wurde dafür nicht umgeschaltet oder committet. Kein ungenutzter Branch bleibt als Warteplatz.
