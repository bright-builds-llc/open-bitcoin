# Architecture Research

**Domain:** Open Bitcoin v2.6 canonical chain bootstrap and bounded validation parity
**Researched:** 2026-10-13
**Confidence:** HIGH for inspected source behavior; MEDIUM for proposed integration
**Baseline:** Bitcoin Knots `29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`

This is a proposed architecture, not executed v2.6 validation evidence. It follows repository `AGENTS.md`, `AGENTS.bright-builds.md`, the placeholder-only overrides, and the architecture, verification, testing and Rust standards. Research runs inside the authorized new-milestone workflow; it changes no production code or saved datadir. Initial executable evidence should cover real default-regtest chains and bounded, classified mainnet historical rule-context fixtures. Canonical identities for other networks do not establish their validation support; unsupported modes refuse before effects.

## Standard Architecture

### System Overview

Keep the existing first-party crates and single active Fjall chainstate. Establish canonical identity and history-derived rules in the pure core; execute their decisions through the existing managed runtime owner.

```text
Offline fixtures / explicit operator configuration / opt-in peers
                             |
                 codecs + resolved chain identity
                             |
       consensus: canonical genesis, rules, target-derived ProofWork
                             |
       network: admitted header ancestry + exact-work candidate
                             |
       chainstate: contextual connect / disconnect / reorg staging
                             |
       node: existing managed owner + lifecycle receipts
                             |
       Fjall: identity, headers, coins/B/H, undo, bodies, BASIC
                             |
       RPC / CLI / status / wallet eligibility projections

Test-only Bun runner -> actual pinned Knots and Open Bitcoin
                      -> identical serialized input + independent observations
```

### Existing Seams and Required Changes

| Component                            | Inspected current behavior                                                                                                       | v2.6 change                                                                                                                  |
| ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `consensus/chain_proof.rs`           | `ProofWork([u64;4])`, exact proof, ordering and Knots-style wrapping arithmetic already exist                                    | Reuse this arithmetic; add explicit fixed-width encoding/display and one authoritative cumulative-work API                   |
| `consensus/context.rs`               | Caller supplies `ConsensusParams` and `ScriptVerifyFlags`; defaults describe one modern regtest-like profile with 210000 halving | Introduce immutable network rules plus per-candidate history-derived validation context                                      |
| `chainstate/types.rs`                | `ChainPosition`, `AnchoredBlock` use caller-supplied `u128`; genesis checks only zero parent                                     | Replace work representation and constructors; require canonical root identity                                                |
| `network/header_store.rs`            | Zero-parent header becomes root; work score is 1; fork ordering uses work, height, hash                                          | Canonical root admission, contextual headers, exact work and explicit candidate ordering                                     |
| `network/header_store/proof.rs`      | Separate exact arithmetic facts and best-proof tip coexist with synthetic fork choice                                            | Merge arithmetic into the authoritative header entries; retain derived diagnostics without a second competing tip            |
| `chainstate/engine/overlay_apply.rs` | Empty chain accepts a zero-parent block and applies its transactions                                                             | Dedicated canonical genesis stage that creates no genesis UTXO; ordinary block path requires initialized ancestry            |
| `node/sync/open_runtime.rs`          | Recovers coins/meta/undo and headers, initializes BASIC owner, fixes runtime flags to P2SH                                       | Identity/read-only compatibility gate before recovery mutations; bootstrap before BASIC catch-up; derive flags per candidate |
| `node/sync/types.rs`                 | Four network selectors; all use 210000 halving and unconditional BIP34/BIP113/Segwit booleans                                    | Resolve selectors into one core chain specification; regtest interval 150; preserve network-specific rules                   |
| `node/sync/block_reconcile.rs`       | Copies header `u128` work into connect/replacement paths and compares candidates                                                 | Consume shared exact-work entries and branch-specific contexts; keep staged lifecycle ownership                              |
| `node/storage/snapshot_codec*`       | Chain/header DTOs persist `u128`; store schema is 2; blobs 1/2 remain readable                                                   | Version affected record formats and provenance; refuse incompatible chain authority before mutation                          |
| `node/storage/fjall_store/coins.rs`  | Open can automatically migrate schema-1 coins and write schema metadata                                                          | Separate format compatibility from canonical validation provenance; do not let old automatic migration confer v2.6 authority |
| Existing prune/BASIC owner           | Accepted-state receipts, immutable retained filters, coins fences, recovery and missing-history refusal                          | Reuse owner and receipts with canonical identities; adapt genesis and exact-work records without bypassing fences            |

