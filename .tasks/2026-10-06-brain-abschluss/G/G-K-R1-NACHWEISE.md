# G-K-R1: Fehlerabrechnung und JSON-Schlussprüfung

status: gebaut und lokal geprüft, noch ohne Produktgate, 07.10.2026

Task `w7kbtvxfg`, Run `wf_18a32653-098`, Agent `a479471c9d2ac757f` tatsächlich abgeschlossen. Keine weiteren Vertrags-/Provider-/Kernel-Schreiber aktiv. Bereichsführung prüfte Quellbindung mit `sha256sum --check --status`: 50/50 unverändert, Exit 0. Manifest-SHA256 `01eba7f7d39cab050652285ae4a7498e30115e8618c011ab0fa3dc1fc8022141`.

## Kompatibler Vertrag

Bestehende Portsignaturen und PortError-Arten bleiben erhalten. Neue passende Methoden `answer_accounted`, `answer_turn_accounted`, `execute_accounted`, am Kernel `answer_for_publication_accounted`.

```rust
pub struct UsageAccounting {
    pub observed: Usage,
    pub reserved: Usage,
    pub unaccounted: bool,
}

pub struct PortFailure {
    pub error: PortError,
    pub accounting: Option<Box<UsageAccounting>>,
}

pub struct Accounted<T> {
    pub value: T,
    pub accounting: UsageAccounting,
}
```

Observed ist beobachteter Verbrauch. Reserved ist zusätzliche konservative Belastung, keine behauptete gemessene Tokenzahl; konfigurierte Kostenobergrenzen bleiben dort erkennbar. charged kumuliert beide Teile. Fehlende Fehlerabrechnung reserviert das ganze Restbudget, markiert unaccounted und beendet die Anfrage. before_call kennzeichnet belegte lokale Ablehnung vor einem Fremdaufruf. Kein zweiter Ledger, keine globale Nebenvariable und kein frisches Budget.

Provideranschluss im bestehenden transport.rs, Kernelverbrauch in execution.rs. Cache und Single-Flight tragen die Abrechnung; Fehler werden nicht gespeichert und Wiederverwendung belastet nicht doppelt. G-V muss tatsächliche Toolfehler über execute_accounted abrechnen und die getrennte Kernelabrechnung konsumieren.

## Belege

Beide Wireformen behalten trotz späterem Parser-/Validierungsfehler 17 Input, 7 Output und eine Netzrunde als gemessen. Fehlende oder mehrdeutige Usage bleibt reserviert. Vorheriger Providerturn, erfolgreicher Toolcall und späterer Providerfehler kumulieren am Kernel zu 32 Input, 30 Output, 4 Netzrunden. Fehler des zweiten Toolcalls erhält vorherige Erfolge: 22 Input, 25 Output, 3 Netzrunden; kein dritter Call. Folgeturnbudget und abgebrochene Deadline verlieren beobachtete Usage nicht.

Doppelte JSON-Felder, Call-ID-Bindung und unzitierte Quellenprüfungen bleiben erhalten. Die letzte G-P-R1-Ergänzung ist nun in den tatsächlichen Vertrags-/Provider- und Source-Läufen enthalten; der vorherige offene Compiler-/Teststand ist damit ersetzt.

## Tatsächliche Prüfung

Vollständige Befehle in `G/pruefungen/g-k-r1/cargo-commands.log` und commands.log. Bestehender Buildslot, Debugtarget `/tmp/brain-g-k-20261007-target`, --locked --offline --jobs 3; Tests --include-ignored --test-threads=1.

| Prüfung | Exit | Ergebnis |
| --- | ---: | --- |
| Format | 0 | Eigene Dateien und vorhandener JSON-Eingang |
| Compiler, drei Zielcrates, alle Targets | 0 | Contracts, Provider, Kernel |
| Striktes Clippy, alle Targets | 0 | -D warnings |
| Verbraucherkompilierung | 0 | Serve, Retrieval, Sources gegen damaligen gemeinsamen WIP |
| Contracts und Provider vollständig | 0 | 113 passed, 0 failed, 0 ignored |
| Drei Zielcrates vollständig | 101 | 170 passed, 18 failed, 0 ignored |
| Abrechnungsfälle als Teilmenge | 0 | 17 passed, 171 filtered; nicht zur Gesamtsumme addiert |
| Sources-JSON-Suite | 0 | 6 passed, 0 ignored, 1 Livefall ausdrücklich ausgeschlossen |

Bereichsführung bestätigte die tatsächlichen Testmarker. 113 ergibt sich aus 39, 23, 11, 11 und 29 bestandenen Fällen. Kernel separat 57 passed/18 failed, zuvor 49/18, ursprüngliche Baseline 34/18. failure-baseline.json enthält identische 18 Fehlernamen, added/removed leer. Gesamtsuite bleibt rot, keine grüne Gesamtbehauptung. Der erste eigene Baselineanlauf startete mangels Buildslot keinen Cargo-Prozess.

## Grenze

Keine Git-, Dienst-, ENV-, Modell- oder Timeoutänderung durch Worker. Echter Dispatcher, E-Receipt/Herkunft und Luna-Probe weiter offen. Produktänderungen nach kohärenten Teilpaketen durch regulären Gate, bei BLOCK frischer Fixer. Gemergt und live noch nicht.

TESTNACHWEIS[TW-1]: 170 passed, 0 ignored | Baseline: 18 rot
