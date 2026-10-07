status: geliefert
Datum: 2026-10-03

# Q9: Shadowrunner lokal geprüft, echter Vergleich gesperrt

Vorhandenen Runner erhalten. Engster Compilerfix: `Observed` aus `brain_contracts::value`, danach ausschließlich beide Runnerdateien formatiert. Keine Manifeste, Locks, fremden Quellen, Datenbankstände oder Dienste geändert.

Rust 1.97.1, beide Hostlocks und frische NonZombie-Probe, höchstens zwei Jobs, --locked --offline. Nach Fix: cargo check, striktes Clippy, Tests mit --include-ignored und Offline-prepare jeweils Exit 0. Drei Tests bestanden, null fehlgeschlagen, ignoriert oder gefiltert. Wrapper 3290580 und 3384153 beendet. Kein echter Anbieteraufruf.

TESTNACHWEIS[TW-1]: 3 passed, 0 ignored | Baseline: nicht erhoben rot

Logs im Q-Worktree: `.q9-shadow-check.log`, `.q9-shadow-verification.log`, `.q9-shadow-check-fixed.log`, `.q9-shadow-clippy.log`, `.q9-shadow-tests.log`, `.q9-shadow-prepare.log`.

## Offlinekorpus

`architecture/migration/evals/q-shadow/offline-cases-20261003.json`: 103 eindeutige IDs und Fragen, davon sieben veröffentlichte Originalfragen und 96 ausdrücklich synthetische Varianten. Alle public, expected=unlabelled, gold_labels=false. Fünf vorgegebene öffentliche Docs-Seiten, 24 Überschriften. Herkunftscommit `4b072aee3127564def674f4b53d9be5d1d42cfdf`. Fall-Datei SHA256 `dff1bd83048b800c02dbd36656f7964b36e118eb11ac0f6e4298947e73b6d37f`.

## Konkrete Vergleichsgrenze für Z

Der Runner startet keinen Dienst. Er kann grundsätzlich eine erhaltene HTTP-Baseline und den lokalen Kandidaten auf demselben festen Corpus vergleichen. Vor gemeinsamem Produktivwechsel muss das tatsächliche Baselinebinary mit unveränderlich gebundener geheimnisfreier Konfiguration auf separatem Loopback-Port erhalten bleiben. Kandidat über den bestehenden brain-serve-Startweg an einem anderen Loopback-Port. Produktionsdienst unverändert lassen. PID-, Binary- oder Konfigwechsel während des Laufs beendet die Attestierung.

Q9 beobachtete Baseline PID 3183434, SHA `f7b02cc1933f7069e68be3037e7f838498cb0487` unter `/opt/deadlock-brain/maintenance-releases/`. Das ist ein zeitgebundener Befund, kein frisch bestätigter aktueller Betriebsstand.

Der überprüfte vorhandene Import mit Paket-SHA256 `3c33bc8340674d0fa6fdeadfd0a565c352b6a5274774ad4c3f2970e81f7b5132` trägt Bot-Revisionen und abgeleitete HTML-Hashes. Alle fünf Herkunftscommits und Raw-Hashes weichen von den gepinnten Docs-Seiten ab. Dokumentierte Publication-/Egressrechte ersetzen keine aktuelle Quellrevision-/Releaseprüfung. Die Herkunft wurde nicht umgeschrieben.

C9 allein löst das nicht: Import liefert docs.public, Runner verlangt bisher bot.public und akzeptiert keinen abweichenden Credential-Release. Nötig ist eine tatsächlich passende Herkunfts- und Scopebindung über bestehende öffentliche Verträge, keine zweite Pipeline.

Außerdem fehlen Kandidatenprozess mit commitbenanntem Binary, PID und explizitem Konfigpfad sowie aktuelle lokale DB-Prüfung aller anbietersichtbaren Revisionen, Heads und Rechte. Direkter Peerzugriff wurde abgewiesen. Vorhandener Infisical-Weg bleibt `/etc/deadlock-brain/infisical.json`, Credential-FD 5; keine Zugangswerte in Artefakten oder Argumenten. Wirksame Modellwahl, identische Budgets und Fristen müssen für beide Pfade belegt werden. API-Audit und Konfigdatei allein beweisen keinen Anbieteraufruf.

Der geprüfte Aufrufvertrag steht in `architecture/migration/evals/q-shadow/profile.json`: run, Docs-Repo, Baseline-/Kandidatenkonfig, Infisicalkonfig, Client-Secretname, beide PID/SHA-Paare, `--execute-approved-public`, neuer JSONL-Ausgabepfad.

`provider_shadow_passed` bleibt null. Keine reale Antwortqualität, Nutzung oder Kosten gemessen. Gebaut und offline geprüft, nicht gemergt, installiert oder live verglichen.
