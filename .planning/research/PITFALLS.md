# Pitfalls Research

**Project:** Open Bitcoin — v2.6 Canonical Chain Bootstrap and Validation Parity
**Domain:** Canonical genesis, historical validation, cumulative work and bounded Knots differential evidence
**Researched:** 2026-10-13
**Confidence:** HIGH for inspected source and archived limits; MEDIUM for proposed implementation and test strategy

## Scope and Evidence

The behavioral authority is Bitcoin Knots `29.3.knots20260210`, pinned at `a9aee730466ac67d35a3c03ee24676be5e045878`. Findings below come from that local source, first-party source and the archived v2.5 audit. Official pinned GitHub chain-parameter and filter-index sources were also checked; the validation webpage returned HTTP 429, so its claims use the inspected local baseline. No new build, differential process or runtime experiment was performed. Proposed tests are acceptance criteria, not passing evidence.

Material guidance: repo `AGENTS.md`, Bright Builds sidecar, placeholder overrides, standards index and architecture/testing/verification/Rust standards. The parent loaded the complete active lesson set. This work is research inside `/gsd-new-milestone`. Preserve first-party Bitcoin ownership, pure core/shell boundaries, one active Fjall chainstate and explicit/default-off public networking. No production dependency, assumeutxo, dual-chainstate, archive-scale, public-network CI or production-funds expansion is justified here.

Phase names are suggested ownership packages: **Bootstrap**, **Validation Context**, **Work Authority**, **Differential Harness**, **Fork and Recovery**, **Canonical Prune/BASIC**, **Final Evidence**. Assign actual phase numbers in the roadmap. Bootstrap, context and work must converge before whole-chain differential acceptance is claimed.

## Critical Pitfalls

### Pitfall 1: A Null Parent Is Mistaken for Canonical Genesis

**What goes wrong:** An arbitrary mined root or a different network's root initializes a supposedly canonical datadir; every subsequent receipt inherits the wrong identity.

**Why it happens:** `engine/overlay_apply.rs` permits the null parent when the active chain is empty. `network/header_sync.rs` recognizes the first null-parent header by shape. Neither inspected path binds that root to a configured canonical genesis. Generic BIP34 contextual validation also applies to height zero, which conflicts with canonical genesis handling.

**How to avoid:** Own canonical serialized genesis/header/hash and network identity in a pure parameter model. A dedicated idempotent bootstrap plan must validate the configured identity before writes, retain its body/history, establish coins best-block without normal transaction connection, and publish consistent metadata/header state. Reject incompatible existing roots and mixed identities before filter catch-up or prune recovery can mutate storage.

**Warning signs:** Startup succeeds with a random null-parent block; an empty BASIC-enabled datadir still says retained genesis is missing; canonical genesis gets `bad-cb-height`; switching networks reuses the prior chain. Test empty, initialized, partially initialized, wrong-root, wrong-network and interrupted-bootstrap reopen separately.

**Phase to address:** Bootstrap; durable integration must precede Canonical Prune/BASIC. Source confidence HIGH; crash ordering requires implementation evidence. [L1, L3, K1, K2]

### Pitfall 2: Genesis UTXO Semantics Are Conflated with Filter Semantics

**What goes wrong:** The genesis coinbase becomes spendable, or a fix removes its script from the BASIC filter and produces incompatible bytes.

**Why it happens:** Current `apply_connect_transactions` calls `add_transaction_outputs` for every transaction, including height zero. Existing synthetic tests intentionally spend their root output. Knots `ConnectBlock` instead advances best-block and skips genesis transactions; its filter builder still extracts scripts from the serialized genesis outputs and requires no genesis undo.

**How to avoid:** Separate canonical genesis initialization from generic synthetic fixture construction. Preserve header/body/history and an empty genesis UTXO effect, while independently deriving the genesis filter from its body with the zero predecessor filter header. Make root disconnect/reconnect behavior explicit rather than assuming ordinary undo exists.

