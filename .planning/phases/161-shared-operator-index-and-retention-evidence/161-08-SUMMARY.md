---
phase: 161-shared-operator-index-and-retention-evidence
plan: "08"
subsystem: testing
tags: [basic-filters, operator-evidence, fjall, rpc, cli, privacy]
requires:
  - phase: 161-04
    provides: Coherent bounded BASIC authority
  - phase: 161-05
    provides: RPC and CLI typed forwarding
  - phase: 161-11
    provides: Sticky source-local query failures
  - phase: 161-06
    provides: Shared human/support rows and reason privacy
  - phase: 161-07
    provides: Dashboard consumers and twelve actual terminal-buffer cases
provides:
  - Genuine public daemon and actual operator binary agreement on frozen BASIC facts
  - Nine committed private-producer captures with mandatory fresh comparison and authenticated consumer replay
  - Independent retained-row accounting, scoped legal owner prune/reopen and wallet/body negatives
affects: [161-09, 162]
tech-stack:
  added: []
  patterns: [Frozen existing authority, explicit private producer capture composition, read-only fixture freshness]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/operator_evidence.rs
    - packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs
    - packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status/captures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults/captures.rs
    - packages/open-bitcoin-node/testdata/basic_operator_captures.json
    - .planning/phases/161-shared-operator-index-and-retention-evidence/161-CALLER-EVIDENCE.md
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index.rs
    - packages/open-bitcoin-cli/tests/operator_flows.rs
    - packages/open-bitcoin-cli/tests/operator_flows/rpc_harness.rs
    - packages/open-bitcoin-cli/tests/operator_flows/domain_fixtures.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/operator_status.rs
    - packages/open-bitcoin-node/src/sync/tests/filter_index/catch_up/rpc_faults.rs
    - docs/parity/source-breadcrumbs.json
key-decisions:
  - Keep private producer controls private; compose fresh-checked safe captures with actual authenticated clients instead of adding a production injection API.
  - Compare one frozen authority or immutable response per scenario and exclude unrelated volatile groups.
  - Preserve manual owner-plan, limited serving-window, synthetic genesis and maturity-one limits explicitly.
  - Treat complete capture wall times as conservative envelopes rather than direct lock-hold or performance guarantees.
requirements-completed: []
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 161-2026-10-11T04-48-29
generated_at: "2026-10-11T08:19:15Z"
duration: 37min including evidence and documentation
completed: 2026-10-11
---

# Phase 161 Plan 08: Genuine Operator Caller Evidence Summary

**Actual daemon and operator clients preserve frozen BASIC evidence, while nine fresh-checked private failure captures prove client/export agreement without a production fault-injection surface.**

## Accomplishments

Both planned tasks are complete across thirteen source/fixture/manifest paths plus the caller evidence and this summary. Public daemon tests reuse the actual accepted/Fjall `DaemonFixture`, production configured open/context and authenticated HTTP handler. They cover catch-up/current, earned latch with later lag, processed ahead of durable, flush/reopen, disabled/no-genesis, raw coins-clone invalidation, protected candidate filtering, actual paired prune, same-height reorg, stale retention and unauthorized empty 401 responses.

The CLI harness accepts existing durable store/coins contexts while preserving its old authentication, normalization, dispatch, batch and error behavior. Actual `open-bitcoin-cli` requests plus `open-bitcoin` status JSON/human, dashboard JSON/text and support file commands compare the same authoritative BASIC capture before/after each frozen scenario. All nineteen labels and semantic values agree. A separate 550 MiB display case preserves the 576,716,800-byte soft block/undo target independently of BASIC growth. Stopped and legacy inputs remain unavailable through every operator export; a hostile captured reason preserves safe values while dropping markers and leaving the source unchanged.

