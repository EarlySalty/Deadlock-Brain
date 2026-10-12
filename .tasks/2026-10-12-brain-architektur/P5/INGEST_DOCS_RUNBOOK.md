# P5 deferred ingestion and live acceptance

This is a preparation document, not evidence of execution. The user directive at 2026-10-12 08:12 prohibits new cargo-slot jobs until the orchestrator explicitly clears build slots. P1 must be live before P5 integration or deployment. No commands below have been executed as part of this document.

## Release prerequisites

1. Published P1 checkpoint ddbeb2fe supplied the ordered migration interface from 3ef22729; later published corrections through 1e9bc821 were imported/pushed as primary eadd02cd and schema bcaada67. P5 version 5 and its shared semantic catalog checker are now source-registered through the existing runner/ledger, with standalone SQL unchanged. Finish deferred compiler, PostgreSQL-16/catalog and registration/reapply verification; source review is not runtime proof. Keep extractor dispatch additive. At final integration use the current origin/main extractor version plus one and actual registry size.
2. Before activating the bumped extractor, finish or publish unfinished runs of the previous version under the existing job/release serialization. An aggregate inspection of raw.run status and extractor_version must establish zero old pending runs. Do not relabel runs or bypass guards.
3. Once build slots are cleared, finish affected formatting, build, Clippy and existing tests, including real scratch-Postgres receipt, scope, publication and bounded-output contracts. Review the exact integration HEAD and obtain local gate ALLOW. No unverified source commit or main publication.
4. Verify P1 is live and no brain-release process exists. Use the shared release/deploy locks and existing plan/build/verify/install wrapper from a clean worktree matching verified origin/main. Release installation owns serve restart. Confirm migration 5 and the released brain-job ingest-docs dispatch before running the job.

The binding contract uses owner-only check-gamefacts/up-gamefacts validation before pointer activation. The source-registered Docs checker participates in that path. Do not invent extra health/LocalPgReader monitoring or runtime rights; a readyz 200 alone does not establish any P5 live acceptance point.

## System-unit preparation

P1 owns ops/brain-job@.service and ops/brain-job@.timer. The common wave-2 contract specifies MemoryMax=512M; P5 does not replace or lower that shared cap. The independent acceptance threshold is an observed ingestion peak below 300,000,000 bytes. A cap is not a peak measurement.

Use the final published P1 system templates, not the interim template from the first checkpoint. Inspect the installed instance metadata to confirm User=nathanael, Group=nathanael, explicit absolute configuration and credential-source paths, LoadCredential, MemoryAccounting=yes, MemoryMax=512M and the expected system-manager ControlGroup. Do not read any credential contents or secret environment. If the final P1 template already supplies the required instance configuration, no P5 unit copy or redundant drop-in is needed.

The imported P1 template at checkpoint ddbeb2fe adds ExecStartPost capture of InvocationID, cgroup membership, enforced limit and memory.max_usage_in_bytes while the job cgroup is still available. Verify that the installed instance actually contains those steps. Bind its sanitized journal measurements to the same ingest-docs invocation and fetched source commit; a P1 Wiki job measurement is not P5 Docs memory proof.

Before enabling a system instance, inspect whether the matching user-manager brain-job@ingest-docs service or timer exists. Stop and disable only those matching old instances if present. Do not change other jobs, serve directly, Caddy, the site, maintenance or dl-knowledge.

When unit installation is actually needed, follow the privileged, independently reviewed P0b install path under /opt/deadlock-brain/.deploy.lock from the clean verified origin/main worktree. Install only the approved package files. Do not enable the docs timer before both real executions below pass.

## Two real ingestion runs

1. Start the system brain-job@ingest-docs.service through the approved serialized path. Confirm successful process exit and complete sanitized count output. Capture the fetched commit, extractor version and existing receipt/publication checks. Import committed Git objects from the job's own cache, never the dirty Deadlock-Docs checkout.
2. Read the actual memory cgroup paths identified by the installed unit. On this cgroup-v1 host the expected files are /sys/fs/cgroup/memory/system.slice/brain-job@ingest-docs.service/memory.limit_in_bytes and memory.max_usage_in_bytes. Record both values while they remain available. The expected 512M cap is 536870912 bytes; the separately measured peak must be below 300,000,000 bytes. Report a missing measurement as unproven, not as zero.
3. Run the same system instance again without an upstream commit or extractor change. Confirm success, the same fetched commit, zero newly written raw objects and zero new facts using the actual count fields and aggregate DB deltas. Reused historical raw objects and backfill do not justify inventing a zero count. Preserve both proof artifacts.
4. Only after those checks pass, enable system brain-job@ingest-docs.timer under the shared deploy lock. Confirm its target is the system instance and its effective schedule is the published P1 timer. Do not create an additional scheduler.

## Scoped live behavior

- With each authenticated public grant separately, docs-client/docs.public and dl-bot/bot.public, retrieve a known public guide and confirm an immutable public GitHub commit-and-line citation.
- Through the existing private second-brain route, use the existing internal grant with empty provider egress. Confirm a known internal section is reachable with a harmless, non-private query; record the successful matching search inputs. Confirm provider transport is not used and publication/provider validation rejects internal evidence. Do not create token files or use a real user account.
- Repeat those same internally matching search inputs separately under docs-client/docs.public and dl-bot/bot.public. Both negative probes must exclude the known internal section from bodies, titles, paths, URLs and counts. An unrelated public-guide query or a probe with only one public grant is not scope proof. Use authenticated read-only checks without forwarding internal evidence to a provider; do not add an endpoint or widen a grant.
- Evaluate the exact Baba early-game question against the fetched committed corpus, canonical alias links and live answer. The earlier public-source inspection found no Baba/Yaga guide. A Billy citation is a wiring proof, not Baba acceptance; missing Baba evidence remains an explicit data-coverage gap outside P5.
- After Brain live acceptance, run the separate Bots gate and release. A real isolated bot session must demonstrate /v1/answer routing, silence for non-answered results, and no request to port 8896 or /v1/retrieve. Static source inspection is not this runtime proof. Keep dl-knowledge switched off.

## Evidence and external open points

Record exact source/release SHAs, commands, exit codes, passed/ignored test counts, aggregate DB checks, cgroup values and sanitized artifact paths. Keep built, reviewed, merged and live states separate. The stale rendered Billy docs URL returned HTTP 404 in earlier research; site deployment is outside P5. Public GitHub source links are the approved citation target.
