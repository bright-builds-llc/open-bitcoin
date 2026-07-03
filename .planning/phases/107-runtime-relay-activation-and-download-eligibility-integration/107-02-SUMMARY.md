---
phase: 107-runtime-relay-activation-and-download-eligibility-integration
plan: 02
subsystem: network
tags:
  - relay
  - runtime-config
  - managed-network
  - rpc-context

requires:
  - phase: 107-runtime-relay-activation-and-download-eligibility-integration
    provides: Plan 107-01 PeerManager relay download policy and eligibility gate
  - phase: 100-relay-activation-boundary-and-permission-semantics
    provides: Default-off RelayActivationConfig and peer relay eligibility policy
provides:
  - ManagedPeerNetwork constructors propagate relay activation into PeerManager download policy
  - Managed network status reports resolved relay activation without changing service bits
  - RuntimeConfig relay and inbound settings reach ManagedRpcContext network construction
  - Regression coverage for default-off, enabled outbound, ordinary inbound, and protected-only inbound download behavior
affects:
  - managed network transaction download scheduling
  - RPC daemon context construction
  - runtime relay activation evidence

tech-stack:
  added: []
  patterns:
    - Single PeerManager relay-download policy assignment in ManagedPeerNetwork::from_peer_manager
    - RuntimeConfig to ManagedPeerNetwork relay propagation through new_with_relay_activation

key-files:
  created:
    - .planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/tests.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/tests.rs
    - docs/metrics/lines-of-code.md

key-decisions:
  - "Set RelayDownloadPolicy in ManagedPeerNetwork::from_peer_manager so all managed constructors share one policy handoff."
  - "Project ManagedNetworkInfo.relay from resolved RelayActivationConfig instead of LocalPeerConfig.relay."
  - "Use config.inbound.enabled as the deterministic Phase 107 inbound-serving input for RPC context construction."

patterns-established:
  - "Default managed constructors remain relay-download default-off while explicit relay activation enables eligible outbound download scheduling."
  - "RPC context propagation tests assert relay evidence through context.network_info().relay."

requirements-completed:
  - ACT-01
  - ACT-02
  - DL-01
  - DL-02
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 107-2026-07-03T02-54-20
generated_at: 2026-07-03T04:20:30Z

duration: 10m
completed: 2026-07-03
---

# Phase 107 Plan 02: Runtime Relay Activation and Download Eligibility Integration Summary

**Resolved runtime relay activation now reaches managed PeerManager download eligibility and RPC context construction.**

## Performance

- **Duration:** 10m
- **Started:** 2026-07-03T04:10:21Z
- **Completed:** 2026-07-03T04:20:30Z
- **Tasks:** 2
- **Files modified/created:** 7, including this summary and refreshed LOC metrics

## Accomplishments

- Wired `ManagedPeerNetwork::new_with_relay_activation` and sync-limit construction through a single `PeerManager::set_relay_download_policy` assignment.
- Changed `ManagedNetworkInfo.relay` to report resolved relay activation while leaving `local_services_bits` unchanged.
- Replaced the RPC runtime context default constructor path with `ManagedPeerNetwork::new_with_relay_activation(..., config.relay, config.inbound.enabled)`.
- Added managed-network and RPC-context regression tests proving default-off suppression, enabled outbound `GetData`, ordinary/protected-only inbound suppression, and explicit runtime relay propagation.

## Task Commits

No commits were created. The execution context explicitly instructed this executor not to commit; the parent workflow owns final commit and push after whole-phase verification is clean.

1. **Task 1: Make managed constructors set relay download policy** - complete, not committed here.
2. **Task 2: Pass RuntimeConfig relay settings through RPC context construction** - complete, not committed here.

## Files Created/Modified

- `packages/open-bitcoin-node/src/network/relay_serving.rs` - Sets `RelayDownloadPolicy { activation: relay_activation, inbound_serving_enabled }` before storing the managed PeerManager.
- `packages/open-bitcoin-node/src/network.rs` - Reports `network_info().relay` from resolved relay activation and keeps service bits unchanged.
- `packages/open-bitcoin-node/src/network/tests.rs` - Covers default-off suppression, enabled outbound scheduling, fallback behavior, and ordinary/protected-only inbound suppression.
- `packages/open-bitcoin-rpc/src/context/network.rs` - Passes `config.relay` and `config.inbound.enabled` into managed network construction.
- `packages/open-bitcoin-rpc/src/context/tests.rs` - Covers default-off and explicit enabled runtime relay propagation.
- `docs/metrics/lines-of-code.md` - Refreshed from the current worktree after Rust changes.

