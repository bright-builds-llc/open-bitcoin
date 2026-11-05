---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T02:59:32Z"
commits: false
---

# Plan 29 provider expectation amendment

This planning-only amendment adds one existing test owner to Plan29 Task2. It corrects exactly two obsolete None count expectations and one measurement label for the count-preserving provider already required by Plan29. Independent planning review and root release are required before the executor changes this source. This text supplies neither approval nor executed evidence.

## Source finding and historical boundary

Accepted28 deliberately persisted None on changed ancestry before29 supplied the real canonical occurrence map. Its accepted summary and receipts remain correct for those historical bytes and stay byte-identical. Plan29 deliberately supplies the actual live Some(exact map) through the real flush; retaining the old None test expectation would reject that intended behavior.

The exact newly owned path is `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs`, currently620 formatted lines/22817 bytes, SHA256 `c1987be909b8127ef9a3c2c5e6330c65938d5fc618fa2ebefc7e5a1bf38073d9`, matching the accepted28 summary. Its existing `ordinary_301` helper constructs the genuine301-descendant history for both BASIC modes. Source inventory found exactly one obsolete stored-map `.is_none()` assertion at338–341, one obsolete reopened metadata tuple `(stored.active_chain.clone(), None)` at395–399 and one `counts_policy=None_pending29` label at353. All other absence checks in this helper describe BASIC/progress/append-proof/endpoint semantics and remain unchanged.

Root explicitly authorized including the second same-owner expectation after source inspection found it. The two unchanged source-named cases are:

- `storage::fjall_store::bounded_rows::durable_tests::phase167_durable_rows_reopen_actual_301_basic_disabled`
- `storage::fjall_store::bounded_rows::durable_tests::phase167_durable_rows_reopen_actual_301_basic_enabled`

The module path is grounded in `storage/fjall_store.rs`'s `mod bounded_rows` and `bounded_rows.rs`'s `mod durable_tests`. Source finding is not behavioral RED. Retain actual named failure after counts publication on these old expectations if observed, separately from the original29 missing-map RED and compile/setup errors.

## Exact execution delta and ownership

Plan29 `files_modified` grows from eleven to exactly twelve; Task1 remains its original nine files and Task2 grows from three to four files. No new task, file, helper, case or source owner is added. The original task actions and verify commands remain byte-exact; the appended execution amendment governs this narrow extra owner and clarifies final twelve-owner attribution wherever the original text still says eleven.

Replace the obsolete stored-map assertion with:

```rust
        assert_eq!(
            stored.maybe_confirmed_txid_counts,
            snapshot.maybe_confirmed_txid_counts
        );
```

Keep the separate original live Some-map assertion requiring exactly301 entries byte-exact. Equality therefore proves complete actual map preservation, including every identity and occurrence value, rather than merely nonabsence or length.

Keep the existing `assert_eq!` on `reopened.load_chain_meta_for_open().expect("metadata")`, replacing only its obsolete expected tuple with:

```rust
            (
                snapshot.active_chain.clone(),
                snapshot.maybe_confirmed_txid_counts.clone()
            )
```

This compares both complete active positions and the exact full count map directly to the original captured live snapshot. Preserve the existing wallet/hydration whole-snapshot comparisons to `stored` and all surrounding reopen checks.

Replace only the measurement label string `counts_policy=None_pending29` with `counts_policy=Some_exact_live_map`. Keep `DURABLE_301`, mode and actual measurement debug output unchanged. New measurements include complete canonical counts and must be reported as new observations; they must not be asserted or described as equal to the historical28 missing-count measurements.

All other source bytes must invert exactly: unchanged301 blocks, matured-history fixture semantics, original header/index/active positions/full work/MTP, all Coins/full undo/bodies/status/provenance, BASIC catch-up cap/monotonicity/rows/progress/fresh recovery anchor, original physical manifest equality, complete logical/physical/copied inventory checks, all-handle reopen and every resource ceiling. Preserve both test names, the separate301 map-length assertion and every unrelated None branch. No helper/refactor, test ignore/skip, default, proof factory, migration, raw authority upgrade, dependency, second count owner, budget, deadline or limit change is authorized. All twelve formatted source owners must remain below629 lines; an overrun requires checked adjudication without minification or exception.

