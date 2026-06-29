# Pitfalls Research

**Domain:** Open Bitcoin v2.0 transaction relay and mempool participation boundary
**Researched:** 2026-06-29
**Confidence:** HIGH

## Scope Note

This research is for a subsequent milestone that adds bounded transaction relay and mempool participation to existing Open Bitcoin. The roadmap for v2.0 is still being defined, so phase names below are recommended Phase 100+ prevention gates for roadmap creation rather than claims that those phases already exist.

The central failure mode is over-activation: making a parsing, policy, or local mempool feature look like public relay readiness. v2.0 should keep the current project boundaries intact: pinned Bitcoin Knots `29.3.knots20260210` behavior for in-scope surfaces, first-party Rust implementation, functional-core/imperative-shell separation, deterministic default verification, no public-network CI default, redacted support surfaces, opt-in inbound serving, and no production-readiness overclaim.

## Critical Pitfalls

### Pitfall 1: Activating Relay-Like Permissions Too Broadly

**What goes wrong:**
The existing `relay`, `forcerelay`, and `mempool` permission tokens are parsed and surfaced, but v1.9 intentionally kept their runtime effects inactive. A v2.0 implementation can accidentally treat those labels as full Bitcoin Knots permission semantics, enabling transaction relay, mempool dumps, eviction immunity, or public serving paths that were never explicitly scoped.

**Why it happens:**
The permission parser already recognizes Knots-like names, and the implementation has an `InactivePermissionEffectLabel` type. It is tempting to replace "inactive" with broad conditionals and assume that matching token names means matching all Knots effects.

**How to avoid:**
Create an explicit relay activation matrix before implementation. The matrix must separate ordinary relay behavior, permissioned relay behavior, protected peer admission, BIP35-style `mempool` handling, `forcerelay`, bloom/filter permissions, and unrelated compact-block behavior. Add tests proving that enabling one permission does not imply the others. Keep public listener defaults, inbound serving defaults, and relay defaults independent.

**Warning signs:**
Code branches directly on parsed permission strings outside a narrow activation module; tests assert only that tokens parse; `relay` or `mempool` permissions silently bypass normal policy; support/status output reports permissions as active effects without evidence.

**Phase to address:**
Phase 100: Relay Activation Boundary and Permission Semantics.

### Pitfall 2: Mixing Txid and Wtxid Relay Identity

**What goes wrong:**
The node requests or announces the wrong inventory type, such as asking a wtxid-relay peer for `MSG_TX`, asking a non-wtxid peer for `MSG_WTX`, or clearing only one identity after receiving a transaction. This causes missed downloads, repeated requests, privacy leaks through unnecessary announcements, and parity drift from Knots tx download behavior.

**Why it happens:**
Open Bitcoin already tracks both txids and wtxids, but they are both represented as hash-like values. A simplified relay path can collapse them into one "transaction hash" concept and lose the protocol distinction.

**How to avoid:**
Keep a typed relay identity model equivalent to a `GenTxid`: every announcement, request, notfound, known-inventory update, and received transaction path must know whether it is keyed by txid or wtxid. Tests must cover mismatch ignoring, correct request type selection, `notfound` cleanup, received-transaction cleanup for both identities, and announcement choice based on the peer's negotiated `wtxidrelay` state.

**Warning signs:**
Maps keyed only by raw `Hash32`; helper names like `request_transaction(hash)` with no identity type; announcing `MSG_TX` to all peers; treating incoming `tx` messages as satisfying only txid requests; no tests for wtxidrelay mismatch cases.

**Phase to address:**
Phase 101: Transaction Download, Request Scheduling, and Orphan Handling.

### Pitfall 3: Treating `ReceivedTransaction` as Direct Mempool Admission

**What goes wrong:**
An inbound `tx` message is submitted directly to the mempool and any rejection bubbles out as a peer/network error. The node then lacks recent-reject tracking, orphan handling, request expiry, peer fallback, notfound fallback, candidate-peer selection, and clean peer disconnect cleanup.

