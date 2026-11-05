---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 167-2026-10-21T16-38-23
generated_at: "2026-10-21T16:58:00Z"
---

# Phase 167: Candidate-History Validation Rules - Research

**Researched:** 2026-10-21
**Domain:** Pinned Bitcoin contextual consensus, validated branch ancestry, durable coins
**Confidence:** HIGH for pinned rules and existing seams; MEDIUM for implementation decomposition

<user-constraints>
## User Constraints (from CONTEXT.md)

The following decisions, discretion and deferred scope are copied verbatim from the current phase context. [VERIFIED: .planning/phases/167-candidate-history-validation-rules/167-CONTEXT.md]

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

Phase168 actual pinned-daemon build/runner and differential qualification; Phase169 differential branch/recovery; Phase170 prune/retained-BASIC integration; Phase171 classified historical evidence and parity closure. No new wallet product, public-network defaults, automatic legacy repair/import or production/service/funds claims. No pending todos matched.
</user-constraints>

<phase-requirements>
## Phase Requirements

| ID | Description | Research Support |
| --- | --- | --- |
| CVLD-01 | A contributor observes the pinned subsidy schedule for each supported validation mode, including default-regtest halvings at heights 150 and 300 and ordinary coinbase maturity. | Selected immutable parameters, actual ordinary 301-block corpus and maturity negatives. |
| CVLD-02 | A contributor observes contextual block and transaction decisions derived from the selected network and validated deployment history, including BIP34 height, BIP65/BIP66 block-version constraints, BIP113/sequence-lock context and Segwit commitment rules, without caller-selected blanket activation switches. | Candidate history proof; buried activation; shared contextual rules; branch/UTXO lock-time facts. |
| CVLD-03 | A contributor observes pinned GetBlockScriptFlags behavior, including its baseline flags, block-hash exceptions and deployment-dependent additions, through the actual durable connect path. | Exact replacement-before-addition derivation; durable script and contextual sigops evidence. |
| CVLD-04 | A contributor observes pinned network difficulty and contextual time decisions at supported retarget, minimum-difficulty and median-time boundaries; header admission and body connection use the same validated network authority. | Linked branch view, 11-sample MTP, immutable difficulty, contextual header authority and missing-history negatives. |
| CVLD-06 | A contributor observes pinned duplicate-coinbase/BIP30 and coin-overwrite behavior in the supported historical contexts, with the required ancestor/hash facts and correct disconnect restoration rather than a global permissive exception. | Exact repeats, parent BIP34 ancestor, recheck boundary, transactional overwrite undo, explicit pinned disconnect difference. |

Descriptions transcribe REQUIREMENTS.md; all five are independently mandatory roadmap criteria, not optional examples. [VERIFIED: .planning/REQUIREMENTS.md:27-32; .planning/ROADMAP.md Phase167]
</phase-requirements>

## Summary

Implement one selected-network rule authority whose inputs come from the candidate's validated parent chain. Reuse existing exact work, canonical genesis, staged coins overlays, accepted-validation publication and BASIC owners. The existing public ConsensusParams and ScriptVerifyFlags are useful fixture carriers but cannot authorize canonical acceptance. HeaderProofFacts currently establishes arithmetic, not contextual validity; imported entries and raw ChainPosition values likewise grant no canonical ancestry. [VERIFIED: packages/open-bitcoin-consensus/src/context.rs; packages/open-bitcoin-network/src/header_store/proof.rs; packages/open-bitcoin-chainstate/src/types.rs; packages/open-bitcoin-node/src/chainstate/validation_history.rs]

Three integration defects must be fixed before removing ordinary refusal: small-height BIP34 encoding uses a data push instead of CScript's OP_1–OP_16; chainstate MTP takes eleven existing times and then appends a twelfth; actual overlay connection never invokes the aggregate contextual sigops check present in validate_block_with_context. Reuse/factor those rules into the actual owner rather than adding a second validator. Existing split_sigop_cost also counts P2SH without checking the P2SH flag, unlike the pin. [VERIFIED: packages/open-bitcoin-consensus/src/block.rs:354-414,461-503; packages/open-bitcoin-chainstate/src/engine.rs:523-545; packages/open-bitcoin-chainstate/src/engine/apply.rs; packages/bitcoin-knots/src/consensus/tx_verify.cpp:GetTransactionSigOpCost]

The main design risk is confusing a supported implementation with proved candidate facts or actual daemon qualification. Earn ordinary default-regtest acceptance from Phase166 genesis first; support mainnet/testnet only where every required fact is proved, and classify independently seeded historical envelopes separately. Keep Signet ordinary admission refused. Additional pinned contextual prerequisites include checkpoint behavior, future time and Knots software expiry; these must be implemented, explicitly scoped, or cause refusal before broader support claims. [VERIFIED: phase167 CONTEXT D-01,D-05,D-07; packages/bitcoin-knots/src/validation.cpp:ContextualCheckBlockHeader; .planning/ROADMAP.md scope gates]

**Primary recommendation:** Build restricted candidate-history rules and wire them through real header ingress, staged durable connect/reorg and bounded proved reopen before updating support and claims.

## Project Constraints (from AGENTS.md)

- Preserve Knots29.3.knots20260210 at a9aee730466ac67d35a3c03ee24676be5e045878; record deliberate differences in docs/parity/index.json plus companion docs. First-party production Bitcoin domain implementation; no new production dependency or workspace crate. [VERIFIED: AGENTS.md Project/Conventions; phase167 CONTEXT]
- Keep pure decisions in consensus/network/chainstate; filesystem, time capture, storage and process control stay in node/RPC/CLI shells. Use private constructors/newtypes, early returns, maybe_ optional names, foo.rs plus foo/; no new unwrap/panic allowance. Split coherent modules before expanding central files. [VERIFIED: AGENTS.bright-builds.md; standards/core/architecture.md; standards/core/code-shape.md; standards/languages/rust.md; standards-overrides.md]
- Bun owns substantial automation; preserve the existing narrow Python BASIC oracle exception. No package.json/bootstrap install is required. [VERIFIED: AGENTS.md Repo-Local Guidance/Technology Stack; standards/languages/typescript-javascript.md]
- Register parity breadcrumbs for every new first-party Rust source/test file through docs/parity/source-breadcrumbs.json and scripts/check-parity-breadcrumbs.ts. Update relevant contributor READMEs after the feature. [VERIFIED: AGENTS.md Repo-Local Guidance]
- Use bash scripts/verify.sh as full offline completion/pre-commit contract, including fresh pure-core coverage and Bazel. Cargo/Bazel ad-hoc jobs use scripts/command-timings.ts and never overlap Cargo target ownership. Keep normal hooks, tracked generated LOC and complete failure/outcome evidence. [VERIFIED: AGENTS.md; standards/core/verification.md; phase167 D-10]
- Unit tests cover pure rules, one concern each, with explicit Arrange/Act/Assert comments; actual durable integration must verify observable effects and refusal preservation. [VERIFIED: standards/core/testing.md; phase167 D-08]
- Root owns shared task/state/requirements/catalog reconciliation and all commits at the strict lifecycle gate. This research owns only this file. No body standalone triple-dash separators. [VERIFIED: parent research assignment; AGENTS.md GSD Workflow Enforcement; supplied global instructions]
- Both active lesson files were read fully:6898 global bytes +1958 repo bytes=8856 bytes,2953 conservative tokens. Relevant lesson: show explicit repo-local Cargo/Bazel UAT commands. No project skill files were found in .claude/skills or .agents/skills. [VERIFIED: startup filesystem probes; .codex/tasks/lessons.md]

