---
phase: 167-candidate-history-validation-rules
plan: "27"
subsystem: chainstate
tags: [rust, basic-filters, complete-undo, resource-accounting]
requires:
  - phase: 167-07
    provides: Complete overwrite batches and distinct captured-empty provenance
  - phase: 167-08
    provides: Root-accepted lossless persistence and complete node regression
provides:
  - One private checked complete-undo allocation traversal for both existing work entry points
  - Actual pre-fix allocation RED and final exact-byte, capture and refusal evidence
affects: [167-10, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Shared checked logical allocation accounting, Small near-limit accumulator probes]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/filter_index/phase167_tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate/filter_index.rs
key-decisions:
  - "Count each complete undo container element and both owned script sequences once, retaining the existing byte and item ceilings."
  - "Overwrite resource bytes affect undo and owned clone bytes only; BASIC script selection and provenance remain unchanged."
requirements-completed: [CVLD-06, CVLD-03]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T22:52:39.155Z"
commits: false
duration: 4min18s
completed: 2026-10-21
---

# Phase 167 Plan 27: Complete undo resource accounting Summary

**Accepted owned and borrowed BASIC work now count every overwrite batch, entry and both Coin script allocations through one checked traversal before reviewed copy sites.**

## Files Created/Modified

- Modified: `packages/open-bitcoin-node/src/chainstate/filter_index.rs` — shared private add_undo_work, both actual caller sites and test child registration;529lines/19686bytes.
- Created: `packages/open-bitcoin-node/src/chainstate/filter_index/phase167_tests.rs` — eleven allocation, capture, logical-length and checked-refusal tests;418lines/14523bytes.

Exactly one task and two Rust owners. This summary and ignored plan27 receipts are execution records. The preexisting main-repository checkout reference `1b472fb225a290df5cdb06eeea2dc67c624c648e` resolves to a commit; it is **not a task commit**. No staging, commit, push, worktree or hook bypass occurred. Root owns shared STATE/ROADMAP/REQUIREMENTS/config/todo and all saving after the complete phase gates.

## Completed Task

Task1 is complete within its bounded accounting scope. Both actual fact_work and borrowed_undo_work call the same private add_undo_work, which checks each addition through existing add_bytes. Ordinary restored-input script accounting remains in that shared traversal. Borrowed absence still returns default TurnWork; borrowed errors still map to the existing StorageError::Corruption BASIC undo bound category. No public interface, authority constructor, dependency, crate or error variant was added.

The exact logical undo formula is:

```text
sizeof(BlockUndo)
+ each TxUndo: sizeof(TxUndo)
+ each restored Coin: sizeof(Coin) + restored script length
+ each BlockOverwriteUndo: sizeof(BlockOverwriteUndo)
+ each OverwrittenCoin: sizeof(OverwrittenCoin) + previous script length + new script length
```

Vector headers and both inline Coins/outputs are already inside their containing types. No inline field is charged again. Elements use logical length; spare capacity, allocator bookkeeping and RSS are outside this contract. Equal previous/new script contents still contribute both independently owned byte sequences. The small exact fixture totals513bytes:125ordinary/header bytes +104batch bytes +272entry bytes +12script bytes on this target.

Overwrite data increases only undo_bytes and therefore the owned body_bytes+undo_bytes+sizeof(ChainPosition) clone sum. Borrowed cloned_bytes stays zero. Output/restored-input script_items and script_bytes remain unchanged; overwrite scripts do not become extra BASIC spent inputs/filter elements. The384MiB byte and1,000,000item ceilings, checked arithmetic and three preexisting phase157 controls remain exact.

## Commands and Actual Outcomes

All Cargo commands ran serially through scripts/command-timings.ts with pinned Bun1.3.9 PATH, Rust1.94.1, existing targets/cooperative build lock and process-local physical TMPDIR from pwd -P. Complete outcomes, source snapshots and exit-code receipts are retained under `.local/open-bitcoin-dev/phase167/plan27/`. No native registration, threshold, timeout, source guard, frozen receipt or accepted corpus was changed.

