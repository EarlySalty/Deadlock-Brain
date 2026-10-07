# G0: stabiler Werkzeugvertrag

Stand: 07.10.2026. G0 ist für den Anschluss von G-P und G-K verifiziert. Vorhandenes WIP wurde fortgeführt; keine Rücksetzung, kein Git-Schreibschritt und kein Runtimeeingriff. Gepinnter Arbeits-HEAD: `f129c91a3c24c0b414ab23eeef947fa9bbf3e1f3`.

## Produktdateien und Exporte

- `/home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/brain-contracts/src/lib.rs`
- `/home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/brain-contracts/src/provider_input.rs`
- `/home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/brain-contracts/src/tools.rs`

Am Crateroot exportiert: `ModelBlock`, `PinnedGameContext`, `ProviderFinishReason`, `ProviderTurn`, `ToolCall`, `ToolConversation`, `ToolDefinition`, `ToolEvidenceDependency`, `ToolExecution`, `ToolExecutionPort`, `ToolMessage`, `ToolName`, `ToolRequest`, `ToolResult`, `ToolSubrequest`, `ToolValidationPurpose`.

Die konkreten Unteranfragen, Szenariotypen, `ToolLanguage`, `ToolPlaystyle` und `validate_definitions` sind öffentlich unter `brain_contracts::tools`. Darstellung und Zählung liegen unter `brain_contracts::provider_input`. Bestehende `Query`, `AuthorizedContext`, `Evidence`, `ProviderAnswer` und `Usage` behalten ihre Felder und Drahtverträge.

## Providerport

Die endgültigen Traitmethoden lauten, jeweils mit `&self`:

```rust
fn answer(
    &self,
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
) -> std::result::Result<ProviderAnswer, PortError>;

fn answer_turn(
    &self,
    query: &Query,
    context: &AuthorizedContext,
    evidence: &[Evidence],
    tools: &[tools::ToolDefinition],
    conversation: &tools::ToolConversation,
) -> std::result::Result<tools::ProviderTurn, PortError>;
```

`answer_turn` hat einen Default. Er ruft `answer` ausschließlich bei einer gültigen Textanfrage mit `query.domain == None`, leerer Werkzeugliste und leerem Werkzeuggespräch auf. Alle anderen Fälle scheitern vor dem Textport. Die erfolgreiche Textantwort wird in `Final` mit `EndTurn` umgewandelt und validiert; Usage bleibt unverändert. Der Default ersetzt oder erneuert den Kontext und die Deadline nicht.

Unveränderte vorhandene Textprovider, die Abobrücke und Kernel-Mocks kompilieren weiterhin. Es gibt keinen stillen Werkzeug-zu-Text-Rückfall.

## Aufrufe und geschlossene Schemas

```rust
pub struct ToolDefinition {
    pub name: ToolName,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub struct ToolCall {
    pub id: String,
    pub name: ToolName,
    pub arguments: serde_json::Value,
}
```

Öffentliche Methoden:

```rust
ToolDefinition::validate(&self) -> Result<(), PortError>
ToolCall::validate(&self, definitions: &[ToolDefinition]) -> Result<ToolRequest, PortError>
ToolName::as_str(self) -> &'static str
validate_definitions(definitions: &[ToolDefinition]) -> Result<(), PortError>
ToolRequest::name(&self) -> ToolName
ToolRequest::arguments(&self) -> &serde_json::Value
ToolRequest::subrequest(&self) -> &ToolSubrequest
```

`ToolRequest` hat private Felder und keinen `Deserialize`-Eingang. Er entsteht ausschließlich aus einem validierten `ToolCall`. Er enthält Werkzeugname, ursprüngliches Argumentobjekt und die konkrete typisierte Unteranfrage. `ToolCall.id` wird gesondert an den Ausführungsport übergeben. Die serialisierte Requestdarstellung enthält `name` und `arguments`; die typisierte Unteranfrage wird daraus bei einer späteren Rekonstruktion erneut über `ToolCall::validate` gewonnen.

