---
phase: 159-authenticated-basic-filter-and-index-rpcs
plan: "03"
subsystem: chainstate
tags: [validation-provenance, accepted-receipts, fjall, trusted-headers, failure-boundaries]
requires:
  - phase: 159-01
    provides: Sealed retained-history codec, bounded publication and exclusive recovery
provides:
  - Genuine managed connect and all-replacement reorg history before fallible persistence
  - One bounded unresolved accepted batch with prepare-time refusal before mempool effects
  - Trusted new header admission and same-authority durable header snapshots
  - Genuine acceptance, fault, retained-row and actual progress-persistence evidence
affects: [159-04, 159-05, 159-07, 159-08]
tech-stack:
  added: []
  patterns: [private absorbed receipts, pre-admitted prepared tokens, monotonic history, trusted header snapshots]
key-files:
  created:
    - packages/open-bitcoin-node/src/chainstate/validation_history.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history/tests.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history/tests/fixtures.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history/tests/network.rs
    - packages/open-bitcoin-node/src/chainstate/validation_history/tests/coverage.rs
    - packages/open-bitcoin-node/src/network/validation_history.rs
    - packages/open-bitcoin-node/src/network/runtime_authority/validation_history.rs
    - packages/open-bitcoin-node/src/sync/runtime_state/tests.rs
  modified:
    - packages/open-bitcoin-node/src/chainstate.rs
    - packages/open-bitcoin-node/src/chainstate/fjall_store.rs
    - packages/open-bitcoin-node/src/network.rs
    - packages/open-bitcoin-node/src/network/runtime_authority.rs
    - packages/open-bitcoin-node/src/storage/validation_history.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/validation_history.rs
    - packages/open-bitcoin-node/src/storage/fjall_store/coins.rs
    - packages/open-bitcoin-node/src/sync/runtime_state.rs
key-decisions:
  - Checked identity allocation and occupied-ledger refusal occur in genuine preparation before network preview or mempool effects; commit also rejects stale prepared tokens.
  - Only private receipts sealed immediately after actual absorption mint accepted storage batches; reorg sealing checks all replacement positions and genuine core receipt lineage.
  - Accepted ledger publication precedes BASIC and coins persistence; existing BASIC reorg fencing still runs if history publication fails.
  - Normal trusted header progress preserves complete coverage; raw snapshot writers remain conservative and invalidating.
patterns-established:
  - Live pending ScriptsValid facts take precedence over durable absence and poisoned history publication.
  - Header admission is bounded separately from accepted replacement batches, preserving normal 2000-header messages.
requirements-addressed: [CFRP-01]
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 159-2026-10-09T16-11-49
generated_at: "2026-10-09T17:37:43Z"
duration: approximately 30min active execution including review fixes
completed: 2026-10-09
---

# Phase 159 Plan 03: Genuine Accepted History and Trusted Header Persistence Summary

**Genuine connect/reorg absorption now retains scripts-valid provenance before later failures, while trusted header persistence preserves honest new-store coverage.**

## Performance

- Tasks: 3/3 implemented and scoped verification complete; all commits remain pending root's consolidated gate.
- Source files: 16, comprising eight created and eight modified, including the subsequent foundation-review fixes.
- Duration: approximately 30 active minutes, comprising approximately 24 initial minutes and six review-fix minutes; precise executor-start timestamps were not separately recorded.
- Material guidance: AGENTS.md, AGENTS.bright-builds.md, placeholder-only overrides and architecture/code-shape/testing/verification/Rust standards. Both active lessons were loaded completely: 7,188 bytes, 2,397 conservative estimated tokens; no audit trigger.

## Accomplishments

- Checked bounded identities are copied from actual core staging into private prepared acceptance tokens. Genuine managed absorption seals the receipt immediately afterward; neither a preview, rejected stage, public position, snapshot, payload nor filter row grants acceptance.
- Ordinary sealing checks the actual absorbed position. Reorg sealing checks every returned connected identity against its own pre-admitted stage, plus the genuine core receipt's old endpoint, retained ancestor and new endpoint. The private receipt has no public/raw/test factory, Clone, Default or deserializer.
- A single pending accepted batch stays live on publication error. Further preparation and stale prepared commits refuse before absorption. Successful same-store SyncAll publication releases that batch; later coins/BASIC failures cannot erase the already durable accepted record.
- History publication runs before BASIC/coins effects. Reorg BASIC acceptance/fencing still executes on a history write failure, preserving existing conservative index and coins behavior. Old positive rows remain through equal-height/longer replacement, zero-replacement disconnect, BASIC disable, body/undo/filter removal and actual Fjall reopen.
- A newly validated and admitted header observation earns only known-header provenance under complete coverage. Genuine new admissions use bounded chunks; previously scripts-valid identities retain precedence. Legacy loaded/raw-seeded data remains UnknownLegacy and receives no retrospective promotion.
- The actual DurableSyncRuntime::persist_progress path now publishes a private same-authority trusted header snapshot. Complete-coverage snapshots require authentic matching retained identities for every entry. Raw save_header_entries/save_chainstate_snapshot/save_chain_meta behavior remains invalidating; legacy coverage cannot become complete through the trusted path.

