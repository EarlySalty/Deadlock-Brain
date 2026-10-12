# P1 memory-controller verification

Verified on 2026-10-12 by independent workflow `wf_2e273b9e-586`, task `wxcly8kqz`.

The actual user manager uses cgroup v1. Its service reports `Delegate=yes`, but `DelegateControllers` is empty and its memory membership remains `/user.slice`.

A uniquely owned transient probe, `brain-job-p1-memory-probe-121638e5.service`, used `MemoryAccounting=yes`, `MemoryMax=8M` and `RuntimeMaxSec=8s`. It touched 32 MiB, reached 44,416 KiB RSS and exited successfully after 1.347 seconds. The configured 8 MiB limit was not enforced. `MemoryCurrent` and `MemoryPeak` remained unset while running and after completion; the memory-controller subgroup was absent.

The probe was stopped and its final `LoadState=not-found` was verified. No production source, persistent unit, existing service or host controller was changed.

Consequence: the requested `systemctl --user show -p MemoryPeak` acceptance proof is unavailable on this host. A configured `MemoryMax=512M` does not establish enforcement. OS/provider-admin remediation is outside P1. Do not change controllers or reboot from this package.

Evidence: `/home/nathanael/.claude/projects/-home-nathanael--worktrees-Deadlock-Brain-p1-baba-pg/121638e5-3cc5-4f1c-870e-ccf3f7d0895b/subagents/workflows/wf_2e273b9e-586/journal.jsonl`.

Historical decision: the main orchestrator initially confirmed OpenVZ/cgroup-v1 and approved GNU time maximum RSS or sampled process-tree RSS as a P1 acceptance deviation. That route was never used for a completed ingest proof.

Current decision on 2026-10-12 supersedes it: P0b merges before P1, and P1 exclusively owns the new job service/timer templates. After rebasing onto the P0b `origin/main` state, adapt those templates and their installation to the documented system-manager convention with `User=nathanael`, existing `LoadCredential` and `MemoryMax`, using `sudo -n systemctl`. Acceptance uses the actual enforced cgroup-v1 limit and `memory.max_usage_in_bytes` for each ingest, not RSS alone. The peak still must remain below 300 MB.

Execution method to finalize after P0b: discover the live job's `ControlGroup` and the actual memory-controller mount, record its `memory.limit_in_bytes`, and capture `memory.max_usage_in_bytes` while the oneshot cgroup still exists. Reset or recreate only the owned job measurement between runs so the unchanged second run has an independent peak. Record actual bytes, unit/release/source revisions, completion status and object counts here; no measurements are completed yet. Do not infer enforcement from unit text or use the old user-manager probe as system-scope proof.

Routing: original blocker T3 sequence `2483238`; initial deviation was subsequently approved, then superseded by the explicit P1/P0b ownership and cgroup-measurement decision. The historical user-manager probe remains valid only for that user scope.