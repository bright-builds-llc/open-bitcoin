---
phase: 167-candidate-history-validation-rules
plan: "28"
subsystem: storage
tags: [fjall, durable-storage, inspection, canonical-history]
requires:
  - phase: 167-10
    provides: Genuine managed descendant acceptance and reusable fixtures
provides:
  - Atomic bounded header/index/chain-metadata row families in the existing store
  - Complete data-only decoding and actual301 durable inspection/reopen evidence
  - Synthetic401 count-enrichment compatibility with preserved publication semantics
affects: [167-29, 167-11, 167-12, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Typed complete row families, existing guarded batch replacement]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/chain_meta.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/codec.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/chainstate/live_fixture.rs
key-decisions:
  - Preserve exact small legacy roots and generic large-aggregate read compatibility
  - Keep decoded bytes nonauthorizing and changed-ancestry counts unknown until29
  - Recover a fresh process-local BASIC anchor explicitly after data-only reopen
patterns-established:
  - One typed codec and existing-store batch adapter serve all three logical families
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-22T02:33:45Z"
commits: false
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
duration: "73m 55.147s across executor intervals"
completed: 2026-10-22
---

# Phase 167 Plan 28: Bounded Durable Rows Summary

**Atomic bounded header/index/metadata rows preserve complete301-descendant data through actual copied inspection and storage reopen, with synthetic401 fixture compatibility.**

## Files Created/Modified

The exact nine source owners are:

- `packages/open-bitcoin-node/src/storage/fjall_store.rs` — Registers the cohesive adapters and declared test-only measurement re-export; delegates existing header methods.
- `packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs` — Replaces both trusted header projections in the existing guarded SyncAll batch.
- `packages/open-bitcoin-node/src/storage/fjall_store/coins.rs` — Retains coins/undo/migration behavior after metadata extraction.
- `packages/open-bitcoin-node/src/storage/fjall_store/chain_meta.rs` — Extracted metadata writers/loaders and the same-guard synthetic fixture bridge.
- `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows.rs` — Shared preparation, atomic family replacement and complete reads.
- `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/codec.rs` — Pure typed manifest/segment integrity and bounded detached decoding.
- `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/tests.rs` — Legacy/genesis bytes, actual public writes, replacement and18 independent tamper cases.
- `packages/open-bitcoin-node/src/storage/fjall_store/bounded_rows/durable_tests.rs` — Genuine301 modes, physical/logical measurements, close/reopen and publication faults.
- `packages/open-bitcoin-node/src/chainstate/live_fixture.rs` — Existing enrichment uses the shared family replacement; inline synthetic401/proof/refusal controls.

Four existing files and five new Rust files total3663 formatted lines/136659 bytes. Relative to the exact Plan28 `before/` snapshots, the changes are2008 additions/166 deletions, including the declared moves. No additional source owner, dependency, allowlist or minification was introduced.

## Task Outcomes

1. **Task1 complete:** One data-only representation serves actual public/trusted header writers and every extracted metadata writer/loader. Both header projections remain in one existing batch. Small values retain their exact old root bytes; large values remove the aggregate and use a strict70-byte manifest plus fixed65536-byte nonfinal fragments and a nonempty final fragment. Namespace/family, version, count, length, generation, exact membership and domain-separated complete-stream digest are checked before reconstruction. Malformed/mixed/incomplete families refuse without repair. Generic old oversized aggregates remain readable; strict canonical detached reads refuse them.
1. **Task2 complete:** Genuine genesis+301 descendants in both BASIC modes, complete all-handle data reopen, before/after publication faults, synthetic401 full-map enrichment and raw mixed-family corruption controls pass. The exact original400-prefix regression and all three existing fixture-count controls pass unchanged.

