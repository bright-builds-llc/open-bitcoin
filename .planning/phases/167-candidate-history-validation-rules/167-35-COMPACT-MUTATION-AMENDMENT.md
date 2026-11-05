# Compact unitB: Plan35 fixture and mutation contracts

This formal gsd-plan-phase candidate adds compact lifecycle unitB as new Plan35, using the selected consolidation and exact11-owner union. Root explicitly authorized planning and contract-first ordering; no source/test/build/guard execution or phase acceptance is released. Lifecycle remains `yolo`, `167-2026-10-21T16-38-23`. All34 existing plans remain whole-byte exact, including accepted Plan17 guard ownership and accepted34's17-source/five-task scope. No active30 plan/source/evidence or root metadata is edited.

## Scope and inventory

The selected source is `.local/open-bitcoin-dev/phase167/compact-lifecycle-design/partition/consolidation.md` and its `consolidated-owner-matrix.json`. ExistingA is accepted34. NewB consolidates C0+C2: coherent witnessed positive fixture construction and the independently tested shared mutation predicate. LaterC/D/E remain mandatory and unplanned here. No original plan task/command, requirement, lifecycle or source gate is removed.

| Task                              | Exact allocation                                                                                        | Files |
| --------------------------------- | ------------------------------------------------------------------------------------------------------- | ----: |
| 1 shared contracts                | consensus block.rs, block/witness.rs, new block/mutation.rs, new block/mutation/tests.rs                |     4 |
| 2 complete pin matrix             | the same block/mutation/tests.rs child                                                                  |     1 |
| 3 network fixture construction    | network lib.rs, new test_block_commitments.rs, compact_download/tests.rs, its completion_cases.rs child |     4 |
| 4 peer/node positive constructors | peer/tests/compact_fixtures.rs, peer/tests/compact_download_fallback_cases.rs, node/network/tests.rs    |     3 |

The union is11, with one test child shared by the first two tasks. Task1 owns its real child registrar and initial tests, avoiding an absent-file compiling stage. The root selected four cohesive tasks; the normal2–3-task preference is an explicit scope warning, not a hidden split or added owner. Every task touches at most4 files and must retain actual attributable tests.

Current inventory becomes **35 plans,72 tasks,28 waves** from34/68/28. Plan35 depends on `[167-30,167-32,167-34]`, wave21. The root authorized a deliberate scheduling gap:30 is currently wave19; proposed resource predecessor36 may later move30 to20. Wave21 remains valid in either case and does not invent a dependency on unplanned36. Root serializes this35 check before any separate36 schedule publication. Current13/14/etc metadata remain exact. Machine-complete current DAG, all owner overlaps and wave validation are in the candidate's `dag.json`; no same-wave owner overlap is permitted.

Relevant current edges remain12→31→32→33→30→13→14→15→16→17 and31→34→17. New incoming edges are30→35,32→35,34→35. Their maximum current predecessor wave is19;21 is an intentional spare wave. LaterC must follow35, D followC and E followD plus actual30; then a separately formalized E→17 dependency/schedule reconciliation is mandatory **before any Plan17 guard code**. Accepted Plan17's current dependencies16/34 do not waive this hold. No future plan number/API/acceptance is assumed.

## Actual API and pin closure

Accepted32 already exposes public `BlockView`, `TransactionView`, `WireBlock`, shared stripped/witness serializers, `block_merkle_root_view`, `transaction_txid_view` and `transaction_wtxid_view`. The complete raw model preserves signed amount/script/witness bytes. B uses these serialization facts without Amount conversion, raw-store authority or new decoder/hash calculation. The current domain and raw views share the same algorithms.

Current30 owns the selected domain normalizer `block::update_uncommitted_block_structures(&mut Block,&CandidateRules)`, its private highest-output witness selector and existing witness implementation. Its resource-policy hold is real: block.rs485 lines/SHA256 `3af3a523ae571690c33bc75871bf49b1b40d213404d8803ce488a5351c177205` and witness.rs407 lines/SHA256 `8a9a7617d7f806a3136c2f54ff49c25a0df13cd66dd35b6410c316c38dd735cd` are **observations only**, not final accepted source freezes. All eight active30 sources/SUMMARY/execution/evidence are excluded from immutable claims. Before35 execution, root must accept30 and the executor must revalidate actual final32/30 APIs, layout and behavior. If the owner closure changes, stop for a checked revision. B does not repair, execute or qualify30.

