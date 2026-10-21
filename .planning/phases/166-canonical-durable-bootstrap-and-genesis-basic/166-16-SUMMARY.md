---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "16"
subsystem: rpc
tags: [canonical-genesis, sealed-capability, unavailable-wallet, durable-reopen]
requires:
  - phase: 166-08
    provides: Canonical genesis runtime and saved Active BASIC ownership
  - phase: 166-15
    provides: Same-executable isolated inspection worker and registration macro
provides:
  - Specialized consuming canonical genesis RPC context/runtime factory
  - Explicit unavailable wallet with fixed fallible refusals before effects
  - Four-selector startup, actual reopen and physical/logical refusal evidence
affects: [166-09, 166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [sealed consumption, shared assembly, immutable preflight, scoped process-test serialization]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/context/canonical_genesis.rs
    - packages/open-bitcoin-rpc/src/context/canonical_genesis/tests.rs
    - packages/open-bitcoin-rpc/src/context/canonical_genesis/tests/refusals.rs
  modified:
    - packages/open-bitcoin-rpc/src/context.rs
    - packages/open-bitcoin-rpc/src/context/wallet_state.rs
    - packages/open-bitcoin-rpc/src/context/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/context/network.rs
    - packages/open-bitcoin-rpc/src/context/rescan.rs
    - packages/open-bitcoin-rpc/src/context/tests/construction.rs
    - packages/open-bitcoin-rpc/src/lib.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Consume OpenedCanonicalGenesis by value and construct the actual runtime internally once.
  - Derive consensus parameters and effect paths from proved genesis/storage; retain ordinary history gates.
  - Preserve HTTP wallet errors and existing direct importdescriptors per-item failure responses.
  - Separate closed-source physical preservation from after-capability logical preservation and zero initialization.
requirements-completed: [CBOT-02, CBOT-03, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T18:22:39Z"
commits: false
duration: 25m 7s
completed: 2026-10-20
---

# Phase 166 Plan 16: Sealed Canonical Genesis RPC Context Summary

**A consumed canonical genesis opener now produces its own durable RPC context/runtime pair, with selected consensus parameters, actual BASIC observations and no wallet authority.**

## Completion Boundary

Tasks: **3/3 complete**. Recorded implementation/check window: 2026-10-20T17:57:32Z–18:22:39Z; earlier read-only preparation is excluded. Eleven implementation/evidence paths changed, plus this summary. The factory has 88 lines, its test root 553, refusal module 216 and wallet state 578; all remain below 629 lines.

Requirement frontmatter records this plan's contribution, not whole-phase closure. No staging, commit, push, hook bypass, shared STATE/ROADMAP/REQUIREMENTS/task edit, LOC refresh or historical contract renewal occurred. Root owns independent acceptance, subsequent 09 integration and final normal-hook saving.

## Implemented Boundary

`packages/open-bitcoin-rpc/src/context/canonical_genesis.rs` exposes only `ManagedRpcContext<FjallChainstateStore,FjallCoinsView>::from_canonical_genesis_runtime_config(&RuntimeConfig, OpenedCanonicalGenesis) -> Result<(Self,DurableSyncRuntime),CanonicalContextError>`. It checks both selectors, capability chain, peer-filter prerequisites and all five activation requests before its sole canonical runtime call. The opener is consumed by value; no borrowed-runtime, raw-store, fixture, numerical-work or caller-handle substitute is accepted.

The factory maps Basic/Disabled/Unspecified to Enabled/Disabled/PreserveSaved, captures proof-derived consensus parameters, derives its handle/store from the resulting runtime and reuses the existing assembler. A cloned assembly config binds its data directory to `runtime.store().datadir()`. Both absent and divergent caller paths are tested; no alternate target is created. The ordinary supplied-handle constructor, wallet builder and history gate retain their bodies; the assembler's only change is `pub(super)` visibility.

`WalletState::Unavailable` carries no wallet/registry store. Selection, metadata, balance, UTXO, descriptor/address, build/sign, snapshot and rescan APIs return the exact wallet error `wallet unavailable: canonical genesis history is not qualified`. Registry selection refuses before loading, rescan job/freshness return errors, and guard/prepare/save-failed callbacks remain uncalled. Request-name operations remain harmless. `wallet_info` now returns Result; its sole existing construction caller unwraps the same successful fixture value without dropping assertions.

Authenticated HTTP wallet scope returns code -4 with that fixed error. Direct dispatch retains its existing importdescriptors protocol: an outer successful response contains only unsuccessful items with the fixed unavailable message. Direct singular import returns Err. Existing Local/Registry and ordinary-owner regression assertions remain intact.

## Actual Storage and Evidence Scope

All four selectors use genuine sealed openers and compare the actual context tip, coins B, empty effective coins, exact 64-digit work, selected consensus parameters and blockchain/BASIC dispatch against frozen independent vectors. Public `BasicFilterQuery::Found` supplies checked stored filter bytes/hash/header/zero predecessor; the public summary establishes the active owner. Disabled and absent Unspecified remain without an active index. Saved Active Unspecified reopens through accepted 08 ownership.

The retained-owner control drops the runtime while context/network/store owners remain and requires refusal; a retained store still blocks after the other handles drop. After every owner drops, the real existing-only opener runs the registered RPC executable's inspection child and reconstructs equivalent genesis/BASIC observations. No-job routing tests are classified separately from real recovery.

Closed foreign, raw and forged numerical/coins fixtures refuse at the real opener with identical source bytes, modes and mtimes; atime is excluded. After-capability config/activation refusals instead compare existing public logical authority/wallet/mempool/prune records using the same explicitly non-authorizing generic reader before capability acquisition and after refusal. Its preconditions are already-current schema and complete coverage, where schema/history recovery is a no-op. The observer never constructs a runtime/index or establishes authority.

Private BASIC rows are not claimed to have been directly compared by RPC tests. A cfg(test) thread-local counter immediately before the only runtime constructor is 1 in the positive control and 0 for every preflight refusal. The immutable opener exposes no writer before that boundary; source review rules out raw opens/other mutators. This combines zero initialization with the available logical observations without adding a public proof/record API. Physical equality is deliberately not asserted across an admitted live engine's teardown. Plan 09 retains the earlier pre-opener daemon physical-preservation boundary.

## Deviations and Limits

- **[Rule 3 — test structure]** Root amended ownership for `packages/open-bitcoin-rpc/src/context/canonical_genesis/tests/refusals.rs` after rustfmt expanded the unsplit test root to 702 lines. All names/assertions remain; no size exception was added.
- **[Rule 3 — test harness]** An initial focused run passed 20/21, failing a negative fixture's generic observer with a redacted store-lock error. Application and engine locks share that error, and the original stage was not recorded. A staged exact relay run passed once; this did not prove a fix or original cause. Root approved a test-only scenario mutex following the existing node process-test pattern for the known fork/exec descriptor-inheritance hazard. It spans each new canonical scenario, preserves intentional contention controls and adds no production change, retry, sleep or timeout relaxation. The resulting full focused run passes once. Existing RPC Bun-oracle tests and the node-owned macro's routing test remain outside that mutex; this is not a claim of whole-library process-test serialization.
- **[Rule 3 — aggregate freshness]** Root authorized manifest entries matching three paused 09 source headers. After 09's scoped formatting exposed a 659-line CLI support file, 09 extracted its existing helper unchanged into its own child module; only that child's existing explicit-none breadcrumb was registered here. The CLI source maintenance belongs to 09. Final aggregate checks pass without an exception.
- First Clippy found three test-only range-loop warnings. Iterator/enumerate forms preserve all selector cases; all three affected behavior cases pass again. Production behavior is unchanged from the 21-case GREEN.

The simplification pass retained one runtime construction and shared context assembly, with focused preflight/mode helpers and shared test setup. No new dependency, endpoint, core proof getter, schema, authentication path or unmodeled threat surface was introduced. No goal-blocking stub or auth gate remains. Repo AGENTS/native verifier/tool pins/parity rules, Bright Builds sidecar, overrides (no concrete exception), architecture/code-shape/verification/testing/local-guidance/Rust standards and both complete active lesson files informed execution: 8,856 bytes / 2,953 estimated tokens. No archives/project skills were loaded or lesson files edited.

Evidence is macOS/Unix, source-derived genesis/BASIC and software runtime/storage evidence. It grants no ordinary descendants, Signet challenge qualification, public activation, actual Knots-daemon differential, hardware power-loss or arbitrary-writer/crash guarantee. Actual Cargo/Bazel daemon continuation belongs to 09; contributor docs to 13; full default native/Bazel/LOC and independent phase review to 14/root.

## Automated Evidence

All Cargo commands ran serially through `scripts/command-timings.ts`, pinned Bun PATH, the approved isolated target and canonical process-local TMPDIR. Resumable commands were polled within 60 seconds; none was terminated for quiet output. Counts below are functions, not inflated internal selector loops.

| Check | Actual result | Full repository-relative receipt |
| --- | --- | --- |
| Wallet state scaffold RED, existing Local memory fixture | Exit 101; 0 passed / 2 failed expected refusal assertions; no durable product claim | `.local/open-bitcoin-dev/phase166/plan16/wallet-red.log` |
| Wallet state GREEN | Exit 0; 3/3 | `.local/open-bitcoin-dev/phase166/plan16/wallet-green.log` |
| Refusing factory scaffold RED | Exit 101; 0/4; executed test scaffold, not daemon startup | `.local/open-bitcoin-dev/phase166/plan16/factory-red.log` |
| Initial real fresh factory GREEN | Exit 0; 4/4 | `.local/open-bitcoin-dev/phase166/plan16/factory-green.log` |
| Reopen test initial compilation | Exit 101; private node reader API mistakes; no tests executed | `.local/open-bitcoin-dev/phase166/plan16/reopen-red.log` |
| Actual all-handle reopen before RPC worker registration RED | Exit 101; 0/1, StorageUnavailable after release | `.local/open-bitcoin-dev/phase166/plan16/reopen-behavior-red.log` |
| Expanded fixture compilation | Exit 101; named-parameter type and test-only node writer misuse; no tests executed | `.local/open-bitcoin-dev/phase166/plan16/expanded-first.log` |
| Expanded behavior | Exit 101; 8/9; incorrect import fixture wire field; actual reopen/physical cases pass | `.local/open-bitcoin-dev/phase166/plan16/expanded-second.log` |
| Initial full focused harness | Exit 101; 20/21; observer store-lock failure | `.local/open-bitcoin-dev/phase166/plan16/focused-final.log` |
| Exact staged relay diagnostic | Exit 0; 1/1, 2.92s; single observation | `.local/open-bitcoin-dev/phase166/plan16/relay-diagnosis.log` |
| Full focused harness with scoped serialization | Exit 0; 21/21, 232.33s; 20 behavior + 1 routing | `.local/open-bitcoin-dev/phase166/plan16/focused-serialized-final.log` |
| Final-source affected actual reopen | Exit 0; 1/1, all four selectors, 45.37s | `.local/open-bitcoin-dev/phase166/plan16/affected-reopen-final.log` |
| Final-source affected log-directory binding | Exit 0; 1/1, 5.49s | `.local/open-bitcoin-dev/phase166/plan16/affected-directory-final.log` |
| Final-source affected selector/capability refusal | Exit 0; 1/1, 24 combinations, 20.98s | `.local/open-bitcoin-dev/phase166/plan16/affected-selectors-final.log` |
| Unchanged ordinary-owner regressions | Exit 0; 10/10 | `.local/open-bitcoin-dev/phase166/plan16/ordinary-final.log` |
| Construction regressions | Exit 0; 5/5 | `.local/open-bitcoin-dev/phase166/plan16/construction-final.log` |
| Rescan eligibility regressions | Exit 0; 19/19 | `.local/open-bitcoin-dev/phase166/plan16/rescan-final.log` |
| Wallet dispatch regressions | Exit 0; 5/5 | `.local/open-bitcoin-dev/phase166/plan16/wallet-dispatch-final.log` |
| First Clippy | Exit 101; three test-only loop lints | `.local/open-bitcoin-dev/phase166/plan16/clippy-final.log` |
| Final RPC Clippy, all targets/features, warnings denied | Exit 0; 3.95s | `.local/open-bitcoin-dev/phase166/plan16/clippy-pass.log` |
| First workspace formatting | Exit 1; seven paused 09 paths only | `.local/open-bitcoin-dev/phase166/plan16/workspace-format-first.log` |
| Final workspace formatting | Exit 0 | `.local/open-bitcoin-dev/phase166/plan16/workspace-format-complete.log` |
| Final breadcrumb registration | Exit 0; 1241 Rust files | `.local/open-bitcoin-dev/phase166/plan16/breadcrumbs-complete.log` |
| Intermediate managed check after 09 formatting | Exit 1; 09 support file length only | `.local/open-bitcoin-dev/phase166/plan16/managed-final.log` |
| Final managed checks | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan16/managed-complete.log` |
| Final whitespace review | Exit 0 | `.local/open-bitcoin-dev/phase166/plan16/diff-complete.log` |

The initial wallet/factory REDs are intentionally staged unit/factory assertions, not observed daemon failures. Original failed 09 receipts are untouched. Supporting liveness receipts are `.local/open-bitcoin-dev/phase166/plan16/focused-liveness.log` and `.local/open-bitcoin-dev/phase166/plan16/serialized-liveness.log`; the latter records an actual exact-route child. Final library discovery totals 409, including both registered root tests. The 21-case filter excludes the reserved no-job helper. Overlapping affected reruns are not added as distinct coverage.

## Source Anchor Handoff and Task Commits

Root/11/12 must review the new consuming factory and its preflight/counter, the two new context test modules, WalletState::Unavailable, the Result wallet_info signature, fixed CanonicalContextError variants and RPC root worker registration before renewing assigned current guards. Existing ordinary constructor/assembler bodies remain preserved. Final eleven-path digests are retained at `.local/open-bitcoin-dev/phase166/plan16/anchors-final.txt`; no historical guard, receipt or registration contract was renewed here.

No commits: `commits: false` follows the strict wrapper. Shared lifecycle/state updates and saving remain root-owned.

## Self-Check: PASSED

All eleven implementation/evidence paths, the three new Rust files, this summary and every referenced receipt exist. The 21-case focused harness, three final-source affected reruns and 39 unchanged regressions pass. Final all-target/all-feature RPC Clippy, complete workspace formatting, 1241-file breadcrumbs, managed checks and whitespace review pass. Every new Rust path is registered, owned files remain below 629 lines, the ordinary constructor/assembler bodies remain preserved and no goal-blocking stub remains.
