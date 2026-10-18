---
phase: 165-shared-exact-work-authority
plan: "17"
subsystem: fixture-authority-integration
tags: [earned-work, basic, readiness, storage-fencing, source-guards]
requires:
  - phase: 165-16
    provides: Verified fixture integration and bounded current guard layout
provides:
  - Genuine managed branch and four-block transport fixtures
  - Same-core two-block accepted prefix with explicit maturity-one parameters
  - Actual earned readiness and preserved raw legacy request provenance
  - Strict undo identity before explicit live fixture reopening
  - Nine reviewed current source owners and eleven loaded-source controls
affects: [165-18, 165-20, 165-14]
tech-stack:
  added: []
  patterns: [Actual prefix before faults, Same owned Fjall core, Explicit legacy degradation]
key-files:
  created: []
  modified:
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/branch/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/transport/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/query/tests.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/accepted.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/filter_index/readiness/tests/interleavings.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store/tests/fixture.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/reorg.rs
    - scripts/check-phase165-shared-exact-work.ts
    - scripts/check-phase165-shared-exact-work/contracts.ts
    - scripts/check-phase165-shared-exact-work.test.ts
key-decisions:
  - Accept and flush only the actual old prefix before BASIC assembly or fault injection.
  - Preserve raw legacy classification through an explicit one-case coverage downgrade after genuine setup.
  - Compare durable undo before the existing fixture-only count preparation.
  - Keep current Phase 164 contracts unchanged because none of these nine owners was previously inventoried there.
requirements-completed: []
requirements-contributed: [CWOR-02, CWOR-03, CWOR-05]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 165-2026-10-14T16-09-59Z
generated_at: "2026-10-17T05:03:42Z"
client_date: "2026-10-17"
duration: approximately 28min
completed: 2026-10-17
tasks_completed: 3
files_modified: 12
---

# Phase 165 Plan 17: Genuine BASIC Fixture Prerequisites Summary

**Actual accepted prefixes now reach branch serving, transport extension, script-fact faults, readiness interleavings, bounded append release and the explicit storage mutation after reopening.**

## Delivered Scope

All three tasks are complete within the fifteen declared implementation owners. Twelve files changed in this plan; the three declared current Phase 164 files remain byte-identical to the completed Plan 16 summary. No new Rust path or breadcrumb registration was needed. The approved plan identity is 7734e64842e7f5b02924f260c64af73a17703b91dbead52e65c75b7f31fbcdbf.

Branch persistence consumes genuinely empty ownership before actual managed acceptance. The transport adapter builds four real mined/validated blocks, retains their bodies, flushes the same Fjall core and performs normal BASIC assembly and indexing. The codec-only shared historical fixture remains unchanged.

The local ValidatedHistory adapter accepts only its actual old two-block prefix with P2SH and explicit maturity 1 parameters. It saves only those bodies and the genuine prefix undo through a real same-core flush, then uses Plan 15's public constructor and the ordinary index turn. Memory direct/staged cases independently validate the same prefix. Query/readiness callers share that local adapter; no future undo is copied into source state. Requested-body cases read the constructor's already-bound two-header graph and preserve real handshake, successor Headers, GetData, body intake and original BODY/UNDO/COINS/META fault assertions. Future requested body data remains absent before intake.

Readiness acceptance retains actual accepted body bytes for subsequent scheduled indexing. The dedicated NeverConnected case retains the genuine manager across genesis flush and BASIC startup, with the same full parameters for later ingress and acceptance. Existing waker, cancellation, target, lock-release and final-read assertions remain intact.

The raw-legacy request case explicitly downgrades Complete coverage after genuine construction and normal descendant header ingress. Its request-time storage observation must be UnknownLegacy before capturing the barrier; later actual acceptance and the original UnknownLegacy/initially-synchronized final-read assertion remain. Positive ScriptsValid rows are preserved. This one-case adjustment was explicitly approved by the parent; it creates no raw header save, source ownership change or production API.

The count 17/count 129 append test now accepts/flushes genuine genesis on the same owned Fjall source before its original suffix and bounded zero-record release. It retains the exact independent genesis snapshot assertion and actual all-handle reopen. The lock-preservation mutation caller alone selects the existing reopen_live_fixture. That helper compares complete durable undo against genuine source undo before its existing count preparation; ordinary reopen and strict support_runtime identity remain unchanged. No tie preference is reconstructed.

## Executed Evidence