**Warning signs:** Genesis outpoint appears in `gettxout` or coins after reopen; a maturity wait makes it spendable; genesis filter is empty solely because coins are empty; catch-up requires nonexistent genesis undo.

**Phase to address:** Bootstrap owns coins semantics; Canonical Prune/BASIC owns independent byte/hash/header comparisons against Knots. HIGH source confidence. [L1, K1, K4]

### Pitfall 3: Validation Uses Current Defaults Instead of Candidate History

**What goes wrong:** Old valid blocks fail BIP34/witness rules, inactive rules are enforced, or active script rules are omitted. Regtest rewards remain 50 BTC after height 149 even though Knots halves at height 150.

**Why it happens:** `ConsensusParams::default` and `sync/types.rs` set BIP34/BIP113/SegWit booleans true; every sync network gets a 210,000-block subsidy interval. `sync/open_runtime.rs` selects only P2SH script flags. Copying fixed booleans into a per-network struct preserves the historical error.

**How to avoid:** Derive an immutable validation context from the candidate's parent branch, height, block hash, timestamps and network parameters; derive transaction/mempool contexts through their own correct next-block boundary. Match pinned buried deployments, version constraints, finality cutoff, relative locks, witness commitment and subsidy behavior. Test maturity differences 99/100 (spend heights 100/101 for a height-1 coinbase), and regtest rewards at 149/150/299/300; do not reduce maturity to one for acceptance proof.

**Warning signs:** Header and body callers manufacture different contexts; script flags are a runtime constant; context follows the active tip while validating a side branch; regtest acceptance needs `-testactivationheight`, `-vbparams`, maturity or reward overrides.

**Phase to address:** Validation Context; Differential Harness supplies boundary and near-miss corpora. HIGH for present mismatch, MEDIUM for full boundary inventory until phase research. [L2, L3, K1, K2]

### Pitfall 4: A Textbook Activation Table Replaces Pinned Knots Behavior

**What goes wrong:** A well-intended activation implementation disagrees with the actual pinned node on historical scripts or advertises unsupported signet/testnet coverage.

**Why it happens:** Knots `GetBlockScriptFlags` starts with P2SH, WITNESS and TAPROOT, applies hash-keyed `script_flag_exceptions`, then adds deployment-dependent DERSIG, CLTV, CSV and NULLDUMMY. It is not simply an independent activation-height toggle for every flag. BIP30 exceptions and contextual witness rules are separate. Signet has a body challenge check; the pinned source also defines testnet4, while current `SyncNetwork::Testnet` identifies testnet3.

**How to avoid:** Transcribe the pinned decision boundaries with source anchors and explicit historical exceptions. Declare the supported network/parameter matrix. A signet root/PoW match must not imply challenge validation: implement the required check or refuse that unsupported validation path explicitly. Define whether custom signet, testnet4, regtest deployment overrides and general versionbits state are supported or excluded before coding; do not add them implicitly.

**Warning signs:** Historical exceptions disappear during cleanup; policy flags such as LOW_S/CLEANSTACK become block consensus requirements; one fixed activation height is used for a history-sensitive deployment; signet transactions are declared scripts-valid after only ordinary PoW checks.

**Phase to address:** Validation Context, with explicit scope decisions before its plan; Final Evidence preserves exclusions. HIGH source confidence; precise additional-network effort remains open. [K1, K2, K7, L2, L3]

### Pitfall 5: Exact Work Remains a Sidecar to Synthetic Authority

**What goes wrong:** Status reports correct 256-bit work while fork choice, accepted receipts, index fencing or restart still trusts block count or caller-provided `u128` values.

**Why it happens:** `header_store/proof.rs` already derives `ProofWork` from target, but explicitly operates independently of synthetic fork choice. `HeaderEntry`, `ChainPosition`, `ChainTipIdentity` and snapshot DTOs carry `u128`; `header_work_score` returns one and `next_chain_work` increments one. Changing only the visible work field leaves conflicting authorities.

