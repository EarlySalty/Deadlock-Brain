# G-K-R3: deterministischer Buildprüfanschluss

status: Fixdelta abgeschlossen, Produktionsadapter offen, 07.10.2026

Frischer Fixer `wa1nunog0` / `wf_cc12e1e5-662` / `ab7ecc9619f5f56c0` tatsächlich zurückgegeben. Start `3242fb36ed847dc06ef71d92c71108614b5d493f`, eigener Commit `b352472fbe75429a221cefe5a59133f41bd35520`. Geändert: contracts/tools.rs und Kernel-lib.rs, execution.rs, outcome.rs, flight.rs. Kein Push, Main, Runtime oder echter Provideraufruf.

## Wirkung und Grenze

Der vertrauenswürdige Toolport prüft das tatsächlich ausgeführte BuildPlan-Ergebnis mit typisierter Anfrage, Pin, Ausgabezweck und ursprünglicher Deadline. Ohne Implementierung verweigert der kompatible Standardanschluss. Cache und Flight behalten das Ergebnis und verlangen die erneute Prüfung. Gewöhnliche Belege und eine bloße BuildPlan-Definition reichen nicht als Buildnachweis. G-K-R2s Zwecktrennung bleibt erhalten.

Positive Anschlussfälle verwenden Kernel-Fakes. Fs echter Planer und der G-V-Produktionsadapter sind noch nicht angeschlossen. Das Delta-ALLOW ersetzt weder deren Abnahme noch die gemeinsame Provider-/Kernelprüfung.

## Tatsächliche Prüfungen

Befehle und Exits: `G/pruefungen/g-k-r3/commands.log`. Kontrolliertes Format, Compiler und striktes Clippy für Contracts, Kernel, Providers, Sources und Serve jeweils Exit 0. Vollständige Suites mit `--include-ignored --test-threads=1`:

- Contracts: 73 passed, 0 failed, 0 ignored.
- Kernel: 71 passed, 18 failed, 0 ignored; vorher 61 passed, 18 failed. Fehlernamensvergleich ohne hinzugekommene oder verschwundene Namen.
- Zusammen: 144 passed, 18 failed, 0 ignored, 0 filtered; Exit 101.
- Verbraucher: 414 passed, 14 failed, 0 ignored, 0 filtered; Exit 101. Keine passende Vorher-Baseline für diesen Gesamtlauf. Provider darin 44 passed, 0 failed.

Die Bereichsführung prüfte die Rohmarker: `buildschutz-final.log` ist ein früherer Acht-Fälle-Lauf. Der anschließend tatsächlich gelesene letzte Lauf `buildschutz-verified.log` enthält 10 passed, 0 failed, 0 ignored und insgesamt 79 filtered. Die zwei zusätzlichen Fälle prüfen den erhaltenen deterministischen Domainweg sowie unzitierte Quellen, Pin und Zweck nach dem Finalturn. Acht und zehn wurden nicht gleichgesetzt.

Quellmanifest `sources.sha256`: 423/423 durch Bereichsführung erneut geprüft, Exit 0. Manifest-SHA256 `103c6ab44bc012a99de79669355715e2c58a27eb450f636775d6f92c1194f56c`.

TESTNACHWEIS[TW-1]: 144 passed, 0 ignored | Baseline: 18 rot

## Regulärer Fixgate

`gate.log` und `gate.exit` tatsächlich gelesen: Exit 0 auf `3242fb36..b352472f`, Antwort `[gpt-6.1-sol] ALLOW: No blocking defect established by the supplied code.` NIT an tools.rs:918: Produktionsintegration unbewiesen, bisher Testoverrides von `validate_build_plan`. Ohne echten F/G-V-Anschluss bleibt ein erfolgreicher BuildPlan gesperrt.

Der anschließende gemeinsame Gate `a6568629..b352472f` lieferte einen neuen BLOCK zur fehlenden Weitergabe angesammelter Toolbelege an die konkreten Provider. Dieser steht in `G/REVIEW.md` und geht an einen weiteren frischen Fixer. Das vorliegende Delta-ALLOW ist kein gemeinsames ALLOW.

## Verbrauchergrenze

Zwei konkrete Serve-Fehler aus consumers.log nachgelesen: der große Panel-/Dokufall scheitert mit `InvalidResponse("invalid chat schema")`; der öffentliche Livefaktenfall scheitert am vorhandenen Infisicalresolver mit `SecretSource`. Für die übrigen zwölf Fehler und einen Vorhervergleich steht die Auswertung aus. Keinen dieser vierzehn Fälle pauschal als vorbestehend behaupten.

MERGEPROTOKOLL[MS-1]: 2 Git-Schritte einzeln | Anläufe: 0 | Gate: [gpt-6.1-sol] ALLOW für Buildschutzdelta; gemeinsamer Gate BLOCK; kein Main-Merge