**Why it happens:**
The current node path can accept a simple peer transaction into the mempool, store it, and serve it. That makes a narrow smoke path look like a complete relay manager.

**How to avoid:**
Introduce a first-party pure relay/download state machine before enabling real mempool participation. It should model announcement registration, per-peer in-flight request caps, request expiry, `notfound`, peer disconnect fallback, accepted/rejected callbacks from mempool policy, recent rejects, reconsiderable rejects, and bounded orphan state. The node shell should call this manager on peer events, mempool accept/reject results, and chain events.

**Warning signs:**
`PeerAction::ReceivedTransaction` calls mempool admission and returns the error directly; no state transition after mempool acceptance or rejection; no fake-clock tests for request timeout; no tests for `notfound` or disconnect fallback.

**Phase to address:**
Phase 101: Transaction Download, Request Scheduling, and Orphan Handling.

### Pitfall 4: Letting Missing-Input Handling Become Either Permanent Rejection or Unbounded Orphan Storage

**What goes wrong:**
Transactions with missing parents are permanently rejected, endlessly re-requested, or stored in an unbounded orphan pool. Valid child-before-parent relay fails, invalid orphan floods consume memory, and package-related behavior is overclaimed without package validation.

**Why it happens:**
The current mempool core correctly rejects missing inputs for direct single-transaction admission. Real relay needs a separate bounded orphan/download path, and Knots has substantial logic around missing parents, orphan candidates, reconsideration, and package validation.

**How to avoid:**
Scope v2.0 orphan behavior explicitly. Implement a bounded orphanage, parent-request generation, orphan reconsideration after parent acceptance, orphan cleanup on block connection and peer disconnect, and deterministic eviction when the orphan cap is reached. If package relay is not implemented, say so in docs and tests. Never bypass normal validation to admit a child before its parents are available.

**Warning signs:**
`MissingInput` is logged as a final policy rejection; an orphan map has no explicit max count or byte budget; a child transaction is inserted by bypassing consensus/policy validation; docs imply package relay because orphan handling exists.

**Phase to address:**
Phase 101 for orphan/download state and Phase 102 for mempool admission integration.

### Pitfall 5: Breaking Mempool, Relay Cache, and Chainstate Coherence

**What goes wrong:**
Confirmed, evicted, replaced, or conflicted transactions remain in the mempool-facing relay store. The node serves stale transactions through `getdata`, relays transactions already mined in a block, fails to reconsider transactions on reorg, or leaves descendant/ancestor state inconsistent.

**Why it happens:**
Open Bitcoin currently stores accepted transactions in maps separate from the mempool structure for simple serving. Without explicit chain-event callbacks, those maps can diverge from mempool truth.

**How to avoid:**
Phase 102 must own chain-event integration. On block connect, remove block transactions, conflicts, and dependent relay cache entries through normal mempool APIs. On disconnect or reorg, reconsider eligible disconnected transactions through normal admission rules, not direct insertion. Notify the relay/download manager about accepted, rejected, confirmed, disconnected, and removed transactions.

**Warning signs:**
`transactions_by_txid` or `transactions_by_wtxid` grows independently of mempool entries; `connect_stored_block` has no mempool side effects; support status shows unchanged mempool counts after connecting a block that includes mempool transactions; tests inspect maps instead of observable serving behavior.

**Phase to address:**
Phase 102: Mempool/Chainstate Admission, Pressure, and Repair.

### Pitfall 6: Advertising Mempool Pressure Behavior Without Rolling Minimum-Fee State

**What goes wrong:**
The node accepts or announces transactions below the effective mempool minimum fee after eviction pressure, reports misleading `mempoolminfee` values, or repeatedly churns low-fee transactions when the mempool is full.

