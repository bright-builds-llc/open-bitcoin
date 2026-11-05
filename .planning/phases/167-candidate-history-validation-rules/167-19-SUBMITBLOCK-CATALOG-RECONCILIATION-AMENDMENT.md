---
phase: 167-candidate-history-validation-rules
plan: "19"
type: checked-execution-amendment
generated_by: gsd-execute-plan
generated_at: "2026-10-31T05:53:09Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
---

# Plan 19 submitblock catalog reconciliation dependency

On the reviewed current Phase 167 source, the default documentation reconciliation checker exits nonzero with three exact `submitblock` findings. The accepted Rust `SupportedMethod` enum has 30 serde names, including the baseline-backed `submitblock`; the checker still requires 29, while the RPC catalog omits `submitblock` and says 29 total / 21 baseline-backed plus eight unchanged extensions. Pinned Knots `rpc/mining.cpp` defines the method. This is a current catalog and checker drift exposed by the Phase 167 product route, not a reason to remove the Rust method or suppress the default verifier.

ROOT adds exactly two existing files to Plan19 Task1 ownership: `scripts/check-current-documentation-reconciliation.ts` and `docs/parity/catalog/rpc-cli-config.md`. The checked allocation becomes **12 unique paths**, Task1 **8** and Task2 **4**. Eleven files were present in the original 1,825-path post-FINAL18 stage, and the new Phase 167 parity page was genuinely absent; **1,814 original paths remain protected**. Before either new owner changes, close all owned Bun jobs, capture their complete original/live bytes and all current source in a 12-owner stage frame, issue a supplementary ROOT source release, and obtain independent release review. Earlier nine- and ten-owner releases remain historical boundaries, not retroactive authorization.

Renew only the current supported-method cardinality and catalog: 30 total, 22 baseline-backed including `submitblock`, and the same eight Open Bitcoin extensions. Record the pinned Knots mining anchor and the bounded local default-Regtest route; do not imply unrestricted selected-network submitblock, actual-daemon differential, whole-chain, public-default or production qualification. Preserve the checker's set equality, baseline/extension grouping, wallet/CLI and other reconciliation checks. Use the existing live-corpus reconciliation failure as genuine RED and run it to GREEN; test missing or misgrouped `submitblock` and wrong count in scratch inputs or existing tests without a new unowned test-file edit.

Run the direct reconciliation checker, its existing focused test, Phase164/165/167 current claims/checkers, complete breadcrumbs, original Plan19 commands and affected broad suites on one final source frame. Reissue source conservation and strict original-before inverse for all 12 owners, obtain independent source/claim and final evidence QA, then complete the canonical summary. Interrupted pre-refinement broad output is diagnostic only. No Rust, verifier, AGENTS, PROJECT, frozen historical artifact, other source owner, Git or Phase168 action is granted. Global CVLD and phase acceptance remain open.
