---
phase: 167-candidate-history-validation-rules
plan: "06"
subsystem: chainstate
tags: [rust, candidate-history, canonical-admission, median-time, provenance]
requires:
  - phase: 167-01
    provides: Immutable selected rules and complete proved-genesis parameters
  - phase: 167-03
    provides: Checked structural CandidateHistory and CandidateRules
  - phase: 167-04
    provides: Shared header checks and original-clock owner contract
  - phase: 167-05
    provides: Selected block checks and branch-derived transaction interfaces
provides:
  - Proof-initialized canonical selector on the existing coins work owner
  - Bounded private header admission and original-clock branch-bound stages
  - Attributable completion of original Task1 only
affects: [167-21, 167-23, 167-07, 167-09, 167-10, 167-16, 167-19, 167-20]
tech-stack:
  added: []
  patterns: [Private owner-held admission, Fresh transfer identities, Explicit seeded fixture taint]
key-files:
  created:
    - packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs
    - packages/open-bitcoin-chainstate/src/engine/tests/phase167_context.rs
  modified:
    - packages/open-bitcoin-chainstate/src/engine.rs
    - packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs
    - packages/open-bitcoin-chainstate/src/engine/work_authority.rs
    - packages/open-bitcoin-chainstate/src/engine/stage.rs
    - packages/open-bitcoin-chainstate/src/engine/tests.rs
key-decisions:
  - "The existing WorkAuthority holds selection and pending admission; structural contexts cannot initialize canonical identity."
  - "Clone/rebind preserve selected facts with fresh identities and no transferable admission; preview permits only the original stage continuation."
  - "Current06 completes only original Task1; interpreter and actual coins obligations remain separately attributable in21/23."
patterns-established:
  - "Exact headers, earned branch and original shell observation bind each canonical body stage."
requirements-completed: [CVLD-01, CVLD-02, CVLD-03, CVLD-04]
generated_by: gsd-execute-plan
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T19:59:19Z"
commits: false
duration: 3min24s
completed: 2026-10-21
---

# Phase 167 Plan 06: Canonical coins context and stage provenance Summary

**Proved genesis initializes private canonical selection on the existing coins owner; canonical stages require bounded exact-header admission, earned branch context and the original shell clock.**

## Files Created/Modified

- Modified: `packages/open-bitcoin-chainstate/src/engine.rs` — candidate module, selected connect admission/context and eleven-sample MTP.
- Created: `packages/open-bitcoin-chainstate/src/engine/candidate_rules.rs` — private admission, branch-derived context and tainted seeded fixture stage.
- Modified: `packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs` — proof-only selector initialization after existing empty-coins/root checks; accepted01 mapping retained.
- Modified: `packages/open-bitcoin-chainstate/src/engine/work_authority.rs` — existing owner selection, bounded pending admission lifetime and fresh clone/rebind identities.
- Modified: `packages/open-bitcoin-chainstate/src/engine/stage.rs` — exact disconnect/replacement admission and per-candidate branch context.
- Created: `packages/open-bitcoin-chainstate/src/engine/tests/phase167_context.rs` — ten context/provenance tests and canonical test helpers.
- Modified: `packages/open-bitcoin-chainstate/src/engine/tests.rs` — existing registrar; shared partial original Task2 adapter edits remain allocated to23.

These are the seven declared Rust owners, not seven new edits in this finalization. Two files were created and five existing files modified during original execution. Finalization changed no Rust source. candidate_rules.rs and tests.rs also contain pre-split partial Task2 work; listing them does not accept that work. All fifteen original combined06 source owners match the revision-start SHA256 manifest, including the seven retained owners and eight carried-forward owners. Receipt: [source preservation](../../../.local/open-bitcoin-dev/phase167/plan06/source-preservation-context-finalization.json).

Source checkout reference only, not a task commit: `1b472fb225a290df5cdb06eeea2dc67c624c648e`. Its existing Git commit object was checked. commits:false remains in force.

## Completed task and scope boundary

1. **Bind candidate rules to coins lineage and stages:** complete. The existing WorkAuthority holds immutable selected facts and one pending batch capped at4096 headers. Only successful from_canonical_genesis initializes selection. Generic Live roots, matching labels, snapshots, copied params/context and mechanical fixtures cannot initialize it. Mutable-coins invalidation removes selection. Clone and exact consuming rebind preserve selected facts with fresh owner/generation identity and no pending admission or old stage authority.

