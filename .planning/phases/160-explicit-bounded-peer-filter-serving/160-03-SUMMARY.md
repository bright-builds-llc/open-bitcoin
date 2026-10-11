---
phase: 160-explicit-bounded-peer-filter-serving
plan: "03"
subsystem: daemon
tags: [basic-filters, configuration, startup, permissions, handshake]
requires:
  - phase: 160-02
    provides: Typed requested-versus-expanded capability and prune-safe service composition
provides:
  - Default-off peerblockfilters CLI and bitcoin.conf resolution
  - Shared BASIC prerequisite validation before storage selection and runtime recovery
  - Configured shared daemon authority before inbound and outbound version construction
affects: [160-08, 160-09, 160-10, 160-11, 160-12]
tech-stack:
  added: []
  patterns: [validate-before-effects, single configured authority, requested permission provenance]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs
  modified:
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/config/loader.rs
    - packages/open-bitcoin-rpc/src/config/loader/blockfilter.rs
    - packages/open-bitcoin-rpc/src/config/tests/blockfilter.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-network/src/inbound/permissions.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
key-decisions:
  - Reuse one RuntimeConfig prerequisite validator in the loader and both daemon startup boundaries.
  - Read parsed permission classes through an immutable registry slice so expanded all never substitutes for an explicit request.
  - Configure the selected durable or transient handle before publication; existing listener startup consumes that same configured authority.
requirements-addressed: [CFNET-01]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 160-20261010-peer-filter-serving
generated_at: "2026-10-10T19:32:28Z"
duration: approximately 15min
completed: 2026-10-10
---

# Phase 160 Plan 03: Explicit Daemon Capability Before Effects Summary

**Daemon startup validates BASIC prerequisites before storage effects and configures its shared authority before version construction, preserving the all-only/no-backend exception.**

## Accomplishments

- Added default-false `RuntimeConfig.peer_block_filters`, narrowly resolved from `peerblockfilters`/`nopeerblockfilters` CLI and applicable `bitcoin.conf` sections. The existing boolean parser is reused. CLI scalar selection wins over active-network then default configuration, and configuration negations reset prior values with first-scalar selection resuming after the reset.
- Added `RuntimeConfig::validate_peer_block_filters()`, called by the loader, before `open_runtime_store` selection and before `open_authoritative_network_runtime` recovery. General activation and explicitly requested `blockfilters`, including `all,blockfilters`, refuse with actionable pinned vocabulary when BASIC is disabled or omitted.
- Preserved `all` alone, implicit-equivalent classes, `noban`, `download`, `forceinbound` and Bloom behavior. Index enablement, general serving, listener activation, relay and public exposure remain separately configured.
- Added `ManagedNetworkHandle::configure_basic_peer_serving(bool)` over Plan 02 composition. Both daemon authority branches call it before the handle can reach sync or listener workers; the durable branch retains the existing owner and shared handle.
- Observed configured CF plus `NETWORK_LIMITED | WITNESS` without `NETWORK` in both real authority version constructors while the index remained lagged and initial synchronization false. A production loopback listener wrote a matched all-only peer's CF version with explicit index disablement, no durable backend and no global CF bit.

Material guidance: AGENTS.md, its Bright Builds sidecar, placeholder-only overrides, architecture/code-shape/testing/verification/operability/Rust standards, finalized context/research/decision coverage and the preloaded active lessons informed this work. Parent synchronization and cooperative Cargo serialization were preserved.

## Task Acceptance

### Task 1: Resolve peerblockfilters and enforce explicit/all prerequisites

- [x] Bare, empty, 1, 0, negated and double-negative forms use the existing boolean contract.
- [x] CLI overrides configuration; active network overrides defaults; inactive sections are ignored. Repeated config scalars and negation reset/revival are covered.
- [x] General true without BASIC refuses; explicit `blockfilters` and `all,blockfilters` refuse with both omitted and explicit-zero indexes. BASIC accepts both explicit classes.
- [x] `all` alone and unrelated permission classes require no index. Outbound permission-class grammar remains rejected.
- [x] No configuration case implicitly enables listeners, relay, block relay, pruning or history acquisition.

