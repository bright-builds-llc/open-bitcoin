---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "10"
subsystem: storage
tags: [durable-qualification, exact-work, checkpoint, physical-preservation, confirmed-counts]
requires:
  - phase: 166-09
    provides: Accepted configured Cargo/Bazel operator and daemon genesis routes
  - phase: 166-08
    provides: Canonical BASIC owner, saved-mode recovery and publication-fault evidence
  - phase: 166-06
    provides: Atomic genesis and proved original-store reopen
  - phase: 166-01
    provides: Frozen independent source-derived genesis/BASIC and numerical vectors
provides:
  - Finite actual-byte refusal matrix with physical preservation and fixed redacted categories
  - All 32 coordinated work-byte mutations and forged wide-number authority refusal
  - Four-selector actual managed checkpoints with BASIC enabled and disabled
  - Complete parent-handle release and marker-proved independent child reopen
  - Existing confirmed-transaction counts retained only for identical complete stored ancestry
affects: [166-11, 166-12, 166-13, 166-14]
tech-stack:
  added: []
  patterns: [actual-byte product qualification, same-ancestry count preservation, separate arithmetic evidence]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/qualification_tests.rs
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/test_support.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/checkpoint_metadata.rs
  modified:
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/tests/coins_migration.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Retain existing failure controls and receipts instead of introducing duplicate fault engines or artificial RED tests.
  - Read and decode actual saved metadata under the publication guard before metadata-side invalidation; preserve counts only when every stored ancestry position equals the proposed ancestry.
  - Keep missing/legacy counts absent, discard counts for changed ancestry and propagate malformed metadata without replacement.
  - Spawn qualification children only after parent owners close and require a bounded completion marker after actual assertions and teardown.
