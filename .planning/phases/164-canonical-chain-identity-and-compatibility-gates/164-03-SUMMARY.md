---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "03"
subsystem: network
tags: [rust, canonical-admission, immutable-ownership, effect-preservation]
requires:
  - phase: 164-01
    provides: Exact canonical identity and immutable ordinary-history refusal
  - phase: 164-02
    provides: Non-opening ordinary Fjall compatibility boundary
provides:
  - Immutable fixture versus selected-canonical managed admission ownership
  - Early header, import, direct/stored body, compact dispatch and replacement guards
  - Ordinary runtime refusal before initialization even for supplied raw stores
  - Exact durable namespace and managed-state preservation regressions
affects: [164-04, 164-05, 164-06, 164-10, 164-11, 164-12, 164-13, 165, 166, 167]
tech-stack:
  added: []
  patterns: [one immutable admission scope, shared first-effect guards, read-only bound-chain query]
key-files:
  created:
    - packages/open-bitcoin-node/src/network/canonical_admission.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests.rs
    - packages/open-bitcoin-node/src/network/canonical_admission/tests/fixtures.rs
  modified:
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/action_translation.rs
    - packages/open-bitcoin-node/src/network/header_sync.rs
    - packages/open-bitcoin-node/src/network/validation_history.rs
    - packages/open-bitcoin-node/src/network/mempool_lifecycle.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/tests.rs
    - packages/open-bitcoin-node/src/network/relay_serving.rs
    - packages/open-bitcoin-node/src/network/peer_network_clone.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Preserve the first canonical binding across consuming calls, owner clones and every handle wrapper.
  - Preflight complete ordinary header batches and retain a closure guard before insertion or receipt accumulation.
  - Guard general, sync, durable and compact body dispatch before peer effects as well as guarding owner connect/reorg methods.
  - Keep typed refusal categories bounded while mapping into existing network and sync error structures.
patterns-established:
  - Fixture is an explicit lower-level ownership classification and cannot overwrite an already bound canonical store.
  - Root identity is checked before unsupported scope; exact identity never grants ordinary acceptance.