`ToolName` ist ein geschlossenes Enum mit genau diesen Drahtnamen:

| Variante | Drahtname |
| --- | --- |
| `EntityFind` | `entity_find` |
| `EntityProfile` | `entity_profile` |
| `HeroCompare` | `hero_compare` |
| `DamageCalculate` | `damage_calculate` |
| `PatchHistory` | `patch_history` |
| `BuildPlan` | `build_plan` |
| `ServerKnowledge` | `server_knowledge` |

Ein Schema beginnt als geschlossenes Objekt mit `type: object`, `properties` und `additionalProperties: false`. Unterstützt werden verschachtelte geschlossene Objekte, Arrays mit `items`, die skalaren Typen `string`, `integer`, `number`, `boolean`, `null`, außerdem `required`, `enum`, `description`, `title`, Längen- und Zahlengrenzen. Unbekannte Schlüssel, Referenzen, Typvereinigungen und Kombinationsschemas werden abgewiesen. Optionale Felder können fehlen; explizites `null` benötigt ein entsprechend zugelassenes Schema. Doppelte Definitionen, Pflichtfelder und Enumwerte sind ungültig. Ganzzahlgrenzen werden ohne f64-Rundung verglichen.

Die Prüfung erfolgt in drei Schritten: angebotene Definition finden, Argumente gegen das Schema prüfen, dann in die konkrete Unteranfrage des Werkzeugnamens deserialisieren und fachliche Grundbedingungen prüfen. Auch ein Schema mit einem unbekannten Fachfeld kann dadurch keinen neuen Unteranfragetyp erzeugen.

Serverbindungen und ausführbare Parameter sind auch in verschachtelten Schemas verboten, darunter Actor, Rechte, Release, aktuelle Version, Deadline, Request-/Conversation-ID, DSN, URL, SQL, Provider, Modell, Endpunkt und Ausdruck. Historische Clientversionen sind ausschließlich das ausdrücklich benannte Feld von `PatchHistoryRequest`; sie ersetzen keinen serverseitigen aktuellen Pin.

## Konkrete Unteranfragen

`ToolSubrequest` ist ein öffentliches Enum mit genau sieben Varianten. Jede Variante enthält den folgenden konkreten Typ. Die serialisierte Form verwendet `name` und `arguments`; ein ungeprüfter Deserialisierungseingang für dieses Enum ist nicht exportiert.

| Variante und Typ | Öffentliche Felder |
| --- | --- |
| `EntityFind(EntityFindRequest)` | `query: String`, `kind: Option<EntityKind>`, `language: ToolLanguage` |
| `EntityProfile(EntityProfileRequest)` | `entity: ToolEntityRef`, `fields: Vec<String>`, `scenario: Option<ToolScenario>` |
| `HeroCompare(HeroCompareRequest)` | `hero_ids: Vec<u64>`, `metrics: Vec<String>`, `scenario: ToolScenario`, `ranking_population: Option<ToolRankPopulation>` |
| `DamageCalculate(DamageCalculateRequest)` | `hero_id: u64`, `scenario: ToolScenario` |
| `PatchHistory(PatchHistoryRequest)` | `entity: ToolEntityRef`, `ability_id: Option<u64>`, `fields: Vec<String>`, `from_patch: Option<String>`, `to_patch: Option<String>`, `historical_client_versions: Vec<i64>` |
| `BuildPlan(BuildPlanRequest)` | `hero_id: u64`, `playstyle: ToolPlaystyle`, `budget: Option<u64>`, `imbues: Vec<ToolImbue>` |
| `ServerKnowledge(ServerKnowledgeRequest)` | `question: String`, `public_channel_id: Option<u64>` |

`EntityKind` bleibt das vorhandene `Hero | Ability | Item`. Weitere öffentliche Typen:

