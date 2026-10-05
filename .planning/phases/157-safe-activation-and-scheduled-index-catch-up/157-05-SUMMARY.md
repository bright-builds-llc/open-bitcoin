---
phase: 157-safe-activation-and-scheduled-index-catch-up
plan: "05"
subsystem: storage
tags: [rust, fjall, basic-filters, bounded-append, resource-admission]
requires:
  - phase: 157-03
    provides: TurnWork and immutable envelope ceilings
  - phase: 157-04
    provides: Recovery-minted opaque append proof and sealed own-flush durability
provides:
  - Consuming bounded preparation and atomic consecutive suffix publication
  - Separate maintained processed frontier and exact durable-tip safe release
  - Budgeted proof acquisition with pre-decode operator-map admission
affects: [157-06, 157-07, 157-09, 157-10]
tech-stack:
  added: []
  patterns: [opaque prepared batches, checked pre-work admission, achieved-only publication]
key-files:
  created:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/append.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/fencing.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests/append/admission.rs
  modified:
    - packages/open-bitcoin-node/src/storage/fjall_store/filters.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/ownership.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/filters/tests.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/prune/records.rs
key-decisions:
  - "Safe release waits for the exact sealed durable tip; intermediate processed progress retains the prior conservative checkpoint and lock."
  - "Append requires budgeted proof acquisition; bare recovery/own-flush proof acquisition cannot start a turn."
  - "Work budgets and acquisition costs are private bookkeeping, independent of authority identity comparison."
  - "Operator-map reservations include conservative UTF comparison and logical allocation ceilings before borrowed decoding."
requirements-completed: []
requirements-addressed: [CFIX-01]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 157-2026-10-05T14-50-48
generated_at: 2026-10-05T19:18:39Z
duration: approximately 57min
completed: 2026-10-05
commits: []
git_finalization: pending root whole-phase verification and strict finalization
---

# Phase 157 Plan 05: Bounded Consecutive Append Summary

**Opaque budgeted work publishes only a contiguous suffix, with atomic records/projections and safe release earned at the authentic durable tip.**

## Performance and Scope

- Tasks: 3/3 after the root-approved operator-map admission amendment.
- Ten first-party Rust paths, four new; one summary. New files carry pinned parity breadcrumbs immediately; Plan 10 owns catalog registration.
- Approximate work window: parent Plan 05 handoff around 18:22 UTC through closeout around 19:19 UTC. This is an approximate execution interval, not a measured CPU duration.
- All Cargo runs used Bun 1.3.9 through the command-timings wrapper and ran serially against the shared target. No preserved old dependency cache was enumerated or changed.
- No Git staging/commit/push or shared STATE/ROADMAP/REQUIREMENTS/config mutation occurred. Final native verification and Git finalization remain root-owned.

## Accomplishments

- Bounded preparation checks envelope count/bytes before encoding, validates each candidate's encoding/hash/header and direct predecessor edge, compares an existing immutable row locally, and checks consecutive active projections. Saved hidden rows can be reused only after these ordered local checks. Identical retry performs bounded validation and returns a zero-byte/no-batch outcome.
- The hot path uses direct endpoint, predecessor and candidate reads. It never invokes the full forest validator, genesis projection builder, ancestor-walking record reader, checkpoint scanner, chain-metadata loader or full fence verifier. Those general recovery/read interfaces remain available with complete integrity validation.
- Completion consumes private prepared bytes and rechecks store incarnation, lifecycle generation, branch, prior processed/safe frontier, maintained durable revision and actual H/B under publication. A separate local processed-row/projection probe covers processed progress ahead of safe authority.
- One SyncAll database batch inserts new immutable rows and consecutive projections, explicit safe state and the preserved operator lock map with its reserved entry. Achieved owner construction and all admission happen before commit. The existing finisher invalidates authority before commit; only actual success reinstalls achieved processed/safe facts at the new revision.
- BeforeRecords/BeforeCheckpoint/BeforeProtection faults leave the complete old durable state. AfterCommit reports failure, poisons live work and reopens to the complete achieved new batch. Actual partial coins H and interleaving writers invalidate prepared work without filter effects or unsafe release.
- Budgeted acquisition borrows the native map value, inspects byte length and the u32 count, rejects impossible count/envelope pairs, then reserves parse, copy, allocation and name-comparison work before decoding. Limits and acquisition work travel privately into preparation/completion. The final aggregate fix reserves the second proof check against acquisition work before its decoder runs.
- Ordinary recovery/own-flush acquisition keeps its complete-map behavior and codec diagnostics. General operator CRUD/schema remain compatible. Duplicate-name validation uses a standard-library BTreeSet of borrowed names; parsing still precedes duplicate validation, retaining existing error order.