**Why it happens:**
The current policy config includes static minimum relay and incremental relay fee settings, and the mempool can trim to a maximum size. The parity catalog already flags long-lived rolling minimum-fee behavior as a gap, while Knots updates and decays rolling fee state under pressure.

**How to avoid:**
Either implement rolling minimum-fee state with deterministic time/block inputs and surface it accurately, or explicitly bound v2.0 to static minrelay behavior and mark pressure parity deferred. Tests must prove that trimming raises the effective admission threshold, that later decay or reset behavior is deterministic, and that operator/RPC surfaces match the actual admission rule.

**Warning signs:**
`mempoolminfee` is always equal to `minrelaytxfee`; policy code has no time or block-height input for decay; docs claim Knots-compatible mempool pressure behavior without eviction-threshold tests.

**Phase to address:**
Phase 102: Mempool/Chainstate Admission, Pressure, and Repair.

### Pitfall 7: Serving Any Locally Stored Transaction Instead of Relay-Eligible Transactions

**What goes wrong:**
The node serves transactions from a local cache even after they are replaced, evicted, confirmed, conflicted, or rejected. Peers can request stale transactions outside the intended relay window, and `notfound` behavior no longer reflects mempool relay eligibility.

**Why it happens:**
The existing `serve_inventory` path can return from `transactions_by_txid` and `transactions_by_wtxid`. That is useful for current bounded behavior, but it is not enough for relay serving once transaction lifecycle transitions matter.

**How to avoid:**
Make relay serving derive from mempool entries plus a bounded relay cache with explicit sequence and last-announced semantics. Remove or mark entries on replacement, eviction, block connection, reorg repair, and rejection. Test `getdata` for unknown, stale, confirmed, replaced, and valid relay-eligible transactions.

**Warning signs:**
`ServeInventory` bypasses mempool lookup; no tests assert `notfound` after replacement or block connection; old txids remain in maps after RBF replacement; every stored local transaction is considered servable.

**Phase to address:**
Phase 103: Relay Serving, Announcement, and Rebroadcast Policy.

### Pitfall 8: Broadcasting Accepted Transactions Without Peer Eligibility, Fee, Rate, or Queue Controls

**What goes wrong:**
Every accepted transaction is announced immediately to every peer. The node ignores relay opt-outs, wtxidrelay negotiation, future fee filters, permission boundaries, preferred-peer delays, unbroadcast retry semantics, inventory caps, and bandwidth budgets.

**Why it happens:**
The network core already has an `announce_transaction` helper, so a loop over peers is the shortest path to a visible relay demo. Knots, however, uses per-peer inventory queues, intervals, caps, fee filtering, and retry behavior.

**How to avoid:**
Add a relay announcement scheduler with a fake-clock test surface. It must maintain per-peer queues, enforce inventory and rate limits, respect negotiated inventory identity, avoid relay to ineligible peers, and represent unbroadcast local transactions separately from externally learned transactions. If fee filters or rebroadcast are deferred, the roadmap and release docs must say so.

**Warning signs:**
A mempool acceptance path iterates all peer IDs immediately; no per-peer queue type exists; tests do not advance a clock; metrics count "broadcasted" without distinguishing queued, sent, skipped, rejected, or deferred.

**Phase to address:**
Phase 103: Relay Serving, Announcement, and Rebroadcast Policy.

### Pitfall 9: Changing Default Verification or Runtime Into Public Relay Participation

**What goes wrong:**
The milestone accidentally makes public-network relay the default daemon behavior, or adds live public-network checks to the default verifier. That contradicts current release boundaries and makes deterministic verification dependent on external peers.

**Why it happens:**
Transaction relay is network-facing, and implementation teams often equate "feature exists" with "feature should run by default." Existing v1.9 inbound serving is already opt-in, so the activation boundary must remain explicit.

**How to avoid:**
Keep default verification hermetic and deterministic. Public-network relay, if tested at all, belongs in opt-in UAT or manually run operator workflows. Add release-boundary checks for docs, help text, and defaults: no public relay default, no public-network CI default, no compact-block claim, and no production full-node readiness claim.

