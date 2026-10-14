---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "04"
subsystem: daemon
tags: [rust, canonical-selection, ordinary-startup, metrics, source-preservation]
requires:
  - phase: 164-02
    provides: Non-opening ordinary Fjall compatibility refusal
  - phase: 164-03
    provides: Ordinary durable-runtime support guard before initialization
provides:
  - Shared typed selection of canonical identity from existing RPC configuration
  - Daemon and metrics refusal before backend or accepted-authority effects
  - Explicit test-only historical daemon startup adapter
affects: [164-05, 164-06, 164-09, 164-12, 164-13, 165, 166, 167]
tech-stack:
  added: []
  patterns: [shared exhaustive selector mapping, ordinary opener before daemon effects, named cfg-test fixture assembly]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/config/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs
  modified:
    - packages/open-bitcoin-rpc/src/config.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/automatic_prune.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/phase162/fixtures.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush/tests/filter_index.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - One shared pure selector resolver rejects all inconsistent chain/sync selections before effects.
  - No ordinary transient daemon owner is constructed while canonical validation prerequisites are absent.
  - Legacy synthetic daemon tests use explicit cfg-test startup and share durable assembly and metrics-worker machinery.
requirements-completed: []
requirements-addressed: [CBOT-04, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T04:15:31Z"
duration: 22min
completed: 2026-10-14
---

# Phase 164 Plan 04: Daemon Compatibility Gates Summary

**Ordinary daemon startup and inbound metrics refuse unproved storage and unsupported canonical history before backend, initialization, authority or worker effects.**

## Performance

- Tasks completed: 2/2; focused regressions, lint and production linking passed.
- Approximate execution duration: 22 minutes from the parent-recorded Plan 04 start at 03:53:16 UTC to final verification at 04:15:31 UTC.
- Implementation files: 15; this summary is the sixteenth artifact.
- All staging, commits, pushes, hook operations, worktrees and shared tracker/state/roadmap/requirements updates remain parent-owned under strict phase finalization.
- Requirements remain pending until lifecycle-valid complete phase verification.

## Accomplishments

- Added public pure `config::selected_canonical_chain(&RuntimeConfig) -> Result<CanonicalChain, CanonicalChainSelectionError>`. Existing `AddressNetwork` and `SyncNetwork` selectors map exhaustively to Mainnet/Testnet3/Signet/Regtest. A mismatch yields one fixed typed bounded diagnostic instead of choosing silently.
- Removed daemon `is_dir`/existence shortcuts from ordinary durable selection. Sync, inbound, either prune mode, explicit BASIC enable and explicit BASIC disable all route an available path through `FjallNodeStore::open_ordinary`; an absent configured path yields the same typed missing observation rather than a successful fallback.
- Guarded `open_authoritative_network_runtime` before examining supplied raw stores or constructing accepted owners. It uses the ordinary durable-runtime opener after the support boundary. The transient enum variant and synthetic transient constructor now exist only in the test build; no config, flag or environment route selects them.
- Guarded metrics fallback with the same selected-chain ordinary opener. Supplied lower-level handles and absent datadirs also require ordinary support before worker creation. Extracted one shared metrics spawn function so retained fixture tests exercise the actual worker machinery.
- Kept production main calls ordinary in test builds too. New refusal tests call `crate::open_runtime_store`, `crate::open_authoritative_network_runtime` and the production metrics entrypoint explicitly; shared test aliases cannot redirect them.
- Added physical manifests of real closed Fjall fixtures comparing names, types, lengths, bytes, link targets, permissions/modes and mtimes, excluding atime. Four selectors exercise all 31 nonzero durable-trigger bitmasks, missing/empty and absent paths, raw supplied authority, metrics missing/empty/closed paths, supplied metrics handles, all 12 selector conflicts, symlink aliases and regular-file paths. Hostile path names never enter diagnostics.
- Registered all three new Rust paths in the existing parity breadcrumb map.

## Task Commits

1. Task 1, ordinary daemon/metrics gates and refusal evidence: deferred strict phase finalization.
2. Task 2, explicit legacy fixture startup and call-site audit: deferred strict phase finalization.

No task hashes exist or are claimed. The parent owns Git finalization after complete clean phase verification.

## Classified Opener Inventory

| Call site | Classification and effect boundary |
| --- | --- |
| `open-bitcoind.rs::main` | Actual product entrypoint calls ordinary store selection, then ordinary authority selection, before `serve_authoritative_runtime`, RPC context, listeners, logs and workers. |
| `open-bitcoind.rs::open_runtime_store` | Product ordinary opener only; fixed compatibility refusal before Fjall. No raw backend opener remains. |
| `open-bitcoind.rs::open_authoritative_network_runtime` | Product selector/support guard before supplied-store initialization; uses `DurableSyncRuntime::open_ordinary_configured`. No ordinary transient accepted-history construction. |
| `open-bitcoind.rs::assemble_durable_runtime` | Shared lower-level assembly receiving an already constructed runtime; no backend open. The ordinary caller reaches it only after both ordinary guards. |
| `inbound_metrics.rs::start_inbound_metrics_worker` | Product no-handle ordinary opener; supplied handles require support before shared spawn. No raw fallback remains. |
| `tests/fixture_runtime.rs` | Named test-only raw storage, configured synthetic durable runtime and transient construction; compiled exclusively through the root `#[cfg(test)] mod tests`. No ordinary CLI/config selection. |
| Existing daemon raw storage seeds | Owned test-only synthetic fixtures; their success is regression evidence, never compatibility proof. |
| RPC `context/network.rs` | Two existing `from_network_fixture` construction sites remain Plan 05-owned at this handoff. |
| RPC `context/wallet_state.rs` | Existing registry raw-open fallback remains Plan 05-owned at this handoff. |
| RPC `context/mempool_recovery.rs` | Existing product no-handle raw open remains Plan 05-owned; its other raw open is inside an existing test module. |

The inventory deliberately records remaining next-plan work rather than claiming every RPC context was already converted. No `Database::builder` or `open_unrecovered` call exists in the daemon or metrics adapter.

## Verification and TDD Evidence

All Cargo commands run serially through the checkout's cooperative timing wrapper.

| Stage | Actual result |
| --- | --- |
| Initial RED compile, session 23221 | Exit 101: test referenced nonexistent `load_active_chain`; corrected to public `load_chainstate_snapshot` before behavioral RED. |
| Behavioral RED, session 36457 | `phase164-daemon-compatibility` command exited 101: all four original refusal tests failed against the old successful/raw-opening behavior. |
| First GREEN, session 80724 | 3 passed, 1 failed: ordinary classifier correctly rejected macOS's symlinked temp ancestor as unsafe. |
| Diagnostic fixture check, session 46744 | Confirmed fixed `unsafe_path` refusal; changed only the owned fixture parent to a canonical physical temp path and added explicit unsafe-path regressions. |
| Focused GREEN, session 17552 | Exit 0: 4 passed, 105 filtered. |
| Expanded focused GREEN, session 7318 | `bun run scripts/command-timings.ts run --key phase164-daemon-compatibility -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind canonical_chain`: exit 0, 8 passed, 105 filtered; 13.39s test runtime. |
| Initial full binary regression, session 14940 | Exit 101: 83 passed, 25 failed, 1 ignored, 0 filtered, 311.87s. All 25 failures trace to three additional legacy test-only fixture callers of newly guarded product helpers; the integrated Phase 162 lifecycle itself passed. |
| Final full binary regression, session 99839 | `bun run scripts/command-timings.ts run --key phase164-daemon-legacy-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind tests::`: exit 0, 112 passed, 0 failed, 1 intentionally ignored, 0 filtered; 332.36s test runtime. |
| Shared selector regression, session 70945 | `bun run scripts/command-timings.ts run --key phase164-canonical-selection -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib canonical_chain_selection`: exit 0, 1 passed covering all 16 pairs, 376 filtered; 2m09s compilation. |
| Focused RPC lint, session 68664 | `bun run scripts/command-timings.ts run --key phase164-daemon-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-targets --all-features -- -D warnings`: exit 0, 11.87s. |
| Production binary build, session 23759 | `bun run scripts/command-timings.ts run --key phase164-daemon-production-build -- cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind`: exit 0, 4.95s; confirms the non-test compilation without transient fixture construction. |
| Source authority check | `bun scripts/check-phase127-authoritative-network-state-unification.ts`: exit 0. |
| Earlier source timing check | `bun scripts/check-phase123-runtime-timing-evidence-integrity.ts`: exit 1, only `P123 daemon sync activation wiring must preserve configured activation arguments`; old product-constructor token is superseded by the ordinary opener. Parent later source-check adaptation owns reconciliation. |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts --check`: exit 0, 1,153 Rust paths verified. |
| Managed standards | `bun scripts/bright-builds-check.ts all`: exit 0, no file-length or lesson findings. |
| Formatting and whitespace | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all 14 Rust paths and `git diff --check`: exit 0. |

The parent owns full native verification and independent reviews. No production startup, public-network acceptance, bootstrap, work compatibility or whole-chain parity is inferred from these fixture passes.

Actual nonzero discovery from that final binary execution:

| Binary module | Discovered | Passed | Ignored |
| --- | ---: | ---: | ---: |
| `tests::daemon_sync` | 14 | 14 | 0 |
| `tests::filter_index` | 46 | 45 | 1 |
| `tests::automatic_prune` | 7 | 7 | 0 |
| `tests::canonical_chain` | 8 | 8 | 0 |
| `coins_flush` | 15 | 15 | 0 |

The sole ignored case is the pre-existing subprocess child control, intentionally invoked only by its bounded subprocess driver. No test was ignored to evade the new ordinary guards.

## Deviations from Plan

**[Rule 2 - Missing critical shared boundary] Added reusable selector resolution.** Parent approved `config/canonical_chain.rs` plus minimal `config.rs` export wiring so RPC and CLI use the same exhaustive selection and typed conflict refusal. Keeping a daemon-private table would duplicate the authority boundary across later plans. Both files are included in this executor's ownership; no I/O or dependency was added.

**[Rule 3 - Blocking fixture callers] Mechanically adapted three additional test-only paths.** The first actual binary run exposed callers not present in the plan's four-child-file list. Parent explicitly authorized only constructor/import substitutions in `coins_flush.rs`'s `#[cfg(test)] phase159_tests`, `coins_flush/tests/filter_index.rs`, and `tests/filter_index/fixtures.rs`. Assertions, fixture recipes, original receipts and historical documentation are unchanged. The four originally planned mechanical imports are `tests/filter_index.rs`, `tests/automatic_prune.rs`, `tests/filter_index/peer_serving.rs` and `tests/filter_index/phase162/fixtures.rs`.

## API and Downstream Handoff

- Reuse exported `open_bitcoin_rpc::config::{selected_canonical_chain, CanonicalChainSelectionError}` for later RPC/CLI adapters. `Testnet` selectors map to canonical `Testnet3`; all 16 selector pairs have a pure resolver regression.
- `tests::fixture_runtime::{fixture_open_runtime_store, fixture_open_authoritative_network_runtime, fixture_start_inbound_metrics_worker}` are named test-only adapters. They share `assemble_durable_runtime`, `basic_filter_startup_mode` and actual `spawn_inbound_metrics_worker` machinery instead of copying a second daemon/worker implementation.
- Plan 05 will need to convert unrelated, unused `ManagedRpcContext::from_runtime_config(default)` setup in new metrics refusal tests to its named fixture-context constructor once ordinary context construction refuses. The actual metrics and daemon entrypoints under assertion must stay ordinary. Existing legacy RPC fixture context adaptation remains Plan 09-owned.
- The Phase 123 current-source constructor check needs narrow parent-owned reconciliation. Its original receipts and assertions were not rewritten here. Phase 127's structural daemon composition guard already passes.

## Simplification and Diff Review

Kept one pure configuration mapping, one ordinary storage opener, one shared durable assembly helper and one shared metrics-worker implementation. Historical transient construction moved wholly into the named test adapter instead of adding a production boolean or a test branch inside ordinary helper functions. Only mechanical imports/calls changed in the seven legacy fixture paths; no assertions were weakened. No third-party dependency, backend inspection parser, repair fallback, source mutation or new network selector was introduced.

The local AGENTS guidance, Bright Builds sidecar, overrides, architecture/code-shape/verification/testing/Rust standards and both complete active lesson files informed execution. Historical Phase 162/163 receipts and documentation remain untouched. The added files contain no goal-blocking stubs and introduce no unexpected endpoint, auth or filesystem trust surface beyond the planned ordinary boundary.

## Remaining Phase Work

No plan-owned implementation or focused verification remains. Full phase verification, current-source checker reconciliation and requirement completion remain parent-owned. No authentication gate occurred.

## Self-Check: PASSED

Checked all 15 implementation artifacts and this summary: all 16 files exist. Focused ordinary-refusal tests, complete binary regression, pure selector regression, RPC all-target/all-feature lint, production build, source authority check, breadcrumb registration, managed standards, scoped Rust formatting and whitespace checks passed. The exact earlier Phase 123 source-check failure remains disclosed for parent-owned reconciliation. Historical Phase 162/163 artifacts have no executor diff. No commits were expected or claimed under the strict parent-owned Git contract; no staging, commits, pushes, hook skips, worktrees or shared tracker mutations were performed.