- `ToolLanguage`: `German | English`, Drahtnamen `german | english`.
- `ToolEntityRef`: `kind: EntityKind`, `id: u64`.
- `ToolRankPopulation`: ausschließlich `ActiveHeroes`, Drahtname `active_heroes`.
- `ToolPlaystyle`: `Weapon | Spirit | Tank`, Drahtnamen `weapon | spirit | tank`; `as_str(self) -> &'static str`.
- `ToolProgression`: `Souls(u64) | Boons(u32)`, Drahtform `{"kind":"souls|boons","value":...}`.
- `ToolItemTransition`: `Buy { item_id: u64 }`, `Sell { item_id: u64 }`, `Upgrade { from_item_id: u64, to_item_id: u64 }`, intern mit `kind` getaggt.
- `ToolImbue`: `item_id: u64`, `ability_id: u64`.
- `ToolAbilityRank`: `ability_id: u64`, `rank: u32`.
- `ToolDistanceUnit`: `GameUnits | Meters`, Drahtnamen `game_units | meters`.
- `ToolDistance`: `value: f64`, `unit: ToolDistanceUnit`.
- `ToolTargetKind`: `Player | Npc | Objective`, entsprechende snake_case-Drahtnamen.
- `ToolTargetValues`: jeweils `Option<f64>` für `health`, `health_regen`, `bullet_shield`, `spirit_shield`, `bullet_resist`, `spirit_resist`.
- `ToolTargetState`: `at_seconds: f64`, `values: ToolTargetValues`.
- `ToolTarget`: `kind: ToolTargetKind`, `values: ToolTargetValues`, `distance: Option<ToolDistance>`, `hit_chance: Option<f64>`, `headshot_fraction: Option<f64>`, `states: Vec<ToolTargetState>`.
- `ToolScenario`: `progression: ToolProgression`, `level: Option<u32>`, `ability_points: Option<u32>`, `total_spirit: Option<f64>`, `item_ids: Vec<u64>`, `item_transitions: Vec<ToolItemTransition>`, `imbues: Vec<ToolImbue>`, `ability_ranks: Vec<ToolAbilityRank>`, `target: Option<ToolTarget>`, `horizon_seconds: Option<f64>`.

Default-leere Listen: Szenario-Items, Übergänge, Imbues und Fähigkeitsränge; Zielzustände; Patchfelder und historische Versionen; Build-Imbues. Alle Drahtstrukturen lehnen unbekannte Felder ab.

Grundbedingungen umfassen positive gültige Entitäts-IDs, eindeutige ID-/Feldlisten und Bindungen, mindestens zwei verschiedene Vergleichshelden, positive explizite Level/Buildbudgets/Horizonte, endliche Szenariowerte, nichtnegative Spirit-/Distanz-/Lebens-/Schildwerte, Trefferanteile in `[0, 1]` und streng aufsteigende Zielzustandszeiten innerhalb eines expliziten Horizonts. Resistwerte dürfen negativ sein. Die Prüfung behauptet keine Entitätsauflösung oder vollständige Spielmechanik; erlaubte Felder, Metriken und weitergehende fachliche Grenzen legt das angebotene Schema beziehungsweise der bestehende Fachport fest.

## Modellblöcke, Gespräch und Ergebnis

```rust
pub enum ModelBlock {
    Text { text: String },
    ToolUse { call: ToolCall },
}

pub struct ToolResult {
    pub call_id: String,
    pub name: ToolName,
    pub result: serde_json::Value,
    pub evidence_ids: Vec<String>,
    pub is_error: bool,
}

pub enum ToolMessage {
    Assistant { blocks: Vec<ModelBlock> },
    ToolResults { results: Vec<ToolResult> },
}

pub struct ToolConversation {
    pub messages: Vec<ToolMessage>,
}
```

`ModelBlock` ist mit `type` getaggt, `ToolMessage` mit `role`. Insbesondere ist `ToolUse { call }` eine Structvariante und kein Tupel. Das Ergebnisfeld heißt endgültig `result`, nicht `content`. Unbekannte Blöcke und zusätzliche Drahtfelder werden abgewiesen.