## Verification Evidence

All Cargo commands used pinned Bun through scripts/command-timings.ts and shared target access was serialized. No native full verifier, staging, commits, pushes or hook bypass were performed by this executor.

| Check | Result |
| --- | --- |
| Initial acceptance RED, phase159-history-accept-red | Expected missing acceptance/provenance APIs and pending-manager integration compile errors. |
| Actual progress writer semantic RED, phase159-history-progress-red | Restoring the old raw writer made the real persist_progress test fail: UnknownLegacy instead of NeverConnected. Trusted handoff restored afterward. |
| Actual network prepare semantic RED, phase159-history-network-prepare-red | Removing the prepare-time occupied-ledger guard while retaining commit refusal removed the genuine P2SH spend from the mempool: transaction_count 1 became 0 on a refused block. Guard restored afterward. |
| Initial complete phase159_validation_history, phase159-history-proof | 26 passed, 0 failed, 1,356 filtered: 11 acceptance/network tests, 10 store/runtime tests and five codec tests. |
| Foundation-review RED, phase159-history-coverage-red-reopen | Both WR-01 and WR-02 reproduced complete coverage live and after actual reopen; both tests failed their explicit live/reopen coverage assertions. |
| Final history after review fixes, phase159-history-coverage-green | 29 passed, 0 failed, 1,366 filtered: the original 26 plus three raw-entry coverage controls. |
| Task1 acceptance selector before the additional network regression | 10 passed, 0 failed; final combined run includes all 11 acceptance tests. |
| Task2 store/recovery selector, phase159-history-recovery | 10 passed, 0 failed. |
| Historical phase158 regression | 102 passed, 0 failed, one intentionally ignored explicit accounting/timing test. This broader run preceded the final prepare-token relocation. |
| Final phase158_manager_reorg after prepare-token relocation | 14 passed, 0 failed. |
| Historical phase157_proof | 36 passed, 0 failed. |
| Repeated phase157_proof after WR-01/WR-02 fixes, phase159-history-coverage-proof157 | 36 passed, 0 failed. |
| Final scoped Clippy --lib --tests -- -D warnings | Passed with zero warnings, including production: all 16 previously dormant Plan01 diagnostic groups have real consumers. |
| Initial scoped rustfmt check on all 14 source files, plus all four review-fix source paths | Passed. |
| Bright Builds managed checker | Zero findings; 1,351 tracked source files scanned. Newly created files were also individually checked for size. |
| git diff --check and scoped source diff review | Passed. |

The tests use real managed/core acceptance and genuine replacement spends, actual before/after history publication faults, an actual coins metadata failure, a BASIC publication fault after reorg absorption, rejected/preview stages, missing filter/body/undo, actual handle drop/reopen and the real runtime progress writer. The populated-mempool network regression checks both unresolved ordinary acceptance and a 129-position replacement before any chainstate preview or mempool patch.

The fresh fixture originally encoded genesis coinbase height incorrectly as a one-byte zero push; real staging rejected it with bad-cb-height. The fixture now uses canonical signed script-number encoding, including zero and the sign-padding boundary. This was a fixture correction, not relaxed validation. Transient test import/fault-helper/PolicyTime signature mistakes were fixed before final verification.

## Resource Limits and Durability

- Exactly one unresolved accepted batch, at most 128 genuine identities.
- Each admitted persistent envelope is 70 bytes and each key is 83 bytes; the checked encoded maximum is 128 × 153 = 19,584 bytes. Codec size/bound tests and genuine 128-versus-129 replacement tests pass.
- Allocation/admission happens during preparation; absorption and sealing add no new fallible allocation/publication step before retaining the live receipt.
- Header admission uses the existing 2,000-header message limit and chunks of at most 128 ledger identities. The complete 2,000-header production path is tested; it is not accidentally capped at 128.
- A failed before-commit publication is retained only in process and is not claimed to survive restart. The deterministic injected after-commit failure happens after successful SyncAll but poisons further durable reads until reopen; its live pending accepted fact remains true. No general failed-sync durability claim is made.
- Only bounded pending identities are retained in memory, not a duplicate all-history cache. Ledger recovery does not grant coins, append, filter availability or prune authority.

## Interfaces and Dependent Handoff

