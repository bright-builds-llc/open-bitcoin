---
phase: 167-candidate-history-validation-rules
plan: "20"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-11-02T09:45:25Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 20 coverage Wave B chainstate three-line refinement

The exact Wave B source release is `.local/open-bitcoin-dev/phase167/plan20/root-coverage-b-source-release-v1/source-release.json`, SHA-256 `090b6d1cd566f48a561bd9875bd2ff21259c8c8bab68df189612a58df8e8ae9a`. The current frozen B v5 full frame is `.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-b-applied80-v5/full-frame.json`, SHA-256 `b410ed4d83cc8dfc1fbed033c1fc8dc1ee0d0e801c7c755c1b744a92b658e128`: 1,833 paths, 80 direct owners, 1,753 protected paths, zero drift. This refinement grants no new source owner and does not accept Wave B or Wave C.

The fresh pinned chainstate-package coverage report `.local/open-bitcoin-dev/phase167/plan20/task1-execution-v1/coverage-b-chainstate-report-v1.stdout`, SHA-256 `95ba5ed2046a18cd1dbaa4a48741976975252f7bc5547599f940cb2a341a735a`, ran after locked clean and exited zero/source unchanged. Its runner receipt `coverage-b-chainstate-report-v1.json` has SHA-256 `4ffa9566c99bfb462a25c5f4cfa9263fa206cd82380299ecdcbf9b35af67df0e`. The only remaining chainstate uncovered lines are `packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs:60,477,478`: five successful extended-history calls reach lines 55–59 and 61, but not the line-60 `?` error edge; the test-only `canonical_child` PoW mining body at 477–478 is unentered because the initial Regtest nonce already passes. The fresh consensus and codec reports, respectively SHA-256 `3bbd54603b3d85c94ca011aa2217491452571b2859b6a5605055b105e2d7fa91` and `3ab6ca74d14d8377346fdafa88199a09b528e3d0931e8602b205e5549c8eb778`, show all their original Wave B semantic lines covered. Their package-only remaining lines are preexisting cross-package paths outside the original B 52 and require the final original eight-package gate for aggregation.

On this same source, full native consensus 417/417, chainstate 449/449, codec 78/78, Phase164 Bun 1,255/1,255, Phase165 Bun 2,170/2,170, Phase167 Bun 59/59, and Phase164/165/167 direct CLIs all passed with retained raw streams and unchanged source. Earlier B Clippy E0599, consensus BIP34 seed, chainstate Regtest seed, and B v4 metadata failures remain diagnostic, with no final credit.

## Exact in-owner repair

Only two already leased paths may change: `packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs` and the one attributable current raw `CURRENT_PINS` row for that file in `scripts/check-phase167-candidate-history/contracts.ts`. No other Rust, guard, historical pin, parity breadcrumb, verifier, profile, source owner, or Phase168 change is permitted.

1. Retain the positive `linked_regtest_creating_mtp_extends_past_short_header_window` test: actual checked Regtest ancestry 0..160, candidate 161, a relative-lock input whose creating MTP requires extension beyond the short 155-header window, and its exact successful context assertions. Clone the active chain for a separate negative using the same checked short history. Change only the older height-1 header nonce and reconstruct that `ChainPosition` through its existing constructor so the changed row is internally hash-consistent; leave height-2's previous hash, all newer rows, and the newest parent unchanged. The extended lookup must reject the broken older link as `HistoryError::ParentMismatch { height: 1 }`, propagated at `history_from_chain(...)?` line 60 as exact typed `ChainstateError::Serialization` with candidate-history context/reason. Assert no transaction context is returned. This tests the actual fallible extension; it does not fabricate accepted state or substitute a newest-parent mismatch.
2. In test-only `canonical_child`, find a genuinely invalid nonce using a finite search bounded to 4,096 checks and assert `check_block_header` rejects it. From that invalid nonce, mine using a second finite 4,096-check search and assert the final header satisfies real proof of work. Keep the original merkle root, block and caller tests intact. This executes current lines 477–478 without a synthetic success, unbounded loop, changed PoW threshold or altered production code. If either finite search fails, fail the test visibly.
3. After normal formatting and exact whole-body review, renew only the current `candidate_rules.rs` SHA-256 row in the already-owned Phase167 contract. Preserve every other current row, all frozen historical identities, and source-provenance semantics.

## Verification and stop gates

Capture exact pre-edit whole bodies and the 80-owner/1,753-protected source frame, then obtain ROOT and independent source/security review of this formal amendment before an edit. After the two in-owner changes, freeze a new complete frame and obtain independent applied-source/security review **before Cargo**. Keep `candidate_rules.rs` normally formatted and at most 628 lines; preserve the original tests and positive assertions rather than compressing or dropping them.

Run pinned Bun1.3.9 and the repo cooperative Cargo lock with canonical TMPDIR: `cargo fmt --all`, focused positive/negative chainstate cases with nonzero discovery, full `open-bitcoin-chainstate --lib --all-features`, Phase167 direct checker and full guard test, affected Phase164/165 current and historical controls including full suites as required by the Wave B amendment, all with raw argv/streams/timing/exit and source before-after. Then run fresh locked `cargo llvm-cov clean --manifest-path packages/Cargo.toml --workspace` and `cargo llvm-cov --manifest-path packages/Cargo.toml --package open-bitcoin-chainstate --show-missing-lines --text`. Require all three current chainstate misses gone, no newly uncovered Wave B line, and the prior consensus/codec original B branches still covered. Recheck original eight-package zero-uncovered coverage only after Wave C, as Plan20 requires. A remaining/new B gap, failed behavior, guard or source drift is HOLD for checked diagnosis; do not advance Wave C, change a threshold, stage, commit, push, or enter Phase168.
