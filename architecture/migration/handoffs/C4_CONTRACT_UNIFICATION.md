# C4 — Gemeinsame versionierte IR-Verträge

## Auftrag und Basis

Ausschließlich C4 aus `architecture/migration/INTEGRATION_REVIEW.md`.
Basis: `087c522deda58ecf4bd6843167f51c54f681e944` (`origin/migration/rust-integration` bei Arbeitsbeginn).
Branch: `codex/fix-c4-contract-unification`. Ziel des PR: `migration/rust-integration`.

Eigener Worktree; fremde Änderungen im Hauptcheckout bleiben unangetastet. Kein Merge, Deployment, Dienstneustart, Quellenabruf, Produktions-DB-Zugriff oder Secret-Abruf gehört zu dieser Änderung. C5/C6 werden nicht implementiert. `STATUS.md`, `GATES.csv`, bestehende Freigaben und historische Messergebnisse werden nicht auf grün gesetzt.

## Eine Contract-Crate, tatsächliche Consumer

`brain-contracts` bleibt der einzige Besitzer gemeinsamer Datenverträge. Neu sind dort die Module `value`, `source`, `wiki`, `external` und `replay`.

| Grenze | Kanonischer Vertrag / tatsächliche Verwendung |
|---|---|
| Gemeinsamer Ursprung | `source::{SourceIdentity, SourceRevision, OriginArtifact, SourcePolicy, GameValidity, Versioned}` |
| Missingness | `value::{Observed<T>, UnknownReason}`; keine numerische Default-Implementierung |
| Wiki | `wiki::{WikiIr, IrField, IrValue, SourceLocator, Alias, Dependency, Unit}`; der echte Extraktor hält dieses DTO, nicht mehr eigene semantische Feldtypen |
| Externe Quellen | `external::{ExternalSourceIr, Provenance, Validation, FieldProvenance, FieldState}`; `dbrain-sources::external::SourceIr` hält Raw-Bytes plus dieses DTO |
| Replay | `replay::{ReplayArtifact, ReplayReport, ReplayObservation, ReplayTime, ObservationProvenance, ReplayDecoder, ObservationStore}`; Worker, Supervisor und Orchestrierung verwenden diese Typen |
| Storage | Unveränderter `SourceRecordV2` / `DocumentStorePort`; versionierter Ursprung in `metadata["brain.origin"]` über `OriginArtifact::bind_record` und `origin_from_record` |
| Domain / Kernel | Bestehende `domain::{DomainStorePort, NumericFact, TypedRule, RuleEvaluatorPort}` und `CorpusRelease` bleiben; zusätzliche Quelldetails sind über gemeinsame IR-Typen statt über Adapter-private Typen zugänglich |

Die lokalen Wiki-/Source-Wrapper behalten ausschließlich vertrauensgebundene Konstruktion, Raw-Bytes, Parseroperationen und Diagnostik. Sie lassen sich nicht aus beliebigem JSON zu einem freigegebenen Parser-/Projektionsobjekt deserialisieren. Ein deserialisiertes gemeinsames DTO ist weder eine Rechteentscheidung noch eine fachlich geprüfte Tatsache.

## Versionen und Kompatibilität

| Version | Bedeutung |
|---|---|
| `brain.v1`, `brain.public.v1` | Bestehende interne/öffentliche Antwort- und Clientverträge bleiben unverändert. |
| `brain.store.v2`, `brain.domain.v1` | Bestehende Store-/Domainverträge bleiben erhalten; keine SQL-Migration und kein zweiter Releasezeiger. |
| `brain.ir.v1` | Neuer expliziter Umschlag `Versioned<T>`. Fehlende/null/fremde Versionen und unbekannte Umschlagfelder werden abgelehnt. |
| `brain.replay.v2` | Gemeinsamer Replaybericht mit typisiertem Tick, expliziter GameValidity und Entity-Mapping. v1 wird nicht still als v2 gelesen. |

Die lokalen Re-Exports `dbrain_sources::external::{Provenance, SourceRevision, Validation}`, `dbrain_s12_wiki_probe::knowledge::{IrField, IrValue, SourceLocator, Unit, Alias}` und `deadlock_brain_core::replay::*` verweisen auf dieselben Contract-Typen. Es existiert dort keine zweite Typdefinition.

Die bisherigen kleinen `brain_contracts::{ReplayArtifact, ReplayObservation}` bleiben als `brain.v1`-Summary-Datentypen erhalten, damit bestehende Nutzer nicht still gebrochen werden. Sie sind ausdrücklich **nicht** der neue Decoder-/Storage-Eingang. Für ihre obligatorische bekannte Zeit gibt es keinen Unknown-zu-Null-Konverter.

