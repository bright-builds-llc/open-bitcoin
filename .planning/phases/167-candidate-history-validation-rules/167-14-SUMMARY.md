---
phase: 167-candidate-history-validation-rules
plan: "14"
baseline_commit: 1b472fb225a290df5cdb06eeea2dc67c624c648e
subsystem: operator-canonical-history-status
tags: [rust, consensus, cli, rpc, canonical-history, basic-filter, evidence]
requires:
  - phase: 167-13
    provides: accepted sealed canonical history factory and public RPC chain facts
  - phase: 167-48
    provides: accepted local daemon descendant process evidence
provides:
  - Read-only bounded default-Regtest local ordinary implementation classification, separate from generic qualification
  - Offline parsed CLI status for complete retained-body default-Regtest history
  - Actual 301-descendant RPC-to-CLI same-physical-datadir comparison and refusal controls
  - Repo-local Cargo and Bazel status commands for a prepared closed datadir
affects: [167-15, 167-17, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [selected identity separate from local implementation and differential qualification, sealed history opener before status observation]
key-files:
  created:
    - packages/open-bitcoin-cli/src/operator/runtime/support/phase167_tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/chain_spec.rs
    - packages/open-bitcoin-consensus/src/chain_spec/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support/canonical_genesis.rs
    - packages/open-bitcoin-rpc/src/context/canonical_chain/tests.rs
    - packages/open-bitcoin-cli/src/operator/runtime/support.rs
    - packages/open-bitcoin-cli/README.md
key-decisions:
  - "Retain generic require_ordinary_history refusal; only a separately sealed Regtest history opener can expose descendants."
  - "Report actual pinned Knots-daemon differential qualification as false."
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
generated_at: "2026-10-27T20:17:51Z"
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
commits: false
status: canonical-summary-pending-independent-qa-and-root-final14
---

# Phase 167 Plan 14: Bounded operator history status

**The parsed offline CLI now reopens complete default-Regtest retained history through the sealed owner and reports the accepted 301-descendant state. Selected identity, local implementation and actual daemon qualification remain separate.**

This is a seven-owner source contribution pending Root FINAL14 and whole-phase acceptance. The exact [final evidence index](../../../.local/open-bitcoin-dev/phase167/plan14/execution-v1/index.json) is SHA-256 `9234620f8cbcfc07519da27070c7e5f0c853ba8c7d88bd75711866070a1636ac`. No Git staging, commit or push occurred.

## Evidence references

- Accepted [FINAL13](../../../.local/open-bitcoin-dev/phase167/plan13/root-final13-after-lint-v1/final-source-acceptance.json), SHA-256 `bc2cadc5877eb86b2c184c9b4e90be05cad0e9e2d47f753b457bab39b5336468`; accepted [FINAL48](../../../.local/open-bitcoin-dev/phase167/plan48/root-final48-after-wide-oracle-v1/final-source-acceptance.json), SHA-256 `d44365e3e24c045bb50eddc5b9530a53c0805f4c2bed6166cf8a11fbb1f61540`.
- [Amended final before frame](../../../.local/open-bitcoin-dev/phase167/plan14/execution-v1/amended-final-source-index.json), SHA-256 `b4629a57008dff04df00016c3139d23f4687df80fb2bfa49cbd818004fa24620`; [after frame](../../../.local/open-bitcoin-dev/phase167/plan14/execution-v1/amended-final-after-index.json), SHA-256 `5004116d0431e97209c2a037f5f40a57a20b6a87be70caac0a7a213bf72eed5f`.
- [Seven-owner scratch inverse](../../../.local/open-bitcoin-dev/phase167/plan14/execution-v1/amended-scratch-inverse.json), SHA-256 `0057a7f1ae5c65022310da3afe0e29e6411ce13c532ac24a70c7827103345259`.
- [Task 1 final log](../../../.local/open-bitcoin-dev/phase167/plan14/execution-v1/task1-amended-final.log), SHA-256 `b7ffc37ec1876ab4b2e6cd033c6471cd7d9f59bdef88d839fd9cf07b0a6d14a5`; [Task 2 final log](../../../.local/open-bitcoin-dev/phase167/plan14/execution-v1/task2-amended-final.log), SHA-256 `5952ead83d2cf8cc6cbb2dd2c4b311a5f309abc72080eec5cb6e980f2258f3f4`.

## Behavior and source scope

`ChainSpec::validation_scope()` now describes bounded local ordinary default-Regtest implementation separately from immutable selected identity and generic ordinary-history qualification. Mainnet/testnet3 remain missing historical-rule and checkpoint implementation; Signet also lacks challenge validation. The generic `require_ordinary_history()` check still refuses every selector, and no selected profile or fixture parameter grants a canonical owner. Actual pinned Knots-daemon differential qualification remains false.

Offline `sync status` first tries the original four-selector proved-genesis route. On default Regtest only, a failed genesis-only proof may proceed through `reopen_canonical_history_existing`, which proves complete copied and original retained history before the configured runtime consumes it. Wrong selectors, missing/unproved history and a live second owner refuse. JSON and human diagnostics state the observed evidence class, bounded implementation and retained-body reopen bound. The existing live daemon RPC path still owns daemon-connected status. The CLI README gives timing-wrapped Cargo and Bazel commands that require an already prepared, closed physical datadir.

The Plan 14 seven-owner originals were captured before source writes in `before-index.json`: six present whole bodies and one true absence. The amended final before and after frames in `amended-final-source-index.json` and `amended-final-after-index.json` contain 458 present source paths, 201 native inputs and one supplemental native input; all match byte-for-byte across the two original commands. The final scratch inverse in `amended-scratch-inverse.json` restored all seven originals/absence, preserved 451 protected paths, and reapplied all 458 final paths, with zero live source writes. `owned-seven.diff` is the exact seven-owner whole-body diff. Scoped rustfmt and owned diff whitespace checks passed.

Root separately owns three integration files under the checked [CLI compile](167-14-CLI-SUBMITBLOCK-COMPILE-AMENDMENT.md) and [status diagnostic](167-14-STATUS-DIAGNOSTIC-AMENDMENT.md) amendments: `packages/open-bitcoin-cli/src/client.rs`, `packages/open-bitcoin-cli/src/client/tests.rs`, and `packages/open-bitcoin-cli/src/operator/runtime.rs`. The first two close the cross-plan `MethodCall::SubmitBlock` match and test its envelope; Root's closure is `.local/open-bitcoin-dev/phase167/plan14/root-cli-integration-v1/closure.json` (SHA-256 `822f39194d7594d603a3db8c70c0a46ab02494bdef837f57520df22fb7f79e1f`). The third changes only the later observation error wording to “canonical status observation unavailable.” These three are protected external owners in the final Plan 14 frame, not part of the seven-owner inverse.

## Test-first and actual command outcomes

The first consensus test-first run failed compilation because the new support API did not exist; it is setup signal only, with no behavioral RED credit. The first CLI package-filter run was diagnostically stalled in an ignored `target/debug/deps` directory read, then an isolated-target retry stopped at the unrelated `SubmitBlock` match compile error. Neither gets RED or GREEN credit. The compiled isolated CLI library test then accepted 301 actual descendants and failed at the old genesis-only offline status route, returning `chain_compatibility_refused`/`existing_state_unproved` instead of success. This is the genuine behavioral RED, preserved in `task2-red-lib.log` (exit 101; one named failed test; 2455.34 seconds). The final amended implementation passes the same named 301 test.

The two literal original commands ran serially through `scripts/command-timings.ts` with pinned Bun on the isolated Plan 14 Cargo target. Their complete raw logs and numeric exit values are recorded in the final evidence index:

| Command key         | Named passes | Native exit | Final outcome                                                                                                                              |
| ------------------- | -----------: | ----------: | ------------------------------------------------------------------------------------------------------------------------------------------ |
| `phase167-14-task1` |            2 |           0 | Four-selector support facts and fixture mutation separation                                                                                |
| `phase167-14-task2` |            8 |           0 | Parsed 301-descendant RPC/CLI comparison; four-selector genesis; missing, unproved, wrong-selector, locked and neutral-diagnostic controls |

The 301 test compares RPC `getblockchaininfo` accepted height and display-order hash with parsed CLI status on the same canonicalized physical datadir after all writer handles drop. It compares raw-order hash, exact 256-bit work against the paired typed accepted tip, BASIC accepted/processed/safe heights, retained body/count and protection, observed replay class, and unavailable actual-daemon qualification. Negative tests drive both typed offline helper and actual parsed CLI routes, then compare complete pre/post source manifests. Short probes independently passed before the final long command. Zero-discovery auxiliary suites are recorded but grant no pass credit.

## Residual gates

Independent source, log, frame and inverse reviews passed; independent document QA of this canonical Summary and Root FINAL14 remain before downstream source release. Plan 19 owns the new Rust breadcrumb manifest mapping. Plan 20 and Root own default `bash scripts/verify.sh`, coverage, Bazel, security/goal/lifecycle review and normal-hook Git finalization. This contribution does not qualify an actual pinned Knots daemon, activate unattended sync, or begin Phase 168.
