# Feature Research

**Project:** Open Bitcoin v2.6 — Canonical Chain Bootstrap and Validation Parity
**Domain:** Brownfield, headless Bitcoin node; pinned observable-behavior parity
**Researched:** 2026-10-13
**Confidence:** HIGH for cited source behavior; MEDIUM for proposed acceptance design; execution remains unperformed.
**Baseline:** Bitcoin Knots `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`; local submodule HEAD verified.

## Scope and Evidence Boundary

The user selected six deliverables: canonical bootstrap, historical rules, exact cumulative work, genuine differential validation, forks/recovery, and canonical prune/BASIC integration. All six are milestone essentials; dependencies may require more than six phases. This research proposes requirements, not completed functionality or full consensus parity.

Material guidance: `AGENTS.md`, `AGENTS.bright-builds.md`, placeholder-only `standards-overrides.md`, `standards/index.md`, and architecture/testing/verification pages. Preserve pure rule decisions, shell-owned I/O, first-party Bitcoin ownership, one active Fjall chainstate, and the native verification contract. Use the research template's feature/dependency structure adapted to a parity milestone.

The archived v2.5 requirements and audit establish completed BASIC indexing/serving and prune integration **within their declared synthetic-root/work/halving limits**. They do not establish canonical-network ordinary validation. Retain their historical acceptance; earn new evidence for the changed boundary.

## Feature Landscape

### Table Stakes for This Milestone

| Feature                          | Expected observable behavior                                                                                                                                                  | Complexity | Requirement category |
| -------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------------------- |
| Canonical network identity       | A fresh datadir is bound to the selected network and exact genesis; an incompatible existing identity refuses before mutation.                                                | HIGH       | Bootstrap            |
| Durable genesis initialization   | Empty startup creates the canonical height-zero body/header/history/best-block consistently; restart is idempotent and interrupted initialization recovers or refuses safely. | HIGH       | Bootstrap/recovery   |
| Genesis coin semantics           | Genesis has no spendable coinbase UTXO; indexing can still consume its body without non-genesis undo.                                                                         | MEDIUM     | Bootstrap/coins      |
| Fresh BASIC-enabled startup      | Explicit indexing can initialize from canonical genesis on an empty durable datadir; no missing-prefix error caused by startup ordering.                                      | HIGH       | Bootstrap/index      |
| Network/history rule resolution  | Headers, blocks and scripts use selected-network ancestry, height, hash exceptions and deployment state; callers cannot weaken consensus flags.                               | HIGH       | Validation           |
| Correct rewards and maturity     | Default regtest uses its actual subsidy interval and 100-block maturity; excessive coinbase reward and premature spends reject.                                               | MEDIUM     | Validation           |
| Exact cumulative work            | Target-derived 256-bit work selects candidates and survives persistence/reopen without narrowing, caller substitution or display-roundtrip authority.                         | HIGH       | Work                 |
| Independent Knots differential   | Identical serialized candidates go through ordinary validation in both implementations; compare outcomes and resulting authoritative state.                                   | HIGH       | Evidence             |
| Competing branches and rejection | Higher-work valid branches win; invalid candidates cannot change coins, active history, BASIC progress or durable authority.                                                  | HIGH       | Forks                |
| Checkpoint and actual reopen     | The durable checkpoint, coins best-block, headers, chainwork and validation provenance agree after process/store destruction and reopening.                                   | HIGH       | Recovery             |
| Canonical prune/BASIC continuity | Legally prune real canonical-regtest history, retain exact filter commitments, and preserve truthful block availability after reopen.                                         | HIGH       | Integration          |
| Honest parity inventory          | Sources, execution identity, corpus limits, intentional differences and unsupported network modes remain explicit in docs and claim checks.                                   | MEDIUM     | Closure              |

All are P1. Complexity describes integration risk, not permission to drop correctness prerequisites. [Pinned validation behavior](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp) and [work/selection behavior](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp) define the reference surface.

### Differentiators Aligned With the Existing Product

