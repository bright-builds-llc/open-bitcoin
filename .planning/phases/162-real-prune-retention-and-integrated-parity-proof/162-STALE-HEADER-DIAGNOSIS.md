---
status: diagnosed
artifact_role: historical_diagnostic_snapshot
snapshot_of: .planning/debug/phase162-reopened-stale-filter-eligibility.md
snapshot_at: 2026-10-11T19:31:01Z
trigger: "phase162-reopened-stale-filter-eligibility: configured reopened peer disconnects on displaced canonical h1000 getcfilters stop while authenticated RPC stale lookup succeeds"
created: 2026-10-11T19:00:47Z
updated: 2026-10-11T19:09:02Z
---

## Current Focus

hypothesis: Confirmed missing durable trusted-header publication for genuine local acceptance/reorg/coins-flush; cold stale header absence is the exact first failed peer precondition.
test: Authorized short diagnostic test directly measured live Authorized, absent saved snapshot, cold missing stale header with surviving ScriptsValid and Invalid(UnknownOrDisallowedStop); existing trusted-publication control restores cold Authorized.
expecting: Checked production prerequisite must preserve accepted header metadata across local durable settlement; keep peer semantics and frozen post-reopen stale assertions unchanged.
next_action: Return root cause and fix candidates for parent checked plan. Cargo is idle; no production fix or full pressure-corpus rerun authorized/performed.

## Symptoms

expected: Same continuous normal-maturity100 history, reorg999..1002, manual0..20 and automatic21..714 paired prune, all-handle configured reopen; actual peer stale getcfilters997..1000 should match independent exact frames and retain zero live charges.
actual: Authenticated stale RPC h999/h1000 succeeds. Actual general-serving inbound listener handshakes and requests0..3 around19..21 and713..715 match frames/sentinel fences. Request4 stale canonical h1000 stop EOF/disconnects before expected frame; admitted1/rejected0/resource0/timeouts0/latest_event=getcfilters.
errors: Parent reports two full runs repeat same boundary; failed corpora cleaned and Cargo idle. No Plan04 completion summary.
reproduction: Run checked Plan04 retained/pruned reopened configured-peer sequence through request4; do not repeat full584-row corpus before distinguishing instrumentation is verified.
started: Reported during Phase162 Plan04; event time not separately measured by diagnostic agent.

## Eliminated

- hypothesis: Reopen loses the stale stop's ScriptsValid provenance.
  evidence: Authorized diagnostic directly reads ScriptsValid both live and cold, with and without trusted-header publication control.
  timestamp: unrecorded
- hypothesis: Timestamp age or exact target-derived proof age rejects the stale stop, or supplied chain-work scalar determines stale-age proof.
  evidence: Diagnostic live stale proof work4, best work6, best block proof2, spacing600 produce exact age600s and timestamp age100s; live capture Authorized. Recipe source likewise predicts age1200s/time217s. Exact proof derives bits independently from caller-supplied cumulative_work.
  timestamp: unrecorded
- hypothesis: HeaderStore cold hydration corrupts or excludes a correctly persisted stale header.
  evidence: Existing trusted persist_progress control writes saved snapshot including stale header; after dropping all store/runtime/authority handles, cold HeaderStore restores entry/proof and capture Authorized.
  timestamp: unrecorded
- hypothesis: Coins flushing overwrites a prior all-header snapshot with active-only headers.
  evidence: Source proves chainstate flush passes empty header_entries and Fjall sink returns without writing; no snapshot exists in the uncontrolled diagnostic. Missing publication is the actual mechanism.
  timestamp: unrecorded

## Evidence

- timestamp: unrecorded
  checked: Parent handoff, repo AGENTS/sidecar/overrides, active lessons and knowledge-base headings.
  found: Diagnostic ownership only; no source/Cargo/visibility API changes authorized. Active lessons5230+1958=7188bytes,estimated2397tokens,read completely. Relevant architecture/testing/verification/Rust standards already loaded earlier in this session. No prior matching knowledge-base entry.
  implication: Preserve frozen client and stale policy; propose exact small test scope if source/public observations cannot determine runtime values.
