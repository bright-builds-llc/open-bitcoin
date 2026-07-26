---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 133-2026-07-26T16-12-51
generated_at: 2026-07-26T16:17:11.873Z
---

# Phase 133: Package-Aware Download and Orphan Bridge - Context

**Gathered:** 2026-07-26
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Deliver the bounded peer-download and orphan bridge that distinguishes hard rejection from reconsiderable package evidence, assembles only the pinned sender-aware same-peer one-parent/one-child candidate from ordinary transaction messages, and routes that candidate through the authoritative Phase 132 package-admission engine. This phase does not add a package wire protocol, arbitrary multi-parent reconstruction, ordinary relay fanout, transport receipts, or the Phase 134 full lifecycle projection across serving, compact, persistence, retry, and operator state.

</domain>

<decisions>
## Implementation Decisions

### Bounded reconsiderable and reject evidence

- **D-01:** Replace unbounded exact recent-reject membership with two node-global, fixed-memory rolling filters matching the pinned Knots split: one hard-reject filter keyed by wtxid and one reconsiderable filter keyed by either wtxid or the existing permutation-independent `PackageFingerprint`.
- **D-02:** Configure each rolling filter for 120,000 insertions and a 0.000001 target false-positive rate, rotate by insertion generation, and reset both filters on every active-chain-tip change. Do not add wall-clock TTLs or per-peer reject maps to this evidence.
- **D-03:** Keep candidate bodies and sender provenance out of the reject filters. Bodies remain in the separately bounded orphanage, retaining explicit global, per-peer, TTL, and per-parent reconsideration limits.
- **D-04:** Hard transaction failures enter only the hard-reject filter. Reconsiderable member failures and failed package fingerprints enter only reconsiderable evidence, so a fee-reconsiderable candidate is not suppressed as an ordinary hard reject.

### Same-peer 1P1C assembly and identity

- **D-05:** Use Knots-style announcer-qualified, parent-triggered assembly. When peer `P` supplies a reconsiderable parent, consider only orphan children that spend that parent and whose retained announcer provenance includes `P`; do not require `P` to have been the first peer that delivered the child body.
- **D-06:** Select the newest eligible same-peer child first under a deterministic bounded traversal. A candidate is exactly the topologically ordered pair `[parent, child]`; never aggregate siblings, multiple parents, grandchildren, or an arbitrary package graph.
- **D-07:** Preserve sender provenance request-index aligned with the two members and attribute both candidate members to the qualifying peer `P`, while keeping provenance outside the content-derived package fingerprint.
- **D-08:** Construct and validate the ordered pair once through the Phase 132 `WellFormedPackage` and child-with-unconfirmed-parents refinement. Preserve its cached `PackageFingerprint` unchanged through admission, failed-package suppression, and evidence.
- **D-09:** Support both transaction arrival orders through ordinary inventory/request/transaction behavior and retained orphan announcers. Do not introduce a retained reconsiderable-parent body cache or a package-specific wire message.

### Authoritative admission bridge and outcome feedback

- **D-10:** Keep `open-bitcoin-network` responsible for a neutral typed candidate containing the ordered transactions, aligned peer provenance, and the evidence needed to prove same-peer eligibility. Do not make the network crate construct `open-bitcoin-mempool::SubmissionPackage` or add a network-to-mempool dependency.
- **D-11:** Make the `open-bitcoin-node` admission bridge refine the candidate into the Phase 132 submission command, call the authoritative package engine exactly once, and preserve its exact `PackageReport`, `PackageFingerprint`, and `MempoolLifecycleDelta`. Do not reimplement package policy in networking or RPC code.
- **D-12:** In Phase 133, feed back only state required to keep later candidate selection correct: retire candidates for finally present, already present, witness-alias, hard-rejected, or post-trim-absent members; retain or restage only true missing-input reconsiderations; and record other reconsiderable member identities plus failed fingerprints in the bounded reconsiderable filter.
- **D-13:** Keep full lifecycle projection out of this phase. Applying package deltas to relay serving, fanout, peer known/request state, compact reconstruction, unbroadcast state, persistence dirtiness, and operator evidence remains Phase 134 responsibility.

