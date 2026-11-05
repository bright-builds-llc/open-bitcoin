---
phase: 167-candidate-history-validation-rules
plan: "04"
subsystem: network
tags: [rust, canonical-headers, candidate-history, difficulty, median-time]
requires:
  - phase: 167-01
    provides: Immutable selected network rules and proved genesis parameter binding
  - phase: 167-03
    provides: Checked structural CandidateHistory and shared CandidateRules
provides:
  - Private canonical header lineage earned from proved genesis and admitted parents
  - Shared selected PoW, contextual time, difficulty, version and expiry checks
  - Body-context access using retained original admission clock evidence
affects: [167-06, 167-09, 167-10, 167-12, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Owner-held admission evidence, Bounded earned suffix, Explicit re-admission]
key-files:
  created:
    - packages/open-bitcoin-network/src/header_store/canonical_history.rs
    - packages/open-bitcoin-network/src/header_store/canonical_history/tests.rs
    - packages/open-bitcoin-consensus/src/candidate_history/header_rules.rs
  modified:
    - packages/open-bitcoin-network/src/header_store.rs
    - packages/open-bitcoin-network/src/header_store/canonical_genesis.rs
    - packages/open-bitcoin-consensus/src/candidate_history.rs
key-decisions:
  - "Canonical insertion accepts header bytes and explicit shell now, never caller-provided structural context or arithmetic work."
  - "Every row in a strict suffix is drawn from this owner's earned lineage; independently seeded contexts have no canonical insertion route."
  - "Body rechecks recover the original admission clock from private owner state; duplicate admission preserves it."
patterns-established:
  - "Genesis and contextual admission are distinct private evidence variants."