The v2.5 audit explicitly excludes whole-chain differential claims because of synthetic roots/work and the regtest subsidy mismatch. Its receipts remain historical evidence; they cannot certify the revised bootstrap. [S1–S5]

## Recommended Project Structure

Add focused modules inside existing crates, with names finalized during phase planning:

```text
packages/open-bitcoin-consensus/src/
  chain_params.rs          # new: pure chain specification and canonical genesis
  deployment.rs           # new: buried/versionbits history decisions
  context.rs              # modify: resolved candidate consensus context
  chain_proof.rs          # modify: existing exact work becomes shared authority
packages/open-bitcoin-network/src/
  header_store.rs         # modify: admitted ancestry/work/candidate policy
packages/open-bitcoin-chainstate/src/
  types.rs                # modify: canonical positions and exact work
  engine/overlay_apply.rs # modify: genesis versus ordinary transitions
packages/open-bitcoin-node/src/
  chainstate/             # modify: startup and accepted transition orchestration
  storage/                # modify: identity/provenance schema, compatibility gate
  sync/                   # modify: candidate contexts and checkpoint/recovery
scripts/
  canonical-chain-oracle.ts # new: bounded test-only actual Knots orchestration
```

Use the existing primitives `Block`, `BlockHeader`, `NetworkMagic`, `BlockHash`, codecs, `ProofWork`, `CoinsOverlay` and accepted receipts. A resolved `ChainSpec` belongs below `open-bitcoin-node`, avoiding a core dependency on shell-owned `SyncNetwork`. Genesis bytes may live as audited constants/fixtures or a pure constructor; always verify serialized bytes, hash and merkle root against the pin. No production Rust Bitcoin library or new runtime service is needed.

## Architectural Patterns

### Pattern 1: Canonical Root as a State Transition

**What:** Bootstrap establishes chain identity, header/index/body and coins best-block consistently; genesis transaction outputs never enter spendable coins.
**Why:** Knots `ConnectBlock` skips genesis transaction connection and sets best-block. This is a special genesis rule, not OP_RETURN filtering, coinbase immaturity, or a generally invalid output script. The familiar P2PK genesis output remains in its serialized body and BASIC filter input. [S6]

Proposed bootstrap protocol:

1. Resolve network and any explicitly supported regtest/signet parameters into immutable identity.
1. Inspect compatibility/provenance before legacy schema migration, index recovery, pruning or replacement writes.
1. On a genuinely empty compatible datadir, stage the exact canonical genesis header/body/index, height 0, `block_proof(genesis.bits)` and genesis MTP.
1. Establish empty genesis coins state with best-block at genesis; create only the metadata/undo representation required by existing recovery and filter contracts. Do not invent a Knots genesis undo transaction.
1. Commit through the managed owner using replay-safe initialization markers; repeated open validates/completes its own bounded bootstrap or refuses inconsistent partial state.
1. Only then initialize/catch up explicitly configured BASIC and permit ordinary validation.

Reject arbitrary zero-parent roots even if their PoW is valid; prevent ordinary reorg/disconnect from removing genesis. Genesis is not downloaded as an unknown peer root. Distinguish absent state from mismatched or partially populated state. A header-only checkpoint is not body-validated chainstate.

### Pattern 2: One Work Number, Separate Validation Provenance

**What:** Every admitted header stores `parent.work + block_proof(bits)` using the existing exact 256-bit arithmetic. Genesis accumulates from zero. Chain positions, candidates, replacement branches, checkpoints, DTOs and status consume that same value.
**Trade-off:** This touches many signatures and fixtures, but avoids a permanent split between diagnostic correct work and operative synthetic work. Arithmetic alone proves neither valid difficulty nor canonical ancestry nor script validity. [S2, S7]

Admit headers only after identity, parent, PoW limit, expected difficulty, timestamp/MTP and version rules for their network/history are established. Decode persisted work losslessly and recompute/check it against ancestry; never trust a saved number because its width is correct. Preserve Knots arithmetic semantics rather than substituting saturation. [S7, S8]