The enabled301 test retains actual bodies, validates/accepts all301 descendants and makes the full trusted header/coins checkpoint, then initializes the actual ordered BASIC owner and catches up from retained validated history using ordinary production turns. This is **postcheckpoint retained-history catch-up**, not an attached BASIC owner during all301 accepts. At most302 test calls, strict monotone progress, every production per-turn cap and the30-second worker deadline remain unchanged.

Before closing, the actual live append proof agrees with live progress and binds an ancestor in the complete active chain; it drops before inspection. A raw storage reopen has no append proof. Existing explicit Enabled BASIC configuration validates the stored data and installs a **fresh process-local anchor at the exact captured live/stored301tip**. Generation/protection/processed/safe comparisons and all302 filter/header/active-projection bytes remain exact. Literal token continuity across process recovery is not claimed.

## Performance and Saving

- Initial executor source/check interval: `2026-10-22T01:12:57.058180Z`–`01:31:08.752408Z`,1091.694seconds (18m11.694s).
- Preserved checkpoint/handoff gap:208.730seconds (3m28.730s).
- Fresh continuation source/check interval: `2026-10-22T01:34:37.482379Z`–`02:30:20.935Z`,3343.453seconds (55m43.453s).
- Combined executor intervals:4435.147seconds (73m55.147s); complete wall interval including handoff:4643.877seconds (77m23.877s). These are elapsed intervals, not an invented active-coding duration.
- Tasks:2/2. Source owners:9. Commits/staging/push: none, as required by `commits:false` and root's saving contract.

`1b472fb225a290df5cdb06eeea2dc67c624c648e` is the existing **main baseline**, not a task commit. Root retains STATE/ROADMAP/REQUIREMENTS/config/todo updates, independent acceptance and normal-hook final saving. No worktree, hook bypass, source-guard renewal, corpus/frozen receipt change or Phase168 work occurred.

## Commands and Results

All Cargo invocations used pinned Bun1.3.9/Rust1.94.1, the existing target, physically resolved process-local TMPDIR and `scripts/command-timings.ts`, serialized through the existing cooperative lock. Each `final-*.json` receipt records the complete argv, UTC interval, exit, log digest and before/after hashes for all nine owners.

Test commands use `cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node FILTER -- --test-threads=1`, wrapped by `bun run scripts/command-timings.ts run --key KEY --`, with the explicit additions shown below. All listed commands exited0; every test selection had nonzero unit discovery, zero failed and zero ignored cases.

| Receipt                    | Timing key / filter or command detail                                                                                                                                    |      Passed |    Harness seconds |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----------: | -----------------: |
| `final-task1`              | `phase167-28-task1`; `phase167_durable_rows`; `--nocapture`                                                                                                              |          32 |             108.41 |
| `final-task2`              | `phase167-28-task2`; `phase167_durable_rows_reopen`; `--nocapture`                                                                                                       |           8 |              94.42 |
| `final-existing400`        | `phase167-28-existing400`; `--lib sync::tests::filter_index::reorg::measurements::phase158_reorg_measurement_retained_over_budget_gap_refuses_before_preview`; `--exact` |           1 |              24.22 |
| `final-fixture-controls`   | `phase167-28-fixture-count-controls`; `phase165_rebind_fixture_counts`                                                                                                   |           3 |               2.31 |
| `final-storage`            | `phase167-28-storage`; `storage::fjall_store::`; `--nocapture`                                                                                                           |         413 |            1090.48 |
| `final-phase166`           | `phase167-28-phase166`; `phase166_`                                                                                                                                      |         133 |            1143.53 |
| `final-snapshot-codec`     | `phase167-28-snapshot-codec`; `storage::snapshot_codec::`                                                                                                                |          75 |               2.88 |
| `final-validation-history` | `phase167-28-validation-history`; `validation_history`                                                                                                                   |          47 |              48.88 |
| `final-durable-undo`       | `phase167-28-durable-undo`; `phase167_durable_undo`, once                                                                                                                |           2 |              14.39 |
| `final-clippy`             | `phase167-28-clippy`; `cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-node --all-targets --all-features -- -D warnings`                                | Strict pass | 135s Cargo display |