Beim Wiki-CLI liegt die semantische Ausgabe nun unter `contract.data`, die Pflichtversion unter `contract.contract_version`; `report` bleibt separat diagnostisch. Der historische flache `wiki-ir-v1`-Dump war kein unterstützter vertrauenswürdiger Import. Bestehende Feld-ID-Berechnung, Mapping-Review-Pins, Leaf-Serialisierung und die geprüfte Karten-Golden bleiben erhalten. Die lokale Feld-ID-Domäne `wiki-ir-v1` ist nicht mit der neuen Wireversion oder mit einer Spielversion zu verwechseln.

Externe Quellen behalten die alten Metadatenfelder für Bestandsleser und schreiben zusätzlich denselben versionierten Contract in `metadata.contract`. Originalbytes bleiben im bestehenden Raw-Artefakt. Der Dokument-Derivationsschlüssel unterscheidet Originalrevision, Parserrevision, Schemafingerprint und eine tatsächlich bekannte Schema-/API-Version. Ohne bekannte Schema-Version bleibt seine bisherige Schlüsselrepräsentation erhalten.

## Wiki-IR

Die gemeinsame `WikiIr.source_id` bindet die Seiten-/Alias-Locators an einen konkreten Quellen-Namespace; eine MediaWiki-Seiten-ID allein ist keine globale Source-ID. `sources`, `artifacts` und `source_revision` pinnen Quellidentität, echte Seitenrevision und Raw-Hash. `OriginArtifact` hält Parserrevision/-familie, Zeitachsen mit benannten Einheiten, Sprache, Policy und Patch-/Mode-Gültigkeit getrennt.

Felder behalten Werte und Einheiten, Originalbedingungen, Varianten, Unknown-Gründe und sämtliche Locators. Aliase behalten ihre Sprache und den konkreten Seiten-/Revisions-/Hash-Locator. Abhängigkeiten tragen Quellidentität, bekannte Zielrevision oder explizites Unknown; unresolved Referenzen und unvollständige Abhängigkeiten gehen nicht verloren.

Bedingungen/Varianten bleiben bewusst **Quellenausdrücke**, keine durch den Parser ausführbare Regel-DSL. Die kompatible Feldrepräsentation ist:

| Repräsentation | Bedeutung |
|---|---|
| `condition/variant = Some(text)` | Tatsächlich extrahierter Quellenausdruck; keine nackte unbedingte Tatsache. |
| `None` mit `missing_condition` / `missing_variant` in `unknowns` | Mapping verlangte den Kontext, Extraktion fehlgeschlagen: Unknown, nicht wegoptimieren. |
| `None`, keine entsprechenden Unknowns | In diesem geprüften Mapping kein Ausdruck konfiguriert; das ist keine allgemeine Domain-Freigabe. |
| `IrValue::Unknown` oder weitere `unknowns` | Keine zulässige implizite numerische Ersatzgröße. |

`IrField::is_unconditional_known` bildet die bestehende Verlustschutzbedingung ab. C6 muss zusätzlich Mapping-/Source-Review, tatsächliche GameValidity und fachliche Verifikation prüfen. `project_card` behält seine bisherigen Ablehnungen kontextabhängiger oder unbekannter Felder; keine C6-Regelimplementierung wird vorgezogen.

## Externe Quellen

`SourceRevision::{Http, Git, Wiki, Api, Replay}` ist gemeinsam definiert. Bei HTTP bleiben Body-Hash, ETag und Last-Modified die beobachtete Originalrevision; sie werden nicht aus `info.version`, Schemahash oder Parserrevision erzeugt. Git behält den konkreten Commit. `Api` kann eine belegte API-Version und eine getrennte Originalrevision ausdrücken.

Die vorhandenen Assets-/Match-Adapter und das Schema-Gate reichen die bekannte `OpenApiSnapshot.api_version` separat weiter. Raw- und normalisierter Hash sind weiterhin verschieden. Ein Parserwechsel verändert die Ableitung, nicht die Originalrevision. Gemeinsame Ursprungsartefakte bleiben explizite Artefaktreferenzen statt fälschlich unabhängiger Quellenstimmen.

`field_provenance` verwendet RFC-6901-Pointer einschließlich korrekter `~`-/`/`-Escapes und bindet jedes beobachtete Feld an Originalrevision, Parser und Raw-Hash. Nicht konsumierte Zukunftsfelder bleiben im Raw-/Payload-Vertrag. `field(pointer)` unterscheidet fehlend, explizites JSON-null, Quarantäne und einen tatsächlich beobachteten Wert `0`.

