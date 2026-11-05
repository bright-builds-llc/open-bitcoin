---
phase: 167-candidate-history-validation-rules
plan: "01"
subsystem: consensus
tags: [rust, network-rules, canonical-genesis, subsidy, proof-of-work]
requires:
  - phase: 166
    provides: Sealed canonical genesis proof and exact-work coins owner
provides:
  - Immutable four-selector monetary, difficulty, activation and exception facts
  - Complete selected height-zero consensus parameter binding without fixture defaults
affects: [167-03, 167-04, 167-06, 167-14, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Private selected-chain rule wrapper, read-only exact exception tables]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/chain_spec/rules.rs
    - packages/open-bitcoin-consensus/src/chain_spec/rules/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/chain_spec.rs
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs
key-decisions:
  - "NetworkRules stores only private selected identity; immutable getters prevent contradictory caller-supplied scalar combinations."
  - "Canonical genesis binds height-zero activation facts explicitly; descendant candidate activation remains a separate authority."
patterns-established:
  - "Source-selected facts do not confer ancestry, ordinary support or actual-daemon evidence."
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T17:23:48Z"
commits: false
duration: 9min
completed: 2026-10-21
---

# Phase 167 Plan 01: Immutable selected-network rule contracts Summary

**Private selected-network rules now bind canonical genesis monetary, difficulty and height-zero activation parameters, including regtest halving150 and difficulty interval144.**

