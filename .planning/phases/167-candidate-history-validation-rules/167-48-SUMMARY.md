---
phase: 167-candidate-history-validation-rules
plan: "48"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: rpc-daemon-canonical-history
tags: [rust, rpc, daemon, canonical-history, basic-filter, evidence]
requires:
  - phase: 167-13
    provides: Accepted sealed RPC history factory, SubmitBlock method and public chain facts
provides:
  - Configured local Regtest daemon history route before strict genesis-only fallback
  - Authenticated normal-product process evidence for descendants, durable reproof and BASIC
affects: [167-14, 167-15, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Sealed paired runtime, normal Cargo product process, pinned Knots test-only BASIC oracle]
key-files:
  created:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/phase167_tests.rs
    - packages/open-bitcoin-rpc/tests/phase167_canonical_startup.rs
    - packages/open-bitcoin-rpc/tests/phase167_canonical_startup/basic_evidence.rs
  modified:
    - packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs
    - packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs
    - scripts/basic-filter-oracle.py
key-decisions:
  - Keep the accepted Plan 13 shared history test byte-exact and read-only.
  - Register the inspection worker in the integration harness before stopped reproof.
  - Separate genuine one-child old-daemon RED from full final-source 301-block GREEN.
  - Compute opted-in wide txid and Merkle bytes with pinned Knots and require nonvacuous undo evidence.
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
generated_at: "2026-10-27T17:46:38Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: source-core-reviewed-independent-summary-qa-pending
---

# Phase 167 Plan 48: Local daemon history and durable process evidence

**The configured Regtest daemon now reopens earned local history through the accepted Plan 13 sealed factory. Four original commands passed on the V12 final source, including the full 301-descendant process test. The seven-owner scratch inverse and independent CORE review passed. Independent Summary QA and Root FINAL48 remain pending.**

This canonical Summary records the bounded Plan 48 contribution and its reviewed evidence. It does not grant Plan 48 source acceptance, whole Phase 167 completion, Git finalization or Phase 168 start.

## Bounded behavior and source ownership

The daemon selects the sealed history factory for an explicit durable local Regtest trigger before the strict genesis-only fallback. It preserves the original genesis opener signature, default-off external networking and unavailable wallet. The process test launches Cargo's freshly built `CARGO_BIN_EXE_open-bitcoind`, clears inherited environment, uses isolated authenticated loopback and an owned datadir, and stops and reaps the daemon before direct durable observation or restart. The route adds no public arbitrary-runtime constructor, unattended full sync or other-chain history qualification.

The original checked Plan 48 named five owners. Its checked BASIC split added a test-only Rust helper; its later wide-commitment amendment added the existing Python oracle as a seventh owner. The original seven consist of four whole present bodies and three true absences. Six final bodies changed; the accepted Plan 13 shared history test stayed byte-identical and read-only. The fresh V12 final frame has 440 present source paths, 433 protected source paths, 201 native inputs and one separate Byteview input. Its capture claims ordered complete-body evidence, not an atomic snapshot.

