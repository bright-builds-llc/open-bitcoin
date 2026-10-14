# Project Research Summary

**Project:** Open Bitcoin — v2.6 Canonical Chain Bootstrap and Validation Parity
**Domain:** Brownfield headless Bitcoin node; canonical authority and bounded validation parity
**Researched:** 2026-10-13
**Confidence:** MEDIUM overall; HIGH for inspected source facts
**Baseline:** Bitcoin Knots `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`

## Executive Summary

Open Bitcoin must replace its synthetic root, work and fixed-rule assumptions with one canonical validation authority across existing first-party crates. The six accepted deliverables belong to one dependency chain: canonical identity, history-derived rules and exact work enable genuine differential validation, which enables meaningful fork/recovery and prune/BASIC evidence. The recommended scope is complete default-regtest ordinary-path scenarios, finite classified mainnet historical contexts, and truthful support/refusal for existing selectors. Finite results cannot establish exhaustive consensus parity or production readiness.

Add no production dependency and no workspace crate. Reuse the existing `ProofWork`, codecs, chainstate overlays, managed lifecycle owner, Fjall storage and BASIC protection. Place canonical specifications and candidate-branch decisions in the pure core; keep history loading, clocks, storage and process control in adapters. Gate incompatible datadirs before automatic schema/coins migration or other recovery effects. Bootstrap exact genesis body/header and coins best-block, with no spendable genesis output, before BASIC initialization. Promote the existing 256-bit work representation through every operative consumer, preserving pinned modulo arithmetic.

The principal risk is apparent correctness from incomplete integration or overstated evidence: correct display work alongside synthetic selection, modern flags applied to historical branches, widened legacy scores treated as provenance, or helper results called differential acceptance. Require an actual source-built, identity-verified Knots daemon to receive identical serialized candidates under ordinary rules. Preserve raw outcomes and resulting authority through forks, actual all-handle reopen and paired pruning. Routine offline verification may replay qualified corpora without fetching or rebuilding Knots each time; affected source, runner, corpus or build changes must invalidate qualification and require a fresh actual run before closure. Research performed no new build, oracle execution or runtime acceptance.

## Key Findings

### Recommended Stack

[STACK.md](STACK.md) recommends integration within the pinned stack, with a test-only CMake-built reference daemon. Exact encoding and provenance are more important than adding numerical or Bitcoin libraries.

**Core technologies:**

- **Rust 1.94.1 / edition 2024:** existing pure consensus, primitives, codecs, network and chainstate modules own all production Bitcoin behavior.
- **Existing `ProofWork([u64; 4])`:** one target-derived exact work type; fixed 32-byte encoding and canonical hexadecimal projection must be lossless and explicit.
- **Fjall 3.1.10:** retain the single active chainstate, coins fences and recovery owner; version canonical authority independently from unrelated wallet/mempool blobs.
- **Bun 1.3.9:** bounded test orchestration, corpus manifests, process receipts and qualification checks; there is no `package.json` bootstrap.
- **Bazel 8.6.0 / rules_rust 0.69.0:** retain Bzlmod/Cargo synchronization and the repo-native smoke build.
- **Pinned Knots daemon:** independent ordinary-validation oracle, outside production dependencies. Provision with CMake 3.22+, C++20 compiler, Boost and libevent; record actual inputs and executable digest.

### Expected Features

[FEATURES.md](FEATURES.md) treats all six accepted deliverables as essential; no additional product expansion is needed.

**Must have:**

- Canonical existing-network identity, exact genesis initialization, empty genesis coins and safe incompatible-state refusal.
- Candidate-branch historical rules, pinned script exceptions, correct difficulty/time, subsidy and coinbase maturity.
- Exact work across headers, connected branches, reorg, durable records, recovery and operator projections; callers cannot mint authority.
- Actual pinned-daemon differential accept/reject and post-state comparisons using identical bytes and ordinary default regtest.
- Competing branches, disconnect/reconnect, rejected-state preservation, checkpoint interruption and real close/reopen.
- Canonical legal pruning with retained branch-correct BASIC, honest body/wallet availability and finite historical-context evidence.

**Should have within those deliverables:** reproducible source/build/corpus receipts, explicit evidence classes, one typed validation authority and actionable nonmutating compatibility diagnostics.

