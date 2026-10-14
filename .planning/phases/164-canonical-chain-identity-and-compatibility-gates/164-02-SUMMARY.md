---
phase: 164-canonical-chain-identity-and-compatibility-gates
plan: "02"
subsystem: storage
tags: [rust, fjall, chain-compatibility, nonmutating-refusal]
requires:
  - phase: 164-01
    provides: Canonical chain identity and immutable unsupported validation scope
provides:
  - Filesystem-only datadir classification before any backend access
  - Network-bound ordinary opener with exhaustive typed refusal
  - Closed-Fjall physical preservation and live backend-spy evidence
affects: [164-03, 164-04, 164-05, 164-06, 164-07, 164-08, 165, 166]
tech-stack:
  added: []
  patterns: [non-opening compatibility inspection, fixed bounded diagnostics, closed-store physical manifests]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/chain_compatibility.rs
    - packages/open-bitcoin-node/src/storage/chain_compatibility/tests.rs
    - packages/open-bitcoin-node/src/storage/chain_compatibility/tests/preservation.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Every current ordinary-open branch refuses before Fjall; neither missing nor empty paths authorize bootstrap.
  - Stored root/network and stored work remain separate unavailable observations rather than fabricated mismatches.
  - Keep the inherent ordinary opener with its compatibility policy to preserve the storage adapter file-length bound.
patterns-established:
  - Inspect borrowed ancestor prefixes root-first and only the first directory entry; never read source files or open a backend.
  - Physical test manifests compare names, types, lengths, bytes, permissions and mtime while excluding atime.
