---
phase: 155-recoverable-index-and-pre-prune-startup-protection
slug: recoverable-index-and-pre-prune-startup-protection
status: secured
threats_total: 17
threats_closed: 17
threats_open: 0
asvs_level: 1
block_on: high
generated_by: gsd-secure-phase
lifecycle_mode: yolo
phase_lifecycle_id: 155-2026-10-04T16-01-08
created: 2026-10-04
scope: declared-plan-threats-only
native_gate: passed
---

# Phase 155 — Security

All seventeen declared threats have verified dispositions. No implementation mitigation gap was found. T-155-15 closed after the default full native gate passed; the timestamped delta below records its exact evidence. Earlier pending-gate audit observations are preserved as historical context. Security closure does not assert production readiness, a full ASVS assessment, or hardware durability certification.

## Inputs and verification method

Read all four PLAN threat models, all four SUMMARY security reviews, current codec/core/storage/startup implementation, focused malformed/recovery/fault tests, Phase 155 native checker and mutation tests, parity owner and catalog documentation. Plan 04 SUMMARY was read in full after it appeared and before finalizing this report. The global SECURITY template and Phase 154 SECURITY frontmatter informed the report structure only.

Local AGENTS.md, AGENTS.bright-builds.md, placeholder standards-overrides.md, standards/index.md, architecture, code-shape, testing, verification, Rust and TypeScript/JavaScript guidance informed pure boundaries, error handling, real reopen evidence and claim limits. Both active lesson files were read completely: 7,188 bytes, conservative 2,397 tokens. The auditor modified only this SECURITY.md, ran no Cargo/Bazel/commit command, and preserved concurrent work.

Every threat was classified before verification. `mitigate` dispositions were checked against the declared source controls and behavioral assertions; `accept` dispositions are recorded verbatim in substance from the approved plans below. No mitigation was converted into acceptance. There are no transfer dispositions. Source references below are repository-relative `file:line`.

## Trust boundaries

| Boundary | Required guarantee |
| --- | --- |
| Persisted BASIC bytes → typed record | Canonical bounded encoding, exact key/envelope correspondence, byte hash and contextual header/predecessor integrity. |
| Saved checkpoint → recovered authority | Full saved projection proof, current recovered coins B and compatible contiguous active metadata; immutable/ahead rows alone do not advance progress. |
| Saved protection → deletion eligibility | Validate the saved pair before reconciliation; preserve stronger protection and compare live intent directly with earliest-required input. |
| Batch result → credited progress | One same-database SyncAll publication; errors propagate and poison the live publisher until actual reopen. |
| Runtime construction → prune resume | Mandatory guard after coins H/B recovery and before intent deletion, cache construction or readiness. |
| Durable evidence → contributor claim | Actual production reopen tests supplement static mutation guards; full native gate remains a separate requirement. |

## Threat register

CLOSED accepted risks mean the declared limitation is documented, not eliminated. Counts include both accepted dispositions.

