[Orchestrator]
# K-Ortsvertrag: Cachebindung und mechanische Literalanschlüsse

Entscheidung zur bereits offenen K-Eigentumsfrage, keine neue Funktion. K-ORTSVERTRAG-KERNELBEDARF.md und G/REGISTER.md gelesen. Aktiver G-Produktwriter besitzt S3/S4-Reasonerdateien; die nachfolgenden begrenzten Deltas sind davon getrennt. Dringender Privatsperrenfix bleibt VOR diesem Ortsabschluss.

## Exklusive Zuordnung

K f19bfcf9-1045-480a-b328-1f1c7c62f086 erhält zusätzlich:

1. rust/crates/brain-kernel/src/flight.rs ausschließlich das tatsächliche query.answer_context im vorhandenen cache_key_for_purpose-Tupel. Bestehende Zweck-/Principal-/Release-/Budget-/Scopebindung unverändert. Passende bestehende Cache-/Flighttests auf None und unterschiedliche Orte erweitern. Kein zweiter Cache oder neue Engine.
2. Ausschließlich mechanische answer_context: None-Ergänzungen in den ausdrücklich aufgelisteten Query-Literalen des K-Berichts ORTSVERTRAG-KERNELBEDARF.md, Abschnitt „Mechanische Literalbedarfe“, Stand dieser Entscheidung: brain-contracts/bot_tasks, brain-feeds/tests/match_store; brain-kernel fact_relevance, flight/c3_shared_validation, flight/review_dependencies, flight/review_publication, lib und die dort genannten Tests; brain-legacy-import/import_firstparty; brain-maintenance/tests/workflow; brain-policy lib/core_policy; brain-providers lib/faults; dbrain-retrieval contract_port/entity_profile_port und die vier genannten Tests; dbrain-sources forum/entity_profiles; dbrain-wiki/domain_projection; deadlock-brain brain-mcp/main. Nur tatsächliche Contract-Query-Literale, keine CLI-Enums oder unnötigen Struct-Update-Ergänzungen. Keine weitere Semantik-/Formatänderung.

Die Liste des Berichts ist bindend, keine pauschale Schreibfreigabe für diese Crates. Produktive Consumer liefern tatsächlichen Ortskontext; None nur für die bestehenden ortslosen Aufrufer und Fixtures. G-Reasoner-, Deadline-, Toolport- und I-Planer-/Assets-/Receiptaufgaben bleiben unverändert. G und I werden vor K benachrichtigt, diese exakten Deltas nicht parallel zu implementieren. Bei einem tatsächlichen bereits aktiven Writer auf einer benannten Stelle geordneten Halt/Übergabe über Delegator melden, keinen fremden WIP überschreiben.

## Integration und Nachweis

K arbeitet ausschließlich im eigenen Worktree, erhält den pausierten Vertrags-WIP und integriert den inzwischen gelieferten Spiegel b7289d11 erst geordnet gegen gesicherten Stand. Keine Änderungen an Is unveränderlichem Release-Buildbaum. Öffentliches JSON bleibt kompatibel, Rust-Literaländerungen und Flightbindung im selben konsumierbaren geprüften Vertragscommit liefern. Compiler, passende bestehende Prüfungen und regulärer Gate müssen tatsächlichen gemeinsamen Stand abdecken; Featureconsumer erst auf gesicherten Commit pinnen.

Alle bisherigen Datenschutz-, Payloadbudget-, Rechte-, Cache- und Providergrenzen bleiben bestehen. Rollenbindung NEVER aus Ortsdaten ableiten. Technische IDs und Fremddaten NEVER ans Modell. Keine bestehenden Rechte zum Bauen oder Neustarten erweitern. Fehlender regulärer Brain-Neustartweg bleibt eigener technischer Blocker, kein Grund zur Umgehung und kein Anlass, den dringenden Botsfix liegenzulassen.

Direkter Auftraggeber bleibt 481426fe-b477-42b3-91c6-901811fcba1d. Keine zusätzlichen Threads oder Doppelworker. Zustellungen und Status im zentralen REGISTER, Fachnachweise bei K.