## Local RED / GREEN / REFACTOR Evidence

| Boundary                        | Observed RED                                                                                                                                         | GREEN / final evidence                                                                                                          |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Initial append contracts        | Three registered tests failed compilation with the missing outcome/prepare/complete API; exit 101.                                                   | Initial 3/3, expanded 16/16, then complete suite below.                                                                         |
| Actual deferred-flush release   | The genuine runtime behavior succeeded, but the first `<20` point-read assertion undercounted acquisition/prepare/completion plus the durable probe. | Correct complete counts and equal 17/129-history controls pass; no real check was removed.                                      |
| Larger validated fixture        | Height 128 exposed the one-byte signed coinbase-height fixture encoding; a subsequent attempted private helper import failed compilation.            | Fixture now serializes the signed script number correctly and accepts all 129 blocks through real validation.                   |
| Pre-decode map admission        | A deliberate temporary mutation bypassed only map-cost admission: the two malformed size/count controls failed; two overflow controls passed.        | Restored four controls passed 4/4 in 1.27s. This is mutation RED evidence, not a fabricated initial baseline.                   |
| Aggregate acquisition + recheck | New behavioral test failed 0/1 because the second map decoder ran before acquisition costs were combined; exit 101.                                  | Wrapper starts with captured acquisition work; preparation consumes the combined result. The same control and final 35/35 pass. |
| Scoped style                    | Clippy found four test `.err().expect()` usages whose successful types support Debug.                                                                | Replaced them with `expect_err`; Clippy style gate passes with the staged normal-lib warnings preserved.                        |

The simplification pass removed the redundant uncounted guarded forwarding wrapper, avoided cloning a decoded candidate solely to compare it, reused the existing publication mutex and finisher, and extracted one checked map-cost reservation helper. No full-history cache, new mutex, dependency, schema or temporary production budget default was introduced.

## Verification

All commands use `--manifest-path packages/Cargo.toml -p open-bitcoin-node` through pinned Bun and the timing wrapper. Counts overlap and are not a distinct-test sum.

| Check                                                             | Result                                                                                                    |
| ----------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| `test --lib phase157_append_ -- --nocapture`, final aggregate fix | **35/35 passed**, 19.47s; 1,173 filtered                                                                  |
| `test --lib phase157_append_lock_map_`, restored admission        | **4/4 passed**, 1.27s                                                                                     |
| `test --lib phase157_proof_`, final closeout                      | **36/36 passed**, 20.28s; 1,172 filtered                                                                  |
| `test --lib filter_index`, after map and compatibility changes    | **116/116 passed**, 67.88s; includes missing suffix/body/undo refusal and indexed-source-loss safe resume |
| `test --lib storage::fjall_store::tests::prune`                   | **39/39 passed**, 25.76s; operator CRUD, reserved maps and real paired unlink controls                    |
| `clippy --all-targets --all-features -- -D clippy::all`           | **Passed**, with seven normal-lib dead-code diagnostic groups still visible; final run 4.21s              |
| `clippy --all-targets --all-features -- -D warnings`              | **Not passed**: staged append/budget roots and their reachable getters await actual Plan 07 consumption   |
| Scoped rustfmt, whitespace/diff review                            | **Passed** across all ten source paths                                                                    |