Private missing-body/undo, before/after publication, owner-integrity/resource/invalidation and discovered read-backend/integrity scenarios produce nine safe-only committed groups. Exact env=1 and serial generation are explicit. Normal tests freshly execute each genuine producer and compare its exact group to the fixture. Catalog, typed round-trip and 4-KiB bounds reject private/unknown fields or stale data. Consumer replay is clearly named and separated from live public authority proof; six counted network-status calls cover the baseline RPC client and five operator collections for each safe capture. No ignored artifact or nested Cargo is required.

The actual daemon retained-row test independently enumerates 402 immutable query rows after one real same-height replacement: 4,801 payload bytes and 108,115 logical record key/value bytes, including displaced and body-pruned records. Node raw-row retention regressions also pass. Real closed-handle reopen preserves those facts. In the CLI retained fixture, a configured in-window positive body intent accompanies separate old-body paired absence/NotFound/no DurableBlock proof. Actual wallet rescan refuses requested missing height20 and leaves the wallet unchanged.

## Verification

All Cargo work ran serially through the pinned Bun 1.3.9 timing runner; Rust is 1.94.1. Exact commands, timings, case provenance, counters and exclusions are recorded in `161-CALLER-EVIDENCE.md`.

| Check | Actual result |
| --- | --- |
| Genuine missing capture RED | 1 discovered, failed against bootstrap fixture |
| Explicit serial generation | 87 passed; not counted as freshness verification |
| Generation-disabled node freshness/regressions | 88 discovered, 88 passed |
| Final relocated capture checks | 2 discovered, 2 passed |
| Actual CLI consumer selection | 6 discovered, 6 passed; nine internal failure replay cases |
| Final full `operator_flows` | 11 discovered, 11 passed |
| Added complete stopped exports | 1 discovered, 1 passed |
| New daemon/RPC selection | 3 discovered, 3 passed |
| Historical daemon filter/index/auth/maintenance/reopen | 9 discovered, 9 passed |
| Invalid generation mode | Exact env=2 refused; fixture checksum unchanged; subsequent normal selector passed |
| Final node/RPC/CLI all-target/all-feature strict Clippy | Passed |
| Scoped Rustfmt | Eleven touched Rust paths passed final check |
| Parity / managed checks | 1,122 Rust paths; 1,439 managed inputs; zero findings |
| Whitespace, scoped source review, stub/threat scan | Passed |

The non-overlapping final selections cover **111 distinct tests** (88 node + 11 CLI + 3 new daemon + 9 historical daemon). Subset reruns, generation and nine internal replay cases are not additional discovered tests. The prior Plan07's twelve actual TestBackend cases remain inherited presentation proof, not new results attributed to this plan.

Growing-history polling makes exactly two metadata reads and admits 91 bytes per BASIC observation, within eight reads/4 KiB. Sixty-four polls at prefixes16/128 preserve all nine namespaces, owner and cumulative validation/integrity-read counters; BASIC JSON is 1,090/1,097 bytes. Largest complete helper wall times were 90,625/50,625 ns in the generation-disabled run. These include surrounding work and bound contained lock lifetimes conservatively; they are not direct lock-hold measurements or performance guarantees. No peer/index/history/body/undo/lock traversal or filter-payload copy was added.

## Deviations and Simplification

1. **Parent-authorized reachability adjustment:** Existing node private test controls cannot be called across production crate boundaries. The plan was independently revised/checked to permit the two existing node test owners, committed safe capture fixture and small private children. Private fault producer freshness composes with actual consumer replay; no public test API/feature or false live-daemon fault claim was introduced.
2. **Rule 1 — test fixture bug:** Growing continuous acceptance failed `bad-cb-height` at height128 because the old CLI test script-number helper omitted positive sign padding. The owned helper now adds the zero sign byte; successful 401-block acceptance is the regression guard.
3. **Authorized maintainability splits:** CLI replay/privacy and node capture helpers/tests live in focused private children. The managed file-length finding is resolved without exemptions or suppressed checks. Historical producer behavior and visibility remain unchanged.