**Warning signs:**
`scripts/verify.sh` opens public peers; default config enables listener plus relay on public interfaces; README or release docs say "production-ready relay"; tests rely on live peer discovery or external transaction propagation.

**Phase to address:**
Phase 100 for activation defaults and Phase 105 for release boundary verification.

### Pitfall 10: Leaking Transaction, Peer, Permission, or Wallet-Adjacent Data in Support Surfaces

**What goes wrong:**
Relay debugging adds raw tx hex, txids/wtxids, inventory payloads, peer endpoints, permission strings, wallet-created transaction data, or high-cardinality peer labels to logs, metrics, support bundles, or operator reports.

**Why it happens:**
Relay failures are hard to diagnose, so raw payloads are attractive. v1.9 already tightened support-bundle redaction for peer-serving evidence, but transaction relay adds more sensitive and high-cardinality data.

**How to avoid:**
Use allowlisted, low-cardinality status fields. Support bundles should report counts, bounded redacted samples, capability summaries, and `Unavailable` reasons instead of raw payloads. Add sanitizer tests for tx hex, txids/wtxids where not explicitly allowed, peer endpoints, permission class names, and wallet-adjacent fields. Metrics labels must remain fixed enums, not transaction or peer identifiers.

**Warning signs:**
Structured logs include raw `tx` payloads; support markdown dumps inv lists; metrics labels contain txids, wtxids, peer addresses, or permission names; `Debug` output from relay state is embedded directly in reports.

**Phase to address:**
Phase 104: Operator, RPC, Metrics, Support Evidence, and Redaction.

### Pitfall 11: Overclaiming RPC and Operator Surfaces Before Relay Parity Exists

**What goes wrong:**
RPC or CLI surfaces expose Knots-like fields for mempool and network relay status even when behavior is scoped, disabled, or deferred. Operators infer that relay, rebroadcast, package behavior, or production readiness exists when it does not.

**Why it happens:**
RPC shapes are easy to copy before backing semantics are complete, and Open Bitcoin already has operator-facing workflows that value clear status. Filling fields with zeros, constants, or "true" flags can look harmless but creates false evidence.

**How to avoid:**
Every new operator/RPC field must have an evidence basis: implemented, unavailable, deferred, or deliberately different. Help text and release docs should use the same vocabulary as the runtime. Do not add `getrawmempool`, full `sendrawtransaction` parity, fee-filter status, or network relay claims unless the corresponding behavior has deterministic tests and parity anchors.

**Warning signs:**
`localrelay` or similar status is `true` while relay activation is disabled; copied Knots fields are hardcoded; docs omit residual risks; CLI output says "mempool synced" or "relay ready" without scoped qualifiers.

**Phase to address:**
Phase 104 for surfaces and Phase 105 for release readiness.

### Pitfall 12: Losing Parity Traceability While Adding Cross-Cutting Relay Code

**What goes wrong:**
New Rust relay, mempool, network, node, or tests are added without specific Bitcoin Knots breadcrumbs or without recording intentional differences. Later roadmap work cannot audit whether behavior matches the pinned baseline or is an Open Bitcoin-specific boundary.

**Why it happens:**
Transaction relay cuts across network processing, mempool policy, validation, chain events, RPC, and support evidence. It is tempting to cite broad files such as `net_processing.cpp` once and move on.

**How to avoid:**
Require source-breadcrumb updates in every implementation phase that adds first-party Rust source or tests under the checked paths. Use concrete anchors such as Knots `net_processing.cpp`, `node/txdownloadman.*`, `txmempool.cpp`, `policy/policy.h`, functional `p2p_tx_download.py`, RBF tests, and package tests. Use an explicit `none` breadcrumb only for genuinely Open Bitcoin-specific support glue, and record behavior differences in `docs/parity/`.