requirements-completed: [CBOT-04]
requirements-addressed: [CBOT-04, CVLD-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 164-2026-10-14T01-59-33Z
generated_at: "2026-10-14T03:28:24Z"
duration: 20min
completed: 2026-10-14
---

# Phase 164 Plan 02: Non-Opening Compatibility Gate Summary

**Ordinary datadir attempts now refuse before Fjall recovery, with bounded truthful diagnostics and physical preservation evidence for closed durable stores.**

## Performance

- Tasks completed: 2/2.
- Implementation files: 6; this summary is the seventh artifact.
- Execution duration: approximately 20 minutes.
- No staging, commits, pushes, hook skips, worktrees or shared tracker changes were performed by this executor.
- Full phase verification, requirement completion and Git finalization remain parent-owned.

## Accomplishments

- Added `DatadirObservation`, `InspectionIoKind`, `ObservedChainIdentity`, `WorkProfileObservation`, `CompatibilityReason` and `CompatibilityRefusal` under `storage::chain_compatibility`.
- Added `FjallNodeStore::open_ordinary(path, chain)`, which always returns a classifier-derived refusal. It cannot reach `Database::builder`, namespace creation, schema/coins migration, validation-history recovery, initialization, BASIC publication or prune resume.
- Inspected ancestor components from root to leaf through `symlink_metadata`, rejecting symlink/file/special components before descending. Trailing separators/dots and relative paths cannot bypass symlink refusal. Missing ancestors remain absence; permission and other I/O failures remain distinct typed observations.
- Classified directory shape with exactly one iterator advancement and constant auxiliary inspection space. A 2,048-entry physical directory and a counted million-entry iterator verify the first-entry limit. Inspection does not recurse, read source-file contents or create anything.
- Kept diagnostic text and JSON serialization below 1,024 UTF-8 bytes for every selector/observation combination. Messages include selected canonical network/hash, unavailable observed network/root/work, identity-only scope, ordinary-history refusal, typed missing prerequisites and fixed review/separate-datadir guidance. Hostile paths and injected error text are excluded.
- Preserved actual closed Fjall schema 1/schema 2, four canonical-looking roots and synthetic variants, malformed backend bytes, interrupted two-head coins/recovery records, valid/malformed/missing-coverage history, pending prune intent and valid/malformed BASIC artifacts. Wallet, metrics, runtime and unrelated file/directory/link sentinels survive.
- Compared complete physical manifests before and after refusal for all four selectors: relative names, types, lengths, exact file bytes, link targets, permissions/modes and modification times. Access times are deliberately excluded. Missing/empty paths, unsafe paths targeting a closed store and injected permission/other I/O failures have their own physical equality evidence.
- Proved the panic-before-backend spy is live using an owned raw-open control. Independent schema 1 and prune controls prove that raw recovery paths really migrate coins and delete both block/undo mates; these controls never run against the preserved sources.
- Registered three new Rust paths as explicit Open Bitcoin-only infrastructure breadcrumbs. The raw `open` API remains available and now documents its recovery/migration effects and lack of canonical admission guarantees.

## Task Commits

1. Task 1, bounded classification and ordinary refusal: deferred strict phase finalization.
2. Task 2, physical preservation and control fixtures: deferred strict phase finalization.

No task commit hashes exist. The originating plan and parent reserve all Git operations for finalization after clean complete phase verification.

## Verification and TDD Evidence

All Cargo commands ran serially through the checkout's cooperative timing wrapper.

| Stage | Command | Actual result |
| --- | --- | --- |
| Task 1 RED | `bun run scripts/command-timings.ts run --key phase164-compatibility-core -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib chain_compatibility` | Exit 101: missing ordinary opener and compatibility types; session 66224. |
| Task 1 GREEN | Same command | Exit 0: 11 passed, 1,699 filtered; session 96684. |
| Task 2 RED | `bun run scripts/command-timings.ts run --key phase164-compatibility-preservation -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib chain_compatibility_preserves` | Exit 101: missing physical-manifest assertion; session 73005. |
| Task 2 final GREEN | Same command | Exit 0: 13 passed, 1,710 filtered; session 95939. |
| Final combined regression | Task 1 command | Exit 0: 24 passed, 1,699 filtered; session 57585. |
| Final node lint | `bun run scripts/command-timings.ts run --key phase164-compatibility-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings` | Exit 0; session 62420. |
| Breadcrumbs | `bun scripts/check-parity-breadcrumbs.ts --check` | Exit 0: 1,147 Rust paths verified. |
| Formatting | Scoped `rustfmt --check --edition 2024 --config skip_children=true` on all five touched Rust files | Exit 0. |
| Whitespace | `git diff --check` | Exit 0. |

The default native full verifier and independent phase reviews are deliberately deferred to the parent, as required by this plan. These focused tests do not claim canonical bootstrap, positive reopen compatibility, full consensus acceptance or production readiness.

## API Notes for Downstream Executors

- Call `FjallNodeStore::open_ordinary(path: impl AsRef<Path>, chain: CanonicalChain) -> Result<Self, CompatibilityRefusal>`. The inherent implementation lives in `storage/chain_compatibility.rs`; normal method call syntax is unchanged.
- Use `inspect_datadir_without_backend(&Path) -> DatadirObservation` for non-opening shape observation. Results are `Missing`, `EmptyDirectory`, `ExistingUnproved`, `UnsafePath` or `InspectionIoFailure(InspectionIoKind::{PermissionDenied, Other})`.
- `CompatibilityRefusal::from_observation(chain, observation)` exposes `selected_chain()`, `selected_scope()`, `selected_genesis_hash()`, `observation()`, `observed_identity()`, `work_profile()`, `missing_prerequisites()`, `reason()` and `guidance()`.
- `CompatibilityReason::code()` and `DatadirObservation::code()` return fixed bounded strings. `Display` is a complete safe diagnostic; serde serializes that diagnostic as a JSON string.
- Observed identity and work each expose only `Unavailable`; neither a canonical-looking file nor a marker can create an accepted capability.
- Generic raw `FjallNodeStore::open` remains lower-level recovery/fixture infrastructure. Later adapter plans must wire ordinary paths to `open_ordinary`; this plan does not claim those adapter paths are already converted.

## Decisions Made

Followed D-03 through D-08 without new dependencies, production database decoders, identity markers, repair/reset paths or fallback overrides. The local AGENTS instructions, Bright Builds sidecar, overrides, architecture/code-shape/testing/verification/Rust standards and both active lesson inputs informed execution.

The classifier revisits borrowed ancestor prefixes rather than allocating a depth-sized stack. Filesystem metadata calls grow only with the path depth; directory-content work stops at one entry. Concurrent filesystem changes cannot grant authority because every possible observation still refuses.

## Deviations from Plan

**[Rule 3 - Blocking] Colocated the ordinary opener implementation with compatibility policy.** Adding the opener and test hook directly to `fjall_store.rs` grew it to 643 lines, exceeding the repository guard. Its unchanged inherent method now lives in `chain_compatibility.rs`, alongside the test-spy state; `fjall_store.rs` retains raw-opener documentation and the hook before backend creation. The adapter is 625 lines. No new module, public API change or guard exception was introduced. Final compatibility tests and Clippy pass.

## Issues Encountered

- Replaced an initial unavailable `tempfile` test helper with owned standard-library temporary fixtures, preserving the no-new-dependency policy.
- Explicit closures resolved the generic `symlink_metadata` function item's lifetime mismatch at the injected inspection seam.
- Shortened owned fixture names after macOS rejected a Unix socket path exceeding its fixed path bound; the first behavioral run had 10 passing checks and that fixture-only failure.
- Cloned the borrowed fixture header when constructing a `ChainPosition`.
- Added the real active-chain body to the schema 1 fixture after its independent control exposed the legacy confirmation migration prerequisite; the original physical preservation checks already passed.
- Removed the production ancestor iterator `expect()` after the crate's strict Clippy policy rejected it. Borrowed-prefix iteration now uses `filter_map` without a panic branch.

All issues are resolved. No deferred implementation issues, authentication gates or goal-blocking stubs remain.

## Simplification Review

Kept one filesystem classifier, one fixed refusal mapping and one unconditional ordinary refusal. No accepted token, unsupported success branch, pluggable production backend factory, recursive production scanner, sidecar trust, fallback raw open or repair abstraction was added. Recursive byte collection and mutation controls exist only in owned tests. The implementation preserves the raw fixture API and keeps the compatibility boundary in a small module.

## Next Phase Readiness

Adapter plans can use the ordinary opener and bounded diagnostic without touching Fjall first. Phase 165 can extend the separate unavailable work observation; positive canonical bootstrap/reopen remains Phase 166-owned. Requirements remain pending until lifecycle-valid whole-phase verification and parent finalization.

## Self-Check: PASSED

All seven plan-owned implementation/summary artifacts exist. Final focused tests, Clippy, scoped Rust formatting, breadcrumb registration and whitespace checks passed. No commits were expected or claimed under the strict parent-owned finalization contract; no Git/shared-state mutations were performed.

## Parent Accepted Requirement Closure | 2026-10-14

CBOT-04 has unique completion ownership in this summary after full default native session41473 exited zero (1h0m21.973s), clean 146-file review, 30 closed security controls and lifecycle-valid 27/27 phase verification. The original execution date and earlier evidence remain preserved. All four ordinary histories remain refused pending Phases165–167; no current ordinary qualification is asserted. Wrapper-owned normal-hook commit and push remain the final transaction.

The originally created `chain_compatibility/preservation.rs` helper was later moved byte-identically to `chain_compatibility/tests/preservation.rs` during the recorded panic-policy repair. The current key-file pointer above names its final compiler-valid test path; original helper SHA3d004f91bd72a0f3902588a36fca0438b04b5ac034bc9e227893664496835630 and all test behavior are unchanged.