## Standard Stack

| Component | Pinned/current repository version | Purpose | Planning decision |
| --- | --- | --- | --- |
| Rust/edition | 1.94.1 /2024 | Existing pure core and adapters | Retain pins; rustc1.94.1 was available. [VERIFIED: rust-toolchain.toml; packages/Cargo.toml; rustc --version] |
| First-party consensus/network/chainstate/node | Workspace0.1.0 | Existing validation, ancestry, transactional coins, durable publication | Extend existing crates; no new crate. [VERIFIED: packages/Cargo.toml; phase167 CONTEXT] |
| Fjall | 3.1.10 at aa30dca811399a201e0b9595da93a4582dcb2b57 | Existing single durable coins owner | Reuse staged overlays/checkpoints and compatibility inspection. [VERIFIED: packages/Cargo.lock; storage/fjall_store.rs] |
| Bun | Repository1.3.9; PATH observed1.4.2 | Existing scripts/guards | Root located cached1.3.9; prepend its bin for verification. Do not upgrade pins. [VERIFIED: .bun-version; bun --version; parent environment follow-up] |
| Bazel | .bazelversion8.6.0; bazel/bazelisk present | First-party smoke/provenance | Run through native verifier. Executed Bazel version was not probed in research. [VERIFIED: .bazelversion; command -v] |

**Installation:** None. This phase adds no package; registry/latest-version and publish-date research would conflict with the locked existing stack. Retain Cargo.lock/Bzlmod. [VERIFIED: phase167 CONTEXT Agent Discretion; AGENTS.md]

### Don't Hand-Roll

| Problem | Use existing owner instead | Reason/source |
| --- | --- | --- |
| Work/proof ordering | consensus chain_proof plus header eligibility and chainstate work_authority | Existing full256-bit modulo work, generation/stage binding and tie authority must survive. [VERIFIED: phase165 CONTEXT; network/header_store/eligibility.rs; chainstate/engine/work_authority.rs] |
| Durable transactions/rollback | CoinsOverlay, staged connect/reorg, ManagedChainstate | Rejection before overlay absorption is already the mutation boundary. [VERIFIED: chainstate/engine/overlay_apply.rs; engine/stage.rs] |
| Accepted body/BASIC lineage | AcceptedValidationReceipt and current index owner | Header validity is not script/coins validity; publish only genuine accepted receipts. [VERIFIED: node/chainstate/validation_history.rs; node/network/runtime_authority/filter_index/] |
| Safe reopening | Existing lease + copied inspection worker + immediate original reproof | Raw Fjall opening performs effects; a generic runtime constructor cannot replace pre-effect compatibility proof. [VERIFIED: phase166 VERIFICATION; node/storage/fjall_store/inspection.rs] |
| Historical oracle | Fixed source-derived expected vectors plus serialized corpus | Local source-derived evidence must never become an actual-daemon receipt. [VERIFIED: phase167 D-09; ROADMAP Phase168] |

## Pinned Rule Inventory

Unless stated otherwise, source paths below are under packages/bitcoin-knots/src at the exact pin, independently checked by git rev-parse and clean submodule status. Local validation.cpp bytes equal git show HEAD:src/validation.cpp (SHA256 5bb31c7417ef3fe06a2afefee8a1c32607aed737fdd623368cdc4be72d23d495). Treat local pinned bytes as authoritative over web renderings. [VERIFIED: git/submodule and SHA256 probes]

### Immutable selected-network facts

| Selector | Halving | BIP34 / BIP66 / BIP65 / CSV / Segwit heights | PoW and ordinary disposition |
| --- | --- | --- | --- |
| Mainnet |210000|227931 /363725 /388381 /419328 /481824| powLimit compact1d00ffff;600s spacing,14days; no min difficulty/no-retarget=false. Enable only ancestry-proved required context. |
| Testnet3 |210000|21111 /330776 /581885 /770112 /834624| same compact/spacing/timespan; allow min difficulty, retarget=true. Enable only proved context. |
| Default Signet |210000|1 /1 /1 /1 /1| compact1e0377ae;600s/14days; no min difficulty; ordinary refusal until challenge validation. |
| Default Regtest |150|1 /1 /1 /1 /0| compact207fffff;600s/one day interval144; min difficulty=true,no-retarget=true; canonical ordinary evidence target. |

All values are source-derived; signet disposition comes from the locked phase decision. Coinbase maturity is100 for all four, not a per-fixture override. Default regtest BIP94 is false; do not inherit the source comment mentioning optional regtest BIP94 as a default requirement. [VERIFIED: kernel/chainparams.cpp CMainParams/CTestNetParams/SigNetParams/CRegTestParams; kernel/chainparams.h RegTestOptions; consensus/consensus.h; phase167 D-01,D-02]

The current canonical_genesis::consensus_params only overrides three PoW fields over generic defaults; it leaves regtest halving210000 and timespan14days. Replace its canonical parameter authority from ChainSpec, including interval144, rather than changing global fixture defaults and laundering fixture ownership. [VERIFIED: packages/open-bitcoin-chainstate/src/engine/canonical_genesis.rs:consensus_params; packages/open-bitcoin-consensus/src/context.rs:Default]