| Threat ID | Category | Disposition | Status | Source and behavioral evidence |
| --- | --- | --- | --- | --- |
| T-155-01 | D | mitigate | CLOSED | `packages/open-bitcoin-codec/src/block_filter/validation.rs:46` bounds MAX_SIZE and count/minimum bits, streams available bits without N-sized allocation, and checks delta arithmetic. `validation/tests.rs:22`, `:42`, `:72`, `:116` cover malformed count, huge count, unary truncation and overflow. CompactSize supplies the count bound before safe range/minimum-bit products. |
| T-155-02 | T | mitigate | CLOSED | `packages/open-bitcoin-chainstate/src/filter_index.rs:189` checks recomputed hash, unique/consecutive/genesis ancestry and B-tip equality; `filter_index/recovery.rs:97` consumes contiguous projection and recovered hash/parent correspondence. `filter_index/tests.rs:313`, `:407`, `:441`, `:462` prove bad metadata, fork reconciliation and refusal. `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs:104` also rejects same-height saved fence/endpoint disagreement. |
| T-155-03 | E | mitigate | CLOSED | `packages/open-bitcoin-chainstate/src/filter_index.rs:149`, `:236`, `:247`, `:269` encode terminal exhaustion, bounded lock endpoint and direct earliest-required comparison. `filter_index/tests.rs:138`, `:332`, `:347` prove 0/1/max and malformed ranges. |
| T-155-04 | T | accept | CLOSED | Accepted risk AR-155-04 below; `packages/open-bitcoin-node/src/storage/filter_index.rs:59` requires historical generator inputs and explicit predecessor for new rows. `packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs:24` stages/commits actual historical and same-block spends before generating records. Hash integrity remains distinct from authenticated historical provenance. |
| T-155-05 | T/D | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/filter_index.rs:161`, `:193`, `:215` perform bounded version/type/length/key/encoding checks before hashing, then verify commitment. `storage/filter_index/tests.rs:52`, `:85`, `:177`, `:203` cover every truncation, consistent-hash noncanonical bytes, flags/trailing bytes and projection identity. |
| T-155-06 | T | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs:134`, `:209`, `:227` reread current B/full metadata, reject differing immutable fields and atomically batch records/projection/state/full preserved lock map with SyncAll. `filters/tests.rs:135`, `:245`, `:299`, `:350` independently prove reopen, rewind, stale fence/bounds and conflict retention; `filters/tests/faults.rs:239` proves complete checkpoint/protection pairs. |
| T-155-07 | R/T | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs:39`, `:331` propagate errors and require reopen after persistence/fault uncertainty. `filters/tests/faults.rs:239` explicitly rejects poisoned live retry and drops the store before actual reopen; `:162`, `:338`, `:343` cover post-record/post-checkpoint and pre-record boundaries. Production-runtime counterparts are in `packages/open-bitcoin-node/src/sync/tests/filter_index/faults.rs:11`, `:239`. |
| T-155-08 | E | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs:72`, `:116`, `:134` remain crate-internal, with shared per-store publication serialization. `storage/fjall_store/prune/records.rs:47` shares the guard for full lock replacement. Reserved identity is defined in `packages/open-bitcoin-chainstate/src/filter_index.rs:19` and parsed at `:236`. No new public activation, operator bypass or second deletion owner is introduced; serialized coins/metadata ownership is explicit in publication module docs. |
| T-155-09 | I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/filter_index.rs:110` returns BlockIndex/Repair category errors; fixed envelope messages contain no script/filter payload, key value or path. `packages/open-bitcoin-chainstate/src/filter_index.rs:298` and codec validation `:27` format bounded categories. Malformed codec tests `storage/filter_index/tests.rs:52` and production helper `sync/tests/filter_index.rs:191` assert categorized refusal. Existing backend failures remain explicit rather than swallowed. |
| T-155-10 | E | mitigate | CLOSED | `packages/open-bitcoin-node/src/chainstate/flush_lifecycle.rs:235` unconditionally invokes the guard after recovery and before resume/readiness/cache. `storage/fjall_store/filters/startup.rs:19` is its concrete consumer. `sync/tests/filter_index.rs:101` and `sync/tests/filter_index/startup.rs:12` use actual runtime open after full store drop and independently inspect preserved inputs. Constructor anchors remain intact in `sync/open_runtime.rs`. |
| T-155-11 | T | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs:35` parses saved protection, refuses absence/weakness before recovery mutation, and publishes only the conservative reconciliation. Core `filter_index/recovery.rs:55` retains stronger protection. `sync/tests/filter_index/startup.rs:208`, `:232`, `:372`, `:438`, `startup/recovery.rs:130` prove no recreation, durable strengthening and no mutation on unsafe rewind. |
| T-155-12 | T/D | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs:95` checks the live intent directly before publication; reserved endpoints are validated before the generic buffered helper. `sync/tests/filter_index/startup.rs:54`, `:248` prove actual production 0/1/u32::MAX refusal and overflowing lock refusal, preserving payload and intent. Core terminal/boundary tests are `filter_index/tests.rs:138`, `:332`. |
| T-155-13 | R | mitigate | CLOSED | `packages/open-bitcoin-node/src/sync/tests/filter_index.rs:191`, `:214` require index-specific fail_closed/Repair diagnostics and exact body/undo values. `startup.rs:12`, `:165`, `:463`, `:501`, `:573` separately assert live intent, byte-for-byte index snapshot, protection/projection where decodable, unsafe single-mate survival and safe mate deletion. Height-400 sparse metadata excludes generic keep-window refusal as a confounder. |
| T-155-14 | T | mitigate | CLOSED | `packages/open-bitcoin-node/src/sync/tests/filter_index/recovery.rs:24`, `:250`, `:301`, `:374` use actual staged validation, historical/same-block spend undo and alternative branch records with production reopen; assert safe cursor, hidden immutable suffix/fork, payloads and protection. `faults.rs:59`, `:158`, `:239` exercise real partial coins H/B replay, concrete metadata-write failure and production reopen for publication boundaries. The metadata seam is test-only in `storage/fjall_store/coins.rs:41`; no production failure switch exists. |
| T-155-15 | R | mitigate | CLOSED | Unique owner is `docs/parity/index.json:834`; `scripts/check-phase155-filter-index.ts:40` and its tests validate actual source links, production evidence, unconditional guard ordering, dependency/schema boundaries and claim scope. Default executable checker/test steps are `scripts/verify.sh:311`. Default full native gate passed under pinned Bun 1.3.9/Rust 1.94.1: `.local/open-bitcoin-dev/phase155/full-native.log:8749` and timing run `50f46fd9-79f1-4f93-ae14-24fa5f1f1319`, exit 0, completed 2026-10-04T19:19:14.762Z. See delta below. |
| T-155-16 | I | mitigate | CLOSED | `packages/open-bitcoin-node/src/storage/fjall_store/filters/startup.rs:24` adds fixed fail_closed context to category errors; no raw payload interpolation occurs in the new corruption path. `docs/parity/catalog/basic-compact-filters.md:140` documents Category-only/no-dump/no implicit repair or download; checker `scripts/check-phase155-filter-index.ts:130` requires this diagnostic contract and scoped claims. |
| T-155-17 | D | accept | CLOSED | Accepted risk AR-155-17 below. `packages/open-bitcoin-node/src/storage/fjall_store/filters.rs:194` streams each complete row/direct parent edge and projection reference without a full encoded-index cache or repeated per-row ancestry walk. `filters/tests/faults.rs:126` drops/reopens 256 records/projections and asserts exactly `4N - 1` actual key/prefix reads. `docs/parity/catalog/basic-compact-filters.md:136` states the total runtime/memory and archive/latency limitations. |

Within a table cell, shortened test paths such as `validation/tests.rs`, `filter_index/tests.rs`, `filters/tests.rs` and `startup.rs` refer to the immediately preceding crate/module directory in that cell.

## Accepted risks log

| Risk ID | Threat Ref | Rationale and retained limitation | Accepted By | Date |
| --- | --- | --- | --- | --- |
| AR-155-04 | T-155-04 | Plan 01 accepts that self-consistent hashes detect corruption but cannot authenticate fabricated historical provenance. New immutable writes consume Phase 154 complete historical-input generation and explicit predecessor identity; genuine staged spend/branch evidence is present. Saved historical higher fence fields are publication provenance, not current authority or proof of an unavailable historical header chain. Current recovered B, full compatible metadata and the verified contiguous saved projection independently decide recovery. A same-height saved fence/endpoint conflict refuses. | Approved 155-01 PLAN disposition | 2026-10-04 |
| AR-155-17 | T-155-17 | Plan 04 accepts necessary linear retained-history startup work. Direct-edge streaming bounds additional record bytes, not total process memory: durable metadata validation uses its existing ancestry input and an identity set, and single-hash reads may retain ancestor hashes. The 256-record corpus demonstrates read/work shape only. No total runtime/memory cap, archive-scale benchmark, latency guarantee, hardware/power-loss simulation, production readiness or production-funds claim is made. | Approved 155-04 PLAN disposition | 2026-10-04 |

The existing runtime owner must serialize coins/metadata/prune operations. The filter mutex and fence reread are not a CAS guarantee against arbitrary raw concurrent writers. Startup runs before runtime construction exposes such writers. Activation/scheduled catch-up, ordinary prune ownership/CRUD/disable, runtime reorg orchestration, RPC/peer/operator serving and integrated retained-client proof remain assigned to later phases.

## Open item at initial audit (resolved below)

| Threat ID | Expected remaining evidence | Files checked | Next action |
| --- | --- | --- | --- |
| T-155-15 | Passed default full `bash scripts/verify.sh`, including complete required integration tests, Bazel smoke and pure-core coverage; exact successful run and pinned-environment provenance. | 155-04-PLAN.md; all SUMMARY files; scripts/verify.sh; Phase 155 checker/tests; parity root/catalog. | Parent runs the gate and supplies successful exit/log evidence; update this report after checking relevant source deltas. Do not mark complete from static wiring or focused passes alone. SUMMARY 04 reports focused Bun evidence on PATH 1.4.2 rather than repository/CI pin 1.3.9; full gate must resolve environment correspondence. |

No HIGH implementation finding was identified in the declared threat register. This gate-dependent item was not an invented vulnerability or accepted risk. Its initial pending status is preserved above; the full native evidence below now closes it.

## Summary threat flags

SUMMARY 01–04 have no literal `## Threat Flags` section. Their security-review notes map to T-155-01–17: accepted provenance/linear-work limits, runtime serialization, linear-scan correction, same-height fence refusal, direct low-height protection, actual staged spend/fork and software-fault limits. SUMMARY 04's concrete metadata-failure seam maps to T-155-14; its pending full gate and PATH Bun/pin difference map to T-155-15. No unregistered flag is reported.

