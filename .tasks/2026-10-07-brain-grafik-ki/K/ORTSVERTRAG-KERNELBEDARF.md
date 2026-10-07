# K: konkreter Kernelanschluss des Ortsvertrags

status: zusätzliche Eigentumsnaht offen, 7. Oktober 2026

Auftraggeber 481426fe-b477-42b3-91c6-901811fcba1d. Exklusive K-Freigabe der beiden Vertragsdateien ist umgesetzt in Arbeit, nicht mit G geteilt. Zusätzlicher Writerbedarf außerhalb bisheriger K-Schreibbereiche:

## Belegter Ort

rust/crates/brain-kernel/src/flight.rs:154-172, cache_key_for_purpose serialisiert ein explizites Tupel mit Hauptzweck, Principal, Konversation, Release, Budget, Frage, Domäne, Profil, Patch, Modus und Scopes. Neuer Query.answer_context ist darin noch nicht enthalten. rust/crates/brain-kernel/src/cache.rs:67 verwendet dieselbe Naht für den Antwortcache. Flightpfad und Cache müssen denselben Ortsunterschied sehen.

## Begrenztes erforderliches Delta

In genau cache_key_for_purpose das tatsächliche &query.answer_context ins vorhandene Tupel aufnehmen. Keine neue Cachelogik, keine neue Engine, keine Änderung von Principal-/Release-/Budget-/Zweckbindung. Bestehende Cache-/Flighttests mit zwei sonst identischen Anfragen an unterschiedlichen Orten sowie None passend erweitern. G-Reasoner-/Toolport-/Deadlineaufgaben unverändert lassen.

Ohne diese Bindung können zwei ansonsten gleiche Anfragen bei derselben Konversation für verschiedene Orte denselben Schlüssel erhalten. Der neue strukturierte Payload allein repariert das nicht. K hat den Fund am aktuellen Quellstand unabhängig bestätigt, aber noch keine Kernelquelle geändert.

## Mechanische Literalbedarfe außerhalb bisheriger Writerfreigabe

Rust verlangt das neue optionale Feld auch in bestehenden Struct-Literalen; serde(default) macht die JSON-Anfragen kompatibel, ersetzt aber keine Compile-Time-Felder. Der Vertragswriter meldet folgende konkrete Fundstellen relativ zu rust/crates/. Minimaldelta ist answer_context: None, ohne andere Semantik. Bei Struct-Update-Syntax nicht unnötig doppeln. Fremde aktive Quellen bisher nicht angefasst.

- brain-contracts/src/bot_tasks.rs:319. Bereits das Vertragscrate benötigt dieses produktive Literal für den Compiler.
- brain-feeds/tests/match_store.rs:164,200 (Struct-Update bei 200 prüfen).
- brain-kernel/src/fact_relevance.rs:238; src/flight/c3_shared_validation.rs:85; src/flight/review_dependencies.rs:102; src/flight/review_publication.rs:68; src/lib.rs:277; tests/core_kernel.rs:42; tests/evidence_errors.rs:84; tests/patch_validity.rs:88; tests/pr61_dependencies.rs:107; tests/review_dependencies.rs:33.
- brain-legacy-import/src/import_firstparty.rs:480; brain-maintenance/tests/workflow.rs:761.
- brain-policy/src/lib.rs:258; tests/core_policy.rs:11.
- brain-providers/src/lib.rs:392; tests/faults.rs:15.
- dbrain-retrieval/src/contract_port.rs:185; src/entity_profile_port.rs:158; tests/chunked_retrieval.rs:63; tests/core_retrieval.rs:23; tests/knowledge_projection.rs:92; tests/patch_validity.rs:75.
- dbrain-sources/src/forum.rs:949; tests/entity_profiles.rs:2219; dbrain-wiki/tests/domain_projection.rs:131.
- deadlock-brain/src/bin/brain-mcp.rs:278,809,882; src/main.rs:1472.

Eigene API-/Client-/Serve-Literale korrigiert K im bisherigen Bereich. brain-maintain.rs:60/150 sind CLI-Enumvarianten, keine Contract-Query. G-Reasoner-/Deadline-/Toolportdateien nicht als pauschal freigegeben betrachten. Keine allgemeine Refactoringfreigabe erforderlich, aber die fremden mechanischen Zeilen müssen exklusiv einem Writer zugeordnet werden, bevor der gemeinsame Ruststand lieferbar ist.

## Bereits konkret gelieferte Signatur

Query.answer_context: Option<AnswerContext>, serde(default), None beim Serialisieren ausgelassen. AnswerContext ist platform-getaggtes enum mit snake_case Discord(DiscordAnswerContext) und Twitch(TwitchAnswerContext). Discord: optionale channel_name/category_name/topic/purpose/thread_name, is_thread/is_direct_message, input_kind. Twitch: optionale channel_name/is_partner/input_kind. AnswerInputKind: Message, Mention, SlashCommand. Sämtliche Angaben sind Daten, keine Rechte. Keine technischen IDs im Ortsblock. Aktuell uncommittet und ungeprüft, kein Vertragscheckpoint oder Livebeweis behauptet.
