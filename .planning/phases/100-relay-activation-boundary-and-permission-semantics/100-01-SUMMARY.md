---
phase: 100-relay-activation-boundary-and-permission-semantics
plan: 01
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 100-2026-06-29T16-18-03
generated_at: 2026-06-29T18:39:56Z
subsystem: network-relay-policy
tags: [relay-activation, peer-permissions, network-policy, parity, v2.0]
key-files:
  created:
    - .planning/phases/100-relay-activation-boundary-and-permission-semantics/100-01-SUMMARY.md
    - packages/open-bitcoin-network/src/relay.rs
  modified:
    - docs/metrics/lines-of-code.md
    - docs/parity/source-breadcrumbs.json
    - packages/open-bitcoin-network/src/inbound.rs
    - packages/open-bitcoin-network/src/inbound/permissions.rs
    - packages/open-bitcoin-network/src/inbound/tests.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/peer/tests.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-rpc/src/dispatch/tests.rs
    - packages/open-bitcoin-rpc/src/inbound_listener/tests.rs
requirements-completed: [ACT-01, ACT-02, ACT-03, ACT-04]
duration: 60m
completed: 2026-06-29
---

# Phase 100 Plan 01: Relay Activation Policy Summary

**Plan 100-01 adds the pure network policy boundary for default-off relay activation and scoped relay permission evidence without changing peer socket or mempool behavior.**

## Performance

- **Duration:** 60m
- **Started:** 2026-06-29T17:39:25Z
- **Completed:** 2026-06-29T18:39:56Z
- **Tasks:** 2
- **Files modified:** 11

## Accomplishments

- Added `RelayPermissionEffectLabel` for `transaction_relay_policy_input`, `force_relay_policy_input`, and `mempool_policy_input`.
- Updated inbound permission decisions, RPC/node tests, and peer tests so relay-like permission tokens are scoped v2.0 policy inputs, while bloom/filter labels remain inactive.
- Added `packages/open-bitcoin-network/src/relay.rs` with `RelayActivationConfig`, `RelayEligibilityReason`, `RelayEligibilityInput`, `RelayEligibilityDecision`, and `classify_relay_eligibility`.
- Covered default-disabled activation, outbound/manual eligibility, inbound-serving requirements, protected-inbound non-eligibility, inactive filter labels, and service-bit invariance.
- Registered the new relay policy module in source breadcrumbs with Knots anchors.

## Task Commits

Each task was committed atomically:

1. **Task 1: Promote relay-like permissions into scoped policy-effect labels** - `5e168925`
2. **Task 2: Add pure relay activation and eligibility matrix** - `92ae954b`

## Verification

- `cargo fmt --manifest-path packages/Cargo.toml --all --check` passed.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-targets --all-features -- -D warnings` passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --all-features --no-run` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib scoped_relay_permission -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib all_permission_emits_scoped_relay -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib legacy_inactive_relay_like_effect_labels_remain_stable -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib relay_activation -- --nocapture` passed.
- `timeout 90s cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib relay_eligibility -- --nocapture` passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` passed.
- Rust pre-commit sequence passed before the Task 2 commit: `cargo fmt --manifest-path packages/Cargo.toml --all`, `cargo clippy --manifest-path packages/Cargo.toml --all-targets --all-features -- -D warnings`, `cargo build --manifest-path packages/Cargo.toml --all-targets --all-features`, and `cargo test --manifest-path packages/Cargo.toml --all-features`.
- Repo-native commit hook verification passed through `bash scripts/verify.sh` for both task commits; the Task 2 hook completed in 3m 22.900s.

## Deviations

- The breadcrumb checker initially failed for `packages/open-bitcoin-network/src/relay.rs` because the script scans Git-tracked files. Staging the new file made the mapping visible, and `bun run scripts/check-parity-breadcrumbs.ts --check` then passed.
- Coverage required a direct legacy label stability test after new permission parsing stopped emitting `inactive_relay`, `inactive_forcerelay`, and `inactive_mempool`. The legacy enum variants remain available for existing support/status sanitizer compatibility.
- Cross-crate tests were updated where scoped permission-effect counts and inactive-effect expectations changed.

## Self-Check: PASSED

- [x] Relay activation defaults to disabled.
- [x] Peer eligibility is classified by a pure data-in/data-out policy.
- [x] Relay-like permissions are scoped policy inputs only.
- [x] Bloom/filter permission labels remain inactive.
- [x] Service bits and peer socket behavior remain unchanged.
