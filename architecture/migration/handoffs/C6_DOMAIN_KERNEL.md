# C6 — Domain-/Kernel-Verdrahtung

Stand: 26.09.2026. Branch: `codex/fix-c6-domain-kernel-wiring`.
Ziel des PR: `migration/rust-integration`. **Nicht mergen, nicht deployen.**

## Basis und Abhängigkeiten

Der frisch geholte Integrationsbranch steht auf `087c522deda58ecf4bd6843167f51c54f681e944`. C4 ist noch nicht integriert: PR #43 enthält `ea2bd15d65df57e0d56069f3d4f7e0a8cf79c78e`. Dieser C6-Branch baut ausdrücklich auf diesem C4-Commit auf. Der PR gegen den Integrationsbranch enthält deshalb bis zur separaten C4-Integration auch dessen Vorläufercommit. Keine Änderung am Zielbranch und kein eigenmächtiger Merge.

C2/C3, PR #45, Head `f0cd5add9642c1527cda475a168e883a34044bae`, ist ebenfalls noch offen. C6 ändert weder das Default-Providerbudget noch das bisherige Prosa-Retrieval, um dessen bekannte Budgetfehler zu verdecken. Die vollständige Default-Pilotabnahme nach gemeinsamer Integration bleibt eine gesonderte Prüfung; Einzelbranch-Ergebnisse werden nicht als Kombinationsnachweis ausgegeben.

## Tatsächlicher Antwortpfad

```text
BrainClient / POST /v1/answer
  → Query.domain (optionaler typisierter Intent)
  → vorhandener ReleaseRetriever
  → DomainReader / kanonischer Release-Snapshot
  → gemeinsame HeroKnowledgeCard, NumericFact, TypedRule, BuildCatalogRef
  → CoreRuleEvaluator ODER bestehendes Reasoner-Inventar
  → neu berechneter, revisionsgebundener DomainAnswer-Evidenznachweis
  → Kernel: Fact | Rule/Build | generative Card-Erklärung
  → öffentlicher Antwortvertrag mit auflösbaren Beleglabels
```

Es gibt keine neue Engine und keine neue Contract-Crate. `brain-contracts` wird erweitert; `dbrain-builds::CandidateSet`, `dbrain-reasoner::CoreRuleEvaluator`, `Inventory`, `InventoryRules` und `ItemModel` werden weiterverwendet. Der bestehende ReleaseRetriever aktiviert den Pfad automatisch; ein separater, nur in Tests verkabelter Retriever ist nicht nötig. Der von C1 bereitgestellte Composition Root muss weiterhin diesen ReleaseRetriever verwenden. C1 wird hier nicht nachgebaut.

### Verträge und Kompatibilität

`brain.domain.v2` ergänzt die bestehenden NumericFact-/TypedRule-Objekte um `HeroCard` und `BuildCatalog`. Historische `brain.domain.v1`-Facts/-Rules bleiben lesbar und werden nicht umgeschrieben. V2-Objekte in einem V1-Envelope und unbekannte zukünftige Versionen werden abgewiesen. Es ist keine SQL-Migration nötig; die Objekte liegen als normale revisionsgebundene SourceRecords im vorhandenen Store.

`Query.domain` ist optional und wird bei `None` nicht serialisiert. Bestehende Textanfragen behalten ihre Wire-Darstellung. `DomainRequest` kennt Fact, Rule, Build und Card; verschachtelte unbekannte Felder werden nicht akzeptiert. Der zusätzliche Antwortstatus `build_rejected` bezeichnet eine **belegt illegale** Kaufreihenfolge. Er verlangt wie `answered` Text und Zitationen. Strikte ältere Consumer müssen die neue Enum-Variante vor gemeinsamer Aktivierung kennen; dies ist kein behaupteter rückwärtskompatibler Enum-Rollout an bereits laufende Bots.

Beispiel einer reproduzierbaren, ausdrücklich synthetischen Pilotanfrage:

```json
{
  "request_id": "c6-example",
  "conversation_id": "c6-example-conversation",
  "text": "Prüfe diese Kaufreihenfolge.",
  "profile": "build",
  "requested_scopes": [],
  "patch": "pilot-20260925",
  "mode": "ranked",
  "domain": {
    "kind": "build",
    "hero": "Fixture Hero",
    "locale": "en",
    "catalog_id": "fixture-catalog",
    "items": ["101", "102", "103"]
  }
}
```

Der Client ermittelt nicht selbst eine Freigabe: Principal, Knowledge-Release, Rechte und Budget kommen weiterhin aus dem serverseitigen Policy-Kontext. Patch und Mode sind Pflicht für den Domainpfad; es gibt kein stilles `current`/`latest`. Ein Buildprofil ohne vollständigen typisierten Intent fällt **nicht** auf generative Prosa zurück.

