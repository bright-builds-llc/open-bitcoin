---
phase: 157-safe-activation-and-scheduled-index-catch-up
reviewed: 2026-10-05T23:34:30.308000Z
depth: standard
files_reviewed: 91
files_reviewed_list:
  - README.md
  - docs/parity/catalog/basic-compact-filters.md
  - docs/parity/checklist.md
  - docs/parity/index.json
  - docs/parity/source-breadcrumbs.json
  - packages/open-bitcoin-chainstate/src/filter_index.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/budget.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/budget.rs
  - packages/open-bitcoin-chainstate/src/filter_index/catch_up/tests/durability.rs
  - packages/open-bitcoin-node/src/chainstate.rs
  - packages/open-bitcoin-node/src/chainstate/filter_index.rs
  - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
  - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
  - packages/open-bitcoin-node/src/chainstate/tests.rs
  - packages/open-bitcoin-node/src/chainstate/tests/filter_index.rs
  - packages/open-bitcoin-node/src/lib.rs
  - packages/open-bitcoin-node/src/network.rs
  - packages/open-bitcoin-node/src/network/lifecycle_projection/authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/catch_up/inputs.rs
  - packages/open-bitcoin-node/src/storage/coins_view.rs
  - packages/open-bitcoin-node/src/storage/fjall_store.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/admission.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/filters/turn_inputs.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
  - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
  - packages/open-bitcoin-node/src/sync/block_response.rs
  - packages/open-bitcoin-node/src/sync/open_runtime.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/failures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/fixtures.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/measurements.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/accepted_faults.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/history_loss.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/evidence/retention.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured.rs
  - packages/open-bitcoin-node/src/sync/tests/filter_index/startup/configured/faults.rs
  - packages/open-bitcoin-node/src/sync/tests/restart_chainstate/persist_and_hydrate.rs
  - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/checkpoint.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
  - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
  - packages/open-bitcoin-rpc/src/config.rs
  - packages/open-bitcoin-rpc/src/config/blockfilter.rs
  - packages/open-bitcoin-rpc/src/config/loader.rs
  - packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs
  - packages/open-bitcoin-rpc/src/config/tests.rs
  - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
  - scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
  - scripts/check-phase123-runtime-timing-evidence-integrity/checks.ts
  - scripts/check-phase128-production-compact-announcement-transport.test.ts
  - scripts/check-phase128-production-compact-announcement-transport.ts
  - scripts/check-phase128-production-compact-announcement-transport/impl-attributes.ts
  - scripts/check-phase134-apply-boundaries/aggregate-roots.ts
  - scripts/check-phase134-apply-boundaries/reachability.ts
  - scripts/check-phase134-authoritative-lifecycle.test.ts
  - scripts/check-phase134-authoritative-lifecycle.test/apply-helpers.ts
  - scripts/check-phase135-snapshot-recovery.test.ts
  - scripts/check-phase135-snapshot-recovery.ts
  - scripts/check-phase155-filter-index.test.ts
  - scripts/check-phase155-filter-index.ts
  - scripts/check-phase157-index-catch-up.test.ts
  - scripts/check-phase157-index-catch-up.ts
  - scripts/check-phase157-index-catch-up/contracts.ts
  - scripts/check-phase157-index-catch-up/rust-evidence.ts
  - scripts/verify.sh
findings:
  critical: 0
  warning: 0
  info: 0
  total: 0
status: clean
generated_by: gsd-code-review
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T23:34:30.308000Z
review_partition: aggregate
source_fingerprint: 8f4ab44656fb68d307e64af13c3c89f1197bf3801417d096282eff612cf9e387
resolved_findings: [WR-01, WR-02, WR-L01]
---

# Phase 157 aggregate source review

All 91 distinct source/documentation files passed independent standard-depth review. Root verified each exact partition inventory and frozen source fingerprint, respecting its documented hash separator. No reviewed source changed after final review.

| Partition | Files | Report                           |
| --------- | ----: | -------------------------------- |
| runtime   |    58 | [Review](157-REVIEW-runtime.md)  |
| daemon    |     9 | [Review](157-REVIEW-daemon.md)   |
| closeout  |    15 | [Review](157-REVIEW-closeout.md) |
| legacy    |     9 | [Review](157-REVIEW-legacy.md)   |

WR-01 was reproduced with an actual retained BASIC backlog failure and successful Always B40. Eager worker joins/mempool settlement and conservative clean-marker gating passed real-worker controls and independent nine-file re-review. [Initial finding](157-REVIEW-daemon.iter1.md) and [first fix](157-REVIEW-FIX.iter1.md) preserve its evidence.

WR-02 was reproduced by the complete checker accepting the actual async daemon function with a test-only attribute. Offset-preserving modifier masking, its actual async negative control and eleven lexical controls passed with the final 568-control suite; independent fifteen-file re-review closed it. [Initial finding](157-REVIEW-closeout.iter1.md) and [second fix](157-REVIEW-FIX.iter2.md) preserve its evidence.

The default native verifier then exposed historical routing anchors, which were corrected without production changes. Phase 123 passed55 controls, Phase128 passed50 and unchanged129 passed12, and Phase134 passed257 with positives required before negatives. Legacy review found WR-L01, a test-only owning-implementation attribute gap. A small balanced attribute helper and independent ordered/multiple/comment/preceding-item controls passed, and the exact nine-file legacy re-review closed it. [Initial finding](157-REVIEW-legacy.iter1.md) and [third fix](157-REVIEW-FIX.md) retain proof.

The simplification passes reuse existing authorities, typed adapters, the checkpoint/prune owner and lexer. They avoid new dependencies, whole-history caches, services and another parser framework; the small pure attribute helper keeps the established guard readable within its file limit.

Aggregate SHA256 uses sorted relative path, NUL, lowercase SHA256 of file bytes and LF. Generated LOC and GSD planning/state artifacts are excluded; native freshness and formal goal verification cover them. Full native attempt2 is running from the beginning. Coverage, Bazel, formal lifecycle/goal verification, current-doc closure and Git finalization remain pending; no whole-phase or production-readiness claim is made.

## Verified closure overlays 2026-10-05T23:34:30.308Z

[Four current claims/ledger documents](157-REVIEW-closure.md) and [one completed-state fixture repair](157-REVIEW-fixture.md) received focused independent re-review with zero findings. Root verified each fresh per-file hash and all remaining frozen partition hashes. The exact91-source inventory is unchanged; its current aggregate fingerprint is8f4ab44656fb68d307e64af13c3c89f1197bf3801417d096282eff612cf9e387. The final completed-ledger fixture rerun passed568/0 with1573assertions. Production guard/Rust behavior is unchanged after the historical full-native pass; the mandatory normal commit hook now owns a complete rerun on final staged content.
