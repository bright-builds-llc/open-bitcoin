---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "04"
subsystem: storage
tags: [rust, fjall, basic-filters, capability, durable-receipts]
requires:
  - phase: 157-02
    provides: Complete configured prefix/suffix preflight before prune resume
  - phase: 157-03
    provides: Pure progress contracts and exported envelope ceilings
provides:
  - Recovery-minted opaque same-store append proof and bounded frontier checks
  - Concrete writer invalidation and pending-only metadata receipts
  - Private recovered manager lineage and consuming exact own-flush completion
affects: [157-05, 157-06, 157-07, 157-09, 157-10]
tech-stack:
  added: []
  patterns: [opaque capabilities, checked own-publication receipts, accepted-before-persist lineage]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append_proof.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/storage/coins_view.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle/fjall_sink.rs
    - packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/sync/open_runtime.rs
key-decisions:
  - "Public and generic snapshot constructors are untracked; only the actual recovered specialized runtime constructor seeds lineage."
  - "Matching raw coins and metadata can produce pending facts, never append authority."
  - "A current proof revision R must match the actual own completed coins R+1 and metadata R+2 before a private consuming completion refreshes authority."
  - "PreserveSaved retains compatibility without new suffix preflight or bounded append proof."
requirements-completed: []
requirements-addressed: [CFIX-01, CFAC-02]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T18:18:39Z
duration: 84min
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase verification and strict finalization
---

# Phase 157 Plan 04: Recovery Proof and Sealed Durable Publication Summary

**Complete configured recovery mints bounded append authority; only an actual recovered manager's own validated coins-and-metadata flush can refresh it.**

## Performance

- First recorded RED began: 2026-10-05T16:54:18.560Z.
- Summary written: 2026-10-05T18:18:39Z.
- Measured execution/documentation window: approximately 84 minutes, including the required architectural amendment and independent checking. Initial instruction/context loading precedes this measured window.
- Tasks: 3/3 after the checked root amendment.
- First-party Rust paths: 14, including one new test file; summary: one.
- All Cargo work used pinned Bun 1.3.9 and the command-timings wrapper serially. No old preserved target cache was enumerated or changed. No Git staging/commit/push or shared STATE/ROADMAP/REQUIREMENTS/config mutation occurred.

## Accomplishments

- `BasicFilterAppendProof` has private fields and binds the publication Arc/store incarnation to exact effective ownership, lifecycle generation, recovered branch, processed and safe checkpoints, current durable tip and checked revision. Only complete configured enable/recovery and required-history preflight install it. Full forest, projection, ancestry and general record readers retain their existing validation.
- Preparation/completion compare maintained capability facts, bounded saved endpoint/direct predecessor/projection and actual coins H/B. The 16-versus-256 committed-prefix control proves equal nonzero indexed point-read counts below 20 for preparation plus checking; this is a deterministic point-read bound, not a total latency or all-resource measurement.
- Every concrete B/H batch path, including replay/limited writes, invalidates before mutation under the shared publication guard. Raw metadata, seeds and migration metadata share a guarded unverified publication helper. The no-tip migration writer holds the same guard through its concrete coin/B effects. All store clones see invalidation, and ambiguous filter publication errors poison live work.
- Ordinary coins completion retains named pending coins facts; compatible full metadata serialization checks actual B, absence of H and complete borrowed ancestry against the old verified durable anchor, then retains named pending metadata. Public raw/sink calls cannot restore authority.
- `ManagedChainstate::from_recovered_chainstate` is crate-private and specialized to the genuine Fjall parent and concrete sink. Actual `DurableSyncRuntime` invokes it after configured initialization. Public/general constructors, snapshot reconstruction and memory Clone carry None. Actual direct validation/prepared absorb advance lineage before fallible persistence; disconnect/reorg/preview/test replacement clear it.
- A tracked managed flush captures R only from current valid proof. Its own actual cache flush always calls the genuine parent; coins complete at checked R+1 and metadata at R+2. Only that same invocation returning `Ok(wrote_coins=true)` constructs non-Clone, non-Copy, non-serializable `CompletedValidatedFlush`. Concrete confirmation checks the exact receipt, store/branch/generation/accepted endpoint, actual B/no H and bounded owner before consuming pending state and restoring proof.
- Error/noop cleanup cannot rebaseline pending facts into authority, preserves unaffected valid proof, and keeps achieved delete/error accounting. Late rejected receipts clear only their matching pending facts. Safe checkpoint/protection do not advance merely because a durable flush or raw record exists.

## Local RED / GREEN / REFACTOR Evidence

