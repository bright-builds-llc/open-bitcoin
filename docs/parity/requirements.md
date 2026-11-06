# Approved v2.6 Parity Requirements

These are the 30 user-approved requirement descriptions from 2026-10-13. Current ownership and status live in `docs/parity/index.json`; this file defines wording only.


### Canonical Bootstrap

- **CBOT-01**: An operator's selected existing network binds the accepted root to pinned canonical genesis header/body identity; a different null-parent root is rejected.
- **CBOT-02**: An operator can initialize and reopen an empty durable datadir with canonical genesis as best block, while the genesis coinbase contributes no spendable UTXOs and interrupted initialization never publishes partial authority.
- **CBOT-03**: An operator can enable BASIC indexing on a fresh canonical datadir, obtain its genesis filter and zero-predecessor commitment, and continue ordinary indexing without seeded history or implicit public-network activation.
- **CBOT-04**: An operator opening a foreign-network or noncanonical-root datadir receives a bounded diagnostic before automatic coins/schema migration, initialization, prune resume or index mutation; prior durable data remains intact.

### Network and Historical Validation

- **CVLD-01**: A contributor observes the pinned subsidy schedule for each supported validation mode, including default-regtest halvings at heights 150 and 300 and ordinary coinbase maturity.
- **CVLD-02**: A contributor observes contextual block and transaction decisions derived from the selected network and validated deployment history, including BIP34 height, BIP65/BIP66 block-version constraints, BIP113/sequence-lock context and Segwit commitment rules, without caller-selected blanket activation switches.
- **CVLD-03**: A contributor observes pinned `GetBlockScriptFlags` behavior, including its baseline flags, block-hash exceptions and deployment-dependent additions, through the actual durable connect path.
- **CVLD-04**: A contributor observes pinned network difficulty and contextual time decisions at supported retarget, minimum-difficulty and median-time boundaries; header admission and body connection use the same validated network authority.
- **CVLD-05**: An operator can identify the exact supported validation/evidence scope of existing mainnet, testnet3, signet and regtest selectors; any mode lacking required consensus checks, including Signet challenge validation, refuses before accepting unsupported blocks rather than using another network's defaults.
- **CVLD-06**: A contributor observes pinned duplicate-coinbase/BIP30 and coin-overwrite behavior in the supported historical contexts, with the required ancestor/hash facts and correct disconnect restoration rather than a global permissive exception.

### Exact Cumulative Work

- **CWOR-01**: A contributor obtains exact target-derived 256-bit block proof and cumulative work with pinned compact-target validity and cumulative-arithmetic semantics, including values beyond 128 bits; any additional overflow refusal is documented as an intentional difference rather than silently changing the reference arithmetic.
- **CWOR-02**: An operator's live header best-chain selection follows verified cumulative proof of work rather than header count; defined equal-work arrival/activation behavior matches the pinned reference, while genuinely ambiguous internal ties remain explicitly bounded.
- **CWOR-03**: An operator's connected-chain and reorg selection consume the same internally verified work authority as accepted headers; caller-supplied work cannot authorize a stronger active branch.
- **CWOR-04**: An operator can checkpoint and reopen exact cumulative work without truncation, and recovered work is bound to stored headers, network identity and the recovered coins best block.
- **CWOR-05**: An operator opening legacy synthetic-work state receives an explicit compatibility decision before automatic coins/schema migration or live/durable authority changes; unprovable state refuses without silent reinterpretation, destructive conversion or automatic repair.

### Genuine Knots Differential Harness

- **CDIF-01**: A contributor can reproduce a differential run against an actual pinned Knots daemon whose executable SHA-256 is bound to the exact source tree and recorded successful build inputs/options, and Open Bitcoin's ordinary managed runtime validation path, using isolated local datadirs, identical serialized inputs, disabled external networking and disabled assumevalid shortcuts.
- **CDIF-02**: A contributor can compare both implementations from canonical regtest genesis through normal maturity and at least the 150- and 300-block halving boundaries, with no synthetic root, maturity override or caller-selected work.
- **CDIF-03**: A contributor can compare acceptance/rejection and unchanged state for invalid blocks and spends, with versioned normalization that preserves meaningful rejection differences.
- **CDIF-04**: A contributor can compare resulting best-block identity, UTXO facts, consensus-visible work and BASIC commitments where supported, while separating intended storage/operator differences from unexplained mismatches.
- **CDIF-05**: A contributor obtains bounded, reproducible case-level results tied to source identities, corpus digests and actual process outcomes; missing/wrong oracle, timeout, incomplete execution or mismatches cannot be recorded as passing parity.

### Forks and Durable Recovery

- **CREC-01**: A contributor can compare competing valid branches and equal-work arrivals against Knots, observing the same branch choice for defined activation order and separately classifying genuinely reference-ambiguous restart ties.
- **CREC-02**: A contributor can compare disconnect/reconnect and replacement spends, observing branch-correct UTXOs and no active-state mutation from rejected candidates.
- **CREC-03**: An operator can stop, release all storage handles and reopen the canonical chain, preserving earned active-tip, coins and exact-work authority through the ordinary durable path.
- **CREC-04**: An operator encountering a supported interrupted checkpoint or inconsistent branch/work record recovers to a proved durable prefix or receives an explicit refusal, without inventing progress or silently repairing history.

### Canonical Prune and BASIC Integration

- **CINT-01**: A contributor can reproduce actual paired block/undo deletion on a continuous canonical regtest chain above the pinned pruning eligibility threshold while retaining the required keep window and index protection.
- **CINT-02**: A client obtains branch-correct retained BASIC RPC and inbound/outbound peer responses after genuine pruning and all-handle reopen; deleted bodies remain unavailable and filters do not restore wallet eligibility.
- **CINT-03**: An operator observes coherent canonical tip, exact work, index progress and retention/refusal facts through existing RPC, repo-local CLI, dashboard and redacted support consumers.
- **CINT-04**: A contributor can audit finite mainnet historical validation fixtures with independent pinned expectations and exact ancestry/UTXO/activation prerequisites, without presenting contextual or sampled evidence as whole-chain mainnet validation.

### Parity Closure

- **CPAR-01**: A contributor can trace each v2.6 parity claim and intentional behavior difference to pinned source anchors, actual differential results and lifecycle-valid requirement owners; prior v2.5 receipts and limitations remain attributable to their original source.
- **CPAR-02**: A contributor can run the repo-native offline verification contract with deterministic v2.6 regressions and reproduce separately identified actual Knots qualification; changes to canonical production rules, runner/normalization, corpus or oracle build inputs invalidate affected receipts and require renewed actual qualification before closure, while claim checks reject unsupported whole-chain, public-default or production-readiness conclusions.
