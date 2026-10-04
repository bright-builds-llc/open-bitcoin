---
phase: 154-basic-generation-and-commitment-parity
slug: basic-generation-and-commitment-parity
status: secured
threats_total: 15
threats_closed: 15
threats_open: 0
asvs_level: 1
block_on: high
generated_by: gsd-secure-phase
lifecycle_mode: yolo
phase_lifecycle_id: 154-2026-10-04T04-36-23
created: 2026-10-04
scope: declared-plan-threats-only
---

# Phase 154 — Security

All 15 declared threats in completed Plans 01–04 are closed by source mitigation evidence or the plans' documented accepted dispositions. Root's full native verifier passed, closing T-154-15. This report verifies the plans' dispositions; lifecycle finalization remains root-owned and this is not a full ASVS assessment.

## Inputs and verification method

Read all four PLAN.md threat models, CONTEXT.md, all four SUMMARY.md files, completed implementation/tests, independent oracle/generator/checker, parity documentation and REVIEW.md. Summaries 03/04 and the Plan 04 helper were read during the delta audit below.

Local AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, architecture, code-shape, testing, verification, Rust and TypeScript/JavaScript standards informed correspondence checks, pure boundaries, pinned tooling and evidence limits. Both active lesson files were read fully: 7,188 bytes, conservative 2,397 tokens. No implementation file was changed and no Cargo, Bazel or commit command was run by this auditor.

Mitigate dispositions were checked against the declared source controls and behavioral test assertions. Accept dispositions were copied from the approved plans into the accepted risks log below; no pending mitigation was converted into acceptance. There are no transfer dispositions.

## Trust boundaries

| Boundary                                    | Data and guarantee                                                                                                                                          |
| ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Caller scripts/counts → encoder             | Public script slices and mapped values; bounded count/range/order/output accounting before writing.                                                         |
| Raw hashes → commitments                    | Public protocol bytes; distinct filter hash/header types and explicit display reversal.                                                                     |
| Predecessor → contextual header             | Explicit genesis or preceding height/parent identity; caller owns validated provenance.                                                                     |
| Retained block/position/undo → script facts | Transaction-body commitment, structural/cardinality correspondence and tagged undo identity; retained coin authenticity remains caller-authority dependent. |
| Actual stage → generator                    | Actual validation-produced undo; historical and same-block facts tested directly.                                                                           |
| Pinned oracle → expectations                | Exact clean upstream pin, independent Python helpers, ten corpus agreements and no-write reproduction.                                                      |
| Fixture/oracle child → test harness         | Public local inputs; bounded child input/output/duration and visible failures.                                                                              |
| Evidence → claims                           | BASIC-only construction/commitments; later runtime and production claims deferred.                                                                          |
| Verifier argv → child                       | Plan 04 must preserve operand boundaries, timing and exact exit status while selecting explicit test files.                                                 |

## Threat register

All source references are repository-relative `file:line`; CLOSED for accepted threats means the declared limitation is documented, not eliminated.

