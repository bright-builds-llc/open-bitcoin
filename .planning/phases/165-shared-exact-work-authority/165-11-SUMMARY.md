---
phase: 165-shared-exact-work-authority
plan: "11"
subsystem: daemon-operator-work-refusal
tags: [proof-work, compatibility, daemon, operator, preservation, closed-manifests]
requires:
  - phase: 165-10
    provides: Typed work compatibility policy and pre-backend refusal
provides:
  - Real non-test daemon startup refusal with closed filesystem preservation
  - Parsed operator sync, wallet backup and soak-start work-refusal evidence
  - Exact adapter module and executable test ownership controls
affects: [165-12, 165-14, 166]
tech-stack:
  added: []
  patterns:
    - Separate actual normal-daemon subprocess evidence from configured factory evidence
    - Hash reviewed source regions and independently guard executable test attributes
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/phase165_work.rs
    - packages/open-bitcoin-rpc/tests/phase165_work_startup.rs
    - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests/phase165_work.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/canonical_chain.rs
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs
    - packages/open-bitcoin-cli/src/operator/runtime/chain_compatibility/tests.rs
    - packages/open-bitcoin-cli/src/operator/wallet/tests.rs
    - packages/open-bitcoin-cli/src/operator/soak/tests/recovery_resume_bounds.rs
    - scripts/check-phase164-canonical-identity.ts
    - scripts/check-phase164-canonical-identity/contracts.ts
    - scripts/check-phase164-canonical-identity.test.ts
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - "Existing adapter propagation is verification-only: initial GREEN is honest; no artificial product RED was manufactured."
  - "Actual main startup is tested with Cargo's normal daemon binary; factory calls remain separately attributable."
  - "Exact executable test and file attributes supplement body digests, preventing disabled evidence from retaining an accepted function body."