requirements-completed: []
requirements-addressed: [CBOT-01, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: 2026-10-14T03:52:14Z
duration: 19min
completed: 2026-10-14
---

# Phase 164 Plan 03: Immutable Managed Admission Summary

**Selected-chain owners reject unsupported headers, imports, bodies, cached dispositions and replacement branches before managed or durable effects, while explicit fixture behavior remains intact.**

## Performance

- Tasks completed: 3/3.
- Implementation files: 14; this summary is the fifteenth artifact.
- Recorded execution evidence starts at 2026-10-14T03:33:19.423Z; completion is 2026-10-14T03:52:14Z, excluding initial context loading.
- All staging, commits, pushes, hook operations and shared state/roadmap/requirements/todo updates remain parent-owned under strict phase finalization.
- Requirement completion is deliberately empty pending full phase verification.

## Accomplishments

- Added private `AdmissionScope::{Fixture, Canonical(CanonicalChain)}` to the managed owner. Both lower-level constructors initialize Fixture, owner clones copy the scope, and handle wrappers preserve it. A consuming binding cannot switch an existing selection or return it to Fixture.
- Added a read-only bound-chain handle query and a canonical constructor that does not route through the fixture wrapper.
- Added fixed typed noncanonical-header, noncanonical-body and unsupported-scope categories. Network adapters use fixed reject codes and bounded diagnostics; startup uses the existing `SyncRuntimeError::InvalidData` channel.
- Guarded complete header batches before peer bookkeeping and retained the shared check inside the insertion closure, before validation, insertion, identity accumulation or durable prefix publication. Imported snapshots and the old fixture seed helper cannot replace a canonical owner's store.
- Guarded local/stored bodies before chainstate preparation or Duplicate/NonExtending/Disconnected caching. Replacement candidates and even empty replacement operations require support before reorg preview installation, mempool staging or receipts.
- Guarded general, sync and durable message dispatch before peer mutation. Compact-block and blocktxn dispatch also require support before reconstruction state changes, and `ReceivedBlock` actions guard before compact-slot cleanup.
- Added `DurableSyncRuntime::open_ordinary_configured`, which checks the selected canonical specification before initialization, coins recovery, BASIC configuration, prune resume or recovered owner construction. Every current selector refuses; a supplied raw store does not grant ordinary admission.
- Documented retained constructors/startup/seed methods as lower-level fixture/infrastructure mechanisms, and registered all three new Rust paths against pinned chainparams/validation breadcrumbs.

## Concrete Preservation Evidence

| Boundary | Arranged cases | Preserved observations |
| --- | --- | --- |
| Immutable ownership | Four selectors, owner clone, consuming repeated binding, canonical wrapper, fixture wrapper and shared handle clone | Exact first selected chain; Fixture query remains None |
| Headers/import | Empty/nonempty stores, all 16 selected/root pairs, arbitrary easy-PoW null root, empty/non-root batches, exact-root prefix plus child, empty/multiple-root imports and forged `u128::MAX` work | Full HeaderStore including best tip and proof facts, peer state, chain snapshot, mempool, caches, lifecycle and effect ledgers |
| Root bodies | All 16 selected/root pairs, intact/empty altered transaction bodies, local/stored/reorg paths | Same whole-owner snapshot; exact genesis creates no accepted tip or spendable outputs |
| Cached body/reorg | Empty and explicitly populated synthetic fixture histories; duplicate child, nonextending child, disconnected child, replacement branch and forged work | Coins, tip, headers, body/transaction caches, local inventory, mempool and lifecycle evidence |
| Dispatch/compact | Headers/full blocks/compact blocks/blocktxn through general, sync and durable routes; direct ReceivedBlock action with an owned pending compact slot | Peer download/request state and the nonempty compact slot, caches and owner observations |
| Durable admission | Four selectors against owned raw stores with genuinely complete empty validation coverage | Exact key/value bytes across all nine application namespaces, absent canonical receipt, unchanged filter/prune/coins records and owner snapshot |
| Ordinary startup | Four selectors × Enabled/Disabled/PreserveSaved BASIC modes; malformed pending coins H, prune intent and BASIC state sentinels | All nine namespace key/value byte sets remain identical; fixed support refusal precedes sentinel decoding, and Signet reports its missing challenge validation |

The durable evidence compares application records while a supplied raw store is open. It does not claim the physical non-opening filesystem guarantee delivered separately by Plan 02, nor undo any effects the caller already incurred when opening that raw handle.

## Task Commits

1. Task 1, immutable ownership and ordinary startup: deferred strict phase finalization.
2. Task 2, guarded headers and imports: deferred strict phase finalization.
3. Task 3, guarded bodies, cache shortcuts and reorgs: deferred strict phase finalization.

No task commit hashes exist; none are fabricated. The parent performs final commits after complete phase verification.

## Verification and TDD Evidence

All Cargo commands ran serially through `bun run scripts/command-timings.ts run`; every focused GREEN run discovered nonzero tests.

| Stage | Command suffix / key | Actual result |
| --- | --- | --- |
| Owner RED | `phase164-admission-owner -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib canonical_chain_owner` | Exit 101, missing scope/query/constructor APIs; session 99540 |
| Header RED | `phase164-admission-headers -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib canonical_chain_header` | Exit 101, 1 passed/2 failed: contextual bad-diffbits occurred instead of scope refusal; session 63938 |
| Body RED | `phase164-admission-bodies -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib canonical_chain_body` | Exit 101, 0 passed/2 failed: stored Disconnected returned success and bypassed refusal; synchronous run |
| Final owner GREEN | Owner command above | Exit 0, 3 passed/1,733 filtered; session 85850 |
| Final header GREEN | Header command above | Exit 0, 5 passed/1,731 filtered; session 68819 |
| Final body GREEN | Body command above | Exit 0, 5 passed/1,731 filtered; session 45681 |
| Existing affected network fixtures | `phase164-admission-fixture-regressions -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib network::` | Exit 0, 523 passed/1,213 filtered, 32.05 seconds; session 74259 |
| Final node lint | `phase164-admission-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` | Exit 0; session 94437 |
| Formatting | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all 13 changed Rust paths | Exit 0 |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts --check` | Exit 0, 1,150 Rust paths verified |
| File lengths | `bun scripts/bright-builds-check.ts file-lengths` | Exit 0, no findings or new exceptions |
| Whitespace | `git diff --check` | Exit 0 |

Default repo-native verification and independent full-phase reviews remain parent-owned. No authentication gate occurred.

## API Notes for Downstream Executors

- `ManagedPeerNetwork::into_canonical_scope(self, CanonicalChain) -> Self` binds only Fixture; calling it on a bound owner preserves the first chain. No public setter, inverse or runtime/config switch exists.
- `ManagedNetworkHandle::from_canonical_network(network, chain) -> Self` is the additional canonical constructor. It binds the owner without fixture routing and also preserves an existing first selection. Product code must compare its selected config to the actual query where it accepts supplied owners.
- `ManagedNetworkHandle::maybe_canonical_chain(&self) -> Result<Option<CanonicalChain>, ManagedNetworkAuthorityError>` matches the prescribed query: Fixture is None, bound owners return the exact chain, and cloned/shared wrappers do not change it.
- `ManagedPeerNetwork::try_seed_canonical_headers(&mut self, HeaderStore) -> ManagedResult<()>` requires canonical binding, checks roots and refuses missing support before assignment. Existing unit-returning `seed_header_store` now operates only on Fixture; it leaves canonical owners untouched.
- `DurableSyncRuntime::open_ordinary_configured(store, config, mode) -> Result<Self, SyncRuntimeError>` matches the prescribed API and always refuses for the current four immutable profiles. Existing raw startup methods remain lower-level fixture infrastructure.
- `network::CanonicalAdmissionError` adds `NonCanonicalHeader`, `NonCanonicalBody`, `UnsupportedValidationScope(UnsupportedValidationScope)` and a fixed `code()` accessor. Managed error variants remain unchanged; `NetworkError::InvalidHeader.reject_reason` carries `noncanonical-genesis-header`, `noncanonical-genesis-body` or `unsupported-validation-scope`.
- Existing `transient_runtime` product binding and ordinary RPC/CLI call-site conversion remain with subsequent plans; the raw constructor defaults are not canonical product authority.

## Decisions and Simplification Review

The local AGENTS guidance, Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust pages, active lessons, final context and research informed execution. Kept one immutable scope and one pair of identity/support guards, with thin message/body adapters. Generic HeaderStore/ChainPosition APIs, historical receipts and consensus arithmetic remain unchanged. No new dependency, endpoint, runtime flag, alternative validation engine or canonical bootstrap was added.

Reviewed first-effect placement and diffs. Header preflight and closure checks intentionally share the same implementation: preflight prevents peer/prefix effects and the closure protects insertion itself. New files are 195/602/132 lines; existing network/runtime sources remain 627/625 lines. Empty test vectors are deliberate refusal cases, not missing product data. No goal-blocking stubs or new unmodeled production trust surfaces were found.

## Deviations from Plan

**1. [Rule 3 - Blocking] Wired scope into actual split constructor and clone implementations.** `network/relay_serving.rs` and `network/peer_network_clone.rs`, outside the original list, needed field initialization/copying for compilation and immutable ownership. The parent explicitly authorized these narrow edits; clone/wrapper tests verify their behavior.

**2. [Rule 2 - Missing Critical Functionality] Closed dispatch and compact-cleanup bypasses.** General/durable dispatch used raw PeerManager admission, and ReceivedBlock cleared compact slots before the guarded stored-body call. The parent authorized narrow `network/action_translation.rs` changes. Shared pre-dispatch/action guards now precede those effects; pending-slot preservation regressions pass.

**3. [Rule 3 - Blocking] Split test infrastructure to respect file boundaries.** Added `canonical_admission/tests/fixtures.rs` for owned durable/compact setup and moved the existing unchanged BASIC test query from `runtime_authority.rs` into its existing `runtime_authority/tests.rs`. This keeps all files below 629 physical lines without blank-line compression, formatting skips or exceptions. Public API behavior is unchanged.

## Issues Encountered

Expected RED failures were followed by passing focused suites. Compilation exposed one second constructor requiring the scope field and the private read-helper boundary, so the read-only query stays in runtime_authority. A startup test initially used `expect_err` on a runtime lacking Debug and was corrected to `let Err`; Clippy's two nested-guard findings were corrected with let chains. No unresolved issue remains.

## Next Plan Readiness

Ready for ordinary product adapter wiring and diagnostics. All four ordinary history modes remain refused pending verified bootstrap, exact work and candidate-history support in Phases 165–167; exact root identity and retained synthetic regression success do not claim canonical acceptance.

## Self-Check: PASSED

All three new Rust paths, changed existing paths, breadcrumb mapping and this summary exist. Recorded focused and regression outputs substantiate the claims. Scoped formatting, breadcrumbs, file lengths and whitespace checks passed. Commit checks are intentionally inapplicable because strict phase finalization is parent-owned; no hashes or completed requirements are fabricated.