### Task 2: Publish capability on actual startup handles

- [x] Actual parsed general activation reaches the durable shared authority before outbound and inbound version construction.
- [x] Both own versions advertise configured CF before catch-up and preserve prune service classification after RPC context construction.
- [x] Invalid typed runtime configurations refuse before store creation or runtime recovery; retained bodies/undo, prune state, locks and a previously saved enabled index owner remain unchanged.
- [x] Explicit index-zero all-only startup selects the existing transient path with no backend initialization. A matched actual loopback peer receives CF while the global base remains off.
- [x] Existing index-only, disabled, offline, sync and inbound startup regressions pass. No second durable owner or default public listener was introduced.

## Verification and TDD Evidence

All Cargo commands used pinned Bun 1.3.9 through `bun run scripts/command-timings.ts run --key <stable-key> --`, respecting the cooperative build lock. Nonmatching integration binaries executing zero tests are not counted as task evidence.

| Stage | Cargo command after timing wrapper | Result |
| -- | -- | -- |
| Task 1 RED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase160_config -- --nocapture` | 2 executed, 2 failed: unrecognized general option and missing prerequisite refusal. The first explicit fixture omitted its existing required `in` direction; it was corrected before GREEN. |
| Task 1 GREEN | Same command | 5 executed, 5 passed after adding the form/precedence/permission matrix. |
| Negation refinement RED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib phase160_config_conf_precedence -- --nocapture` | 1 executed, 1 failed: a later configuration negation did not reset the prior true scalar. |
| Negation refinement GREEN | Task 1 command | 5 executed, 5 passed, including reset/revival. |
| Task 2 RED | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind phase160_startup -- --nocapture` | 2 executed, 2 failed: invalid general configuration selected a runtime and durable startup omitted CF. |
| Task 2 GREEN | Same command | 6 executed, 6 passed, including actual loopback advertisement. |
| RPC library regression | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib` | 325 executed, 325 passed; before the final negation refinement, which was separately verified afterward. |
| Daemon regression | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind` | 80 executed, 80 passed in 131.47s. Existing automatic-prune/index-protection work completed normally after its 60-second progress notice. |
| Final combined scoped tests | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc phase160_ -- --nocapture` | 5 library plus 6 daemon tests executed and passed after the final production change. |
| Targeted lint | `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings` | Passed, including production network/node dependencies. Plan 05 independently reports network/node all-target/all-feature Clippy passing. |
| Formatting and diff | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all nine owned Rust paths; `git diff --check` | Passed. |

The loopback fixture initially used unavailable Tokio `io-util` extension traits; it now uses existing standard-library I/O in a bounded blocking task, with no dependency or feature change. Its all-expanded protected admission correctly requires an explicit reserved slot; fixture configuration was adjusted without relaxing production policy.

## Interfaces and Downstream Notes

- `RuntimeConfig.peer_block_filters: bool` defaults false. No JSONC schema extension was added.
- `RuntimeConfig::validate_peer_block_filters(&self) -> Result<(), ConfigError>` owns shared activation refusal. Both startup helpers call it before their first effectful opening/recovery operation.
- `PeerPermissionClassRegistry::classes(&self) -> &[ParsedPeerPermissionClass]` exposes immutable typed classes for requested-token validation. The parent approved this narrow additional Plan 02 file change.
- `ManagedNetworkHandle::configure_basic_peer_serving(&self, enabled: bool) -> Result<(), ManagedNetworkAuthorityError>` changes configured general capability under the existing authority mutation lock. Callers must validate prerequisites before activation; capability remains independent of backend readiness.
- `inbound_startup.rs` needed no mutation: daemon startup already passes the selected shared handle to its production listener function after configuration. The new loopback test calls that actual function with the selected transient authority.
- Later plans still own actual compact-filter request serving, achieved response transport, broad protocol scenarios, documentation/README/UAT reconciliation and phase-wide verification. The socket test here proves version advertisement only.