`ToolConversation::validate(&self, definitions: &[ToolDefinition]) -> Result<(), PortError>` akzeptiert das leere Gespräch oder vollständige Werkzeugrunden. Ein Assistant-Eintrag enthält mindestens einen validierten Aufruf. Danach müssen genau die zugehörigen Ergebnisse folgen, jeweils mit passender ID und passendem Werkzeugnamen. Die Reihenfolge der Ergebnisse innerhalb der Ergebnisgruppe darf von der Aufrufreihenfolge abweichen. Doppelte IDs innerhalb eines Turns oder über mehrere Turns, doppelte Ergebnisse, fehlende Ergebnisse und Ergebnisse ohne Aufruf sind ungültig. Eine finale Textantwort wird nicht als Werkzeuggesprächseintrag angehängt.

## Turn und Usage

```rust
pub enum ProviderFinishReason {
    EndTurn,
    ToolUse,
    MaxTokens,
    Refusal,
}

pub enum ProviderTurn {
    Final {
        answer: ProviderAnswer,
        finish_reason: ProviderFinishReason,
    },
    ToolCalls {
        blocks: Vec<ModelBlock>,
        finish_reason: ProviderFinishReason,
        usage: Usage,
    },
}
```

`ProviderTurn` verwendet `kind` als Drahttag. Öffentliche Methoden:

```rust
ProviderTurn::usage(&self) -> &Usage
ProviderTurn::validate(&self, definitions: &[ToolDefinition]) -> Result<(), PortError>
ProviderTurn::into_answer(self) -> Result<ProviderAnswer, PortError>
```

`From<ProviderAnswer>` erzeugt einen finalen `EndTurn`. `Final` hat Usage ausschließlich in `answer.usage`; `ToolCalls` hat sein eigenes Usagefeld. Das bestehende `Usage` enthält `provider: Option<String>`, `model: Option<String>`, `input_tokens: u64`, `output_tokens: u64`, `network_rounds: u32`, `cost_micros: u64`.

Nur `Final` mit `EndTurn` ist abschließend. Nur `ToolCalls` mit `ToolUse` und mindestens einem gültigen Aufruf ist ein Werkzeugturn. `MaxTokens`, `Refusal`, doppelte Beleg-IDs und unvollständige Turns werden nicht als erfolgreiche Antwort akzeptiert. Finale Beleginhalte und Freigaben prüft weiterhin der Kernel. G-P setzt Usage aus der tatsächlichen Transportmessung, nicht aus Modellargumenten.

## Ausführungsport und vollständige Abhängigkeiten

```rust
pub struct PinnedGameContext {
    pub client_version: i64,
    pub language: ToolLanguage,
    pub mechanic_revision: String,
}

pub struct ToolEvidenceDependency {
    pub request: ToolRequest,
    pub game_context: Option<PinnedGameContext>,
    pub evidence: Vec<Evidence>,
}

pub struct ToolExecution {
    pub result: ToolResult,
    pub dependencies: Vec<ToolEvidenceDependency>,
    pub usage: Usage,
}

pub enum ToolValidationPurpose {
    Provider,
    Publication,
    Cache,
}
```

Die endgültigen Traitmethoden von `ToolExecutionPort: Send + Sync` lauten:

```rust
fn definitions(
    &self,
    query: &Query,
    context: &AuthorizedContext,
    game_context: Option<&PinnedGameContext>,
) -> Result<Vec<ToolDefinition>, PortError>;

fn execute(
    &self,
    query: &Query,
    context: &AuthorizedContext,
    game_context: Option<&PinnedGameContext>,
    call_id: &str,
    request: &ToolRequest,
) -> Result<ToolExecution, PortError>;

fn validate_dependencies(
    &self,
    query: &Query,
    context: &AuthorizedContext,
    game_context: Option<&PinnedGameContext>,
    dependencies: &[ToolEvidenceDependency],
    purpose: ToolValidationPurpose,
) -> Result<(), PortError>;
```

`execute` ist verpflichtend. Die Defaults von `definitions` und `validate_dependencies` scheitern geschlossen mit `Unavailable`, auch bei leerer Abhängigkeitsliste. Eine Implementierung muss die kanonische Prüfung ausdrücklich bereitstellen.

Weitere öffentliche Methoden:

