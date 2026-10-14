# Requirements: Open Bitcoin

**Milestone:** v2.6 Canonical Chain Bootstrap and Validation Parity
**Defined:** 2026-10-13
**Status:** Approved by the user on 2026-10-13; all 30 requirements remain Pending.
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## Scope and Evidence Contract

The behavioral reference is Bitcoin Knots `29.3.knots20260210`, pinned at `a9aee730466ac67d35a3c03ee24676be5e045878`. User acceptance covers the six milestone deliverables in [PROJECT.md](PROJECT.md). Requirements below define approved observable completion criteria, not delivered functionality.

Genuine default-regtest histories use the canonical genesis, normal 100-block coinbase maturity, 150-block halvings, real serialized blocks and transactions, and internally derived work. Mainnet historical checks must name their exact block/rule context and distinguish independently seeded contextual fixtures from ordinary ancestry-complete chain validation. Network selection is not proof of complete validation support: each currently exposed mode must have an explicit support/evidence classification, and missing consensus prerequisites must refuse before accepting unsupported history. No testnet4 or custom-network support is added implicitly.

Preserve one active Fjall chainstate, first-party Bitcoin domain ownership, functional-core/shell boundaries, explicit/default-off public networking and offline native verification. Live public-network runs remain optional UAT. A real pinned-Knots local oracle is required for differential acceptance; source-derived helpers, frozen corpora or Open Bitcoin self-comparison alone cannot substitute for that receipt.

## v2.6 Requirements

### Canonical Bootstrap

- [ ] **CBOT-01**: An operator's selected existing network binds the accepted root to pinned canonical genesis header/body identity; a different null-parent root is rejected.
- [ ] **CBOT-02**: An operator can initialize and reopen an empty durable datadir with canonical genesis as best block, while the genesis coinbase contributes no spendable UTXOs and interrupted initialization never publishes partial authority.
- [ ] **CBOT-03**: An operator can enable BASIC indexing on a fresh canonical datadir, obtain its genesis filter and zero-predecessor commitment, and continue ordinary indexing without seeded history or implicit public-network activation.
- [ ] **CBOT-04**: An operator opening a foreign-network or noncanonical-root datadir receives a bounded diagnostic before automatic coins/schema migration, initialization, prune resume or index mutation; prior durable data remains intact.

### Network and Historical Validation

- [ ] **CVLD-01**: A contributor observes the pinned subsidy schedule for each supported validation mode, including default-regtest halvings at heights 150 and 300 and ordinary coinbase maturity.
- [ ] **CVLD-02**: A contributor observes contextual block and transaction decisions derived from the selected network and validated deployment history, including BIP34 height, BIP65/BIP66 block-version constraints, BIP113/sequence-lock context and Segwit commitment rules, without caller-selected blanket activation switches.
- [ ] **CVLD-03**: A contributor observes pinned `GetBlockScriptFlags` behavior, including its baseline flags, block-hash exceptions and deployment-dependent additions, through the actual durable connect path.
- [ ] **CVLD-04**: A contributor observes pinned network difficulty and contextual time decisions at supported retarget, minimum-difficulty and median-time boundaries; header admission and body connection use the same validated network authority.
- [ ] **CVLD-05**: An operator can identify the exact supported validation/evidence scope of existing mainnet, testnet3, signet and regtest selectors; any mode lacking required consensus checks, including Signet challenge validation, refuses before accepting unsupported blocks rather than using another network's defaults.
- [ ] **CVLD-06**: A contributor observes pinned duplicate-coinbase/BIP30 and coin-overwrite behavior in the supported historical contexts, with the required ancestor/hash facts and correct disconnect restoration rather than a global permissive exception.

### Exact Cumulative Work