These selections **overlap**: the8 reopen tests are a subset of32; storage413 includes the30 storage-owned bounded-row cases and affected snapshot persistence/migration controls, but not the two chainstate fixture cases. Phase166 and validation-history selections overlap other selections. Their counts are not summed as distinct tests. The two durable-undo tests are separate source-bound **seeded historical BIP30** reopen/restore/reconnect and corruption-matrix evidence for both real repeats, not ordinary historical-chain authority.

`final-task1` held its exact nine hashes stable throughout. After it completed, root requested only the helper's two-line documentation clarification; an exact inverse hash proves that is its sole delta from final source. `final-task2` and every later receipt use the final documented bytes and hold those hashes unchanged. Final scoped `rustfmt --edition 2024 --config skip_children=true --check` and `git diff --check` pass for the nine owners. No whole-node/default-native/coverage/Bazel result is inferred.

## Actual Durable Measurements

The final storage selection repeats both genuine modes after all source changes:

| Observation                                         |           Disabled |            Enabled |
| --------------------------------------------------- | -----------------: | -----------------: |
| Active positions / complete Coins / complete undos  |      302 /301 /301 |      302 /301 /301 |
| Complete all-namespace logical entries              |               1252 |               1859 |
| Complete logical key/value/namespace bytes          |            2300681 |            2400525 |
| Maximum actual authority value bytes                |              65536 |              65536 |
| Maximum BASIC value bytes                           |                  0 |                174 |
| Maximum body / full undo bytes                      |          285 /1218 |          285 /1218 |
| Complete physical inventory entries, including root |                 60 |                 60 |
| Actual regular-file bytes / copy consumption        | 67128198 /67128198 | 67128198 /67128198 |
| Actual isolated copy/recovery seconds               |       10.436160041 |       11.706605416 |
| Original names/types/contents/modes/mtime equality  |               true |               true |

The copy-byte observation is the measured original regular-file-byte sum consumed by the unchanged copy routine, not extra scratch allowance. All three final measurement runs are retained; their slowest301 inspection is11.706605416seconds. Physical and logical inventories each retain the4096-entry and536870912-byte ceilings; each actual authority value retains65536bytes, and depth/path/worker limits are unchanged. Malformed detached-map controls run inside copied inspection while the original physical manifest remains equal.

Finite support actually demonstrated is302 genuine positions in both modes and401 synthetic positions. A family can contain at most4095 fragments plus its manifest under the existing entry limit, with the shared all-namespace limit often tighter. No arbitrary-length history or larger resource envelope is claimed.

Every reopened header/index entry, active position/full work/MTP, Coin, full undo, retained body, validation-history record/provenance and complete BASIC linked filter/header/active projection is compared. Wallet and ordinary hydration loaders agree on the complete stored snapshot. Enabled processed/safe endpoints both reach301. The stored changed-ancestry occurrence map is explicitly **None pending29**, while the live genuine map has301 entries. Synthetic401 enrichment preserves every supplied pair and multiplicity, remains synthetic and preserves full state/provenance, coverage semantics, BASIC state/progress/protection, payload epoch and already-held append proof validity; equal enrichment is write-free and original refusal branches remain nonmutating.

## RED, Setup, Checkpoint and Continuation Attribution

The preserved initial checkpoint and attempts are historical; intermediate results do not validate final bytes.