Every Bun command used the command-local pinned Bun 1.3.9 PATH; Cargo used Rust 1.94.1 and serial command-timings wrappers. Heavy Bun matrices started only after the Rust sequence ended. Sessions were polled within 60s; no process was suspended or terminated. Logs use the ignored prefix .local/open-bitcoin-dev/phase165-plan17-.

| Selected Rust filter               | Final actual result            | Execution time |
| ---------------------------------- | ------------------------------ | -------------- |
| phase160_branch                    | 4 passed / 0 fail / 0 ignored  | 1.59s          |
| phase160_filter_receipts           | 16 passed / 0 fail / 0 ignored | 4.28s          |
| phase159_basic_query               | 10 passed / 0 fail / 0 ignored | 31.24s         |
| phase157_accepted                  | 14 passed / 0 fail / 0 ignored | 6.47s          |
| Owned top-level phase159 readiness | 7 passed / 0 fail / 0 ignored  | 3.11s          |
| Owned readiness interleavings      | 5 passed / 0 fail / 0 ignored  | 5.43s          |
| phase157_append_actual_runtime     | 1 passed / 0 fail / 0 ignored  | 4.29s          |
| phase158_storage_reorg             | 35 passed / 0 fail / 0 ignored | 16.64s         |

The final Rust session 94926 recovered exit 0:92 selected cases, workspace format check, strict Node all-target/all-feature clippy and all-target/all-feature build. Clippy took 6.43s and cached build 0.08s. The initial eight baseline children all exited101 with 63 pass/29 fail; those actual receipts remain separately preserved. A compiler-only E0599 for Fjall snapshot was corrected to overlay_snapshot and is not called behavioral RED.

| Source/policy gate                 | Actual result                                                                                |
| ---------------------------------- | -------------------------------------------------------------------------------------------- |
| Stale inventory pin RED            | Exit1 before any source read after adding the nine reviewed owners                           |
| Focused current Phase 165 controls | 17 passed / 0 fail;111 expectations;6.29s                                                    |
| Complete current Phase 164 matrix  | 841 passed / 0 fail;1,399 expectations;341.01s                                               |
| Complete current Phase 165 matrix  | 1,230 passed / 0 fail;4,579 expectations;400.52s                                             |
| Exact current Phase 164 checker    | 135 fixed Rust inputs / 477 reviewed edges; exit 0                                           |
| Exact current Phase 165 checker    | 98 reviewed owners / 39 executable test-owner files; exit 0                                  |
| Breadcrumbs                        | 1,201 Rust files; exit 0                                                                     |
| Managed checks                     | 1,521 scanned inputs; zero findings/exemptions; lesson check passed                          |
| Stable closure                     | All 243 input/checker/module hashes unchanged across both complete matrices and exact checks |
| Final aggregate session            | Session 39212 recovered exit 0; whitespace check passed                                      |

The nine newly registered source files contain83 statically recognized enabled test declarations. That source inventory is distinct from the92 selected Rust executions, whose filters also include existing tests in other owners and do not select every unchanged test in all nine files. No source mutation is relabeled executed Rust semantic mutation evidence.

The commands used the following contract; individual filters above ran serially under keys phase165-p17-final-0 through phase165-p17-final-7, with logs final-0.log through final-7.log.

```bash
export PATH='/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:'"$PATH"
bun run scripts/command-timings.ts run --key phase165-p17-final-0 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features --lib phase160_branch
bun run scripts/command-timings.ts run --key phase165-p17-fmt -- cargo fmt --manifest-path packages/Cargo.toml --all -- --check
bun run scripts/command-timings.ts run --key phase165-p17-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings
bun run scripts/command-timings.ts run --key phase165-p17-build -- cargo build --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features
bun test ./scripts/check-phase164-canonical-identity.test.ts
bun test ./scripts/check-phase165-shared-exact-work.test.ts
bun run scripts/check-phase164-canonical-identity.ts
bun run scripts/check-phase165-shared-exact-work.ts
bun scripts/check-parity-breadcrumbs.ts --check
bun scripts/bright-builds-check.ts all
git diff --check
```

## Source Renewal and Preservation

Only nine new reviewed current Phase 165 owner identities and their real enabled-test counts were appended. Eleven actual loaded-function controls require a unique present anchor, unchanged passing baseline and an attributed work-owner finding. Their literal table lives in the already-owned contracts.ts; executable tests remain in the main test file. No helper path or cap exception was added.