- timestamp: unrecorded
  checked: Creation metadata.
  found: Actual filesystem birthtime is 2026-10-11T14:00:47-0500, equivalent to 2026-10-11T19:00:47Z; initial clock read preceded creation. Updated uses measured clock 2026-10-11T19:03:16Z before this evidence update. Individual source-read event times were not measured.
  implication: Do not infer per-event chronology from these source observations.
- timestamp: unrecorded
  checked: node network/runtime_authority/filter_index/branch.rs::validate_network and network filter_serving/policy.rs::validate_filter_request/stale_allowed; pinned Knots net_processing.cpp::BlockRequestAllowed/PrepareBlockFilterRequest.
  found: Actual stop must exist in live HeaderStore before any provenance read. Nonactive stop requires ScriptsValid plus timestamp age and exact proof-equivalent age strictly below 2592000s. Invalid semantic stops disconnect before resource reservation. Knots performs equivalent stop-existence, ScriptsValid, best-header timestamp/proof-age gates.
  implication: RPC row success is not enough to establish peer eligibility; do not weaken age/provenance/ancestry gates.
- timestamp: unrecorded
  checked: Phase162 shared recipe child and lifecycle::extend_and_reorg; header_store/proof.rs.
  found: Every mined header has bits 0x207fffff, target-derived work2; canonical h1000 time101081, replacement h1002 time101298. Exact stale age against best replacement is 1200s and timestamp age217s, safely below30days if both headers exist. Explicit replacement chain_work2000+offset is used by the existing synthetic branch-selection API; exact proof facts independently recompute ancestry from bits and do not trust that supplied work.
  implication: Supplied cumulative work is not a most-work proof, but it does not itself explain this stale refusal. Missing headers/provenance remain distinct hypotheses.
- timestamp: unrecorded
  checked: node chainstate/validation_history.rs::commit_prepared_connect/commit_prepared_reorg; storage validation-history schema and reads.
  found: Genuine accepted identities are durably published as ScriptsValid; disconnected identities are not erased. Stored identity contains hash,parent,height,status but no block header. Existing Phase159 tests cover accepted provenance across disconnect/prune/reopen.
  implication: Retained identity can support RPC lookup but cannot reconstruct stale proof or header ancestry by itself.
- timestamp: unrecorded
  checked: node network/runtime_authority/filter_index/query.rs::query_network.
  found: RPC identity lookup falls back from pending/live header identities to durable validation-history records; existing stored filter record returns Found before consulting validation_provenance or peer stale policy.
  implication: Reported RPC success is consistent with missing stale HeaderStore entries and does not independently prove ScriptsValid or age eligibility.
- timestamp: unrecorded
  checked: node network local-connect lifecycle transaction, reorg_to_branch, chainstate::flush_applying_plan, flush_lifecycle/fjall_sink.rs::persist_header_entries, sync/runtime_state.rs::persist_progress, sync/open_runtime.rs.
  found: Local acceptance and reorg add positions to live peer_manager headers. Managed coins flush intentionally passes an empty header_entries slice; Fjall header sink no-ops empty. Only ordinary sync persist_progress calls the private trusted all-header snapshot producer. Configured reopen seeds recovered active chain then optionally replaces it with saved HeaderStore. Disabled fixture local acceptance never calls sync persist_progress.
  implication: Earlier commentary candidate that coins flush overwrites header metadata with active-only snapshot was disproved: it writes NO header snapshot. The likely defect is missing publication, not overwriting. Absent saved snapshot causes displaced headers to disappear on reopen.
- timestamp: unrecorded
  checked: Phase160 branch tests and Phase158 reorg retention tests.
  found: Phase160 stale-pruned authorization explicitly constructs/seeds the stale HeaderStore and does not cold reopen it. Phase158 tests prove displaced filter-record equality after cold reopen but do not perform stale peer capture. Existing sync runtime-state test explicitly calls persist_progress before checking header snapshot recovery.
  implication: Existing constituent tests leave exactly the integrated local-acceptance-to-cold-stale-peer gap exposed here.