| Feature                           | Value                                                                                                                                 | Complexity | Boundary                                                 |
| --------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ---------- | -------------------------------------------------------- |
| Auditable acceptance corpus       | Contributors reproduce an exact pass/fail claim against a fixed source and executable, with raw input/output hashes.                  | MEDIUM     | Test-only tooling; no new production Bitcoin dependency. |
| One validation authority          | RPC ingestion, managed connect/reorg, sync, storage recovery and filter protection use the same typed network/history/work decisions. | HIGH       | Extend existing crates and adapters.                     |
| Explicit evidence classes         | A reader distinguishes real canonical-chain execution, arithmetic vectors, rule-context cases and storage fault probes.               | LOW        | Do not inflate narrow evidence into network-wide parity. |
| Safe incompatible-datadir refusal | Old synthetic roots or unprovable work cannot silently become canonical provenance.                                                   | MEDIUM     | No implicit migration, repair or deletion.               |

### Anti-Features and Explicit Exclusions

| Feature/shortcut                                                 | Why problematic here                                                                                | Required alternative                                                                             |
| ---------------------------------------------------------------- | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Synthetic height-zero roots as canonical proof                   | A mined null-parent block is not the selected network's genesis.                                    | Keep such tests labeled synthetic; add exact canonical genesis execution.                        |
| Maturity-one or tiny-halving fixtures as differential proof      | They exercise a different network rule set.                                                         | Default regtest 100-block maturity and 150-block halvings; retain small unit vectors separately. |
| Caller-supplied work or height-as-work                           | Forged values can determine fork choice independently of verified headers.                          | Derive exact proof from validated target and parent work.                                        |
| Persisted validation receipts as proof of consensus equivalence  | Genuine managed absorption establishes which path accepted a block, not that its rules equal Knots. | Differentially exercise the canonical production rules used by that path.                        |
| Sparse mainnet blocks claimed as genesis-to-boundary validation  | Required ancestry and spent outputs may be absent.                                                  | State exact contextual inputs and what was actually validated.                                   |
| Trusting a serialized tip/work summary on reopen                 | It can disagree with verified ancestry, coins best-block or durable checkpoint.                     | Validate/rederive authority and reject incompatible metadata.                                    |
| Copying Knots' process-address tie breaker                       | It is an internal implementation detail and nondeterministic across processes.                      | Test defined activation-order behavior; declare genuinely ambiguous equal-work cases.            |
| Testnet4 hidden behind existing `testnet`                        | Current selector/magic identifies testnet3; testnet4 has distinct genesis and BIP94 rules.          | Separate explicit scope decision and identity if added.                                          |
| Signet treated as ordinary PoW with another limit                | Signet also requires challenge verification.                                                        | Implement supported challenge semantics or refuse unsupported canonical validation clearly.      |
| New production dependencies or storage engines                   | Violates selected first-party implementation and Fjall boundary.                                    | Use existing Rust/Bazel/Bun stack; baseline executable is test-only.                             |
| Assumeutxo, assumevalid shortcuts or dual chainstate             | Bypasses or expands the selected ordinary-validation proof.                                         | Run oracle with assumevalid disabled; retain one active chainstate.                              |
| Archive-scale/full-history certification                         | Finite bounded evidence cannot establish it.                                                        | Publish actual corpus and residual limits.                                                       |
| Wallet expansion, BIP37/V0, public defaults or production claims | Separate product/security surfaces.                                                                 | Preserve shipped eligibility, BASIC-only and opt-in boundaries.                                  |

Other exclusions: live Core/Knots datadir import/export, automatic reindex/redownload/destructive repair, default public-network CI, production service/funds safety, and unrelated v2.4 advisory cleanup. A retained advisory that blocks canonical startup/recovery must be resolved in scope rather than hidden by the exclusion.

## Production Findings Versus Existing Fixture Limits