**How to avoid:** Promote one existing exact-work domain value through header admission, body connection, branch plans, accepted identities, durable serialization, recovery, index/prune fences and projections. Compute work from validated target and parent identity internally; remove caller-selected production work. Specify compact-target validity, numeric ordering, byte order, cumulative arithmetic and overflow behavior against `GetBlockProof`/`arith_uint256`; hexadecimal strings must not define numeric order accidentally.

**Warning signs:** Any live `+1`, saturating score, lossy cast or authoritative `u128` remains; equal-sized limbs change ordering after encoding; exact work differs across headers/coins metadata/reopen; an invalid target receives ordinary work. Arithmetic `u128` intermediates are not themselves evidence of a defect.

**Phase to address:** Work Authority before Fork and Recovery. Acceptance needs unequal-target forks, values above 128 bits, compact-target edge vectors and reopened authority agreement. HIGH source confidence. [L1, L3, L4, K3]

### Pitfall 6: Equal Work and Header Validity Are Treated as Full Branch Acceptance

**What goes wrong:** A node reorganizes on height/hash ordering unlike Knots, or a best-header update installs an unvalidated body as the active chain.

**Why it happens:** Current header and chainstate comparators break equal work by height/hash; exact header facts also use `(work, height, hash)`. Knots candidate ordering uses work, then sequence/earliest activatable order, with an internal pointer fallback after disk load. Header proof says nothing about transaction inputs, scripts, body availability or failed ancestry.

**How to avoid:** Separate best header, body availability, validation status and active validated chain. Define the observable equal-work arrival behavior against pinned processes; do not copy pointer addresses into durable Rust policy. Preserve the current active branch where justified by pinned behavior, and model body-complete activation ordering explicitly. Reject descendants of known-invalid ancestry without blessing a separate accepted-state receipt.

**Warning signs:** Merely announcing a high-work header changes coins; equal-work branches switch when hashes are swapped; restart changes tip without a documented pinned boundary; missing body/undo becomes a scripts-valid receipt.

**Phase to address:** Work Authority defines selection; Fork and Recovery proves equal-work order, shorter higher-work selection, invalid ancestry and missing-body refusal. HIGH comparator evidence; disk-load tie guarantees need focused runtime research. [L1, L4, K1, K3]

### Pitfall 7: A Source-Derived Oracle Is Labeled a Differential Node Run

**What goes wrong:** Independently computed filter/formula results are presented as proof that pinned Knots accepted the same canonical chain under ordinary validation.

**Why it happens:** The shipped Python BASIC oracle imports pinned helpers and computes source-derived facts; its provenance is valuable but does not execute C++ `ConnectBlock`. Synthetic-root/manual-work/custom-maturity evidence in v2.5 was deliberately bounded. Generating separate histories in the two implementations also compares different input bytes.

**How to avoid:** Execute a provenance-verified Knots binary from the exact pin in an owned isolated datadir. Feed identical serialized blocks/transactions to both engines; record ordered input digest, canonical root/network, executable/build identity, arguments, clock, results and resulting tips/work/selected coins/filter commitments. Retain formula/vector tests as a distinct evidence class. Disable assumevalid shortcuts for ordinary-validation claims; do not use trusted snapshot imports or activation overrides to make a corpus pass.

**Warning signs:** Report contains only Python output or a version string; executable provenance is not tied to source; two generators mine separately; expected results come from Open Bitcoin; missing Knots binary becomes a silent passing skip.

**Phase to address:** Differential Harness; first deliver a small smoke case early, then reuse the same process boundary for later proof. HIGH provenance gap; harness implementation confidence MEDIUM. [L5, A1, K1, K5]

### Pitfall 8: Rejection Classification Is Over-Normalized

**What goes wrong:** A timeout, missing parent, duplicate, policy refusal or side-branch storage result is counted as the same consensus rejection; invalid inputs contaminate later cases.

**Why it happens:** Knots `submitblock` exposes BIP22 result handling and can return `inconclusive`; its cached-invalid/duplicate behavior depends on prior submission state. Mempool policy and block consensus have different contracts. Comparing formatted debug messages alone misses ordering and result categories.