### Contextual activation, locks and witness

- Buried DeploymentActiveAfter uses parent.height+1; ActiveAt uses candidate.height. BIP34/BIP65/BIP66/CSV/Segwit are buried in this pin. Required block versions are >=2/>=3/>=4 for BIP34/BIP66/BIP65 respectively, including header admission. [VERIFIED: deploymentstatus.h; consensus/params.h; validation.cpp:4570-4575]
- BIP34 compares the exact prefix CScript()<<height; heights1–16 use OP_1–OP_16. Coinbase script length still must satisfy CheckTransaction, so append sufficient data to tiny-height fixture coinbases. Include17,127,128 and signed-byte transitions. [VERIFIED: validation.cpp:4622-4631; script/script.h:push_int64; consensus/tx_check.cpp; first-party block.rs:serialized_script_num]
- BIP113 selects parent MTP only when CSV active, otherwise candidate block time. Finality uses strict less-than or all sequences final. BIP68 requires tx.version>=2 and CSV-derived lock flags; disabled/type/mask/granularity semantics remain separate from policy. [VERIFIED: validation.cpp:ContextualCheckBlock/ConnectBlock; consensus/tx_verify.cpp:IsFinalTx/CalculateSequenceLocks]
- Sequence input time is MTP of the candidate ancestor at max(coin.created_height-1,0), not source block timestamp, active-tip MTP or unproved saved metadata. Existing Coin.created_median_time_past can remain a cached value only if earned from that branch and checked on recovery. Same-block input creation uses candidate parent's MTP. [VERIFIED: consensus/tx_verify.cpp:CalculateSequenceLocks; first-party chainstate/engine/apply.rs:build_transaction_context/add_transaction_outputs]
- Witness commitment activation is SegwitHeight, independent of the baseline WITNESS script flag. Last matching commitment output wins; coinbase witness is exactly one32-byte reserved item; coinbase witness-merkle leaf is zero. Unexpected witness, malformed nonce, wrong commitment and excessive block weight reject. [VERIFIED: validation.cpp:CheckWitnessMalleation/ContextualCheckBlock; first-party block/witness.rs]

### GetBlockScriptFlags: use this exact order

1. Begin P2SH|WITNESS|TAPROOT.
2. Replace the entire baseline for a selected-network exact candidate hash.
3. Add DERSIG at BIP66, CHECKLOCKTIMEVERIFY at BIP65, CHECKSEQUENCEVERIFY at CSV, NULLDUMMY at Segwit. No policy flags are introduced.

This order is the pinned function, not a historical activation approximation. [VERIFIED: validation.cpp:2645-2684]

| Network/hash | Replacement |
| --- | --- |
| Mainnet 00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22 | NONE |
| Mainnet 0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad | P2SH|WITNESS |
| Testnet3 00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105 | NONE |

These are exact network-scoped hash exceptions; normal deployment additions still follow replacement. [VERIFIED: kernel/chainparams.cpp:90-93,255-256]

**Versionbits finding:** versionbits.cpp implements period/MTP/signalling state, but the block-consensus paths above do not consult Taproot BIP9 state: TAPROOT is baseline and all their deployment additions are buried. Do not implement a generic BIP9 engine or derive Taproot flag activation from a hardcoded activation height for this phase. Preserve the source distinction if deployment reporting is added later. [VERIFIED: validation.cpp complete DeploymentActive/TAPROOT reference search; deploymentstatus.h; versionbits.cpp:GetStateFor; consensus/params.h SegwitHeight comment]

### Difficulty and time

- Reuse next_work_required with a proved candidate parent, retarget anchor height(candidate.height-interval), and a branch walk to last non-minimum target or period boundary. The min-difficulty exception is strictly candidate.time>parent.time+2*spacing; equality recovers the prior target. Retarget timespan clamps to[one quarter,four times], computes from the previous target and caps at powLimit. Default regtest still performs non-boundary min-difficulty/recovery logic despite no retargeting. [VERIFIED: pow.cpp:GetNextWorkRequired/CalculateNextWorkRequired; first-party block/difficulty.rs]
- MTP uses current header and at most ten parents, with sorted upper median for short histories. Require min(11,height+1) linked samples; a truncated high-height seed is missing context, not a shorter valid MTP. Existing HeaderStore median_time_past can stop on a missing parent; ancestor_at_height follows maps without independently proving every height/link. Canonical context constructors must validate required linkage/provenance and distinguish true genesis from truncation. [VERIFIED: chain.h:CBlockIndex::GetMedianTimePast; first-party network/header_store.rs:308-344; chainstate/engine.rs:523-545]
- Header timestamp must exceed parent MTP and be at most shell-observed now+2hours. Knots does not re-run future-time admission on ConnectBlock; do not make stage_reorg's candidate timestamp stand in for now. Carry the trusted admission result/time, recheck branch-dependent facts at staging and avoid clock-reversal-induced false consensus invalidation of a previously admitted header. [VERIFIED: validation.cpp:ContextualCheckBlockHeader/ConnectBlock; first-party engine/stage.rs:stage_reorg]
- Additional Knots header gates: checkpoint hash at checkpoint heights, rejection below latest known checkpoint, and software expiry when candidate timestamp and MTP at parent minus min(144,parent.height) are expired. Expiry is strictly >g_software_expiry; default comes from COPYRIGHT_YEAR2026 formula. Recommend preserve default expiry as a pure predicate with shell-captured time/history, no product deployment overrides. Implement applicable checkpoint rules or keep affected mainnet/testnet ordinary contexts refused; regtest checkpoint is genesis only. [VERIFIED: validation.cpp:4517-4590; kernel/chainparams.cpp:checkpointData; clientversion.cpp:IsThisSoftwareExpired; clientversion.h; CMakeLists.txt]

### BIP30 and the disconnect distinction

Connect skips the duplicate-outpoint scan only for the two exact repeat height/hash pairs, or when the candidate parent's ancestor at BIP34Height equals the selected BIP34Hash; checking resumes for all candidates at height>=1983702. Thus activation height alone cannot disable BIP30, and the activation block itself has no parent ancestor at that height. Regtest's null BIP34Hash does not match a real height1 block, so it keeps scanning. The scan checks existing unspent output outpoints for every transaction before applying the block; duplicate txid with all prior outputs spent is distinct from live-coin overwrite. [VERIFIED: validation.cpp:2780-2861; kernel/chainparams.cpp BIP34Hash]