## Daten, Provenienz und Gültigkeit

`DomainKnowledgeCard` umschließt die vorhandene `HeroKnowledgeCard` mit exakter Gültigkeit, Source-/Parserrevision, Locators, Abhängigkeiten, sprachgebundenen Aliasen, unverlusteten C4-IR-Feldern, Effects und Unknowns. Die vorhandene Wiki-Projektion wird durch `project_domain_card` weiterverwendet. Ungeprüfte Revisionen, Bedingungen und Varianten bleiben erhalten und werden nicht zu unbedingten Facts abgeflacht. Effects, beschreibende Rules und Synergies sind Quellenwissen, keine automatisch ausführbaren Formeln.

`BuildCatalogRef` verweist per `DocumentRevision` und JSON-Pointer auf **vorhandene eingefrorene Reasoner-Modelle und Inventarregeln**. Er enthält zusätzlich die originalen Abhängigkeiten, einen Review-Verweis, die verifizierte aktive-Slot-Regel und ein ausdrücklich vorhandenes `unknown_legality_fields`. Keine zweite Item-/Hero-Statstabelle wird gepflegt. Ein normalisierter möglicherweise voreingestellter Preis 0 oder ein fehlendes/null Legalitätsfeld darf nicht als belegter Kaufpreis dienen. Ein tatsächlich bekannter statischer Zahlenwert 0 bleibt dagegen ein bekannter Wert.

Der Producer/Reviewer muss diese eingefrorenen Modelle und ihre Originalquellen in denselben Release pinnen. C6 aktiviert keinen automatischen Export aus einer unversionierten Live-Datenbank und erfindet keine Freigabe für ungeprüfte Spieldaten. Die Tests prüfen diesen bestehenden Modellvertrag; sie behaupten keinen aktuellen, vollständig abgenommenen Live-Itemkatalog.

Der Reader prüft sowohl die Freigabe des abgeleiteten Objekts als auch **jede** referenzierte Roh-, Alias-, Fact-, Rule-, Effect- und Synergy-Quelle. Historische ACLs und aktuelle Heads wirken gemeinsam. Der resultierende Beleg trägt die strengste Sichtbarkeit und die Vereinigung der benötigten Scopes. Aktuelle Tombstones, Quellenwiderrufe und Egress-Sperren bleiben auch bei alten Releases wirksam. Eine öffentliche Karte kann eine private Quellabhängigkeit nicht freigeben.

C4-`OriginArtifact` ist für versionierte Quellen maßgeblich. Das historische Wiki-Metadatenfeld kann den JSON-Text `null` enthalten; dies ist kein Patchname. Unbekannte Ursprungsgültigkeit wird nicht erraten: Sie benötigt die ausdrückliche geprüfte Domainprojektion. Widersprechende **bekannte** Source-Patch-/Mode-Gültigkeit kann diese Projektion nicht überschreiben.

Jede Domain-Evidenzvalidierung berechnet das Ergebnis erneut aus kanonischen, aktuell autorisierten Daten. Ein selbst gesetztes Fact-/Rule-Label oder ein manipulierter JSON-Nachweis genügt nicht. Auch Cache und Single-Flight verwenden den typisierten Intent als Identitätsbestandteil und revalidieren belegte Antworten. Der öffentliche API-Text referenziert `Beleg 1`; Rohpfade, Parsermetadaten und ACLs werden nicht als öffentliche Zitationsdaten herausgegeben.

## Deterministische Semantik und Grenzen

Factantworten geben exakt gespeicherte, bekannte Werte einschließlich Einheit aus. Ruleantworten benutzen den bestehenden verifizierten, einheitenprüfenden CoreRuleEvaluator und enthalten Rule-/Evaluatorversion sowie die verwendeten Fact-IDs und Originalrevisionen. Nicht auswertbare oder fehlende Eingaben ergeben Unknown/`insufficient_evidence`, nicht 0.

Buildantworten prüfen eine **geordnete Kaufsequenz ohne Verkäufe**. Die vorhandenen Inventarübergänge übernehmen Kaufbarkeit, deaktivierte Items, Duplikate, Upgrade-Komponenten und deren Kostenanrechnung, Gesamtplätze und aktive Plätze. Das aktive Limit kommt aus einem verifizierten Fact statt aus einer im Adapter neu gepflegten Spielkonstante. Frühere Reasoner-Aufrufer behalten ihr bestehendes Verhalten. Unknown-Hero, Unknown-Item, Aliasmehrdeutigkeit, fehlende Regeln und unvollständige Quellfelder werden nicht als bewiesen illegal oder kostenlos ausgegeben. Ein belegter Regelverstoß liefert `build_rejected` mit konkretem Grund und Referenzen auf die geprüften Eingaben.