`payload` verwendet absichtlich `Observed<Value>`, nicht `Option<Value>`: Ein tatsächlich empfangenes JSON-`null` muss einen Serde-Roundtrip überleben und darf nicht mit fehlender/parsingfehlgeschlagener Payload kollabieren. Quarantäne bleibt auch dann wirksam, wenn die Raw-Payload noch vorhanden ist; Feldzugriff liefert in diesem Fall `Unknown(Quarantined)`.

## Replay und die alte Tick-Sentinelrepräsentation

| Eingang | Gemeinsame Repräsentation |
|---|---|
| Quelltick `-1` | `Observed::Unknown { reason: InitializationTick }` |
| Echte Quellticks `0..=i32::MAX` | `Observed::Known { value: tick as u32 }` |
| Fehlender oder JSON-null-Tick eines archivierten v1-Berichts | Nur im expliziten Import: `Unknown(NotPresent)` |
| Andere negative Werte, Floats, Strings, Überlauf | Fehler; kein Ersatz durch `0`. |
| Fehlende/null-Zeitfelder im neuen v2-Vertrag | Serde-Fehler; der Producer muss Unknown ausdrücklich senden. |

`tick_from_legacy` ist die einzige Quellsentinel-Umsetzung im aktiven Decoder. `migrate_v1_report` ist ein expliziter **Archiv-Leseadapter**, kein automatischer Fallback und keine neue akzeptierte Worker-Generation. Er erhält Original-/Parserrevision, IDs, Rechte und Raw-Locators, füllt nur neue fehlende Semantik mit Unknown/leerem Mapping und lehnt injizierte v2-Felder in v1 ab. Die archivierte Generations-/Observation-ID wird dabei nicht als frisch neu berechneter v2-Nachweis ausgegeben. Für eine neue aktive Decoder-Generation ist ein regulärer Reparse erforderlich.

Game-Time und Tickinterval bleiben unabhängig beobachtet oder Unknown. Weder ein Tick noch eine Parser-/Schema-/Source-Version begründet automatisch Game-Time, Patch oder Mode. Der Decoder liefert unbekannte GameValidity und kein kanonisches Entity-Mapping; der Supervisor lehnt unerwartete vom Worker injizierte Zuordnungen/Gültigkeiten ab.

`ReplayReport::observation_provenance(id)` bindet eine tatsächlich vorhandene Observation an Replay-/Generationsidentität, den gemeinsamen Ursprung, Parser-/Schema-/Extraktionsrevision, Rechte und den vollständigen Raw-Locator einschließlich `requires_state_prefix`. Netzwerkindex plus CREATE-Ordinal ist nur der Mapping-Schlüssel, keine Hero-/Spieler-ID. Ohne belegtes Mapping bleibt `canonical_entity` ausdrücklich Unknown.

## Policy, ACL und Storage

`bind_record` verlangt passende Source-/Logical-ID, Raw-Hash, Store-Revision bei Wiki und exakt passende gespeicherte ACLs. Source-/Parserrevisionen werden nicht in eine erfundene numerische Store-Revision konvertiert: Git/API-Consumer müssen die bestehende monotone Store-Revision bereitstellen und die Originalrevision getrennt erhalten.

Neue Herkunftsmetadaten werden bei `SourceBatch::validate` geprüft. `record_allowed` lehnt beschädigte oder inkompatible Herkunftsmetadaten ab und respektiert `provider_egress_allowed=false` zusätzlich zu den bestehenden Principal-/Visibility-/Scope-Regeln. Datensätze ohne neuen Metadatenschlüssel behalten für Bestandskompatibilität die bestehenden Regeln; dieses Fehlen ist keine neue Rechtefreigabe.

`CorpusSnapshot::authorized` prüft weiterhin sowohl gepinnte Revision als auch aktuellen Head. Es vereinigt erforderliche Scopes, behält die strengere Sichtbarkeit und beschränkt gemeinsame Policy-Flags. Das ausgegebene Ursprungsartefakt trägt diese wirksamen Einschränkungen bei unveränderter ursprünglicher Revision. Ein alter Release darf widerrufene Rechte nicht wiederherstellen.

Publikationsfreigabe und fachliche Wahrheit sind keine Eigenschaften, die aus erfolgreicher Deserialisierung folgen. Unbekannte Lizenz-/Autorisierungsangaben bleiben Unknown, und nicht belegte Erlaubnisse bleiben gesperrt. Es wurde kein kanonischer Publish-/Fact-Writer neu gebaut. Reale DB-Roundtrips und Rechtefreigaben für den C5-Durchstich sind eine gesonderte Abnahme, nicht durch Memory-Tests ersetzt.