**Warning signs:**
New relay files use `none` breadcrumbs for baseline behavior; tests mention parity but have no source anchor; docs claim parity without catalog updates; phase verification does not run the breadcrumb checker.

**Phase to address:**
Phase 105 as a closeout gate, with enforcement in every implementation phase.

## Technical Debt Patterns

Shortcuts that seem reasonable but create long-term problems.

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
|----------|-------------------|----------------|-----------------|
| Submit inbound peer transactions directly to mempool admission | Fastest demo for "peer tx accepted" | No orphan handling, fallback, recent rejects, peer attribution, or bounded request lifecycle | Only for pre-relay hermetic smoke paths hidden behind disabled activation |
| Use raw hashes for all tx request state | Less type plumbing | Txid/wtxid mismatch bugs, wrong inv type, incorrect `notfound` cleanup | Never for v2.0 relay code |
| Keep `transactions_by_txid`/`transactions_by_wtxid` as the relay truth | Reuses existing serving maps | Stale, replaced, confirmed, or invalid transactions remain servable | Only as an internal cache if reconciled by mempool lifecycle tests |
| Treat static `minrelaytxfee` as full mempool pressure policy | Avoids rolling fee state | Overclaims Knots-compatible eviction behavior and reports misleading `mempoolminfee` | Acceptable only if documented as a scoped deviation and not surfaced as pressure parity |
| Enable permission tokens by name before defining effects | Quick config story | `relay`, `forcerelay`, and `mempool` become accidental broad capabilities | Never without the Phase 100 activation matrix |
| Add operator/RPC fields before semantics are implemented | UI and docs look complete | False release evidence and future compatibility debt | Only when fields explicitly return unavailable/deferred states |

## Integration Gotchas

Common mistakes when connecting relay and mempool behavior to existing project boundaries.

| Integration | Common Mistake | Correct Approach |
|-------------|----------------|------------------|
| Peer permissions | Mapping parsed permission tokens directly to active relay effects | Route every token through an activation matrix with explicit tests for inactive, ordinary, and permissioned effects |
| Peer inventory | Requesting every announced tx immediately and uniformly | Use a tx download scheduler with caps, peer eligibility, identity type, timeout, and fallback |
| Wtxid relay | Treating txid and wtxid as interchangeable transaction hashes | Keep typed identity in announcement, request, response, known-inventory, and serving state |
| Mempool admission | Mutating mempool or relay cache before all policy/replacement checks pass | Preserve atomic admission: validate first, mutate once, then notify relay state |
| Chainstate | Connecting or disconnecting blocks without mempool and relay-cache callbacks | Use block connect/disconnect/reorg hooks that remove, reconsider, and notify through normal APIs |
| RPC/CLI | Copying Knots-compatible field names before matching behavior exists | Surface scoped capability states and only expose parity-shaped values when backed by tests |
| Support bundles | Dumping raw relay state for diagnosability | Use allowlisted redacted counts, bounded samples, and sanitizer tests |
| Verification | Adding live peer tests to prove relay works | Keep default verification deterministic; put public-network checks behind opt-in UAT commands |

## Performance Traps

Patterns that work at small scale but fail as usage grows.