| Threat ID | Category | Component                                    | Disposition | Status | Evidence                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --------- | -------- | -------------------------------------------- | ----------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T-154-01  | D        | codec::block_filter                          | mitigate    | CLOSED | `packages/open-bitcoin-codec/src/block_filter.rs:55` checks MAX_SIZE and checked N\*M; `:68` checks range/order before unary writing; `:105` checks output arithmetic and `:121` uses fallible reservation. Tests `packages/open-bitcoin-codec/src/block_filter/tests.rs:49`, `:57`, `:69`, `:82`, `:95` exercise unsorted/range/count/output/allocation refusal.                                                                                                                                                                                                                                                                              |
| T-154-02  | T        | consensus::block_filter                      | mitigate    | CLOSED | `packages/open-bitcoin-consensus/src/block_filter.rs:32` excludes output-first OP_RETURN, excludes empties, deduplicates raw slices before mapping and retains mapped collisions; `:172` composes raw SHA256d commitments. SipHash at `packages/open-bitcoin-consensus/src/crypto/siphash.rs:46` has pinned-length tests `:149` and independent wrapped-length tests `:227`. Independent byte/hash/header corpus and collision assertions: `packages/open-bitcoin-consensus/tests/basic_filter.rs:53`, `:175`, `:201`, `:283`.                                                                                                                 |
| T-154-03  | S/T      | FilterHeaderPredecessor                      | mitigate    | CLOSED | `packages/open-bitcoin-consensus/src/block_filter.rs:91` guards genesis/height/parent; explicit enum at `:141`. FilterHash/FilterHeader conversion tests at `packages/open-bitcoin-primitives/src/hash.rs:222` and `:237`; refusal tests at `packages/open-bitcoin-consensus/src/block_filter/tests.rs:98`–`:144`, branch commitments at `:162`, independent ordering at `packages/open-bitcoin-consensus/tests/basic_filter.rs:228`.                                                                                                                                                                                                          |
| T-154-04  | I        | Public block-derived SipHash keys            | accept      | CLOSED | Accepted risk AR-154-04 below. Keys are derived from public block hash words at `packages/open-bitcoin-consensus/src/block_filter.rs:54`; no secret key contract is claimed.                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| T-154-05  | S/T      | BasicFilterInputs::from_historical           | mitigate    | CLOSED | `packages/open-bitcoin-chainstate/src/block_filter.rs:40` enforces full header/hash correspondence, genesis/structure rules, Merkle body binding/mutation refusal, tagged undo identity, exact transaction/input counts and missing non-genesis refusal. Tests `packages/open-bitcoin-chainstate/src/block_filter/tests.rs:68`, `:97`, `:116`, `:164`, `:220`, `:326`, `:362`, `:381` cover substitutions, mutations, absence, identity and cardinality. WR-01 is closed on source/regression review; final Rust execution remains root-owned.                                                                                                 |
| T-154-06  | T        | StagedChainstateConnect::basic_filter_inputs | mitigate    | CLOSED | `packages/open-bitcoin-chainstate/src/engine/stage.rs:22` forwards actual staged position/undo to the common constructor. Continuous fixture uses actual validation at `packages/open-bitcoin-chainstate/src/block_filter/tests/fixtures.rs:104` and starts from empty state at `:121`. `tests/validated.rs:79` asserts actual funding/same-block coins and ordered scripts before membership; `:159` proves retained history after current coins disappear.                                                                                                                                                                                   |
| T-154-07  | D        | Script projection                            | mitigate    | CLOSED | `packages/open-bitcoin-chainstate/src/block_filter.rs:86` derives exact undo counts from accepted structure; `:120` borrows scripts; `:129` propagates bounded generator/header errors. Borrowing/order tests at `tests/validated.rs:229`; typed propagation/error tests at `tests.rs:509`. The constructor assumes previously validated block/position and authoritative undo; it is not an arbitrary-size network parser.                                                                                                                                                                                                                    |
| T-154-08  | R        | Stand-alone retained undo authenticity       | accept      | CLOSED | Accepted risk AR-154-08 below; explicit warning at `packages/open-bitcoin-chainstate/src/block_filter.rs:26`. Actual-stage proof is evidence for the trusted path, not authentication of manually fabricated undo.                                                                                                                                                                                                                                                                                                                                                                                                                             |
| T-154-09  | T/R      | Vector generation                            | mitigate    | CLOSED | `scripts/generate-basic-filter-vectors.ts:122` checks exact pin and clean tracked upstream inputs, then requires all ten block/byte/header agreements before rendering; `:148` compares in memory without writing. `scripts/basic-filter-oracle.py:14` imports pinned SipHash/messages helpers, with independent selection/range/Rice/hash logic at `:18`, audited against Knots `blockfilter.cpp:27`, `:187`, `:258`, `:275` and `util/golombrice.h:15`. Checker tests `scripts/check-phase154-basic-filters.test.ts:86`, `:114`, `:139`, `:147`, `:158` reject independence/pin/count/drift/oracle disagreement. Pinned reproduction passed. |
| T-154-10  | D        | Oracle orchestration                         | mitigate    | CLOSED | `scripts/generate-basic-filter-vectors.ts:62` checks 2,000,000-byte child input, maxBuffer and ten-second timeout, propagating errors/nonzero status; `:70` bounds/validates corpus. Frozen collision at `:44`; no default discovery loop, network request or full baseline build. Tests `scripts/check-phase154-basic-filters.test.ts:123`, `:128`, `:134` cover child failure and oversized input.                                                                                                                                                                                                                                           |
| T-154-11  | T        | Parity/README scope                          | mitigate    | CLOSED | `scripts/check-phase154-basic-filters.ts:29` checks exact provenance, source-linked surface, test evidence, executable verifier wiring and scoped claims. Mutation assertions in `scripts/check-phase154-basic-filters.test.ts:94` cover index/RPC/peer/V0/BIP37/GUI/prune/catch-up/production overclaims. `docs/parity/catalog/basic-compact-filters.md:8` and `README.md:53` retain explicit deferred boundaries. Pinned checker and 26 tests passed.                                                                                                                                                                                        |
| T-154-12  | I/E      | Hermetic test scripts                        | accept      | CLOSED | Accepted risk AR-154-12 below; supplied fixture data and checked subprocess arguments are public/local. This does not imply the oracle is an isolation sandbox.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
| T-154-13  | T/E      | run_step argv normalization                  | mitigate    | CLOSED | `scripts/verify/helpers.sh:233` copies quoted argv, `:236` guards exact bun/test, `:239` stops at options, `:240` preserves absolute/explicit paths, `:241` normalizes only existing relative .test.ts files, and `:254` invokes a quoted array without eval/string reconstruction. Plan 04 diff preserves native caller literal contracts; SUMMARY.md and REVIEW.md record preserved operand boundaries, including spaces, flags/values and non-test commands.                                                                                                                                                                                |
| T-154-14  | D        | Bun checkout scan                            | mitigate    | CLOSED | `scripts/verify/helpers.sh:243` prefixes known relative file operands with ./ before execution. `154-04-SUMMARY.md` records actual original bare-path sourced run_step on pinned Bun 1.3.9: 26 passed, recorded 472 ms/status 0/cleared fields. Final REVIEW.md independently repeats the original bare-path helper call: 26 passed, 466 ms/status 0/cleanup confirmed.                                                                                                                                                                                                                                                                        |
| T-154-15  | T/R      | Verification outcomes                        | mitigate    | CLOSED | `scripts/verify/helpers.sh:250`–`:262` preserves timing, captured child status, errexit restoration, timing record, cleanup and exact return. Plan 04 child exit 37 was returned/recorded with restored errexit and cleanup. Root full native verifier passed exit 0 in 31m 19.275s; `/tmp/open-bitcoin-phase154-native.log:8512` confirms completion after benchmark/Bazel/provenance and pure-core coverage gates.                                                                                                                                                                                                                           |