requirements-completed: [CVLD-02, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T18:26:53Z"
commits: false
duration: 11min
completed: 2026-10-21
---

# Phase 167 Plan 04: Contextually earned header ancestry and time/difficulty Summary

**Canonical regtest headers now earn private lineage through selected branch-specific PoW, difficulty, MTP, version, future-time and expiry checks, with original admission clocks retained for body rechecks.**

Source checkout baseline, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`, observed with git rev-parse HEAD before implementation and at final checks. This existing main-repository object identifies the checkout; commits:false remains in force.

## Files Created/Modified

- Modified: `packages/open-bitcoin-network/src/header_store.rs` — contextual insertion entrypoint, private registry field/reset, legacy arithmetic-path refusal on canonical descendants, complete numerical MTP samples.
- Created: `packages/open-bitcoin-network/src/header_store/canonical_history.rs` — proved-root registry, bounded owned ancestry, canonical admission/re-admission, retained body clock and error mapping.
- Created: `packages/open-bitcoin-network/src/header_store/canonical_history/tests.rs` — nineteen positive/refusal tests using actual store APIs and deliberate private-corruption controls.
- Modified: `packages/open-bitcoin-network/src/header_store/canonical_genesis.rs` — initialize private lineage after existing proved genesis construction; preserve its borrowed-proof signature.
- Created: `packages/open-bitcoin-consensus/src/candidate_history/header_rules.rs` — shared pure header checks and five explicitly seeded source-formula tests.
- Modified: `packages/open-bitcoin-consensus/src/candidate_history.rs` — register the shared header-rule module, preserving accepted03 contracts.

## Completed tasks

1. Mint contextual lineage from proved parents. The private registry starts only from CanonicalGenesisProof. Canonical descendants derive CandidateHistory from the exact parent and up to155 earned rows, sufficient for selected regtest's parent MTP, parent-minus144 MTP, interval144 anchor and recovery chain. Each supplied row must have earned membership before structural hash/height/link/window checks run. GenesisLinked and CheckedSuffix retain03 meanings; every suffix row is independently owned. There is no canonical API accepting a caller-provided CandidateHistory, detached seed, HeaderProofFacts, raw height/work or fixture parameters. Imports, record_position and seed_from_chain confer no membership. Explicit re-admission recomputes every descendant decision and full-width work from raw header bytes. Existing modulo256-bit work addition and strict-greater tip replacement preserve first-arrival ties.
2. Enforce shared selected header rules. The consensus helper reuses existing PoW validation and check_block_header_contextual/next_work_required, then CandidateRules versions and the pinned strict expiry predicate. Tests exercise future7200 equality, strict MTP, invalid PoW/bits/version/context, unchanged entire-store rejection, regtest143/144/145 and287/288/289, retarget quarter/four-times clamps, powLimit cap, compact rounding, and Testnet3 strict1200/equality recovery. Default regtest BIP94 remains false. Its sole checkpoint is the authenticated genesis. Mainnet/Testnet3 canonical descendants remain refused for missing checkpoint prerequisites, and Signet remains refused for missing challenge validation. No global support flag changed.

The generic numerical MTP accessor now requires min(11,height+1) linked samples, checks heights and parent-map/header links, and selects the upper median. Canonical context additionally verifies every row's actual header hash through CandidateHistory. Numerical fixture projections never become canonical evidence.

## API and authority handoff

HeaderStore::from_canonical_genesis keeps the existing borrowed CanonicalGenesisProof signature. The proof's checked identity is absorbed into the private root registry; no caller constructor for that registry exists. HeaderStore::insert_canonical_header(header, shell_observed_now) is the contextual ingress. The shell must capture now independently. The old insert_header/insert_header_with_params APIs remain arithmetic fixture paths and refuse canonical descendants because they have no trusted clock.

HeaderStore::maybe_canonical_chain() and is_canonically_admitted(hash) are read-only projections. canonical_header_history(hash) returns structural facts only for an actually admitted candidate; genesis uses its separate proved-root path. Returning CandidateHistory does not turn that public structural type into transferable owner authority. Coins owners must independently bind their branch and generation and query the held HeaderStore rather than accepting an arbitrary caller-provided context as admission evidence.

HeaderStore::canonical_header_validation_context(&header) requires this exact candidate's private admission record, rebuilds its earned branch context, rechecks shared rules with the original stored shell now and returns the existing BlockValidationContext carrier. Duplicate reception preserves the original clock and store. This differs deliberately from the unowned legacy eligibility::header_validation_context(header,current_time,params), which remains the numerical fixture helper. Plans06/09/10/12 must route canonical bodies through the new accessor and preserve that original admission evidence; candidate.time and persisted status labels cannot replace it. Existing legacy body insertion refuses canonical descendants until those consumers are integrated.

HeaderStore::readmit_canonical_entries(proof,parent_first_entries,shell_now) constructs a fresh proved-root owner and revalidates header bytes. Imported claimed hashes/heights/work are ignored and recomputed. This explicitly establishes a new admission order; it does not recover historical arrival/body sequence from persisted numbers. Body eligibility remains a separate owner and is not granted by header lineage.

CandidateHistory::check_header_rules(shell_now) checks selected PoW and contextual rules. CandidateHistory::check_contextual_header_rules(shell_now) is the explicitly narrower contextual/formula seam without PoW. Neither helper grants lineage, checkpoint support, Signet qualification, coins validity or ScriptsValid. IndependentlySeeded remains tainted even when these formulas pass. Canonical insertion constructs only strict checked context from its own registry and never consumes this seeded path.

## Executed commands and actual outcomes

Every Cargo invocation used the pinned Bun PATH prefix and ran serially through scripts/command-timings.ts with the existing target and cooperative lock. Complete logs are under `.local/open-bitcoin-dev/phase167/plan04/`. The first retained RED log was created at2026-10-21T18:15:52Z; summary generation is timestamped above. Duration records that observable implementation interval.

| Command after required PATH prefix | Actual outcome | Log |
| --- | --- | --- |
| `bun run scripts/command-timings.ts run --key phase167-04-task1 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase167_header_history` | Initial setup exit101: ProofWork lacks Default; corrected to ZERO. Behavioral RED discovered2:0 passed/2 failed (arithmetic canonical extension and truncated MTP). GREEN discovered10:10 passed. | task1-red.log; task1-behavior-red.log; task1-green.log |
| `bun run scripts/command-timings.ts run --key phase167-04-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase167_header_rules` | Behavioral RED discovered1:0 passed/1 failed (expired canonical descendant accepted). | task2-red.log |
| `bun run scripts/command-timings.ts run --key phase167-04-task2 -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-consensus phase167_header_rules` | First GREEN attempt exit101: existing validation error field is debug_message, not maybe_debug_message; corrected owned mapper. GREEN discovered11:5 consensus and6 network passed. | task2-green.log; task2-green-after-mapping.log |
| `bun run scripts/command-timings.ts run --key phase167-04-focused -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-consensus phase167_header` | After simplification24/24 passed:5 consensus and19 network. | focused-final.log |
| `bun run scripts/command-timings.ts run --key phase167-04-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-consensus --all-features` | First run: consensus310 passed; network672 passed/1 failed in the unmodified Phase165 missing-recovery-ancestor test. Generic MTP hash checking changed synthetic error precedence; removed only that unneeded numerical hash check, retaining canonical hash/link/sample checks. Complete post-fix run992 passed; subsequent confirmation after the exact legacy test992 passed. | regression.log; regression-final.log; regression-confirmed.log |
| `bun run scripts/command-timings.ts run --key phase167-04-legacy-regression -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network phase165_eligibility_context_refuses_missing_recovery_ancestor_beyond_mtp_window` | Original unmodified regression discovered1:1 passed after the owned source correction. | legacy-regression-final.log |
| `bun run scripts/command-timings.ts run --key phase167-04-clippy -- cargo clippy --manifest-path packages/Cargo.toml -p open-bitcoin-network -p open-bitcoin-consensus --all-targets --all-features -- -D warnings` | Exit0, no warnings. | clippy.log |
| Scoped rustfmt edition2024 with skip_children=true check of all six owned Rust files | Exit0. | rustfmt-check.log |
| `git diff --check` | Exit0; tracked owned diff and complete new source/test files reviewed. | diff-check.log |
| gsd-tools verify key-links for Plan04 | Original conceptual target failed0/1; root corrected to two real source edges. First quoted-pattern metadata attempt failed0/2 because the parser preserved single quotes. Root used plain YAML patterns; final2/2 passed. No fake code/comment was inserted to satisfy the parser. | key-links-quoted-pattern.log; key-links-final.log |

The992 aggregate is297 consensus unit+13 consensus integration+4 consensus doctests+673 network unit+4 network integration+1 network privacy doctest. Focused integration binaries discovered zero matching tests and are not counted as behavioral evidence. The24 focused tests preceded the generic MTP hash-check removal; both complete992 runs and the unchanged targeted regression cover final source. No source changes followed those successful runs. Clippy and scoped formatting cover final source.

Some resumable compilations were quiet or waited for the artifact lock; process identity/liveness and log progress were inspected, sessions were polled within60s, and no build was terminated or target copied. All actual failed attempts remain recorded above and their command logs are retained. No authentication gate occurred.

## Source and evidence

Pinned Knots source HEAD was independently observed at a9aee730466ac67d35a3c03ee24676be5e045878. Source-derived expectations follow local ContextualCheckBlockHeader, GetNextWorkRequired/CalculateNextWorkRequired, GetMedianTimePast, default regtest checkpoint/deployment facts and IsThisSoftwareExpired. Exact source SHA256 values are retained in source-digests.log, including validation5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495 and pow9b27f47410b0906b30f12ead4a95e4d345f7832606838f1440ef35b84c708fe2. Default expiry is exactly1857151480=(58*31558060)+26784000; both candidate time and the complete parent's144-lookback MTP must be strictly greater.

These are local behavioral/source-formula tests. Seeded Mainnet/Testnet3 and hard-target regtest probes intentionally omit actual PoW/admission and retain IndependentlySeeded. They do not establish canonical insertion, historical serialized execution, whole-chain acceptance or actual pinned-daemon differential qualification. No Plan02 partial corpus, cache or producer data was read or modified.

AGENTS.md, the Bright Builds sidecar/overrides and relevant architecture/code-shape/testing/verification/Rust guidance informed the changes. Both active lesson files were loaded completely:8856bytes and2953 conservative tokens. No project skill was present. The execute-plan, summary, TDD and checkpoint contracts were applied within the root-owned saving/progress exception.

## Decisions and simplification

One private registry augments the existing HeaderStore rather than introducing a parallel work owner. Explicit Genesis/Contextual evidence variants avoid optional-clock sentinel states. Existing difficulty, PoW, CandidateRules and best-tip arithmetic remain the single calculators. A bounded155-row owned suffix avoids copying the whole chain while proving every required default-regtest fact. All owned files remain below the managed628-line refactor trigger; no production dependency, crate, I/O, panic or unwrap was introduced.

## Deviations from Plan

No cross-owner scope expansion. The existing genesis constructor signature was preserved with root approval; the checked proof is absorbed semantically into lineage. The required numerical MTP completeness fix initially added a hash check that changed an existing synthetic regression's error precedence; the owned root-cause correction removed that redundant numerical check while preserving stricter canonical verification. The root independently corrected conceptual key-link metadata to actual paths/calls. Setup errors, behavioral RED results and the full initial regression failure are preserved above.

## Task commits

commits:false. No staging, task/metadata commits, push, hook bypass, new worktree, subagent or shared STATE/ROADMAP/requirements/config/AGENTS/task/guard/breadcrumb-manifest edit was performed. Root owns shared progress and normal-hook saving after full-phase acceptance.

## Known stubs and threat scan

No unresolved stubs remain. Genesis has a deliberate distinct admission state; unsupported checkpoint/Signet paths are explicit refusals. No new endpoint, authentication path, filesystem access, production dependency or durable schema trust boundary was introduced. T-167-04-01/02 are covered by private construction, import/fixture/version/context/branch/refusal tests and unchanged whole-store checks; T-167-04-03 is covered by source provenance, retained failures and explicit evidence limits.

## Remaining phase obligations

The requirements list records this plan's bounded contribution and does not mark phase-wide requirements complete. Canonical body/coins owner integration, actual shell-clock capture, selected stage/generation binding, runtime ingress, durable restart/replay and historical serialized consumers remain their later owners. Plan19 registers the three new Rust breadcrumbs and renews affected guards after source review. Plan20 owns full native verification, fresh coverage, Bazel and independent final source/security/goal reviews. Ordinary product support, networking defaults, wallets and actual-daemon claims remain unchanged.

## Self-Check: PASSED

All six owned Rust files and this summary exist. Final complete regression992/992, original legacy regression1/1, focused24/24 with final-source coverage supplied by the broad runs, scoped Clippy, scoped formatting and diff checks passed. The new privacy doctest passed. Both actual key links pass2/2. No task commits are claimed; the explicitly labelled source-checkout baseline is an existing Git object. Shared metadata and other owners' files were preserved.