- timestamp: unrecorded
  checked: Parent communication.
  found: Proposed one diagnostic test only in existing packages/open-bitcoin-node/src/sync/runtime_state/tests.rs with real Regtest coinbase-only blocks at maturity100, genuine reorg and no-pressure Always flush/reopen; observations cover live/reopened stop+best proof, durable provenance and public branch capture. Optional private trusted persist_progress control differentiates publication versus hydration. Exact focused Cargo selector phase162_reopened_stale_eligibility_diagnostic proposed through pinned Bun command-timings wrapper; no edit or run yet authorized.
  implication: Do not spend another full584-row corpus until distinguishing runtime evidence is obtained.
- timestamp: unrecorded
  checked: Parent-authorized diagnostic amendment and exclusive Cargo handoff.
  found: Parent explicitly authorized one distinguishing test in existing node sync/runtime_state/tests.rs, no production/client edits, and optional private trusted persist_progress control solely for diagnosis. Executed exactly the targeted selector first through pinned Bun timing wrapper; no full corpus.
  implication: This test edit/run is an explicit scoped amendment to diagnosis-only ownership, not a source-fix authorization.
- timestamp: unrecorded
  checked: Directly executed phase162_reopened_stale_eligibility_diagnostic.
  found: Real Fjall Regtest runtime with maturity100 accepts coinbase-only genesis+old h1, flushes Always, genuinely reorgs to alternate h1+h2, flushes Always, and drops all store/authority/runtime handles before reopening. Live stale entry/proof present, provenance ScriptsValid, best cumulative6/stale4/best proof2, timestamp age100s and exact proof age600s; capture Authorized. Without trusted control saved snapshot=false, cold stale entry=false/proof=None, best proof present, ScriptsValid survives and capture Invalid(UnknownOrDisallowedStop). With trusted control saved snapshot=true/stale_entry=true and cold entry/proof plus capture Authorized survive unchanged ages/provenance.
  implication: Missing stop-header metadata publication is causally isolated; persisted ledger, proof arithmetic, capability, range, resource and header hydration gates are not the defect.
- timestamp: unrecorded
  checked: Actual timing JSON and tool output; scoped rustfmt and diff review/check.
  found: Receipt .local/open-bitcoin-dev/command-timings/phase162-reopened-stale-eligibility-diagnostic/2026-10-11T19-05-33.003Z-740ee024-1b51-4e34-abcd-56cf6c85e6c4.json records started19:05:33.003Z,ended19:05:42.706Z,duration9703ms,exit0. Libtest1 passed,0 failed,1648 filtered,1.59s. Sole source hunk is diagnostic test appended to existing file; scoped rustfmt check and git diff --check pass. Both unique datadirs cleaned after handles dropped. Cargo idle reported to parent.
  implication: This diagnosing agent directly ran the small test; parent-reported integrated failure remains separate evidence. No production fix or integrated serving proof has been verified.

## Resolution

root_cause: The genuine local acceptance/reorg path records accepted positions in the live peer HeaderStore and durably publishes ScriptsValid identities, but regular managed coins/prune settlement supplies no header snapshot and does not call the private trusted-header producer. Cold configured runtime reconstructs active positions only when no saved all-header snapshot exists, so displaced stale stops disappear from HeaderStore. branch.rs::validate_network fails its first stop-entry lookup and returns UnknownOrDisallowedStop before provenance/age; the adapter disconnects InvalidDisconnect. RPC still succeeds using ledger identity fallback and stored immutable row. This is a production local durable-header publication gap exposed by the fixture, not invalid proof work or loss of filter/validation rows.
fix: Not applied. Candidate producer/caller correction is to include existing private trusted all-header publication in ordinary managed durable settlement (local accepted/reorg/coins/prune boundary), preserving complete admitted headers before claiming successful durability and before removing any needed payload. Reuse existing snapshot capability and error propagation; do not call private persist_progress from test fixture as final proof, seed raw HeaderStore/save_header_entries, weaken stop/age/ScriptsValid policy, change recipe cumulative_work, or move peer proof before reopen. Exact lifecycle/failure/no-op placement requires checked prerequisite plan.
verification: Diagnosis directly confirmed by one authorized no-pressure genuine acceptance/reorg/Always-flush/cold-reopen test with controlled existing trusted snapshot publication;1 passed,0 failed,1648 filtered. Scoped test formatting/diff checks passed. Parent's two integrated EOF failures were not rerun by this agent. Production fix and full Phase162/native/integrated proof remain pending.
files_changed: [packages/open-bitcoin-node/src/sync/runtime_state/tests.rs (diagnostic test only, parent-authorized amendment)]

