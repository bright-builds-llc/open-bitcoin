# Phase 156 Native Verification Record

## native-attempt-1 | 2026-10-05 UTC

Command: default `bash scripts/verify.sh` with pinned Bun 1.3.9 and Rust 1.94.1.
Exit: 101 after 2m 56.280s. Native guard suites, provenance, pure-core dependency,
file-length and panic checks passed. Strict Clippy stopped at
`storage/fjall_store/prune.rs:185` with `nonminimal_bool`; builds, workspace tests,
benchmark, Bazel and coverage were not reached.

Repair: replace negated `Option::is_some_and` equality with `Option::is_none_or`
inequality. Missing and mismatching positions still refuse; matching positions
still proceed. Existing behavioral assertions and source contracts are unchanged.
Root will reformat and rerun the full contract. This failed attempt is not a
completion pass. The diagnostic log is retained in the ignored local Phase 156
output directory.

## native-attempt-2 | 2026-10-05 UTC

The lint rewrite changed the report's source-content fingerprint. The full verifier refused
the stale generated LOC report at its first freshness gate, exit 1 after 504ms.
Root regenerated the tracked worktree report and independently checked it current.
No build or test ran in this attempt. The next run restarts the complete contract.

## native-attempt-3 | 2026-10-05 UTC

Default `bash scripts/verify.sh` exited 0 after **32m 37.914s (1,957,914ms)**
with Bun 1.3.9 and Rust 1.94.1. LOC/provenance, all native guard and mutation
suites, dependency/file-length/panic checks, formatting, strict all-target and
all-feature Clippy, all-target build, workspace tests/doctests, benchmark
list/smoke and report, six Bazel smoke targets and build provenance, and configured
pure-core coverage all passed. The coverage missing-lines gate accepted the report.

Workspace tests and doctests recorded **3,382 passes, zero failures, one ignored**
before the separate coverage rerun; coverage results are not double-counted.

The full node library suite recorded **1,122 passes, zero failures, one ignored**
in 222.40s; the ignored case is the existing explicitly opt-in public Bitcoin
network smoke test. RPC library recorded 270 passes and daemon recorded 43 passes,
including the genuine legal-target index-protection scenario, in 135.26s. This
records offline local evidence and does not claim new public-network or CI evidence.

The live executable launch delay was sampled in the system loader before test code;
it eventually completed without intervention. Both earlier failed attempts remain
above. No source changed after the reviewed lint predicate and regenerated LOC
fingerprint. Formal security/lifecycle closure and the required commit hook follow.
