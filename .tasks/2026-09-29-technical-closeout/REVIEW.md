status: aktiv
Datum: 2026-09-29

# Review-Register

Kein Paket ist bisher zur Integration freigegeben. Ein nicht gestarteter oder ignorierter Test zählt nicht als bestanden. Alle späteren Urteile müssen den exakt geprüften Head und die Basis nennen.

## Bisherige Prüfungen

- Koordinationscommit 80129bd gegen origin/migration/rust-integration: lokales gate_hook.py --review --codex-astra, Exit 0, Ausgabe `ALLOW: no reviewable changes`. Das ist KEIN inhaltliches unabhängiges Code-Review.
- PR #57 ist Draft. Neue CI-Läufe 36550365750 und 36550365924 scheitern beim Bezug von haste_core bfb292d4798031350861ad297aa26753267a1ea6 aus einem Repository mit HTTP 404. Kein Billing-Befund. Paket G repariert die Reproduzierbarkeit bei unveränderten Pins.
- Keine Branch Protection und keine Branch Rules für migration/rust-integration von GitHub gemeldet. Diese Beobachtung hebt weder Nutzergrenzen noch lokalen Merge-Gate noch die unabhängige Reviewpflicht auf. Kein --admin/Policy-Bypass.

## Annahmekriterien für die unabhängigen Reviews

Auth, ACL und Account-/Public-Scope; Leak-Schutz in beiden Richtungen; fail closed bei unbekannten Modi, Drift oder widersprüchlicher Evidenz; stale state; Parallelität und persistente Idempotenz; isolierte PostgreSQL-Pools und Recovery; Unknown bleibt Unknown; Provenienz/Raw-/Schema-/Parserpins; Patch-/Modebindung; keine versteckten Legacy-/LLM-/DB-Fallbacks; keine DL-Main-Credentials oder Analytics-Duplikation; keine Tests abgeschwächt; keine falschen Deploy- oder Freigabeannahmen.

Für A zusätzlich echter API-Vertrag und tatsächliche Store-/Release-/Runtime-Verdrahtung. Für B/D keine sichtbare Verzögerung durch Shadow-Probes. Für C finale 18-Fall-E2E plus benannte Fault-Fälle und 600/8, 600/16, 600/32 bei Pool 4. Für E Steam-Transaktion und Neustart, Patchnotes-Ordering/Limits. Für G gleiche geprüfte Quellrevisionen/Lizenzen und Erwerb ohne lokalen Git-Cache.

## Runden

Runde 1: frischer unabhängiger großer Reviewer, vollständige konkrete Mängelliste.
Runden 2 bis 4: mittel gegen bestätigte Liste.
Runde 5 falls nötig: großer Reviewer erneut.
Fixes an Luna bei klar kleinen Befunden, größere Integrationsfixes an Sol. Kein eigener Implementierer als unabhängiger Reviewer. Ein Gate-BLOCK wird behoben, nicht übersteuert.
