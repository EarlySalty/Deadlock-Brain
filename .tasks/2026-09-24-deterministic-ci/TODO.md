# Deadlock-Brain: CI-Basis und verbleibende Abnahme

## Auftrag und unveränderliche Zuordnung

- Repository: `EarlySalty/Deadlock-Brain`, ausdrücklich nicht Deadlock-2nd-Brain.
- PR: https://github.com/EarlySalty/Deadlock-Brain/pull/10
- Branch: `ci/deterministic-pr-gate-20260924`.
- Eigener Worktree: `/home/nathanael/.worktrees/brain-deterministic-pr-gate-20260924`.
- Ausgangsbasis: `dfefc8f2e0df623d838e0ec7c1f18d86492ae1b5`.
- Schema-Vertrag: `bb49393e87d9da8b7804eaf84a9b8ea3797b12be`.
- Erste vollständige CI-Konfiguration: `72d1ae10d32d40d7d37e58777f7182cf528e105b`.
- CodeQL-/URL-Korrektur: `579fbd7631935512fffc2cc6ad38c3a8c6eaf75d`.

PR-Testbetrieb nach der verbindlichen Entscheidung vom 24. September 2026:
PR offen lassen, keine automatische Genehmigung, kein Merge, kein Main-Push,
kein Deploy, keine Produktionsmigration, kein Dienst-Neustart, kein Cleanup
fremder oder eigener Worktrees vor einer späteren Freigabe. Die fremden,
uncommitteten Änderungen im ursprünglichen Checkout wurden nicht bearbeitet.

Die Akte dokumentiert konkrete Beweisstände, nicht eine zeitlose Freigabe.
Jeder neue Commit, einschließlich dieses Dokumentationscommits, benötigt einen
eigenen Actions-Lauf. Der abschließende PR-Bericht führt den danach aktuellen
vollständigen Head-SHA, den tatsächlich geprüften synthetischen Merge-SHA,
Run-ID, Versuch und Schlusszustand auf. Alte grüne Jobs gelten nicht für neue
Commits. Es gibt noch keine erfolgreiche Gesamtabnahme.

## Implementiert

Die versionierte Policy steht in `SECURITY-CI.md`, der Schema-Vertrag in
`schema/README.md`, der Workflow in `.github/workflows/ci.yml`.

Rust: gepinnte Toolchain, Formatierung, Clippy mit `-D warnings`, Build aller
Targets, echte Workspace-Tests und SQLx-Validierung gegen das isolierte echte
PostgreSQL-Schema, Cargo-Audit und Cargo-Deny. Python: vorhandene pytest-Tests,
Ruff-Korrektheitsregeln, Syntaxprüfung, gehashte Runtime-/Test-/Scanner-Locks,
pip-audit, Bandit. Keine erfundene Mypy- oder npm-Toolchain.

Gitleaks, Semgrep für Rust/Python, Trivy HIGH/CRITICAL, actionlint, zizmor und
Dependabot sind eingebaut. CodeQL läuft mit security-extended für Rust und
Python auf PRs und im deklarierten Wochenplan. Der Zeitplan kann erst nach
einer später erlaubten Integration in den Default-Branch tatsächlich laufen.

`Required PR Gate` verlangt alle sechs erwarteten Job-IDs und ausschließlich
`success`; fehlende, zusätzliche, abgebrochene, übersprungene oder fehlgeschlagene
Prüfungen blockieren. Nur Diagnose-Uploads dürfen separat fehlschlagen.
Die unabhängige GitHub-Mergeanforderung ist noch nicht eingerichtet, siehe unten.

## Tatsächliche lokale Prüfungen

Unter `/tmp/brain-ci-20260924/` liegen die lokalen Rohlogs. PostgreSQL lief als
eigene Wegwerfinstanz ausschließlich auf Loopback-Port 56439 und wurde nach
jedem Testblock wieder gestoppt. Kein Produktions-DSN oder Secret wurde geladen.

- Ausgangsstand: 531 Rust-Tests bestanden, 60 ignoriert. Ein alter DB-Test
  lieferte ohne Datenbank fälschlich Erfolg; dieser Fehler wurde reproduziert.
- Implementierter Stand: 539 Rust-Tests bestanden, 52 explizit inventarisierte
  historische Tests nicht ausgeführt. Acht zuvor ignorierte selbständige
  PostgreSQL-Roundtrips sind jetzt verpflichtend und dürfen nicht selbst skippen.
