---
phase: 167-candidate-history-validation-rules
plan: "20"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: native-and-independent-phase-acceptance
tags: [validation, coverage, bazel, review, lifecycle]
requires:
  - phase: 167-19
    provides: Accepted current parity and contributor claims
  - phase: 167-25
    provides: Accepted current root project excerpts
provides:
  - Passed complete default native verifier on initial and revised host-tool configurations
  - Passed fresh six-target Bazel build and provenance on revised host-tool configuration
  - Fresh zero-uncovered pure-core coverage for the original eight selected packages
  - Retained source-bound failed attempts, corrections and validation receipt
affects: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
requirements_scope: bounded-plan20-native-contribution-only
global_cvld_acceptance: pending-independent-reports-and-root-finalization
generated_by: gsd-execute-plan
generated_at: "2026-11-05T06:01:04Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: revised-config-default-passed-hook-pending
---

# Phase 167 Plan 20: Native validation and acceptance handoff

Plan 20's initial complete default `bash scripts/verify.sh` [v14 receipt](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-c-default-verify-v14.json) exited **0** on source-stable bytes with pinned Bun 1.3.9. A later normal hook exposed a host-tool Bazel failure and led to a one-line `.bazelrc` correction. The revised configuration's complete default [v15 log](../../../.local/open-bitcoin-dev/phase167/plan20/revised-default-v15.log) also records **exit 0** after **2h 34m 54.863s**, with Phase 164 1,255/1,255, Phase 165 2,170/2,170, the nine original stress families, complete scoped native suite, benchmark, six-target Bazel/provenance, and fresh original eight-package zero-uncovered coverage. The revised source had no product edits during v15. Exact commands, counts, streams and failed attempts are in [167-VALIDATION.md](167-VALIDATION.md). A complete normal hook on final staged bytes remains pending.

## Source and repair scope

The initial verifier runs revealed current guard drift, four-thread full-suite scheduling pressure, test-only physical/reopen and RPC readiness assumptions, prune undo-size oracle drift, one generated Bazel lock input hash, and the original 159-line pure-core coverage gap. Every correction has a checked Plan 20 amendment and retained RED/GREEN commands. The controlled verifier preserves all nine original 16-thread stress selectors and runs the complete workspace/CLI native test block at the repo-native scoped four-thread setting without changing tests or deadlines. `scripts/verify/helpers.sh`, the eight-package selection and zero-uncovered gate remain unchanged. The accepted Bazel lock change is the exact Cargo-manifest freshness hunk, not a coverage workaround.

After v14, the staged normal hook found two stale copied Phase 164 positives, corrected by a narrow parity-catalog rationale edit; the full Phase 164 file passed 1,255/1,255. The next normal hook passed the guard and native portions but failed Bazel on malformed host proc-macro dylibs. The [host-tool amendment](167-20-BAZEL-HOST-TOOLCHAIN-AMENDMENT.md) checked one added `.bazelrc` line, `build --host_compilation_mode=fastbuild`; a fresh default six-target Bazel build passed all 753 actions and the provenance checker passed, followed by the complete v15 default pass. Target mode, selected packages, consensus behavior, nine stress selectors, coverage gate and normal hook remain unchanged. The failed hooks and clean/expunged diagnosis remain separate evidence and give no final saving credit.

Coverage closure proceeded through four bounded owner waves: A1 compact/peer (`63` original misses), A2 header/request (`34` original misses, five incidentally covered by A1), B chainstate/consensus/codec (`52`), and C mempool policy (`10`). The [A1](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-a1-final-v1/receipt.json), [A2](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-a2-final-v1/receipt.json), [B](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-b-final-v1/receipt.json), and [C](../../../.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-c-final-v1/receipt.json) receipts retain exact source frames, per-wave checks and missing-line comparisons. The final C frame has **1,833 rows, 82 direct owners, 1,751 protected original rows and zero protected drift**; its frame SHA-256 is `b98af995b4ce42116a7d732f948bf89eabc31e22b1351b3cf2e15057cc284901`. The original eight-package report then had no uncovered heading. No production validation rule, historical seed, security refusal or test bound was weakened to pass coverage.

The current product scope remains local default-Regtest genesis plus **301 validated descendants** and exact retained-history reopen. Mainnet/Testnet3 script and Mainnet BIP30 cases are independently seeded historical fixtures; they do not constitute earned ordinary product admission for those networks. The actual pinned-Knots daemon differential is Phase 168 work. Phase 167 makes no whole-chain, public-default or funds claim.

## Handoff and remaining limits

The initial worktree-source LOC was provisional; ROOT staged the accepted source and is reconciling index-source LOC. Task 2's independent canonical [source review](167-REVIEW.md), [security review](167-SECURITY.md) and [goal verification](167-VERIFICATION.md) must account for the revised build configuration and v15 full default pass, then validate the originating yolo lifecycle. ROOT owns the complete normal hook on final staged bytes, final claim/status alignment, commit and push. This summary claims no Git commit, push or Phase 168 work.

`AGENTS.md` Repo-Local Guidance, `AGENTS.bright-builds.md`, `standards-overrides.md`, and managed verification/testing/Rust/TypeScript pages were applied; the existing Rust panic-site override was not broadened. `git diff --check` passed on the worktree before this summary.

## Self-Check: REVISED-CONFIG DEFAULT PASSED; HOOK PENDING

- The complete v14 default verifier exited zero on unchanged source-owner hashes for its earlier build configuration; v15 exited zero on revised `.bazelrc` with no product edits during the run.
- The revised `.bazelrc` passed a fresh default six-target Bazel build, provenance check and full v15 verifier without changing the original eight-package zero-uncovered gate.
- Updated independent reports, lifecycle, staged-index LOC reconciliation, complete normal hook and Git saving remain pending ROOT/Task 2 work.