| Stage / actual command                                                                                               | Actual outcome                                                                                                                                                                       | Receipt                                             |
| -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------- |
| Initial cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase167_undo_budget -- --test-threads=1 | Exit101: test imports incorrectly assumed new undo types were root reexports. Corrected only the test imports to chainstate::types. Compilation/setup error, **not behavioral RED**. | red.log                                             |
| Same focused command, corrected test imports and unchanged production accounting                                     | Exit101:4discovered/4failed,0ignored. Owned/borrowed125vs513; empty batch0vs104; entries0vs272.                                                                                      | behavior-red.log                                    |
| Same focused command, adding one distinct script-allocation expectation before production work                       | Exit101:5discovered/5failed,0ignored. All prior omissions remain; the separate both-script assertion independently observes0vs12. Production accounting is still unchanged.          | behavior-red-all.log                                |
| Scoped rustfmt write on only the two owned files after implementation                                                | Exit0.                                                                                                                                                                               | format-write.log                                    |
| Final prescribed focused phase167_undo_budget command                                                                | Exit0:11discovered/11passed,0failed/ignored;13.22s compile,0.00s harness.                                                                                                            | green.log                                           |
| cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node chainstate::filter_index:: -- --test-threads=1   | Exit0:14discovered/14passed,0failed/ignored,including all three unchanged phase157 controls;0.16s preparation,0.00s harness.                                                         | existing-controls.log                               |
| cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings    | Exit0,14.08s; no compiler warning/error.                                                                                                                                             | clippy.log                                          |
| Prescribed scoped rustfmt --edition2024 --config skip_children=true --check                                          | Exit0.                                                                                                                                                                               | rustfmt-final.log                                   |
| Prescribed scoped git diff --check                                                                                   | Exit0.                                                                                                                                                                               | diff-check-final.log                                |
| Full tracked diff and whole untracked child review                                                                   | Both complete sources reviewed; new-file diff exit1 means expected new-file differences. Neither owner exceeds628lines.                                                              | owned-tracked.diff;owned-new.diff;final-checks.json |
| GSD current key links                                                                                                | all_verified:true,2/2. Actual overwrite traversal and real registered test child; both caller sites confirmed separately.                                                            | key-links-final.json                                |

Separate zero-match integration binaries in focused runs are not counted as evidence. These intentionally focused selectors do not claim a complete node regression; the fully accepted08 unfiltered result remains attributable to08 and final20 owns the next whole-phase native run.

The initial run briefly waited for Cursor rust-analyzer's workspace Cargo artifact lock. Observed CLI rustc processes established liveness and the wait resolved normally; red-processes.log retains the observation. No IDE/process was killed or reconfigured, target was copied/cleaned, limit was changed, or quiet job was interrupted. Resumable jobs were polled within60seconds. No authentication gate occurred.

Before production edits, the parent baseline matched its checked522line/19810byte source and SHA2569cf19c472ce8b25d0c3d0c33dc7e2465d689fe813333baeb50e558dced088c48. Original source, registration-only parent and each RED test snapshot are under before/. The first setup snapshot is retained separately from both actual behavioral REDs. Exploratory combined reads initially exceeded display budgets and one shell glob omitted the amendment's167prefix; actual files were then read at their complete paths and relevant sections reloaded in bounded portions. Absent guessed filter-header/formatter config paths were read-only setup observations. No source or behavioral failure is attributed to them.

## Executable Boundary and Capture Evidence

The eleven named tests independently calculate concrete sizeof totals and known script lengths. They cover multiple entries, unequal scripts, equal-content independently owned scripts, exact empty-batch/entry/script deltas, owned clone totals, borrowed equality and zero clones, ordinary restored script contributions, absent undo and spare vector capacity. Original assertions were retained; no ignore, skip, filter exception, budget override or proof seam was introduced.

Small real add_undo_work accumulators initialized near the unchanged ceiling accept exactly the cap, refuse one byte over and refuse u64 overflow. A separate table makes each newly counted batch, entry, previous script and new script cross its remaining allowance. Tests allocate only tiny undo values; no hundreds-of-MiB allocation or oversized cloned object is used. Partial work on an error is private and no facts are published by that result.

The positive capture_reorg test uses a consistent synthetic transaction body/merkle/header/position and ordinary restored-input correspondence. It executes nonempty, one empty batch and zero outer batches, comparing the full preserved undo and position. It verifies exact copied body/undo/ChainPosition bytes, independently specified BASIC script/filter contents, unchanged output/restored counters and witness elision while the original witness remains present. Batch metadata, including its deliberately untrusted synthetic integrity bytes, is preserved rather than re-sealed. These are **structural/resource fixtures only**: they do not authenticate prior Coins, genuine historical acceptance, canonical lineage or an actual daemon.