## Deviations from Plan

**1. [Rule 2 - Missing Critical] Exposed immutable parsed permission classes for one shared prerequisite guard.**

- Found during Task 1/Task 2 integration: runtime startup cannot inspect explicit provenance through the registry's prior matching-only interface without reconstructing or duplicating configuration parsing.
- Added the parent-approved immutable `classes()` getter in `packages/open-bitcoin-network/src/inbound/permissions.rs`; both loader and startup now reuse one validator. No mutable exposure, new authority or new permission parser was added.
- Verification: omitted/zero index explicit and all-plus-explicit refusal, accepted all-only startup and full configuration/daemon regressions.
- Commit: deferred to outer consolidated gate.

**2. [Rule 1 - Bug] Preserved configuration negation reset semantics.**

- Found during simplification/source review against pinned `common/settings.cpp` after the initial GREEN implementation.
- Added a failing reset/revival regression, then selected the first scalar after the most recent negation in each configuration section. The existing boolean parser remains shared.
- Files: `config/loader/blockfilter.rs`, `config/tests/blockfilter.rs`.
- Verification: nonzero RED followed by five passing configuration tests and the final eleven-test combined run.
- Commit: deferred to outer consolidated gate.

## Security, Simplification and Stub Review

- T-160-07: validation is the first operation in store selection and authoritative runtime opening, before durable startup can resume prune recovery or alter index lifecycle. Direct tests preserve source bodies/undo, prune flags, locks and saved index state. No new raw prune-intent test injection API was added.
- T-160-08: default false and independent activation fields preserve listener/public/relay defaults; full existing config and daemon regressions pass.
- T-160-09: configure the existing selected handle, then share its clone with context and adapters. Both version constructors observe the same capability after context prune configuration.
- Simplification retained one typed prerequisite validator and one narrow handle setter. Configuration resolution remains in the existing blockfilter loader child; no extra production module, dependency, backend or status surface was added. Loader length remains 628 lines and tests 618 lines.
- Stub scan and scoped diff review found no introduced placeholder implementation, TODO/FIXME, unwired data source, schema change, new endpoint or unmodeled trust-boundary surface. Existing tests and other executors' edits were preserved.

## Breadcrumb Registration Handoff

The parent owns serial manifest registration. Exactly one Rust path was created:

`packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs`

Its in-source ordered anchors are `packages/bitcoin-knots/src/init.cpp`, `packages/bitcoin-knots/src/protocol.h`, then `packages/bitcoin-knots/src/net_processing.cpp`. Existing modified files retain their breadcrumb groups. This executor did not modify `docs/parity/source-breadcrumbs.json`.

## Task Commits and Shared State

Both tasks and this summary await the strict outer consolidated commit. No staging, commit, push, branch, worktree, hook bypass, STATE, ROADMAP, requirement-checkbox or task-ledger change occurred. `requirements-completed` remains empty. Parent-owned native verification, independent review and lifecycle-valid phase verification remain required before finalization.

## Authentication Gates and Deferred Issues

None. Transient compile warnings from the concurrently implemented Plan 05 were coordinated with its owner and absent from final passing lint. Broad operator projection and integrated later-phase claims remain explicitly out of scope.

## Self-Check: PASSED

All nine owned Rust paths and this summary exist. Final scoped formatting, diff whitespace, five configuration tests, six startup tests and targeted all-target/all-feature lint passed. Full RPC library and daemon regressions passed as recorded above. Commits are intentionally deferred; no commit hash or requirement-completion claim is fabricated. Shared planning artifacts and other executors' changes were preserved.