## Security audit trail

| Audit Date | Threats Total | Closed | Open | Run By |
| --- | --- | --- | --- | --- |
| 2026-10-04 | 17 | 16 | 1 | gsd-security-auditor, State B declared-disposition verification |
| 2026-10-04 | 17 | 16 | 1 | Final SUMMARY 04 delta review; no new implementation gap or unregistered flag |

Auditor observed `bun run scripts/check-phase155-filter-index.ts` exit 0 and `bash -n scripts/verify.sh` exit 0. Rust test assertions were inspected directly; execution results are attributed to executor summaries rather than claimed as auditor-run tests. SUMMARY 01 reports 302 scoped core tests with 100% new core production line coverage; SUMMARY 02 reports the 29-test concrete store/codec matrix and 1,023-read corpus; SUMMARY 03 reports 56 focused node tests, 23 initialize tests and 10 restart tests. SUMMARY 04 reports 63 matching node tests, a final four-test fault rerun after the last fixture refinement, final node Clippy and 53 checker tests/93 assertions. These overlap and are not a distinct-test sum. Full native verification remains pending.

Report review confirmed exactly 17 classified register rows, 16 CLOSED and one OPEN, and only the opening/closing frontmatter delimiters. Scoped whitespace review passed. No implementation file was modified.

## Sign-off