```rust
PinnedGameContext::validate(&self) -> Result<(), PortError>
ToolExecution::validate_for(
    &self,
    call: &ToolCall,
    request: &ToolRequest,
    game_context: Option<&PinnedGameContext>,
) -> Result<(), PortError>
```

Spielwerkzeuge benötigen eine positive gebundene Clientversion und gültige Mechanikrevision. Bei `entity_find` muss die angefragte Sprache zum Pin passen. `server_knowledge` hat keine Spielbindung in seinen Abhängigkeiten.

`validate_for` bindet Ergebnis-ID, Werkzeugname und ursprüngliche Argumente an den validierten Aufruf. Jede Abhängigkeit hält dieselbe konkrete Unteranfrage und die dazugehörige Spielbindung. Alle Evidenceobjekte werden geprüft, ihre IDs müssen global innerhalb der Ausführung eindeutig sein. Jede `result.evidence_ids`-ID muss in diesen Abhängigkeiten vorkommen. Erfolgreiche Ausführungen ohne Belegabhängigkeit sind ungültig; ein Fehlerergebnis darf ohne Belege zurückgegeben werden. Nicht zitierte Abhängigkeiten bleiben erhalten und unterliegen denselben späteren Freigabeprüfungen.

Diese Strukturprüfung ersetzt weder Quellen-/Versionsprüfung noch Rechte, Publikationsfreigabe oder Cache-Neuprüfung. Solche Prüfungen laufen für die ganze Abhängigkeitsmenge über den konkreten Port und den jeweiligen `ToolValidationPurpose`.

## Gemeinsame Darstellung und Eingabezählung

Die öffentlichen Signaturen unter `brain_contracts::provider_input` sind:

```rust
pub enum ToolWireFormat {
    Native,
    OpenAiCompatible,
}

pub fn grounded_messages(query: &Query, evidence: &[Evidence]) -> Vec<ChatMessage>;
pub fn grounded_input_ceiling(query: &Query, evidence: &[Evidence]) -> u64;

pub fn grounded_turn_payload(
    query: &Query,
    evidence: &[Evidence],
    definitions: &[ToolDefinition],
    conversation: &ToolConversation,
    format: ToolWireFormat,
) -> Result<serde_json::Value, PortError>;

pub fn grounded_turn_input_ceiling(
    query: &Query,
    evidence: &[Evidence],
    definitions: &[ToolDefinition],
    conversation: &ToolConversation,
    format: ToolWireFormat,
) -> Result<u64, PortError>;

pub fn transport_input_ceiling(payload: &serde_json::Value, chat: bool)
    -> Result<u64, PortError>;
```

`ChatMessage` behält `role: &'static str` und `content: String`.

`grounded_turn_payload` validiert zuerst das Gespräch und liefert ausschließlich die gemeinsamen modellseitigen Felder. Native Darstellung: separates `system`, Text-/`tool_use`-Blöcke, `tool_result`-Blöcke mit `tool_use_id`. OpenAI-kompatible Darstellung: Systemnachricht, Assistant-Text plus `tool_calls`, Funktionargumente als JSON-String und Ergebnisnachrichten mit `tool_call_id`. Ergebnisinhalt enthält `result` und `evidence_ids`; der Fehlerstatus wird ebenfalls dargestellt. G-P ergänzt ausschließlich seine bestehenden Transportoptionen.

`grounded_turn_input_ceiling` zählt exakt diese Darstellung durch `transport_input_ceiling`. Auch der alte Textpfad und Embeddings verwenden denselben Zähler. Keine zweite Zählformel und keine bytes/4-Schätzung. Modelltext wird konservativ nach UTF-8-Bytes einschließlich Framing gezählt. Definitionen, Beschreibung und Schema, Aufrufname/ID, Argumente, Ergebnis-/Beleg-IDs, Textblöcke und jede wiederholte Gesprächsrunde werden vollständig mitgezählt. `tool_choice`, `response_format`, `stop` und insbesondere modellseitige Schemas in `output_config` werden mitgezählt. Reine bekannte Transportoptionen wie Modellname, Tokenlimit und Streamingflag zählen nicht als Modelltext. Unbekannte Felder und Blöcke führen zu einem Fehler statt zu ungezählter Eingabe.