requirements-completed: []
requirements-contributed: [CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: 2026-10-17T01:52:34Z
duration: "approximately 25min implementation/verification; delayed evidence closure"
completed: 2026-10-16
tasks_completed: 2
files_modified: 13
---

# Phase 165 Plan 11: Daemon and Operator Work Refusal Summary

**The normal daemon binary and parsed operator routes expose the shared explicit work refusal while closed source, wallet, configuration, link and destination manifests remain unchanged.**

## Delivered Behavior and API Handoff

No production admission API or fallback was added. Existing adapters consume Plan 10's `storage::chain_compatibility::WorkCompatibilityDecision` through `CompatibilityRefusal::work_decision()`. `WorkProfileObservation::Unavailable` remains unchanged. Missing/empty sources report `bootstrap_unavailable`, existing unproved sources report `existing_unproved`, and unsafe sources report `unsafe_path`; inspection-I/O policy remains covered by the predecessor's shared gate tests. Exact-looking numbers, malformed records and forged work-profile markers never become authority.

The registered daemon factory module exercises valid selected-network configuration loading, the real `open_runtime_store` factory and actual metrics fallback calls. These three cases are factory evidence. They do not independently claim to execute all of `main` or prove listener ordering merely by reserving a port.

The separate integration test uses `env!("CARGO_BIN_EXE_open-bitcoind")`, so Cargo supplies the normal non-test daemon binary. One discovered case executes **32 owned daemon children**: missing, empty, closed forged and unsafe sources, each across mainnet/testnet3/signet/regtest and inbound metrics off/on. Options are accepted by the actual loader; the test asserts the shared work diagnostic, unavailable work and identity-only unsupported scope rather than accepting an earlier configuration failure. An occupied loopback RPC port ensures an attempted bind cannot masquerade as the expected refusal. Missing-source startup omits `-datadir`, supplies a valid explicit config and uses an isolated HOME; explicit BASIC-index disabling provides a durable trigger. Existing-source variants supply the owned datadir. All child networking is off or explicitly loopback-only. Each child has a fixed 15-second refusal deadline and only that owned child is killed/reaped if its actual deadline expires; no child reached that deadline.

Recursive manifests include names, entry types, file bytes and lengths, modification times, readonly state, Unix modes and link targets. The normal-process matrix preserves the entire owned root, including a private-mode config and link, closed Fjall bytes, wallet and prune/BASIC sentinels. Factory cases additionally cover synthetic exact-looking sidecar JSON and malformed recovery material. These are untrusted fixture bytes, not successful snapshot-decode or canonical qualification evidence. No source is repaired, reencoded or reopened by a raw production fallback.

Operator coverage includes typed offline status/pause/resume across all seven existing source shapes and four selectors; selected genesis and unsupported scope are asserted. Actual `OperatorCli::try_parse_from` plus `execute_operator_cli_with_default_data_dir` reaches the real human/JSON sync and wallet-backup routes with selected networks resolved from valid JSONC. Another parsed bounded soak-start case reaches unavailable work before any soak ledger is created. The existing wallet lookup/backup and soak-enrichment cases now assert work refusal; backup covers absent, existing and absent-parent destinations, and enrichment preserves existing sync/peer fields. Closed manifests retain config/wallet bytes and private config modes/links. Diagnostics exclude source-root paths, test secrets and backend payload sentinels, with JSON envelopes checked as JSON. Inherited live authentication/operation failures still refuse without downgrading to offline lookup.

Test-only reuse is available through `operator::runtime::chain_compatibility::tests::phase165_work::{forged_source, assert_work_message}`. The daemon's existing `NETWORKS`, `owned_path`, `PhysicalEntry`, `manifest` and `config` have only `pub(super)` visibility expansion; their behavior/assertions are retained. No production CLI helper change was necessary.

## Executed Verification

Every Bun/Cargo invocation used the command-local pinned Bun 1.3.9 PATH. Every Cargo invocation used `bun run scripts/command-timings.ts run --key <key> -- ...`. Cargo jobs and the complete native/Bun suites were serialized; live commands were polled without timeout relaxation or suspension.

| Command or check                                   | Actual result                                                                                                                                                                                                         |
| -------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| First declared daemon command                      | Compilation stopped at the pre-existing private `staged.undo` access in the daemon BASIC fixture. This is a compile diagnostic, not behavioral RED.                                                                   |
| Initial daemon factory GREEN                       | 3/3 passed.                                                                                                                                                                                                           |
| Initial actual daemon-main integration             | 1/1 passed, comprising 32 real owned children.                                                                                                                                                                        |
| Initial declared operator `phase165` command       | 6/6 passed.                                                                                                                                                                                                           |
| Pre-attribute-hardening full Bun guard             | 711/711 passed, zero failed, 939 expectations; 265.55s. Distinct earlier-source receipt retained.                                                                                                                     |
| Later daemon rerun diagnostic                      | 2 passed / 1 failed: concurrent new fixtures collided on a timestamp-only label. Fixed with an atomic suffix in the new module.                                                                                       |
| Final declared `phase165-daemon-work-refusal`      | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --bin open-bitcoind tests::phase165_work`: 3/3 passed; 1.12s test execution.                                                       |
| Final declared `phase165-daemon-main-work-refusal` | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --all-features --test phase165_work_startup`: 1/1 passed, 32 children; 1.26s test execution.                                                      |
| Final declared `phase165-operator-work-refusal`    | `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-cli --all-features phase165`: 6/6 passed; 3.05s test execution. Other filtered harnesses discovered zero matching tests, not additional passed cases. |
| Inherited daemon compatibility                     | `tests::canonical_chain`: 8/8 passed; 8.85s.                                                                                                                                                                          |
| Inherited CLI compatibility                        | `chain_compatibility`: 11/11 passed; 5.11s, including live failure/no-offline-fallback controls.                                                                                                                      |
| Final strict lint                                  | Both affected crates, all targets/all features, `cargo clippy ... -- -D warnings`: passed; 3.95s.                                                                                                                     |
| Final build                                        | Both affected crates, all targets/all features: passed; 2.48s.                                                                                                                                                        |
| Final workspace formatting                         | Timed `cargo fmt --manifest-path packages/Cargo.toml --all --check`: passed.                                                                                                                                          |
| Final complete Bun guard                           | `scripts/check-phase164-canonical-identity.test.ts` (Bun test): **751/751 passed**, zero failed, 1,039 expectations; **308.33s**. Actual session `57509` exit **0** recovered at evidence closure.                    |
| Final current-source CLI                           | `scripts/check-phase164-canonical-identity.ts` (Bun run): passed, **131 fixed Rust inputs / 452 reviewed edges**. Rechecked at closure.                                                                               |
| Stable-source check                                | All **162** saved loaded-input SHA-256s still match at closure.                                                                                                                                                       |
| Managed checks                                     | 1,521 scanned inputs, zero exemptions/findings.                                                                                                                                                                       |
| Breadcrumbs                                        | Passed for **1,198 Rust files**; rechecked at closure.                                                                                                                                                                |
| File cap, diff and index                           | All 12 owned Rust/TypeScript paths at most 628 lines; `git diff --check` passed and index empty.                                                                                                                      |

One actual final stable-source full guard result serves both task gates. No second identical final guard run is claimed. The earlier 711-case run predates executable-attribute hardening and the fixture-race repair. Ignored evidence is preserved separately in `.local/open-bitcoin-dev/phase165-plan11-pre-attribute-hardening-guard.log`, `phase165-plan11-final-guard.log` and `phase165-plan11-stable-input-hashes.json`.

The final guard log last write is **2026-10-15T07:32:33.775261Z**. Model-capacity interruptions delayed summary closure until October 17 UTC. The existing guard process was preserved, its exit was recovered, and no completed Cargo/Bun suite was restarted during closure. Approximately 25 minutes describes the original implementation/verification interval; it excludes the long interruption and is not runtime-performance evidence.

## Reviewed Guard Ownership

Only current reviewed owners were added/renewed. New module paths are enforced as executable registrations. Exact `#[test]` ownership rejects attribute removal, `#[ignore]` and `#[cfg(any())]`; lexical inner-attribute checks reject file-level disabling while masked comment examples remain accepted. These add 40 attribute controls, alongside 12 route/registration mutation controls and missing-edge controls for 29 added reviewed regions. Mutation anchors are explicitly asserted against loaded current sources. Actual route calls and manifest/work assertions cannot be replaced with successful stubs while retaining an accepted digest.

Historical manifests, tooling, corpus receipts and source attribution are retained. The protected Node preservation source remains **20,688 bytes**, SHA-256 **`e6cb5740cb5c83655aed14c43b1206847d264431d944741a365771c3e8d76f5b`**, matching the predecessor. Its original registration and assertions were not edited. No direct per-child syscall/thread counter is claimed: adapter evidence is actual gate/error ordering plus closed manifests, composed with Plan 10's independently effective actual backend-builder/migration/prune controls and the retained reviewed product boundaries.

## Deviations and Scope

The parent approved and formalized four narrow additions: five daemon helper/type visibility changes in `canonical_chain.rs`; exact current registration enforcement in the checker; the one-line `staged.undo().transactions` consuming-fixture repair; and the actual normal-daemon integration test. The accessor repair was required to compile the existing consumer after its predecessor API transition. The atomic test-source suffix fixes the observed concurrent fixture collision without changing the shared original helper.

The parent changed the adapter tasks to verification-only because Plan 10 already provided working diagnostic propagation. Initial GREEN and real negative controls are reported honestly; no artificial product RED or temporary production bypass was introduced. The simplification pass retained existing product routes and manifest helpers instead of adding a startup abstraction, public spy API or new production authority surface.

AGENTS.md, its Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust/TypeScript standards informed execution. Both active lesson files were loaded completely: 7,188 bytes and 2,397 conservative estimated tokens. No new production API, endpoint, auth path, schema or file-access surface was added, and no blocking stubs remain. Contributor-facing support claims do not change, so no README claim update was required.

CWOR-05 adapter evidence is ready for the parent; requirement activation and full phase acceptance remain parent-owned. Phase 166 retains canonical bootstrap, all-authority exact reopen and genesis BASIC; 167 retains contextual completeness; 168 retains actual pinned-Knots-daemon qualification. These tests execute Open Bitcoin's daemon and do not claim pinned-daemon differential qualification. Parent Plan 14 still owns full default native/workspace preflight, independent review/security/verifier and strict finalization.

## Task Commits

All task, RED/GREEN and metadata commits are explicitly deferred to parent strict finalization. This executor staged, committed and pushed nothing, changed no hooks, and did not edit shared STATE/ROADMAP/REQUIREMENTS/config/todo. No commit hash is invented.

## Self-Check: PASSED

All 13 owned implementation/test/checker/mapping paths exist, together with this summary. All three new Rust files have immediate parity breadcrumbs and recognized test paths. The largest owned code file is the contracts manifest at 592 lines; the largest owned Rust file is the soak test file at 540. Final declared affected commands, inherited compatibility, lint, build, formatting, 751-case guard, boundary CLI, breadcrumbs, caps, whitespace and source-hash checks passed. The actual final guard exit was recovered as zero. Protected preservation bytes remain unchanged, the Git index is empty, and shared state/Git completion remains assigned to the parent.