**Defer:** testnet4, custom Signet, arbitrary activation overrides, assumeutxo/assumevalid shortcuts, dual chainstate, automatic legacy repair/import, archive scale, wallet expansion, V0/BIP37, public defaults and production/service/funds claims.

### Architecture Approach

[ARCHITECTURE.md](ARCHITECTURE.md) recommends one immutable selected-chain specification and one branch-derived context consumed by ordinary managed validation. Distinguish header-valid, body/script-valid, active, checkpointed and recovered facts; exact arithmetic cannot prove body validity. Preserve accepted receipts and conservative BASIC/prune fences instead of introducing another authority.

**Major components:**

1. **Consensus and codecs:** canonical genesis, immutable network rules, historical/deployment decisions, exact proof and bounded serialized input.
1. **Header store:** admitted ancestry, contextual header checks, cumulative proof and observable candidate ordering; best header remains separate from active coins.
1. **Chainstate:** canonical genesis transition, contextual connect/disconnect and replacement staging on existing coins overlays.
1. **Node and Fjall adapters:** read-only compatibility gate, managed startup, versioned authority, checkpoint/recovery, retained bodies/undo and BASIC protection.
1. **Existing consumers and test runner:** RPC/CLI/dashboard/support project achieved facts; Bun coordinates isolated Knots/Open Bitcoin processes and auditable observations.

### Critical Pitfalls

1. **Wrong root or genesis semantics:** bind exact selected genesis; retain its body/BASIC script inputs while skipping genesis spendable outputs and ordinary undo assumptions.
1. **Rules detached from candidate history:** resolve branch-specific context, exact hash exceptions and pinned `GetBlockScriptFlags`; separate consensus from policy and refuse missing prerequisites.
1. **Split or fabricated work authority:** remove operative count/`u128`/caller scores everywhere; preserve modulo-`2^256` cumulative arithmetic, compact-target checks and ancestry-bound recovery.
1. **Mutation before compatibility or durable proof:** inspect identity/provenance before schema migration, prune resume or index writes; partial bootstrap/checkpoints must recover a proved prefix or refuse unchanged.
1. **False differential or integration claims:** source helpers and frozen expectations cannot replace actual daemon qualification; retain rejection categories, real reopen, paired deletion and original v2.5 evidence limits.

## Implications for Roadmap

Recommend eight phases beginning at 164. Requirement references below suggest ownership; later integration evidence must also exercise earlier foundations.

### Phase 164: Canonical Chain Identity and Compatibility Gates

**Depends on:** shipped v2.5 foundations; no new milestone phase.
**Rationale:** mutable open currently includes migration/recovery effects, so identity checks must precede all authority changes.
**Delivers:** pure existing-network specifications, exact genesis fixtures, supported/refused validation matrix and nonmutating wrong-root/network/legacy checks.
**Addresses:** CBOT-01/04, CVLD-05; canonical identity and explicit unsupported-mode refusal. Establish the read-only inspection seam needed by CWOR-05 in 165.
**Avoids:** synthetic roots, schema widening as provenance, unsupported Signet acceptance and implicit testnet4/custom profiles.

### Phase 165: Shared Exact Work Authority

**Depends on:** 164.
**Rationale:** new canonical persistence needs the final work representation rather than another temporary numeric schema.
**Delivers:** existing `ProofWork` across live headers, connected candidates and reorg inputs; lossless encoding, target/proof vectors above 128 bits and pinned selection rules.
**Addresses:** CWOR-01/02/03/05; target-derived work, elimination of caller/synthetic authority and legacy-work refusal through 164's inspection seam.
**Avoids:** saturation, count-based ranking, diagnostic-only exact work and header validity treated as accepted bodies. Contextual admission is completed by 167.

### Phase 166: Canonical Durable Bootstrap and Genesis BASIC

**Depends on:** 164–165.
**Rationale:** root, exact work and versioned authority must become one recoverable startup before ordinary validation/indexing.
**Delivers:** fresh/idempotent/interrupted genesis initialization, empty genesis UTXOs, exact recovered work and BASIC startup after retained canonical history exists.
**Addresses:** CBOT-02/03, CWOR-04; normal empty-datadir bootstrap and proved new-format reopen.
**Avoids:** genesis spending, nonexistent genesis undo requirements, index-before-history ordering and partial authority publication.

