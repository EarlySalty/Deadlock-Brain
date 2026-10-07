status: Bestand lesend erfasst, eigene Übernahme offen
Datum: 2026-10-03

# Bots: sichere Consumerübernahme

Quelle: abgeschlossener nativer Bestandsworkflow wf_b4a925b8-1f3, Werkzeug-ID w2dat39ph. Keine fremden Worktrees verändert, keine Tests, Compiler oder Securityprüfung ausgeführt.

## Schutz und Ausgangsstand

Lesende Threadprobe um 16:32 UTC: a99dc9e9-3ce1-41bc-ba6b-391cc190f518 stopped, c0b1d111-e402-4ed3-bb0b-68958a1ba699 ready. Der zweite Zustand belegt keinen Abschluss des separat genannten Migrationsresolvers. Beide Threads bleiben unangetastet. Vor eigenem Schreibbeginn den Schutzstatus erneut lesen.

PR #459 ist offen und Draft, mit Konflikten. Alter PR-Kopf: e805fbed60a9176984538f2edaa3960209c589ab, Branch codex/fix-c9-consumer-wiring. Draftstatus erhalten. Keine GitHub-Automation auslösen und keinen PR neu anlegen.

Remote-main und lokaler origin/main bei der Probe: ae490cd9e9f6d647f5107cc530db63e6a11d231f. Eigener Worktree ist noch nicht angelegt. Für die Übernahme frisch fetchen und einen eigenen Branch von origin/main verwenden.

## Kleinster sicherer Quellbereich

Vorhandener eigenständiger Consumer: sol/abschluss/7bf0e0375ee34a00, Kopf e3e649ccc13193c9dee16cca7651cabb26f25ab3.

Den vollständigen Bereich 1e6cdf648429e50014e548b0b4204c8ac48cb4ec..e3e649ccc13193c9dee16cca7651cabb26f25ab3 übernehmen: elf Commits für Consumer, Lockstand, geschützte Laufzeitfelder, Credentialloader und Adapterkorrekturen sowie Testmodusgrenze. Der erste Bindingcommit 32732443735e2371cb5872fd4b01c896802d3672 allein reicht nicht.

Betroffene Pfade:

- rust/crates/dl-brain/
- rust/crates/dl-core/
- rust/crates/dl-token-secrets/src/lib.rs
- rust/bin/dl-bot/src/main.rs und modglue.rs
- rust/Cargo.lock

Dieser eigenständige Bereich bringt keine neuen Community- oder Privacy-Migrationen mit. Aktueller main enthält vier zusätzliche Launcherkorrekturen. Bei der Übernahme deren Verhalten erhalten und Konflikte anhand der aktuellen Datei auflösen, keine ganzen alten Dateien zurückspielen.

## Nicht pauschal übernehmen

Der gekoppelte Integratorstand sol/abschluss/7b50cc4c87b8e9ee, Kopf 635f6b6be005899cb747e82ab095d7123eb9a276, integriert den Consumer selektiv mit e510305633878f0c244a07e28fab10ec686cfdd4. Seine Adapter-, Config-, Credential- und Prüfscripte entsprechen dem eigenständigen Consumer, der gesamte Bereich aus 19 Commits enthält jedoch zusätzlich Community, Clips, Punkte, Privacy, Migrationen und systemd. Diese fremden Bereiche gehören nicht zum engen Paket K.

Separater Guidebranch integrate/serverguide-deploy-20261003, lokal 3e5f9056d6fb3442d189229f257b637650b78e72: Resolverabschluss und endgültiger SHA sind nicht belegt. Den alten PR-Kopf ebenfalls nicht vollständig übernehmen; er enthält weitere Bildfragen- und Build-Publishing-Arbeit.

## Vertrag und offene Prüfung

brain-client ist an 3b86d3cbe5ea39a67b8b1fbd8a3d48ab935982ef gebunden. Scope bot.public, bestehender Principal und normale TOML. Standardmodus legacy, tatsächliche Aktivierung noch nicht belegt. Kein fremder grüner Bericht zählt als eigene Prüfung.

Nächster Bauschritt: eigener Worktree Deadlock-Bots-brain-consumer-fertig, Branch feat/brain-consumer-fertig-20261003, vollständiger sicherer Quellbereich, aktuelle Launcherkorrekturen erhalten. Danach die betroffenen Rust-Prüfungen unter beiden Hostlocks, frische Intent-Abnahme und SHA-gebundene Übergabe an Z. Keine eigene gekoppelte Integration, Aktivierung oder Produktionstest vor Zs gemeinsamer Installation.