## Named verification and final attribution

Retain every original29 automated/module/core/undo/Clippy/style command unchanged. Add the following discovery and exact executions under the original pinned Bun1.3.9, Rust1.94.1, physical process-local TMPDIR and serial cooperative timing-lock contract. Root releases the source correction only after independent approval. Poll within60seconds; retain liveness before termination and all stdout/stderr/exit/source hashes.

```bash
bun run scripts/command-timings.ts run --key phase167-29-provider301-discovery -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::bounded_rows::durable_tests::phase167_durable_rows_reopen_actual_301_basic_ -- --list
bun run scripts/command-timings.ts run --key phase167-29-provider301-disabled -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::bounded_rows::durable_tests::phase167_durable_rows_reopen_actual_301_basic_disabled -- --exact --nocapture --test-threads=1
bun run scripts/command-timings.ts run --key phase167-29-provider301-enabled -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib storage::fjall_store::bounded_rows::durable_tests::phase167_durable_rows_reopen_actual_301_basic_enabled -- --exact --nocapture --test-threads=1
```

Discovery must list exactly the two unchanged301 case names; each exact execution discovers/passes one case, zero failed/ignored. Capture the old-expectation failure as its own attributable compatibility observation, never relabel a compilation failure as behavioral RED. Final full-count resource observations for each mode must include all actual authority/max-row/logical/physical/entry/byte/BASIC/copy/source-equality results under unchanged65536-byte authority rows, separate4096-entry inventory ceilings,536870912-byte budgets and existing depth/path/30-second worker limits. No equality to old28 values is implied; measured new values must independently pass.

Before source mutation, preserve the exact original provider bytes/hash already captured below. After the three authorized replacements and scoped formatting, retain its exact final bytes/hash, a three-change delta and whole-file inverse back to the original620-line accepted28 file. Account for formatting explicitly without broad unrelated differences. Record final before/after hashes for all twelve29 source owners, source/extraction inverses, plan/amendment hashes and the complete command interval/argv/exit/log hashes so each result is attributable to the source state it actually ran. Earlier eleven-owner runs cannot alone qualify this final twelve-owner result. Preserve all failed/intermediate receipts rather than rewriting them.

## Preservation, schedule and decision coverage

The exact Plan29 metadata inverse removes its one added frontmatter owner, its one added Task2 files entry and its appended context/execution-amendment suffix; it restores the full26251-byte original29 file with SHA256 `6c86d241a72abae17c0c01e7d4fa02c3d395429117dde25c7956f0933cfb5a6c`. Every original29 task action, verification command, generated timestamp/lifecycle, objective, interface, threat and success contract remains byte-exact. All other28 plans, original11 six owners/existence checkpoint, historical durable-rows amendment, accepted28 summary, receipts and current checker history remain byte-identical. The accepted28 source provider may change only through the later root-released exact delta above; its summary and receipts are historical evidence and do not change.

There remain29plans/50tasks/24waves. Dependencies and schedules are unchanged:28→29→11→12;29 stays wave13 after28. This second shared28→29 source is dependency-ordered, with no same-wave overlap. Original11 is still0/2 with no summary and exactly its original six owners. Full replay/count comparison, original copied-source/pre-effect equality, immediate original reproof, sealed product reopen/next admission and original11 RED remain mandatory11/12obligations. Source review precedes current guard renewal in17;18offline registration,19parity/breadcrumb/docs and20full default native/fresh pure-core coverage/Bazel plus independent REVIEW/SECURITY/VERIFICATION/lifecycle gates remain unchanged. Compact fallback adjudication remains17/20; stop before168. Root owns shared metadata and normal-hook saving/push. No commit, source guard or requirement acceptance follows from this amendment.