Keep header-valid and fully connected facts distinct in types/receipts. Best header drives download selection; active chain changes only after contextual block/UTXO/script validation and accepted transition installation. Remove caller-selected work from production entrypoints; test fixture helpers can compute genuine proofs rather than mint trusted authority.

Equal work needs its own parity decision. Knots block candidates use activation sequence before an address tie-break; current Open Bitcoin uses height/hash. Do not claim exact Knots equal-work behavior from a new numeric type. Preserve arrival/activation facts where observable, test no unnecessary equal-work reorg, and declare unresolved restart ties explicitly rather than copying pointer addresses. [S9]

### Pattern 3: Rules Derived for the Candidate's Branch

**What:** Resolve a pure candidate context from chain specification, candidate hash/header and validated ancestor facts. Supply clock observations and history as data; keep filesystem/network/cache loading in the shell.

Separate:

- Static network identity, genesis, maturity, subsidy schedule, PoW parameters and hash-specific exceptions.
- Buried height activations and contextual version/BIP34/CSV/Segwit commitment rules.
- Branch-dependent versionbits state, period/MTP boundaries, thresholds and minimum activation height where the pinned behavior requires them.
- Block consensus script flags versus mempool standardness/policy flags.

**Pinned nuance:** `GetBlockScriptFlags` starts with P2SH/WITNESS/TAPROOT, applies exact block-hash exceptions, then adds DERSIG/CLTV/CSV and NULLDUMMY at their pinned deployment boundaries. A generic “turn every flag on at activation height” implementation is wrong for this baseline. Segwit contextual checks and versionbits APIs are separate from that script-flag rule. [S6, S10]

Reorg validation derives each replacement block's context from the retained common prefix plus the preceding replacement headers. Never reuse the displaced active tip's flags or a process-global deployment cache without branch identity. Missing required history yields a typed refusal; an unproven context cannot default to all-active.

### Pattern 4: Actual Differential Oracle, Reproducible Evidence

**What:** A test-only Bun runner starts a locally built, exact-pinned Knots executable in an isolated temporary regtest datadir and feeds identical serialized blocks/transactions to both ordinary validation paths. Existing Python BASIC oracle remains narrow protocol evidence, not a consensus daemon substitute. [S11]

Record source pin, executable digest/build provenance, effective consensus settings, input bytes/digests, ordering, raw outcomes and compared observations. Disable external peers/discovery, ambient datadir/config/wallet use and assumevalid shortcuts; use explicit local process/RPC transport. Freeze time where required without making it part of consensus core I/O. Availability of an actual compatible binary is a required gate, not permission to silently replace it with golden expected output.

Compare accept/reject categories/reasons where stable, tip hash/height/exact work, selected spendable outpoints and disconnect/reconnect results. Do not compare incompatible serialized UTXO database formats. Knots accepting the same block is independent evidence only if its ordinary checks actually ran; self-generated fixtures and matched expected JSON alone are insufficient.

## Data Flow

### Startup and Recovery

```text
Resolved chain identity + storage read-only compatibility inspection
    -> Empty compatible store: canonical bootstrap
    -> Compatible canonical store: verify ancestry/work + recovery markers
    -> Unknown/synthetic/mismatched store: typed refusal; preserve source bytes
    -> Existing initialize/coins recovery -> managed active chain
    -> BASIC ownership/catch-up -> optional runtime networking
```

### Ordinary Block and Fork Flow

```text
Decoded header -> branch context -> admitted exact-work header
    -> candidate selection -> required retained bodies/undo preflight
    -> staged contextual validation on coins overlay
    -> existing accepted lifecycle receipt / managed commit
    -> coins checkpoint + header/meta/body/undo durability
    -> branch-correct BASIC append/replacement -> pruning eligibility
    -> RPC/CLI/status projection of achieved facts
```

Preserve the existing crash-window and flush contracts: prepared, accepted, checkpointed and recovered facts are distinct. A successful in-memory connect does not certify durable reopen. Filter retention never restores deleted bodies or wallet eligibility.

## Cross-Layer Compatibility and Migration

Use a new versioned canonical-chain authority envelope recording network identity, genesis hash, consensus-profile identity (including supported overrides/challenge), work encoding and validation provenance. A genesis hash alone cannot distinguish custom signets because their genesis is shared; challenge-derived identity matters. Mainnet and testnet3 also share the genesis transaction/merkle root, so do not key network identity from those fields. [S12]