### Phase 167: Candidate-History Validation Rules

**Depends on:** 164–166.
**Rationale:** genuine-chain evidence requires rules selected by validated candidate ancestry throughout header and body paths.
**Delivers:** subsidy/maturity, buried/contextual version rules, historical script/hash exceptions, BIP30/overwrite restoration, deployments, difficulty, finality/sequence and witness decisions.
**Addresses:** CVLD-01/02/03/04/06; default-regtest maturity 100 and halving boundaries 149/150/299/300.
**Avoids:** blanket activation switches, P2SH-only runtime flags, displaced-tip contexts and permissive missing-history defaults. Define mainnet fixture prerequisites here.

### Phase 168: Actual Pinned-Knots Differential Qualification

**Depends on:** 164–167 for acceptance; isolated runner/provisioning design can begin after 164.
**Rationale:** shared canonical production authority is the prerequisite for an independent ordinary-validation comparison.
**Delivers:** build/executable receipts, bounded owned processes, identical-byte canonical regtest corpus through maturity and both halvings, raw outcome normalization and post-state observations.
**Addresses:** CDIF-01/02/03/04/05; actual daemon qualification is mandatory, including unavailable/failure handling.
**Avoids:** helper-only proof, altered rule profiles, ambient datadirs/public peers, hidden assumevalid shortcuts and transport faults counted as rejections.

### Phase 169: Differential Fork Selection and Durable Recovery

**Depends on:** 165–168.
**Rationale:** reuse the qualified runner to test authority transitions rather than merely arithmetic or successful linear connects.
**Delivers:** competing branches, defined equal-work arrivals, higher-work activation, invalid ancestry/replacement controls, disconnect/reconnect and checkpoint/all-handle-reopen fault cases.
**Addresses:** CREC-01/02/03/04; coins, tip, work and dependent progress agree after transitions or refuse explicitly.
**Avoids:** height/hash substitutes for defined arrival behavior, pointer-based tie copying, rejected-state mutation and a cloned store handle labeled restart.

### Phase 170: Canonical Prune and Retained BASIC Integration

**Depends on:** 166–169.
**Rationale:** shipped filter/retention code must be proved on genuine canonical rules and recovered branch authority.
**Delivers:** continuous default-regtest history beyond ordinary prune eligibility, protected indexing, actual paired body/undo deletion and retained RPC/peer results after reopen.
**Addresses:** CINT-01/02/03; shared RPC/CLI/dashboard/redacted-support canonical tip/work/index/retention facts.
**Avoids:** fastprune/synthetic substitutes, unsafe index progress, deleted-body restoration claims and filters restoring wallet eligibility.

### Phase 171: Classified Historical Contexts and Parity Closure

**Depends on:** 167–170; mainnet context/corpus design begins in 167.
**Rationale:** closure must integrate actual results, independent historical expectations and precise residual limits across all six deliverables.
**Delivers:** finite mainnet fixtures with named ancestry/UTXO/deployment prerequisites, evidence manifests, source-linked parity inventory, qualification invalidation and offline native registration.
**Addresses:** CINT-04, CPAR-01/02; current claims and lifecycle owners remain auditable alongside unchanged historical receipt meanings.
**Avoids:** sparse samples promoted to whole-chain validation, stale qualification, unsupported-network claims and rewriting v2.5 evidence as canonical proof.

### Phase Ordering Rationale

- The strict acceptance path is **164 → 165 → 166 → 167 → 168 → 169 → 170 → 171**; parallel runner planning and historical fixture selection do not remove these correctness dependencies.
- Keep canonical activation gated until identity, work encoding, bootstrap and ordinary rules converge; never expose a partially migrated authority as completed support.
- Defined process-observable equal-work arrival/activation must match the pin. Only truly ambiguous internal/disk-load/restart ties may remain explicitly bounded.
- Keep small pure vectors, ordinary-chain differential, seeded historical contexts and storage fault probes as distinct evidence classes. Each proves its declared boundary.

### Research Flags

**Needs focused phase research:** 164 (pre-migration inspection), 165 (all work carriers/selection), 166 (bootstrap crash ordering/genesis BASIC), 167 (pinned contextual/deployment inventory), 168 (build provenance and bounded runner), 169 (observable ties/recovery), 170 (canonical corpus budget/protection), 171 (historical prerequisites and qualification invalidation).

