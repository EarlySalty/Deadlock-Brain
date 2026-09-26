# CODEX Consumer / CI Completion — Übergabe

Stand: 2026-09-25. **Remote-Bausteine und PRs vorbereitet; kein vollständiger Consumer-Cutover und keine Produktionsfreigabe.**

CODEX A wurde als Referenz verwendet: `codex/core-completion-20260925`, Commit `64cbf3b0f59847d607ae97800a550ac0d7f8319f`, [PR #38](https://github.com/EarlySalty/Deadlock-Brain/pull/38). Dieser Handoff behauptet nicht, dass CODEX A abgeschlossen oder nach `main` gemergt sei. Der Brain-PR bleibt auf dessen Branch gestapelt. Alle unten genannten PRs wurden als Draft angelegt und bleiben ungemergt.

## 1. Vollständige Repository-/Branch-/Commit-/PR-Liste

| Repository | Branch | Implementierungs-Commit | PR | Benötigte lokale Prüfung |
| --- | --- | --- | --- | --- |
| EarlySalty/Deadlock-Brain | `codex/consumer-ci-completion-20260925` | `dd375291bd0517a106b00cd95913f58d05597943` | [#39](https://github.com/EarlySalty/Deadlock-Brain/pull/39), Basis CODEX A | Echte Auth-/ACL-/Release-Konfiguration, Postgres-Migration und Rollen-/Cache-Isolation; spätere Zusammensetzung mit Wiki/Sources/Replay auf dem tatsächlich kombinierten Commit erneut testen. Kein produktiver SLO aus Fixtures ableiten. |
| EarlySalty/Deadlock-Twitch-Bot | `codex/consumer-ci-completion-20260925` | `2c2dcb5ebe171aa8789718086a691254ed3e201c` | [#983](https://github.com/EarlySalty/Deadlock-Twitch-Bot/pull/983), Basis main | Öffentlichen Token/Scope und vertrauenswürdige Request-/Conversation-Bindung einsetzen; stateless Self-Explainer isoliert verdrahten; Auth, Rate-Limits, 400-Zeichen-Ausgabeaufteilung, Quellenlabels und Timeouts prüfen. Bekannten Seed-Altfehler bearbeiten. Historie/Dashboard-Karten benötigen vorher Vertrags-/Codearbeit. |
| EarlySalty/Deadlock-Bots | `codex/consumer-ci-completion-20260925` | `af0f6c4ad65006450234d155d701edf15b50d89f` | [#457](https://github.com/EarlySalty/Deadlock-Bots/pull/457), Basis main | `AiAnswerer` bewusst im Composition Root verbinden; separate Tokens/Scopes und eindeutige opake Namespaces; Allowlist, Command-Usage, Cooldowns, Queue-Fristen und Discord-Chunking isoliert prüfen. FAQ/Concierge/OpenTest/Build-Publishing benötigen vorher zusätzliche Vertrags-/Codearbeit. |
| EarlySalty/Deadlock-Docs | `codex/consumer-ci-completion-20260925` | `f8841624064fa57a4f502bad8795e6afc9aeb0cd` | [#3](https://github.com/EarlySalty/Deadlock-Docs/pull/3), Basis main | Vorgesehenen Operator-/Tool-Aufrufer verbinden; Token-Prinzipal, `docs.public`, öffentliche Corpus-Freigabe und konkrete Wissensrelease prüfen. Interne Dokumente sind durch diesen Adapter nicht freigegeben/importiert. |
| EarlySalty/Deadlock-2nd-Brain | `codex/consumer-ci-completion-20260925` | `c01b9356e3c45168f11c3b7dd9ae9ec547490854` | [#1](https://github.com/EarlySalty/Deadlock-2nd-Brain/pull/1), Basis main | Vertrauenswürdige interne Scope-Bindung und Operator-Sitzung, Token-ACL, Wissensrelease, Rollenwiderruf und serverseitigen Modell-Egress prüfen. Hosted-CI-Ausführungsfreigabe kontoseitig prüfen; keine Kontoeinstellungen wurden verändert. |

Die Brain-Zeile nennt den erfolgreich in CI geprüften Code-/Workflow-Commit. Diese Handoff-Datei folgt als eigener Dokumentationscommit; ihren genauen Commit liefert `git log -1 --format=%H -- architecture/migration/handoffs/CODEX_CONSUMER_CI_COMPLETION.md`. Die Consumer pinnen bewusst den unveränderlichen Client-Commit `bdcc6dec3424bd313d36e5f545de2a07df564c7f`, nicht einen beweglichen Branch oder den späteren Dokumentationscommit.

Die Twitch-Arbeit und Baseline-Gegenprobe basieren auf `8c800bb9`. Bei PR-Erstellung war dessen `main` bereits auf `93b223c5ee7df2963990ea90a6afb31822afdfcd` weitergelaufen. Es wurde weder automatisch gemergt noch rebased oder force-gepusht; CI-Provenance und lokalen Ausgangsstand deshalb nicht gleichsetzen.

## 2. Inventar und tatsächliche Consumer-Abdeckung

Quellen: `architecture/migration/inventory/CODE_RUNTIME_INVENTAR.md`, `DATENFLUSS.md` und die tatsächlich zugänglichen Repository-Quellen. Die fünf oben genannten Repositories sind die in S01 belegten Arbeitsbereiche. Es wurden keine zusätzlichen Consumer allein aus ähnlich klingenden Repository-Namen erfunden.

Twitch und Deadlock-Bots haben Antwortpfade. Docs und Second Brain sind Wissensbestände, keine autonomen Antwortbots; sie erhalten daher ausdrücklich aufrufbare Rust-Query-Adapter statt neuer Hintergrunddienste. Feeder/Importer, Outbox und Publishing sind nicht als neue Antwort-Consumer umgedeutet worden.

### Implementiert, aber nicht aktiviert

- Brain: gemeinsamer synchroner und nativer asynchroner Client, kanonische Typen, Loopback-Konstruktor für interne/public Bot-Consumer, gemeinsame Wire-Validierung.
- Twitch: `tb_knowledge::brain::BrainKnowledgeAdapter` und `self_explainer::answer_stateless_via_brain`; bestehende Frage-/Antwortaufbereitung wird wiederverwendet, keine aktive Route verändert.
- Bots: `dl_brain::brain_api::BrainApiAnswerer` am bestehenden `AiAnswerer`-Port; der unveränderte Dispatcher bleibt Eigentümer von Command-Checks/Cooldowns/Rendering.
- Docs/Second Brain: `tools/brain-adapter/`, jeweils `prepare` ohne Netz/Token und ausdrücklich aufgerufenes `answer`; begrenztes stdin-JSON, typisierte Ausgabe, keine Corpus-Dateien eingelesen oder veröffentlicht.

Alle vier Consumer verwenden den gepinnten gemeinsamen Brain-Client. Der Adapter enthält keinen Provider-/Modellaufruf, keine lokale RAG-Suche und keinen automatischen Fallback auf den alten Pfad. Bearer-Authentifizierung und echte Dokumentberechtigung bleiben serverseitig; eine Scope-Angabe ist keine Autorität.

## 3. Vertragslücken — nicht als reine Runtime-Aufgabe ausgeben

Der öffentliche `Query` trägt Request-ID, Conversation-ID, Text, Scopes, Profil, Patch und Modus. Die öffentliche Antwort liefert Status, Text, Wissensrelease und opake Quellenlabels. Daraus lassen sich folgende bestehende Funktionen nicht verlustfrei ableiten:

| Bestehende Funktion | Noch erforderlich |
| --- | --- |
| Historie und persönliche Dashboard-Karten | Typisierte, sicher eingeordnete Kontextfelder und zugehörige Server-/Policy-/Regressionstests. Der neue stateless Twitch-Port weist solche Eingaben ausdrücklich zurück. |
| Sprache/Persona und FAQ-/Concierge-/Patenaktionen | Erhalt ihrer spezifischen Semantik im kanonischen Vertrag. Fehlende Intent-/Aktionsfelder dürfen nicht geraten werden. |
| Source-Kind und echte Quellen-URLs | Erweiterung des öffentlichen Projektionsvertrags oder bewusste Anpassung der Consumer-Oberfläche; Labels sind keine erfundenen Links. |
| Build-/Publishing- und OpenTest-Spezialpfade | Eigener freigegebener Vertrag/Port; vorhandene Aktionen bleiben unangetastet. |
| `dl-knowledge` Retrieval-Port | Keine fertige Antwort als angeblich rohe Retrieval-Evidenz an ein zweites Modell weiterreichen. |

Deshalb ist das Ziel „Claude muss nur noch alle Consumer verdrahten“ **noch nicht für sämtliche Modi erreicht**. Die unterstützten stateless/Command-/Operator-Ports sind vorbereitet; die Tabelle benennt die verbleibende Codearbeit. Bestehende Featureparität bleibt durch unveränderte bisherige Runtime-Pfade erhalten, nicht durch eine behauptete vollständige Adapter-Abdeckung.

## 4. CI-Abdeckung und Herkunft

Workflow: `.github/workflows/rust-core-verification.yml`. Rust 1.97.1, versionierte Lockfiles, kontrollierte Abhängigkeitsbeschaffung, danach Offline-Tests, bereinigte App-Umgebung, eigene Test-HOMEs, hosted Runner. Keine Produktions-Secrets oder Self-hosted-Produktivrunner.

| Bereich | Codebasis / Beleg |
| --- | --- |
| Core | Aktueller PR-Checkout: Contracts, Policy, bestehender Core |
| Storage | Aktueller PR-Checkout plus eigener echter PostgreSQL-Testcluster, Unix-Socket, keine Wiederverwendung/Änderung eines vorhandenen Clusters |
| Ingestion | Aktueller PR-Checkout |
| Retrieval | Aktueller PR-Checkout: Retrieval und Reasoner |
| Provider/Jev | Aktueller PR-Checkout plus Fixture-Prüfsummen |
| Kernel/API/Client | Aktueller PR-Checkout plus Client-Lint und synthetischer Release-Messlauf |
| Quality Tooling | PR #27, `5676167e433c081b77e1074cd96478df46bca530`; `architecture/migration/evals/` in den aktuellen PR übernommen und dort ausgeführt |
| Cutover Audit Tooling | PR #26, `e2377f766a00f1fbe1799a8ae877adb3bd91ecb0`; `infra/cutover/runtime-audit/` im aktuellen PR ausgeführt |
| Wiki | Separater gepinnter Referenz-Checkout PR #35: `f7fafa5f6bd984df0f94c8117b0c17e2a2a08d27` |
| Sources | Separater gepinnter Referenz-Checkout PR #36: `b44f2fe4f416cc1e2c8c56323fe9e5b2c74415fb` |
| Replay Audit/Decoder | Separater gepinnter Referenz-Checkout PR #37: `f0b5bf15b9c16d75dcc2df5e7051f97901f5742e`; synthetischer Decoder und ausdrücklich blockierter Echtkorpus |

Die Referenz-Suites sind keine stillen Merges und keine Abnahme ihrer Zusammensetzung mit CODEX A. Historische S10-/S11-Berichte sind als Herkunft übernommen, nicht als aktuelle Runtime-Werte ausgegeben. Ausführliche Provenance: `scripts/ci/CONSUMER_OFFLINE.md`.

`Consumer Offline Gate` fordert Erfolg aller drei Matrizen und genau elf nichtleere, nicht abgelaufene Artefakte des aktuellen Run-Attempts. Fehlende/übersprungene/abgebrochene Suites sind kein Erfolg. Teil-Reruns müssen durch einen vollständigen Rerun ersetzt werden, um die Attempt-Bindung einzuhalten. Upload ausschließlich erlaubter `.log`-/`.txt`-/`.tsv`-Berichte, keine Test-HOMEs. Der zunächst fehlgeschlagene Upload versteckter Dateien wurde in `dd375291bd0517a106b00cd95913f58d05597943` korrigiert; Fehler wurden nicht ignoriert.

### Verifizierte GitHub-Runs

| Repository / Prüfung | Run | Abgelesener Zustand |
| --- | --- | --- |
| Brain, alle elf Suites plus Gate, Code `dd375291...` | [36180952681](https://github.com/EarlySalty/Deadlock-Brain/actions/runs/36180952681) | erfolgreich abgeschlossen |
| Docs, neuer Adapter, `f884162...` | [36181636667](https://github.com/EarlySalty/Deadlock-Docs/actions/runs/36181636667) | erfolgreich abgeschlossen |
| Bots, neuer Adapter, `af0f6c4a...` | [36181937960](https://github.com/EarlySalty/Deadlock-Bots/actions/runs/36181937960) | erfolgreich abgeschlossen; weitere vorhandene Repository-Workflows sind davon getrennt |
| Second Brain, neuer Adapter, `c01b935...` | [36181724279](https://github.com/EarlySalty/Deadlock-2nd-Brain/actions/runs/36181724279) | Runner kontoseitig nicht gestartet; kein ausgeführter Testfehler. Identischer Offline-Einstieg remote bestanden, Hosted-CI-Erfolg nicht behauptet. |
| Twitch, neuer Adapter, `2c2dcb5e...` | PR #983 / Actions | Bei Erstellung dieses Handoffs noch keine abgeschlossene Hosted-Gesamtprüfung bestätigt. Lokale Remote-Worktree-Ergebnisse und Altfehler siehe unten. |

## 5. Tatsächlich ausgeführte gezielte Prüfungen

| Suite | Ergebnis im isolierten Remote-Worktree |
| --- | --- |
| Brain-Client | 14 Tests bestanden, 0 ignoriert; Formatprüfung und Clippy `-D warnings` bestanden |
| PR #27 Quality Tooling | 85 Tests bestanden, 0 ignoriert; Formatprüfung/Clippy bestanden |
| Cutover Audit Tooling | 54 Tests bestanden, 0 ignoriert; Formatprüfung/Clippy bestanden |
| Bots `dl-brain` | 10 Tests bestanden, 0 ignoriert; Formatprüfung/Clippy bestanden |
| Docs Adapter | 10 Tests: 5 Bibliothek + 5 CLI; alle bestanden, 0 ignoriert; Formatprüfung/Clippy bestanden |
| Second Brain Adapter | 11 Tests: 6 Bibliothek + 5 CLI; alle bestanden, 0 ignoriert; Formatprüfung/Clippy bestanden |
| Twitch `tb-knowledge` | 23 Unit + 3 Load + 4 Seed bestanden; 1 Seed fehlgeschlagen, 0 ignoriert; Formatprüfung/Clippy bestanden |
| Twitch Self-Explainer | 19 gezielte Tests bestanden, 0 ignoriert, übrige 1250 Dashboard-Tests bei diesem Aufruf gefiltert; Formatprüfung bestanden |

Twitch-Altfehler: `stoerung_stream_info_felder_findet_uplink_stoerungen` in `rust/crates/tb-knowledge/tests/seed.rs`. Identische Gegenprobe im unveränderten separaten Baseline-Worktree `8c800bb9`: 4 Seed-Tests bestanden / derselbe eine fehlgeschlagen. Selector, Corpus und Seed-Test wurden für den Adapter nicht verändert. Die CI lässt den Test aktiv und rot; kein Herausfiltern oder `continue-on-error`.

### Security-/Performance-Nachweis und Grenzen

Neue Client-Tests prüfen unsichere Endpunkte, Token-/URL-Diagnostik, Auth-/Wire-Vertrag, Unicode, Scope-/Authority-Felder, falsche Request-ID/Version, ungültige Belege, nicht-JSON, Redirect-Abfluss, deklarierte und chunked Übergrößen, Serialisierungs-Übergröße sowie echte Fixture-Timeouts. Consumer-Tests prüfen explizite Scope-Bindung, CLI-End-to-End gegen Loopback, unveränderte Dispatcher-Cooldowns/Usage und fehlende stillschweigende Kontext-/Backend-Fallbacks.

Die Performancemessung dekodiert/validiert 4096 selbst verfasste JSON-Fixtures im Release-Profil und berichtet Iterationen, Bytes und gemessene Zeit. Sie enthält **keine** Produktionsinferenz, keinen echten Nutzertraffic und keine Netzwerklatenz. `production_slo` bleibt `null`; kein erfundener Grenzwert oder lokaler Wert wird als Produktions-SLO ausgegeben.

Zusätzliche bestehende Scannerhinweise bleiben offen: Beim Bots-Push meldete GitHub 9 Befunde des Default-Branches. Beim unverändert ausgeführten Twitch-Push-Hook meldete OSV Befunde an drei bestehenden Python-Abhängigkeiten; Gitleaks/RustSec/Cargo-Deny bestanden mit Hinweisen, Trivy meldete keine Treffer. Die Werkzeuge haben unterschiedliche Prüfbereiche; daraus folgt keine vollständige Dependency-Sicherheitsfreigabe. Der Hook wurde nicht deaktiviert oder umgangen. Diese Änderung aktualisiert keine unbeteiligten Runtime-Abhängigkeiten auf Verdacht.

## 6. Reproduktionsbefehle ohne Runtime-Aktivierung

Brain:

```sh
cargo fetch --manifest-path rust/Cargo.toml --locked
cargo fetch --manifest-path architecture/migration/evals/Cargo.toml --locked
cargo fetch --manifest-path infra/cutover/runtime-audit/Cargo.toml --locked
bash scripts/ci/check_consumer_offline.sh client
bash scripts/ci/check_consumer_offline.sh quality
bash scripts/ci/check_consumer_offline.sh cutover
```

Bots:

```sh
cargo fetch --manifest-path rust/Cargo.toml --locked
bash rust/scripts/check-brain-consumer.sh
```

Twitch:

```sh
cargo fetch --manifest-path rust/Cargo.toml --locked
bash rust/scripts/check-brain-consumer.sh knowledge
bash rust/scripts/check-brain-consumer.sh self-explainer
```

Docs und Second Brain jeweils im eigenen Repository:

```sh
cargo fetch --manifest-path tools/brain-adapter/Cargo.toml --locked
bash tools/brain-adapter/check.sh
```

Detailanleitungen: Twitch `rust/crates/tb-knowledge/BRAIN_API_ADAPTER.md`, Bots `rust/crates/dl-brain/BRAIN_API_ADAPTER.md`, Docs/Second jeweils `tools/brain-adapter/README.md`.

## 7. Nicht ausgeführte Aktionen / Freigabegrenze

Keine Änderungen an Produktionskonfiguration oder laufenden Services; keine Bot-Neustarts, keine Community-Nachrichten, kein Deployment, kein Merge, kein Auto-Merge und kein Force-Push. Fremde Änderungen in bestehenden Haupt-Checkouts wurden nicht verändert, bereinigt oder gestasht. Alle Arbeiten fanden in separaten eigenen Branches/Worktrees statt.

Die Consumer sind vorbereitet, aber nicht automatisch aktiviert. Das Bots-Release-Gate ist **als Branch-Änderung** auf read-only/report-only umgebaut; die noch unveränderte Main-Konfiguration wird dadurch nicht als bereits umgestellt ausgegeben. Für diese Draft-PRs wurde keine Merge-/Releasefreigabe erteilt.

Vor einer späteren Aktivierung sind die konkret benannten Vertragslücken, der Twitch-Altfehler, die unabhängigen Scannerbefunde und alle Zeilen „benötigte lokale Prüfung“ abzuarbeiten. Produktive Akzeptanz- oder Performancewerte müssen separat an der echten freigegebenen Runtime erhoben werden.