**How to avoid:** Capture raw endpoint response, typed local result and post-state. Define a narrow comparison adapter that preserves acceptance/rejection/inconclusive and stable rejection reasons; retain debug text separately unless exactness is an explicit requirement. Use fresh contexts for independent invalid cases and deliberate repeated submissions for duplicate/cached-invalid tests. Mutate one intended rule after constructing an otherwise valid candidate; classify unsupported cases openly.

**Warning signs:** Broad catch maps every error to rejected; only substring assertions exist; `submitblock` success is assumed to mean active-tip selection; invalid cases depend accidentally on execution order; transport failure passes an expected rejection test.

**Phase to address:** Differential Harness owns comparison semantics; Fork and Recovery owns failure-state preservation and retry/cached-invalid evidence. HIGH source confidence. [K1, K5]

### Pitfall 9: Historical Samples Lack the Context That Makes Them Valid

**What goes wrong:** An authentic mainnet block is rejected because its parent history, UTXOs, retarget anchor, median time, deployment state or clock is missing; a fixture wrapper fabricates those facts and overstates acceptance.

**Why it happens:** Raw bytes and genuine hashes establish identity, not complete validation context. Sparse later-height corpora cannot establish ordinary genesis-to-tip acceptance. Bounded regtest mining is easy, while mainnet PoW and historical inputs make full replay expensive.

**How to avoid:** Use canonical regtest for continuous whole-body differential proof. Use bounded genuine historical-network prefixes or explicitly labeled context/vector fixtures for buried deployments, difficulty transitions and hash exceptions. Record the exact context and provenance for each sample; pin candidate time and reference clock without rewriting historical bytes. A context-seeded predicate test must not become a whole-chain acceptance receipt.

**Warning signs:** Corpus names claim mainnet acceptance with no ancestors/spent outputs; timestamp failures are bypassed globally; height is patched into the candidate; regtest proves mainnet retarget behavior despite no-retarget parameters; latest-era scripts are assumed covered by an early prefix.

**Phase to address:** Validation Context defines minimum historical inputs; Differential Harness defines evidence tiers and byte/time/count budgets. MEDIUM strategy confidence; exact corpus selection remains open. [K1, K2, L2, A1]

### Pitfall 10: New Chainwork Is Reinterpreted from Old Durable State

**What goes wrong:** A legacy synthetic datadir is accepted as canonical by zero-extending old work, or a partial conversion leaves coins, header metadata and BASIC/prune fences on different identities.

**Why it happens:** Snapshot DTOs already persist caller-selected work; safe checkpoint/reopen is broader than changing the integer width. Pruned blocks may prevent full replay, and prior completion does not certify canonical root/validation context.

**How to avoid:** Define versioned canonical identity/work schemas and compatibility checks before write paths change. Prefer safe explicit refusal of incompatible legacy/synthetic data over implicit repair/import. Recompute only where the required trustworthy ancestry and validation provenance are established; header-derived proof alone cannot promote body validation. Preserve existing data on refusal. Test failures between coins, chain metadata, header checkpoint and filter/protection publication using real Fjall handles, then drop every handle and reopen.

**Warning signs:** Deserialization treats old `u128` as trusted exact work; changing network rewrites metadata; schema upgrade clears coins/filters; reopen passes with contradictory best-block/branch identities; a cloned live handle is called restart evidence.

**Phase to address:** Bootstrap defines compatibility policy; Work Authority owns schema; Fork and Recovery owns interruption matrix. HIGH source hazard; specific migration support is intentionally unresolved. [L3, L4, A1]

### Pitfall 11: Canonical Prune/BASIC Proof Reuses Synthetic Success Claims

**What goes wrong:** Filters work on a canonical prefix, but ordinary retention, branch replacement or actual reopen breaks protection; old shortcuts still fund wallets from genesis or promise consensus differential parity.