## Accepted risks log

| Risk ID   | Threat ref | Rationale and exact limit                                                                                                                                                                                                                                                                                                                                                                               | Accepted by                         | Date       |
| --------- | ---------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------- | ---------- |
| AR-154-04 | T-154-04   | SipHash keys and commitments are public protocol data. This feature introduces no secret material or logging surface.                                                                                                                                                                                                                                                                                   | Existing 154-01-PLAN.md disposition | 2026-10-04 |
| AR-154-08 | T-154-08   | Shape, tagged identity and transaction Merkle checks cannot authenticate fabricated same-shape restored coins. Caller must retain validated block/position and authoritative undo; actual stage tests prove that path only. Transaction-ID Merkle binding protects the filter-relevant body, without claiming witness authentication or full block revalidation. No untrusted serving surface is added. | Existing 154-02-PLAN.md disposition | 2026-10-04 |
| AR-154-12 | T-154-12   | Test scripts consume public fixture material, use no credentials/runtime listener, do not mutate source datadirs and perform no remote effects. Explicit --write regenerates only its checked-in test fixture; --check does not write it. Local Python/git execution is bounded orchestration rather than a sandbox guarantee.                                                                          | Existing 154-03-PLAN.md disposition | 2026-10-04 |

## Unregistered flags

None across all four summaries. Summaries 01/02 contain `Known Stubs and Threat Surface` rather than `## Threat Flags`; their statements map to registered T-154-01 through T-154-08. The WR-01 body-binding addendum strengthens existing T-154-05 and is resolved in source/tests. Summaries 03/04 contain no `## Threat Flags` or unmapped attack-surface flag; their oracle and verifier boundaries map to T-154-09 through T-154-15. No unsolicited new-threat scan was performed.

## Audit trail — 2026-10-04 initial verification

| Metric                         | Count |
| ------------------------------ | ----- |
| Declared threats classified    | 15    |
| Mitigate dispositions          | 12    |
| Accept dispositions documented | 3     |
| Transfer dispositions          | 0     |
| Closed                         | 12    |
| Open/pending                   | 3     |

Independent auditor execution with `/tmp/open-bitcoin-bun-1.3.9/bun-darwin-aarch64/bun` (Bun 1.3.9): `bun test ./scripts/check-phase154-basic-filters.test.ts` passed 26/26 with 34 assertions, reported 429 ms; generator `--check` passed ten pinned corpus agreements and frozen edge commitments; the Phase 154 checker passed provenance/evidence/scoped claims. These checks establish the declared Plan 03 controls, not the root Rust/native/Bazel/coverage gates.

