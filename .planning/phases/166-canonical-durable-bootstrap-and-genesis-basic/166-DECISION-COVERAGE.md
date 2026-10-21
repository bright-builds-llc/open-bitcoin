---
generated_by: gsd-plan-phase
lifecycle_mode: yolo
phase_lifecycle_id: 166-2026-10-18T22-36-40Z
generated_at: "2026-10-19T23:48:00Z"
---

# Phase166 Decision and Outcome Coverage

Every row is full planned coverage, not implementation completion. Sequential execution reflects shared source/breadcrumb ownership, lifetime storage contracts, shared Cargo target and final-only Git saving.

| Decision | Plans/tasks | Coverage | Evidence boundary |
| --- | --- | --- | --- |
| D-01 |01/T1-T2,02/T2,05/T1-T2,06/T1-T2,10/T1 | Full | Exact selected complete header/body and four independent pinned-source vectors. |
| D-02 |01/T1,02/T1,05/T2,06/T1,07/T1,08/T1,10/T2 | Full | Actual noC/noH coins and B=genesis; exact outpoint lookup before/after reopen; no transaction application. |
| D-03 |01/T1,02/T2,05/T2,07/T2,09/T1-T2,11/T1-T2,12/T1-T2,16/T1-T3 | Full | Earned genesis scope separate from identity; descendant and activation refusal, including missing Signet challenge. |
| D-04 |03/T1,04/T2,15/T2-T3,06/T1-T2,08/T2,10/T2 | Full | Atomic SyncAll genesis; child failure/uncertainty yields no capability; complete proof or explicit refusal. |
| D-05 |03/T1-T2,04/T1-T2,15/T1-T3,05/T1-T2,06/T2,09/T1-T2,10/T1,16/T1-T3 | Full | Same-executable child scratch proof before original backend; parent retains source locks through reaping/recheck/checked cleanup and handoff/last handle; physical preservation. |
| D-06 |01/T1-T2,02/T1-T2,05/T2,06/T2,07/T1-T2,10/T2 | Full | All32bytes recomputed and bound to actual ancestry/network/tip/B; wide numeric vectors remain separate. |
| D-07 |03/T1-T2,04/T1-T2,15/T1-T3,05/T1-T2,06/T1-T2,08/T2,10/T1-T2 | Full | Inconsistent/missing records fail closed; bounded request/result/deadline, kill/reap and checked cleanup; pinned wrapping arithmetic; faults/reopen. |
| D-08 |01/T2,08/T1-T2,09/T1,10/T2,16/T1-T3 | Full | Exact source-derived BASIC/zero predecessor through accepted owner/fencing. |
| D-09 |07/T2,08/T1-T2,09/T1-T2,11/T2,12/T1-T3,13/T2,16/T1-T3 | Full | Configured adapters; Disabled absence; distinct PreserveSaved and existing continuation owner behind167 gate. |
| D-10 |01/T2,03/T2,04/T1-T2,15/T1-T3,06/T1-T2,08/T2,09/T1-T2,10/T1-T2,11/T1-T2,12/T1-T3,13/T1-T2,14/T1-T3,16/T1-T3 | Full | Genuine raw-engine SIGABRT RED and controlled-process GREEN, real bytes/independent controls, honest filesystem setup limits; immutable historical evidence; full native and independent reviews. |

| Roadmap criterion | Requirement | Primary plans/tasks |
| --- | --- | --- |
|1. Initialize/reopen canonical best block; no genesis spendable output | CBOT-02 |02/T1,06/T1-T2,07/T1-T2,09/T1,10/T2 |
|2. Interrupted initialization complete proof or explicit refusal | CBOT-02 |04/T2,15/T1-T3,06/T1-T2,08/T2,10/T1-T2 |
|3. Explicit genesis BASIC and continuation owner without seed/public activation | CBOT-03 |08/T1-T2,09/T1,10/T2; descendants stay behind Phase167 qualification |
|4. Exact checkpoint/all-handle reopen bound to ancestry/network/B | CWOR-04 |01/T2,02/T1-T2,05/T1-T2,06/T2,07/T1,10/T1-T2 |

