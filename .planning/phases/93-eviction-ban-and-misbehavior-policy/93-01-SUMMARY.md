---
phase: 93-eviction-ban-and-misbehavior-policy
plan: 01
subsystem: networking
tags: [rust, open-bitcoin-network, eviction-policy, ban-policy, misbehavior-policy]

provides:
  - Pure Phase 93 eviction scoring policy
  - Typed scoped ban/discourage state and manual unban decisions
  - Bounded misbehavior response policy with protected-peer no-action outcomes
  - Thin PeerManager hooks for eviction and misbehavior decisions
affects: [phase-93-status-evidence, phase-93-docs-checker]

tech-stack:
  added: []
  patterns:
    - Pure data-in/data-out peer policy
    - Stable `as_str()` labels for later status and support projection

key-files:
  created:
    - packages/open-bitcoin-network/src/peer_policy.rs
  modified:
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-network/src/lib.rs
    - docs/parity/source-breadcrumbs.json

requirements-completed: [EVICT-01, EVICT-03, EVICT-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 93-2026-06-26T13-15-10
generated_at: 2026-06-26T13:39:00Z

completed: 2026-06-26
---

# Phase 93 Plan 01: Peer Policy Core Summary

Implemented the pure Phase 93 peer-policy core for deterministic eviction scoring, scoped ban state, manual unban, and bounded misbehavior responses.

## Accomplishments

- Added `peer_policy.rs` with typed eviction, ban, unban, and misbehavior policy decisions.
- Added stable evidence labels including `eviction_candidate_selected`, `eviction_suppressed`, `ban_active`, `ban_expired`, `unbanned`, `misbehavior_observed`, `disconnect_requested`, `discouraged`, and `protected_no_action`.
- Added `PeerManager::eviction_decision` and `PeerManager::misbehavior_decision` as side-effect-free hooks over existing peer state.
- Registered the new Rust file in `docs/parity/source-breadcrumbs.json`.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer_policy --no-fail-fast`
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network peer --no-fail-fast`
- `bun run scripts/check-parity-breadcrumbs.ts --check`

## Deviations

- Task-level commits were intentionally deferred. The invoked wrapper commits only after phase verification passes, so this summary records implementation evidence without claiming intermediate commits.

## Self-Check: PASSED

- New policy file exists and is exported.
- Focused tests passed.
- Breadcrumb checker passed after marking the new file intent-to-add.

---
*Phase: 93-eviction-ban-and-misbehavior-policy*
*Completed: 2026-06-26*