Source checkout baseline, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`, independently verified with git rev-parse HEAD. This pre-existing main-repository object identifies the checkout used for execution; commits:false remains in force.

## Files Created/Modified

- Created: `packages/open-bitcoin-consensus/src/chain_spec/rules.rs` — immutable selected rule getters and exact exception tables.
- Created: `packages/open-bitcoin-consensus/src/chain_spec/rules/tests.rs` — independent selected-source behavior and refusal tests.
- Modified: `packages/open-bitcoin-consensus/src/chain_spec.rs` — rule-module registration and public type exports.
- Modified: `packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs` — complete selected height-zero parameter binding.
- Modified: `packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs` — monetary boundaries, fixture refusal and complete mapping checks.

## Completed tasks

1. Define selected rules without granting support: ChainSpec::rules() returns a private-field NetworkRules. All four source tables expose maturity, subsidy, compact PoW limit, spacing, timespan, difficulty flags, default BIP94=false, buried activation heights, exact BIP34 ancestor hashes, three script replacements, two network-scoped BIP30 repeat pairs and recheck height1983702. Public ScriptFlagException exposes borrowed immutable entries and complete replacement flags.
2. Bind canonical genesis monetary and PoW defaults: consensus_params(proof) initializes every ConsensusParams field from selected rules. No struct-default spread remains. BIP34/BIP113 are inactive at height0; Segwit is active at height0 only for default regtest. Existing exact root/work/empty-coins ownership checks remain intact.

Generic fixture defaults remain unchanged. Fixture maturity1, mainnet halving210000 or generic defaults fail binding to the canonical regtest owner. All selectors still refuse ordinary history; Signet still reports missing challenge validation. These are selected source facts and local behavioral evidence, not actual Knots daemon qualification.

## Executed commands and actual outcomes

All commands used the required prefix:

`PATH=/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:$PATH`

Logs are retained under `.local/open-bitcoin-dev/phase167/plan01/`.

| Command after PATH prefix | Actual outcome | Log |
| --- | --- | --- |
| `bun run scripts/command-timings.ts run --key phase167-01-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_network_rules` | RED: 8 discovered, 2 passed/6 behavior failures with deliberately unbound selector scaffold; GREEN: 8/8 passed. | task1-red.log; task1-green.log |
| `bun run scripts/command-timings.ts run --key phase167-01-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_genesis_params` | Initial compile/setup failure E0425 from wrong block_subsidy import; corrected to block::block_subsidy. Actual RED: 4 discovered, 1 passed/3 behavior failures. GREEN: 4/4 passed. | task2-red.log; task2-red-behavior.log; task2-green.log |
| `bun run scripts/command-timings.ts run --key phase167-01-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-chainstate --all-features` | 686 passed: 661 unit/integration tests and 25 doctests, including both new compile-fail privacy examples. | regression.log |
| `bun run scripts/command-timings.ts run --key phase167-01-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-chainstate --all-targets --all-features -- -D warnings` | Initial test-only field_reassign_with_default error; initializer corrected. Final exit0 with no warnings. | clippy.log; clippy-final.log |
| `bun run scripts/command-timings.ts run --key phase167-01-final-focused -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus -p open-bitcoin-chainstate phase167_` | Final owned source: 12 discovered, 12 passed (4 canonical params +8 rules). | final-focused.log |
| `rustfmt --edition 2024 --config skip_children=true --check packages/open-bitcoin-consensus/src/chain_spec.rs packages/open-bitcoin-consensus/src/chain_spec/rules.rs packages/open-bitcoin-consensus/src/chain_spec/rules/tests.rs packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs packages/open-bitcoin-chainstate/src/engine/canonical_genesis/tests.rs` | Exit0; only owned files were formatted. | rustfmt-check-final.log |
| `git diff --check` | Exit0; owned diff and both new files reviewed. | diff-check-final.log |

Zero-test integration binaries under focused filters are recorded but do not constitute focused success; nonzero unit discovery is reported above. The broad regression preceded the final test-only initializer/assertion cleanup; final focused tests and clippy cover that cleanup. Test behavior includes independent all-network literals, selector cross-pairs, wrong-network script exceptions, exact BIP30 pairs, private field refusal, fixture isolation, subsidy149/150/299/300, 64-halving/u32::MAX exhaustion, empty genesis coins and exact work.

The cooperative lock serialized task2 behind task1 when both wrapper processes briefly existed; log task2-red.log records the wait. Subsequent Cargo invocations were started only after the preceding session completed. All resumable jobs were polled within60s. Rustdoc progress/liveness was observed; no jobs were terminated.

## Source and evidence

Pinned Knots HEAD was independently observed as `a9aee730466ac67d35a3c03ee24676be5e045878`. Read current chainparams tables, GetBlockSubsidy, GetBlockScriptFlags/BIP30, difficulty, maturity and coins source; no new dependency or external API research was needed.

Relevant source digests are retained in source-digests.log:

- kernel/chainparams.cpp: `17170008ef7abe50f1fbde9a854e443b7d0b1119780c8a58329538e8ee038485`
- validation.cpp: `5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495`
- consensus/consensus.h: `b87cd3fc5ade6924e15b3f3cb558065b6abc656b4c19683238948c2c71eea862`

AGENTS.md, Bright Builds sidecar/overrides, architecture/code-shape/testing/verification/Rust guidance and both active lesson inputs informed implementation. Active lessons total8856bytes,2953 conservative tokens; loaded completely. No project skills were present.

## Decisions and simplification

One immutable selected identity drives getters instead of storing independently inconsistent scalar fields. Exception flags use a private two-choice representation because existing BitOr is not const; its public getter returns exactly NONE or P2SH|WITNESS. No flag owner, rule calculator, work proof or consensus dependency was duplicated. The simplification pass retained the existing owner/stage boundary and no effectful code entered consensus.

## Deviations from Plan

No scope deviation. Existing Phase166 parameter assertions in the owned tests were corrected to selected regtest/height-zero facts alongside new regressions. The setup import and test initializer fixes are recorded above, separately from behavioral RED. An initial apply_patch batch was rejected for addressing a path twice; no source mutation occurred from that rejected batch, and the subsequent targeted patch succeeded.

## Task commits

`commits:false`: no staging, commits, push, hook bypass or new worktree. The strict root wrapper owns saving after whole-phase acceptance. Shared STATE/ROADMAP/requirements/task metadata were not edited by this executor.

## Known stubs and threat scan

No stubs found in owned production or test changes. Empty exception tables are the exact selected Signet/regtest facts; empty genesis UTXOs are required behavior. No new endpoint, authentication path, filesystem access or schema trust boundary was introduced. Threat mitigations T-167-01-01/02/03 are covered by immutable selected facts, fixture-binding negatives and attributable source/test evidence.

## Remaining phase obligations

The requirements array denotes this plan's bounded contribution, not phase-wide completion. Candidate-history derivation, earned ordinary validation, durable historical execution, parity manifest registration and full native/fresh-coverage/Bazel verification remain their explicit later plan owners. Plan19 must register the two new Rust files in docs/parity/source-breadcrumbs.json. No ordinary-history, wallet, network-default or actual-daemon claim was enabled.

## Summary parser compatibility repair

The initial verify-summary rejection was a parser mismatch: a backtick-formatted rustfmt command ending in .rs was treated as a file, historical command failures were mistaken for the final self-check, and the pinned submodule commit was searched in main-repository history. The original summary and actual rejected output remain in summary-before-parser-repair.md and verify-summary-rejected.json under the existing ignored plan01 log directory. This repair lists real files first, labels the verified checkout baseline separately from task commits and places executed-command history before the final self-check. All original failed-command records and pinned-source evidence are retained.

## Self-Check: PASSED

All five owned Rust files and this summary exist; focused final tests, scoped formatting, clippy and diff checks passed. There are no claimed commits to verify. Shared metadata remains root-owned.