| Trap | Symptoms | Prevention | When It Breaks |
|------|----------|------------|----------------|
| Unbounded orphan storage | Memory growth under child-before-parent or adversarial peers | Max orphan count/bytes, eviction policy, peer cleanup, and tests at cap | As soon as a peer sends many missing-parent children |
| Immediate fanout to all peers | Bandwidth spikes, duplicate invs, hard-to-reproduce relay ordering | Per-peer inventory queues, rate caps, fake-clock tests, and eligibility checks | With tens of peers or large mempool churn |
| Oversized inbound inv/getdata handling | CPU spikes and request-state explosions | Enforce Knots-anchored or explicitly scoped caps for inv items, getdata items, in-flight txs, and per-peer announcements | During adversarial large-inventory messages |
| Full mempool recomputation on every relay tx without bounds | Slow acceptance under large mempools | Keep v2.0 load claims bounded, add benches before public relay/load claims, and avoid broad runtime promises | As mempool entries approach realistic relay sizes |
| High-cardinality metrics labels | Metrics backend growth, privacy leakage, slow dashboards | Fixed enum labels and redacted counters only | Any real peer count with per-tx or per-peer labels |
| Re-request loops for rejected or missing transactions | Repeated getdata/notfound cycles, wasted bandwidth | Recent-reject, reconsiderable, and orphan tracking with expiry | Under malformed txs, missing parents, or flaky peers |

## Security Mistakes

Domain-specific security issues beyond general application security.

| Mistake | Risk | Prevention |
|---------|------|------------|
| Letting `forcerelay` bypass policy for invalid transactions | The node forwards invalid or non-standard data and becomes an abuse amplifier | Only relay transactions that pass the scoped acceptance path; test invalid txs are not forwarded |
| No per-peer tx request and announcement caps | Memory, CPU, and bandwidth denial of service | Apply request, inv, orphan, and queue limits with deterministic cap tests |
| Txid/wtxid confusion | Malleability-related cache mistakes, wrong requests, or false "already have" results | Typed relay identity and mismatch tests |
| Raw tx/peer data in support artifacts | Privacy leakage and operational exposure | Redaction allowlists, sanitizer tests, and low-cardinality summaries |
| Public relay default drift | Users expose relay behavior without explicit consent | Explicit opt-in activation and release-boundary checks |
| Serving stale replaced or confirmed transactions | Peers receive misleading data and local policy state is bypassed | Relay serving must be tied to mempool lifecycle and bounded relay-cache state |

## UX Pitfalls

Common user experience mistakes in this domain.

| Pitfall | User Impact | Better Approach |
|---------|-------------|-----------------|
| Status says "relay enabled" when only message parsing exists | Operators overestimate readiness and may expose the node publicly | Report exact states: disabled, parsing-only, download-only, serve-only, relay-active, or unavailable |
| Rejection errors are flattened | Operators cannot distinguish missing inputs, policy rejection, replacement failure, eviction, or unsupported package behavior | Return scoped, redacted rejection categories with links to local docs |
| Support bundle exists but lacks boundary context | Reviewers treat support output as release validation | Include explicit unavailable/deferred reasons and release-boundary notes |
| CLI examples use aliases only | UAT is harder to reproduce from the repo | Provide repo-local Cargo and Bazel commands for relay/mempool workflows |
| Docs omit compact-block and public-network boundaries | Users assume full public node readiness | Keep explicit deferred/no-claim sections in release readiness and operator docs |

## "Looks Done But Isn't" Checklist

Things that appear complete but are missing critical pieces.

- [ ] **Permission activation:** Tokens parse, but verify an activation matrix exists and tests prove no accidental `relay`, `forcerelay`, `mempool`, bloom, filter, compact-block, or public-default coupling.
- [ ] **Wtxid relay:** Announcements work, but verify mismatch cases, request identity, received-tx cleanup, and `notfound` cleanup for txid and wtxid separately.
- [ ] **Tx download:** `getdata` is sent, but verify in-flight caps, expiry, disconnect fallback, notfound fallback, peer cleanup, and recent-reject behavior.
- [ ] **Orphans:** Missing-input txs are stored or rejected, but verify bounded orphanage, parent request generation, reconsideration on parent acceptance, and cap eviction.
- [ ] **Mempool admission:** A tx can enter the mempool, but verify RBF, ancestor/descendant limits, fee checks, eviction, no partial mutation on rejection, and parity breadcrumbs.
- [ ] **Chain events:** Blocks connect, but verify mempool removal, conflict cleanup, relay-cache cleanup, and reorg reconsideration.
- [ ] **Relay serving:** `getdata` returns txs, but verify replaced, evicted, confirmed, rejected, unknown, and stale transactions return the correct result.
- [ ] **Announcement/rebroadcast:** Accepted txs announce, but verify peer eligibility, rate limits, per-peer queues, wtxid negotiation, and explicit treatment of deferred fee-filter or rebroadcast behavior.
- [ ] **Operator/RPC surfaces:** Fields exist, but verify every value is implemented, unavailable, deferred, or intentionally different with tests and docs.
- [ ] **Support and metrics:** Relay status appears, but verify sanitizer tests reject raw tx hex, txids/wtxids where disallowed, peer endpoints, permission strings, and dynamic labels.
- [ ] **Release boundary:** The feature works locally, but verify no default public relay, no public-network CI default, no compact-block claim, and no production-readiness claim.