| Observed source                                             | Finding                                                                                                               | Implication                                                                                        | Confidence |
| ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ---------- |
| `open-bitcoin-node/src/sync/types.rs:134–164`               | All networks currently get 210,000-block halvings and always-on BIP34/BIP113/SegWit booleans; only PoW settings vary. | Default regtest's 150-block schedule and mainnet historical rules require production changes.      | HIGH       |
| `open-bitcoin-node/src/sync/open_runtime.rs:121–125`        | Durable runtime stores fixed `ScriptVerifyFlags::P2SH`.                                                               | Ordinary sync must resolve canonical consensus flags; richer unit-test flags alone do not fix it.  | HIGH       |
| `open-bitcoin-chainstate/src/engine.rs:316–367`             | Public connect/stage APIs accept caller `u128` work, flags and parameters.                                            | Seal production rule/work authority; do not merely widen fixture fields.                           | HIGH       |
| `open-bitcoin-chainstate/src/engine/overlay_apply.rs:32–81` | Height zero follows ordinary transaction connection after contextual checks.                                          | Add selected-genesis identity and special coin treatment without weakening non-genesis checks.     | HIGH       |
| `open-bitcoin-chainstate/src/engine.rs:479–488`             | Equal work currently compares height then block hash.                                                                 | Knots activation-order behavior must be investigated and aligned or documented narrowly.           | HIGH       |
| `open-bitcoin-node/src/sync/types.rs:71–104`                | Networks are Mainnet/Testnet/Signet/Regtest; testnet magic is testnet3.                                               | A selector is not evidence of complete network validation; no existing testnet4 selector observed. | HIGH       |
| Archived v2.5 audit, “Preserved limits”                     | Synthetic-root/work/halving evidence expressly excludes whole-chain consensus differential.                           | Keep old evidence valid within limits; build a separate genuine corpus.                            | HIGH       |

Pinned Knots sources: `LoadGenesisBlock` persists the real genesis; `ConnectBlock` skips its transactions; `GetBlockSubsidy` uses network interval; `GetBlockScriptFlags` has hash exceptions plus deployment-derived flags; `AddToBlockIndex`/disk loading derive chainwork. These are source facts, not a claim that this research executed a node.

## Requirement Candidates by Deliverable

Identifiers below are provisional categories for roadmap decomposition, not final requirement ownership.

### 1. Canonical Bootstrap — BOOT

- **BOOT-01:** Selected identity determines exact genesis bytes/hash/header, PoW limit and durable network identity; unknown/conflicting identities reject.
- **BOOT-02:** Empty durable startup initializes canonical header/body, active/coins best-block and validation history through supported initialization; reopen repeats no genesis effects.
- **BOOT-03:** Genesis creates no spendable coins, while its retained body and zero-predecessor BASIC commitment remain obtainable.
- **BOOT-04:** BASIC-enabled fresh startup orders genesis, durable validation history, index initialization and prune protection safely; incomplete writes recover or refuse without phantom progress.
- **BOOT-05:** Existing datadirs with synthetic roots, network mismatch, legacy unverifiable work or inconsistent authority refuse non-destructively with a concrete diagnosis; no implicit upgrade success.

### 2. Network and Historical Rules — RULE

- **RULE-01:** Resolve immutable network defaults and ancestry-derived context inside the shared production boundary; freeze supported overrides separately from ordinary defaults.
- **RULE-02:** Enforce network subsidy, aggregate fees/reward, maturity and valid coinbase encoding at genuine boundaries.
- **RULE-03:** Resolve block-consensus script flags, historical hash exceptions, BIP34/BIP65/BIP66/CSV/SegWit contextual rules and relevant deployment state from pinned source; keep mempool policy flags separate.
- **RULE-04:** Validate expected difficulty/retarget/min-difficulty behavior, version, MTP/future-time and locktime/sequence context for supported modes, using explicit deterministic clock inputs.
- **RULE-05:** Inventory historical prerequisites including BIP30 duplicate-coinbase exceptions, overwrite/disconnect semantics and witness commitments; implement those needed by the supported production rules and chosen histories.
- **RULE-06:** Bind every network selector to a truthful capability contract; signet challenge and testnet distinctions cannot silently fall back to permissive defaults.