Other intermediate failures were setup/assertion corrections, not product bugs: source re-export/module paths, wallet detail location, deferred durable response-plan assertions, protected-candidate filtering semantics and borrowed JSON assertion types. Initial timing keys containing `red` that passed are not claimed as failing RED. One final import needed scoped formatting.

The explicit simplification pass extracts actual wallet/body/reorg fixture helpers and one export assertion path; the largest new public test is below the function-size trigger and all touched files remain below the managed size limit. Available evidence stays typed; renderers gain no collector, cache, retries, state inference or mutation. Material guidance included repository AGENTS/sidecar, placeholder-only overrides, architecture/code-shape/verification/testing/operability/Rust standards, finalized context/research/UI and prior API summaries. Both active lesson files were completely loaded (7,188 bytes; 2,397 conservative tokens); no archive, lesson or tracker was edited.

## Threat Mitigations

T-161-08-1 is covered by actual daemon 401/privacy and historical cookie/scope tests. T-161-08-2 is covered by safe-only exact-catalog captures, independent expected categories, fresh comparison, hostile-wire actual exports and unchanged source. T-161-08-3 is covered by real accepted durable history, protected candidate filtering, paired absence, retained/stale rows, closed-handle reopen and independent body/wallet negatives. T-161-08-4 is covered by structural work/counter assertions, bounded serialization and exact consumer capture counts. No new production trust boundary or additional threat flag was introduced.

## Known Stubs and Residual Limits

No implementation stubs prevent this plan's goal. Private backend/fact-limit/invalid-facts controls are explicitly scoped producer evidence; they do not claim a live socket injected a physical backend fault or an oversized production allocation. Complete capture wall-time observations do not upgrade latency or archive-scale guarantees.

The retained fixtures use synthetic genesis, easy valid PoW, P2SH, canonical BIP34 heights and coinbase maturity1. Manual owner-plan deletion at tip400/height20 respects the trailing288 window (last prunable112) and BASIC protection. It bypasses ordinary prune-RPC planning and does not exercise regtest's unchanged automatic prune-after1000. ManualOnly already refuses an out-of-window old body while present; the in-window positive control proves serving is enabled, while actual paired absence and old-body negatives remain separate facts before/after reopen. No same-hash success-to-refusal causal claim is made.

CFPR-02, complete continuous legal prune/reopen/RPC/both-socket integration and milestone closure remain Phase162. CFOP-01 is not marked complete by this summary. D-16 exclusions remain intact: no repair/reindex/download, wallet acceleration, public-network/default activation, production-service/readiness, archive-scale, funds-safety or stronger crash-window claim. Root/Plan09 owns full `bash scripts/verify.sh`, independent source/security review, formal lifecycle closure, trackers and final git actions.

## Task Commits

None under the explicit outer-wrapper contract. This executor staged, committed and pushed nothing, and did not edit STATE, ROADMAP, REQUIREMENTS, configuration, todo or lessons.

## Self-Check: PASSED

All fifteen source/fixture/manifest/evidence/summary paths exist, the committed fixture has exactly nine cases, both new Markdown artifacts have only their two top frontmatter delimiters, and final whitespace checks pass. Source verification is complete with nonzero discovery, strict lint, scoped formatting and registered breadcrumbs. No implementation stub or new production trust boundary was found. Commit checks are inapplicable because finalization belongs to root. Sources are released; no Cargo session remains active.


## Formal Source-Link Reconciliation

The node private capture helper uses a static relative `include_str!` resolving the committed node testdata file; the CLI capture helper uses `concat!(env!("CARGO_MANIFEST_DIR"), ...)`. Plan08 prescribed the manifest-root form for both, so the node include mechanism is a literal implementation deviation. Both compile-time links resolve the same committed fixture and the ordinary fresh-producer comparisons pass; no ignored file, current-working-directory dependency or generation fallback is introduced. Moving the node helper requires updating its relative include, which compilation catches. This mechanism difference does not alter the consumer/provenance behavior or expand the scoped proof.
