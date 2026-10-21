---
phase: 166-canonical-durable-bootstrap-and-genesis-basic
plan: "05"
subsystem: storage
tags: [genesis, pure-proof, exact-work, strict-decoding, basic, coins]
requires:
  - phase: 166-01
    provides: Sealed complete canonical genesis facts and internally derived proof work
  - phase: 166-02
    provides: Canonical genesis coins and header owner contracts
  - phase: 166-15
    provides: Accepted isolated inspection with complete untrusted detached records
provides:
  - Strict current-format detached-record decoder with explicit missing-row refusals
  - Pure sealed proof of the complete canonical genesis authority tuple
  - Saved genesis BASIC content and prune-lock validation using existing codecs
  - Storage-private canonical identity envelope for the atomic writer
affects: [166-06, 166-07, 166-08, 166-10, 166-11, 166-12, 166-14]
tech-stack:
  added: []
  patterns: [strict current-shape decoding, complete tuple proof before live authority, regenerated BASIC commitments]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/records.rs
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/proof.rs
    - packages/open-bitcoin-node/src/storage/canonical_bootstrap/tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage.rs
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Reuse current codecs and keys through narrowly widened storage-only visibility rather than duplicate persistence formats.
  - Require explicit current empty confirmed-transaction counts; legacy absence cannot trigger authority-producing hydration.
  - Treat complete coherent empty BASIC prefixes separately from committed canonical genesis filters.
patterns-established:
  - A parsed numerical record cannot construct a live store or runtime capability.
  - Optional unrelated metadata uses explicit known-key classes rather than whole-namespace exemptions.