| Receipt/history                                                               | Actual attribution                                                                                                                                                                                            |
| ----------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Original Plan11 corrected-work measurement                                    | Genuine logical-stream RED:302positions/301Coins/undos; headers/index591086bytes each, complete metadata768467, body285/undo1218. Not an actual persisted inventory. Its earlier ZERO-work attempt was setup. |
| `task1-red.log`                                                               | Genuine unchanged public-writer RED:1case, persisted759267-byte authority value exceeded65536.                                                                                                                |
| `task1-first-green.log`, `existing400-pre-adapter-red.log`, `task2-first.log` | Import/visibility/type compilation setup errors; not behavioral RED.                                                                                                                                          |
| `existing400-pre-adapter-behavior-red.log`                                    | Genuine exact unchanged400 regression RED:1/1,34.64s, mixed aggregate/segmented metadata after segmentation and before the fixture bridge.                                                                    |
| `task1-expanded.log`, `task2-expanded.log`                                    | Intermediate25/25 and7/7; pre-real-BASIC enabled954logical/basic44 observations are superseded and are not complete BASIC evidence.                                                                           |
| `task2-final-candidate.log`                                                   | 8discovered/6passed/2failed: ineffective physical-Coin mismatch setup and absent retained bodies for actual BASIC. Setup issues.                                                                              |
| `task2-corrected-setups.log`                                                  | 8/6/2,79.07s: physical H marker correctly caused the original CoinsStorage error;10 ordinary turns only reached79. Initial executor stopped at its correction limit with0/2 tasks complete.                   |
| `continuation-task2.log`                                                      | 8/7/1,90.57s: reviewed H/progress corrections pass; raw reopen correctly lacks a process-local BASIC proof until explicit configured recovery.                                                                |
| `continuation-final-task1.log`                                                | 32/31/1,109.58s: configured recovery succeeded and all bytes/state matched; a test incorrectly required literal live/recovered branch-token identity.                                                         |
| `continuation-clippy-pre-final.log`                                           | One new test-module placement lint; the whole unchanged test module moved to EOF without suppression.                                                                                                         |
| Final receipts above                                                          | All actual final behavior/diagnostic gates pass, with complete command/source attribution.                                                                                                                    |

Root reviewed and released the bounded test corrections. The H scenario now retains before/after physical observations as Results and expects the exact original CoinsStorage detail; other mismatches retain WorkRebindMismatch and origin retains UntrustedWorkLineage. The genuine unflushed Coin overlay remains in the mismatch setup. BASIC catch-up retains ordinary caps and monotone progress. The recovered branch check now validates the intended fresh anchor against the complete captured chain, retaining all generation/protection/endpoint/row comparisons.

An external model-capacity interruption occurred while the existing serialized verification cell continued; commands were neither repeated nor interrupted because of it. One rejected auxiliary poll used an OS PID instead of a tool session and had no process effect. An unsupported read-only GSD help probe changed no state. The ignored source-review note's initial Markdown check was formatted/rechecked with installed GFM/frontmatter plugins. The long Phase166 command crossed its advisory timing threshold, retained automatic liveness diagnostics, then completed normally; no deadline or budget was increased. An initial summary-only parser check matched the earlier `Verification` heading and misclassified the retained historical chronology; renaming that heading to `Commands and Results` preserved the history and corrected parser selection. This was metadata validation, not behavioral RED.

## Source Closure, Simplification and Threat Review

The final source hashes are relative to `packages/open-bitcoin-node/src/`:

