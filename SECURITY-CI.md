# Deterministische CI und Security-Policy

Stand: 2026-09-24. PR-Testbetrieb: kein Merge, Main-Push, Deploy oder Dienst-Neustart.

## Tatsächlicher Ausgangsstand

Basis ist `dfefc8f2e0df623d838e0ec7c1f18d86492ae1b5`, Repository
`EarlySalty/Deadlock-Brain`, nicht Deadlock-2nd-Brain. Der unveränderte Main
hatte keine Workflows, 130 Rust-Dateien und 38 Python-Dateien. Der Rust-Workspace
enthält elf Brain-Crates sowie das eingebundene Transport-Vendor-Crate.
Es gab vier Python-Testfunktionen in `mcp/test_server.py`, keine konfigurierte
Python-Lint-/Typecheck-Toolchain und kein npm-Projekt. `tests/` enthielt keine
zusätzlichen Python-Tests. Die Rust-Baseline führte 531 Tests aus und ignorierte
60. Ein ignorierter PostgreSQL-Test ließ sich ohne DB als erfolgreich ausführen.

## Required PR Gate

`.github/workflows/ci.yml` läuft ohne Pfadfilter auf jedem PR, auch auf Drafts
und Dokumentationsänderungen, außerdem auf Main-Push, Merge-Group, manuellem
Start und wöchentlich montags um 04:19 UTC. Alle Deep-Scans laufen im Testbetrieb
auch auf PRs. Der Zeitplan wird von GitHub erst aktiv ausgeführt, wenn die
Workflow-Datei im Default-Branch liegt; dieser PR wird dafür NICHT gemergt.

Der Check **Required PR Gate** verlangt die exakt erwarteten Job-IDs
`rust`, `python`, `rust-security`, `source-security`, `workflow-policy`, `codeql`.
Jeder Status muss `success` sein. Fehlende oder zusätzliche Jobs, Fehler,
Abbruch, unerwarteter Skip und sonstige Schlusszustände sind Fehler. Die
CodeQL-Matrix muss als Ganzes erfolgreich sein. Eine vollständige Absetzung
des Workflows kann den Check abbrechen, aber niemals erfolgreich machen.
Die Plattformregel muss genau diesen Check verlangen; ein erfolgreicher Jobname
allein stellt noch keinen GitHub-Mergeschutz dar. Die tatsächlich konfigurierte
Regel und die Run-Ergebnisse werden in der Task-Abnahme festgehalten.

Es gibt keine LLM-/Copilot-Abnahme. Alle Runner sind GitHub-hosted; PR-Code
bekommt keine Produktions-Secrets oder privaten Sibling-Deploy-Keys.
`permissions` ist auf `contents: read` begrenzt. Checkout speichert keine
Credentials. Actions sind an vollständige Commit-SHAs gebunden. PostgreSQL ist
an einen Image-Digest gebunden, Rust an 1.97.1, Python-Abhängigkeiten an
Versionen und Hashes. Binärscanner werden mit versionierten URLs und
SHA256-Prüfung installiert. CodeQL verwendet die mit dem Action-SHA verbundene
Tool-Version (`tools: linked`). Vulnerability-Datenbanken bleiben bewusst
aktuell; Download-/Updatefehler sind kein erfolgreicher Scan. Der Ubuntu-Runner
und die jeweils aktuelle Python-3.12-Patchversion sind keine bitidentisch
reproduzierbare Betriebssystem-Abbildung.

## Funktionale Prüfungen und DB-Isolation

Rust: `cargo fmt`, Clippy mit `-D warnings`, Build aller Workspace-Targets,
Workspace-Tests und Doctests, jeweils `--locked`. SQLx-Makros werden beim
funktionalen Build **online gegen das echte Testschema** validiert.
Formatabweichungen des Ausgangsstands wurden rein mechanisch mit dem gepinnten
rustfmt normalisiert. Interne Path-Abhängigkeiten haben passende Versionsangaben;
unveröffentlichte Workspace-Crates sind ausdrücklich `publish = false`.

Die ursprünglichen acht selbständigen PostgreSQL-Roundtrips sind jetzt reguläre
Tests. `scripts/ci/required-db-tests.txt` führt sie einzeln auf. Sie verlangen
`BRAIN_TEST_DATABASE_URL`, einen Loopback-Host und einen Datenbanknamen mit
`ci` oder `test` als Unterstrich-separiertem Bestandteil. Fehlende, unbenannte,
entfernte oder nicht erreichbare Infrastruktur führt zu einem Fehler, nicht zu
`return` mit grünem Teststatus. Die Fixture-Pools lesen keine Produktions-DSN.
Sie prüfen Build-Upsert, Dokument-/Snapshot-Idempotenz, Domain-Entity-Upsert,
Patch-Deduplizierung, YouTube-Queue, Transcript-Speicherung und Claim-Verarbeitung.