## Decisions Made

- The managed relay-download policy is assigned in `from_peer_manager`, not separately in each public constructor, so future constructors cannot forget the PeerManager handoff.
- `ManagedNetworkInfo.relay` is the resolved Open Bitcoin relay activation flag; service-bit projection still uses `local_config.services.bits()`.
- RPC context construction uses `config.inbound.enabled` as the Phase 107 inbound-serving input, matching the plan's deterministic runtime boundary.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The Task 1 RED run failed as intended: enabled managed networks did not emit `GetData`, fallback requests were absent, and default status still reported the local relay flag.
- The Task 2 RED run failed as intended: `RuntimeConfig { relay.enabled: true }` still produced `context.network_info().relay == false`.
- `cargo fmt --check` initially reported formatting drift in the new tests; `cargo fmt --manifest-path packages/Cargo.toml --all` fixed it and the subsequent format check passed.

## Known Stubs

None. A targeted scan of modified files found no TODO, FIXME, placeholder, coming-soon, not-available text, or hardcoded empty UI/data stubs.

## Threat Flags

None. This plan changes in-process constructor wiring, status projection, and deterministic tests only; it adds no network endpoints, auth paths, filesystem trust boundary, schema changes, service-bit changes, compact block behavior, package relay, bloom/filter serving, or public relay defaults.

## Verification

- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib managed_network_transaction_relay -- --nocapture` - RED failed before implementation, then passed with 8 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib managed_rpc_context_builds_from_runtime_config -- --nocapture` - RED failed before implementation, then passed with 2 tests.
- `cargo fmt --manifest-path packages/Cargo.toml --all --check` - passed after formatter run.
- `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib relay_serving_cases -- --nocapture` - passed with 3 tests.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib open_bitcoin_jsonc_accepts_relay_activation_enabled -- --nocapture` - passed.
- `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib open_bitcoin_jsonc_defaults_relay_activation_to_disabled -- --nocapture` - passed.
- `rg -n "RelayDownloadPolicy|set_relay_download_policy|RelayActivationConfig::default\\(\\)" packages/open-bitcoin-node/src/network/relay_serving.rs` - passed.
- `rg -n "new_with_relay_activation\\(|config\\.relay|network_info\\(\\)\\.relay" packages/open-bitcoin-rpc/src/context/network.rs packages/open-bitcoin-rpc/src/context/tests.rs` - passed.
- `rg -n "ManagedPeerNetwork::new\\(" packages/open-bitcoin-rpc/src/context/network.rs` - no matches, as required.
- `bun run scripts/generate-loc-report.ts --source=worktree --output=docs/metrics/lines-of-code.md` - passed.
- `bun run scripts/check-parity-breadcrumbs.ts --check` - passed, 339 Rust files verified.
- `git diff --check` - passed.

## User Setup Required

None.

## Next Phase Readiness

Plan 107-03 can build operator/status evidence on top of truthful runtime relay activation and managed download eligibility. Remaining Wave 1 changes are preserved uncommitted for the parent workflow.

## Self-Check: PASSED

- Created summary file: `.planning/phases/107-runtime-relay-activation-and-download-eligibility-integration/107-02-SUMMARY.md`
- Verified `ManagedPeerNetwork::from_peer_manager` sets `RelayDownloadPolicy` with the relay activation constructor inputs.
- Verified `ManagedRpcContext::from_runtime_config_with_store` calls `ManagedPeerNetwork::new_with_relay_activation` with `config.relay` and `config.inbound.enabled`.
- Verified managed and RPC regression tests contain default-off and explicit enabled relay evidence.
- Verified no 107-02 git commit was created, matching the execution context.

*Phase: 107-runtime-relay-activation-and-download-eligibility-integration*
*Completed: 2026-07-03*