| Final source owner                                                             | Bytes | Lines | SHA-256                                                            | Disposition                        |
| ------------------------------------------------------------------------------ | ----: | ----: | ------------------------------------------------------------------ | ---------------------------------- |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs`         |  6158 |   161 | `9569dbdde8a4b9e2b68793f5620033a7432ae0e2c0dbd50eee156290db8ee439` | Changed daemon route               |
| `packages/open-bitcoin-rpc/src/bin/open_bitcoind/phase167_tests.rs`            |  3652 |   111 | `b1739ee41837f7cbae51a6b517c935d00f0d82d987c2b2295691d6bbc5e4fba0` | New daemon tests                   |
| `packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs`                           | 19106 |   553 | `23e092280337d1c44ee8c6842289be5d11f41b25c88be0acab3eb7bbc219d729` | Changed daemon entry               |
| `packages/open-bitcoin-rpc/src/context/canonical_history/tests.rs`             | 20067 |   539 | `95c7f661bf3bd7f67b205627ec92b0c1b72b2b559548c2effac5ac0fa4c1ce87` | Shared Plan 13, unchanged          |
| `packages/open-bitcoin-rpc/tests/phase167_canonical_startup.rs`                | 22799 |   624 | `3b244644643ca74d41e8bd4021a238da49793c1bcaa51b0862849da89d05b63e` | New process test                   |
| `packages/open-bitcoin-rpc/tests/phase167_canonical_startup/basic_evidence.rs` | 12614 |   331 | `4ec5a99de870c5a4d3759561b9e2a8640379ad02995502e2fd58ced563af15d0` | New BASIC helper V12               |
| `scripts/basic-filter-oracle.py`                                               | 13484 |   265 | `c55128bf26ca4c068bab53c1b38f9b544043c22a1634fdd403e89d37beee6c72` | Changed test-only Knots oracle V12 |

The checked [BASIC split](167-48-BASIC-EVIDENCE-TEST-SPLIT-AMENDMENT.md), [inspection-worker recovery](167-48-INSPECTION-WORKER-TDD-RECOVERY-AMENDMENT.md) and [wide-commitment amendment](167-48-WIDE-COMMITMENT-ORACLE-AMENDMENT.md) retain the original one-task/four-command obligation. The accepted FINAL13 receipt is `.local/open-bitcoin-dev/phase167/plan13/root-final13-after-lint-v1/final-source-acceptance.json` (SHA-256 `bc2cadc5877eb86b2c184c9b4e90be05cad0e9e2d47f753b457bab39b5336468`). The fresh original-seven capture is `.local/open-bitcoin-dev/phase167/plan48/root-after-checked-seven-owner-v2/capture.json` (SHA-256 `9f8a1cdb7e0a601bf665c6a5f74cfc5a487e74e0e637372e060a6bdf9893223c`).

## Test-first recovery and independent wide proof

The first process-test attempt stopped at compiler `E0583`, so it earned no behavioral RED. The original old-daemon V7 full-history run and first rewired run both failed at stopped reproof because the integration harness omitted the same-executable inspection worker. Root's [reattribution](../../../.local/open-bitcoin-dev/phase167/plan48/root-harness-reattribution-v1/disposition.json) (SHA-256 `59ff9992c1ac2117d1be1ea806378a1f6aab696eb1c58b65ca09051559a2ae66`) and independent review withdrew their RED and GREEN credit without erasing raw records. After the worker registration passed its reserved-route control, the compiled one-child test against the old daemon accepted a child, passed stopped reproof and failed at the old genesis-only restart on the same datadir. The genuine old-daemon RED review (`f59bb6f97f28514b1d376d9bb0827bcdade20bf0564d3ffe44f88efbbaeef03d`) and independent review (`9ff53a067e7c6f7167f439d9e5b5cf263277491b68606924688ef1fb85b94e3a`) bind that failure. Reapplying the reviewed daemon bodies made the same one-child test pass stopped reproof and restart (`2a25db554132bd62119ec44e02d06beda2995bbaf284b52dec27d82128e76563`; independent `5a66a837d7b250a492cdbae1fecaebc4adb236970ce0f632244482bd7357d9de`).

An earlier V9b 301-descendant process test passed BASIC and native tip/state checks, but was historical after subsequent source edits. V10 fixed one test-only Clippy `needless_as_bytes` warning and its four original commands passed. Independent final-source review then found that the wide block's txid and Merkle root were derived from production functions alone; its V10 CORE HOLD (`72720f9e0b8840832f3ba27eb6f92d166a866904fc4e1cdf051c8057f119889d`) grants no exact-wide-commitment or final-source credit. The V12 correction changed only the BASIC helper and test-only Python oracle relative to V10. The default oracle response schema remains unchanged; the explicit wide 302 and next 303 opt-in uses pinned Knots `CBlock` to compute raw coinbase txid and Merkle bytes from serialized data, then checks the raw header Merkle commitment. The Rust helper compares those independent bytes with the product, derives the Coins outpoint from the Knots txid, checks active and complete Coins absence, requires no coinbase transaction undo, and requires exactly one captured empty overwrite batch. This avoids vacuous undo assertions while retaining the exact 10,001-byte wide script, RPC, BASIC and stopped native evidence.

The Python default-schema, pinned-genesis, wide-output and malformed-opt-in probes, scoped Rustfmt and strict RPC Clippy all exited zero with no warnings or errors (`85b892e3b8309a6e7988d75c3d53d153c59ba56e7968dd4e39e4cb74e5cf356d`; independent `d6272cb1d7b65b88f6e067d016018a31675cc34a05fddd2a36c656e984c3c434`). These are diagnostic checks, not any of the four original commands. Independent V12 seven-owner source/API review found zero actionable findings (`.local/open-bitcoin-dev/phase167/plan48/independent-v12-final-source-code-review-v1/review.json`); it preceded command 2, so command pass credit comes only from the later raw execution.

The full process test runs a bounded, test-owned 304-case batch through the pinned Knots Python oracle: genesis, descendants 1–301, wide block 302 and next block 303. It checks BASIC bytes and header chain, authenticated RPC facts, exact retained raw body, stopped native BASIC tip/state/projection/protection/provenance at 302/303, complete stop/reap/drop, restart and next-block acceptance. Its test-owned Python stdout was fixture-cleaned and is not retained as a standalone 304-row artifact. The compiled test assertions, captured oracle source and raw original command 2 process output substantiate the executed checks; this Summary makes no archived-oracle-output claim.

## V12 final capture, original commands and scratch inverse

Root froze `.local/open-bitcoin-dev/phase167/plan48/root-final-evidence-after-v12-v3/final-capture.json` (326344 bytes; SHA-256 `190cc940576e3c86cdc23c07287065a6fbe8ee994fda77e8543b4c380d197429`) after the V12 correction and cheap checks. Independent capture review (`c3b22fc246a0c849d5e896513cee307452d93201889b75914ada15689daff327`) authenticated all 440 source, 201 native and one supplemental copied and live bodies. The earlier V10 frame and passes remain historical.

All four literal checked commands then ran in order through the repo-owned Bun timing wrapper on the unchanged V12 frame, with serial Cargo use, native exit zero, nine named passing executions, zero failures or warnings, complete raw stdout/stderr and eight exact before/after 440/201/1 frames. Counts overlap across commands and are not nine unique tests. Root's four-command review is `.local/open-bitcoin-dev/phase167/plan48/root-final-evidence-after-v12-v3/four-command-review.json` (SHA-256 `236bce209fec59ec383417a00f74fcc90bb93ede3586f280dd175af68e5f26a6`); independent review (`1a336c944370354cefff1f58a4a4a7e717b23767fed6ad1e60e0711bbb19095b`) passed. Command 2 includes the named full 301-descendant process test on the final V12 source.

| Order | Original key                        | Named passes | Native exit | Evidence                                                                 |
| ----: | ----------------------------------- | -----------: | ----------: | ------------------------------------------------------------------------ |
|     1 | `phase167-13-task2`                 |            2 |           0 | Daemon route and scope unit tests                                        |
|     2 | `phase167-13-task2-process`         |            5 |           0 | Full 301-descendant process, wide commitment, BASIC and focused controls |
|     3 | `phase167-13-task2-genesis`         |            1 |           0 | Four-selector daemon genesis regression                                  |
|     4 | `phase167-13-task2-process-genesis` |            1 |           0 | Four-selector product-process genesis regression                         |

Root's scratch-only inverse completion `.local/open-bitcoin-dev/phase167/plan48/root-final-evidence-after-v12-v3/inverse-execution-v1/completion.json` (SHA-256 `b9d8c8326725cd346eb41f693024ef577f4b26c455c09282bbeb566bcf460ec8`) restored four original whole bodies and three true absences, preserved 433 protected source and 201-plus-one native inputs, then reapplied the six changed V12 bodies while leaving the shared Plan 13 test unchanged. Scratch final-before and final-reapplied frames are byte-identical. The independent inverse review (`d847f44b9dbddd03dd2948c53b9dcc5c1a016e0674ca9e9c687a38ea529bd334`) passed; no live source writes were recorded. The earlier V5 inverse HOLD (`51602d289232b1ade2ab9e0feb419c170c5e71d5e441de88ccd2981d9fe426eb`) has no final inverse credit.

## CORE and residual gates

The actual V12 CORE `.local/open-bitcoin-dev/phase167/plan48/actual-core-after-v12-v2/CORE.json` (28223 bytes; SHA-256 `21f2dafac4643cab4235a3475cce80e301a19261cf2ce16002f60b74fffe9c03`) reconciles FINAL13, the checked amendments, true RED/GREEN, V12 source and oracle correction, four original commands, scratch inverse and historical holds. Independent CORE review `.local/open-bitcoin-dev/phase167/plan48/independent-actual-core-v12-review-v1/review.json` (SHA-256 `eda2097ea9f7a9292728ebb3e8e68c49cdc0a0e67b2b2d2e8dee91cc65ace70f`) authenticated its direct and captured references with zero findings. Root released this canonical Summary stage in `.local/open-bitcoin-dev/phase167/plan48/root-final-evidence-after-v12-v3/summary-stage-release.json` (SHA-256 `7fda862459cadbac500c7eb90daf788d1dcaa415fa4e20e337dbd9688f0a88c6`). These receipts keep `sourceAccepted: false` until independent document QA and Root FINAL48.

Plan 19 must add parity breadcrumb manifest mappings for the three new Rust files. Plans 14–20 and 25, compact C–E, pre-17 full source review, global CVLD-01/02/03/04/06, default `bash scripts/verify.sh`, fresh coverage, Bazel, security/goal/lifecycle reviews and normal-hook commit/push remain separate Phase 167 gates. Stop before Phase 168.

## Self-Check: PENDING DOCUMENT QA

The Summary binds seven exact V12 owners, the four original command outcomes, genuine one-child old RED/new GREEN, scratch original-seven inverse, actual CORE and independent zero-finding CORE review. It does not count compiler/setup failures, withdrawn harness failures, earlier V10 command passes or source-only probes as current full-process proof. Independent Summary QA and Root FINAL48 remain open.