Canonical header admission derives CandidateHistory from the owner's actual earned branch with a155-row header suffix, validates shared selected rules and retains exact header bytes, derived rules, disconnect identity and the first successful shell observation. Repeating the same pending identity retains its clock. Generation changes, transfer and acceptance clear admissions. Selected connect/reorg stages independently derive each staged branch context, preserve full-width expected work and refuse absent/wrong/stale/foreign evidence before accepted mutation. Preview suspends usable selection; only the original authorized stage can continue. MTP uses the new header plus ten parents. Required linked windows and unsupported checkpoint/Signet paths refuse. Accepted01 monetary/height-zero/empty-genesis behavior remains intact.

The explicit seeded historical adapter uses the shared checked body/apply kernel while retaining MechanicalFixture/IndependentlySeeded classification. Neither a passing formula nor a genesis-shaped prefix grants live/product authority. The trusted-shell clock is explicitly supplied data; this API does not prove membership in the separate HeaderStore. Plans09/10 must coordinate the held owners and original clock.

The revised [06 plan](167-06-PLAN.md) retains byte-identical original Task1. The checked [execution-scope amendment](167-EXECUTION-SCOPE-AMENDMENT.md) and latest [independent plan review](167-PLAN-CHECK.md#review-execution-scope-amendment--2026-10-21t195338z) supersede only historical ownership/allocation. [Original two-owner amendment](167-06-AMENDMENT.md) added canonical_genesis/work_authority; the later [five-owner locktime amendment](167-06-LOCKTIME-AMENDMENT.md) grew the original combined union to fifteen. Those historical amendments and their snapshots remain preserved; current06 has one task/seven owners.

Original Task2 remains partial and is carried in full across [21 interpreter/direct probes](167-21-PLAN.md) and [23 actual coins integration](167-23-PLAN.md). This summary accepts neither whole original06 nor the phase. Frontmatter requirement IDs record this bounded contribution, not phase-wide completion or a request to update shared requirements.

## Current finalization evidence

All Cargo work used PATH=/Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin:$PATH and scripts/command-timings.ts serially with the existing target/cooperative lock. No Cargo/Bazel work was overlapped, no target copied and no process terminated.

| Actual command/check | Outcome | Receipt under .local/open-bitcoin-dev/phase167/plan06/ |
| --- | --- | --- |
| bun run scripts/command-timings.ts run --key phase167-06-context-finalization -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-chainstate phase167_context | Exit0;10 discovered/10 passed,0 ignored,0 skipped;399 unit tests filtered. Separate parity integration binary had0 matching/3 filtered and is not counted. | [context-finalization-test.log](../../../.local/open-bitcoin-dev/phase167/plan06/context-finalization-test.log) |
| gsd-tools verify key-links for current167-06-PLAN.md | all_verified:true;2/2 real source links. proof.spec().rules() and CandidateHistory::check are executable source wiring. | [key-links-context-finalization.json](../../../.local/open-bitcoin-dev/phase167/plan06/key-links-context-finalization.json) |
| rustfmt --edition2024 --config skip_children=true --check on all seven declared owners | Exit0; no scoped write or recursive child formatting. | [rustfmt-context-finalization.log](../../../.local/open-bitcoin-dev/phase167/plan06/rustfmt-context-finalization.log) |
| git diff --check restricted to the seven owners; tracked owned diff plus complete new files inspected | Exit0; preserved accepted01 changes and partial Task2 shared-file edits. | [diff-context-finalization.log](../../../.local/open-bitcoin-dev/phase167/plan06/diff-context-finalization.log) |
| SHA256 comparison against revision-start manifest | All15 original combined06 source owners byte-identical, including all seven current owners. | [source-preservation-context-finalization.json](../../../.local/open-bitcoin-dev/phase167/plan06/source-preservation-context-finalization.json) |

The ten final cases cover proof-only initialization/invalidation, original-clock future-time equality and duplicate observation retention, clone/rebind selection versus receipt rejection, header/parent/work/generation atomic refusal, bounded replacement branches, seeded-fixture isolation, preview continuation, missing-window/unsupported-network refusal, missing admission despite caller time, and distinguishing eleven-sample MTP. Existing complete snapshot assertions preserve tip/work/coins/undo; no runtime/BASIC/durable acceptance is inferred.

The retained historical strict chainstate all-target/all-feature Clippy pass is in clippy-final.log. It predates later locktime/reward edits and is not presented as fresh final affected Clippy. Plan23 owns that check and the complete non-skipped consensus+chainstate --all-features regression. Full repo-native bash scripts/verify.sh, fresh coverage/Bazel and independent final reviews remain20.

## Original execution outcomes and preserved failures

The full [checkpoint notes](../../../.local/open-bitcoin-dev/phase167/plan06/checkpoint-notes.md), including the19:35Z continuation, remain the authoritative detailed history. All original logs remain unchanged at their existing paths;21 copied logs totaling559938bytes are also preserved by the amendment's evidence manifest. No RED was replayed merely to change attribution.

| Historical receipt in plan06/ | Exact retained interpretation |
| --- | --- |
| task1-red.log | Setup/import errors for CanonicalGenesisProof and merkle-root API; not behavioral RED. |
| task1-red-behavior.log | Genuine missing-private-admission failure; the original MTP expectation was a non-distinguishing test mistake, not valid MTP RED. |
| task1-green-initial.log | Admission passed; erroneous MTP assertion still failed. |
| task1-green.log |3 passed/5 failed from canonical coinbase helper encoding heights1–16 as pushes rather than OP_1..OP_16. Canonical-only helper corrected; legacy fixture helper retained. |
| task1-green-final.log |8/8 passed after corrected MTP vector and canonical height helper. |
| task1-mtp-inverse.log | Corrected distinguishing vector against original12-sample implementation:1 discovered/1 failed, actual55 versus expected50. Final11-sample source restored immediately. This is the genuine MTP inverse. |
| task1-final.log | Expanded original Task1 final10/10 passed. Current finalization independently repeated10/10 on unchanged source. |
| task2-red.log | Setup import error: private crypto::sha256 corrected to public Sha256::digest. |
| task2-red-behavior.log |9 discovered/9 failed; seven genuine canonical caller-rule/policy/budget gaps. CLTV/CSV failed first in omission controls with UnsupportedOpcode, not valid canonical-bypass RED. |
| task2-green-initial.log |7/9 passed; two missing-interpreter omission controls remained. |
| clippy.log;clippy-final.log | Initial strict check rejected nine-argument apply helper and two cloned_ref_to_slice_refs cases. Existing block context removed redundant height/MTP arguments; slice::from_ref used. Final strict scoped Clippy exit0. |
| regression-partial.log |400 unit+3 integration+23 doctests passed with exactly two known CLTV/CSV tests explicitly skipped. Partial evidence only. |
| final-bounded-focus.log |21 focused passed:10 context+7 script+4 accepted genesis-parameter cases; the same two CLTV/CSV exclusions remain partial. |
| locktime-positive-red.log |1 discovered/1 failed UnsupportedOpcode(b1); loop stopped at first case, proving only missing CLTV in that log. |
| locktime-positive-pair-red.log |2 discovered/2 failed UnsupportedOpcode(b1/b2) in actual funded canonical positive cases. Genuine missing-feature RED. |
| locktime-green-initial.log |11 discovered/11 passed after locktime implementation, including active equality, canonical omission/refusal pairs and original seven controls. |
| locktime-boundaries.log |15 discovered/14 passed/1 setup failure: inherited OP_1 scriptSig meant stack was not empty. UnsatisfiedLockTime was correct for the actual stack; test setup alone corrected. |
| locktime-boundaries-final.log |15 discovered/15 passed after explicit empty scriptSig correction. |
| rewards.log |Separate1/1 passed after the15-case run: actual149/150/299/300 subsidy+10000sat-fee boundaries, +1sat refusal and full state preservation. No combined16-case or broad-regression claim. |

One original apply_patch attempt had nonexistent trailing context and changed no files; a corrected targeted patch succeeded. Transient artifact-lock waits resolved normally with liveness/polling; no job was killed. During this finalization, optional skill-directory inventory returned rg exit2 because both directories are absent; no project skill exists. The GSD init command itself succeeded. No current test/build failure or authentication gate occurred.

## Continuation obligations

Plan21 owns exactly the five consensus locktime implementation files plus its new direct-probe sibling, existing test helper visibility and registrar. Preserve signed five-byte peek semantics, unchanged four-byte callers, typed negative/unsatisfied errors, fixed500000000 types, actual input/final/version/disable/type/mask/order rules, inactive NOP2/3 with DISCOURAGE, skipped branches, minimality/negative-zero/size/ignored-bit cases. Direct funded-input probes are interpreter evidence and leave the chain snapshot unchanged.

Plan23 owns apply/overlay/candidate transaction context and actual canonical scripts tests. It must complete canonical CSV ignored-bit and operand-disable/version1 positives, maturity100 versus99-depth refusal, zero genesis UTXOs, actual LOW_S absence with explicit policy rejection, MINIMALDATA/CLEANSTACK separation, consumed-only distant creating-MTP and missing/inconsistent metadata, finite4096 history refusal, same-block locks, replacement-branch disagreement, selected flags and actual80000/80001 aggregate budget before spending. Preserve complete accepted state and original-clock/stage protections. Final nonzero discovery, scoped formatting/diff/keylinks, fresh strict affected Clippy and full affected regression without skips are mandatory there. Existing partial/skipped receipts cannot satisfy them.

No new locktime test sibling, remaining monetary/policy/MTP test, acquisition/corpus, shared breadcrumb manifest or guard ownership was edited here. Plan19 retains new Rust manifest registration;17 retains source review before guard renewal. No daemon differential, whole-chain, runtime ordinary support, wallet or network-default qualification is claimed.

## Source, guidance and simplification

Current pinned chain.h::GetMedianTimePast uses exactly11 samples; validation.cpp::ContextualCheckBlockHeader applies strict parent MTP and contextual difficulty/time checks. Rechecked source SHA256 values are chain.h c0188574b418a1b67b66d17b4cdb46eb1738bc6e6155514c3d11cc3235db5b1c and validation.cpp5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495, consistent with accepted dependency evidence. Local behavioral/source evidence does not establish actual pinned-daemon differential qualification.

AGENTS.md Repo-Local Guidance, Bright Builds sidecar, standards-overrides.md and architecture/code-shape/testing/verification/Rust standards informed finalization. Both canonically distinct active lesson files were fully loaded:6898+1958=8856bytes;2300+653=2953 conservative tokens. The October19 audit baseline remains applicable, with no current audit trigger. No new dependency or external API lookup was needed.

Simplification review retained one WorkAuthority and the accepted shared rule/work calculators. A bounded private admission batch and fresh identities avoid a second registry/proof owner. No code changes were needed during finalization; carried Task2 helpers were preserved for their explicit owners.

## Deviations and saving

The root-reviewed amendments resolved the original ownership omissions and missing opcode scope before those edits. The current structural split retains completed Task1 and all remaining original obligations without rollback. This finalization follows the revised plan with no source expansion. Setup mistakes and genuine failures are distinguished above. No unresolved Task1 stubs were found; deliberate optional selectors, empty genesis coins and unsupported-selector refusals are explicit states. No endpoint, authentication, filesystem or durable-schema surface was added. T-167-06-01 is addressed by owner/branch/clock/MTP refusal controls;T-167-06-03 by retained attributable evidence. Original Task2 threatT-167-06-02 continues in23.

commits:false. No staging, commits, push, worktree, hook bypass, shared STATE/ROADMAP/requirements/config/todo/AGENTS edit or guard/breadcrumb ownership change occurred. Root alone updates shared progress and performs normal-hook saving after whole-phase acceptance. The checkout reference above is not a task commit.

## Performance attribution

One completed current task; seven declared Rust owners. Original execution began2026-10-21T19:05:48Z; the19:35Z checkpoint described about29minutes across original Task1, partial Task2, continuation and planning wait. An isolated original Task1 elapsed time was not recorded and is not invented here. That combined historical interval is not added again to21/23 carryforward.

Measured finalization verification began2026-10-21T19:55:55Z and reached the first successful summary/frontmatter checks at2026-10-21T19:59:19Z:204seconds (3min24s), excluding initial context loading and subsequent receipt/metadata cleanup. The scoped rerun itself finished in0.01s after0.15s build setup. This interval is separate from original combined work. generated_at records that successful-check observation.

Summary validation returned passed:true/errors:[] and frontmatter validation valid:true. An initial unsupported --check-files option was ignored by the CLI, which checked its default two real file paths; the corrected --check-count7 invocation checks all seven declared owners. This was a validator-invocation correction, not a source/test failure. Independent comparison also confirmed all21 historical logs remain byte-identical, totaling559938bytes. Final receipts are verify-summary-context-finalization.json and key-links-context-finalization.json in the existing plan06 log directory.

## Self-Check: PASSED

All seven declared Rust owners and this summary exist. Current context discovery10/10, both actual links2/2, scoped formatting and owned whitespace checks pass. All15 original combined source-owner hashes match the preserved snapshot. The pre-existing labelled checkout object exists; no task commit is asserted. Current06 completes one original Task1 only, with21/23 continuation explicitly allocated. Shared metadata and other owners remain preserved.