An initial prune selector `storage::fjall_store::prune::` matched zero tests and is not evidence; the corrected 39-test selector above was run immediately. No warning suppression, fake production call or public visibility widening was used to manufacture a lint pass. The narrow storage-private decoder export is actual bounded-reader integration approved by root.

## Measured Work and Bounds

The two controls assert equal complete work at their different history lengths and emit the values through the test harness. Indexed point reads include BASIC state/owner, row and projection probes; they exclude marker/lock-map keys, which are charged separately in TurnWork.

| Control                                                                         | Encoded output | Record operations | Projection operations | Checkpoint reservation | Copy/allocation reservation | Batch key+value bytes | Actual indexed point reads |
| ------------------------------------------------------------------------------- | -------------: | ----------------: | --------------------: | ---------------------: | --------------------------: | --------------------: | -------------------------: |
| Three new records after 16 versus 256 indexed records                           |            522 |                27 |                    15 |                  5,367 |                       9,054 |                 1,142 |                         21 |
| Zero-record safe release after actual runtime histories of 17 versus 129 blocks |              0 |                29 |                    14 |                  5,367 |                       8,298 |                   152 |                         24 |

For the fixed new suffix with a non-genesis safe/processed endpoint, record operations are `3 × 5 + 3 × candidates + new_rows`, yielding 27 for three new rows. Projection operations are `3 × 2 + candidate_probes + encodes + writes`, yielding 15 for three new projections. Retried/existing projections add their real decode probe; older retry predecessor and separate durable probes remain constant endpoint costs. The inherited Plan 04 16/256 preparation/check control still proves equal nonzero point reads below 20.

With `N` locks and `E` encoded map bytes, the checked map CPU reservation is `L = 1024 × N × (N + 1) / 2 + E + N + 1`. It conservatively covers full bounded-name comparisons, UTF parsing and owned selection scans. Logical allocation reservation is `E + N × (4 × size_of(PruneLockInfo) + 1024)`, covering name bytes, Vec growth and borrowed-name BTree nodes. These are explicit conservative resource ceilings, **not** measured comparison counts, allocator RSS or elapsed time. Successful index point counts, encoded output and batch key/value sizes above are actual counts. Record operations include reads, parsing/commitment validation, predecessor validation and writes. All four marker reads across the two existing H/B classification calls and their decoding/copies are included in each proof check.

Hard envelope ceilings remain exactly 128 candidates, `MAX_SIZE + 128 × 170` aggregate and `MAX_SIZE + 170` singleton. Stored row copies are admitted after a native size probe and before allocation/hash validation. Caller-supplied TurnWork limits apply to acquisition, recheck, candidate encoding/read copies, projection work, completion checks, map encoding and effect-batch storage. Plan 07 must reserve driver body/undo/script/generation work separately and supply its measured remaining budget; test-only injected limits are not production defaults.

## API Handoff and Release Cadence

- `maybe_basic_filter_append_proof_with_budget(maximum_work: TurnWork) -> Result<Option<BasicFilterAppendProof>, StorageError>` is the required turn acquisition. It captures admitted work and limits in private bookkeeping. `maybe_basic_filter_append_proof()` remains the unbudgeted recovery/own-flush interface and cannot start an append.
- `check_basic_filter_append_proof(&proof) -> Result<TurnWork, StorageError>` returns acquisition plus current recheck accounting. The sole private guarded counted checker returns the identity and charges its mutable ledger.
- `prepare_basic_filter_append(proof, &[StoredFilterRecord]) -> Result<PreparedBasicFilterAppend, StorageError>` consumes the proof and validates/owns bounded prepared bytes. `complete_basic_filter_append(prepared) -> Result<BasicFilterAppendOutcome, StorageError>` consumes the preparation under publication.
- Outcome fields are `processed`, `safe_checkpoint`, `work` and `batch_bytes`. An error returns no achieved progress. Work/cost bookkeeping cannot mint resume authority and does not enter identity equality.
- Adapter `blocks` and `encoded_bytes` count the selected candidate set/output once. The driver must merge its body/undo/script/generation costs without double-counting those unique block/output counters, while retaining every additional read, clone and validation operation.
- Safe release advances only to the **exact sealed durable tip**, when its authenticated identity appears in the contiguous suffix or is proved by a bounded row/projection probe after a genuine later own flush. Until then the saved safe checkpoint and covering lock remain conservative, even when processed rows advance. This intentionally retains more source history between releases. Continuous durable growth may prolong retention while indexing lags.
- A zero-record completion can release earned safe progress after durable flush; it cannot publish a caller-selected fence or height. Reopen discards processed hints as restart authority and reconstructs from the fully validated, chainstate-fenced saved checkpoint. Ahead immutable rows remain retained and must be replayed locally in order.
- Legal resource exhaustion is an explicit no-effect storage refusal. Plan 07 must yield/refuse/pause appropriately rather than spin on repeated empty turns. Existing error forms do not authorize automatic history repair or mutation.