| Saved state                                       | Proposed treatment                                                                       |
| ------------------------------------------------- | ---------------------------------------------------------------------------------------- |
| Truly empty                                       | Explicit canonical initialization; bootstrap crash/retry tests                           |
| New canonical format, matching identity           | Validate envelopes and authority links, then existing recovery                           |
| Wrong network/challenge/profile                   | Refuse before mutable recovery; no automatic switching                                   |
| Old synthetic root or unproven validation context | Refuse canonical runtime activation; retain source datadir and diagnostic provenance     |
| Old canonical-looking root with `u128` work       | Root alone is insufficient; no automatic authority upgrade or reinterpretation           |
| Pruned legacy history                             | Refuse replay when required bodies/history are missing; retained BASIC cannot substitute |
| Malformed or interrupted new bootstrap            | Bounded finish of its own validated intent or typed refusal; no generic repair           |

The safest initial release supports fresh datadirs and proven new-format reopen. A later explicit dry-run-first export/replay workflow can rebuild into a separate destination when complete canonical blocks exist. It must revalidate consensus, work, coins and dependent filter fences; merely widening integers cannot repair synthetic roots, historical flags or genesis coins. Do not overwrite, delete, reset, auto-import/download, or “repair” existing state as a startup side effect.

Keep wallet/mempool blob readability separate from chain authority versions. Map every persisted carrier: active metadata, header checkpoint, runtime/reorg checkpoint, coins B/H markers and accepted history/fences referencing their identities. Preserve unknown facts as unavailable instead of fabricating canonical provenance.

## Integration Points and Safest Build Order

| Proposed phase                           | Deliverable and dependency                                                                                                | Acceptance gate                                                                                  |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| 1. Chain identity and compatibility      | Core chain specs, network mapping, canonical genesis fixtures, read-only legacy refusal                                   | Exact pinned genesis identities; no changed source datadir on refusal                            |
| 2. Exact work foundation                 | Extend existing `ProofWork` encoding and migrate live/header/chainstate signatures before declaring canonical persistence | Known target proofs, >128-bit ordering, no synthetic production work path                        |
| 3. Canonical durable bootstrap           | New authority schema, genesis connect semantics, complete startup/reopen path; depends on 1–2                             | Empty/repeat/partial initialization, genesis unspendability, wrong network refusal               |
| 4. Candidate rule derivation             | Buried/history rules, script exceptions, subsidy, contextual header/block checks; depends on 1–3                          | Genuine maturity 100 and regtest 150 halving; historical boundary controls                       |
| 5. Actual Knots differential             | Hermetic runner/provenance, ordinary shared-byte accept/reject corpus; runner scaffolding can begin earlier               | Real pinned executable, canonical roots, no weakened consensus test profile                      |
| 6. Fork and recovery differential        | Work-based branches, disconnect/reconnect, failed replacement, checkpoint and all-handle reopen; depends on 4–5           | Same branch choice and coins observations; refusal on missing required data                      |
| 7. Canonical pruning and BASIC           | Reuse existing ownership/fencing; real genesis filter, retained/stale branch and actual paired loss/reopen                | Exact filters/commitments, honest availability, no deleted-body recovery claim                   |
| 8. Historical contexts and claim closure | Bounded classified mainnet fixtures, supported-network/refusal matrix, native integration                                 | Every claim names network/profile/history and executed evidence; offline default verifier passes |

Six milestone deliverables can therefore become eight phases. Implement only deployment modes necessary for the accepted profiles; validate bounded historical context decisions independently. Dynamic deployment overrides and complete signet support are separate scope decisions, with early typed refusal as the safe bounded alternative. Do not release a partially migrated authority between work/schema phases; use explicit compatibility gating until the complete path exists.

## Scaling Considerations and Anti-Patterns

Bound corpus size, retained branch depth, child-process lifetime and fixture bytes. Real regtest maturity and canonical prune thresholds make integration scenarios longer than maturity-one fixtures; separate small pure-core vectors from full-path canonical scenarios. Existing `Vec<ChainPosition>`, all-undo loading and finite reorg caps remain scaling limits, not reasons to add archive-scale infrastructure.