Prior scoped Rust results are executor evidence in summaries. The summary explicitly distinguishes pre-WR-01 results from pending post-fix execution. REVIEW.md independently confirms WR-01 source/regression closure and clean completed-plan scope; its Plan 04 delta is pending. Root retains full native verification, current-source Rust regressions, 100% pure-core line coverage, Bazel smoke and lifecycle closure. No HIGH finding is being accepted by this report.

Required delta: verify Plan 04 exact argv guard, narrow file selection, quoted arrays/no eval, actual pinned-Bun execution, unchanged child status/timing/errexit/cleanup; incorporate summaries 03/04 and successful root full-verifier evidence before closing T-154-15. Until then return OPEN_THREATS and do not infer phase advancement.

## Security audit — 2026-10-04 Plan 04 delta

Read the completed Plan 04 helper and scoped diff, all of summaries 03/04, the post-WR-01 verification addendum in summary 02, and the final completed-plans-01–04 REVIEW.md. Confirmed guarded argv selection and quoted execution in source, unchanged timing/error/cleanup ordering, preserved literal native caller contracts and no eval. The stop-at-first-option rule deliberately covers the native caller shape; operands following options remain untouched and no general Bun argument parser is claimed.

Plan 04 executor evidence records actual pinned Bun 1.3.9 original bare-path helper execution: 26/26 passing, 34 expectations, reported Bun 460 ms, helper 472 ms/status 0 and cleared current-step fields. Its real `bash -c "exit 37"` child returned 37, recorded 37/6 ms, restored errexit and cleared fields. The independent source reviewer repeated the actual original helper call, reporting 26/26 passing, 466 ms/status 0/cleanup. Source inspection supports these narrowly attributed execution reports; this auditor did not rerun Cargo or the full native verifier.

Summary 02's follow-up now records 28 body-binding/filter tests, all-target/all-feature strict Clippy and 212 chainstate coverage unit tests passing, with historical input module 140/140 and staged module 59/59 lines covered. Summary 03 records eight independent consensus integration tests and strict scoped Clippy passing. These are executor results, not substitutes for the pending global native/coverage/Bazel gate.

| Metric                      | Count                  |
| --------------------------- | ---------------------- |
| Declared threats classified | 15                     |
| Closed                      | 14                     |
| Open                        | 1                      |
| Newly closed                | 2 (T-154-13, T-154-14) |
| Unregistered flags          | 0                      |

Only T-154-15 remains open for required full native verification. All existing accepted risks and their limits remain unchanged. Return OPEN_THREATS until root supplies the successful current-source full-verifier result.

## Security audit — 2026-10-04 full native gate

Root reported `bash scripts/verify.sh` completed with exit 0 using pinned Bun 1.3.9. Read the relevant terminal log evidence from `/tmp/open-bitcoin-phase154-native.log`: line 5200 validates the benchmark report, line 6070 reports a successful Bazel build with 88 actions, line 6647 records the Bazel provenance check passing, line 6945 reports another successful Bazel build with 84 actions, lines 6947–8510 show pure-core LLVM coverage test execution completing, and line 8512 states `verify.sh completed in 31m 19.275s (1879275ms)`.

The root gate completed workspace formatting, strict Clippy, full build/tests/doctests, benchmark validation, Bazel smoke/provenance and pure-core coverage. The verifier runs full coverage in its full mode, and `run_coverage_report` propagates Cargo failure and rejects any `Uncovered Lines:` entry before the completion line. No uncovered-line failure occurred. This auditor inspected the relevant log and existing coverage rejection path; the full execution and exit-status evidence are root-owned.

Combined with the already verified quoted argv guard, actual pinned-Bun file execution, exact child exit 37 propagation, timing and cleanup, this closes T-154-15. All declared threats have their original dispositions; no implementation mitigation was converted into an accepted risk. The fabricated same-shape retained-undo limit, public-key limit, test-only oracle boundary and all deferred runtime/production scope remain unchanged.

| Metric                      | Count        |
| --------------------------- | ------------ |
| Declared threats classified | 15           |
| Closed                      | 15           |
| Open                        | 0            |
| Newly closed                | 1 (T-154-15) |
| Unregistered flags          | 0            |

Return SECURED for the declared Phase 154 threat register. No source changes, Cargo/Bazel commands or commits were performed by this auditor. Subsequent implementation changes require relevant delta verification; root retains lifecycle finalization.