| Purpose | Exact height/hash |
| --- | --- |
| Repeat allowed on connect |91842 /00000000000a4d0a398161ffc163c503763b1f4360639393e0e4c8e300e0caec |
| Repeat allowed on connect |91880 /00000000000743f190a18c5577a3c2d2a1f610ae9601ac046a38084ccb7cd721 |
| Earlier coinbase mismatch tolerated on disconnect |91722 /00000000000271a2dc26e7667f8419f2e15416dc6955e5a6c6cdf3f2574dd08e |
| Earlier coinbase mismatch tolerated on disconnect |91812 /00000000000af0aed4792b1acee3d966af36cf5def14935db8de83d6f9306f2f |

These pairs serve different operations. [VERIFIED: validation.cpp:DisconnectBlock/IsBIP30Repeat/IsBIP30Unspendable]

**Mandatory planning consequence:** Knots AddCoins permits coinbase overwrites after the BIP30 gate; its ordinary undo stores noncoinbase spent inputs, not overwritten coinbase outputs. Disconnect of the repeated coinbase therefore does not itself restore the overwritten original; the pin instead tolerates the missing earlier coin at91722/91812. CVLD-06/D-06 explicitly require correct earlier-coin restoration. Preserve that stronger approved contract with explicit per-outpoint overwrite undo and document the observable disconnect difference rather than claiming it is Knots-equivalent. Root accepted this resolution during research. [VERIFIED: coins.cpp:AddCoins; validation.cpp:UpdateCoins/DisconnectBlock; undo.h; phase167 D-06; parent research follow-up]

Record the previous complete Coin (output, height, coinbase bit, earned creation MTP) before replacement; disconnect removes the new occurrence, verifies exact identity and restores only the recorded earlier coin. Keep overwrites scoped by the derived BIP30 decision, never by a general caller bool. Include malformed/missing/duplicate undo and wrong block/outpoint controls. Extend durable DTO roundtrips; missing legacy field may mean empty only for records proved never to require overwrite, otherwise refuse. Keep original91722/91812 semantics separately represented in source expectations. [VERIFIED: first-party chainstate/types.rs:BlockUndo; engine/apply.rs; node/storage/snapshot_codec.rs; stronger phase167 D-06]

## Architecture Patterns

### Trust model and project structure

The following are proposed seams, not already implemented APIs. They follow the existing private proof, owner/generation and stage-binding patterns. [VERIFIED: consensus/chain_spec/bootstrap.rs; chainstate/engine/work_authority.rs; node/storage/canonical_bootstrap/proof.rs]

| Layer | Authority to construct | Payload/invariant | Forbidden substitution |
| --- | --- | --- | --- |
| Immutable NetworkRules | ChainSpec only | selected chain, monetary/PoW/buried rules, hashes and exceptions | public ConsensusParams/default fixture config |
| Validated candidate history | canonical root plus previously contextually admitted linked parent; chainstate stage preview for replacement branch | selected chain, candidate hash/parent/height, exact ancestry links/MTP/anchors | best header, active tip from another branch, arithmetic-only HeaderProofFacts |
| CandidateRules | pure derivation from preceding two | versions, BIP34/finality/sequence/witness flags, BIP30 decision | caller booleans/ScriptVerifyFlags |
| Canonical stage | actual coins owner and current generation | history/rules bound to the same parent, work, UTXO branch and candidate bytes | stale/foreign preview, raw snapshot or independently supplied high score |
| Durable accepted lineage | existing genuine managed absorption | publish only accepted bodies; index uses same owner | header admission or body availability as ScriptsValid |
| Seeded historical envelope | explicit test-only/bounded constructor | independently supplied exact ancestry/UTXO prerequisites and evidence class | promotion to product canonical authority or whole-chain claim |

Recommended focused modules: consensus chain_spec/rules.rs and candidate_history.rs (or similarly named cohesive modules), chainstate engine/candidate_rules.rs and overwrite_undo.rs, network header_store/canonical_history.rs, node sync/canonical_history.rs and storage/canonical_history.rs. Keep wrappers and error mapping in existing owners; do not create a parallel chainstate engine. [VERIFIED: existing ownership inspected in source; phase167 Agent Discretion]

### Data flow and freshness

Selected ChainSpec → proved genesis/header parent → shared pure candidate context → contextual header admission → body identity/context → staged coins/script/sigops/reward/BIP30 checks → existing work/eligibility preference → absorption → accepted-validation publication → BASIC continuation/checkpoint. On reorg, disconnect into the same overlay and derive each replacement context from its preceding staged branch, never reuse one flag set across all replacement blocks. [VERIFIED: existing engine/stage.rs and overlay_apply.rs; node/network/validation_history.rs; node/sync/block_reconcile.rs; phase167 D-03,D-07,D-08]

Close all current alternate routes: direct managed connects/reorgs, stored-body reconcile, inbound Headers/Block/CompactBlock/BlockTxn, local submitted body, header imports, fixture readmission, runtime setters and RPC construction. A broad require_ordinary_history()->Ok on Regtest is insufficient because try_seed_canonical_headers currently seeds supplied entries without re-admitting them; maybe_proof_facts proves only arithmetic. Keep fixture APIs isolated and reject their use on sealed canonical owners. [VERIFIED: node/network/canonical_admission.rs; node/network/header_sync.rs; node/sync/block_reconcile.rs; rpc/context/canonical_chain.rs]

### Bounded canonical reopen

Phase166 record proof explicitly requires one header/index/active entry, no coins and empty txid counts. Accepted descendants would cease to reopen under that proof. Add a separate bounded history proof path reusing locked copy/worker/immediate-original equality; retain the genesis-only proof and its tests. Recompute linked headers/work/MTP and replay retained canonical bodies into an isolated pure coins owner, then compare active endpoint, coins, undo/provenance and BASIC binding before minting live authority. No raw store/metadata marker/decoded ScriptsValid field is sufficient. Keep pruning/missing-required-body refusal; Phase169 owns broader differential/recovery qualification. [VERIFIED: node/storage/canonical_bootstrap/{records,proof}.rs; node/storage/fjall_store/inspection.rs; phase166 VERIFICATION; ROADMAP Phase169]