- [ ] **CWOR-01**: A contributor obtains exact target-derived 256-bit block proof and cumulative work with pinned compact-target validity and cumulative-arithmetic semantics, including values beyond 128 bits; any additional overflow refusal is documented as an intentional difference rather than silently changing the reference arithmetic.
- [ ] **CWOR-02**: An operator's live header best-chain selection follows verified cumulative proof of work rather than header count; defined equal-work arrival/activation behavior matches the pinned reference, while genuinely ambiguous internal ties remain explicitly bounded.
- [ ] **CWOR-03**: An operator's connected-chain and reorg selection consume the same internally verified work authority as accepted headers; caller-supplied work cannot authorize a stronger active branch.
- [ ] **CWOR-04**: An operator can checkpoint and reopen exact cumulative work without truncation, and recovered work is bound to stored headers, network identity and the recovered coins best block.
- [ ] **CWOR-05**: An operator opening legacy synthetic-work state receives an explicit compatibility decision before automatic coins/schema migration or live/durable authority changes; unprovable state refuses without silent reinterpretation, destructive conversion or automatic repair.

### Genuine Knots Differential Harness

- [ ] **CDIF-01**: A contributor can reproduce a differential run against an actual pinned Knots daemon whose executable SHA-256 is bound to the exact source tree and recorded successful build inputs/options, and Open Bitcoin's ordinary managed runtime validation path, using isolated local datadirs, identical serialized inputs, disabled external networking and disabled assumevalid shortcuts.
- [ ] **CDIF-02**: A contributor can compare both implementations from canonical regtest genesis through normal maturity and at least the 150- and 300-block halving boundaries, with no synthetic root, maturity override or caller-selected work.
- [ ] **CDIF-03**: A contributor can compare acceptance/rejection and unchanged state for invalid blocks and spends, with versioned normalization that preserves meaningful rejection differences.
- [ ] **CDIF-04**: A contributor can compare resulting best-block identity, UTXO facts, consensus-visible work and BASIC commitments where supported, while separating intended storage/operator differences from unexplained mismatches.
- [ ] **CDIF-05**: A contributor obtains bounded, reproducible case-level results tied to source identities, corpus digests and actual process outcomes; missing/wrong oracle, timeout, incomplete execution or mismatches cannot be recorded as passing parity.

### Forks and Durable Recovery

- [ ] **CREC-01**: A contributor can compare competing valid branches and equal-work arrivals against Knots, observing the same branch choice for defined activation order and separately classifying genuinely reference-ambiguous restart ties.
- [ ] **CREC-02**: A contributor can compare disconnect/reconnect and replacement spends, observing branch-correct UTXOs and no active-state mutation from rejected candidates.
- [ ] **CREC-03**: An operator can stop, release all storage handles and reopen the canonical chain, preserving earned active-tip, coins and exact-work authority through the ordinary durable path.
- [ ] **CREC-04**: An operator encountering a supported interrupted checkpoint or inconsistent branch/work record recovers to a proved durable prefix or receives an explicit refusal, without inventing progress or silently repairing history.

### Canonical Prune and BASIC Integration

- [ ] **CINT-01**: A contributor can reproduce actual paired block/undo deletion on a continuous canonical regtest chain above the pinned pruning eligibility threshold while retaining the required keep window and index protection.
- [ ] **CINT-02**: A client obtains branch-correct retained BASIC RPC and inbound/outbound peer responses after genuine pruning and all-handle reopen; deleted bodies remain unavailable and filters do not restore wallet eligibility.
- [ ] **CINT-03**: An operator observes coherent canonical tip, exact work, index progress and retention/refusal facts through existing RPC, repo-local CLI, dashboard and redacted support consumers.
- [ ] **CINT-04**: A contributor can audit finite mainnet historical validation fixtures with independent pinned expectations and exact ancestry/UTXO/activation prerequisites, without presenting contextual or sampled evidence as whole-chain mainnet validation.

### Parity Closure

- [ ] **CPAR-01**: A contributor can trace each v2.6 parity claim and intentional behavior difference to pinned source anchors, actual differential results and lifecycle-valid requirement owners; prior v2.5 receipts and limitations remain attributable to their original source.
- [ ] **CPAR-02**: A contributor can run the repo-native offline verification contract with deterministic v2.6 regressions and reproduce separately identified actual Knots qualification; changes to canonical production rules, runner/normalization, corpus or oracle build inputs invalidate affected receipts and require renewed actual qualification before closure, while claim checks reject unsupported whole-chain, public-default or production-readiness conclusions.

## Future Requirements

These remain deferred and have no v2.6 owner. Existing IDs are preserved from the archived v2.5 inventory.