Proposed public factual APIs live under the already-public consensus `block` module, with thin wrappers in the owned block.rs:

```rust
pub fn is_block_mutated(
    block: &impl BlockView,
    check_witness_root: bool,
) -> Result<bool, CodecError>;
pub fn witness_merkle_root(block: &impl BlockView) -> Result<MerkleRoot, CodecError>;
```

No consensus lib.rs/reexport owner is required. `check_witness_root` is pure selected-rule data, never authority. LaterC derives it from actual held selected context; no generic product constructor accepts it as a seal. The witness-root fact wrapper exposes the existing shared algorithm for both predicate and test fixture construction; it is not a normalization operation. A private witness error discriminant distinguishes `Encoding(CodecError)` from `Invalid(BlockValidationError)`. The ordinary wrapper retains exact semantic reason/result/debug diagnostics and existing encoding mapping; the mutation predicate returns true for semantic malleation but propagates encoding errors as Err. No string recovery, catch-all false/true fallback, full intrinsic validator shortcut or second witness implementation is allowed.

The exact pin is `validation.cpp:4277–4355,4468–4496`:

1. Check exact txid Merkle root and real duplicate-pair mutation first; failure returns true.
1. If empty or the first transaction is not coinbase, **immediately return** whether any transaction has exactly64 stripped bytes. With no64, this returns false and skips witness checks even when witnesses would otherwise fail. An empty body with matching zero root returns false.
1. Only the first-coinbase branch performs selected `CheckWitnessMalleation`. A64-byte first coinbase does not trigger the no-coinbase early return.

This is not CheckBlock: no PoW, transaction/monetary/Coins/size/sigop validation is imported into the predicate. Existing32 `check_submission_wire_block` and private `check_block_view` impose extra checks and cannot implement it. Normal full validation and30 normalization retain their order. An invalid compact candidate is never repaired by inserting a missing reserve before classification.

## Evidence matrix

| Concern                | Required controls                                                                                                                                                                                                                 |
| ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Exact base commitment  | Mismatch; real duplicate pairs/subtrees; odd padding without real duplication; valid complete raw/domain equivalence.                                                                                                             |
| Early return           | Empty+zero root; noncoinbase63/64/65 stripped bytes; later64; no64 plus bad witness returning false; first-coinbase64 following witness branch; Merkle failure before all these branches.                                         |
| Selected witness       | True/false, no commitment, correct/wrong root, one32-byte reserve versus absent/multiple/31/33, highest matching output, trailing commitment bytes, unexpected witness and no-witness success. Input bytes/header/hash unchanged. |
| Encoding error         | Existing encoder Err propagates. A bounded exact-size borrowed view can declare an oversized count rejected by CompactSize before iteration/allocation; no giant vector or raised limit.                                          |
| Ordinary compatibility | Existing witness reason/category/debug fields and30 normalizer tests remain intact; full intrinsic order and non-mutation consensus invalidity stay distinct.                                                                     |
| Coherent fixture       | Complete missing/matched transaction set, reserve and commitment precede txid Merkle root; peer mining then block hash/short-ID selection; exact body/IDs and preserved witness stacks.                                           |
| Integration limits     | Structural completion stays unchanged; no production invocation/classifier, first/later fallback, consumed lifetime, fetch flags, admission, stored-success or scheduler claim before C-E.                                        |

Expected outcomes are pin-derived, not copied from production result getters. Construction may reuse shared hashes while the classification expectations remain independent. Distinguish missing-API compilation RED from executed behavioral RED and retain both honestly. The existing malformed/incomplete fixture bodies must produce actual failed coherence assertions before correction; preserve real RED/GREEN and nonzero final discovery. No actual Knots daemon/differential execution is inferred.

## Fixture and owner feasibility

The network helper is declared `#[cfg(test)]` only in its owned lib.rs. It is crate-visible test construction, not production support. Use the one public witness-root fact algorithm and existing double_sha256 for commitment preimages; no copied hash/Merkle/decoder implementation. Preserve existing witness stacks and negative witness coverage. Derive selector and short IDs only after complete body commitments; peer fixtures re-mine against their existing easy-target check. Pure structural fixtures need no invented PoW claim.