The pinned block script flags begin with P2SH/WITNESS/TAPROOT subject to hash exceptions, then add deployment-derived DERSIG/CLTV/CSV/NULLDUMMY. A simplistic “all features off before activation” reconstruction differs from the actual reference. Contextual witness/finality rules are separate. [Pinned `GetBlockScriptFlags` and contextual checks](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp).

### 3. Exact Work Authority — WORK

- **WORK-01:** Compute proof as `floor(2^256 / (target + 1))` with exact integer operations, compact-target validation and network PoW limits; match pinned arithmetic boundary behavior.
- **WORK-02:** Derive cumulative work from verified parent linkage and target across header admission, candidate ranking, managed connect/reorg and live sync; block callers cannot choose it.
- **WORK-03:** Use one 256-bit domain representation through active positions, durable headers/chain metadata, replay/recovery and comparisons; conversions are explicit and lossless.
- **WORK-04:** Preserve exact operator output and documented encoding; deserialize with bounds/identity checks and refuse legacy or contradictory authority safely.
- **WORK-05:** Define equal-work behavior from Knots activation sequence and validity/data availability; do not assume hash or height is its universal tie rule.

Independent arithmetic vectors must cross 128-bit boundaries even though a small genuine regtest chain will not. They prove arithmetic/encoding behavior; canonical differential chains prove its ordinary-path integration. [Pinned proof formula](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp) and [candidate ordering/recomputation](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp).

### 4. Genuine Differential Harness — DIFF

- **DIFF-01:** A hermetic test-only harness verifies Knots executable/source identity and records version, exact invocation/options, corpus hashes and raw results; missing oracle is failure/explicit unavailable, never a parity pass.
- **DIFF-02:** Feed identical serialized blocks/transactions, canonical genesis, deterministic time and ordinary rules to each implementation; disable oracle assumevalid shortcuts and external networking.
- **DIFF-03:** Compare accept/reject class/reason where exposed, tip/hash/height/work, relevant UTXOs and normalized block/filter commitments after each transition; never derive expected values from Open Bitcoin itself.
- **DIFF-04:** Malformed/rejected candidates leave all authoritative state unchanged; classify temporary/future-time errors separately from permanently invalid history where reference behavior does.
- **DIFF-05:** Every case declares evidence class, network, initial context, options, covered rule and limitations; finite passes never authorize complete consensus/network coverage claims.

### 5. Forks and Durable Recovery — FORK

- **FORK-01:** Genuine canonical-regtest competing branches exercise valid activation, equal-work arrival behavior and strictly higher-work replacement through ordinary production validation.
- **FORK-02:** Disconnect/reconnect restores exact coins and active metadata; rejected replacement branches cannot advance receipts, tips, work or filter progress.
- **FORK-03:** Full checkpoint then real close/reopen preserves chosen branch and exact work; faulted flush/reorg recovery either produces the valid durable state or refuses explicitly.
- **FORK-04:** Recovery cross-checks network/genesis, validated ancestry, coins best-block, durable chain/header metadata and index progress; stale snapshots or standalone summaries cannot create authority.

“Checkpoint” here means the durable storage checkpoint; Knots consensus checkpoint policy is a separate oracle option that must be recorded for historical/fork cases.

### 6. Canonical Prune/BASIC Integration — HIST

- **HIST-01:** Extend a default-regtest canonical chain far enough to meet ordinary prune eligibility and keep-window rules; actual paired body/undo deletion is required evidence.
- **HIST-02:** Before and after prune/reopen, compare genesis and spent-output BASIC bytes/hash/header against Knots, then exercise retained RPC and bounded peer responses.
- **HIST-03:** Index inputs remain protected until safe durable progress; reorg, interrupted-prune recovery, missing-history activation and corruption retain shipped explicit-refusal behavior.
- **HIST-04:** Pruned block serving and wallet eligibility remain truthful; retained filters restore neither deleted payloads nor spendability. Preserve initial-sync versus current-lag semantics.
- **HIST-05:** Add bounded mainnet historical-rule evidence and a source-linked residual-coverage inventory; closure guards distinguish ordinary-chain differential from contextual vectors.