- **FUT-19**: Archive-node or production-scale historical serving.
- **FUT-21**: Assumeutxo, assumevalid, IBD snapshot shortcuts or a second chainstate.
- **FUT-22**: Live Knots/Core LevelDB `chainstate/` import or export.
- **FUT-23**: Automatic destructive reindex or coins repair.
- **FUT-24**: Public serving or relay enabled by default.
- **FUT-25**: Public-network runs become default CI or release-blocking gates.
- **FUT-26**: Production full-node readiness, production service operation or production-funds wallet safety claims.
- **FUT-27**: Temporary initial-download prune target (`-pruneduringinit`).
- **FUT-28**: Knots V0 filter generation/indexing and its additional local RPC scope.
- **FUT-29**: Historical filter import or explicit missing-history recovery under a separate provenance and mutation policy.
- **FUT-30**: `scanblocks`, filter-assisted wallet acceleration or a new lightweight-client wallet.

Full address relay/address-manager persistence and wallet PSBT/multisig/multiwallet expansion remain separate candidate milestones. Testnet4, custom Signet and deployment overrides require separately scoped compatibility work; their absence cannot justify accepting blocks without required checks in an existing mode.

## Out of Scope

| Feature                                                                                            | Reason                                                                                             |
| -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Assumeutxo, dual chainstate or snapshot-based validation shortcuts                                 | Establish ordinary canonical authority before adding a second validation lifecycle.                |
| Exhaustive consensus parity or a new full public-mainnet replay claim                              | Finite regtest and historical-context evidence must retain its exact limits.                       |
| Automatic rewriting of legacy or foreign datadirs                                                  | Compatibility failures require explicit diagnostics and a separate migration policy.               |
| Testnet4, custom Signet or arbitrary deployment override product support                           | Existing exposed network identities and truthful validation support are the selected boundary.     |
| Archive-scale storage or physical prune-capacity guarantees                                        | Existing logical retention and honest availability remain distinct from physical allocation.       |
| New wallet product features, V0/BIP37, `scanblocks` or filter import                               | These are separate capabilities beyond canonical validation and existing BASIC integration.        |
| Public-network default verification, public activation defaults or production/service/funds claims | Existing capability, support and evidence gates remain independent.                                |
| Unrelated inherited v2.4 advisory cleanup                                                          | Change an advisory only when required to satisfy a selected canonical authority/recovery contract. |

## Traceability

The approved roadmap maps every active requirement to exactly one phase beginning at 164. All 30 active requirements remain Pending; no phase execution or verification is claimed.

| Requirement | Phase     | Status  |
| ----------- | --------- | ------- |
| CBOT-01     | Phase 164 | Pending |
| CBOT-02     | Phase 166 | Pending |
| CBOT-03     | Phase 166 | Pending |
| CBOT-04     | Phase 164 | Pending |
| CVLD-01     | Phase 167 | Pending |
| CVLD-02     | Phase 167 | Pending |
| CVLD-03     | Phase 167 | Pending |
| CVLD-04     | Phase 167 | Pending |
| CVLD-05     | Phase 164 | Pending |
| CVLD-06     | Phase 167 | Pending |
| CWOR-01     | Phase 165 | Pending |
| CWOR-02     | Phase 165 | Pending |
| CWOR-03     | Phase 165 | Pending |
| CWOR-04     | Phase 166 | Pending |
| CWOR-05     | Phase 165 | Pending |
| CDIF-01     | Phase 168 | Pending |
| CDIF-02     | Phase 168 | Pending |
| CDIF-03     | Phase 168 | Pending |
| CDIF-04     | Phase 168 | Pending |
| CDIF-05     | Phase 168 | Pending |
| CREC-01     | Phase 169 | Pending |
| CREC-02     | Phase 169 | Pending |
| CREC-03     | Phase 169 | Pending |
| CREC-04     | Phase 169 | Pending |
| CINT-01     | Phase 170 | Pending |
| CINT-02     | Phase 170 | Pending |
| CINT-03     | Phase 170 | Pending |
| CINT-04     | Phase 171 | Pending |
| CPAR-01     | Phase 171 | Pending |
| CPAR-02     | Phase 171 | Pending |

*Requirements defined: 2026-10-13. Last updated: 2026-10-13 after user approval of v2.6 requirements and phase traceability.*
