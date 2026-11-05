---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: 2026-10-21T16:38:23Z
---

# Phase 167: Candidate-History Validation Rules - Context

**Gathered:** 2026-10-21
**Status:** Ready for planning
**Mode:** Yolo; recommended choices selected in one pass under the strict-push wrapper.

<domain>
## Phase Boundary

Close CVLD-01, CVLD-02, CVLD-03, CVLD-04 and CVLD-06 through selected-network and validated-candidate-history rules in ordinary header admission and durable body connection. Preserve canonical genesis, exact-work authority, pre-effect compatibility refusal and one active Fjall chainstate. Actual pinned-daemon differential qualification belongs to Phase168; fork/recovery differential qualification, canonical pruning and classified historical parity closure retain Phases169–171 ownership.
</domain>

<decisions>
## Implementation Decisions

### Network authority and earned support

- **D-01:** The pinned Knots source at `a9aee730466ac67d35a3c03ee24676be5e045878` defines rules for the four existing selectors. Network identity, local implementation support and actual differential qualification are distinct facts. Enable only paths whose required checks are implemented and whose candidate prerequisites are proved; missing ancestry/deployment facts refuse before accepting or mutating authority. Default Signet remains refused without challenge validation. Do not add testnet4, custom networks or deployment-override product support.
- **D-02:** Derive network subsidy and difficulty parameters from the selected immutable chain specification. Genuine default regtest uses halving interval150 and coinbase maturity100, including ordinary accepted boundary cases149/150 and299/300 and premature-spend refusals. Canonical genesis contributes no spendable outputs. Fixture/custom consensus settings must not confer canonical product authority.

### Candidate ancestry and rule derivation

- **D-03:** Derive BIP34 height, BIP65/BIP66 version constraints, BIP113 finality cutoff, sequence locks and Segwit commitment activation from the actual candidate branch and pinned deployment history. No caller boolean, current active tip or best-header shortcut substitutes for required candidate ancestors. Preserve existing exact-work/tie semantics from Phase165.
- **D-04:** Durable script checks use the pinned `GetBlockScriptFlags` behavior: baseline P2SH/WITNESS/TAPROOT, selected-network exact-block-hash exception replacement, then DERSIG/CLTV/CSV/NULLDUMMY additions according to candidate deployment state. Keep consensus flags separate from policy flags. Prove the flags actually reach script execution rather than testing a disconnected calculator.
- **D-05:** Ordinary headers and durable bodies share selected-network difficulty and contextual time authority, including retarget anchors, minimum-difficulty recovery and median-time boundaries. Validate required ancestry links and scope every seeded historical context explicitly; incomplete history refuses. Research must identify additional pinned contextual conditions needed before any ordinary support claim is enabled.
- **D-06:** Implement pinned BIP30/duplicate-coinbase and overwrite decisions using exact height/hash and BIP34 ancestor facts, including the pinned recheck boundary. Reject unrelated duplicates. Connect/disconnect must preserve and restore earlier coins for supported overwrite contexts; no global permissive switch or lossy undo. Distinguish bounded independently seeded historical contexts from ancestry-complete ordinary validation.

### Ordinary integration and evidence

- **D-07:** Exercise canonical histories through ordinary selected-network header admission and the actual durable body/coins owner, continuing from the proved Phase166 genesis. Keep public networking default-off. Use restricted constructors/proofs to prevent fixture contexts, raw numeric work or arbitrary params from bypassing canonical gates. Wire runtime adapters only after prerequisites exist; do not flip a global support boolean to conceal incomplete checks.
- **D-08:** Rejected blocks, headers, immature spends, wrong versions, flags, difficulty and missing-context inputs leave accepted tip, coins, exact work, BASIC progress and durable authority unchanged. Include branch-specific disagreements and required-history negatives. Reuse existing checkpoint/reopen infrastructure; broader differential fork/recovery evidence remains Phase169.
- **D-09:** Use independent pinned-source expectations and serialized behavioral fixtures for this phase; preserve exact source anchors, evidence classes and intentional differences. Local source-derived tests and corpora do not claim actual Knots differential success. Keep prior evidence attributable and renew affected current ownership records only after source review; never weaken historical tests, deadlines or evidence contracts to pass verification.
- **D-10:** Integrate deterministic regressions into the full offline native verifier, retain complete command outcomes, review source/security/goal independently and validate the originating lifecycle before saving. Verify fresh pure-core coverage and Bazel. Provide copy-pasteable repo-local Cargo/Bazel examples for operator workflows. All nested commits are deferred to the wrapper's clean phase-verification/lifecycle gate, with normal hooks mandatory; stop before Phase168.

### Agent Discretion

Choose the smallest robust typed design within existing consensus/network/chainstate/node adapters. Keep pure rules free of I/O, avoid new production dependencies/workspace crates and split coherent modules rather than expanding central files. Research must resolve branch-history trust, activation semantics, script-flag exceptions and overwrite undo before executable plans are accepted. Plan count may grow to cover all approved requirements without narrowing them.
</decisions>