## Dependency Graph

-166-01 / wave1: needs existing164/165 plus approved166 research; creates packages/open-bitcoin-consensus/src/chain_spec/bootstrap.rs; no human checkpoint.
-166-02 / wave2: needs 166-01; creates packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs; no human checkpoint.
-166-03 / wave3: needs 166-02; creates packages/open-bitcoin-node/src/storage/fjall_store/open_lease.rs; no human checkpoint.
-166-04 / wave4: needs166-03; creates bounded copying/owned cleanup/non-authorizing reader foundation; no human checkpoint. Full former backend inspection goal remains unaccepted until15.
-166-15 / wave5: needs166-04 foundation; creates inspection/protocol.rs and worker.rs, same-executable product/test dispatch, bounded child recovery and checked parent cleanup; no human checkpoint. This completes the preserved04+15 inspection goal before05.
-166-05 / wave6: needs166-15; creates packages/open-bitcoin-node/src/storage/canonical_bootstrap/proof.rs; no human checkpoint.
-166-06 / wave7: needs166-05; creates packages/open-bitcoin-node/src/storage/fjall_store/canonical_genesis.rs; no human checkpoint.
-166-07 / wave8: needs166-06; creates packages/open-bitcoin-node/src/sync/canonical_genesis.rs; no human checkpoint.
-166-08 / wave9: needs166-07; creates packages/open-bitcoin-node/src/sync/canonical_genesis/basic.rs; no human checkpoint.
-166-16 / wave10: needs166-08; creates a sealed-consuming canonical RPC context and unavailable wallet, with direct RPC-library worker registration; no human checkpoint.
-166-09 / wave11: needs166-16; creates packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs; no human checkpoint.
-166-10 / wave12: needs166-09; creates packages/open-bitcoin-node/src/storage/canonical_bootstrap/qualification_tests.rs; no human checkpoint.
-166-11 / wave13: needs166-10; creates scripts/check-phase166-canonical-bootstrap.ts; no human checkpoint.
-166-12 / wave14: needs166-11; creates scripts/verify.sh; no human checkpoint.
-166-13 / wave15: needs166-12; creates docs/parity/v2-6-canonical-durable-bootstrap.md; no human checkpoint.
-166-14 / wave16: needs166-13; creates .planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-VERIFICATION.md; no human checkpoint.

Interface order: sealed facts → earned coins/header root → retained app lease → actual-engine-locked bounded private copy → same-executable raw scratch recovery child → complete result plus exit0 and parent source recheck/private bounds/checked cleanup → pure authority tuple proof → atomic writer/proved opener → recovered managed runtime → existing BASIC owner → sealed-consuming local RPC context without wallet authority → configured adapters → durable corruption/fault/reopen evidence → bounded semantic guards → scope/registration inverses → parity/contributor/task docs → full native/independent code/security/goal review → wrapper normal-hook saving.

## Preserved Boundaries and Owners

No partial decisions, deferred feature implementation, new dependency/crate or user setup. Ordinary candidate history belongs to167; actual pinned-daemon differential168; broader fork/recovery169; canonical pruning170. Existing index continuation owner is fully wired now and consumes descendants only once167 earns validation. Supported-app locking excludes arbitrary direct filesystem/Fjall writers.

Root owns continuous targeted todo progress, shared STATE/ROADMAP/REQUIREMENTS and all final Git operations.13 owns docs/task reconciliation;14 owns generated LOC/full native and independent reviews. Every executor summary says commits: false; only the wrapper records actual normal-hook/commit/push outcomes after clean verified completion.