Raw-Wire-Prüfung kontrolliert ebenfalls angebotene Definitionen, Aufrufargumente, eindeutige IDs und vollständige Ergebniszuordnung. Alle Summen verwenden Checked Arithmetic; Überlauf ergibt `BudgetExceeded`. Der kompatible alte `grounded_input_ceiling` liefert bei einem Zählfehler `u64::MAX`, sodass die Budgetprüfung geschlossen bleibt. Ohne Werkzeuge/Historie stimmt die neue Obergrenze in beiden Darstellungen mit dem bisherigen Textpfad überein.

## Kleinster Anschluss

G-P überschreibt `answer_turn`, nutzt die gemeinsame Darstellung und dieselbe Zählung und bildet tatsächliche Modellblöcke, Abschlussgrund und Usage auf `ProviderTurn` ab. Der vorhandene Textport bleibt bestehen; keine neue Antwortstrecke, kein eigener Zähler und keine neue Modell-/Timeoutkonfiguration.

G-K besitzt den Loop: Definitionen validieren, jeden Turn und Aufruf validieren, die konkrete `ToolRequest` mit unverändertem `AuthorizedContext`, Spielpin und ursprünglicher Deadline ausführen, anschließend `validate_for` aufrufen. Vor Modellweitergabe, finaler Veröffentlichung und jedem Cachetreffer sämtliche Abhängigkeiten kanonisch erneut prüfen. Budget und Usage bleiben über alle Runden kumuliert. Release, Actor, Rechte, aktuelle Version und Deadline kommen niemals aus Modellargumenten. Finale Antworten gehen durch die vorhandenen Beleg- und Ausgabeprüfungen.

Kein Dispatcher, SQL-Leser, Runtimeanschluss oder Produktions-/Netzbeweis ist Bestandteil von G0. Gate und Paketcommit verbleiben bei der Bereichsführung.

## Verifikation und Rohbelege

Prüfverzeichnis: `/home/nathanael/.worktrees/brain-g-v2-20261007/.tasks/2026-10-06-brain-abschluss/G/pruefungen/g0-r1/`.

Alle Cargoläufe verwenden `/home/nathanael/.cargo/bin/cargo`, Debugtarget `/tmp/brain-g0-r1-target`, höchstens zwei Jobs und den vorhandenen Buildslot `/tmp/deadlock-cargo-release.lock`. Kein ENV wurde verändert. Zu jeder genannten `.log` liegt der unverdeckte Prozess-Exit in der gleichnamigen `.exit` vor.

| Abschlussprüfung | Exit | Rohbeleg im Prüfverzeichnis |
| --- | --- | --- |
| Formatprüfung ausschließlich eigener Dateien | 0 | `fmt.log`, `fmt.exit` |
| Compiler, Crate und alle Testtargets | 0 | `check.log`, `check.exit` |
| Strict Clippy, alle Targets, `-D warnings` | 0 | `clippy.log`, `clippy.exit` |
| Bestehende und ergänzte Cratetests | 0 | `test.log`, `test.exit` |
| Direkte Verbraucher und deren Testtargets | 0 | `consumers-direkt.log`, `consumers-direkt.exit` |
| Provider, Kernel, Retrieval und Serve, alle Targets | 0 | `consumers-abschluss.log`, `consumers-abschluss.exit` |

Tatsächliche Testzahlen: 32 Unitfälle und 23 Integrationsfälle bestanden; 0 fehlgeschlagen, 0 ignoriert, 0 gemessen, 0 weggefiltert. Doc-Tests: 0 Fälle. Insgesamt 55 bestandene Fälle. Die Prüfungen liefen gegen die echten Vertragsvalidatoren und Renderer, nicht gegen Ersatzvalidatoren. Der Textport-Test verwendet nur für den Aufrufzähler einen Testprovider.