## Supported Network and Evidence Recommendation

| Network                      | Required v2.6 evidence                                                                                                                                    | Special prerequisites / limits                                                                                                                                 |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Default regtest              | Real canonical genesis through ordinary connects, maturity/halving, competing branches, durable checkpoint/reopen and legal prune/BASIC continuity.       | 150-block halving, maturity 100, no retargeting; no maturity/activation overrides in the canonical proof.                                                      |
| Mainnet                      | Exact genesis/fresh startup plus bounded real historical inputs and source-derived boundary cases; enumerate which cases have complete validated context. | Historical activation, hash/BIP30 exceptions and retarget ancestry are correctness prerequisites. Sparse fixtures cannot prove ordinary full-chain validation. |
| Testnet3 (current `testnet`) | Explicit identity/rule inventory and targeted boundary coverage if canonical validation is supported; otherwise an explicit unsupported boundary.         | Min-difficulty recovery and different historical activations. Do not relabel as testnet4.                                                                      |
| Signet                       | Explicit identity plus challenge-validation capability decision before claiming canonical non-genesis acceptance.                                         | BIP325 challenge validation is additional to PoW; custom challenge scope is not implied.                                                                       |
| Testnet4                     | Explicitly excluded from current recommended selector expansion unless separately selected.                                                               | Distinct genesis, BIP94 difficulty/timewarp behavior and identity; not an alias.                                                                               |