Use explicit existing resource caps and refuse larger/incomplete histories; inspect the64KiB authority-row limit and4096-entry/512MiB inspection limits against the planned301-block corpus before adopting a bound. Do not silently raise deadlines or transform legacy state. A scoped maximum is an implementation/evidence limit, never a consensus rule. [VERIFIED: canonical_bootstrap/records.rs:MAX_AUTHORITY_RECORD_BYTES; inspection/copy.rs:MAX_ENTRIES/MAX_BYTES; phase167 D-09]

## Implementation Dependency Map and Suggested Ownership

This is planning decomposition, not executable PLAN content. Each row should become one or several small plans with positive/negative verification before its consumers are enabled.

| Order | Responsibility / exact existing owners | Deliverable / dependencies |
| --- | --- | --- |
|1|consensus chain_spec.rs, chain_spec/; chainstate engine/canonical_genesis.rs| Immutable four-network rules and support/evidence types; monetary/PoW facts; remove generic defaults from canonical genesis binding. |
|2|consensus candidate_history.rs; network header_store.rs, header_store/eligibility.rs, proposed canonical_history.rs| Linked candidate trust/MTP/anchor derivation and context admission seal; retain exact-work/tie ownership. Depends1. |
|3|consensus block.rs, block/difficulty.rs, block/witness.rs, context.rs| Version, tiny-height BIP34, finality/sequence and witness rules plus additional contextual header gates. Depends1–2. |
|4|consensus contextual script/sigops helpers; chainstate engine/apply.rs, overlay_apply.rs, stage.rs| Exact flag derivation reaches actual interpreter; aggregate flag-sensitive sigops at overlay owner; per-block branch rule derivation. Depends2–3. |
|5|consensus BIP30 rule; chainstate types.rs, engine/apply.rs, proposed overwrite_undo.rs; node storage/snapshot_codec.rs| Exact ancestor/hash decision and transactional earlier-coin undo, codec rejection/roundtrip; intentional difference. Depends1–4. |
|6|node network/canonical_admission.rs, header_sync.rs, validation_history.rs, runtime_authority/; chainstate/validation_history.rs; sync/block_reconcile.rs| Actual canonical ingress/connection/reorg calls consume derived authority; fixture imports cannot bypass it. Depends2–5. |
|7|node storage/canonical_bootstrap.rs and inspection integration; proposed storage/canonical_history/; sync/open_runtime.rs, canonical_genesis.rs| Safe bounded descendant checkpoint/reopen; re-prove before effects; retain genesis route and unsupported selectors. Depends4–6. |
|8|rpc context/canonical_chain.rs, canonical_genesis.rs; bin/open_bitcoind/canonical_startup.rs; affected CLI opener/status adapters| Carry only sealed canonical runtime to product adapters; keep wallet unavailable, public networking default-off and unsupported routes refused. Depends6–7. |
|9|node integration tests, test-harness helpers, packages/testdata/phase167/, docs/parity/fixtures/phase167-*| Serialize independent finite corpus; ordinary genesis→301, negative snapshots, historical envelopes and durable script/undo/reopen evidence. Fixtures can be built early; acceptance depends4–8. |
|10|scripts Phase164/165/166 current owners, new focused Phase167 checker/tests, verify.sh; docs/readmes; parity breadcrumbs/index| Targeted current-source renewals, offline registration, bounded claims, final native/source/security/goal/lifecycle evidence. Scaffold early, renew after reviewed source stabilizes. |

These ownership seams are based on actual inspected files; additions/names remain planner choices. Root must serialize shared-current-owner edits rather than assign the same central files concurrently. [VERIFIED: source inspection; parent ownership/guard-scout findings]

## Guard and Evidence Ownership Migration

- Phase164 test-modules.ts owns SOURCE_FILES, registrations/counts and SOURCE_CONTRACTS; source-files.ts owns EDGES. Renew only touched current source after semantic review. checkFacts currently hashes MISSING_HISTORY and assumes ordinary refusal, so changing its digest alone is insufficient: replace that semantic contract with scoped earned support plus Signet/missing-context/fixture negatives. [VERIFIED: scripts/check-phase164-canonical-identity.ts:checkFacts; corresponding test-modules.ts/source-files.ts; parent guard scout]
- Phase165 INPUT_INVENTORY_SHA changes only if ordered membership changes. verify.sh additions must each occur once in VERIFY_COMMAND_ORDER and run_step; Phase164 historical-tooling CURRENT_VERIFY_ADDITIONS must invert to unchanged original verify.txt/VERIFY_BASE_SHA. Renew only required current inverse generation, never frozen originals, bridge corpora or historical receipts. [VERIFIED: parent guard scout of scripts/check-phase165-shared-exact-work*, check-phase164-canonical-identity/historical-tooling.ts and contracts.ts]
- Preserve Phase166 genesis, opener/worker, BASIC and wallet boundaries. Add a narrow current-scope claims exemption with controls for unqualified Signet, actual-daemon, whole-chain, public-default and production claims; do not globally relax ordinary-history claim checks. [VERIFIED: parent guard scout; scripts/check-phase164-canonical-identity/claims.ts; phase166 VERIFICATION]
- Review README.md, packages/crate READMEs, support matrix, v2.6 identity and new Phase167 parity companion/index, current architecture/project status. Preserve historical attribution; stale earlier progress prose cannot grant authority. No generic historical-receipt audit or new exception framework is needed. [VERIFIED: phase167 D-09,D-10; parent guard scout; inspected READMEs]

## Common Pitfalls

