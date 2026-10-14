---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "05"
subsystem: rpc
tags: [rust, canonical-context, first-effect-guard, typed-refusal, fixture-contracts]
requires:
  - phase: 164-02
    provides: Non-opening ordinary storage compatibility refusal
  - phase: 164-03
    provides: Immutable canonical owner query and ordinary support facts
  - phase: 164-04
    provides: Shared exhaustive runtime selector resolution
provides:
  - Fallible ordinary context factories and supplied-owner guard before startup mutations
  - Typed wallet/mempool fallback refusal and explicit cross-crate fixture factories
  - Ten canonical context preservation tests awaiting coupled caller migration
affects: [164-09, 164-10, 164-11, 164-12, 164-13, 165, 166, 167]
tech-stack:
  added: []
  patterns: [shared pure selector, immutable owner qualification, named fixture assembly, fixed diagnostics]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs
    - packages/open-bitcoin-rpc/src/context/canonical_chain/tests/fixtures.rs
  modified:
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/wallet_state.rs
    - packages/open-bitcoin-rpc/src/context/mempool_recovery.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Ordinary constructors never select fixture factories, including under cfg(test).
  - Matching canonical identity remains unsupported; raw supplied owners cannot authorize context mutation.
  - Storage and authority failures retain typed causes while displaying fixed bounded categories.
