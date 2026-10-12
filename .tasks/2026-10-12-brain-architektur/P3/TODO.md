# P3 paused for ingestion redesign

Pause decision: 2026-10-12, 16:45 CEST. The user stopped P1 and will rethink its ingestion design. P3 is paused because it depends on P1. This supersedes the earlier plan to resume after P1 live and a build-slot release. Await a new briefing before starting workers, fixers, builds, tests, reviews or implementation. Preserve the branch and worktree; no main merge, deploy, discard or cleanup.

Worktree: /home/nathanael/.worktrees/Deadlock-Brain-p3-timers
Branch: feat/brain-p3-timers-20261012
Pushed implementation WIP: c7b185afc4c5ce51d043def262615e2df7e409f5
Pushed interface-only checkpoint: 3bf1b2e28f6c53b7be89932f68264d3a03a28474
P1 checkpoint incorporated: 3ef22729188f435f070e6ef8182a90a28df6659c
Dependency merge: 9f3766af5a2e8681c78855098a533ac4eb158286
Evidence and decision history: AUFTRAG.md in this directory.

The WIP preserves the closed get_timer request, timer extractor, bounded Postgres read path, receipt-backed fixtures and timer-specific migration. Reserved version 2 is registered against P1's old ordered registry, and the private direct-SQL fixtures stamp installed versions. These integration adaptations remain unexecuted. They must not be assumed compatible with the redesigned ingestion path. Later P1 corrections were not chased.

P3's tracked checks and read-only reviewer had completed before the hold; no P3 worker, fixer or Cargo job is known to remain active. No new job was started during the hold. The continuation-note change is a documentation checkpoint, not a verified runtime release.

## Continuation after the new briefing

1. Read the redesigned ingestion contract and ownership in the new briefing. Reassess the Store API, migration registry/version reservation, raw-object receipt and occurrence-aware locator contracts before adapting the preserved draft. Do not automatically resume the old P1 rollout plan or request interfaces from the stopped worker.
2. Reconcile the new approved base and extractor-version policy without rewriting applied migrations, relabelling receipts/runs or bypassing guards. Recalculate the actual tool-definition count after integration. The former p1-v1 unfinished-run activation precondition is historical and must be reevaluated against the new design.
3. Once jobs are explicitly authorized, run the corrected real-PG indexed-key test, the previously unreached storage checks and the explicit public Store/raw-receipt/idempotence proof using both public source checkouts. Measure actual scoped cgroup limit/peak separately from compiler memory. Run the matching formatter, strict lint, build and existing tests on the integrated state.
4. Complete independent acceptance and the local merge gate on the final implementation. The responsible integrator decides main integration and deploy according to the new briefing. Do not use this WIP checkpoint as gate or live approval.
5. Preserve the timer acceptance unless the new briefing changes it: source-backed get_timer, raw-object-resolving citation, no fabricated urn first-spawn value, ordered trace without legacy asset loading, production indexed EXPLAIN and actual job/serve memory. Cleanup follows approved integration and live acceptance.

Open evidence: the first edited-target run had one P3 plan-fixture failure; its correction has not run. No full green P3 PostgreSQL verification, gate ALLOW, main merge, deploy or live proof is claimed. The public Store/cgroup proof and productive timer/urn response evidence are still missing. Existing logs and source observations are preserved in AUFTRAG.md, not treated as proof of the future redesign.