The actual node parent has only `build_block` for one coinbase; the older feasibility text's assumed parent `build_block_with_transactions` is not present. A similarly named real builder exists in an unowned child, which is read-only evidence. Within owned node/network/tests.rs, factor its own complete-body builder from the existing merkle/mine path, include the actual missing transaction before commitments/mining, and retain caller signatures. Node cannot import a dependency's cfg(test) module. Existing arbitrary-root mapping, collision, suppression or incomplete-response tests remain their explicit structural/negative class, not body-validity evidence. Any necessary edit outside these11 owners must trigger checked replanning.

| Owner                         |     Current lines | Normal-format allocation estimate                |
| ----------------------------- | ----------------: | ------------------------------------------------ |
| network lib.rs                |               220 | 222, cfg(test) registrar only                    |
| new test_block_commitments.rs |            absent | at most160, narrow fixture builder/tests         |
| compact_download/tests.rs     |               129 | at most180                                       |
| completion_cases.rs           |               150 | at most210                                       |
| peer compact_fixtures.rs      |               242 | at most310                                       |
| peer fallback cases           |               227 | at most280                                       |
| node/network/tests.rs         |               544 | at most610, factor rather than duplicate builder |
| consensus block.rs            | 485 observed/held | at most525, thin wrappers/registrar              |
| consensus witness.rs          | 407 observed/held | at most550, one fact/diagnostic algorithm        |
| new mutation.rs               |            absent | at most180                                       |
| new mutation/tests.rs         |            absent | at most560, bounded matrix/registrar-owned tests |

These are feasibility budgets, not formatted implementation, compilation or resource success. Every actual final source must format normally and remain at most628. Do not minify, suppress diagnostics, erase witness/tests, pack an extra owner or increase limits. Three new source files begin from actual absence, not empty placeholder snapshots. Preserve source comments and delegate the shared breadcrumb manifest to19.

## Commands and retained gates

Plan35 includes exact runnable cooperative commands for the new predicate matrix, existing ordinary witness tests, pure compact download tests, network compact regressions and node compact regressions. Final commands run format check, strict all-target/all-feature Clippy for consensus/network/node and complete tests for those crates. The plan contains their exact command strings and keys; each filter must discover intended nonzero tests. No build/test command runs during this planning task.

Local AGENTS guidance, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification and Rust standards apply. No dependency/crate, production panic, cap/deadline/timeout policy, reconstruction limit, generic authority owner or qualification flag changes. Existing strict>60 expiry and request/resource bounds stay explicit local policy. Pin batch timer/16/3 compact limits/optimistic scheduler remain deferred. All C-E, final source review17 before guard renewal,18security,19source/docs/breadcrumb reconciliation,20complete native/fresh pure-core coverage/Bazel,25goal and lifecycle/compact/Git gates remain mandatory; no whole P2P/full-chain/production-funds or default-network claim.

## Preservation and inverse

The bounded before capture under `.local/open-bitcoin-dev/phase167/compact-unit-b-plan/plan/` retains all34 original plan bytes/hashes, actual35/amendment absence, all three new-source absences, nonactive owner/API/pin observations, accepted17/34 references and the347173-byte canonical report SHA256 `a79d99b76bc2e6f5a3b9967316de8355438ddd57b2a4fafd2803fd2db7780985` with historical prefixes. Active30 sources are observations excluded from final immutable/acceptance claims. Root live STATE/ROADMAP/todo/config/metrics/requirements/Git and active30 execution remain excluded.

The complete inverse deletes only new35 PLAN and this new amendment, restoring actual absence; all34 original plans/tasks/commands/schedules remain exact. Future source implementation must retain its own complete inverse and true new-file absences. Before candidate freeze require GSD schema/structure, balanced task tags, GFM formatting, exact11-owner union, four task file allocations, valid complete35-plan DAG/no same-wave overlap, unchanged old plans/report/prefixes and no source edits. Only an independent checker appends the canonical verdict. Root owns all shared metadata and release decisions; candidate completion is not B-E completion or source approval.