**Standard patterns to reuse without separate broad research:** existing pure-core types/overlays, accepted-transition owner, Fjall coins fences, BASIC construction/serving and operator projections; formatter/native-check registration after the new evidence contract is fixed. Their changed canonical seams still need focused research above.

## Confidence Assessment

| Area         | Confidence                                        | Notes                                                                                                              |
| ------------ | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ |
| Stack        | HIGH                                              | Pins and existing facilities inspected; daemon prerequisites source-backed, host build unqualified.                |
| Features     | HIGH for scope; MEDIUM for acceptance design      | Six deliverables accepted; concrete corpus and network evidence boundaries need planning.                          |
| Architecture | MEDIUM                                            | Existing seams and mismatches are clear; compatibility/crash ordering and cross-layer integration remain proposed. |
| Pitfalls     | HIGH for hazards; MEDIUM for mitigation execution | Local/pinned source and archived limits agree; new runtime proof is absent.                                        |

**Overall confidence:** MEDIUM. Proceed to roadmap definition with explicit phase research and execution gates.

### Gaps to Address

- **Compatibility inspection:** CBOT-04/CWOR-05 now explicitly require pre-migration refusal; phase planning must enumerate every open-time write and implement the read-only seam.
- **Historical rule inventory:** CVLD-02 now names BIP65/BIP66 block-version constraints and validated deployment history. Inventory exact ancestor facts, script/BIP30/hash exceptions and testnet3/signet refusal boundaries during planning.
- **Oracle qualification:** CDIF-01 now binds executable SHA-256 to source tree and successful build inputs/options, with networking and assumevalid disabled. Choose host prerequisites, cache provenance and qualification execution bounds.
- **Receipt invalidation:** CPAR-02 now invalidates affected qualification on rule, runner/normalization, corpus or oracle build-input changes. Implement precise source dependency fingerprints and rerun gates; offline replay remains distinct.
- **Recovery/ties:** inventory authority carriers and fault points; measure defined equal-work activation and distinguish genuinely ambiguous restart behavior without broad parity exemptions.
- **Corpus bounds:** select finite mainnet contexts, prune-chain size, retained branches, clock policy, deadlines/log caps and cleanup checks. Source-derived historical predicates cannot be relabeled ordinary full-chain acceptance.
- **Verification ownership:** parent workflow owns final hook/native verification and commits. This synthesis supplies no runtime result and requests no scope expansion.

## Sources

### Primary (HIGH confidence)

- [STACK.md](STACK.md), [FEATURES.md](FEATURES.md), [ARCHITECTURE.md](ARCHITECTURE.md), [PITFALLS.md](PITFALLS.md) — inspected-source findings, pin/toolchain facts and detailed source inventories.
- [PROJECT.md](../PROJECT.md), [draft requirements](../REQUIREMENTS.md), [v2.5 audit](../milestones/v2.5-MILESTONE-AUDIT.md) — accepted scope, proposed obligations and preserved historical limits.
- [Pinned chain parameters](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp), [validation](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp), [difficulty](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/pow.cpp) — canonical identity and history/context decisions.
- [Pinned proof](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp), [arithmetic](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/arith_uint256.h), [block index/candidates](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp) — work semantics and selection.
- [Pinned filter construction](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.cpp), [filter index](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp), [mining RPC](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/mining.cpp) — genesis BASIC and ordinary submission outcomes.
- [Pinned dependencies](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/doc/dependencies.md), [CMake options](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/CMakeLists.txt) — source-built oracle prerequisites.
- Local `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder overrides, standards index and architecture/verification/testing pages — material workflow, pure-core, preservation and evidence guidance. Parent loaded active lessons completely.

### Design Inferences (MEDIUM confidence)

- Eight-phase decomposition, exact ownership suggestions, corpus/runtime budgets and qualification policy are synthesis recommendations requiring planning and implementation evidence.
- BIP specifications linked in the detailed reports provide context; the exact pinned implementation governs this milestone. No tertiary claim is needed.

*Research completed: 2026-10-13. Ready for roadmap: yes. Actual v2.6 qualification: not performed.*