Dies ist keine Behauptung, dass der Build optimal ist. Zielprioritäten, Gegnerkonfiguration, Verkaufsstrategien und nicht belegte Bedingungen werden nicht hinzuerfunden. Deterministische Fact-/Rule-/Buildantworten benötigen **keinen Provider**, keine Provider-Netzwerkrunde und kein Input-/Kostenbudget. Nur die gewünschte generative Kartenerklärung benutzt den normalen, gesondert autorisierten Providerpfad.

Die Aliasauflösung ruft den bestehenden `dbrain-normalize::normalize_alias` gemäß ADR S05 auf. DE/EN-Zuordnungen bleiben quellen- und sprachgebunden. Unterstriche, Whitespace und die bestehenden S05-Trennzeichen werden gleich behandelt; keine Fuzzy-/Prefix-Heuristik, kein `ß`→`ss`-Casefold und keine stillschweigende EN→DE-Umbenennung. Mehrdeutige Aliase werden nicht nach erstem Treffer aufgelöst.

## Regressionen und Pilot

Neue aktive Tests liegen in `brain-api/tests/domain_kernel.rs`, `dbrain-wiki/tests/domain_projection.rs` sowie in Kernel-/Storage-Unit-Tests. Der gemeinsame synthetische Producer liegt nur unter `brain-api/tests/support/domain_fixture.rs`.

Der tatsächlich beobachtete Rotnachweis: `build_never_falls_back_to_generative_prose` lieferte vor der Änderung `Answered` statt `InsufficientEvidence`, obwohl nur Prosa vorhanden war. Derselbe Test wird durch die neue Build-Sperre grün. Die zusätzliche echte Wiki-Extractor→Projektion→Store→Kernel-Gegenprobe deckte die `null`-Metadaten-/Origin-Verwechslung auf; die Lösung ändert nicht die Quellenrechte.

Die Regressionen umfassen legale Upgradesequenz, illegales Duplikat, Platz-/Aktivlimits, unbekannte Entities, Patch/Mode, zwei Hero-Revisionen, DE/EN/S05-Grenzen, Dezimalwerte und echte Null, Rule-Erklärung, unbekannte Felder trotz Normalisierungsdefaults, Aliasmehrdeutigkeit, widersprüchliche Source-Gültigkeit, ACL-Widerruf einschließlich Cache, Delete, manipulierte Belege, Egress, öffentliche Zitationslabels und V1-Lesekompatibilität.

Der bestehende lokale PostgreSQL-/HTTP-Pilot enthält nun zusätzlich `legal_build` und `illegal_build`. Beide importieren denselben synthetischen Domain-Producer über `SourceBatch`/`PgStore`, pinnen die echten Dokumentrevisionen, überleben den Scratch-DB-Crash/Restart und werden über den tatsächlichen BrainClient/API-Pfad geprüft. Pro Fall sind Status, Begründung, Zitationen und **exakt null Provideraufrufe** Pflicht. Die vorhandenen 15 Dokumentfälle bleiben erhalten, insgesamt also 17. Ein leerer Scratch-Rebuild verwendet dieselben Daten; Batch-Replays werden nicht als neu geschriebene Records gezählt.

**Messgrenze:** Die bestehenden lokalen Dokumente sind echte Pilotdokumente. Die neuen Domain-Eingaben sind klar benannte synthetische Fixtures, keine bestätigten aktuellen Deadlock-Stats. Der Provider ist der vorhandene kontrollierte Loopback-Stub. Weder Live-Spielvalidität noch Qualität echter Modellantworten oder Produktionsbetrieb werden daraus abgeleitet.

## Prüfprotokoll

Abschließend geprüfter Code-/Test-Commit: **`01c6142f81e61e4d322db2a073dff5faf38b2262`**. Danach folgt nur dieser Handoff. Alle fünf Workspacebefehle wurden nach der letzten Pilotkorrektur vollständig erneut ausgeführt; die finalen Logs heißen `acceptance-*.log` mit separaten `acceptance-*.exit`-Dateien.

| Prüfung | Tatsächliches Ergebnis |
|---|---|
| `cargo fmt --all -- --check` | Exit 0 |
| `cargo check --locked --workspace` | Exit 0 |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | Exit 0 |
| `cargo test --locked --workspace` | Exit 0; **872 bestanden, 0 fehlgeschlagen, 67 bestehende Ignore-Markierungen**, 75 Test-/Doc-Test-Suiten |
| `cargo build --locked --workspace --release` | Exit 0 |
| Neue aktive C6-Regressionen | **26**: 20 Domain-API/E2E, 2 Wiki-Projektion, 3 Kernel, 1 V1-Reader-Kompatibilität; kein neuer Ignore |
| Default-Pilot ohne Budget-/Limit-Overrides | **Exit 1; 12/17 Fälle bestanden**, Inputbudget 12.000, Retrieval-Limit 6 |
| `legal_build` im PostgreSQL-/HTTP-Pilot | Bestanden; `answered`; **0 Provideraufrufe**, keine Provider-Egressdaten |
| `illegal_build` im PostgreSQL-/HTTP-Pilot | Bestanden; `build_rejected`; begründetes Duplikatverbot; **0 Provideraufrufe**, keine Provider-Egressdaten |
| Import und leerer Scratch-Rebuild | Beide Exit 0; jeweils 18 Dokumentrevisionen |
| Crash/Restart und deterministischer Snapshot | Import, nach Restart und leerer Rebuild haben denselben untenstehenden Digest |
| Unveränderter Reimport / Delete | 0 neue Records; 1 getesteter Tombstone; aktuelle ACL-/Delete-Fälle bestanden |