- ChainstateStore::maybe_validation_history_store() defaults to None for memory/custom stores; FjallChainstateStore supplies its actual configured store. No new generic RPC bounds were introduced.
- ManagedChainstate::validation_provenance(hash) checks the live pending accepted batch first, then reads the same store's ledger. A matching pending identity remains ScriptsValid even when the disk publication control is poisoned. Memory defaults remain UnknownLegacy.
- ManagedChainstate::has_pending_validation_history() reports the occupied bounded batch. Prepared connect/reorg tokens carry their checked private history reservation, allocated before network mempool preparation or reorg preview. Commit rejects a stale token when another unresolved accepted batch is present.
- AcceptedValidationBatch::from_absorbed accepts only the private-field AcceptedValidationReceipt. Its constructor is not a raw identity authority factory.
- AdmittedValidationHeaders::from_admitted accepts only private-field TrustedHeaderAdmission evidence from the validated header insertion seam. TrustedHeaderSnapshot likewise has private fields; storage checks same-store binding and complete-coverage identity admission before atomic SyncAll header/index snapshot publication.
- Plan04 must use the managed provenance read, not only the durable store read. Known-block resolution should use direct header lookup or retained identity. If both a header entry and durable ledger row are absent after an accepted publication failure, add a narrow pending-identity accessor in Plan04 so the matching live fact remains recognizable; never infer it from active membership or payload presence.
- ReorgFixture's historical genesis is deliberately raw seeded and stays UnknownLegacy. Its later genuinely managed child/replacement accepts earn positive rows. Tests do not promote that legacy genesis.

### Genuine Genesis and Configured Startup Ordering

The new fresh fixture constructs an empty Chainstate with CoinsCache::from_parent(store.coins_view()), a same-store recovered manager and ready lifecycle, then connects genesis through the genuine managed method. It saves the validated body and forces an ordinary coins flush. The ledger is ScriptsValid and complete coverage survives that trusted metadata write and real reopen. Only then does configured BASIC startup run; its initialize-owner/drive-first-turn path publishes the genesis filter before returning the exposed runtime. The test explicitly checks that row after configured open.

A truly empty configured BASIC startup still refuses with validated genesis history required. No unrelated startup genesis factory or retrospective revalidation was added. Plan07's continuous daemon fixture must repeat genuine managed genesis acceptance and flush, rather than raw seed_coins_from_snapshot; raw fixture genesis missing-row classification remains intentionally UnknownLegacy. The new source fixture demonstrates the construction pattern, but remains cfg(test)-only and is not a production dependency for the RPC crate.

## Task Commits

1. Task1: Genuine accepted capture and bounded publication — pending root consolidated commit.
2. Task2: Trusted header admission and conservative recovery — pending root consolidated commit.
3. Task3: Acceptance/failure/replacement/retention evidence — pending root consolidated commit.

No STATE, ROADMAP, REQUIREMENTS, todo, lesson or config updates were made by this executor. Root owns those lifecycle artifacts and final Git operations.

## Deviations from Plan

### Rule2: Preserve trusted progress coverage through the actual runtime writer

- Found during Task2: the real persist_progress method always called generic save_header_entries, which would durably lose complete coverage after every genuine header admission.
- Parent authorized a narrow same-authority handoff in sync/runtime_state.rs and a thin network/runtime_authority.rs method. The substantive implementation lives in new child modules to keep entry files within the 628-line standard.
- Added network/validation_history.rs, network/runtime_authority/validation_history.rs and sync/runtime_state/tests.rs. Actual progress, reopen, normal full-message admission and failure controls pass; the old writer semantic mutation fails as expected.

### Rule2: Refuse history work before the network mempool transaction

- Found during final source review: fallible count/allocation admission only inside commit could return an error before absorption after the network had already prepared its mempool patch.
- Moved reservation and refusal into genuine prepare tokens; retained the stale-token commit guard. Added a real serialized ManagedNetworkHandle regression with one genuinely admitted standard P2SH spend and an oversized genuine replacement.
- The deliberate missing-prepare-guard mutation proves the regression catches mempool removal on a refused block. Final GREEN and Clippy pass. Added chainstate/validation_history/tests/network.rs.

### Rule3: Keep module entry and test files below managed size limits

- Moved existing commit methods and their flush-window helper from chainstate.rs into the owned acceptance module. Split reusable genuine fixture code into chainstate/validation_history/tests/fixtures.rs. No unrelated behavior or broad formatting was changed.
- Current touched entry files remain within 628 lines, including network/runtime_authority.rs at exactly 628. New substantive modules/tests remain below the limit.

All deviations remain within genuine acceptance/header integration, were communicated to the parent, and are pending consolidated commit. No new dependency, endpoint, authentication scheme, production activation or broader operator surface was introduced.

## Known Stubs and Threat Review

No TODO/FIXME placeholders, empty successful substitutions or raw acceptance factories were added. Pending RPC/query/readiness/daemon proof belongs to Plans04–07 and is not claimed by this plan.