Abgedeckt sind alle sieben typisierten Unteranfragen, ungültige Schemas/Felder/Namen, unbekannte Modellblöcke, Duplikate, Ganzzahlgrenzen, Szenariogrundbedingungen, tatsächliche Ergebnis-/Callzuordnung, vollständige gebundene Abhängigkeiten, sichere Defaults, Abschlussgründe, Usageerhalt und vollständige wiederholte Eingabezählung in beiden Darstellungen.

Der erste erweiterte Verbrauchercheck ist als `consumers.log`/`consumers.exit` erhalten: Exit 101 wegen eines fehlenden `SimulationTarget.updated_at` in `/home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/dbrain-reasoner/src/combat.rs`. G0 hat diese fremde Datei nicht geändert. Der abschließende tatsächliche erweiterte Check besteht mit Exit 0 ohne Warnungen; der ältere Fehlversuch ist kein aktueller Blocker und wurde nicht als Baseline-Ausrede für den Vertrag benutzt. Auch Compiler und Strict Clippy des eigenen Crates bestehen ohne Warnungen.

Die historischen roten G0-Prüfungen und die alten grünen Zwischenlogs wurden vor der Fortsetzung untersucht. Maßgeblich sind ausschließlich die neuen Abschlusslogs. Der vorhandene historische Crate-Baselinelauf `/tmp/brain-g0-20261007/baseline.log` hatte 22 Unit- und 23 Integrationsfälle bestanden, 0 fehlgeschlagen. Daraus wird kein grüner Gesamtworkspace oder Produktionsbeweis abgeleitet.

Exakte Abschlussbefehle:

```sh
/home/nathanael/.cargo/bin/rustfmt --check --edition 2021 --config skip_children=true /home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/brain-contracts/src/lib.rs /home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/brain-contracts/src/provider_input.rs /home/nathanael/.worktrees/brain-g-v2-20261007/rust/crates/brain-contracts/src/tools.rs

flock /tmp/deadlock-cargo-release.lock /home/nathanael/.cargo/bin/cargo check --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package brain-contracts --all-targets --locked --offline --jobs 2 --target-dir /tmp/brain-g0-r1-target

flock /tmp/deadlock-cargo-release.lock /home/nathanael/.cargo/bin/cargo clippy --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package brain-contracts --all-targets --locked --offline --jobs 2 --target-dir /tmp/brain-g0-r1-target -- -D warnings

flock /tmp/deadlock-cargo-release.lock /home/nathanael/.cargo/bin/cargo test --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package brain-contracts --locked --offline --jobs 2 --target-dir /tmp/brain-g0-r1-target -- --include-ignored --test-threads=1

flock /tmp/deadlock-cargo-release.lock /home/nathanael/.cargo/bin/cargo check --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package brain-providers --package brain-kernel --all-targets --locked --offline --jobs 2 --target-dir /tmp/brain-g0-r1-target

flock /tmp/deadlock-cargo-release.lock /home/nathanael/.cargo/bin/cargo check --manifest-path /home/nathanael/.worktrees/brain-g-v2-20261007/rust/Cargo.toml --package brain-providers --package brain-kernel --package dbrain-retrieval --package brain-serve --all-targets --locked --offline --jobs 2 --target-dir /tmp/brain-g0-r1-target
```

`source.sha256` im Prüfverzeichnis bindet diese Belege an die drei endgültigen Quelldateien. Die Fingerprints waren auch nach dem erweiterten Verbraucherabschluss unverändert.

TESTNACHWEIS[TW-1]: 55 passed, 0 ignored | Baseline: 0 rot

WIRKUNGSPRUEFUNG[WP-1]: 1 Befunde | Zwillingssuche: grep-belegt | Fremddienst-Pfade: 0/0 geprüft

Der behobene zusätzliche Befund war die zuvor ungezählte native `output_config`-Struktur. Die Zwillingssuche bestätigte den gemeinsamen Zähler und prüfte die vorhandenen bereits gezählten Felder `response_format`, `tool_choice` und `stop`. Neue externe Dienstpfade wurden nicht eingeführt.