**Why it happens:** v2.5 established real prune/index boundaries using explicitly limited synthetic roots/work/halving fixtures. Canonical regtest now crosses real maturity, height-150 halvings and the ordinary 1,000-height prune threshold; replacing fixture roots changes receipts and hardcoded proof contracts throughout scripts/docs.

**How to avoid:** Drive one continuous accepted canonical history through enabled indexing, scheduled catch-up, validated fork replacement, checkpoint, actual paired deletion and all-handle reopen. Assert genesis filter, spent-script completeness, branch-correct immutable lookup, conservative safe progress and protection before deletion. Distinguish ordinary threshold from Knots `-fastprune` test mode and soft logical target from physical reclamation. Keep missing-history/body/undo refusal and post-prune wallet restrictions.

**Warning signs:** Prune is asserted from metadata without checking absent payload keys; safe prefix advances beyond durable coins; reopen skips startup deletion; retained filters grant body/wallet availability; fixed-height fixture funding fails after reward halving; historical v2.5 provenance is rewritten as new canonical evidence.

**Phase to address:** Canonical Prune/BASIC after Fork and Recovery; Final Evidence updates current guardrails while preserving historical receipts. HIGH inherited limits; new canonical integration evidence is still absent. [A1, L5, K2, K4]

## Technical Debt Patterns

| Shortcut                                         | Immediate Benefit        | Long-term Cost                           | When Acceptable                                         |
| ------------------------------------------------ | ------------------------ | ---------------------------------------- | ------------------------------------------------------- |
| Keep `u128` authority plus exact-work display    | Small diff               | Contradictory forks/recovery             | Never in canonical production paths                     |
| General `is_genesis = parent == 0`               | Reuses synthetic helpers | Wrong network/root acceptance            | Explicitly segregated synthetic unit fixtures only      |
| Always-on deployment booleans or P2SH-only flags | Easier fixtures          | Historical false acceptance/rejection    | Named synthetic test contexts, not canonical evidence   |
| Auto-convert legacy scores                       | Smooth startup           | Certifies fabricated work and validation | Never without sufficient explicit validation/provenance |
| Reuse formula oracle as node acceptance          | Fast suite               | False differential claims                | Formula/filter vectors with accurate evidence labels    |
| Copy Knots internal pointer tie-breaker          | Apparent source fidelity | Unstable durable semantics               | Never; establish observable behavior instead            |

## Integration Gotchas

| Integration        | Common Mistake                              | Correct Approach                                                                              |
| ------------------ | ------------------------------------------- | --------------------------------------------------------------------------------------------- |
| Knots executable   | Trust PATH/version text                     | Exact pin, build/executable identity, explicit path and recorded invocation                   |
| Knots RPC          | Treat `submitblock` result as tip           | Record raw result and independently query tip/work/UTXO state                                 |
| Genesis and BASIC  | Derive filter from coins                    | Body-derived filter, no genesis undo, null previous filter header                             |
| Canonical recovery | Initialize after prune intent resumes       | Identity/recovery/protection checks before destructive maintenance                            |
| Archived guards    | Replace old literals/digests to force green | Preserve historical mode; add separate current-source evidence and compatible guard ownership |

## Performance Traps

| Trap                                      | Symptoms                     | Prevention                                                               | When It Breaks                             |
| ----------------------------------------- | ---------------------------- | ------------------------------------------------------------------------ | ------------------------------------------ |
| Process per vector/block                  | Thousands of startup costs   | Bounded corpus sessions; fresh process only for required state isolation | Boundary/fork matrix growth                |
| Copy full ancestry/UTXOs for every branch | Quadratic memory/time        | Shared ancestry/deltas and fixed branch/byte/block limits                | Canonical prune history above 1,000 blocks |
| Mine mainnet difficulty during tests      | Unbounded CPU                | Checked-in genuine bounded fixtures; mine only canonical regtest         | First historical proof-of-work case        |
| Unbounded logs or hangs                   | Disk growth/orphan processes | Bound output; stage deadlines, liveness evidence and owned-child cleanup | First failed RPC/startup/shutdown          |
| Rebuild both stacks in each test          | Verification explosion       | Build provenance once per relevant source and reuse verified artifact    | Full native suite plus differential matrix |