Die verbleibenden 52 historisch ignorierten Tests stehen **mit exakten Namen**
in `scripts/ci/expected-ignored-tests.txt`. Darunter sind Live-/Korpus-Paritäten,
Referenzbuilds, reale Snapshot-Tests und noch nicht in diese Basis überführte
Reasoner-/Cross-Schema-Abnahmen. Nicht jeder dieser Tests benötigt technisch
Produktionsdaten; die nicht aktivierten Abnahmen sind eine bekannte
Abdeckungsgrenze, kein bestandener Testnachweis. Keine neue Ausnahme wird
automatisch aufgenommen. Das Libtest-Ausgabeprüfprogramm verweigert einen leeren
Lauf, einen internen Skip, veränderte Ignore-Inventare und fehlende Pflicht-DB-Tests.
Owner dieser Übergangsliste: Repository-Maintainer EarlySalty; Review bis
2026-10-24. Erweiterungen benötigen begründete, einzelne Änderungen im PR.

Python nutzt die vorhandenen pytest-kompatiblen Tests, ergänzt um Ruff-
Korrektheitsregeln (`E9`, `F63`, `F7`, `F82`) und Syntaxkompilierung. Ein vorher
nicht vorhandener Mypy-Lauf wird nicht behauptet. `mcp/conftest.py` lehnt null
Tests und jegliche pytest-Skips ab. Der Import des MCP-Servers wird im Test so
isoliert, dass der reale Secret-Loader nicht aufgerufen wird. Der alte
Live-DB-Test wurde durch synthetische Daten auf dem echten Schema und derselben
View wie im Rust-Laufzeitpfad ersetzt. SQL-Parameter, Sortierung, Werte,
Schreibschutz, eine SQL-Injection-Gegenprobe und Infrastrukturfehler werden
geprüft. Zusätzliche Tests decken die gehärtete XML-Verarbeitung und die
Ablehnung von Datei-/FTP-URLs vor I/O ab. Kein Whisper-Modell, Browser-Login,
LLM-Service oder Produktionsdatensatz wird für diese Tests verwendet.

Jeder funktionale GitHub-Job hat eine eigene PostgreSQL-16-Testinstanz. Die
verwendeten Credentials sind absichtlich öffentliche Wegwerfwerte und keine
Secrets. Schemaherkunft, Prüfsummen, Guards und Consumer-Aufruf stehen in
[`schema/README.md`](schema/README.md). Alle produktiv angewandten Migrationen
bleiben unverändert. Der optionale MCP-Einstieg ergänzt das echte
`patchnotes.changelog_posts`-DDL und die per `include_str!` geteilte Runtime-View.

## Security-Prüfungen und begrenzte Ausnahmen

* Gitleaks untersucht die vollständige erreichbare Git-Historie, mit Redaction.
  Es gibt keine Gitleaks-Baseline oder pauschale Pfadfreigabe.
* Semgrep führt sechs versionierte lokale Regeln für Python und Rust aus:
  Shell-Aufrufe, dynamische Python-Auswertung, unsichere Deserialisierung und
  abgeschaltete TLS-Prüfung. Das ist eine gezielte Regelbasis, kein vollständiger
  Taint-Scan sämtlicher Sprachkonstrukte. Beide Sprachen müssen tatsächlich
  gescannte Dateien liefern; Parser-/Scanfehler sind blockierend. Bandit und
  CodeQL liefern ergänzende Python-/Rust-Abdeckung.
* Bandit blockiert Python-Befunde ab MEDIUM. Die einzelnen `nosec B608` sind
  geprüfte Fehlalarme bei festen SQL-Fragmenten, intern vorgegebenen Tabellen
  oder korrekt zitierten Identifiers mit separat gebundenen Werten. Sie stehen
  direkt am betreffenden Ausdruck in MCP, CLI, Lineage, Retrieval, Sheet-Tabs,
  Timeline und YouTube-Learning. Es gibt keinen globalen B608-Ausschluss.
  `nosec B310` ist nur an vier URL-Öffnungen gesetzt, deren ursprüngliches
  HTTP(S)-Scheme unmittelbar davor validiert wird. Das ist KEINE vollständige
  SSRF-Abwehr. XML-Feeds verwenden statt einer Ausnahme `defusedxml`.
  Owner: EarlySalty; Review dieser Stellen bis 2026-10-24. Änderungen an
  SQL-Struktur, Wertebindung oder URL-Prüfung verlangen erneute Review.
