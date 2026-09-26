# DB Pooling / Backpressure Handoff

Stand: 2026-09-26
Branch: `codex/fix-brain-db-pooling`
Base: aktueller `origin/migration/rust-integration` nach Integration der offenen Welle-1-Arbeit, insbesondere C1/`brain-serve` und C2/C3 Retrieval.

## Ziel

Das beobachtete PostgreSQL-Connectionproblem im echten Requestpfad ist architektonisch behoben. Vorher existierten zwei voneinander unabhängige Connection-Lebenszyklen:

1. ein begrenzter SQLx-Pool im C1-Composition-Root und
2. pro Operation neu erzeugte synchrone `postgres::Client`-Verbindungen in `LocalPgReader`.

Damit konnten `read_snapshot`, C2/C3-`read_heads`, Conversation Ownership, Evidence Validation und Readiness außerhalb des SQLx-Limits neue PostgreSQL-Sessions erzeugen.

Der Requestpfad verwendet jetzt genau einen gemeinsam geklonten, hart begrenzten `LocalPgReader`-Pool. Es gibt kein `Client::connect()` pro Request/Operation mehr.

## Architektur

```
brain-serve
  -> LocalPgReader (ein gemeinsamer Pool pro Serviceprozess)
       -> max_connections (hart)
       -> bounded acquire wait
       -> wiederverwendete postgres::Client Sessions
       -> PostgreSQL
```

Der Pool reserviert einen Slot **vor** dem eigentlichen Connect. Auch parallele Connection-Erzeugung kann deshalb das konfigurierte Limit nicht überschreiten.

Die Poolinstanz wird geteilt von:

- Startup Core-Schema-Preflight
- Startup DB-Permissions
- `read_snapshot`
- C2/C3 `read_heads`
- Retrieval-Indexaufbau
- Evidence Validation / Cache-Revalidation / Shared-Flight-Revalidation
- `ConversationOwnershipPort::claim_conversation`
- `/readyz`

Readiness besitzt zusätzlich weiterhin einen einzelnen Probe-Slot, erzeugt aber keine eigene DB-Verbindungsfamilie.

## Konfiguration

`postgres.max_connections` bleibt das harte Connection-Limit.

Neu:

```json
"timeouts": {
  "postgres_pool_wait_ms": 250
}
```

`postgres_pool_wait_ms` begrenzt ausschließlich das Warten auf einen freien Pool-Slot. Poolerschöpfung erzeugt `PortError::Unavailable`.

Der API-Policy-Pfad mappt technische Ownership-/Pool-Ausfälle auf eine typisierte `AnswerStatus::Unavailable`-Antwort. Nur echte Policy-/Evidence-Denials dürfen `UnauthorizedEvidence` erzeugen.

## Connection Lifecycle

Ein Checkout verwendet zuerst eine idle Connection. Nur wenn keine idle Connection existiert und `open + connecting < max_connections` gilt, wird eine neue Session aufgebaut.

Beim Drop:

- gesunde Session -> zurück in idle Pool,
- geschlossene/defekte Session -> aus `open` entfernt,
- wartende Requests werden geweckt.

Damit kann der Pool nach einer DB-Störung neue Sessions innerhalb desselben Limits aufbauen.

Erfasste Runtime-Metriken beim geordneten Shutdown:

- `max_connections`
- `open_connections`
- `connecting_connections`
- `idle_connections`
- `checked_out_connections`
- `peak_connections`
- `created_connections`
- `reused_checkouts`
- `wait_count`
- `wait_timeout_count`
- `wait_total_micros`
- `wait_max_micros`

Die Logs enthalten keine DSNs, Passwörter oder DB-Fehlertexte.

## Scratch-PostgreSQL Lastnachweis

Der dedizierte Prozess-E2E in `scripts/test_brain_serve.sh` startet einen lokalen Unix-Socket-Scratch-Cluster mit:

- PostgreSQL `max_connections=12`
- kein TCP Listener
- `brain-serve postgres.max_connections=4`
- `postgres_pool_wait_ms=150`
- echter `brain-serve`-Child-Prozess
- echter `BrainClient`
- Loopback Provider Fixture

Nach Refresh auf den aktuellen Integrationsstand bestand der Lauf vollständig.