## Decisions and Deviations

Local AGENTS guidance, sidecar, placeholder-only overrides, architecture, code-shape, testing, verification, local-guidance and Rust standards informed this work. Both active lesson files were loaded within the budget: 7,188 bytes and 2,397 conservative estimated tokens; no archive was loaded. The inherited audit baseline required no new audit.

- Root approved conservative exact-tip release, preserving full saved-prefix resume and requiring the true ahead/late-flush controls.
- **Rule 2 / checked amendment:** Variable operator-map decode/comparison/allocation was a real hidden cost. Root added Task 3 and two prune paths, retaining operator CRUD/schema and requiring budgeted turn acquisition before decode.
- Root approved two narrow test children to keep every owned file below 628 lines. No production scope was added through those splits.
- **Rule 1 / aggregate admission:** A real RED proved acquisition costs were added too late. The combined ledger now precedes the second decode.
- Fixture/script-number and stale expected-counter corrections are documented above. Remaining ordinary dead-code warnings belong to actual Plan 07 consumption; the two historical constructor-source assertions and Phase 155 Bun anchors remain Plan 10-owned.
- All task/TDD/metadata commits and shared state updates are deliberately deferred to root's strict whole-phase gate.

## Threat Mitigations and Known Limits

T-157-14 is covered by same-store opaque work, exact generation/branch/frontier/durability comparison and complete local candidate validation. T-157-15 is covered by achieved-only identity installation, poison and actual drop/reopen fault controls. T-157-16 is covered by bounded row/projection work, complete operation accounting, pre-decode map admission and budget overflow/refusal controls.

There is no new network endpoint, authentication path, on-disk schema or filesystem trust boundary. The storage-private decoder is the existing map boundary. No goal-blocking stub was found. The production scheduler and measured production limits remain Plan 07; the actual daemon consumer remains later plan work. This summary does not claim normal-lib `-D warnings`, native verification, coverage, Bazel, security/source review or full-phase completion passed.

Software fault seams and real reopen do not simulate hardware power loss. Native Fjall lookup/cache work and fsync latency are not promised as bounded wall time or allocator RSS. Large fixed-prefix histories are codec-valid fixtures; the 17/129 release histories use genuine consensus acceptance with easy local headers and coinbase-only blocks. No public-mainnet, funds or production-readiness claim is made.

## Task Commits and Next Readiness

Tasks 1–3 and summary metadata are pending root Git finalization after clean whole-phase verification. No hashes are claimed. Requirements remain uncompleted here. Plans 06/07 can consume the opaque budgeted append contract; Plan 10 must document exact-tip extra retention, limits and pending source provenance.

## Self-Check: PASSED

All ten owned Rust paths and this summary exist. The three new test children are registered, the four new source files carry pinned breadcrumbs, and owned source lengths stay below 628 lines. The final 35 append controls, 36 inherited proof controls, 116 filter/fencing controls and 39 prune controls have nonzero passing evidence above. Scoped formatting and diff whitespace checks passed. The summary has only its two opening/closing frontmatter delimiters; there is no body delimiter or goal-blocking stub. No unplanned trust surface was added. Commit-existence checks are inapplicable because root explicitly defers all commits. Ordinary staged dead-code diagnostics and whole-phase gates remain accurately pending.
