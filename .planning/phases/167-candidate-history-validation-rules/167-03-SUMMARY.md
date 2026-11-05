---
phase: 167-candidate-history-validation-rules
plan: "03"
subsystem: consensus
tags: [rust, candidate-history, activation, script-flags, bip30]
requires:
  - phase: 167-01
    provides: Immutable selected NetworkRules and exact exception tables
provides:
  - Checked structural candidate branch windows with explicit independent seed classification
  - Shared selected activation, version, finality, script and duplicate-admission rules
affects: [167-04, 167-05, 167-06, 167-07, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Private structural context, Explicit conditional seed classification, Shared rule result]
key-files:
  created:
    - packages/open-bitcoin-consensus/src/candidate_history.rs
    - packages/open-bitcoin-consensus/src/candidate_history/tests.rs
    - packages/open-bitcoin-consensus/src/candidate_history/rules.rs
    - packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs
  modified:
    - packages/open-bitcoin-consensus/src/lib.rs
key-decisions:
  - "GenesisLinked and CheckedSuffix describe structural coverage only; network and coins owners must independently establish earned lineage."
  - "A distinct check_seeded constructor preserves detached BIP34 anchors as IndependentlySeeded facts through CandidateRules; canonical owners must reject that class."
  - "One selected rule result supplies complete existing ConsensusParams carriers without turning those mutable copies into authority."
patterns-established:
  - "Unknown required ancestry errors; before-activation absence, no selected commitment and unconditional recheck are explicit separate states."
requirements-completed: [CVLD-02, CVLD-03, CVLD-04, CVLD-06]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T18:11:06Z"
commits: false
duration: 12min
completed: 2026-10-21
---

# Phase 167 Plan 03: Candidate branch context and derived rules Summary

**Checked candidate ancestry now derives exact selected versions, BIP113/BIP68 context, Segwit commitment activation, script flags and BIP30 decisions while preserving structural and independently seeded evidence classes.**

