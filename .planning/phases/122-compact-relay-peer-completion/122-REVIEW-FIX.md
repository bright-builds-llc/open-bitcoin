---
phase: 122-compact-relay-peer-completion
review_path: .planning/phases/122-compact-relay-peer-completion/122-REVIEW.md
fixed_at: 2026-07-15T16:28:39Z
status: all_fixed
findings_in_scope: 1
fixed: 1
skipped: 0
iteration: 1
---

# Phase 122 Code Review Fix Report

## Fixed Finding

### WR-01: Unannounced `getblocktxn` requests bypassed request-pressure governance

- Moved the raw `request.index_deltas.len()` pressure check before the per-peer announcement-provenance early return.
- Preserved benign silence for in-cap unannounced hashes and deferred differential-index expansion until after peer provenance authorizes the request.
- Added `phase122_unannounced_getblocktxn_over_request_cap_disconnects_before_suppression` to prove oversized unannounced requests produce the existing typed resource-governance disconnect.
- Extended the deterministic Phase 122 checker and mutation suite to require pressure enforcement before provenance suppression.

## Verification

- `bun test scripts/check-phase122-compact-relay-peer-completion.test.ts` — 15 passed, 0 failed.
- `bun run scripts/check-phase122-compact-relay-peer-completion.ts` — passed.
- Focused Rust regression test — 1 passed, 0 failed.
- Full repository verification remains the orchestrator's final gate.

## Residual Risk

None identified for the reviewed finding.
