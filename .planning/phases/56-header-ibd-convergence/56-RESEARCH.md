---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 56-2026-06-03T12-44-57
generated_at: 2026-06-03T12:55:00.000Z
---

# Phase 56 Research

## Findings

- `DurableSyncRuntime::sync_until_idle_with_resolver` already performs
  multi-round sync by comparing `(best_header_height, best_block_height)` after
  each `sync_once`.
- `sync_once_continues_header_batches_when_peer_advertises_more_work` already
  proves multiple `headers` payloads can be accepted in one peer session.
- `runtime_seeds_headers_from_durable_store_on_restart` already proves durable
  header state seeds runtime status after reopen.
- Invalid header tests already prove rejected headers produce typed
  `InvalidData` failures and no contribution credit.
- The live smoke runner already polls fresh `openbitcoinsyncstatus` snapshots
  and reads final durable peer telemetry; it needs only an explicit
  first-header-progress proof object.

## Risk Notes

- Keep live-network smoke opt-in; deterministic tests must cover all new
  behavior.
- Avoid claiming block progress when only headers advance.
- Additive report fields minimize compatibility risk for existing report
  consumers.

## Verification Shape

- Focused Rust sync tests for target reached, no-progress diagnosis, invalid
  no-credit behavior, and durable status.
- TypeScript check through the repo-native `bash scripts/verify.sh` contract.
- Full Rust format, lint, build, test before commit.

---

*Phase: 56-header-ibd-convergence*