Die **fünf weiterhin fehlgeschlagenen Defaultfälle** sind `public_question`, `exact_number`, `alias_en`, `alias_de_lowercase` und `provider_error`; alle liefern `budget_exceeded`, bevor ein Provider aufgerufen wird. Das ist die bekannte C2-Grenze des unveränderten Ganzdokument-Prosa-Retrievals. Die separaten **typisierten** Domain-Alias-/Zahlenfälle sind grün. Der Default-Pilot wird ausdrücklich **nicht** als Gesamtfreigabe ausgegeben. C2/C3 wurde weder still übernommen noch mit einem größeren Testbudget umgangen.

Gemeinsamer Snapshot-Digest der drei Zustände:

```text
b8995efcbe7b0c4d070537a5516160547532446f58e07930d3025e462e17d32e
```

Zusätzlicher beobachteter und behobener Zwischenbefund: Der erste neue Fixture-Importer beanspruchte für einen bereits bestätigten Batch eine neue Lease. Der bestehende idempotente Receipt-Replay beendet diese neue Lease nicht; beim nachfolgenden Delete-Job trat `source already leased` auf. Commit `01c6142` lässt neue Jobs deshalb wie den vorhandenen FileConnector ihren Checkpoint fortschreiben und bei unverändertem Inhalt **keine neuen Records** liefern. Keine Sleeps, künstlichen Lease-Verkürzungen oder Änderung der produktiven Store-Semantik. Die fehlerhafte Gegenprobe ist lokal unter `.core-test-logs/c6/pilot-lease-counterprobe/` erhalten; der abschließende Pilot unter `.core-test-logs/pilot/` hat keinen Leasefehler. Der Scratch-Postgres wurde durch den Script-Trap ordnungsgemäß beendet.

Ausführung im eigenen Worktree, isoliertes Test-HOME, `SQLX_OFFLINE=true`, `CARGO_BUILD_JOBS=2`, ohne Anwendungs- oder Produktions-DB-Zugangsdaten. Die Pilotdokumente wurden nach `/tmp/brain-c6-pilot-20260926` kopiert; die Originaldaten bleiben unverändert. PostgreSQL verwendet ausschließlich `.core-test-pg` dieses Worktrees und einen Unix-Socket, keine bestehende Instanz.

Reproduktion der Pflichtprüfungen aus `rust/`:

```sh
cargo fmt --all -- --check
cargo check --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo build --locked --workspace --release
```

Pilot aus dem Repository-Root mit einer eigenen Kopie des genehmigten lokalen Dokumentkorpus außerhalb des Repositories:

```sh
BRAIN_PILOT_ROOT=/absoluter/pfad/zur/eigenen/pilotkopie scripts/run_local_pilot.sh
```

Keine Budget-/Retrieval-Limit-Overrides für die Defaultprüfung. Lokale Ausgaben: `.core-test-logs/c6/` und `.core-test-logs/pilot/`; keine Rohtexte, Secrets oder Datenbankdateien im PR.

## Übergabe an die gemeinsame Integration

Zuerst C4 separat reviewen/integrationsseitig übernehmen. Beim anschließenden Zusammenführen mit C2/C3 müssen Domain-Dispatch und kanonische Domain-Evidenzvalidierung im neuen ReleaseRetriever erhalten bleiben; die C2/C3-Fehlerklassifikation darf nicht auf den älteren Stand zurückgesetzt werden. `Query.domain` muss im dortigen Cache-/Single-Flight-Key verbleiben. Die Enum-Ergänzungen `build_rejected` und `unavailable` sind kumulativ, keine Alternativen. Danach die **gemeinsamen** Workspacechecks und den Default-Pilot mit allen 17 Fällen erneut ausführen.

Keine Produktionskonfiguration geändert, kein Bot neu gestartet, keine Nachrichten gesendet, keine Deployment-/Merge-Aktion ausgeführt. `GATES.csv`, `STATUS.md` und historische Abnahmen werden durch diesen C6-Nachweis nicht eigenmächtig umgeschrieben.