Accepted scope recommendation: complete default-regtest ordinary-path evidence, bounded honestly classified mainnet historical-rule contexts, and canonical identities for currently exposed networks. Unsupported consensus modes must refuse explicitly; selectors alone establish no new support claim. Do not broaden public networking or imply complete signet acceptance. [Pinned network parameters](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp), [BIP325](https://github.com/bitcoin/bips/blob/master/bip-0325.mediawiki), [BIP94](https://github.com/bitcoin/bips/blob/master/bip-0094.mediawiki).

## Evidence-Based Acceptance Scenarios

These are proposed scenarios, not executed results. Require raw receipts and actual ordinary-path invocation before marking them passed.

| Scenario                               | Observable acceptance                                                                                                                                                 | Evidence class                                                               |
| -------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Empty durable startup, BASIC off/on    | Canonical height zero, exact body/header/work, absent genesis UTXO; explicit BASIC starts with exact genesis filter; actual reopen is identical.                      | Canonical ordinary path + oracle                                             |
| Wrong-network/synthetic legacy datadir | Startup refuses before changing authoritative records; diagnosis identifies incompatible identity or unverifiable authority.                                          | Durable failure probe                                                        |
| Default-regtest maturity               | Coinbase created at height 1 cannot spend at height 100 (age 99); can spend at 101 (age 100), subject to all other valid rules.                                       | Canonical ordinary differential                                              |
| Default-regtest subsidy                | Blocks 149/150 exercise 50/25 BTC maximum subsidy plus fees; overpayment rejects without tip/coins changes.                                                           | Canonical ordinary differential                                              |
| Script and contextual boundaries       | Positive/negative CLTV/CSV/DER/witness cases use correct history; historical exception and deployment resolver outputs match pinned source.                           | Differential where full context exists; otherwise labeled contextual vectors |
| Difficulty and work                    | Bad bits/PoW reject; parent-derived sums match Knots; 256-bit arithmetic and serialization independently cover above-128-bit values.                                  | Canonical ordinary + separate arithmetic vectors                             |
| Competing branches                     | Equal-work controlled arrival and a strictly higher-work valid branch produce expected activation; invalid higher-work candidates leave state intact.                 | Canonical ordinary differential                                              |
| Reorg checkpoint/reopen                | Compare normalized UTXOs, tip, exact work and filter lineage before/after disconnect/reconnect and actual reopened Fjall.                                             | Canonical ordinary + durable probes                                          |
| Canonical pruning                      | Meet default regtest prune-height/keep-window rules, delete actual body/undo pairs, reopen and retain exact BASIC RPC/peer results with truthful pruned-body refusal. | Canonical integration + oracle                                               |
| Bounded mainnet history                | Exact real bytes and source anchors for chosen activation/exception/retarget boundaries; each case states available ancestry/prevouts and oracle validation depth.    | Explicitly bounded historical/contextual                                     |

Default regtest has prune-after height 1,000 in the pin (fastprune is a distinct override); choose sufficient history for eligibility and the keep window rather than treating a small chain as prune proof. Do not require physically realizing the soft 550 MiB target unless selected as a separate measured acceptance goal. [Pinned pruning parameters](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp).

## Feature Dependencies and Roadmap Implications

```text
Network identity + canonical genesis --> fresh durable/BASIC startup
Network parameters + ancestry/activation resolver --> ordinary block/script validation
Exact work primitive --> headers + chainstate selection + durable schema/recovery
Canonical startup + correct rules + exact work --> genuine differential baseline
Differential baseline + branch authority --> forks/disconnect/reconnect/reopen
Canonical history + recovered branch + index protection --> prune/BASIC proof
All evidence classes + residual inventory --> parity closure and claim guards
```

- Build the oracle runner/corpus manifest early so production changes can be checked incrementally; final differential acceptance still depends on shared canonical authority.
- Separate exact-work arithmetic/type propagation from durable encoding/recovery if needed; both must complete before authoritative fork/reopen claims.
- Separate network rule modeling from historical script/transaction correctness when complexity warrants, without declaring an earlier partial stage canonical parity.
- Genesis initialization must precede BASIC manager activation/protection; recovery ordering must protect index inputs before resumed pruning.
- Existing v2.5 filter code is reused, but its new proof must traverse changed canonical startup, rules and work paths.
- Mainnet boundary fixtures require an explicit context/provenance design before promises; never silently replace them with regtest overrides.

## Priorities and Remaining Research

| Priority | Deliverable                                                                                       | Completion gate                                                                                                     |
| -------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| P1       | Bootstrap/rules/work                                                                              | Shared production authority, safe durable identity and exact arithmetic; source-linked unit and integration checks. |
| P1       | Differential/forks/recovery                                                                       | Actual pinned oracle runs with identical bytes and real reopen, including negative cases.                           |
| P1       | Canonical prune/BASIC/closure                                                                     | Actual deletion and retained serving against genuine history; residual evidence limits recorded.                    |
| Deferred | New network selectors, custom signet, archive-scale histories, wallet/public/production expansion | Requires separately selected scope.                                                                                 |

No additional user scope decision is required for this recommendation. Phase research must choose the precise mainnet boundary corpus, independently establish its required context and classify each result. Expanding to whole-chain historical oracle replay, testnet4 or complete/custom-signet validation would alter the accepted scope and requires a separate decision.

## Sources and Confidence

- **HIGH:** Local pinned `packages/bitcoin-knots/src/kernel/chainparams.cpp`, `src/validation.cpp`, `src/chain.cpp`, `src/pow.cpp`, `src/node/blockstorage.cpp`, `src/index/blockfilterindex.cpp`; inspected exact source, not current upstream defaults.
- **HIGH:** Local production files named in the findings table; archived `.planning/milestones/v2.5-REQUIREMENTS.md` and `v2.5-MILESTONE-AUDIT.md`; `.planning/PROJECT.md` user-approved v2.6 scope.
- **HIGH:** [Pinned network/genesis parameters](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp), [validation](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp), [work proof](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp), [PoW](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/pow.cpp), [durable work/comparator](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp), [filter index](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp).
- **HIGH:** Official [BIP158](https://github.com/bitcoin/bips/blob/master/bip-0158.mediawiki), [BIP325](https://github.com/bitcoin/bips/blob/master/bip-0325.mediawiki), [BIP94](https://github.com/bitcoin/bips/blob/master/bip-0094.mediawiki), accessed 2026-10-13; standards context only, pinned code governs milestone parity.
- **MEDIUM:** Requirement granularity, network evidence recommendation and phase dependencies are reasoned design recommendations. No oracle process, corpus replay, build or runtime acceptance was executed during this research.
