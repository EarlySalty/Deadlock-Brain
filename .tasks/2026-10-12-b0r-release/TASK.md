# B0R Brain release scheduler migration

Owner: B0R worker, direct orchestrator 829d42f1-18eb-4f1d-9a14-a409c57242a5. Explicit follow-up authorizes this separate Brain branch, gate, main integration and backup-backed installation of the root helper. Foreign rewrite worktrees are excluded.

Goal: route the existing privileged helper's unprivileged Cargo child through cargo-slot with CARGO_SLOT_DEPLOY=1 and jobs 8. Preserve source provenance, inventory, ELF, hash, hardlink and staging checks. Keep a Brain-only target lifecycle lock across cleaning, compilation and artifact copying. Accept the exact historical jobs-4 build argument array for existing manifests, without loosening other manifest checks.

Code commit 7f9ff8c4 passed the local merge gate. Existing helper formatter and Clippy checks passed. Full local helper suite b24kvsnpk: 21 passed, 0 failed, 0 ignored; verify compilation 24.45 s and a real tiny release fixture build 0.53 s. The previous attempt expired while waiting for occupied verification slots; it produced no test result. No foreign build was stopped. README follow-up references the active scheduler policy instead of incorrectly promising the deployment profile for worker checks.

Pending: gate the exact outgoing SHA, push main from this clean isolated worktree, install the main helper atomically with a backup and no existing brain-release process, validate the installed plan path and binary hash. Installing this CLI helper does not restart Brain services. Keep the cargo-slot legacy-lock migration marker absent until the root helper is switched and all old callers have naturally ended. Real Brain plus Twitch or Bots deploy-build overlap is a separate B0R infrastructure proof, not established by the small fixture.

TESTNACHWEIS[TW-1]: 21 passed, 0 ignored | Baseline: not measured, no pre-existing failure claim