## Recovery Strategies

When pitfalls occur despite prevention, how to recover.

| Pitfall | Recovery Cost | Recovery Steps |
|---------|---------------|----------------|
| Relay-like permission over-activation | MEDIUM | Restore inactive/default-off behavior, add activation matrix tests, update release docs, and audit support/RPC surfaces for overstated state |
| Txid/wtxid request confusion | MEDIUM | Introduce typed relay identity, migrate maps and APIs, add mismatch tests, and replay existing peer inventory tests |
| Direct mempool admission from peer txs | HIGH | Insert a relay/download manager boundary, route accept/reject callbacks through it, and add fake-clock fallback tests before re-enabling relay |
| Unbounded orphan or request state | HIGH | Disable relay activation, add caps and eviction, purge persisted/debug artifacts if any, and test adversarial limits |
| Stale relay serving cache | HIGH | Tie serving to mempool lifecycle, remove stale stores, add replacement/block/reorg tests, and update support evidence |
| Misleading mempool pressure claims | MEDIUM | Either implement rolling minimum-fee state or downgrade docs/status to static-minrelay behavior with explicit deferred parity |
| Support or metrics data leak | HIGH | Remove or redact artifacts, add sanitizer regressions, review logs/metrics labels, and update support-bundle allowlists |
| Public relay or live CI drift | MEDIUM | Revert defaults, move network checks to opt-in UAT, add release-boundary checks, and update README/release readiness |
| Missing parity breadcrumbs | LOW | Add concrete source anchors, update parity docs for intentional differences, and rerun breadcrumb verification |

## Pitfall-to-Phase Mapping

How roadmap phases should address these pitfalls.

| Pitfall | Prevention Phase | Verification |
|---------|------------------|--------------|
| Activating relay-like permissions too broadly | Phase 100: Relay Activation Boundary and Permission Semantics | Activation matrix tests prove each permission effect is explicit, scoped, and default-off where required |
| Mixing txid and wtxid relay identity | Phase 101: Transaction Download, Request Scheduling, and Orphan Handling | Typed identity tests cover announcement, request, received tx, `notfound`, and mismatch cases |
| Treating `ReceivedTransaction` as direct mempool admission | Phase 101 | Peer tx acceptance tests route through relay/download state and cover accept, reject, timeout, disconnect, and fallback |
| Permanent rejection or unbounded orphan handling | Phase 101 and Phase 102 | Missing-parent tests cover bounded orphanage, parent request, reconsideration, cap eviction, and package-deferred docs |
| Mempool, relay cache, and chainstate incoherence | Phase 102: Mempool/Chainstate Admission, Pressure, and Repair | Block connect, disconnect, reorg, replacement, and eviction tests prove cache and mempool lifecycle coherence |
| Advertising pressure behavior without rolling min fee | Phase 102 | Eviction tests either prove rolling min-fee behavior or release docs mark pressure parity deferred |
| Serving any locally stored transaction | Phase 103: Relay Serving, Announcement, and Rebroadcast Policy | `getdata` tests prove stale, replaced, confirmed, rejected, unknown, and valid relay-eligible outcomes |
| Broadcasting without eligibility/rate/queue controls | Phase 103 | Fake-clock scheduler tests prove per-peer queues, caps, eligibility, and negotiated identity |
| Public relay or public-network CI default drift | Phase 100 and Phase 105 | Default config and `scripts/verify.sh` remain deterministic; release-boundary checker catches public relay or production claims |
| Leaking tx/peer/permission data in support surfaces | Phase 104: Operator, RPC, Metrics, Support Evidence, and Redaction | Sanitizer and metrics tests reject raw tx payloads, endpoints, dynamic labels, and disallowed identifiers |
| Overclaiming RPC/operator surfaces | Phase 104 and Phase 105 | Field-by-field evidence tests and docs classify each surface as implemented, unavailable, deferred, or intentionally different |
| Losing parity traceability | Phase 105 and every implementation phase | Breadcrumb checker and parity docs include concrete Knots anchors or explicit Open Bitcoin-specific `none` justifications |