## Clone Ordering, Simplification and Threat Review

Source review traces capture_reorg through fallible fact_work(block,undo) and capture through fallible fact_work(block,staged.undo()) before either clone_basic_body or the full undo clone. Both use the actual shared traversal. Executable small-accumulator refusals and positive captures complement this reviewed order. **An instrumented oversized capture test is not claimed.** Existing copy logic and body accounting are byte-identical to the original source; only the misleading witness-counting comment was clarified to match the unchanged witness-elision behavior.

The simplification pass retains one traversal and one borrowed error-conversion boundary instead of duplicated accounting. T-167-27-01 is covered by exact complete traversal and actual near-cap/overflow controls;T-167-27-02 by preserved full undo, unknown/empty distinction and unchanged BASIC output/spent selection;T-167-27-03 by retained actual pre-fix RED/setup chronology and final nonzero results. No new network endpoint, auth path, file-access pattern, schema change or undeclared security surface was introduced. No unresolved stub remains. Empty vectors in tests intentionally represent unknown or explicit empty scan data; they are never manufactured historical provenance.

AGENTS.md local guidance, Bright Builds sidecar/overrides, managed architecture/code-shape/testing/verification/Rust pages, latest bounded independent27PASS, accepted07/08 summaries and current context/research/coverage/scope amendments informed this implementation. Both canonically distinct active lesson files were read completely:6898+1958=8856bytes,2953conservative estimated tokens. The recent baseline applies; no new audit trigger, archive load, project skill or dependency research was needed. Remote refs were fetched; the dirty shared worktree was preserved without rebase.

## Deviations and Remaining Gates

No ownership, architecture or behavior expansion occurred. The test-only import correction is a retained setup fix within the declared child. The explicit commits:false/root-owned metadata contract supersedes generic executor commit/state-update steps. This bounded CVLD-06/CVLD-03 contribution does not close either whole requirement or phase. Shared metadata was not edited.

Plan17 retains independent semantic source review before current source-guard renewal. Plan19 registers this child's pinned breadcrumbs and retains parity/README reconciliation. Plan18's existing native command registration stays unchanged; ordinary Cargo discovery includes this child. Plan20 still owns full native/fresh pure-core coverage/Bazel and independent source/security/goal/lifecycle gates. Public networking/defaults, consensus/BIP30 restoration, provenance/integrity validation and BASIC semantics remain unchanged. The accepted stronger earlier-Coin restoration versus pinned Knots remains attributable to07/08; no new historical parity claim is made. Phase168 was not started. This executor stops for root acceptance before09/10continuation.

## Performance Attribution

One task,two actual source owners. Measured source/check interval2026-10-21T22:46:49Z through2026-10-21T22:51:07.257Z is258seconds(4min18s),including initial setup correction, actual behavioral REDs and final scoped checks/source review. Earlier context loading, subsequent summary formatting/validation and predecessor durations are excluded. final-checks.json records both endpoints,actual exit outcomes and source hashes. Final parent SHA2566d2d4ab214a578b11c56fac9950b1311f537224a052df6f372802f410002cf05;child SHA2562975e5636668dba95c68cb3a409f4f836bdc643d3a84d276ece23a053269323b. Both match Clippy's recorded input hashes.

The initial summary mdformat check reported that the newly written file was not formatted. The available mdformat1.0.0 with GFM1.0.0/frontmatter2.1.2 then formatted only this summary and the final check passed; summary-format-first.log,summary-format-write.log and summary-format-final.log retain that metadata chronology. Source bytes and behavioral receipts were unchanged. This formatter finding is not a product failure or behavioral RED.

## Self-Check: PASSED

- Both declared source owners and this summary exist; parent529lines and child418lines remain within the source-size guidance.
- Final11focused tests,14module controls,strict node Clippy,scoped formatting and whitespace checks pass.
- The three original phase157controls,body accounting and clone implementation remain byte-identical; actual shared callers and registered child supply both current key links.
- The labelled preexisting main-repository baseline resolves to a commit; no task commit is asserted. commits:false.
- GSD verify-summary --check-count2 reports passed:true,2checked/2found and errors:[]; frontmatter validation reports valid:true,missing:[]. Receipts are verify-summary.json and frontmatter.json. Both executable key links pass2/2.
- Shared planning/saving and whole-phase acceptance remain root-owned; this handoff is limited to27.