Source checkout baseline, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`, observed with git rev-parse HEAD. This existing main-repository object identifies the source checkout; commits:false remains in force.

## Files Created/Modified

- Modified: `packages/open-bitcoin-consensus/src/lib.rs` — register and export the shared context/rule contract.
- Created: `packages/open-bitcoin-consensus/src/candidate_history.rs` — checked rows, exact candidate identity, linked windows and explicit seed classification.
- Created: `packages/open-bitcoin-consensus/src/candidate_history/tests.rs` — 20 structural positive/refusal tests.
- Created: `packages/open-bitcoin-consensus/src/candidate_history/rules.rs` — immutable derived rule result and existing validation-carrier mapping.
- Created: `packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs` — 10 independent boundary/formula/public-path tests.

## Completed tasks

1. Define checked branch-context data contracts: check actual header hashes, selected network, exact candidate parent, parent+1 height, consecutive ancestor heights/links, repeated identities and exact selected genesis. Require complete min(11,height+1) windows for both parent MTP and expiry's parent-minus-min(144,parent.height) MTP. Require the exact retarget anchor and timely minimum-difficulty recovery path; missing rows produce typed errors. BIP34 state distinguishes a linked ancestor from true preactivation absence, an unselected commitment, an unconditional recheck and a detached independent seed.
2. Derive exact activation, flags and duplicate-admission rules: buried version floors2/3/4, BIP34 height, CSV-gated BIP113/BIP68, independent Segwit commitment activation, baseline P2SH|WITNESS|TAPROOT with exact selected-hash replacement before DERSIG/CLTV/CSV/NULLDUMMY additions, and network/height/hash-scoped BIP30 repeat or BIP34-commitment exemptions with unconditional scanning at1983702. Policy flags are absent. Default regtest's null BIP34 commitment never exempts scanning.

Both tasks are complete within their pure-core ownership. No new crate, dependency, I/O, public implementable authority trait, production panic/unwrap site or caller-selected trusted flag was introduced. The four new Rust files carry pinned-source parity breadcrumbs; Plan19 owns manifest registration.

## API and authority handoff

HistoryHeader is an explicitly untrusted input row with chain,height,block_hash,header and a convenience new(chain,height,header) that computes its hash. CandidateHistory::check(NetworkRules, candidate_header, parent, older_ancestors) takes ancestors newest-first and checks all supplied rows. It exposes immutable candidate/hash/height/parent/branch/network getters, parent_median_time_past(), expiry_median_time_past(), median_time_past_at(height), maybe_retarget_anchor(), maybe_min_difficulty_recovery(), maybe_bip34_ancestor(), bip34_context() and origin(). Candidate identity is always computed from header bytes; no separate claimed candidate hash is accepted.

HistoryOrigin::GenesisLinked means supplied links reach the exact selected genesis; those headers may still fail consensus. HistoryOrigin::CheckedSuffix{oldest_height,oldest_hash} means the supplied contiguous suffix terminates above genesis. A canonical owner may consume such a strict suffix only after independently proving that its records, parent and relevant branch facts come from that owner's earned lineage. Neither classification establishes PoW, contextual validity, coins ownership, owner generation, acceptance or actual-daemon evidence.

CandidateHistory::check_seeded(..., maybe_bip34_seed) is a separate conditional historical constructor. It still checks all normal linked windows and the independent seed's network, height and actual header hash, rejects disagreement when the seed overlaps the linked suffix, and ALWAYS classifies the result IndependentlySeeded, including when the supplied linked rows reach genesis. It does not prove the detached anchor's branch membership. Canonical network/coins owners must reject this class for product authority. No unknown required anchor is converted to absence. A required missing seed errors; unexpected unneeded seeds also error. Regtest/Signet's unselected commitment and the unconditional recheck do not require an unneeded anchor.

Bip34Context exposes BeforeActivation, NoSelectedCommitment, UnneededAtRecheck, Linked(hash) or IndependentlySeeded(hash). The maybe_bip34_ancestor() accessor returns only an actual linked ancestor; consumers must inspect bip34_context() to distinguish other cases. The activation block cannot use itself as its parent's ancestor.

CandidateRules::derive(&history), also history.rules(), preserves candidate_hash,parent_hash,height,network_rules,origin and bip34_context. It provides script_flags(), maybe_minimum_block_version(), accepts_block_version(), enforce_bip34_height(), enforce_sequence_locks(), enforce_segwit_commitment(), finality_cutoff(), bip30_disposition(), requires_bip30_scan() and consensus_params(). Bip30Disposition distinguishes ScanRequired, HistoricalRepeat and Bip34Commitment. A seeded commitment exemption remains conditional because its origin/context taint is retained. consensus_params() populates every existing carrier field from selected facts and derived activation; copied carriers confer no authority. history.block_validation_context(shell_observed_now) fills the existing contextual carrier with exact branch MTP, difficulty anchor/recovery facts and those params, without granting a header-admission seal.

Plans04/06 must perform the actual header and owner-bound coins checks using this result. Plan16 must preserve independent seed classification for historical execution; it must not substitute a detached seed for canonical lineage. No needless whole-chain copy is required for a strict suffix whose needed facts are present and whose origin the consuming owner independently proves.

## Executed Commands and actual outcomes

All Cargo commands used the required pinned Bun PATH prefix and ran serially through scripts/command-timings.ts, reusing the existing target and cooperative lock. Logs are retained under `.local/open-bitcoin-dev/phase167/plan03/`.

| Command after required PATH prefix | Actual outcome | Log |
| --- | --- | --- |
| `bun run scripts/command-timings.ts run --key phase167-03-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_candidate_history` | Initial RED discovered11:0 passed/11 failed. Ten were unmet structural behaviors; one was test-helper timestamp overflow at u32::MAX, separately corrected with saturating fixture arithmetic. GREEN discovered14:14 passed. Six later refusal/window tests brought final history coverage to20. | task1-red.log; task1-green.log |
| `bun run scripts/command-timings.ts run --key phase167-03-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_candidate_rules` | RED discovered10:1 passed/9 behavior failures; GREEN10/10 passed. | task2-red.log; task2-green.log |
| `bun run scripts/command-timings.ts run --key phase167-03-focused -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus phase167_candidate` | After simplification30/30 passed; after final style/accessor/probe clarification30/30 passed again. | focused-final.log; focused-final-after-clippy.log |
| `bun run scripts/command-timings.ts run --key phase167-03-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-features` | 309 passed:292 unit,13 integration and4 doctests, including both new compile-fail privacy examples. | regression.log |
| `bun run scripts/command-timings.ts run --key phase167-03-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-consensus --all-targets --all-features -- -D warnings` | First exit101:two collapsible_if findings. Both conditions collapsed; final exit0. | clippy.log; clippy-final.log |
| `rustfmt --edition 2024 --config skip_children=true --check packages/open-bitcoin-consensus/src/lib.rs packages/open-bitcoin-consensus/src/candidate_history.rs packages/open-bitcoin-consensus/src/candidate_history/tests.rs packages/open-bitcoin-consensus/src/candidate_history/rules.rs packages/open-bitcoin-consensus/src/candidate_history/rules/tests.rs` | Exit0. | rustfmt-check.log |
| `git diff --check` | Exit0; owned registration and complete new module/test contents reviewed. | diff-check.log |

The regression suite preceded the final accessor cleanup, two style-only condition collapses and extra explicit synthetic exception probe. Final focused tests and Clippy cover those final changes. Focused integration binaries discovered zero matching tests; their zero-test outcomes are not counted as behavioral evidence. The named focused unit test counts above are nonzero.

Some Cargo sessions waited for the artifact-directory lock. Liveness and process identity were inspected, all resumable sessions were polled within60s, and no job was terminated or duplicate target created. The separate pre-existing Cargo process was left untouched; root independently confirmed its eventual exit. Complete failed logs remain intact.

## Source and evidence

Pinned Knots source HEAD was independently observed as `a9aee730466ac67d35a3c03ee24676be5e045878`. The rules follow local chainparams, deploymentstatus, validation, chain MTP, difficulty, transaction locks and coins source. Source SHA256 values are retained in source-digests.log:

- kernel/chainparams.cpp: `17170008ef7abe50f1fbde9a854e443b7d0b1119780c8a58329538e8ee038485`
- validation.cpp: `5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495`
- chain.h: `c0188574b418a1b67b66d17b4cdb46eb1738bc6e6155514c3d11cc3235db5b1c`
- pow.cpp: `9b27f47410b0906b30f12ead4a95e4d345f7832606838f1440ef35b84c708fe2`
- consensus/tx_verify.cpp: `8fbc9bde843c2cfb0577ae7d179ad11f928b09bf8b0fc3e43a9ede7dc34a7370`
- coins.cpp: `12bce0202076a4dd462f16833a6a8f650fe1f6cb283cb2e9955d31542128a95d`

Public-path tests use computed header identities and structurally checked synthetic branches. Private formula tests use pinned literal activation/hash/commitment facts, including deliberately synthetic Testnet3 heights to prove replacement-before-addition semantics. Those private probes are narrower source-calculator evidence, not actual historical header execution, real PoW, accepted lineage or whole-chain qualification. No partial Plan02 corpus, acquisition cache or producer data was read or changed; full acquired-header/body execution remains its later checked consumer's responsibility.

AGENTS.md, the Bright Builds sidecar/overrides, relevant architecture/code-shape/testing/verification/Rust guidance, the full03 plan/context, accepted01 summary and actual immutable01 APIs informed the implementation. Both active lesson files were fully loaded:8856bytes total,2953 conservative tokens. No project skills were present. The originating execute-plan/TDD/checkpoint/summary contracts were followed with the explicit root-owned saving exception.

## Decisions and simplification

One shared private constructor now checks normal windows for both public construction paths. The independent seed path then adds an explicit persistent evidence classification rather than a trusted boolean. CheckedSuffix avoids conflating strict linked suffixes with detached seeded anchors. One rule derivation drives flags, activation, finality, BIP30 and complete existing carriers; no duplicate validation/work/owner engine was introduced. The complete files remain below the managed628-line refactor trigger; the largest new test module is482 lines.

## Deviations from Plan

No cross-owner scope expansion. Root clarified the already approved independently seeded historical contract when bounded historical envelopes could not provide hundreds of thousands of links back to BIP34. The strict constructor remains linked; the explicit seed path preserves conditional classification. This is within D-06 and the plan's seeded-context requirement. Initial fixture overflow and Clippy findings were corrected only in owned files; all failure evidence remains recorded above.

## Task commits

commits:false. No staging, task/metadata commits, push, hook bypass, worktree, agent delegation or shared STATE/ROADMAP/requirements/config/task ownership changes were performed. Root owns saving after full native, independent source/security/goal and lifecycle acceptance.

## Known stubs and threat scan

No unresolved production or test stubs remain. Internal temporary MTP initializers are always replaced by checked complete windows before any context returns. Optional anchors are explicit source-unneeded states or typed errors, never default target/MTP substitutions. No new endpoint, auth path, filesystem access or durable schema surface was introduced. T-167-03-01/02 are covered by exact identity/window/refusal and boundary controls; T-167-03-03 is covered by source attribution, retained failures and evidence-class limits.

## Remaining phase obligations

The requirements array records this plan's bounded contribution, not phase-wide completion. Header admission, actual coins/interpreter execution, selected-context owner/generation binding, historical corpus consumer tests, durable undo and runtime integration remain later owners. Plan19 registers the four new source breadcrumbs and renews current guards after review. Full native verification, fresh coverage, Bazel and independent reviews remain Plan20. No ordinary selector support, Signet challenge qualification, public-network default or actual pinned-daemon claim was enabled.

## Self-Check: PASSED

All five owned Rust files and this summary exist. Final focused30/30, scoped Clippy, scoped formatting and diff checks passed. The broader consensus regression passed309 tests with the timing noted above. No task commits are claimed or fabricated; the labelled checkout baseline is an existing Git object. Shared planning state and all other owners' edits were preserved.