<canonical-refs>
## Canonical References

**Downstream agents must read these before planning or implementing.**

### Scope and prior authority

- `.planning/ROADMAP.md` — Phase167 five mandatory criteria and later owners.
- `.planning/REQUIREMENTS.md` — CVLD-01/02/03/04/06 and evidence contract.
- `.planning/PROJECT.md` — approved milestone scope; stale historical progress prose grants no authority.
- `.planning/phases/164-canonical-chain-identity-and-compatibility-gates/164-CONTEXT.md` — identity, compatibility and support classification.
- `.planning/phases/165-shared-exact-work-authority/165-CONTEXT.md` — exact work, candidate eligibility and superseding canonical tie rules.
- `.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-CONTEXT.md` — proved genesis, reopen and BASIC ownership.
- `.planning/phases/166-canonical-durable-bootstrap-and-genesis-basic/166-VERIFICATION.md` — completed dependency and retained limits.

### Pinned behavioral source

- `packages/bitcoin-knots/src/kernel/chainparams.cpp` — network constants, deployments and script exceptions.
- `packages/bitcoin-knots/src/consensus/params.h` — deployment and network parameter semantics.
- `packages/bitcoin-knots/src/validation.cpp` — GetBlockSubsidy, ContextualCheckBlockHeader, ContextualCheckBlock, GetBlockScriptFlags, ConnectBlock and DisconnectBlock.
- `packages/bitcoin-knots/src/deploymentstatus.cpp` — candidate deployment boundaries.
- `packages/bitcoin-knots/src/versionbits.cpp` — versionbits history state.
- `packages/bitcoin-knots/src/pow.cpp` — retarget and minimum difficulty.
- `packages/bitcoin-knots/src/consensus/tx_verify.cpp` — maturity, finality and sequence locks.
- `packages/bitcoin-knots/src/coins.cpp` — overwrite/undo semantics.

### Repository contracts

- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards-overrides.md` — workflow and narrow exceptions.
- `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md` — pure core, typed invariants, behavioral testing and native verification.
- `standards/languages/rust.md`, `standards/languages/typescript-javascript.md` — language and automation contracts.
</canonical-refs>

<code-context>
## Existing Code Insights

- `packages/open-bitcoin-consensus/src/context.rs` exposes generic caller-selected ConsensusParams; canonical use needs derived authority.
- `packages/open-bitcoin-consensus/src/chain_spec.rs` separates identity from ordinary-history support and currently refuses all ordinary history.
- `packages/open-bitcoin-network/src/header_store.rs` and its contextual helpers own branch-specific ancestry and exact-work header admission.
- `packages/open-bitcoin-chainstate/src/engine.rs` owns transactional coins/undo and branch contexts; reuse this owner rather than bypassing it.
- `packages/open-bitcoin-node/src/network/canonical_admission.rs`, `packages/open-bitcoin-node/src/sync/open_runtime.rs` and `packages/open-bitcoin-node/src/sync/canonical_genesis.rs` provide canonical admission, durable startup and genesis integration seams.
- `packages/open-bitcoin-rpc/src/context/canonical_chain.rs` and `packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs` enforce product adapter scope.
</code-context>

<specifics>
## Specific Ideas

Use default-regtest serialized accepted histories across both halving boundaries with ordinary maturity, exact expected rewards and unchanged-state negative controls. Historical script exceptions and duplicate coinbases require exact pinned ancestor/hash facts, with fixture evidence explicitly classified. Reuse existing offline guard and parity conventions without importing unrelated historical workflow machinery.
</specifics>

<deferred>
## Deferred Ideas

Phase168 actual pinned-daemon build/runner and differential qualification; Phase169 differential branch/recovery; Phase170 prune/retained-BASIC integration; Phase171 classified historical evidence and parity closure. No new wallet product, public-network defaults, automatic legacy repair/import or production/service/funds claims. No pending todos matched.
</deferred>


## Plan47 submission observation prerequisite

[Plan47](167-47-PLAN.md) and the [Plan13 observation contract](167-47-SUBMISSION-OBSERVATION-AMENDMENT.md) add a read-only node predecessor after accepted30/35, before13. D-01/D-07/D-08/D-09/D-10 authority/effect/evidence rules remain unchanged. The exact earned Regtest command exposes detached complete facts through ordinary inferred API use; unprojected row correspondence, original stamp/catalog/current-source checks and unlock-before-fold are mandatory. At306 retained headers, an absent queried hash is observational input, not new admission. All old decisions and tasks/argv remain exact; global CVLD/whole167/STOP168 gates stay OPEN. Planning publication supplies no source lease or runtime evidence. See the full independent technical review and publication gate in [PLAN-CHECK](167-PLAN-CHECK.md).


## Plan15 pure serialization and frozen-byte ownership

[Plan15](167-15-PLAN.md) and the [serialization refinement](167-15-SERIALIZATION-REFINEMENT-AMENDMENT.md) explicitly allocate its pure helper as the fifth Task1 file/ninth whole-plan owner. Exact Buffer equality of BOTH complete UTF-8 pretty JSON plus one LF files is required; object equality cannot substitute. Preserve all16 existing cases, earn actual BOM/nonUTF8/LF/order/whitespace evidence for both file kinds and derive any additional count from execution. All ordinary301/150-and301 reopen/next302 and [15/16 context](167-15-16-CONTEXT-EVIDENCE-AMENDMENT.md) obligations remain. No new plan/task/DAG/count or source acceptance; all global CVLD/whole167/STOP168 gates remain OPEN.


## Checked R7 typed owned-payload precision | 2026-10-24T03:57:45.890943+00:00

Checked R7 owned-payload planning publication 2026-10-24T03:57:45.890943+00:00: Plan46 corrects exactly three already-owned authority/test files; the pending Plan45 request child must move the same boxed peer snapshot with `Some(*peer)`. The other twelve inert bodies are exact; all tasks, argv, owners, dependencies, waves and counts remain unchanged. Independent technical/precision review passes and copied formatting/static fit measures619/145 with zero deltas; these are not compilation/runtime/source acceptance. Actual46 remains on v3 shape HOLD after12 exit0 commands/255 overlapping passes; the correction is unapplied. A separately checked R7 anchor/loader, fresh three-owner lease and both Tasks/all ten fresh checks precede any nonfinal dependency receipt. Originalde637/expanded498 and exclusive35before5b0 remain exact; native201 plus separate Byteview1 stay distinct. Counts47plans106tasks37waves/34accepted/milestone104of11789%/3of8 verified and all five global CVLD requirements remain OPEN. Actual45 joint strict, full35 and all later167/source-before17/security/parity/docs/native/coverage/Bazel/lifecycle/normal-hook MAINpush gates remain;STOP168.


## public-chain-information-split-candidate | 2026-10-24T06:15:23.208247+00:00

Planning-only split proposed for independent/Root review:13 core/registry/public facts11owners/2tasks, new48 original daemon/process5owners/1task after13 and before14; one shared history-test owner is sequential. Full original Task2 XML/four argv move byte-exact; originalTask1/prior amendedTask1b XML and all8 prior13 argv remain exact across13+48.15-owner cross-plan union is unchanged. Proposed inventory48plans106tasks38waves versus current47/106/37; existing pending14–20+25 waves shift+1 and14 keeps13 plus adds48. Public factual interface/corrected witness76–103/directKnots1791 evidence and full no-scope-reduction/global gates are in167-13-PUBLIC-CHAIN-INFORMATION-AMENDMENT.md. Eleven-fileTask1b remains a disclosed scope red flag requiring separate independent/Root adjudication and demonstrated normal fit; no source/acceptance/lease/phase168 claim. Future unpublished compact CDE IDs48/49/50 must become49/50/51 before future publication, outside this proposal. All current accepted source/research/Summary pairs and original35/45/46 bodies stay exact; all five globalCVLD remainOPEN. No actual35/47 API acceptance assumed. Whole167 source-before17/security18/parity19/docs25/ENTIREnative20/freshpureCorecoverage/Bazel/lifecycle/normal-hookMAINpush/STOP168 remain mandatory.


## Canonical C/D/E planning publication | 2026-10-27T17:56:10.370868+00:00

Plans49/50/51 and the [compact lifecycle companion](167-49-51-COMPACT-LIFECYCLE-AMENDMENT.md) are canonical planning metadata after actual accepted Plan35 source receipt .local/open-bitcoin-dev/phase167/plan35/root-task3-correction-v1/r2-final-source-acceptance.json. The receipt carries accepted45/46 prerequisites. The new dependency chain is 16→49→50→51→17, with 17 retaining 16/34 and adding only51; waves17/18/19/25/20 are 37/38/39/40/41. This publication adds three plans and twelve planned tasks to the live 48-plan/107-task/38-wave frame, yielding 51 plans/119 tasks/41 waves. All existing Task XML and ordered automated commands remain exact. The full before/after/inverse and true-absence proof is under .local/open-bitcoin-dev/phase167/.compact-lifecycle-design/canonical-cde-publication-v1. Source execution remains HELD; all61 new C/D/E task/final commands are UNRUN. Actual source/API/normal-fit/fault reachability, complete source/native frame, original inverse, and separate release remain unproved. Independent whole167 source review precedes17;18 security/native/mutants, 19 parity/breadcrumb/docs,25 goal/writing-for-agents and20 full default native/fresh coverage/Bazel/lifecycle/normal-hook MAIN gates remain. All five global CVLD requirements remain OPEN; STOP168.