## S12 aus der Architekturablage verschoben

Aktiver Pfad: `rust/crates/dbrain-wiki`; normales Mitglied in `rust/Cargo.toml`. Cargo-Paket/Import/Binary behalten den kompatiblen Namen `dbrain-s12-wiki-probe` / `dbrain_s12_wiki_probe`. `dbrain-sources` referenziert ausschließlich den neuen Crate-Pfad.

`src`, `tests` und `examples` sind verschoben. Historische `fixtures`, `reports`, der alte Lockfile und die Dokumentation bleiben in `architecture/migration/s12`. Der alte Manifestinhalt ist als `Cargo.toml.historical` erhalten; dort existiert keine aktive Produktionscrate mehr. Alte Berichtshashes beschreiben ihre damaligen Git-Commits. Insbesondere wurden die gespeicherten Completion-Berichte nicht durch neue grüne Ergebnisse überschrieben.

`check.sh`, `check-completion.py` und die beiden betroffenen Workflow-Aufrufe verwenden den neuen Manifest-/Workspacepfad. Pakettests lesen historische Fixtures explizit; produktiver Parsercode importiert keine Rust-Implementierung aus der Architekturablage. Die globale Test-/Report-Infrastruktur aus C8 wurde nicht umgebaut.

## Tests und Ausführung

Neue Regressionen liegen in:

- `brain-contracts/tests/ir_contracts.rs`: Versionen, Serde-Roundtrips, explizite Missingness, Null/Zero, Bedingungen/Varianten/Locators, Policy und Original-/Parser-/Schema-/Game-Revisionsgrenzen, Replay-v1-Import, Unknown-Entities/Felder und batchseitige Ablehnung unbekannter Herkunftsversionen.
- `dbrain-wiki/tests/shared_contracts.rs` und `dbrain-sources/tests/shared_contracts.rs`: tatsächliche Producer-zu-gemeinsamem-DTO-Roundtrips; zusätzlich bleiben sämtliche vorhandenen Wiki-/Source-/Replay-Regressionen erhalten.
- `brain-storage/tests/ir_contracts.rs`: gemeinsamer Ursprung durch den tatsächlichen `MemoryRepository`, Scopes, gesperrter Provider-Egress und Rechtewiderruf gegen alte Releases.

Ausführung im `rust/`-Workspace mit Rust 1.97.1, `SQLX_OFFLINE=true`, begrenzter Buildparallelität und einer expliziten Umgebungs-Allowlist ohne Anwendungs-/Datenbank-Zugangsdaten. Die abschließenden Prüfungen verwenden ein separates leeres Test-HOME; Cargo-/Rustup-Caches werden nur für die bereits vorhandenen Buildabhängigkeiten referenziert. Tests und Release-Build verwenden den Compiler direkt und `CARGO_NET_OFFLINE=true`. Explizit ignorierte Live-/Scratch-DB-Tests bleiben ignoriert; kein Live-/Produktionsnachweis wird daraus abgeleitet.

<!-- C4_CHECK_RESULTS -->
| Geforderter Befehl im `rust/`-Workspace | Abgeschlossenes Ergebnis |
|---|---|
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Exit 0 |
| `cargo test --workspace --locked` | Exit 0; 846 bestanden, 0 fehlgeschlagen, 67 bestehende Ignore-Markierungen |
| `cargo build --workspace --release --locked` | Exit 0; vollständiges optimiertes Release-Profil |

31 neue aktive Regressionstests: 23 in `brain-contracts`, drei in `brain-storage`, drei in `dbrain-sources` und zwei im verschobenen Wiki-Paket. Kein neuer Test wurde mit `ignore` versehen. Die 67 übersprungenen Bestandsprüfungen sind kein Live-/Datenbank-/Produktionsnachweis.

Die zusätzliche Regression `legacy_head_cannot_supply_missing_rights_for_versioned_origin` wurde vor der Korrektur als Fehler reproduziert und besteht im abschließenden vollständigen Workspace-Lauf. Der Fix prüft die zusammengeführte wirksame Policy erneut, bevor ein Datensatz für den Provider freigegeben wird.

Eine kombinierte Prüfinstanz endete ohne abschließenden Status und wurde ausdrücklich nicht als Erfolgsnachweis verwendet. Tests und Release-Build wurden anschließend separat vollständig wiederholt; die Tabelle enthält nur tatsächlich bestätigte Exit-Codes. Shell-/Python-Syntax der angepassten S12-Einstiegspunkte und `git diff --cached --check` sind ebenfalls geprüft. Historische Fixtures und Berichte wurden nicht geändert.
<!-- /C4_CHECK_RESULTS -->