| Boundary                            | Observed RED                                                                                                          | Final evidence                                                                                            |
| ----------------------------------- | --------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| Initial private proof contracts     | Registered four tests failed compilation with missing capability methods; exit 101.                                   | Initial four passed, then expanded proof matrix passed. An introduced missing trait import was corrected. |
| Concrete writer contracts           | Missing private writer API failed compilation; one initial Coin fixture constructor was corrected.                    | Raw matching metadata, coins-only, limited interrupted H, fault/reopen and pending-only controls pass.    |
| Public raw writer forgery           | Raw CoinsView plus public FlushPersistSink incorrectly refreshed the intermediate implementation: 0/1 passed.         | Pending-only correction passed the same 1/1 control; final suite retains it.                              |
| Actual recovered managed completion | Real configured runtime followed by Always flush produced no usable proof: 0/1 passed.                                | Actual runtime and real direct/staged acceptance-to-flush controls pass.                                  |
| Final amended matrix                | A fixture attempted private network read/mutate and failed compilation; corrected without widening network internals. | **36 passed, 0 failed, 0 ignored**, 1,137 filtered; final execution **23.16s**.                           |

The final matrix contains 16 append-proof controls and 20 manager/writer controls. It includes every raw-clone window before coins, between coins/metadata and after metadata, nominal pending recreation, foreign genuine parent/completion, fake snapshot/custom V, rejected/noop work, revision exhaustion, stale/replayed completion, lifecycle changes, reconstruction/replacement/Clone and actual fault/reopen.

One intermediate default-parallel rerun passed 34/35 but hit a datadir lock before fixture startup because parallel interleave fixtures reused a name and timestamp. A test-only AtomicU64 suffix now guarantees distinct datadirs. Final default-parallel 36/36 passed; no production global permit or thread identity was introduced.

## Verification and Remaining Whole-Phase Gates

All Cargo arguments below use `--manifest-path packages/Cargo.toml -p open-bitcoin-node` through the pinned timing wrapper. Counts overlap and are not a distinct-test sum.

| Check                                                                         | Result                                                                                                |
| ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `test --lib phase157_proof_`, final isolated fixtures and guarded writers     | **36/36 passed**, 23.16s execution; 1,137 filtered                                                    |
| `test --lib filter_index`, after sealing/cleanup                              | **116/116 passed**, 70.60s execution; 1,056 filtered                                                  |
| `test --lib chainstate::tests`                                                | **11/11 passed**, 0.02s; includes no forced Always flush, prepared isolation and ordinary persistence |
| `test --lib storage::coins_view`, initial writer integration                  | **14/14 passed**, 4.97s                                                                               |
| `test --lib schema`, after final guarded raw seed/migration writer adjustment | **19/19 passed**, 4.62s; includes actual schema migration controls                                    |
| `test --lib restart_chainstate`                                               | **8 behavioral controls passed; 2 literal source assertions failed**, 3.87s                           |
| `clippy --all-targets --all-features -- -D warnings`                          | **Not passed:** three staged Plan 05 consumer methods are currently unused in normal lib compilation  |
| Scoped Rust formatting across all 14 paths; diff whitespace                   | **Passed**                                                                                            |

The two restart failures are `open_runtime_does_not_hydrate_leftover_utxos` and `open_stores_initialize_lifecycle_and_cache`, at `persist_and_hydrate.rs:19` and `:32`. Their old `from_chainstate` literal is intentionally replaced by the actual private recovered constructor. Root assigned their narrow migration, along with the historical Phase 155 Bun constructor guards, to Plan 10. No duplicate calls, misleading comments or bypasses were added.

Normal-lib Clippy reports only the two diagnostic groups covering `BasicFilterAppendProof::processed`, `safe_checkpoint` and `FjallNodeStore::check_basic_filter_append_proof`. These are real planned Plan 05 APIs; no allow, visibility widening or fake usage was added. Root will consume them in Plan 05 before full-phase finalization. This summary does **not** claim Clippy, full native verification, coverage, Bazel, historical guards or whole-phase completion passed.

## Files and Ownership

The 14 listed source paths implement the single existing store/manager boundary. Earlier Plans 01–03 edits were preserved. Opaque lineage/completion support and the specialized recovered constructor live in existing `chainstate/fjall_store.rs`; concrete sink and primary matrix live in `flush_lifecycle/fjall_sink.rs`. This root-approved relocation keeps the files at 550 and 600 lines; the narrow lifecycle registration is 625 lines. Every touched source remains at most 628 lines.

The new `append_proof.rs` immediately carries the exact existing pinned index/base.cpp, index/blockfilterindex.cpp and node/blockstorage.cpp breadcrumbs. Plan 10 owns catalog registration and final breadcrumb verification.

## Decisions, Deviations and Simplification

