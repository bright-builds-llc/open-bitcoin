# Packages

Phase 159 implements authenticated BASIC `getblockfilter`/`getindexinfo` through
the existing node authority and RPC HTTP shell. See the
[scoped contract and exact Cargo/Bazel commands](../docs/parity/catalog/basic-compact-filters.md#authenticated-basic-queries-phase-159).
Genuine acceptance, paired deletion and all-handle configured reopen supplement
private node faults composed with actual RPC projection. UnknownLegacy missing
rows fail closed; valid retained records still serve. Initial-sync latch,
processed height and captured-frontier readiness remain distinct. Dependencies
and default-off activation are unchanged. CFRP-01/02 are Complete after full native, independent source/security and
[formal lifecycle verification](../.planning/phases/159-authenticated-basic-filter-and-index-rpcs/159-VERIFICATION.md). Phase 160 implements explicit bounded peers;
Phase 161 adds shared operator evidence below. Current scoped integrated
evidence is linked below; historical Phase 162 source/security, full native and formal evidence gates passed. The dated v2.5 audit found the archive handoff gap now owned by Phase 163. Earlier phase scope below is
historical; synthetic genesis/maturity-one/manual owner-plan evidence does not
establish ordinary prune-RPC, automatic-threshold or production/funds safety.

Phase 161 adds one shared `basic_index` projection from the existing authority
through `openbitcoinnetworkstatus`, status, dashboard and redacted support.
Configured capability, earned initial sync, current lag, processed/safe durable
heights, fixed failures, retained availability and protection remain independent.
Stopped/legacy missing facts are unavailable. Unique immutable records and logical
payload/key-value bytes include stale branches; payload bytes are a subset, and
Fjall overhead/physical allocation are excluded. Growth stays outside the soft
block/undo target and does not restore body/wallet eligibility. See the
[operator contract and Cargo/Bazel commands](../docs/parity/catalog/basic-compact-filters.md#shared-basic-operator-evidence-phase-161).
v2.5 archive readiness is established by the renewed passing milestone audit; v2.5 remains unarchived and unshipped.

See the [archive-safe provenance policy and pinned reproduction commands](../docs/parity/v2-5-archive-safe-provenance.md) for fixed milestone controls, equivalent overlap/conflict refusal, hermetic maps and the required current claim transition after a future actual archive. Phase 163's corrected21-input source `e0c16fb2` passed the complete default native verifier, independent source/security acceptance and canonical functional-closure verification. Earlier digest `34443772` receipts remain dated historical evidence. CFGR-01 closure and the renewed22/22audit are earned; readiness is published through the current handoff policy. Readiness-state tests and finalSC1–SC5whole-phase verification passed; allfive phaseplans are complete. Normal-hook source saving remains the original wrapper gate. Actual archival/shipment remains separate.

Historical publication at 2026-10-12T03:42:55Z, before the later gaps audit (retained as dated delivery evidence):

Phase 162 scoped integrated proof is complete in docs/parity/v2-5-integrated-filter-proof.md; CFPR-02/CFGR-01/CFGR-02 are Complete; v2.5 remains unaudited and unshipped.

The [integrated proof](../docs/parity/v2-5-integrated-filter-proof.md) and
[timed Cargo/Bazel UAT](../.planning/phases/162-real-prune-retention-and-integrated-parity-proof/162-UAT.md)
link actual normal-maturity startup/scheduler/connect/reorg, manual/automatic 550 MiB
paired deletion, cold reopen, authenticated RPC and both peer adapters. Exact
bounded test-wallet import/funding/scan/refusal and h715 Block/h714 NotFound details
remain scoped there; only future descriptor formatting changes, with legacy
decoder/raw-text compatibility preserved. Final06 resource observations retain
soft-target excess and physical point-measurement limits. Scoped
source/security, full native and formal evidence gates passed, without a
default-wallet performance or whole-chain Knots differential claim.
No stronger v2.4 crash-window guarantee is claimed.

This directory holds both the pinned upstream reference baseline and first-party
Open Bitcoin crates.

- `bitcoin-knots/` is the vendored upstream behavioral baseline. Treat it as read-only from the perspective of first-party implementation work.
- `open-bitcoin-primitives/` owns shared low-level value types used across the workspace.
- `open-bitcoin-codec/` owns Bitcoin byte encoding, decoding, and wire-framing helpers.
- `open-bitcoin-core/` re-exports the first-party pure-core surface for downstream package boundaries.
- `open-bitcoin-consensus/` owns script execution, transaction checks, block checks, proof-of-work, merkle behavior, and typed validation errors.
- `open-bitcoin-chainstate/` owns pure-core UTXO state, undo data, active-chain mutation, and reorg behavior.
- `open-bitcoin-mempool/` owns policy admission, replacement, ancestor/descendant accounting, eviction behavior, typed resource, fee, metadata, and lifecycle contracts, and bounded local pure-core package admission with ordered dry-run and staged-submit results.
- `open-bitcoin-network/` owns peer lifecycle, wire-message handling, sync planning, relay state, injected retry-input contracts, bounded reject evidence, and neutral same-peer one-parent/one-child candidate proofs.
- `open-bitcoin-wallet/` owns descriptor parsing, address derivation, balance tracking, coin selection, transaction building, and signing.
- `open-bitcoin-node/` owns adapter-facing orchestration over chainstate, mempool, networking, and wallet state, including the sole `ManagedNetworkHandle` lifecycle authority, complete cross-cache projection, bounded reconciliation, typed peer/snapshot effects, and the single package-admission call for an eligible peer candidate.
- `open-bitcoin-rpc/` owns JSON-RPC envelopes, config loading, method dispatch, HTTP serving, the `open-bitcoind` binary, and truthful resource and fee RPC projection.
- `open-bitcoin-cli/` owns the `open-bitcoin-cli` command-line client and supported `bitcoin-cli`-style startup behavior.
- `open-bitcoin-test-harness/` owns reusable black-box parity cases, target adapters, isolation helpers, and parity report generation.
- `open-bitcoin-bench/` owns deterministic benchmark groups and JSON/Markdown report generation.

First-party crates should depend on each other intentionally. Pure-core crates
must not depend on shell/runtime crates. Adapter and executable crates may
depend on pure-core crates, but I/O and runtime effects should stay outside the
pure-core packages.

The mempool package surface remains local and effect-free. Phase 133 adds a
bounded network-to-node bridge: ordinary `inv`/`getdata`/`tx` flow may yield a
newest-first same-peer one-parent/one-child candidate, and the node owns one
authoritative Phase 132 package-admission call plus typed feedback. Phase 134
projects committed lifecycle facts through the sole `ManagedNetworkHandle`
authority into serving, ordinary fanout state, peer state, compact inputs,
unbroadcast membership, persistence dirtiness, and bounded evidence. Peer and
current-schema snapshot effects use unique authority-incarnation bindings,
peer-local freshness, exact complete-or-abort terminal paths, and I/O outside
the authority lock. Canonical state commits before seven infallible dependent
applies; exact txid/wtxid teardown and symmetric audits expose stale dependent
state without making reconciliation a second mutation authority. Phase 135
snapshot schema/recovery, Phase 136 receive-independent scheduling and package
fanout, Phase 137 operator surfaces, Phase 138 release proof, general package
wire relay, whole-mempool rebroadcast, arbitrary multi-parent peer assembly,
public or default relay, guaranteed propagation, public-network CI or release
gates, and production readiness remain deferred.

The following Phase 158 description preserves its original implementation-stage handoff; Phases 159–162 subsequently completed their scoped historical delivery.

Phase 158 implements internal validated BASIC branch replacement in the pure
chainstate and serialized node adapters. Genuine accepted receipts and live
accepted-position capabilities gate every nonempty turn; recovered coins plus
compatible metadata fence safe progress and reserved prune protection. Immutable
hash-keyed displaced records survive actual configured reopen. Missing required
body/undo and finite resource exhaustion refuse before preview/effects; existing
caps can refuse fully retained forks.

See the [scoped parity page](../docs/parity/v2-5-validated-reorg.md),
[current measurements](../.planning/phases/158-validated-reorg-and-retained-branch-identity/158-REORG-MEASUREMENTS.md)
and [Cargo/Bazel UAT](../.planning/phases/158-validated-reorg-and-retained-branch-identity/158-UAT.md).
CFIX-03 is Complete after full native, independent source/security and formal
lifecycle verification passed; all 21 truths and 25 mitigations are closed. Filter/index RPC 159, peers 160, operator
projections 161 and integrated retained-client proof 162 remain pending and are
not shipped. Compact maturity-one fixtures and the separate unchanged
maturity-100 case do not establish full regtest parameter parity, archive-scale
performance, hardware crash guarantees or production/funds safety.
