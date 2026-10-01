# Reproduzierbare Consumer-/Offline-Prüfung

Stand: 2026-09-25. Keine Produktionsfreigabe, kein Merge, keine Deployment-Aktion.

## Herkunft und Prüfumfang

| Suite | Geprüfter Code | Prüfart |
| --- | --- | --- |
| Core | aktueller PR-Checkout, auf CODEX A `64cbf3b0f59847d607ae97800a550ac0d7f8319f` aufgebaut | Rust-Tests, Contracts, Policy, bestehender Core |
| Storage | aktueller PR-Checkout | Unit-/Integrationstests plus eigener PostgreSQL-Cluster, nur Unix-Socket |
| Ingestion | aktueller PR-Checkout | Adapter-, Batch- und Fehlerpfadtests |
| Retrieval | aktueller PR-Checkout | Retrieval-/Reasoner-Regressionen |
| Provider/Jev | aktueller PR-Checkout | Fixture-Tests und SHA256SUMS |
| Kernel/API/Client | aktueller PR-Checkout | Kernel-/API-/Client-Tests, Client-Clippy, synthetischer Release-Messlauf |
| Quality Tooling | PR #27, Quellübernahme `5676167e433c081b77e1074cd96478df46bca530` | eigenständiges `architecture/migration/evals`, im aktuellen PR getestet |
| Cutover Audit Tooling | PR #26, Quellübernahme `e2377f766a00f1fbe1799a8ae877adb3bd91ecb0` | eigenständiges `infra/cutover/runtime-audit`, im aktuellen PR getestet |
| Wiki | PR #35, Referenz `f7fafa5f6bd984df0f94c8117b0c17e2a2a08d27` | fest gepinnter eigener Checkout, bestehende vollständige Offline-Suite |
| Sources | PR #36, Referenz `b44f2fe4f416cc1e2c8c56323fe9e5b2c74415fb` | fest gepinnter eigener Checkout, Sources-/Core-Tests |
| Replay Audit/Decoder | PR #37, Referenz `f0b5bf15b9c16d75dcc2df5e7051f97901f5742e` | fest gepinnter eigener Checkout, synthetischer Decoder und explizit blockierter Echtkorpus |

Wiki/Sources/Replay werden **nicht** in CODEX A hineingemergt. Ihre grüne Referenzprüfung beweist nicht die spätere Zusammensetzung mit CODEX A. Nach einer autorisierten Zusammenführung müssen dieselben Prüfungen auf dem zusammengesetzten Commit laufen. Die Referenzjobs heißen deshalb ausdrücklich „Pinned reference suite“.

Die mit PR #27/#26 übernommenen historischen Berichtdateien bleiben Herkunftsbelege ihres ursprünglichen Commits. Sie werden nicht als neu ausgeführte oder aktuelle Runtime-Prüfung ausgegeben. Aktuelle Ergebnisse liegen in CI-Artefakten mit SHA und Run-Attempt; manuell reproduzierte Ergebnisse unter `.consumer-ci-reports/`.

## Ausführen

Rust 1.97.1 mit rustfmt/clippy verwenden. Abhängigkeiten zuerst ausschließlich aus den versionierten Lockfiles beziehen:

```sh
cargo fetch --manifest-path rust/Cargo.toml --locked
cargo fetch --manifest-path architecture/migration/evals/Cargo.toml --locked
cargo fetch --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked
bash scripts/ci/check_consumer_offline.sh client
bash scripts/ci/check_consumer_offline.sh quality
bash scripts/ci/check_consumer_offline.sh cutover
```

Die Wrapper entfernen die geerbte Anwendungsumgebung, verwenden ein eigenes Test-HOME und testen `--locked --offline`. Die CI installiert keine Services der Anwendung, erhält keine Produktions-Secrets und nutzt keine Self-hosted-Runner. Einzige PostgreSQL-Instanz ist der eigene Testcluster; bestehende Cluster werden weder wiederverwendet noch gestoppt.

`Consumer Offline Gate` verlangt alle drei Job-Matrizen und genau elf nichtleere Artefakte des aktuellen Workflow-Versuchs. Fehlende, übersprungene, fehlgeschlagene oder abgebrochene Suites sind kein Erfolg. Wegen der Run-Attempt-Bindung muss ein erneuter Gesamtbericht mit **Re-run all jobs** erzeugt werden, nicht nur mit einem Teil-Rerun.

## Messwerte und Freigaben

Der Client-Messlauf dekodiert/validiert 4096 identische, selbst verfasste JSON-Fixtures. Er meldet Bytezahl, Profil, Iterationen und beobachtete Zeit. Er enthält kein Netzwerk, keinen Produktionskorpus und keine echte Modellinferenz. `production_slo` bleibt `null`; es gibt keinen erfundenen Performance-Schwellwert.

Release-/Merge-Gates sind kein Ersatz für Review. Dieser Workflow erstellt nur Berichte und Testartefakte. Kein Auto-Merge, kein Deployment, keine Nachrichten, kein produktiver Service-Neustart.
