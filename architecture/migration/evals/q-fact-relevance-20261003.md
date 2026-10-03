status: aktiv
Datum: 2026-10-03

# Fakten-Relevanz für Paket Q

Prüfstand: `511a347b653beba13c2bf130f4bead7a7196cc2a`. Der vorhandene Kern verwendet eine inhaltliche Freigabeschwelle statt eines pauschalen BM25-Mindestwerts.

## Vertrag

`rust/crates/brain-kernel/src/fact_relevance.rs:117` wählt einen Fakt, wenn die Frage eine eindeutige Entität oder einen belegten Alias sowie ein konkretes Feld benennt. Provenienz ist erforderlich. Angefragte Patch- und Modusgrenzen müssen passen. Zahlen in der Frage müssen zum ausgewählten Feld passen. Mehrere Entitäten oder widersprechende Quellen führen zu keiner direkten Faktantwort; ein höherer Suchscore überstimmt diesen Konflikt nicht.

Die Auswahl wird in `rust/crates/brain-kernel/src/execution.rs` für das Faktenprofil verwendet. Eine fehlende Passung ergibt `insufficient_evidence`; der Antwortprovider wird für die direkte Faktantwort nicht benötigt. Die separate Claim-Schwelle in `dbrain-retrieval` ist kein Ersatz für diesen Vertrag.

## Ausgeführter Nachweis

```text
/home/nathanael/.cargo/bin/cargo +1.97.1 test --manifest-path /home/nathanael/.worktrees/brain-fertig-q/rust/Cargo.toml -p brain-providers -p brain-kernel -p dbrain-retrieval --locked --offline --jobs 2
```

Exit 0, vollständiges Log `.q-baseline-core.log`, beide Hostsperren über den gesamten Lauf gehalten. Compilerlauf 3m 19s. Gesamt: 152 passed, 0 failed, 15 ignored, 0 filtered. Die ignorierten Fälle wurden in diesem Lauf nicht ausgeführt.

Sechs vorhandene Relevanztests bestanden: Entität/Feld/Alias/Zahlen, Alias-Tokenizer, mehrdeutiger Alias trotz höherem BM25, Konflikt derselben Entität aus zwei Quellen, spezifisches Feld vor allgemeinem Feld und Patch-/Modus-/Provenienzpflicht. Diese Fälle sind technische Fixtures, kein aufgezeichneter Community-Verkehr.

TESTNACHWEIS[TW-1]: 152 passed, 15 ignored | Baseline: 0 rot

## Grenze

Dieser Nachweis belegt die vorhandene Relevanzentscheidung auf dem genannten SHA. Er belegt keinen Provider-Shadow, keinen neuen Quellenimport und keinen produktiven Cutover. Nach Integration wird der betroffene Testlauf auf dem endgültigen Paket-SHA wiederholt.