## Security Mistakes

| Mistake                                   | Risk                                           | Prevention                                                                                       |
| ----------------------------------------- | ---------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| Reuse operator datadir/config             | Writes to real node state or reads credentials | Fresh owned temp roots, explicit config, local credentials and deletion bounded to owned paths   |
| Let Knots defaults discover/connect peers | External traffic and nonhermetic state         | Explicit no-discovery/no-connect policy, controlled loopback RPC and P2P only if test needs it   |
| Stop processes by executable name/port    | Kill unrelated user node                       | Track child PID/process ownership; graceful stop then bounded escalation only for owned children |
| Trust supplied cumulative work            | Adversarial branch/resource selection          | Derive from validated targets/ancestry; preserve header/body validation distinction              |
| Use broad error fallback                  | Harness faults counted as parity               | Typed failures, raw logs and fail-visible missing prerequisites                                  |

## UX Pitfalls

| Pitfall                                 | User Impact                          | Better Approach                                                                    |
| --------------------------------------- | ------------------------------------ | ---------------------------------------------------------------------------------- |
| Automatic repair of incompatible state  | Unreviewed mutation/data loss        | Actionable nonmutating refusal with expected/observed network/root/schema          |
| “Consensus parity” from a finite corpus | False readiness expectations         | Publish networks, contexts, corpus digests, observed cases and residual exclusions |
| “Fully synced” from headers/work only   | Confuses proof with validated bodies | Separate header tip, active validated tip, durable coins and safe BASIC prefix     |
| Missing binary reported as success      | Hidden unverified behavior           | Explicit unavailable/failing acceptance status and exact prerequisite              |

## "Looks Done But Isn't" Checklist

- [ ] Empty ordinary datadir retains the configured canonical genesis across actual reopen, with no genesis UTXO.
- [ ] Wrong root/network and incompatible legacy schemas refuse before mutation or prune resumption.
- [ ] Regtest uses maturity 100 and halving 150 without rule overrides; historical contexts match pinned boundaries/exceptions.
- [ ] Exact target-derived 256-bit work owns every live/durable consumer; no caller-selected production score remains.
- [ ] Header-only, body-valid, scripts-valid, invalid and active identities remain distinct; equal-work behavior has process evidence.
- [ ] Verified pinned Knots processes receive identical bytes and preserve raw rejection/inconclusive/duplicate outcomes.
- [ ] Fork/rejection tests compare state before/after, then checkpoint, close all owners and reopen actual Fjall storage.
- [ ] Canonical BASIC/prune proof checks absent paired payloads and retained filter commitments without restoring bodies or wallet eligibility.
- [ ] Final claims identify tested histories and untested networks/deployments; old archived evidence keeps its original scope.

## Recovery Strategies

| Pitfall                                  | Recovery Cost | Recovery Steps                                                                                       |
| ---------------------------------------- | ------------- | ---------------------------------------------------------------------------------------------------- |
| Wrong root or synthetic legacy datadir   | HIGH          | Refuse unchanged; preserve evidence/data; use fresh explicitly selected canonical test datadir       |
| Work/coins/index identity mismatch       | HIGH          | Stop maintenance/serving; retain markers and payloads; apply documented recovery or explicit refusal |
| Oracle process startup/transport failure | LOW           | Preserve bounded logs; stop only owned child; rerun from clean owned fixture context                 |
| Corpus context missing                   | MEDIUM        | Reclassify as vector evidence or supply genuine required history; never fabricate passing acceptance |
| New genesis broke archived guards        | MEDIUM        | Preserve old receipts; add canonical evidence/contracts and update only current claim surfaces       |

## Pitfall-to-Phase Mapping