- Formatierung, striktes Workspace-Clippy und kompletter Target-Build bestanden.
- Der Doctest-Aufruf bestand technisch, enthält aber null Doctests und ist
  ausdrücklich kein zusätzlicher Testnachweis.
- Nach der CodeQL-Korrektur: 40 Python-Tests bestanden, null Skips. Darunter
  echte MCP-Abfragen auf synthetischen Daten im tatsächlichen Runtime-Schema,
  SQL-Parameter-/Readonly-Prüfungen, XML-Sicherheitsprüfungen und 28 URL-Fälle.
- Sieben Rust-Selbsttests prüfen das Testlog-Gate. Gate-/SARIF-Gegenproben
  einschließlich echter CodeQL-Extension-Regelstruktur bestehen.
- Lokale Scanner: Gitleaks, Semgrep, Bandit, pip-audit, Cargo-Deny, actionlint
  und zizmor bestanden. Cargo-Audit blockiert weiterhin RUSTSEC-2023-0071.
- Zwei frische Datenbanken wurden aus den versionierten Schemaquellen aufgebaut.
  `brain.hero_catalog` wurde abgefragt. Ein Replay auf vorhandene Brain-Relationen
  und ein Nicht-Test-Datenbankname wurden vor Änderungen abgewiesen.

Weitere kontrollierte Gegenproben: Python-Skip und null ausgewählte Tests
liefern Exit 1. Fehlende Rust-Test-DSN, nichtlokaler Host, Nicht-Test-Datenbankname
und unerreichbare Testinstanz liefern Exit 101 statt grüner Tests. Die Scanner
lehnen temporäre, nicht ausgeführte Python-/Rust-, synthetische Secret- und
bekannt verwundbare Dependency-Fixtures mit ihrem Findings-Exitcode ab.
Ein Scanner-Programmfehler gilt dabei nicht als erfolgreicher Negativnachweis.

## Echte GitHub-Läufe

Erster Lauf, Versuch 1, Head `72d1ae10d32d40d7d37e58777f7182cf528e105b`:
https://github.com/EarlySalty/Deadlock-Brain/actions/runs/35942187782

Rust/PostgreSQL, Python/MCP, Workflow-Policy und allgemeine Scanner bestanden.
Cargo-Audit, die CodeQL-Ergebnisprüfer und folgerichtig Required PR Gate waren rot.
Die SARIF-Artefakte wurden heruntergeladen und ausgewertet, nicht nur der
Action-Exitcode betrachtet. Python hatte acht URL-Klassifikationsbefunde.
Rust hatte keinen Security-Fund, aber eine fehlerhaft extrahierte Helferdatei
unter insgesamt 131 eigenen Dateien. Beide Ursachen wurden bearbeitet.

Die allgemeinen Security-Artefakte dieses Laufs belegen: Semgrep hat 131 Rust-
und 40 Python-Dateien ohne Parserfehler untersucht. Trivy erkannte 49 Python-
Pakete und 367 Cargo-Lockfile-Pakete. pip-audit prüfte 49 Runtime-Pakete.
Die HIGH/CRITICAL-Schwelle von Trivy ist nicht gleich der strengeren
Cargo-Audit-Policy und erklärt keinen Verzicht auf den RSA-Befund.

Korrekturlauf, Versuch 1, Head `579fbd7631935512fffc2cc6ad38c3a8c6eaf75d`:
https://github.com/EarlySalty/Deadlock-Brain/actions/runs/35943250150

Rust/PostgreSQL einschließlich 539 Tests, die 40 Python-Tests, Python-CodeQL
einschließlich Findings-Gate, allgemeine Scanner und Workflow-Policy sind
dort erneut erfolgreich. Cargo-Audit ist
weiter rot. Der vollständige Abschluss einschließlich Rust-CodeQL wird im
abschließenden PR-Bericht mit SHA und Run-Zuordnung festgehalten.

## Schema-Übergabe an die Twitch-Session

Die Übergabe wurde bereits im zuständigen Twitch-PR veröffentlicht:
https://github.com/EarlySalty/Deadlock-Twitch-Bot/pull/953#issuecomment-5805366610

Unveränderlicher Brain-Schema-SHA: `bb49393e87d9da8b7804eaf84a9b8ea3797b12be`.
Upstream-Source-of-Truth: Deadlock-Bots/dl-central-db am Commit
`ff635f7b354cb09909c01ddd6f773d0682dd89c9`, ergänzt um die beiden unveränderten,
bereits versionierten Brain-Reasoner-/Population-Migrationen.