## Direct Diagnostic Command and Captured Output

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun run scripts/command-timings.ts run --key phase162-reopened-stale-eligibility-diagnostic -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib --all-features phase162_reopened_stale_eligibility_diagnostic -- --test-threads=1 --nocapture
```

Exact captured output excerpts (expanded public proof/capture structures are in the tool transcript):

```text
phase162 stale diagnostic saved: trusted_progress=false snapshot=false stale_entry=false
phase162 stale diagnostic saved: trusted_progress=true snapshot=true stale_entry=true
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1648 filtered out; finished in 1.59s
```

| Direct observation | Live, no extra publication | Cold, no extra publication | Cold, trusted-publication control |
| --- | --- | --- | --- |
| Stale h1 header/proof | Present | Absent / None | Present |
| Genuine validation provenance | ScriptsValid | ScriptsValid | ScriptsValid |
| Best exact cumulative work | 6 | 6 | 6 |
| Stale exact cumulative work | 4 | No header proof | 4 |
| Best block proof | 2 | 2 | 2 |
| Timestamp / exact proof age | 100s / 600s | Stop proof unavailable | Same persisted headers, 100s / 600s |
| Public branch capture | Authorized | Invalid(UnknownOrDisallowedStop) | Authorized |

The control's private persist_progress invocation is diagnostic counterfactual evidence only; it is not a proposed fixture setup change or accepted final serving proof. No snapshot/header authority was seeded. The test currently asserts observed broken cold behavior and must become a desired-behavior regression under the checked fix plan rather than remain a permanent approval of failure.

## Minimal Fix Candidates and Regression Scope

1. Preferred boundary: ordinary managed network durable settlement used by both flush_coins and explicit manual prune application, via network/runtime_authority/prune_flush.rs::flush_and_evict_pruned_blocks and existing network/validation_history.rs::persist_validation_header_snapshot. Preserve its private trusted capability, genuine admission checks, storage errors and authority/publication lock order. Determine correct prefix placement so successful durable settlement preserves stale headers and header-publication failure cannot be reported as full success; do not make FlushMode::None/skipped work unexpectedly durable. Existing private error-adapter file may need a narrow adjustment, not a new public API.
2. Alternative: publish accepted header metadata at managed connect/reorg completion. This is correct only if snapshot ordering, accepted-but-persistence-failed semantics, successor headers and performance are addressed; publishing full snapshots for every connect may create avoidable quadratic copying/writes. A reorg-only snapshot is insufficient if later connects leave a stale persisted snapshot that replaces newer active headers during cold open. Check the shared durable producer before choosing.
3. Fixture-only private persist_progress or raw save_header_entries/seed_header_store is not an acceptable remedy: it adds a special persistence step absent from ordinary disabled local durable settlement and masks the reproduced production gap.
4. Future fix ownership should include this diagnostic test file, narrow production producer/caller files selected by checked plan, and an existing flush/fault test file if needed. No changes to frozen Phase162 client modules, recipe, public visibility, dependencies or parity registry are needed merely to diagnose.
5. Convert uncontrolled cold case to Authorized, retain private-control case as ordinary existing-persistence control or simplify it once regression establishes production path. Assert unchanged ScriptsValid and exact branch/proof facts live/cold. Add successful no-op/None controls and header-publication failure ordering if source placement changes lifecycle effects.
6. Preserve Phase159 header-only/NeverConnected and raw legacy fail-closed controls; Phase160 age equality/nonuniform proof/unsupported capability/range/resource precedence controls; Phase158 actual reorg/retained-row/failure-reopen controls; no body/undo or filter row grants ScriptsValid, no request-time header or filter synthesis.
7. After focused producer regression passes, run unchanged Phase162 post-prune/all-handle-reopen stale cfilters/cfheaders/checkpoint socket assertions on same datadir (with instrumented public header/capture preconditions if needed), then complete parent-owned broader client/native/lifecycle gates. No reduced pressure proof replaces required integrated584-row corpus; its rerun waits until the producer fix is checked.

## Producer Trust, Caller Coverage and Existing Fault Controls

- Shared managed durable callers: network_handle.flush_coins -> automatic_prune::flush -> prune_flush::flush_and_evict_pruned_blocks covers ordinary manual-mode Always flush, periodic/critical/selected writes, automatic prune and daemon coins-flush worker shutdown/error settlement. network_handle.flush_applying_prune_plan enters the same helper directly, including authenticated pruneblockchain via context/prune.rs and dispatch/prune.rs. These are the smallest common normal-caller boundary for Plan14. Chainstate's internal persist/flush paths remain separate and must not be silently assumed to capture live network knowledge.
- Preserve serialization: existing network authority -> automatic-prune state (when applicable) -> filter publication -> history/payload guards. Do not reacquire the network authority via a handle while already holding it, and do not nest the trusted snapshot's publication guard under an already-held publication guard. Prefer capturing the genuine live header snapshot under existing authority then publishing through the existing private capability at the selected ordered durable prefix. Header commit must precede successful durable checkpoint reporting, paired unlink and coins fence where required by the chosen prefix design.
- Existing TrustedHeaderSnapshot has private fields and store identity. save_trusted_header_snapshot verifies store identity and, under Complete validation coverage, every header hash/parent/height matches an admitted ledger identity; it atomically SyncAll-writes Headers and BlockIndex snapshots. It does not promote any record to ScriptsValid. Genuine header-only admissions remain KnownOnly/NeverConnected; accepted block receipts alone mint ScriptsValid. Do not substitute raw save_header_entries, filter records, remote declarations, caller chain_work or test seeding for that capability.
- Legacy/empty/unsupported behavior: missing validation-history store is currently a no-op; empty raw header slices currently do not erase prior snapshots; old UnknownLegacy coverage must remain UnknownLegacy and not become complete or ScriptsValid. Existing legacy recovered header knowledge must not be erased by an empty transient state or rejected merely for lacking new accepted-ledger records. Preserve conditional-write/FlushMode::None/readiness/disk refusal and no-new-index-progress behavior; header knowledge does not advance safe BASIC progress, coins B or prune protection.
- Typed failures: retain StorageError backend/corruption/recovery action through the durable prefix rather than translating failures into ordinary absence or generic semantic invalidity. A header publication failure must not be swallowed or reported as successful coins/prune settlement. Existing admission faults are HistoryPublicationFault::BeforeCommit/AfterCommit in validation_history.rs::publish_history_identities; phase159_validation_history_store_header_publication_fault_stays_closed checks failed genuine header admission blocks trusted publication. These hooks do NOT currently inject failure in save_trusted_header_snapshot's batch commit. No dedicated header-snapshot commit failpoint was found. Existing OrderingSink in flush_lifecycle/tests/execute_flush.rs records header-before-unlink-before-coins ordering; UndoFailingSink checks prefix failure prevents coins. Plan14 may need a narrow private test-only snapshot-write fault under the existing fault infrastructure plus typed prefix-order assertions; never claim the existing admission failpoint already exercises snapshot commit failure.
- Alternative connect/reorg publication must cover all accepted local/stored/network paths and honor genuine absorption/persist-failure semantics; normal durable settlement is the smaller shared scope for this specific bug. No private fixture injection, public API addition, policy relaxation or proof-before-reopen is selected.