Repo-local AGENTS guidance, sidecar, placeholder-only overrides, architecture, code-shape, testing, verification, local-guidance and Rust standards informed this work. Both active lesson sources were fully read: 7,188 bytes and 2,397 conservative estimated tokens; no new lesson audit trigger was identified in the inherited context.

- **Checked architectural amendment:** The original two-task plan could not safely distinguish public raw/sink/fake-Managed publication from validated runtime publication. The behavioral forgery RED proved it. Root commissioned a focused planner and independent checker; the 14-path, three-task amendment passed with zero issues before new owner/hook edits.
- **Root-approved module-shape adaptation:** Moving opaque support into the existing Fjall adapter avoided a 714-line sink and a new test child, while retaining private fields, the same sealed API and exact tests.
- **Rule 2 / concrete writer coverage:** Final source audit moved raw seed/migration metadata through the guarded unverified helper and retained the guard through no-tip B removal, closing the span between early invalidation and actual later effects.
- **Rule 1 / fixture isolation:** Added deterministic unique suffixes after the observed parallel datadir collision.
- The simplification pass reused the existing publication mutex, genuine cache/sink types and one checked R→R+1→R+2 receipt. It rejected thread/global permits, additional mutexes, per-view receipt cells, new dependencies and index-owned history vectors.
- Strict wrapper instructions defer all task/TDD/metadata commits and state/requirements updates to root.

## Adapter Handoff

- `maybe_basic_filter_append_proof(&self) -> Result<Option<BasicFilterAppendProof>, StorageError>` prepares current same-store authority.
- `check_basic_filter_append_proof(&self, &BasicFilterAppendProof) -> Result<(), StorageError>` and guarded `check_basic_filter_append_proof_guarded(..., &PublicationControl) -> Result<BasicFilterAppendIdentity, StorageError>` compare bounded current facts.
- Proof getters expose generation, branch_identity, processed, safe_checkpoint, durable_tip and revision. They confer no public constructor.
- Adapter-only identity contains owner, generation, branch_identity, processed, safe_checkpoint, durable_height/hash and revision. `finish_basic_filter_batch` invalidates before commit; Plan 05 must extend/reinstall only an achieved contiguous publication after success.
- Processed initially equals safe after recovery and remains unchanged by durable flush refresh. When Plan 05 introduces processed rows ahead of safe durability, it must add a separate bounded exact processed row/projection probe; the current safe owner probe covers both while equal. No full reader/fence/prefix scan may enter that path.
- Plan 06 must preserve `maybe_validated_lineage`, `observe_validated_lineage`, constructor None defaults, replacement invalidation and the same-call completion hook while adding its ordered progress owner. It must not infer or reseed authority from pure AcceptedIndexTarget/ValidatedIndexDurability values.
- Envelope ceilings reuse Plan 03 exports: 128 candidates, MAX_SIZE + 128×170 aggregate bytes, and MAX_SIZE + 170 singleton bytes. Production input/work defaults and total turn operation measurement remain Plan 07.

## Threat Mitigations and Limits

T-157-11 is covered by opaque same-store proof, private actual-runtime provenance and default-untracked public construction. T-157-12 is covered by guarded concrete writers, pending-only metadata, exact achieved receipts and adversarial raw/interleave/fault/reopen controls. T-157-13 is covered by bounded endpoint/coins checks; complete integrity/preflight and borrowed full ancestry validation remain at recovery or existing full metadata flush.

No new network endpoint, authentication path, on-disk schema or filesystem trust boundary was introduced outside the amended threat model. No goal-blocking stub was found. Real faults are injected software publication-boundary evidence, not hardware power-loss simulation. The existing full flush's active-chain/undo serialization cost remains outside bounded index turns; no new per-connect flush is forced. Synthetic accepted fixtures and codec-only prefix fixtures do not claim public-mainnet provenance or production/funds readiness.

## Task Commits and Next Readiness

Tasks 1–3 and this summary are ready for root finalization; no commits were created. Requirements remain uncompleted here. Plan 05 can consume the private capability immediately, followed by Plan 06 lineage integration and the remaining measured/runtime/closeout plans.

## Self-Check: PASSED

All 14 declared Rust paths and this summary exist; the new test file is registered and carries its pinned breadcrumbs. The final default-parallel 36-case matrix, inherited fencing/managed core and final affected migration checks passed with the exact counts above. All scoped Rust formatting and diff whitespace checks passed. The summary has exactly two standalone frontmatter delimiters. Stub/threat-surface scans found no goal-blocking placeholder or unplanned boundary. No commit-existence assertion applies because commits are explicitly deferred. The separately identified normal-lib diagnostics and two legacy literal-source failures remain owned by Plans 05 and 10; full-phase completion is not claimed.