### the agent's Discretion

- Exact Rust names and module split, provided the filter-key types, candidate eligibility proof, aligned origin data, and node-owned package refinement remain explicit.
- The standard-library implementation details of the rolling filter, including internal hash derivation and generation storage, provided the configured capacity, false-positive target, reset behavior, deterministic tests, and fixed-memory property are preserved.
- The bounded announcer-set representation and newest-first index shape, provided disconnect cleanup removes only the departing announcer, orphan bodies are not duplicated per peer, and selection remains deterministic.
- Exact feedback enum granularity, provided hard rejects, reconsiderable transactions, failed packages, restaged missing-input children, and successful/terminal candidate retirement cannot be confused.

</decisions>

<canonical-refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Active milestone contract and predecessor decisions

- `.planning/ROADMAP.md` — Phase 133 goal, PPKG-01 through PPKG-03 success criteria, and the boundary against Phases 134 and 136.
- `.planning/REQUIREMENTS.md` — Opportunistic peer-package requirements and explicit deferral of general package wire relay and arbitrary multi-parent reconstruction.
- `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md` — Locked package shape, identity, reconsiderable outcome, staged-admission, final-membership, and lifecycle-delta contracts consumed here.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` — Existing bounded transaction-download and orphanage decisions that this phase extends.
- `.planning/research/ARCHITECTURE.md` — Pure network candidate, node admission bridge, same-peer 1P1C flow, and cross-cache phase ordering.
- `.planning/research/FEATURES.md` — Pinned opportunistic package-relay feature inventory and ordinary-message boundary.
- `.planning/research/PITFALLS.md` — Unbounded reject state, lost sender attribution, policy duplication, and lifecycle-divergence hazards.
- `.planning/research/SUMMARY.md` — Synthesized v2.2 scope, exclusions, and narrow relay claims.

### Pinned Bitcoin Knots behavior

- `packages/bitcoin-knots/src/node/txdownloadman.h` — `PackageToValidate`, sender alignment, transaction-download ownership, and package-candidate interface.
- `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp` — Hard and reconsiderable rolling filters, active-tip reset, received/rejected transaction handling, same-peer candidate selection, and failed-package suppression.
- `packages/bitcoin-knots/src/txorphanage.h` and `packages/bitcoin-knots/src/txorphanage.cpp` — Bounded orphan bodies, announcer provenance, newest-first same-peer child selection, expiry, eviction, and disconnect cleanup.
- `packages/bitcoin-knots/src/net_processing.cpp` — Ordinary `tx` processing, `ProcessInvalidTx`, `ProcessPackageResult`, and the bridge to `ProcessNewPackage`.
- `packages/bitcoin-knots/src/validation.h` and `packages/bitcoin-knots/src/validation.cpp` — Reconsiderable validation categories and authoritative package result semantics.
- `packages/bitcoin-knots/src/common/bloom.h` and `packages/bitcoin-knots/src/common/bloom.cpp` — Fixed-memory generational rolling-filter behavior and parameter semantics.
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py` — Orphan ownership, same-peer handling, eviction, and reconsideration anchors.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py` — Transaction request, reject suppression, announcer, and bounded-download behavior.

### Open Bitcoin integration seams

- `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs` — Existing bounded orphan records, reconsideration candidates, TTL/cap enforcement, and peer cleanup.
- `packages/open-bitcoin-network/src/peer/transaction_relay.rs` — Transaction-relay action and download/orphan coordination.
- `packages/open-bitcoin-network/src/peer.rs` and `packages/open-bitcoin-network/src/peer/inventory_state.rs` — Current exact recent-reject state, peer announcer/request scheduling, known membership, and active peer boundaries.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs` — Current singleton peer admission and orphan reconsideration bridge to refactor for typed package candidates.
- `packages/open-bitcoin-node/src/network.rs` — Authoritative `ManagedPeerNetwork` composition of mempool, peer manager, orphanage, serving, compact, and relay state.
- `packages/open-bitcoin-mempool/src/package.rs` and `packages/open-bitcoin-mempool/src/package/report.rs` — Existing `WellFormedPackage`, `SubmissionPackage`, `PackageFingerprint`, ordered reports, and reconsiderable failures.
- `packages/open-bitcoin-mempool/src/pool/package_admission.rs` — Authoritative Phase 132 package-admission engine.
- `docs/parity/catalog/mempool-policy.md` — Existing transaction-relay and package-policy parity claims and exclusions.
- `docs/parity/source-breadcrumbs.json` — Required source breadcrumb registry for new first-party Rust source and test files.