## Sources

- `.planning/PROJECT.md` - project constraints, v2.0 scope, pinned Knots baseline, first-party Rust implementation, no production-readiness overclaim.
- `.planning/RETROSPECTIVE.md` - accumulated verification and release-boundary lessons through v1.9.
- `.planning/milestones/v1.9-MILESTONE-AUDIT.md` - v1.9 opt-in inbound serving result and relay-like permission watch item.
- `docs/parity/release-readiness.md` - deterministic verifier, no public-network default, redacted support evidence, and no-claim boundaries.
- `docs/parity/catalog/mempool-policy.md` - current mempool policy parity status and known gaps such as package relay, rolling minimum-fee state, and reorg repair.
- `docs/parity/catalog/p2p.md` - current P2P parity scope, inventory handling, inbound serving, and relay boundaries.
- `packages/open-bitcoin-mempool/src/lib.rs`, `types.rs`, `policy.rs`, and `pool.rs` - current pure mempool model, policy config, RBF, ancestor/descendant limits, and admission behavior.
- `packages/open-bitcoin-network/src/lib.rs`, `message.rs`, `peer.rs`, `peer/inventory_state.rs`, `resource.rs`, and `inbound/permissions.rs` - current P2P message model, wtxidrelay state, inventory request paths, resource caps, and inactive permission effects.
- `packages/open-bitcoin-node/src/network.rs`, `network/inventory.rs`, and `mempool.rs` - current node integration, peer tx handling, relay stores, and serving path.
- `packages/bitcoin-knots/src/net_processing.cpp` - pinned baseline inventory relay, `mempool` request handling, per-peer relay state, inventory broadcast intervals/caps, and relay serving guards.
- `packages/bitcoin-knots/src/node/txdownloadman.h` and `txdownloadman_impl.cpp` - pinned baseline tx announcement, request, timeout, orphan, recent-reject, and accept/reject callback behavior.
- `packages/bitcoin-knots/src/txmempool.cpp`, `policy/policy.h`, and `kernel/mempool_options.h` - pinned baseline mempool admission, eviction, fee, and size-policy anchors.
- `packages/bitcoin-knots/test/functional/p2p_tx_download.py`, `p2p_permissions.py`, `src/test/rbf_tests.cpp`, and `src/test/txpackage_tests.cpp` - pinned baseline tests for tx download, permissions, RBF, and package-related behavior.
- BIP references for protocol context: BIP 35 (`mempool`), BIP 125 (opt-in full-RBF signaling policy), BIP 130 (`sendheaders`), BIP 133 (`feefilter`), BIP 338 (`disabletx` / disable transaction relay), BIP 339 (`wtxidrelay`), BIP 330 (transaction announcement reconciliation), and BIP 331 (ancestor package relay).

*Pitfalls research for: Open Bitcoin v2.0 transaction relay and mempool participation boundary*
*Researched: 2026-06-29*