## G1: erfüllt und noch offen

**Durch C4 im eigenen Branch erfüllt:** Ein Besitzer der Wiki-/External-/Replay-IR-Typen in der vorhandenen Contract-Crate; versionierte gemeinsame Herkunft und explizite Unknown-Zeit-/Ticksemantik; verlustfreie Consumer-Übergaben einschließlich Locator, Bedingungen/Varianten und Policy; aktiver S12-Code im regulären Workspace; Vertrags-/Kompatibilitäts-/ACL-Regressionen statt nur neu angelegter unbenutzter Typen.

**Für eine projektweite G1-Freigabe noch offen:** Review und Integration dieses Branches, erneute Prüfung der dann kombinierten Integrationsbasis und formale Bestätigung von Contract-/Schema-/Pfadbesitz durch 00/02/03. Die bestehenden Schema-/Release-Abnahmen werden durch additive IR-Metadaten nicht automatisch erweitert. SLO-/Lastprofil-Freigabe und Quellen-/Rechteentscheidungen sind laut Basisstatus weiterhin offen und wurden hier nicht erteilt.

Konkret enthält `PFAD_OWNER.csv` noch die S000-Zuordnung gemeinsamer Verträge zu `deadlock-brain-core` und keine Zuordnung des neuen `rust/crates/dbrain-wiki`-Pfads. Die Freigabe der kanonischen Contract-Pfade durch 02 und des Parserpfads durch 12/00 ist dort noch nachzuführen; dieser Handoff erteilt diese organisatorische Freigabe nicht selbst.

Die breitere Planpaketforderung nach vollständig kontexttragenden kanonischen Fact-/Effect-/Rule-/Kartenverträgen ist durch ein IR-Refactoring **nicht als vollständig abgenommen belegt**. Die reduzierten bestehenden `brain.v1::Fact`/`HeroKnowledgeCard` dürfen kontextreiche IR-Felder weiterhin nicht verlustbehaftet übernehmen. Gemeinsame Repräsentierbarkeit ist jetzt vorhanden; fachlich freigegebene Projektion und der tatsächliche Kernel-/Release-Anschluss sind getrennt zu belegen. Dieser PR markiert deshalb nicht eigenmächtig das gesamte G1 als bestanden.

Echte Wiki-Capture-/Stage-/Publish-Piloten, freigegebene reale Replays, produktiver Consumeranschluss und Staging-/Restore-/Lastnachweise sind ebenfalls nicht erbracht. Das sind keine durch Serde-Tests ersetzbaren G2–G4-Nachweise.

## Verbindliche Anschlussstellen für C5/C6

**C5:** Den verschobenen `dbrain_s12_wiki_probe::knowledge::extract` und dessen vertrauensgebundenen `WikiIr` weiterverwenden. Die bestehende Funktion `dbrain_sources::wiki_capture_io::stage_sources_with_pool` bleibt erhalten; ihre `SourceRecordV2` tragen jetzt den gemeinsamen Ursprung. Für neue Grenzen `Versioned<brain_contracts::wiki::WikiIr>`, `OriginArtifact`, `SourceRecordV2`, `SourceBatch`, `DocumentStorePort` und den bestehenden `CorpusRelease` verwenden. Discovery-Limits, echte Capture-/Scratch-DB-Probe und kanonische Fact-/Release-Veröffentlichung bleiben C5; keine private Ersatz-IR und kein zweiter Publisher.

**C6:** `brain_contracts::wiki::{IrField, IrValue, Unit, SourceLocator, Alias, Dependency}` samt zugehörigem `OriginArtifact`/`GameValidity` als verlustfreien Source-Input verwenden. Domain-seitig an `NumericFact`, `TypedRule`, `DomainStorePort` und `RuleEvaluatorPort`, anschließend am gleichen `CorpusRelease` und vorhandenen Kernel-/Evidence-Port anschließen. Bedingungen, Varianten, Unknowns, Quellenrechte und konkrete Revision müssen vor jeder kontextreduzierenden Projektion geprüft werden. `HeroKnowledgeCard`/`Fact` bleiben nur für die tatsächlich ausdrückbaren, geprüften Fälle zulässig. Keine Regel, kein numerischer Wert und keine Patch-/Mode-Zuordnung aus Unknown erfinden.

Für spätere empirische Eingänge ausschließlich `brain_contracts::replay::{ReplayReport, ReplayObservation, ObservationProvenance, ReplayDecoder, ObservationStore}` verwenden, nicht die alten zeitpflichtigen Summary-Typen und nicht Adapter-lokale Kopien.