</canonical-refs>

<code-context>
## Existing Code Insights

### Reusable Assets

- `TxOrphanage` already owns transaction bodies with total, per-peer, TTL, and per-parent reconsideration limits; extend its provenance and selection indexes instead of creating a parallel package body cache.
- `TxDownloadScheduler` and `PeerManager` already coordinate announcers, requests, in-flight caps, known transactions, and recent-reject suppression over ordinary messages.
- `PackageFingerprint`, `WellFormedPackage`, `SubmissionPackage`, ordered package reports, and typed reconsiderable failures already provide the authoritative Phase 132 identity and admission vocabulary.
- `ManagedPeerNetwork::process_peer_transaction_admission` and `reconsider_orphans_after_acceptance` already define the node-owned seam between pure network state and authoritative mempool mutation.

### Established Patterns

- Keep peer-selection and bounded caching decisions in the pure network crate, package policy in the pure mempool crate, and their composition in the node shell.
- Use strong types for distinct evidence classes and candidate eligibility; do not pass caller booleans that could forge same-peer or reconsiderable status.
- Keep attempted admission results separate from committed lifecycle facts and from later achieved transport evidence.
- Preserve deterministic, hermetic, parity-anchored verification; no public-network test is required for this phase.

### Integration Points

- Add the rolling-filter abstraction and typed hard/reconsiderable keys under transaction-relay state without adding dependencies.
- Extend orphan records and transaction-download announcer tracking so same-peer eligibility is proven from bounded retained state and survives either transaction arrival order.
- Emit a neutral package candidate from `open-bitcoin-network`, refine and submit it in `open-bitcoin-node`, and map authoritative member/package outcomes back to candidate evidence.
- Leave the admitted package lifecycle delta available for Phase 134 rather than partially projecting it across unrelated caches here.

</code-context>

<specifics>
## Specific Ideas

- “Same peer” follows the pinned announcer-qualified meaning, not the stricter rule that one peer must have delivered both transaction bodies.
- Hard-reject and reconsiderable filters intentionally accept the pinned probabilistic false-positive tradeoff to guarantee fixed memory under adversarial churn.
- Candidate identity is the Phase 132 content-derived fingerprint; peer provenance is aligned metadata and must never alter the fingerprint.
- Package-aware peer admission stays opportunistic and ordinary-message-only. It is not BIP331 or general package relay.

</specifics>

<deferred>
## Deferred Ideas

- Applying every package admission/removal delta to serving, fanout, peer known/request state, compact reconstruction, unbroadcast state, persistence dirtiness, and operator evidence — Phase 134.
- Parent-before-child ordinary transaction fanout and transport receipts — Phase 136.
- RPC, CLI, dashboard, metrics, logs, and support-bundle package evidence — Phase 137.
- General package wire messages, BIP331, arbitrary multi-parent peer reconstruction, and cluster mempool — beyond v2.2.

</deferred>

***

*Phase: 133-package-aware-download-and-orphan-bridge*
*Context gathered: 2026-07-26*