requirements-completed: [CBOT-02, CWOR-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-20T04:04:02Z"
commits: false
duration: 12 min recorded implementation and verification window; initial reading excluded
completed: 2026-10-20
---

# Phase 166 Plan 05: Complete Canonical Genesis Record Proof Summary

**Detached current storage records now earn a pure sealed genesis proof only when the selected complete body, both header projections, active position, every work byte, coins B, validation coverage and optional BASIC state agree.**

## Completion Boundary

Tasks: **2/2 complete**. The first retained RED receipt completed at 2026-10-20T03:51:23Z; implementation and verification ended at 2026-10-20T04:03:00Z. Initial instruction/source loading precedes that recorded window. Ten implementation/evidence paths are owned, plus this summary; four Rust files are new. All touched Rust files stay below 629 lines; the new proof, decoder and main test file have 183, 306 and 611 lines respectively.

This is complete data-derived proof, with **no live store, coins owner or runtime authority**. The same pure API is ready for both inspection and original-record revalidation in 166-06; that product opener and actual same-datadir canonical reopen are not implemented or claimed here. Frontmatter requirement IDs record contributions, not requirement/lifecycle closure. No staging, commit, push, branch change, hook bypass or shared STATE/ROADMAP/REQUIREMENTS/todo update occurred. Root owns those operations and final saving after whole-phase verification.

## API and Atomic Writer Handoff

`packages/open-bitcoin-node/src/storage/canonical_bootstrap.rs` registers the restricted pipeline and fixed payload-free `CanonicalBootstrapRefusal` categories: missing, malformed, unsupported record, inconsistent genesis, unsupported history and inconsistent index. Raw codec errors are mapped to these categories without source values, paths or nested parser diagnostics.

Storage-only APIs are `records::CanonicalGenesisRecords::decode(&InspectedRecords)`, `validate_genesis_records(selected: CanonicalChain, &CanonicalGenesisRecords) -> Result<ProvedGenesisRecords, CanonicalBootstrapRefusal>`, and the result's read-only `genesis()` / `position()` methods. The result has private fields and no public constructor, deserializer, default or live-store handle. `PartialEq` / `Eq` compare the validated canonical facts, position and validated saved BASIC/lock bytes, allowing 166-06 to compare its independently decoded inspection/original results. The DTO's fields are accessible only within the canonical bootstrap module; parsing grants no authority.

`packages/open-bitcoin-node/src/storage/canonical_bootstrap/records.rs` defines `IDENTITY_KEY = "canonical_chain:v1"` in the existing **schema** namespace and `encode_identity(CanonicalChain) -> Vec<u8>`. Its exact 34-byte envelope is version byte 1, selector byte (0 mainnet, 1 testnet3, 2 default Signet, 3 regtest), then the selected canonical raw 32-byte genesis hash. This new envelope is an identity fact, never sufficient proof. The atomic writer must reuse this helper and current schema bytes `2`.

The writer must publish the existing header/index `snapshot` encodings; one complete `block:` body under the existing `block_key`; current `chain_meta` with one genesis position and **Some(empty confirmed-tx occurrence counts)**; coins B equal to the raw genesis hash with no C or H rows; complete validation coverage and one matching ScriptsValid genesis identity. No undo or legacy chainstate snapshot is accepted. Existing header/index/meta codecs supply strict exact-number work decoding; the local boundary additionally compares decoded/re-encoded JSON shapes so schema 1, absent current fields and unknown fields cannot be silently normalized into authority. Each authority value is bounded to 64 KiB before parsing; exact-boundary/one-over tests pass. The inspection owner's existing aggregate/count bounds remain unchanged.

`packages/open-bitcoin-node/src/storage/canonical_bootstrap/proof.rs` proves the selected full block through `CanonicalGenesisProof`, recomputes root hash/work, uses existing modulo-256-bit `wrapping_add`, and compares the complete height-zero/null-parent root to headers, block index and active position, including time and median time. It binds B and the exact validation identity/status/coverage. Any extra ancestry explicitly returns UnsupportedHistory, even when its numerical parent/height/work relationships agree. Every byte of all three work endpoints must match; marker or numerical decoding cannot promote imported history.

## Explicit Namespace and Record Inventory

| Namespace | Supported current record classes |
| --- | --- |
| schema | Required schema_version and canonical_chain:v1 only |
| headers | Required snapshot only |
| chainstate | Required chain_meta only; no legacy snapshot or undo rows |
| coins | Required B only; every C/H/unknown row refuses, including empty H |
| block_index | Required snapshot, one block body, exact complete coverage and one genesis validation row; optional existing BASIC owner/state/record/active keys and prune_locks |
| wallet | Optional snapshot, wallet_registry, selected_wallet, nonempty wallet_snapshot: name and wallet_rescan_job: name classes |
| metrics | Optional snapshot only |
| mempool | Optional snapshot only |
| runtime | Optional snapshot only; recovery_marker refuses |

Unknown namespaces, including empty ones, and unknown keys in any supported namespace refuse. Known wallet/runtime/metrics/mempool values remain unrelated untrusted metadata and never enter the proof; an empty value in these optional classes is not manufactured missing authority. Missing required namespaces, actual empty namespaces, absent required rows and empty present authority values retain explicit refusal paths. Prune intent, have_pruned, prune summary, unsupported BASIC versions and unresolved partial index artifacts refuse.

Optional BASIC state must bind the canonical genesis fence and either a coherent empty prefix or a height-zero canonical endpoint. Existing state, owner, projection, record and prune-lock codecs are reused. Active ownership requires its correctly shaped reserved lock covering saved protection; disabled retained ownership may preserve or release that lock under existing lifecycle semantics. Committed records must exactly equal content regenerated from the complete canonical genesis body with zero predecessor, including its output script despite no spendable genesis coins. Wrong/missing locks, projection, owner, state, record or content refuse. A separate test constructs an internally valid foreign filter commitment under the canonical envelope; the content proof rejects it.

## Decisions, Deviations and Review

**[Rule 3 - Blocking] Expose existing codecs only to their storage sibling.** The coordinator explicitly authorized and amended plan ownership for `packages/open-bitcoin-node/src/storage/fjall_store.rs`, `packages/open-bitcoin-node/src/storage/fjall_store/prune.rs` and `packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs`. Only the existing block-key function, prune-lock key and prune-lock encode/decode visibility/re-exports widened to `pub(in crate::storage)`. No codec, key bytes, backend constructor, external API or storage behavior changed. This avoids duplicating formats and preserves legitimate enabled BASIC reopen. No commit was made.

The two consumed staging expectations were removed from `packages/open-bitcoin-node/src/storage/fjall_store/inspection/reader.rs`; method behavior and visibility remain intact. The already-staged inspection callback expectation remains with its 166-06 consumer. The new canonical bootstrap module has one temporary dead-code expectation while its product opener is staged; 166-06 must remove or narrow it as genuine production callers consume the pipeline. This is an unused integration seam, not a placeholder proof implementation.

The simplification pass kept one pure decoder/validator for both future read sites and reused the existing complete-genesis and BASIC generation paths. Production proof code performs no filesystem, process or network I/O and constructs no runtime capability. All new Rust paths are registered in `docs/parity/source-breadcrumbs.json`. No new dependency, crate, endpoint, authentication path or unmodeled security surface was found; planned T-166-05-01/02 cover the new data trust boundary and fixed diagnostics. No unfinished implementation stub remains.

Guidance applied: repository AGENTS native-verifier/tool-pin/parity rules, Bright Builds sidecar, standards overrides (no concrete exception), standards index and architecture/code-shape/testing/verification/Rust pages, formal execute-plan workflow and summary template. Both canonical active lesson files were read completely: 7977 bytes / 2660 conservative estimated tokens. No project skills exist. Root retains shared-main synchronization, reviewed current-contract renewal, generated LOC and whole-phase verification/saving.

## Issues Encountered

The initial Task 2 scaffold compile referenced a nonexistent ProofWork::from_u128 helper; the test now uses the existing 32-byte constructor. The first expanded GREEN compile needed a borrowed keyspace name. The next expanded run passed twelve cases but its forged-filter fixture correctly failed earlier with BodyMerkleMismatch because it mutated a transaction without changing the merkle root. The final fixture uses an actual foreign canonical filter with a checked internally consistent canonical envelope; all cases pass. These actual failed attempts remain separately retained below. No product verification failure or out-of-scope source fix remains.

The broad storage regression fixture reported one pre-existing large-directory case exceeding 60 seconds. Output continued to advance and live test/child processes accumulated CPU time; no timeout or termination was inferred. All 542 tests completed successfully in 208.09 seconds. No old target/cache, unrelated process or directory was altered. The one new raw-reader unit fixture creates its own random directory, drops all engine handles and removes only that exact owned fixture; it proves detached decoding, not canonical product reopen.

## Task Commits and Limits

None; `commits: false` follows the strict wrapper. There was no authentication gate or external setup. Product atomic publication, original-store admission/revalidation, actual all-handle canonical reopen and store/runtime binding remain 166-06/07/10. Configured BASIC mode changes and publication remain 166-08/09. This plan only validates saved index preconditions and produces no index effects. Ordinary history, Signet challenge validation, candidate connects/reorgs, public networking defaults, actual-daemon differential qualification and canonical pruning remain their existing later gates. Full default native verification, final independent reviews and normal-hook Git saving remain 166-14/root.

## Automated Evidence

All Cargo commands ran serially through `scripts/command-timings.ts`, using pinned Bun 1.3.9 PATH, the coordinator-approved isolated CARGO_TARGET_DIR and canonical process-local TMPDIR. Logs are ignored and preserved. Counts below exclude unrelated zero-match integration harnesses.

| Check | Actual outcome | Full repository-relative receipt |
| --- | --- | --- |
| Task 1 RED | Exit 101; zero passed, two failed | `.local/open-bitcoin-dev/phase166/plan05/task1-red.log` |
| Task 1 GREEN | Exit 0; five passed | `.local/open-bitcoin-dev/phase166/plan05/task1-green.log` |
| Initial Task 2 scaffold compile | Exit 101; no test execution, nonexistent helper | `.local/open-bitcoin-dev/phase166/plan05/task2-red.log` |
| Genuine Task 2 behavior RED | Exit 101; six passed, one failed on complete canonical acceptance | `.local/open-bitcoin-dev/phase166/plan05/task2-red-behavior.log` |
| Initial expanded GREEN compile | Exit 101; no test execution, borrowed keyspace name required | `.local/open-bitcoin-dev/phase166/plan05/task2-green.log` |
| Expanded fixture failure | Exit 101; twelve passed, one failed on fixture BodyMerkleMismatch | `.local/open-bitcoin-dev/phase166/plan05/task2-green-retest.log` |
| Corrected expanded GREEN | Exit 0; thirteen passed | `.local/open-bitcoin-dev/phase166/plan05/task2-green-final.log` |
| Final planned phase166-proof command | Exit 0; sixteen passed | `.local/open-bitcoin-dev/phase166/plan05/proof-final.log` |
| Final planned phase166-records command | Exit 0; sixteen passed | `.local/open-bitcoin-dev/phase166/plan05/records-final.log` |
| Node all-target/all-feature Clippy with denied warnings | Exit 0 | `.local/open-bitcoin-dev/phase166/plan05/clippy.log` |
| Broad storage regression | Exit 0; 542 passed, zero failed, 208.09 seconds | `.local/open-bitcoin-dev/phase166/plan05/storage-regressions.log` |
| Breadcrumb check | Exit 0; 1225 Rust files verified | `.local/open-bitcoin-dev/phase166/plan05/breadcrumbs.log` |
| Managed standards | Exit 0; zero findings | `.local/open-bitcoin-dev/phase166/plan05/managed.log` |
| Scoped rustfmt check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan05/format.log` |
| Diff whitespace check | Exit 0 | `.local/open-bitcoin-dev/phase166/plan05/diff-check.log` |

The sixteen focused cases cover four canonical selectors, twelve foreign-selector pairs, required-row and namespace absence/empty distinctions, current JSON/work formats, strict metadata classes, authority-byte bounds, sixteen individual binding mutations, self-consistent numerical descendants, sixteen saved BASIC combinations, missing/corrupt artifacts, weak locks, forged content, actual detached engine rows and thirty-two work-byte mutations. The above-u128 LE/hex roundtrip is an independent literal numerical vector only; it never authorizes an artificial chain. No native daemon differential or live-store reopen is claimed.

## Self-Check: PASSED

All ten owned implementation/evidence paths and this summary exist. Both final task commands pass all sixteen focused tests; scoped all-target Clippy, all 542 storage regressions, formatting, breadcrumb registration, managed checks and whitespace review pass. The broad storage count includes its existing helper/no-op cases and is not relabeled as 542 substantive new proof cases. Every new Rust file is breadcrumb-registered, all owned Rust files remain within the size limit, and the source/stub/threat scan found no unfinished proof or additional unmodeled surface. Runtime receipts remain ignored. Standalone YAML delimiters appear only at the top. Commit-existence checks are intentionally inapplicable under the no-commit wrapper. Ready for 166-06; final native/lifecycle/Git acceptance remains root-owned.