T-159-08 is covered by genuine post-absorb receipt sealing and rejected/preview controls; T-159-09 by checked one-batch reservation and actual network refusal; T-159-10 by monotonic all-replacement retention and missing-payload/filter/reopen controls; T-159-11 by explicit live/durable/legacy distinctions. The additive stored-byte boundary was already declared in Plan01 and this plan. No undeclared network/auth/schema trust surface was introduced. Independent full-phase source/security review remains root-owned.

## Next Plan Readiness

Plan04 can now consume genuine live/durable provenance without dormant production warnings. Plan07 must use genuine continuous managed acceptance, and owns the larger actual daemon/prune proof. Plan08 must register the eight new Rust paths listed in key-files.created with their exact existing breadcrumb blocks, incorporate the narrow ownership deviations, and run full native verification, source/security review and lifecycle validation before requirements or commits are finalized.

## Foundation Review Fixes: WR-01 and WR-02

The explicit Plans01–03 review in 159-REVIEW.md found two raw-write coverage gaps. Both are implemented and behaviorally verified here; independent reviewer recheck and final phase approval remain root-owned. No review-report status was overwritten by this executor.

### WR-01: Invalidate before raw snapshot effects

- Before the fix, the existing BeforeUndo fault returned an error after raw coins had changed, while complete coverage remained true both live and after dropping every handle and reopening. The RED test verifies the changed coins tip before collecting both incorrect coverage states.
- seed_coins_from_snapshot now durably invalidates validation coverage under the existing publication guard before invalidating BASIC append state or starting any coins/undo write. That guard is released before nested writers acquire it; no lock recursion or new synchronization primitive was added.
- Failure and successful-seed controls prove UnknownLegacy live/reopen and retention of a separate genuine ScriptsValid row. No raw data promotes acceptance, and no failed raw coins state is claimed to be repaired by this ledger fix.

### WR-02: Authenticate metadata before preserving coverage

- Before the fix, the public FlushPersistSink metadata call accepted a representable unaccepted position and retained complete coverage live/reopen. The regression covers the direct FjallNodeStore path and the FjallChainstateStore forwarding path.
- The shared save_validated_chain_meta writer now calls check_validation_coverage_for_metadata before its first metadata/control effect. Complete coverage survives only when every supplied identity exactly matches an authentic same-store ScriptsValid ledger record, including hash, parent and height. Empty, known-only or otherwise unaccepted metadata cannot use the public sink to preserve complete coverage. Existing unknown coverage is never elevated.
- This uses the explicitly permitted authenticated-ledger option, not a caller-controlled trusted flag, a nominal pending-coins bit, a method-name assertion or a new framework. The existing BASIC receipt/flush logic remains unchanged.
- The actual fjall_sink.rs adapter requires no edit because both public adapter paths reach the guarded shared writer. Genuine managed genesis acceptance plus forced flush still preserves complete coverage; that positive control passed in the final 29-test run. All 36 existing Phase157 receipt/flush/raw-writer controls also passed again.

### Review-Fix Ownership and Evidence

- Additional modified source: packages/open-bitcoin-node/src/storage/fjall_store/coins.rs.
- Existing foundation-owned modifications: storage/fjall_store/validation_history.rs and chainstate/validation_history/tests.rs.
- New source requiring Plan08 breadcrumb registration: packages/open-bitcoin-node/src/chainstate/validation_history/tests/coverage.rs, with exact validation.cpp and txdb.cpp anchors already present.
- Final focused evidence: 29 history tests, 36 Phase157 proof tests, clean scoped Clippy with -D warnings, scoped formatting and diff checks. No Cargo/Bazel jobs overlapped; the slot was explicitly returned to the root after commands completed.
- No Plan04 query, payload_usage, publication ownership, root query glue, chainstate identity accessor or unrelated files were edited. No staging, commits, pushes, hook bypass or root lifecycle-file mutations occurred.
- These are Rule1/Rule2 correctness fixes at existing trust boundaries; no new endpoint, authentication scheme, dependency or undeclared schema surface was introduced.

## User Setup Required

None.

## Self-Check: PASSED

- All eight created Rust files and this summary exist; all eight modified source paths are accounted for.
- Final history evidence is 29 passing tests after review closure, plus 36 repeated Phase157 proof controls and clean scoped Clippy/format/diff checks. The earlier 14 final manager regressions and broader 102/36 historical evidence remain recorded accurately.
- Both deliberate semantic mutations were restored before final GREEN.
- No commit hashes or completed requirements are claimed; Git and complete phase acceptance remain pending root's consolidated gates.

Native-gate follow-up: the existing cfg(test)-owned module was renamed to the canonical runtime_state/tests.rs path so the production panic scanner excludes actual test assertions. Module ownership, all three behavior controls and production logic are unchanged; parity and evidence paths follow the rename.
