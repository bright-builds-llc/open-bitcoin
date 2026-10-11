---
phase: 160-explicit-bounded-peer-filter-serving
plan: "02"
subsystem: network
tags: [basic-filters, bip157, permissions, handshake, pruning]
requires:
  - phase: 160-01
    provides: COMPACT_FILTERS service flag and typed CF wire messages
provides:
  - Requested-versus-expanded BASIC capability across admission
  - Typed startup prerequisite preserving the pinned all-only exception
  - Prune-compatible configured services and per-session own advertisement
affects: [160-03, 160-05, 160-08, 160-09, 160-10, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [pure capability policy, global-base and session-overlay separation]
key-files:
  created:
    - packages/open-bitcoin-network/src/filter_serving.rs
    - packages/open-bitcoin-network/src/filter_serving/capability.rs
    - packages/open-bitcoin-network/src/filter_serving/capability/tests.rs
    - packages/open-bitcoin-node/src/network/tests/filter_capability.rs
  modified:
    - packages/open-bitcoin-network/src/inbound/permissions.rs
    - packages/open-bitcoin-network/src/lib.rs
    - packages/open-bitcoin-network/src/peer.rs
    - packages/open-bitcoin-network/src/peer/message_dispatch.rs
    - packages/open-bitcoin-node/src/network/limited_serve.rs
    - packages/open-bitcoin-node/src/network/tests.rs
key-decisions:
  - Keep configured capability independent of backend enablement and catch-up; the startup caller validates prerequisites.
  - Keep the existing global services bit as configuration ownership; overlay matched permissions only on the session version.
  - Preserve existing admission effect reporting until configuration is applied at local version construction.
requirements-addressed: [CFNET-01, CFNET-05]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: 2026-10-10T19:17:30Z
duration: 6min
completed: 2026-10-10
---

# Phase 160 Plan 02: Requested BASIC Capability and Own Services Summary

**Admission preserves explicit blockfilters provenance while version construction composes per-peer BASIC capability with truthful prune services before catch-up.**

## Accomplishments

- Added a pure capability fact and typed BASIC prerequisite. General serving and explicitly named `blockfilters`, including `all,blockfilters`, require BASIC. `all` alone remains accepted without BASIC and retains effective per-peer capability.
- Admission carries the capability independently of active/inactive effect labels. Applying serving configuration conditionally activates only the filter-serving label; Bloom remains inactive and relay labels retain their separate policy.
- Both outbound initial version construction and inbound response version construction snapshot `PeerState::own_advertised_services`. Remote services cannot grant permission or rewrite that snapshot.
- General capability uses the existing base `LocalPeerConfig.services` bit. Prune and explicit serving setters share one composition helper; per-peer overlays never write the global base. Constructor defaults remain unchanged.

Material guidance: AGENTS.md, AGENTS.bright-builds.md, placeholder-only standards-overrides.md, and architecture/code-shape/testing/verification/operability/Rust standards informed the pure policy seam, default-off ownership, bounded file size, pinned breadcrumbs and serialized verification. The parent loaded active lessons and synchronized the checkout before delegation.

## Task Acceptance

### Task 1: Carry requested and effective filter capability across admission

- [x] `all` without BASIC is accepted with effective capability true; explicit `blockfilters` plus `all` refuses without BASIC.
- [x] Empty, implicit-whitelist-equivalent, `noban`, `download`, `forceinbound`, and Bloom permissions grant no filter capability. Capability policy opens no index or listener.
- [x] Outbound permission-class grammar remains explicitly rejected.
- [x] Conditional filter labels preserve Bloom and unrelated relay behavior.

### Task 2: Compose services and snapshot each handshake's own capability

- [x] General serving offers CF on both version constructors with best height `-1` and no synchronization started.
- [x] Matched `all` and explicit `blockfilters` overlay only the matching peer's version; ordinary inbound/outbound sessions and global services remain unchanged.
- [x] Manual and automatic pruning retain `NETWORK_LIMITED | WITNESS` alongside CF and omit `NETWORK`.
- [x] Both setter orders preserve correct service composition; disabling general serving removes only the configured CF capability.
- [x] Remote CF advertisement and later global changes cannot change a previously captured own-session advertisement.

## Verification Evidence

All Cargo commands used pinned Bun 1.3.9 through `bun run scripts/command-timings.ts run --key <key> --`, with Cargo jobs serialized.

| Stage | Command after timing wrapper | Executed result |
| -- | -- | -- |
| Task 1 RED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase160_capability -- --nocapture` | 1 test executed, 1 failed: admission equated all-only with all-plus-explicit provenance. |
| Task 1 GREEN | Same command | 5 passed, 0 failed; integration binaries had zero matching tests, while the required library filter executed 5. |
| Task 2 RED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase160_services -- --nocapture` | 1 test executed, 1 failed: prune update erased COMPACT_FILTERS. |
| Task 2 GREEN | Same command | 6 passed, 0 failed. |
| Network regression | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network --lib` | 565 passed, 0 failed. |
| Node network regression | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node network::tests:: -- --nocapture` | 339 passed, 0 failed. |
| Targeted lint | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-node --all-targets --all-features -- -D warnings` | Passed after replacing a test's manual assignment with `|=`. |
| Formatting | Scoped `rustfmt --edition 2024 --config skip_children=true` on the ten plan paths | Passed; unrelated children were preserved. |
| Diff review | `git diff --check`, scoped diff and line-count review | Passed; largest owned file is peer.rs at 616 lines. |

The initial RED setup briefly referenced not-yet-defined exports, and the first expanded Task 1 compile referenced an absent Bloom constant; both were corrected before the recorded nonzero RED/GREEN behavioral evidence. No runtime or production workaround was added.

## Exported Interfaces and Downstream Notes

- `PeerPermissionSet::requested_token(PeerPermissionToken) -> bool` queries original requested provenance. Existing `contains_token` expansion behavior remains intact.
- `BasicPeerServingCapability { general_configured: bool, effective_permission: bool, explicit_permission_requested: bool }` is exported from the network crate. `from_permissions(general_configured, &PeerPermissionSet)` creates it; `validate_basic_index(basic_enabled)` returns `BasicPeerServingPrerequisite::{Accepted, BasicIndexRequired}`; `advertised()` and `own_services(base)` supply pure advertisement policy.
- `InboundPermissionDecision::basic_filter_capability()` exposes the retained fact. Its consuming `with_basic_filter_serving(general_configured)` applies conditional effect labels idempotently. The new active label is `PermissionEffectLabel::BlockFiltersServingPolicyInput`, serialized by `as_str` as `blockfilters_serving_policy_input`.
- `ManagedPeerNetwork::set_basic_filter_serving(bool)` changes explicit general capability through the same private helper as `set_prune_mode`. Plan 03 must validate the prerequisite before invoking it and publish it through the existing runtime authority before networking.
- `PeerState::own_advertised_services: ServiceFlags` is the session capability authority for later request validation. Zero before local version construction means no capability has been advertised. Never substitute `remote_services_bits` or the current global base for this captured fact.
- Plans 08–10 still own valid all-only/no-backend request behavior and actual socket serving. These tests exercise the production version constructors and admission seam, without claiming achieved socket writes or request serving.
- Plan 12 owns scoped parity/README/UAT reconciliation; no broad Phase 161 projection is added here.

## Breadcrumb Registration Handoff

The parent serially owns `docs/parity/source-breadcrumbs.json`; this executor did not mutate it. Parent reported all four registrations complete and `check-parity-breadcrumbs.ts --check` passing at 1,063 paths before this handoff.

- `packages/open-bitcoin-network/src/filter_serving.rs`
- `packages/open-bitcoin-network/src/filter_serving/capability.rs`
- `packages/open-bitcoin-network/src/filter_serving/capability/tests.rs`

These three files contain ordered anchors `packages/bitcoin-knots/src/net_permissions.h`, `packages/bitcoin-knots/src/net_permissions.cpp`, `packages/bitcoin-knots/src/init.cpp`.

- `packages/open-bitcoin-node/src/network/tests/filter_capability.rs` contains ordered anchors `packages/bitcoin-knots/src/init.cpp`, `packages/bitcoin-knots/src/protocol.h`, `packages/bitcoin-knots/src/net_processing.cpp`.

Pinned `InitializeNode` and implicit whitelist expansion were inspected directly. Existing modified files retain their original breadcrumb groups.

## Task Commits and Shared State

Task 1 and Task 2 commits are deferred to the strict outer wrapper. No staging, commit, push, branch, worktree, STATE, ROADMAP, requirement-checkbox, task-ledger or breadcrumb-manifest mutation occurred in this executor. The parent owns final requirement completion and shared planning-state updates after lifecycle-valid final verification.

## Deviations from Plan

None in implementation scope. The outer lifecycle explicitly overrides per-task commits and full verification: both remain pending at the parent gate. An intentionally failing RED commit was not created.

## Security and Simplification Review

- T-160-04: Separate requested and effective flags plus the complete permission/prerequisite matrix preserve the pinned all-only exception without granting implicit privilege.
- T-160-05: Both local version constructors capture own services; a remote-CF regression proves remote input cannot authorize the session.
- T-160-06: One setter composition helper preserves prune classification and WITNESS across both configuration orders.
- Simplification pass retained one pure capability module, one node composition helper and one peer snapshot helper. No extra constructor field, backend/readiness authority, permission grammar, production dependency or schema was introduced.
- Stub scan found no TODO/FIXME/placeholder implementation in the created files. Empty collections in existing ordinary-admission constructors remain their real default. No new unmodeled network endpoint, auth path, filesystem boundary or schema surface was introduced.

## Authentication Gates and Deferred Issues

None. Native `bash scripts/verify.sh`, independent review and manifest registration are intentionally parent-owned final-phase work.

## Self-Check: PASSED

- All four created Rust paths and this summary exist on disk.
- Final scoped rustfmt check and `git diff --check` passed.
- Final task filters executed 5 and 6 passing tests respectively after the Clippy test-style correction; targeted all-target/all-feature Clippy passed.
- No commit-existence claim is made: commits are explicitly deferred to the outer strict gate. Requirement completion remains empty.