| Pitfall | Failure signal | Required prevention |
| --- | --- | --- |
| Pow-accepted means context-accepted | A caller inserts a valid-PoW wrong-version header and obtains branch facts | Separate contextual seal; negative direct-insertion/import controls. [VERIFIED: header_store/proof.rs; insert_header_with_params; header_sync.rs] |
| Wrong branch/time samples | Active and candidate MTP differ or twelve samples change median | Derive required linked ancestors; tests with nonmonotonic times and divergent branches. [VERIFIED: header_store.rs; chainstate/engine.rs] |
| Flag calculator is disconnected | Unit flags match but runtime still uses P2SH or caller flags | Durable spends whose accept/reject changes under each flag; count real script calls. [VERIFIED: sync/open_runtime.rs:finish_initialized_runtime; engine/apply.rs] |
| Sigops omitted/double-counted | Individually valid spends exceed block budget; P2SH-off exception still counted | Share one flag-sensitive accumulator at transaction application before spent coins disappear. [VERIFIED: block.rs:split_sigop_cost; chainstate/engine/apply.rs; pinned tx_verify.cpp] |
| Staging bypasses future clock | Candidate time is passed as current_time | Trust contextual header admission and shell clock; no candidate-supplied now. [VERIFIED: engine/stage.rs:stage_reorg; validation.cpp ConnectBlock comment] |
| BIP30 activation-only shortcut | Wrong BIP34 ancestor receives overwrite permission | Exact selected height/hash, parent ancestry and>=1983702 recheck. [VERIFIED: validation.cpp ConnectBlock] |
| Overwrite undo added only in memory | Checkpoint/reopen drops prior coin and later disconnect is lossy | Extend both DTO directions, bounds and complete coin metadata; actual all-handle test. [VERIFIED: snapshot_codec.rs; types.rs] |
| Historical exception tested with forged hash | A synthetic body claims the pinned exception hash | Bind parsed serialized body→header hash/merkle; real exact exception bytes or explicitly narrower scalar rule evidence. [VERIFIED: phase167 D-06,D-09; chain_spec/bootstrap.rs proof pattern] |
| Support flip reaches old generic constructor | Mutable params, default wallet or raw open bypasses proof | Expand sealed runtime factory, not generic fixture construction. [VERIFIED: rpc/context/canonical_chain.rs; node/sync/open_runtime.rs] |

## Behavioral Evidence and Negative Test Strategy

This section supplies planning verification requirements despite nyquist_validation=false; no Nyquist Validation Architecture is requested. [VERIFIED: .planning/config.json]

| Requirement / criterion | Minimum independent positive evidence | Required refusal / unchanged-state controls |
| --- | --- | --- |
|CVLD-01 /R1| Start actual ordinary selected-regtest header/durable body owners from proved genesis; serialized chain through>=301; rewards149=50BTC,150=25BTC,299=25BTC,300=12.5BTC plus fees; spend height1 coinbase at101. | Spend at100 refuses depth99; overpay each boundary by1sat; no genesis UTXO; custom maturity1/halving210000 cannot authorize canonical owner. |
|CVLD-02 /R2| Headers require version4 from default-regtest height1; BIP34 heights1–17 and signed-length edges; BIP113 and relative locks derived from candidate's actual ancestors; witness last-commitment/zero-leaf fixtures. | Wrong version at each buried boundary; wrong prefix/nonminimal alternative; parent-MTP equality; sequence lock equality and wrong creating ancestor; missing history; unexpected witness and nonce/commitment faults. |
|CVLD-03 /R3| Durable spends prove DERSIG/CLTV/CSV/NULLDUMMY/P2SH/WITNESS/TAPROOT; historical exact hash replacements then additions; aggregate legacy/P2SH/witness sigops budget on actual coins path. | Same scalar flags with bypassed interpreter must fail controls; policy-only flags not enabled; wrong network/hash loses exception; overbudget block leaves coins/tip unchanged. |
|CVLD-04 /R4| Header and body share expected difficulty/MTP; retarget clamp min/max/cap/compact rounding; testnet recovery at strict2*spacing boundary; default-regtest144-boundary no-retarget behavior. | Missing retarget anchor/recovery chain/MTP sample, wrong parent links/heights, candidate branch different from active branch; invalid bits/PoW/time/version; future-time now spoofing. |
|CVLD-06 /R5| Both exact repeated coinbase contexts, matching/nonmatching BIP34 ancestor, before/at1983702, duplicate fully-spent distinction; actual overwrite undo/durable disconnect restores complete prior coin under approved stronger contract. | Wrong height/hash/network, absent ancestor, unrelated unspent duplicate, malformed/missing overwrite undo, wrong restored outpoint/coin; failure leaves accepted state unchanged. |

Expected rewards/maturity/lock inequalities above are source-derived, not an actual daemon observation. Each case requires the selected support/evidence class. [VERIFIED: validation.cpp:GetBlockSubsidy; consensus/tx_verify.cpp; kernel/chainparams.cpp; REQUIREMENTS.md]

For every refusal, snapshot accepted tip/hash, exact work, coins best block and selected outpoints, BASIC processed/safe endpoints and accepted durable provenance before/after. Header admission and active coins authority are distinct: a valid header can remain known when its body is invalid; invalid header must not acquire accepted header work, and rejected body must not gain ScriptsValid/BASIC progress. Storage of candidate bytes/invalid metadata is not itself accepted authority; do not write tests that incorrectly demand all database bytes remain unchanged after a consensus body refusal. Incompatible-source startup refusal retains the stricter pre-effect original-manifest contract. [VERIFIED: phase167 D-08; phase165 eligibility rules; node/network/validation_history.rs; phase166 VERIFICATION]

Historical envelopes must name chain, exact serialized candidate/hash, linked header interval, parent/height, all needed MTP windows, retarget/recovery anchors, BIP34 ancestor commitment or explicit absence, UTXOs and creating ancestors, expected flags and evidence class. Scalar exception tables alone cannot prove actual durable script wiring. No matching historical exception corpus was found in the scoped first-party fixture and pinned test searches; plan a bounded fixture-acquisition/validation task early, with hash/merkle bindings and provenance, then check it in so native verification stays offline. Real mainnet PoW/hash cannot be manufactured by changing a synthetic header. [VERIFIED: scoped rg searches over packages/testdata/docs/parity and pinned tests; source hash/PoW semantics; phase167 D-09]

Run tests in dependency order: pure rule tables and malformed contexts; network branch/trust tests; chainstate stage/undo/sigops tests; durable managed integration and all-handle reopen; product route/refusal controls; focused guard mutation tests; complete default native plus fresh pure-core coverage/Bazel and independent reviews. Record complete outcomes including failed attempts; do not relax historical budgets. [VERIFIED: phase167 D-10; AGENTS.md verification contract]

## Code Examples

These are source-derived sketches, not existing public API promises.

### Exact flags, replacement before additions

```rust
// Source: pinned validation.cpp::GetBlockScriptFlags.
// Proposed pure helper; selected rules and candidate identity are proved first.
let mut flags = rules.exception_for(candidate.hash())
    .unwrap_or(P2SH | WITNESS | TAPROOT);
if candidate.height() >= rules.bip66_height() { flags |= DERSIG; }
if candidate.height() >= rules.bip65_height() { flags |= CHECKLOCKTIMEVERIFY; }
if candidate.height() >= rules.csv_height() { flags |= CHECKSEQUENCEVERIFY; }
if candidate.height() >= rules.segwit_height() { flags |= NULLDUMMY; }
```