requirements-completed: []
requirements-addressed: [CBOT-04, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T04:32:00Z"
duration: 16min
completed: 2026-10-14
---

# Phase 164 Plan 05: RPC Canonical Context Guards Summary

**Ordinary RPC construction rejects unproved or mismatched owners and unsupported canonical history before prune, wallet, mempool or backend effects.**

## Performance and Completion Boundary

- Plan-owned implementation tasks: 2/2; approximately 16 minutes, 04:16–04:32 UTC.
- Nine implementation artifacts plus this summary.
- Source checks passed. Coupled Cargo compilation and all behavioral GREEN evidence remain explicitly assigned to Plan 13 after Plans 09–13 migrate the 39 fixture caller files.
- No staging, commits, pushes, hook changes, worktrees or shared tracker/state/roadmap/requirements changes were performed. Parent owns strict phase finalization after complete verification; no commit hashes or completed requirements are claimed.

## Accomplishments and First-Effect Inventory

| Surface | Guard and effect boundary |
| --- | --- |
| `try_from_runtime_config` / `try_from_runtime_config_with_store` | Resolve through public `config::selected_canonical_chain`, require ordinary support, then prepare wallet/network. No raw opener, fixture factory or ordinary `expect` bypass. |
| Ordinary `from_runtime_config` family / `for_local_operator` | Fallible aliases to guarded ordinary construction. Existing single-network operator alias still validates its resulting full runtime configuration. |
| Actual generic `from_runtime_config_with_network_handle` | Pure selector resolution, read-only `maybe_canonical_chain`, unproved/mismatch checks and ordinary support refusal precede the first `configure_context_network` mutation. No owner rebinding. Generic arguments and supplied-handle shape remain unchanged. |
| Wallet no-handle fallback | `FjallNodeStore::open_ordinary(data_dir, selected_chain)` returns typed compatibility refusal; no guessed registry or local replacement after refusal. Supplied-store and memory branches require ordinary support. |
| Mempool no-handle fallback | Ordinary opener runs before clock/recovery-status/loader effects. Compatibility errors propagate distinctly; supplied-store and absent-path branches require support. |
| Shared assembly | Private policy configuration, final context assembly, wallet-state constructors and existing mempool loader avoid duplicated runtime machinery. Ordinary callers qualify before reaching them. |
| Raw storage / fixture wrappers | Raw wallet open exists only in named `build_fixture_wallet_state_with_store`; raw mempool fallback exists only in named `recover_fixture_mempool_snapshot_from_store_handle` and its existing test module. `from_network_fixture` occurs only in explicit fixture construction or documented low-level `new`. |

`context::CanonicalContextError` exports `CompatibilityRefusal`, `UnsupportedValidationScope`, `UnprovedOwner`, `ChainMismatch`, `Storage(StorageError)` and `Authority(ManagedNetworkAuthorityError)`. Storage errors remain typed; backend text is excluded from Display. Fixture authority failures use a fixed storage failure category to preserve the required fixture `StorageError` return convention.

## API Handoff for Plans 09–13

| API | Return contract |
| --- | --- |
| `try_from_runtime_config(&RuntimeConfig)` | `Result<Self, CanonicalContextError>` |
| `try_from_runtime_config_with_store(&RuntimeConfig, Option<FjallNodeStore>)` | `Result<Self, CanonicalContextError>` |
| `from_runtime_config(&RuntimeConfig)` | `Result<Self, CanonicalContextError>` |
| `from_runtime_config_with_store(&RuntimeConfig, Option<FjallNodeStore>)` | `Result<Self, CanonicalContextError>` |
| Generic `from_runtime_config_with_network_handle(&RuntimeConfig, ManagedNetworkHandle<S, V>, Option<FjallNodeStore>)` | `Result<Self, CanonicalContextError>` |
| `for_local_operator(AddressNetwork)` | `Result<Self, CanonicalContextError>` |
| `from_fixture_runtime_config(&RuntimeConfig)` | `Self` |
| `from_fixture_runtime_config_with_store(&RuntimeConfig, Option<FjallNodeStore>)` | `Result<Self, StorageError>` |
| Generic `from_fixture_runtime_config_with_network_handle` with the same three arguments | `Result<Self, StorageError>` |
| `for_fixture_local_operator(AddressNetwork)` | `Self` |

Fixture APIs are public for cross-crate tests/benchmarks, narrowly documented as synthetic/source-derived evidence, never ordinary canonical authorization. They are not cfg-test-only. The actual daemon call remains ordinary; only four unrelated unused context arguments in Plan 04's canonical metrics tests now explicitly use the fixture factory. Their metrics/daemon refusal calls and assertions remain ordinary.

## Written Preservation Evidence

Ten new `context::canonical_chain::tests::canonical_chain_*` tests call actual ordinary APIs. They cover four unbound memory owners; all 16 selected/bound memory pairs; all 12 selector conflicts; every ordinary memory alias; a store-call spy reset after observed initialization that requires zero subsequent calls on refusal; durable-backed bound/unbound owners with wallet/registry/selection, chain/coins, index/header, runtime/recovery, prune and mempool snapshots plus exact physical byte manifests; the daemon's actual `FjallChainstateStore/FjallCoinsView` unbound runtime handle; both real fallback helpers over missing, empty, hostile, synthetic and root-looking closed sources; explicit fixture construction; and bounded typed storage diagnostics.

Owner observations include operator/peer/mempool facts, coins/tip, prune mode/locks, recovery facts, immutable binding and checkpoint generation. Closed-source manifests compare names, file types, bytes, lengths, mtimes, permissions/modes and symlink targets, excluding atime. Canonical owner pair tests use a real Fjall-backed store with the publicly constructible memory coins view; a separate actual durable-runtime test covers the daemon's exact generic handle type without exposing private recovery constructors.

These are written tests, not passing behavioral receipts. Plan 13 must run them nonzero and also run existing context/config/dispatch/inbound/HTTP, daemon/filter/prune, cross-crate and benchmark regressions after mechanical migration closure. No ordinary-history acceptance or production readiness is claimed.

## Verification and RED Evidence

| Check | Actual result |
| --- | --- |
| Initial focused RED compile, session 79436 | Exit 101: 89 errors, including expected old infallible callers and three owned errors (wrong support-method receiver and two private node test-construction APIs). Owned errors corrected using `ChainSpec::require_ordinary_history` and public fixture APIs. |
| Corrected RED compile, session 64883 | Exit 101: 86 remaining errors in unmigrated fixture callers; no owned canonical/context-helper diagnostics. |
| Final focused RED compile, session 12810 | `bun run scripts/command-timings.ts run --key phase164-rpc-constructor-red -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --lib canonical_chain --message-format=short`: exit 101, same 86 pending caller errors, zero owned-path errors. Log: `/tmp/open-bitcoin-phase164-rpc-red.log`. No behavioral test execution occurred. |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts --check`: exit 0; 1,156 Rust paths registered. |
| Managed standards | `bun scripts/bright-builds-check.ts all`: exit 0; zero file-length or lesson findings. |
| Formatting and whitespace | Scoped Rust formatting and source-diff checks passed; final scoped check recorded below. |

No isolated RPC GREEN, full native verifier, test pass count or requirement completion is inferred. Full native verification and independent review/security remain parent-owned after coupled closure.

## Deviations from Plan

**[Rule 2 - Required maintainable file boundary] Split constructor and preservation-test responsibilities.** Parent authorized `canonical_chain/tests.rs` and `canonical_chain/tests/fixtures.rs` in addition to the originally listed paths. Existing `network.rs` began at 626 lines; moved its default-type constructor impl into the owned canonical module instead of adding a length exception or compressing whitespace. Registered both new test paths.

**[Rule 3 - Coupled unused test setup] Updated four Plan 04 metrics context arguments.** Parent explicitly authorized changing only unrelated unused default contexts in `bin/open_bitcoind/tests/canonical_chain.rs` to `from_fixture_runtime_config`; the actual ordinary metrics refusal assertions remain intact.

## Simplification and Diff Review

Kept one shared selector resolver and one private final context assembly. Extracted startup policy setup, shared wallet-state constructors and the existing open-store mempool recovery path; removed an unused configuration argument and unnecessary policy clone. No scope boolean, production cfg-test branch, duplicated daemon network table, new dependency or new endpoint was added. No global consensus maturity change, historical receipt edit or third-party source change occurred.

The local AGENTS guidance, sidecar, overrides, architecture/code-shape/verification/testing/Rust standards and both complete active lesson files informed execution. No new security surface exceeds the planned supplied-owner/storage boundaries. No goal-blocking stubs or authentication gates were found.

## Task Commits

1. Guarded ordinary/supplied-owner constructors and fixture contracts: deferred parent-owned strict phase finalization.
2. Guarded fallback propagation, preservation tests and breadcrumbs: deferred parent-owned strict phase finalization.

## Remaining Coupled Verification

Plans 09–13 must mechanically move legacy fixtures to the explicit fixture APIs. Plan 13 must establish nonzero passing new canonical tests and all existing required regressions before phase/requirement completion or Git finalization. The 86 current caller compile errors are the intentionally disclosed coupled migration boundary, not a completed GREEN gate.

## Self-Check: PASSED

All nine implementation artifacts and this summary exist. Source guards and named fixture paths were reviewed, owned compiler diagnostics are absent from final RED evidence, and source formatting/breadcrumb/file-length/whitespace checks pass. This self-check verifies artifacts and the stated source evidence only; behavioral and coupled compile acceptance remain pending Plan 13. Commit checks are intentionally inapplicable under parent-owned Git finalization.