The existing inventory derivation remains unchanged. Its old 219 inputs retain identical relative order; exactly the nine owned Rust paths enter before the existing corpus/document tail. The new 228-entry ordered pre-read pin is 97d735fbc6761933aeb9a50024dd5433da74afb3a6ca86649982ffb4eb0c8f64. Existing missing/extra/reordered/redirected controls pass. The owner-registration receipt records the exact old/new sequence and nine additions.

None of these nine source files was previously guarded by current Phase 164. Its source-files/contracts/test files therefore remain unchanged, including 135 sources/477 edges. Historical162/163 tools, corpus, recipes, manifest and receipts; preservation source; all six native registrations; fourteen-module isolated import controls; and the test-support helper remain unchanged. There was no blanket digest renewal, fallback read, exclusion, count normalization or raw authority promotion.

The stable map is .local/open-bitcoin-dev/phase165-plan17-stable-inputs.json, SHA-256 8ac80c312267a4093a235eae938d2d05dc2e7f39cd2c0db7518c3372640fe3c3. Its closing receipt at 2026-10-17T05:03:29Z records 243 checked paths and zero drift. The immutable support_runtime.rs SHA remains 5a25857c552bb7dcfdd040ad15e19c4616f52ef8b35df69b4f11a42dcd5b1f05.

## Diagnostics, Adjustments and Simplification

- The lower direct persistence case initially reached 13/14 because normal cache policy skipped its intended fault. Restoring its original zero-cache test lifecycle after genuine initialization retained the original physical fault and assertions; final 14/14 passes.
- Readiness initially reached 5/7 because a later accepted body was no longer supplied by the old broad raw seed, and the raw-legacy case lacked its causal coverage condition. Retaining actual accepted bytes fixed the first; explicit parent-approved legacy degradation fixed the second, producing 7/7. The separate NeverConnected corpus passes 5/5.
- Append reached the intended durable reopen but an extra setup store clone kept the database open. Consuming that store into the source preserves actual handle closure and the original reopen assertion; count 17/count 129 execution passes.
- The simplification pass kept local existing-file helpers and one shared actual old-prefix adapter. Short domain imports reduced accepted.rs and fencing.rs to 623 lines without removing assertions. All fifteen owned Rust/TS paths are at most 623 lines; no new file, dependency, capability or production behavior was added.

AGENTS.md, the Bright Builds sidecar, placeholder-only overrides and architecture/code-shape/verification/testing/Rust/TypeScript standards informed execution. Both active lesson files were fully loaded: 7,188 bytes / 2,397 conservative estimated tokens; no project skills or lesson audit trigger applied. No blocking stub or undeclared security-relevant endpoint/auth/schema/file-opening surface was found. No authentication gate occurred.

## Deferred Parent Gates and Commits

Historical group A's 37 diagnostic failures comprise the 29 directly selected failures plus eight inherited readiness/failures callers sharing these prerequisites. Those eight callers were not executed by the scoped filters here. The separate ninth readiness-failures reorg case is Plan 20's weaker-replacement premise. Full readiness remains explicitly gated by Plan 20; no current remaining workspace count is inferred.

The original Plan 14 diagnostic of 249 failures in 46 shared fixture roots remains historical evidence. This plan does not claim complete workspace/default native verification, coverage, Bazel, whole-phase source/security/formal review or pinned-Knots qualification. Plans 18–24 and final Plan 14 closure remain required. All four ordinary selectors still refuse; Phase 166 retains canonical bootstrap/exact all-handle reopen/genesis BASIC,167 contextual completeness and Phase 168 actual pinned-daemon qualification.

All task/RED/GREEN/metadata commits are deferred to the parent's normal-hook phase transaction after those gates. This executor staged, committed and pushed nothing, changed no shared state/tracker/configuration/hooks or other plans, and created no worktree. No task commit hashes are invented; HEAD remains d0104a997c0362bf65b81de1c8fec8ddc3012a1b.

## Self-Check: PASSED

All fifteen declared implementation paths and this summary exist. The twelve actual changed owners have completed scoped behavior/strict compilation and both complete guard matrices with 243 unchanged input/checker/module hashes. Caps, breadcrumbs, managed checks, whitespace and stub scan are verified. The strict support identity and historical boundaries remain intact. Shared phase acceptance and Git saving remain parent-owned; no executor commit is expected.