### 600 Requests, 8 Worker

- answered: **600**
- unavailable: **0**
- unauthorized_evidence: **0**
- client errors: **0**
- beobachtete Reader-Connections: **4**
- elapsed: **728 ms**

### 600 Requests, 16 Worker

- answered: **600**
- unavailable: **0**
- unauthorized_evidence: **0**
- client errors: **0**
- beobachtete Reader-Connections: **4**
- elapsed: **928 ms**

### 600 Requests, 32 Worker

- answered: **600**
- unavailable: **0**
- unauthorized_evidence: **0**
- client errors: **0**
- beobachtete Reader-Connections: **4**
- elapsed: **671 ms**

Das Testskript prüft das PostgreSQL-Log explizit auf `too many clients already`; der Lauf war grün, also **0 Treffer**.

## Pool-Wait / Reuse

Gemessene Poolstatistik aus demselben E2E:

- hard max: **4**
- peak connections: **4**
- created connections: **5** insgesamt, inklusive absichtlicher Fault-/Recovery-Phase
- reused checkouts: **9007**
- wait count: **5416**
- wait timeout count: **13** (1 absichtlich provozierter Fault-Timeout plus 12 sauber klassifizierte Last-Backpressure-Fälle)
- max pool wait: **150068 µs**
- total pool wait: **22245956 µs**

Die Zahlen zeigen, dass Sessions wiederverwendet werden und der Peak das konfigurierte Poollimit nicht überschreitet. Unter höherer Parallelität darf der bounded wait bewusst `Unavailable` erzeugen; entscheidend ist, dass dabei weder zusätzliche Connections noch falsche `UnauthorizedEvidence`-Antworten entstehen.

## Fault-Test: Pool absichtlich blockiert

Der Test hält `brain.conversation_owners_v1` in einem externen `ACCESS EXCLUSIVE` Lock und startet vier Requests, sodass alle vier Brain-Pool-Slots innerhalb echter DB-Operationen blockiert sind.

Ein fünfter Request:

1. wartet auf den Pool,
2. überschreitet keine neue Connection-Grenze,
3. läuft nach ungefähr 150 ms aus dem Acquire-Wait,
4. wird als `AnswerStatus::Unavailable` zurückgegeben,
5. wird **nicht** als `UnauthorizedEvidence` klassifiziert.

Nach Freigabe des Locks:

- die vier blockierten Requests schließen erfolgreich ab,
- ein neuer Request wird wieder `Answered`.

Zusätzlich bleibt der bestehende echte DB-Ausfall-/Recovery-Test erhalten: Verbindungen werden serverseitig beendet, Readiness wird 503, Requests werden `Unavailable`, nach Wiederfreigabe erholt sich Readiness.

## C1/C2/C3 Kompatibilität

C1:

- startbarer `brain-serve` bleibt Composition Root,
- Auth/Secrets/Release-Pins/Readiness/Drain bleiben erhalten,
- falsches DB-Passwort bleibt `database_unavailable`,
- echter Schema-Mismatch bleibt `core_schema_incompatible`.

C2/C3:

- `read_heads` bleibt bounded targeted lookup,
- technische Reader-/Poolfehler bleiben `Unavailable`,
- Evidence-Denials bleiben separat `UnauthorizedEvidence`,
- Retrieval-Index-/Chunking-Semantik bleibt unverändert.

## Verifikation

Bestanden:

- `cargo fmt --all -- --check`
- `scripts/test_brain_serve.sh` mit Scratch PostgreSQL `max_connections=12`, 8/16/32 Worker und Fault/Recovery.

Die restlichen Workspace-Pflichtgates werden vor PR-Erstellung ebenfalls ausgeführt und ihr Ergebnis in der PR-Beschreibung festgehalten.

## Grenzen

- Kein Produktions-`max_connections` wurde erhöht.
- Kein Deploy.
- Kein Merge.
- Kein Legacy-/RAG-Schattenpfad hinzugefügt.
- Der synchrone Port bleibt bewusst synchron, weil die bestehenden Kernel-/Policy-Contracts synchron sind; Backpressure und Connection-Lifecycle sind jetzt trotzdem zentral und hart begrenzt.