| Pitfall                                      | Prevention Phase      | Verification                                                                      |
| -------------------------------------------- | --------------------- | --------------------------------------------------------------------------------- |
| Wrong root; genesis outputs/filter confusion | Bootstrap             | Canonical bytes, empty coins, partial-init/wrong-network and reopen controls      |
| Historical rules/subsidy/exceptions          | Validation Context    | Before/at/after boundaries; exact pinned context/flag comparison                  |
| Synthetic work; equal-work conflation        | Work Authority        | Unequal-target and >128-bit vectors, candidate ordering, durable consistency      |
| Formula-only or over-normalized oracle       | Differential Harness  | Pinned actual process, identical digest, raw outcomes, post-state                 |
| Failed forks or split durable authority      | Fork and Recovery     | Invalid ancestry, no partial state, required-data refusal and actual close/reopen |
| Lost prune protection/canonical filters      | Canonical Prune/BASIC | Real paired deletion plus retained exact filters/branches across reopen           |
| Stale hardcoded claims                       | Final Evidence        | Native source guards, current evidence manifest and preserved historical limits   |

## Sources and Remaining Questions

- **[K1] HIGH:** [Pinned validation.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp), inspected locally: `GetBlockSubsidy`, `GetBlockScriptFlags`, `ConnectBlock`, `AcceptBlockHeader`, `LoadGenesisBlock`, contextual checks and BIP30 exceptions.
- **[K2] HIGH:** [Pinned kernel/chainparams.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp): genesis/network identities, buried deployments, script exceptions, signet/testnet4 and regtest interval 150.
- **[K3] HIGH:** [Pinned chain.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp) `GetBlockProof`; [node/blockstorage.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp) `CBlockIndexWorkComparator`, and local `chain.h` sequence/disk comments.
- **[K4] HIGH:** [Pinned blockfilter.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/blockfilter.cpp) `BuildFilterElements`; [index/blockfilterindex.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/index/blockfilterindex.cpp) `CustomAppend` and retained hash lookup.
- **[K5] HIGH:** [Pinned rpc/mining.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/rpc/mining.cpp) `BIP22ValidationResult`, `submitblock`; local functional `test_framework/test_node.py` for process/clock ownership patterns.
- **[K7] HIGH:** Local pinned `src/validation.cpp` `CheckBlock` calls `CheckSignetBlockSolution`; `src/deploymentstatus.cpp` and `src/versionbits.cpp` require dedicated phase research for any generalized deployment promises.
- **[L1] HIGH:** Local `packages/open-bitcoin-chainstate/src/engine/overlay_apply.rs`, `engine/apply.rs`, `engine.rs`, `types.rs`: root shape, output connection, caller-selected work and tie-breaking.
- **[L2] HIGH:** Local `packages/open-bitcoin-consensus/src/context.rs`, `block.rs`, `block/difficulty.rs`, `block/witness.rs`: fixed rule booleans, genesis contextual handling and available script flags.
- **[L3] HIGH:** Local `packages/open-bitcoin-node/src/sync/{types,open_runtime}.rs`, `network/{header_sync,inventory}.rs`: shared 210,000 interval, fixed P2SH and incremented work; `storage/snapshot_codec.rs` stores `u128` work.
- **[L4] HIGH:** Local `packages/open-bitcoin-network/src/header_store.rs` and `header_store/proof.rs`: synthetic best-tip authority alongside existing exact `ProofWork` derivation.
- **[L5] HIGH:** Local `scripts/basic-filter-oracle.py` and phase162/163 proof contracts: source-derived oracle and existing synthetic/history/provenance constraints.
- **[A1] HIGH as historical evidence:** [Archived v2.5 audit](../milestones/v2.5-MILESTONE-AUDIT.md) and [PROJECT](../PROJECT.md): synthetic-root/work/halving limitations and inherited v2.4 stale-metadata, generic unlink-default and crash-counter advisories remain explicit.

**Open decisions:** Exact supported network/deployment matrix; safe schema refusal versus explicitly supported migration; process-observable equal-work/restart promises; finite historical corpus selection; provenance-verifiable Knots executable provisioning. These need phase research and implementation evidence. No new runtime result in this document resolves them.