Avoid: two best-work authorities; trusted caller totals; rewriting old metadata to look canonical; static global flags; genesis coinbase insertion; branch-global deployment caches; golden-output “differential”; pruning before history preflight; BASIC as a substitute for bodies; silent skipped oracle runs; public-network-dependent native checks.

## Material Scope Risks and Honest Claim Gates

- **Network coverage:** Current `SyncNetwork` has mainnet/testnet3/signet/regtest; the pin also contains testnet4/BIP94. Explicitly decide whether to add it or document a bounded omission. Do not silently reinterpret `testnet` as testnet4. Regtest activation/versionbits overrides and custom signet are separate profiles, not automatic scope.
- **Signet:** No signet validation path was found in the inspected consensus crate. Proof-of-work plus signet genesis does not validate non-genesis signet blocks; implement the challenge/solution checks or refuse unsupported signet validation. Default and custom challenge/magic identity require explicit fixtures. [S12]
- **Historical completeness:** Buried heights alone do not cover BIP30 duplicate-transaction exceptions, exact historical script exceptions, difficulty recovery/retarget context or dynamic deployment histories. Inventory against the pin; bounded fixture admission must say which context it proves.
- **Equal work and recovery:** Work correctness does not alone prove candidate arrival order or restarted tie behavior. Resolve observable rules and record any intentional difference through `docs/parity/`.
- **Evidence:** Existing v2.5 synthetic/maturity-one proofs remain useful regressions with their original labels. New genuine canonical/ordinary validation receipts are required before stronger v2.6 claims.
- **Release:** Preserve `bash scripts/verify.sh` as the default offline contract; integrate deterministic bounded suites and required oracle provisioning without implicit downloads/public peers. No tests/builds were executed by this research.
- **Deferred:** Single chainstate only; default-off networking; no assumeutxo, archive-scale, unattended production full sync, production service/funds safety or complete consensus-parity assertion.

## Sources

All Knots references below use the exact milestone pin; local source inspection is the primary evidence. BIP pages are current specifications checked on the research date, not a replacement for pinned behavior.

- **S1 — HIGH:** [Current project](../PROJECT.md) and [archived v2.5 audit](../milestones/v2.5-MILESTONE-AUDIT.md), especially preserved limits.
- **S2 — HIGH:** Local `packages/open-bitcoin-consensus/src/chain_proof.rs`, `packages/open-bitcoin-network/src/header_store.rs` and `header_store/proof.rs`.
- **S3 — HIGH:** Local `packages/open-bitcoin-chainstate/src/types.rs`, `engine/overlay_apply.rs`, `engine/stage.rs`, `engine/apply.rs`.
- **S4 — HIGH:** Local `packages/open-bitcoin-node/src/sync/{types,open_runtime,block_reconcile,runtime_state}.rs` and `network/mempool_lifecycle.rs`.
- **S5 — HIGH:** Local `packages/open-bitcoin-node/src/storage.rs`, `storage/snapshot_codec.rs`, `storage/snapshot_codec/chain_meta.rs`, `storage/fjall_store/coins.rs`.
- **S6 — HIGH:** [Pinned validation.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/validation.cpp#L2645), script flags, genesis special case, subsidy and contextual validation.
- **S7 — HIGH:** [Pinned chain.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/chain.cpp#L131), `GetBlockProof`; [arith_uint256.h](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/arith_uint256.h).
- **S8 — HIGH:** [Pinned chain parameters](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/kernel/chainparams.cpp), genesis, network heights, subsidy and signet/testnet4 identity.
- **S9 — HIGH:** [Pinned blockstorage.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/node/blockstorage.cpp#L194), candidate comparator.
- **S10 — HIGH:** [Pinned deploymentstatus.h](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/deploymentstatus.h), [versionbits.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/versionbits.cpp), [BIP9](https://github.com/bitcoin/bips/blob/master/bip-0009.mediawiki).
- **S11 — HIGH:** Local `scripts/phase162-filter-oracle.ts`, `scripts/basic-filter-oracle.py` and `scripts/verify.sh`: present independent BASIC helper oracle and native registration, not an actual daemon consensus differential.
- **S12 — HIGH:** [BIP325](https://github.com/bitcoin/bips/blob/master/bip-0325.mediawiki) and [pinned signet.cpp](https://github.com/bitcoinknots/bitcoin/blob/a9aee730466ac67d35a3c03ee24676be5e045878/src/signet.cpp), challenge validation and shared signet genesis.