Voraussetzung: PostgreSQL 16, psql, leere isolierte Test-DB und DDL-Rechte;
Datenbankname beispielsweise `twitch_ci`. Keine zusätzlichen Extensions,
Produktionsdaten, Produktions-Secrets oder privaten Sibling-Checkouts.

```sh
sha256sum --check schema/SHA256SUMS
psql "$DATABASE_URL" -X --set=ON_ERROR_STOP=1 \
  --file scripts/ci/bootstrap-brain-schema.sql
```

Vor Compile-Prüfungen der Brain-Crates aufrufen, `DATABASE_URL` auf diese
Test-DB setzen und `SQLX_OFFLINE=false` verwenden. Nicht über bereits angewandte
Brain-Migrationen legen. Nicht-Brain-Schemas bleiben beim Consumer. Ausschließlich
die Twitch-Session ändert den Twitch-Dependency-Pin. Hier wurde kein Twitch-Code
bearbeitet.

## Offene Punkte in Bearbeitungsreihenfolge

### 1. Cargo-Audit / RSA: keine grüne Gesamtabnahme

`rust/Cargo.lock` enthält `rsa 0.9.10` über die optionale SQLx-MySQL-Abhängigkeit.
`cargo tree --locked --manifest-path rust/Cargo.toml --target all -i rsa
--edges normal,build,dev` findet keinen aktiven Abhängigkeitspfad im vorhandenen
PostgreSQL-Build. Der gesamte Lockfile bleibt dennoch Gegenstand von Cargo-Audit.

RustSec führt RUSTSEC-2023-0071 weiterhin ohne reparierte Version; der am
24. September 2026 abgefragte Eintrag wurde zuletzt am 14. September 2026
aktualisiert und nennt auch die neue Release-Candidate-Linie als betroffen:
https://rustsec.org/advisories/RUSTSEC-2023-0071.html

Owner: EarlySalty / Brain-Maintainer. Einen geeigneten SQLx-/Dependency-Umbau
prüfen oder eine separate, explizit begründete, eng begrenzte und ablaufende
Policy-Entscheidung treffen. Keine pauschale Ignore-Liste, kein ungeprüftes
Prerelease-Upgrade und kein leeres Ersatz-Audit. In diesem PR wurde keine
Advisory-Freigabe erteilt; der Befund bleibt blockierend.

### 2. GitHub-Mergeanforderung administrativ noch offen

Read-back: `GET /repos/EarlySalty/Deadlock-Brain/branches/main/protection`
antwortete 404 `Branch not protected`; `GET .../rulesets` lieferte `[]`.
Die ausstellende GitHub-Actions-App der Checks hat ID 15368.
Ein geplanter additiver Ruleset-Schreibaufruf wurde bereits vom Werkzeug mit
`Sicherheitsstatus nicht bestimmen` blockiert. Es gab keinen bestätigten
GitHub-Schreibvorgang und keinen erfolgreichen Read-back einer neuen Regel.
Das beweist keine bestimmte GitHub-Plan- oder Account-Beschränkung.

Owner: EarlySalty / Repository-Administration. Erst nach der notwendigen
Gesamtabnahme den exakten Check `Required PR Gate`, App GitHub Actions (15368),
für die geschützte Basis als aktive Anforderung einrichten und zurücklesen;
keine bestehenden Regeln entfernen und keinen lokalen Deny umgehen. Der
Workflow-Check allein schützt einen ungeschützten Main-Branch nicht.

### 3. Verbleibende Test- und Extraktionsgrenzen

Die 52 namentlich inventarisierten historischen Tests sind keine bestandenen
Abnahmen. Einige benötigen Referenzkorpora, andere weitere isolierte Fixtures.
Review durch EarlySalty bis 24. Oktober 2026; gezielt in echte, deterministische
Tests überführen, nicht die Ignore-Liste blind erweitern. Inline-Bandit-
Ausnahmen und tatsächliche Sprach-/Extraktionsgrenzen stehen in SECURITY-CI.md.

Ein normaler vollständig grüner PR ist wegen Punkt 1 noch nicht nachgewiesen.
Der offene Draft-PR hat den Gate-Check nachweislich. Der Dokumentationscommit
dieser Akte muss die vollständige CI erneut auslösen. Keine Freigabe aus
alten Runs übernehmen. Merge, Deploy und Neustarts bleiben unabhängig von
späteren grünen Checks durch den PR-Testbetrieb untersagt.