Revision1 current-guard ownership:11/T2 owns scripts/check-phase164-canonical-identity.test.ts and replaces only obsolete daemon/runtime/operator route mutants with actual canonical-opener/capability-bypass controls; incompatible-store/source-preservation/history controls remain.12/T1 edits the process cases;12/T2 then records stale-row rejection, reviews complete final process source and renews only its changed source-files.ts EDGES, shared test-modules.ts SOURCE_CONTRACTS and affected TEST_OWNERS count, with named166 negative controls.11 explicitly leaves those process rows to12.12/T3 retains exact verifier registration and historical inverse ownership. Historical receipts and unrelated rows remain immutable.

## Revision2: process-isolated inspection | 2026-10-19T23:48:00Z

04 is partial at revision time, not accepted full inspection. The controlled final malformed-engine RED has zero passing/one failing behavior test and two child SIGABRT6 outcomes with unchanged source and retained scratch. Only exact parent-owned0600 receipts authorize cleanup after child completion. Earlier504storage/16focused passes do not close this boundary. APFS OS92 non-UTF8 fixture setup is separate evidence, not a backend result. No04SUMMARY is present at revision time.

The bounded split preserves every decision in full across04+15:04 establishes safe non-authorizing copying/cleanup, disables parent raw recovery and cannot admit original storage;15 must earn process-contained recovery before05. Existing behavioral tests remain mandatory, with no ignored/removed malformed-refusal case.15 has three tasks with5/5/5 owned paths; shared paths stay sequential.14 total paths reflect codec/controller/four existing entry roots/Bazel registration/breadcrumb wiring within the same adapter, not another feature or crate. No parent/process crash/OOM cleanup guarantee or OS crash-dump redaction claim is made.

15/T2 defines the macro and node-library registration so its controller tests run independently;15/T3 owns CLI-library and daemon/CLI binary registration plus the existing inspection behavior handoff. Existing integration harnesses either create raw fixtures or spawn product binaries; a new direct canonical-opener harness must register the macro in its own already-owned root.09 preserves pre-runtime/pre-parser dispatch,11 reviews final main/helper and semantic isolation controls,12 reviews only affected source/test authority rows and counts. Exact old receipts/hashes stay unchanged until their assigned reviewed renewal. Root must independently recheck the revised plan set; previous plan-check hashes describe their historical snapshots only.


## Revision3: sealed canonical RPC context | 2026-10-20

Actual09 product startup reaches proved storage/runtime but the existing ordinary RPC context correctly refuses unsupported history. Add mandatory16 between08 and09 completion;16 has three sequential tasks/eleven distinct paths (one measured test-only split), consuming OpenedCanonicalGenesis by value, deriving its own runtime/handle/store and selected parameters, and keeping every ordinary constructor unchanged. WalletState::Unavailable returns fixed errors without wallet registry or mempool recovery. Effect paths derive from the proved store, not a divergent supplied config path. Direct RPC-library reopen tests register the existing same-executable worker.

Current set:16 plans/36 tasks with chain01→02→03→04→15→05→06→07→08→16→09→10→11→12→13→14.09 is paused with actual partial adapter work and failed receipts preserved; it resumes only after root accepts16.10–14 shift one wave; original decisions and roadmap criteria remain fully covered.16 contributes to all three assigned requirements through the actual context boundary, while ordinary descendants, wallet authority, public activation and actual-daemon differential qualification stay refused/deferred.

Closed foreign/unproved source refusal retains strict physical equality. Factory refusals after an already-created live capability require unchanged logical authority/wallet/mempool/index/prune state and zero application initialization effects; engine-only open/drop/background effects must be classified honestly.09 still owns activation refusal before opener creation and its closed-source physical preservation.11 reviews the new reachable factory/unavailable-wallet/root wiring before narrow current-owner renewal;12 retains final process-source and exact registration/inverse ownership. No old receipt/input identity is broadened or silently refreshed. Full native, independent code/security/goal, LOC and normal-hook final saving remain14/root.