requirements-completed: [CBOT-02, CBOT-03, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T19:22:51Z"
commits: false
duration: 21m 33s
completed: 2026-10-20
---

# Phase 166 Plan 10: Durable Bootstrap Qualification Summary

**Real managed checkpoints now preserve required explicit counts for unchanged ancestry, and all four genesis selectors reopen with exact work and optional BASIC through independent durable proof.**

## Completion Boundary

Tasks: **2/2 complete**. Recorded implementation/check window: 2026-10-20T19:01:18Z–19:22:51Z, excluding earlier read-only preparation. Seven implementation/evidence paths changed, plus this summary. The two existing BASIC/atomic test roots declared in the plan remain unchanged; their accepted fault controls are reused. New Rust files have 529, 608 and 248 lines; existing touched Rust files remain at or below 616 lines.

Requirement frontmatter records this plan's contribution, not whole-phase closure. No staging, commit, push, hook bypass, shared STATE/ROADMAP/REQUIREMENTS/task edit, LOC refresh or historical receipt/source-pin renewal occurred. Root owns independent acceptance and final normal-hook saving; full default native verification remains 166-14.

## Actual Durable Qualification

Accepted setup uses only the canonical product initializer/runtime. Explicit test-only raw writers run after all product owners drop and introduce corruption only; they never supply accepted authority. Thirty-three fixed named cases mutate stored selector/root, complete body, header/hash/parent/height/nonce, independently stored header/index/active work and tips, median time, coverage/status, required rows, B/H/C, legacy work, an oversized authority value and correlated numerical descendant ancestry.

Another 32 cases flip one independently expected work byte at a time while making all three stored numerical endpoints agree. All refuse after recomputing authority from the actual canonical body/header. Four independently frozen above-u128 numbers also fail when placed consistently in all three endpoints. A wrong selected-network case brings the named closed-source preservation scenarios to **70**. Each checks its exact refusal category, unchanged original backend-builder count and physical names/types/lengths/bytes/modes/mtime, excluding atime. Actual error strings are bounded and contain neither the source path nor an unrelated private wallet sentinel. Physical assertions deliberately avoid dumping source bytes on failure.

A positive control proves the allowed unrelated wallet sentinel is not itself the cause of refusal or a source of authority. An additional unsafe-alias case preserves both source and symlink. Existing source-change, copy bounds, engine/application contention and cleanup gates remain covered by accepted 166-03/04/15 evidence; saved canonical BASIC cleanup refusal is covered by 166-08. No new inspection engine or broader hostile-writer guarantee is introduced.

Eight checkpoint scenarios cover **four selectors × BASIC Enabled/Disabled**. Each compares fresh and reopened live header/block/active work with all 32 independently expected bytes, coins B, the complete serialized genesis body and absence of the fixed genesis outpoint/C/H. Enabled cases compare exact BASIC bytes/hash/header/zero predecessor; Disabled cases retain absent index progress/checkpoints/artifacts. Every Always flush asserts that it actually wrote coins and retained explicit empty current counts.

Runtime, network handle, store clone and extracted coins view are explicitly retained/released. Sixteen negative probes demonstrate that a live handle or final coins view prevents false reopening. After every owner closes, the same path reopens through product inspection. Each scenario additionally starts the current test executable after parent teardown; that child independently earns and checks the same canonical state, then drops its runtime. A 4 KiB-capped proof marker after those assertions/teardown is required alongside successful exit, so a zero-match child cannot count as evidence. All eight child scenarios passed. A scenario mutex and serial test execution follow the accepted test-only fork/exec pattern without production lock changes, retries or relaxed deadlines.

Four frozen wide codec vectors and four carry/wrap vectors remain explicitly **arithmetic-only-no-chain-authority**. Genesis live work remains the genuine small root value. No artificial above-u128 chain, ordinary descendant acceptance, Signet challenge qualification or actual Knots-daemon differential is claimed.

## Checkpoint Defect and Approved Fix

The first actual managed checkpoint test passed its fresh checks, wrote coins, then failed canonical same-path admission with MissingRecord. `save_validated_chain_meta` encoded absent confirmed-transaction counts, losing the explicitly empty map required by the strict canonical decoder.

Root obtained independent seam/plan review and amended ownership for `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs`, its existing migration test root and the new focused test child. The production change reads actual chain_meta bytes and decodes them under the existing publication guard, before coverage/append invalidation. It preserves an existing decoded map only when the entire stored ancestry equals the proposed ancestry. Missing records and legacy absent counts remain None; changed ancestry—including a same-tip interior change—discards counts. Malformed/read errors propagate before metadata-side replacement/invalidation. The public raw writer, strict canonical decoder, revision/pending-completion protocol and coins-before-metadata failure semantics are unchanged. This is not whole-flush atomicity or inferred empty state.

Six focused controls prove explicit empty/nonempty retention, changed ancestry, same-tip interior change, missing/legacy absence, malformed-byte preservation with unchanged sequence/revision, and public raw-write invalidation of a genuine append proof. Their pre-fix run passed three and failed three; final six pass. All 36 existing phase157 proof controls and four phase159 coverage controls pass unchanged. Independent production source review reported PASS. The earlier WIP Enabled-only coverage note is resolved by the actual eight final mode/selector scenarios.

## Decisions, Corrections and Limits

**[Rule 1 - Bug] Preserve already-known counts across unchanged checkpoints.** The real checkpoint/reopen failure justified the checked ownership amendment and narrow writer fix above. No canonical flag, new counts constructor, inferred empty map, duplicate flush adapter, schema or dependency was added.

The initial matrix expected InconsistentGenesis for some header/hash inconsistencies. Existing DTO decoders correctly reject these earlier as MalformedRecord; source review corrected six fixed test expectations without changing or weakening production refusal. The first numerical test also incorrectly assumed ProofWork Display was decimal. That assertion was removed; fixed hex, all LE bytes and independent wrapping expectations remain. These are test-assumption failures, not production fixes or artificial RED evidence. Initial unused scaffold-helper warnings were eliminated as the full tests were wired.

The simplification pass separated the finite constant case table from test control flow and kept fixture, physical and bounded child helpers in the owned support module. After the qualification binary compiled, only equivalent case-table extraction, formatting and opaque physical-failure assertions changed; the current tree was subsequently recompiled by focused controls/regressions and Clippy. The already-passing qualification suite was not redundantly rerun. No goal-blocking stub or new unmodeled production endpoint/authentication/file-access surface was found. The existing storage trust boundary and checkpoint amendment cover the production metadata read.

Evidence is macOS/Unix, source-derived vectors, actual durable records and software controls. No hardware power-loss, parent/system-crash, OOM, arbitrary filesystem/direct-engine writer, production/funds or actual-daemon differential guarantee is added. Only known owned fixtures/aliases are removed; failed attempts are retained as failures, with no scan or cleanup of unrelated temporary roots. Ordinary history, public networking activation, import/repair and broader canonical pruning remain their assigned future work. No authentication gate or external setup was required.

Guidance applied: repo AGENTS native verifier/tool pins/parity/LOC contracts, Bright Builds sidecar, overrides (no concrete exception), architecture/code-shape/testing/verification/Rust standards and the execute-plan context/template. Both active lesson inputs were read completely in this execution context: 8,856 bytes / 2,953 estimated tokens. No archives/project skills or lesson files were changed.

## Task Commits

None; `commits: false` follows the strict wrapper. Commit-existence checks are inapplicable. Root retains current source-contract/registration reconciliation, lifecycle updates and normal-hook Git saving.

## Automated Evidence

Cargo ran serially through `scripts/command-timings.ts` with pinned Bun 1.3.9 PATH, the approved isolated target and canonical process-local TMPDIR. Tests used `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-features FILTER -- --test-threads=1`; matrix/qualification logs additionally used --nocapture for bounded case/progress markers. Zero-match integration harnesses are excluded from positive counts.

| Check | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| First managed checkpoint/numerical baseline | Exit 101; 0 passed / 2 failed: real MissingRecord after checkpoint plus Display-format assumption | `.local/open-bitcoin-dev/phase166/plan10/baseline.log` |
| Initial matrix | Exit 101; 0/1, wrong expected category for header-hash | `.local/open-bitcoin-dev/phase166/plan10/matrix-initial.log` |
| Corrected finite matrix | Exit 0; 1/1, 33 named cases, 25.91s | `.local/open-bitcoin-dev/phase166/plan10/matrix-expanded.log` |
| Focused checkpoint metadata RED | Exit 101; 3 passed / 3 failed, 16.34s | `.local/open-bitcoin-dev/phase166/plan10/checkpoint-metadata-red.log` |
| Initial checkpoint metadata GREEN | Exit 0; 6/6, 16.31s; staged unrelated unused import subsequently removed | `.local/open-bitcoin-dev/phase166/plan10/checkpoint-metadata-green.log` |
| Complete actual qualification | Exit 0; 8/8, 154.64s; 70 named preserved refusals and eight mode/selector checkpoint-child markers | `.local/open-bitcoin-dev/phase166/plan10/qualification-expanded.log` |
| Final checkpoint metadata | Exit 0; 6/6, 15.18s | `.local/open-bitcoin-dev/phase166/plan10/checkpoint-metadata-final.log` |
| Unchanged phase157_proof regressions | Exit 0; 36/36, 31.53s | `.local/open-bitcoin-dev/phase166/plan10/append-proof-regressions.log` |
| Unchanged phase159_validation_history_coverage | Exit 0; 4/4, 4.56s | `.local/open-bitcoin-dev/phase166/plan10/coverage-regressions.log` |
| Node all-target/all-feature Clippy, warnings denied | Exit 0; 7.56s | `.local/open-bitcoin-dev/phase166/plan10/clippy.log` |
| Scoped rustfmt check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan10/format.log` |
| Breadcrumbs | Exit 0; 1244 Rust files | `.local/open-bitcoin-dev/phase166/plan10/breadcrumbs.log` |
| Managed checks | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan10/managed.log` |
| Diff whitespace | Exit 0 | `.local/open-bitcoin-dev/phase166/plan10/diff-check.log` |

The eight qualification and six focused metadata functions are distinct. Internal vector/scenario loops and eight explicit child invocations are not relabeled as additional top-level Rust tests. The 36/4 regressions are existing controls, not new implementation coverage. No command was terminated for quiet output or an elapsed estimate; progress and live CPU evidence were observed while qualification ran.

Previously accepted unchanged software controls are reused, not rerun or renewed here: atomic BeforeCommit/AfterCommit uncertainty in `.local/open-bitcoin-dev/phase166/plan06/atomic-final.log`, and BASIC enable/append/SyncAll/cleanup failures plus saved-generation/all-handle controls in `.local/open-bitcoin-dev/phase166/plan08/canonical-basic-final.log`. The accepted 166-03, 166-04 and 166-15 summaries retain source-change/lease/copy/cleanup/child-lifecycle bounds and their original limitations. Current whole-repo native acceptance remains 166-14/root.

## Self-Check: PASSED

All seven changed implementation/evidence files, this summary and all 16 referenced receipts exist. Final results match the receipts: qualification 8/8, checkpoint metadata 6/6, existing append-proof controls 36/36 and coverage controls 4/4; the log contains 70 preservation cases and eight completed checkpoint/child scenarios. Clippy with denied warnings, formatting, 1244-file breadcrumbs, managed checks and whitespace review pass. The planned reopen key link is present (1/1), all owned Rust files remain below 629 lines, frontmatter delimiters are valid and no goal-blocking stub or unresolved plan-10 issue remains. Saving and full native/lifecycle acceptance remain root-owned; commit checks are intentionally inapplicable.
