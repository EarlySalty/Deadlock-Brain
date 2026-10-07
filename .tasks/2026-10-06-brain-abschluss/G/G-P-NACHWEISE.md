# G-P: Transportstand und offene JSON-Abnahme

Stand: 07.10.2026. Workflow `wf_9c5666f8-dc6`, Task `w5pqkkyxv`, abgeschlossen. Produkt-WIP noch uncommittiert; kein Gate, keine echte Luna-Brücke und keine Runtimewirkung.

## Tatsächlich gebaut

`brain-providers/src/{lib.rs,transport.rs,hardening.rs}` und `brain-providers/tests/faults.rs` erweitert. `answer_turn` für beide bestehenden Provider, bisheriges `answer` erhalten. Native Toolschemas, Toolblöcke und Ergebnisnachrichten sowie OpenAI-kompatible Toolcalls und nullable Content über gemeinsame Vertragsrenderer. Keine feste Toolliste; Acht-Tool-Fassung einschließlich Boonbereich und Analytics konsumiert.

Gebundene Call-IDs und Finishgründe, finale JSON-/Zitatprüfung, ursprüngliche gebundene Deadline auch in Folgeturn und Retry. Kumulierte Eingabe-/Cache-Usage, Netzrunden und konfigurierte Kostenobergrenze; fehlgeschlagene Retries konservativ belastet. Kein Providerwechsel, Proxyumbau, Liveprovider oder Secretzugriff.

## Bestätigte Grenze

Beide Parser akzeptieren doppelte JSON-Argumentnamen und behalten den letzten Wert. Originalprobe `/tmp/brain-g-p-json-probe.rs`; Log `G/pruefungen/g-p/json-probe.log` durch Bereichsführung gelesen:

```text
native=false: duplicate query accepted, retained="B"
native=true: duplicate query accepted, retained="B"
```

Ein grüner Testlauf bestätigt diese Sicherheitsgrenze nicht als korrekt. Der Produktstand wird vor der Korrektur nicht abgenommen oder committed. Vorhandenen duplikatsicheren Parser einmal in den gemeinsamen Vertragsbereich verlegen; bisheriger Source-Eingang bleibt kompatibel und delegiert. Frischer begrenzter Worker gemäß `G/BRIEFING-G-P-R1.md`, keine neue Parserstrecke.

## Nachgeprüfte Belege

`G/pruefungen/g-p/`: Format, Abschluss-Clippy, Abschlusstest und Verbraucherkompilierung jeweils Exit 0. Bereichsführung las die Exits und tatsächlichen Testmarker; 11 Unit- und 18 Integrationsfälle bestanden, 0 failed, 0 ignored, 0 filtered. Baseline 11 Unit- und 8 Integrationsfälle, jeweils 0 failed und 0 ignored. Doc-Tests enthalten 0 Fälle. Alle sieben Quellenfingerprints in `source.sha256` zum Übernahmezeitpunkt unverändert. Nachfolgende Korrektur braucht eigene Prüfungen und neue Fingerprints.

Die vollständigen Abschlussbefehle stehen in der nativen Rückgabe. Sie verwenden `--locked --offline --jobs 2`, den bestehenden Buildslot und Debugtarget `/tmp/brain-g0-r1-target`; Test zusätzlich `--include-ignored --test-threads=1`. Verbrauchercheck: Provider, Kernel, Retrieval und Serve mit allen Targets. Gemeinsamer WIP-Verbrauchercheck ersetzt keine getrennte G-K-Abnahme.

TESTNACHWEIS[TW-1]: 29 passed, 0 ignored | Baseline: 0 rot