The use of unwrap_or is a total Option fallback, not a panic/unwrap site. Rule lookup is selected-network scoped; a candidate hash supplied independently from its body/header is not proof. [VERIFIED: pinned GetBlockScriptFlags; standards/languages/rust.md]

### MTP window including the newly connected header

```rust
// Source: pinned chain.h::CBlockIndex::GetMedianTimePast.
// Proposed correction to first-party compute_median_time_past.
let mut times = vec![candidate.time];
times.extend(linked_parents.iter().take(10).map(|parent| parent.time));
times.sort_unstable();
let mtp = times[times.len() / 2];
```

The constructor must first prove the expected linked sample count; this calculation alone does not make a truncated seed trustworthy. [VERIFIED: pinned chain.h; first-party header_store.rs]

### Repo-local operator forms

```bash
bun run scripts/command-timings.ts run --key phase167-cargo-status -- cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --config "$phase167_dir/open-bitcoin.jsonc" --datadir "$phase167_dir/data" --network regtest --format json sync status
bun run scripts/command-timings.ts run --key phase167-bazel-status -- bazel run //packages/open-bitcoin-cli:open_bitcoin -- --config "$phase167_dir/open-bitcoin.jsonc" --datadir "$phase167_dir/data" --network regtest --format json sync status
```

These existing command shapes require a physical disposable config/datadir prepared by the eventual UAT task; research did not execute operator workflows or claim future descendant results. [VERIFIED: packages/open-bitcoin-cli/README.md; .codex/tasks/lessons.md]

## Runtime State Inventory

This is a validation/undo-format extension, not a product rename. Only repo-owned state and explicit local probes were inventoried; no external service scan was performed.

| Category | Items found | Action required |
| --- | --- | --- |
| Stored data | Fjall per-outpoint coins, BlockUndo DTO, chain metadata, header/index records, validation and BASIC records. [VERIFIED: node/storage/snapshot_codec.rs; canonical_bootstrap/records.rs] | Extend proved supported undo/history formats; legacy/synthetic/unprovable records refuse before backend effects. No automatic data migration. |
| Live service config | None required by this phase; public networking remains default-off and no service rollout is authorized. [VERIFIED: CONTEXT deferred scope] | Preserve existing config gates. Do not infer that no unrelated machine service exists. |
| OS-registered state | No renamed binary/task/launchd/systemd identity in scope. [VERIFIED: phase167 scope and unchanged product identifiers] | None; no registration mutation. |
| Secrets/env vars | No new credential/env key required by the selected offline rules implementation. [VERIFIED: CONTEXT D-09,D-10; source rule inventory] | None; no secret inspection needed. |
| Build artifacts | Existing Cargo/Bazel outputs may be stale after Rust/DTO edits. [VERIFIED: AGENTS.md native verification contract] | Rebuild through native verifier; fresh coverage uses its own declared clean step. |

## Environment Availability

| Dependency | Available/version | Required action/fallback |
| --- | --- | --- |
| Pinned source | Yes:clean submodule at a9aee730466ac67d35a3c03ee24676be5e045878 | Use local source; no daemon build needed in167. [VERIFIED: git probes] |
| Rust | Yes:1.94.1 | Existing toolchain; Cargo jobs only with timing wrapper. [VERIFIED: rustc --version; AGENTS.md] |
| Bun | PATH1.4.2 differs from pin1.3.9; cached pin found by root | Prepend /Users/peterryszkiewicz/.bun/install/cache/@oven/bun-darwin-aarch64@1.3.9@@@1/bin. [VERIFIED: probes; parent follow-up] |
| Bazel/Bazelisk | Both commands present; pin8.6.0 | Version/runtime qualification occurs in full native verification. [VERIFIED: command -v; .bazelversion] |
| Historical serialized exception corpus | Not found in scoped searches | Acquire finite test-only bytes/provenance and verify identities; scalar tables remain narrower evidence until then. [VERIFIED: scoped fixture searches] |
| Actual pinned Knots daemon | Not probed/required for167 | Phase168 owns executable build/identity and actual qualification. [VERIFIED: ROADMAP Phase168] |

No new runtime/service/library installation is needed. Historical corpus absence is an implementation-plan prerequisite for actual exact-hash durable cases, not permission to replace them with forged scalar contexts. Research ran no Cargo/Bazel jobs or test suite. [VERIFIED: research command log; phase167 CONTEXT]

Root measured26GiB available on the1.8TiB volume. Reuse the cooperative build lock and bounded fixture/inspection budgets; do not create duplicate targets or expand private copies without accounting for this limit. No cleanup task is authorized by this observation. [VERIFIED: parent environment probe report]

## Security Domain

Security enforcement is not disabled in config. Use the requested ASVS4.0.3 category names as a threat-classification aid for this headless consensus phase, without claiming web-app certification. OWASP's current downloads also include5.0.0; category numbering must not be silently mixed. [VERIFIED: .planning/config.json; CITED: https://asvs.owasp.org/downloads/; CITED: https://cornucopia.owasp.org/taxonomy/asvs-4.0.3]

| ASVS4 category | Applies | Phase control |
| --- | --- | --- |
| V2 Authentication | No new login feature | Preserve existing RPC authentication and unavailable wallet. [VERIFIED: phase167 deferred scope; rpc/context/wallet_state.rs] |
| V3 Session Management | No new session feature | Existing transports remain outside core rule derivation. [VERIFIED: inspected phase scope] |
| V4 Access Control | Yes:internal authority boundary | Private canonical/history/stage constructors; no fixture/raw-store promotion; stale/foreign-owner negatives. [VERIFIED: current work_authority and canonical proof patterns] |
| V5 Validation | Yes | Bounded decode, linked ancestry, exact body/hash, flags/versions/time/difficulty and missing-context refusal. [VERIFIED: pinned source inventory; phase167 D-03–D-08] |
| V6 Stored Cryptography | Hash integrity applies; no new encryption | Reuse existing owned SHA256/script verification; no new cryptographic primitive. [VERIFIED: consensus crypto.rs and script.rs; phase167 dependency constraint] |

ASVS category names verified against OWASP's4.0.3 taxonomy; application mapping above is phase-specific analysis. [CITED: https://cornucopia.owasp.org/taxonomy/asvs-4.0.3]