| Decision | Plan/task contribution                          | Coverage | Preserved obligation                                                               |
| -------- | ----------------------------------------------- | -------- | ---------------------------------------------------------------------------------- |
| D-01     | Unchanged original owners                       | Full     | Selectors/support/evidence classes unchanged.                                      |
| D-02     | Original29.2 plus unchanged301provider          | Full     | All301 descendants and separate301-map-length check unchanged.                     |
| D-03     | Original29.1–2 plus appended29.2 provider delta | Full     | Exact live/stored/reopened full map and positions.                                 |
| D-04     | Unchanged original owners                       | Full     | No interpreter/script/policy change.                                               |
| D-05     | Original29.2 and unchanged provider comparisons | Full     | Complete work/MTP/header facts remain exact.                                       |
| D-06     | Original29.1–2 unchanged                        | Full     | Exact occurrence values, seeded None and genuine empty genesis stay distinct.      |
| D-07     | Appended29.2 plus original11/12                 | Full     | Actual stored/reopened data matches live map; replay/product proof still required. |
| D-08     | Appended29.2 and original tests unchanged       | Full     | All other fault/refusal/complete-state/resource assertions preserved.              |
| D-09     | Appended29.2 evidence/inverses                  | Full     | Historical receipts remain exact and new results bind final source.                |
| D-10     | Appended29.2 commands plus original17–20/25     | Full     | Nonzero named checks and all final gates unchanged.                                |

Full denotes complete planning coverage together with existing tasks, not executed acceptance. All existing CVLD requirements remain unchanged; no deferred idea enters this scope. Twelve owners exceed the earlier preferred nine-owner target and generic ten-owner warning while remaining under the fifteen-owner boundary. Independent checker and root must assess this exact bounded warning; no implicit owner expansion follows. Simplification keeps the existing helper, direct equality and one map with no new abstraction.

## Captured baseline and planner-only validation

Ignored evidence is rooted at `.local/open-bitcoin-dev/phase167/amend29provider/`. `capture-before.ts` captured73 original planning/provider/checkpoint path observations and393 historical receipt hashes before plan mutation. `before-manifest.json` records complete snapshot bytes/hashes/existence and discovers exactly the two stale count expectations plus one label in `ordinary_301`. The snapshots include all29plans, phase Markdown history, accepted28 summary, current PLAN-CHECK and original11 six-owner checkpoint; absent original11 owners remain absent. The393 receipt observations cover the existing amend11rows, plan28 and plan11 trees. No historical receipt was rewritten.

| Baseline artifact                |  Bytes | SHA256                                                             |
| -------------------------------- | -----: | ------------------------------------------------------------------ |
| Original29plan                   |  26251 | `6c86d241a72abae17c0c01e7d4fa02c3d395429117dde25c7956f0933cfb5a6c` |
| Accepted28summary                |  24989 | `396968e33007a4e64606a71d062ebb99678e68f6622bbf8af5d52dc33111aff3` |
| Original11durable-rows amendment |  41416 | `f08340333ce129b0823ac5b0a30bfcfe496667d3cc329d9b2cf78b98039f8b44` |
| Pre-append PLAN-CHECK            | 175380 | `8098c8495ff76ab034201fe6d302143847f8c3a399ca34d9bfe888596c315c59` |
| Provider source                  |  22817 | `c1987be909b8127ef9a3c2c5e6330c65938d5fc618fa2ebefc7e5a1bf38073d9` |

Planner validation must check frontmatter/task structure, the complete29inverse and original actions/commands, all28other plan bytes, all preserved history/checkpoint/receipt hashes, unchanged schedules/lifecycle and no same-wave overlap. Retain the exact reconstructed inverse and machine-readable receipt under this new evidence directory. Run Markdown GFM/frontmatter check mode on these two planning files and scoped whitespace checks. Do not format the original29 body as collateral cleanup; its inverse must remain exact.

AGENTS.md local guidance, AGENTS.bright-builds.md, standards-overrides.md and local architecture/code-shape/testing/verification/Rust standards informed the exact existing-map assertion, no extra abstraction, sub629line bound, evidence preservation and serial verification contract. Both canonically distinct active lesson inputs were read fully:6898global+1958repo=8856bytes/2953conservative tokens. The October19audit baseline has no new trigger. No project skills exist. This is Level0internal source-grounded discovery; there is no new external library/API choice. Ongoing root-authorized yolo GSD execution supplies the workflow context; no separate commit is made here.

No source/test modification, Cargo/Clippy/build/Bazel, native verification, shared STATE/ROADMAP/todo/config update, summary/receipt rewrite, staging/commit/push or guard renewal is performed by the planner. Independent checker alone owns the subsequent append-only PLAN-CHECK report. Root alone releases the exact provider source correction after approval.