| Source                                              | Lines | Bytes | SHA256                                                             |
| --------------------------------------------------- | ----: | ----: | ------------------------------------------------------------------ |
| `storage/fjall_store.rs`                            |   561 | 19560 | `9a5ee1b35a6afbda6e4e8013a52023ad3b410b1c2d949ae601367e3410c9d97f` |
| `storage/fjall_store/validation_history.rs`         |   485 | 18195 | `7d5cf7c378312415697a6f2c7a8291e9e50f328cda7100015623acd7a22311fb` |
| `storage/fjall_store/coins.rs`                      |   482 | 18099 | `992ad1e801745877bd11c3926665c90a2a9f501c833441325116e9ab772fd89e` |
| `storage/fjall_store/chain_meta.rs`                 |   170 |  7165 | `383b661a8655b52a66863929e90b74f1832a0f4f4c3f0baa545d25074351e24d` |
| `storage/fjall_store/bounded_rows.rs`               |   146 |  4974 | `354cf0948b1abe2bda01f023632aae0867e8e940917ba1cba50c6f43feaf5333` |
| `storage/fjall_store/bounded_rows/codec.rs`         |   195 |  7012 | `b06a0f43063bbb38b10306e5ee73ae2660718ece4559d4f82cd1ed2c9c7fd0e3` |
| `storage/fjall_store/bounded_rows/tests.rs`         |   426 | 15065 | `0709d21b7ed8772a73b07d010ca176e10bf26cd7c72feec3f654a542ebe6bc85` |
| `storage/fjall_store/bounded_rows/durable_tests.rs` |   620 | 22817 | `c1987be909b8127ef9a3c2c5e6330c65938d5fc618fa2ebefc7e5a1bf38073d9` |
| `chainstate/live_fixture.rs`                        |   578 | 23772 | `f9813e7620ebf9006f6783308fc9ecabd485ba3107381d9135c477b08ecbe58f` |

The exact extraction inverse restores whole original coins.rs and fjall_store.rs, reversing only declared moves/import/privacy adaptations. All five moved metadata methods separately invert their shared-provider substitutions to the original bytes. The raw helper, both unrelated fixture methods, original400 measurement/fixture source and all fixture-count control source remain byte-exact. The enrichment's original predicates/error order invert exactly before its one intended final write replacement. Trusted/public writer diffs retain their existing guards, faults, ordering and same paired batch.

The final audit preserves all51 frozen checkpoint/before/attempt inputs and all10 Plan11 source/existence/log observations, including the six declared owner states and both original measurement logs. Original Plan11 remains0/2 without a summary. The corrected-work log SHA256 remains `bcf1da366de96d452b13f83ea654ce09b4a23a33dd7b43bf69b03306046a6d91`.

Simplification retained one pure codec and one existing-store adapter rather than a second store, serializer or proof factory. Long integration tests intentionally keep a single real lifecycle and complete close/reopen assertions together; every owner stays below629 formatted lines. The planned T-167-28-01–05 mitigations are present: strict bounded hostile-data parsing, existing atomic guarded publication, data-only decoding, actual resource/source evidence and same-held-store fixture semantics. No unplanned trust surface or production stub was found. Existing absence/default values express deliberate legacy/counts/budget semantics.

Local AGENTS/sidecar/overrides and architecture/code-shape/testing/verification/Rust standards informed these checks. Both active lesson files were read completely within8856bytes/2953 estimated tokens; the current audit baseline needs no new audit. Relevant README boundaries were checked: the internal row provider does not complete ordinary product reopen, so Phase19 retains the contributor/parity/breadcrumb integration work.

Primary ignored evidence is under `.local/open-bitcoin-dev/phase167/plan28/`: `continuation-start.json`, `continuation-source-audit-end.json`, `continuation-source-review.md`, `continuation-completion.json`, the ten `final-*.json`/log pairs and final rustfmt/diff logs. Original `before/`, `checkpoint/`, `plan11-before/`, checkpoint notes/closure and historical attempt logs remain unchanged.

## Remaining Phase Boundaries

Plan29 must carry the existing complete canonical occurrence map through the real flush; None on changed ancestry is not complete-count success. Plan11 must independently re-admit/replay every accepted fact using bounded data after29; its original RED is unsuppressed. Plan12 retains sealed product reopening/next acceptance and copied-worker proof integration. Source/security/goal/lifecycle acceptance, current guard review/renewal, parity/breadcrumb/docs registration and full default native/fresh pure-core coverage/Bazel remain their declared later owners. The compact fallback difference still requires mandatory17/20 adjudication. Requirements listed above record this plan's contribution, not whole-phase requirement or product acceptance. Root independently accepts28 before releasing29; this executor stops here.

## Self-Check: PASSED