| Threat pattern | STRIDE | Required mitigation/test |
| --- | --- | --- |
| Forged/partial history grants relaxed rules | Spoofing/tampering | Canonical linked parent proof; deny fixture/imported arithmetic as ancestry authority; missing link tests. [VERIFIED: header proof limitations; phase167 D-03,D-07] |
| Stale rule proof after branch/coins changes | Tampering/elevation | Existing owner/generation binding extended to derived context; cross-owner/stale stage controls. [VERIFIED: engine/work_authority.rs] |
| Unbounded ancestor/seed scan | Denial of service | Bounded walks, explicit missing data, no cycle/truncation default; existing inspection caps. [VERIFIED: header_store.rs bounded walk; inspection/copy.rs] |
| Partial rejection or lossy overwrite undo | Tampering | Stage all effects; checked overwrite undo and codec; reopen/negative state comparisons. [VERIFIED: engine/stage.rs; approved D-06,D-08] |
| Unsupported evidence presented as qualification | Repudiation | Fixed evidence classes and exact provenance; phase168 owns actual-daemon receipt. [VERIFIED: D-09; ROADMAP] |

## State of the Art

| Prior/current repository behavior | Phase167 target | Reason |
| --- | --- | --- |
| Generic booleans and caller flags behind ordinary refusal | Immutable selected rules + earned candidate facts | Missing context must not silently become canonical authority. [VERIFIED: context.rs; chain_spec.rs; D-03–D-07] |
| Present-day Taproot interpreted as deployment-gated | Exact baseline flag plus pinned exceptions | The pin's implementation deliberately simplifies historical script flags this way. [VERIFIED: GetBlockScriptFlags] |
| Genesis-only durable proof | Separate bounded canonical descendant proof | Existing genesis success cannot prove accepted descendant coins/history. [VERIFIED: canonical_bootstrap/proof.rs] |
| Historical source helpers treated as differential | Separate source-derived/seeded/ordinary/actual-daemon evidence | Actual qualification remains168. [VERIFIED: ROADMAP scope gates] |

## Assumptions Log

No unverified external capability, compliance obligation or performance target is adopted. Proposed module/type names and decomposition are recommendations grounded in the inspected owners, not claims of implemented APIs. No [ASSUMED] decision is required to replace the locked context. [VERIFIED: research scope and cited sources]

## Open Questions and Planning Resolutions (RESOLVED)

1. **How many ordinary networks to enable?** Implement all selected facts; earn default-regtest ordinary paths first. Permit any additional network only with all contextual conditions, checkpoints and history prerequisites proved. Otherwise keep precise refusal and use classified historical fixtures. This completes the supported-scope requirement without whole-chain assertions. [VERIFIED: D-01,D-05,D-07; ROADMAP Phase167 scope]
2. **How to satisfy restoration while matching BIP30?** Match pinned connect admission and exact exception ownership; implement approved stronger lossless undo; record the pinned disconnect difference explicitly. Do not assert identical historical disconnect UTXOs. [VERIFIED: coins.cpp/validation.cpp; parent resolution]
3. **Where do exact historical bytes come from?** Scoped corpus search found none. Schedule acquisition/identity/provenance validation before durable historical integration; no web service is a native verifier dependency. Seeded contexts stay noncanonical. [VERIFIED: scoped search; D-09]
4. **What reopen bound fits the corpus?** Measure encoded301-block rows against current caps; choose a documented finite supported bound and reuse inspection/replay proof. Keep larger/pruned/incomplete histories refused until separately supported. Do not promise archive-scale recovery. [VERIFIED: records.rs/inspection caps; ROADMAP later ownership]

## Sources

### Primary: HIGH confidence

- Local pinned Knots source at a9aee730466ac67d35a3c03ee24676be5e045878: kernel/chainparams.cpp, kernel/chainparams.h, consensus/params.h, deploymentstatus.cpp/.h, versionbits.cpp, pow.cpp, consensus/tx_verify.cpp, consensus/tx_check.cpp, consensus/consensus.h, validation.cpp, coins.cpp, undo.h, chain.h, script/script.h, clientversion.cpp/.h and CMakeLists.txt; exact rule inventory and additional prerequisites. [VERIFIED: local reads/searches]
- https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp — navigable upstream source anchor; local Git bytes are authoritative for quoted line anchors. [VERIFIED: local pin/hash; web raw source opened]
- Current consensus context/block/difficulty/witness/chain_spec, network header_store/proof/eligibility, chainstate engine/apply/overlay/stage/work_authority, node canonical admission/sync/storage/accepted history and RPC factories — existing seams and gaps. [VERIFIED: source reads]
- Current167 CONTEXT, REQUIREMENTS/ROADMAP/PROJECT/current STATE,164/165/166 contexts and166 VERIFICATION — authority/evidence/dependency boundaries. [VERIFIED: file reads]
- AGENTS.md, AGENTS.bright-builds.md, standards-overrides.md, standards/index.md, core architecture/code-shape/testing/verification, Rust/TS standards; active lessons — workflow, typed ownership and verification contract. [VERIFIED: file reads]

### Secondary: MEDIUM confidence

- Parent guard-scout findings for current Phase164/165/166 guard renewal and documentation ownership; narrower key contracts were independently located. [VERIFIED: parent report; targeted rg]
- https://cornucopia.owasp.org/taxonomy/asvs-4.0.3 and https://asvs.owasp.org/downloads/ — security category/version verification; mapping is analysis, not certification. [CITED: official OWASP pages]

### Tertiary

None used to lock implementation decisions. Context7 is not available in the exposed tool catalog; package API research is unnecessary because no new dependency is recommended. [VERIFIED: tool discovery; phase scope]

## Metadata

| Area | Confidence | Basis |
| --- | --- | --- |
| Standard stack |HIGH| Existing pins and installed tools inspected; Bun mismatch and Bazel unprobed runtime explicit. |
| Pinned validation rules |HIGH| Exact local source pin, clean status and validation.cpp Git/file hash agreement; cross-function checks. |
| Architecture/decomposition |MEDIUM| Existing seams traced; proposed interfaces still require compile-time implementation design. |
| Pitfalls/test strategy |HIGH| Specific source gaps and locked five requirement behaviors; no execution success asserted. |

**Valid until:** Source/phase constraints change; recheck runtime/tool availability before execution. Research created only this artifact; commits are deferred to root's clean-verification/lifecycle gate. [VERIFIED: parent assignment]