- [x] All 17 threats classified and verified according to disposition.
- [x] Both approved accepted risks documented; no transfer dispositions.
- [x] Source mitigations, actual behavioral assertions and claim boundaries inspected.
- [x] Implementation files remained read-only.
- [x] Final Plan 04 SUMMARY threat flags reviewed.
- [x] T-155-15 full native gate evidence verified.
- [x] `threats_open: 0` confirmed.

Approval: verified 2026-10-04 following the native-gate delta below.

## Security audit delta — 2026-10-04T19:21:53Z

T-155-15 changes from OPEN to CLOSED. The parent reported no implementation source changes after the final source review. This delta changes only the security report; no Cargo/Bazel tests were repeated and no implementation, tracking artifact or commit was modified by the auditor.

The parent ran default `bash scripts/verify.sh` with PATH selecting a task-local checksum-verified Bun 1.3.9. The log independently identifies `bun test v1.3.9 (cf6cdbbb)`, resolves the earlier focused-run Bun 1.4.2 environment caveat, and records the Phase 155 checker mutation suite with 53 passes at line 837 plus live checker success at line 841. Timing metadata independently records `rustc 1.94.1 (e408947bf 2026-03-25)`, `verifyMode: full`, `outcome: success`, `exitStatus: 0`, and no terminating signal.

| Evidence | Observed result |
| --- | --- |
| `.local/open-bitcoin-dev/phase155/full-native.log:8749` | `verify.sh completed in 50m 23.935s (3023935ms)`; successful default verifier completion. |
| `.local/open-bitcoin-dev/command-timings/verify-full/2026-10-04T18-28-50.266Z-50f46fd9-79f1-4f93-ae14-24fa5f1f1319.json` | Run started 2026-10-04T18:28:50.266Z, ended 2026-10-04T19:19:14.762Z, successful exit 0. Wrapper duration 3,024,496ms includes timing-wrapper overhead; it is distinct from the verifier's own 3,023,935ms. |
| `scripts/verify.sh:355` through its final coverage step | Default full contract includes format check, workspace Clippy/build/tests with all features, integration/doctests, benchmark smoke/report, Bazel smoke/provenance and pure-core coverage. Successful exit confirms the mandatory sequence completed. |
| `.local/open-bitcoin-dev/phase155/full-native.log:6271`, `:7144`, `:7146` | Bazel builds completed successfully and build provenance passed. |
| `.local/open-bitcoin-dev/phase155/full-native.log:7147` and `scripts/verify/helpers.sh:269` | Actual pure-core llvm-cov execution appears in the log. `run_coverage_report` refuses uncovered production lines, so completed full exit 0 confirms that gate passed; the ephemeral coverage report is not claimed as a retained artifact. |

| Audit Timestamp | Threats Total | Closed | Open | Run By |
| --- | --- | --- | --- | --- |
| 2026-10-04T19:21:53Z | 17 | 17 | 0 | gsd-security-auditor, report-only T-155-15 native-gate closure |

All seventeen dispositions are now closed, including the two approved accepted risks. Accepted provenance and linear-history work limits, serialization requirements, software-fault limits and all deferred product boundaries remain unchanged. No unregistered flag or HIGH implementation gap was identified.