* `pip-audit` prüft den vollständigen gehashten Runtime-Lock einschließlich
  optionaler MCP- und Browser-Worker-Abhängigkeiten. Die größeren Scanner-
  Abhängigkeiten sind separat gelockt. Trivy untersucht das Repository auf
  Schwachstellen, Secrets und Fehlkonfigurationen mit HIGH/CRITICAL-Schwelle.
  Die konventionell benannte `requirements/requirements.txt` stellt die
  Python-Erkennung sicher. Build-Artefakte unter `rust/target` sind ausgeschlossen.
* Cargo-Audit prüft den gesamten Lockfile einschließlich optionaler Pakete;
  Unsound-/Yanked-Meldungen sind ebenfalls blockierend. Cargo-Deny prüft den
  aktiven Graphen auf Advisories, Lizenzen, Quellen und Wildcards. Interne,
  ausdrücklich unveröffentlichte Crates bekommen keine erfundene Lizenz.
  Mehrfachversionen sind dokumentierte Warnungen; unmaintained ist für direkte
  Workspace-Abhängigkeiten blockierend, transitive Meldungen bleiben sichtbar.
  Es gibt keine freigeschalteten Cargo-Advisory-IDs.

Stand beim Aufbau: rustls, rustls-webpki, anyhow, event-listener, chacha20 und
spin wurden auf passende reparierte Versionen aktualisiert. Der Lockfile führt
weiter `rsa 0.9.10` über die optionale SQLx-MySQL-Abhängigkeit. `cargo tree
--target all -i rsa` zeigt keine aktive Verwendung im PostgreSQL-Build,
Cargo-Audit meldet dennoch `RUSTSEC-2023-0071`. **Dieser Befund bleibt
blockierend**, bis die Dependency-Situation behoben oder eine gesonderte,
nachvollziehbare Policy-Entscheidung getroffen ist. Ein grüner Gesamtstatus
wird hierfür nicht vorgetäuscht.

## CodeQL und Reporting

Rust und Python werden getrennt mit `security-extended` analysiert. Rusts
unterstützter `build-mode: none` nutzt rust-analyzer, Build-Skripte und
Makroverarbeitung; er ist kein vollständiger Cargo-Build. Bedingte Kompilierung,
Makros, generierter Code und SQLx-Offlinedaten können die Extraktion begrenzen.
Python wird ohne fiktiven Build-Schritt extrahiert. Shell, SQL und Markdown
werden nicht als eigene CodeQL-Sprachen behauptet. Actionlint/Zizmor sichern
zusätzlich die Workflows.

Nach der Analyse muss das CodeQL-Quellarchiv tatsächlich eigene Rust-/Python-
Quelldateien enthalten. Ein leeres Archiv, leere Regelmenge, eine fehlende SARIF-
Datei, ein Extraktionsfehler oder ein SARIF-Sicherheitsbefund blockiert. Eine
Analyse-Aktion ohne nachgeschaltete Ergebnisprüfung wäre kein Findings-Gate.
Die extrahierte Dateizahl wird im Job protokolliert; sie beweist nicht die
vollständige semantische Analyse jeder Zeile.

SARIF und andere Diagnoseberichte werden als Actions-Artefakte aufbewahrt.
Code-Scanning-Uploads und DB-Uploads sind abgeschaltet, daher benötigt kein
PR-Job `security-events: write`. Nur Diagnose-Uploads dürfen separat fehlschlagen;
alle Scanner und Ergebnisprüfungen bleiben blockierend.

## Gegenproben und Wartung

`policy-selftest.sh` prüft erfolgreiche, fehlerhafte, abgebrochene, übersprungene,
fehlende und zusätzliche Jobs sowie leere/fehlerhafte SARIF-Berichte.
Das Rust-Logprüfprogramm hat sieben positive/negative Selbsttests. Die
funktionalen Jobs testen fehlende DB-Konfiguration ausdrücklich negativ.
`scanner-selftest.sh` erstellt ausschließlich temporäre, nicht ausgeführte
Scanner-Fixtures für Python/Rust, einen synthetischen Secret-String und eine
bekannt verwundbare Dependency-Version. Jeder Scanner muss dafür genau seinen
Findings-Exitstatus liefern; ein Download-/Programmfehler gilt nicht als Beweis.

Locks erneuern (Resolver-Version im lokalen Prüfbericht festhalten):

```sh
uv pip compile pyproject.toml --extra mcp --extra browser --python-version 3.12 \
  --generate-hashes -o requirements/requirements.txt
uv pip compile pyproject.toml requirements/test.in --extra mcp --extra browser \
  --python-version 3.12 --constraint requirements/requirements.txt \
  --generate-hashes -o requirements/ci.txt
uv pip compile requirements/security.in --python-version 3.12 \
  --generate-hashes -o requirements/security.txt
```

Dependabot ist für GitHub Actions, Cargo und die tatsächlich vorhandenen Pip-
Manifeste eingerichtet. Es gibt keine Node-/npm- oder Dockerfile-Toolchain.
