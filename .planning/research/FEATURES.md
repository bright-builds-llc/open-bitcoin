# Feature Research

**Domain:** Open Bitcoin v2.0 bounded transaction relay and mempool participation boundary
**Researched:** 2026-06-29
**Confidence:** HIGH for feature categories and v1.9 dependencies; MEDIUM for exact implementation sequencing until roadmap phases inspect the current relay runtime paths in detail.

## Research Frame

v2.0 should make a narrow but real claim: Open Bitcoin can validate, store,
request, announce, and relay unconfirmed transactions under explicit bounded
policy, while keeping public relay defaults, compact block relay, production
service operation, production-funds wallet use, migration apply mode, packaging,
hosted dashboards, public-network CI, and production full-node readiness out of
scope.

The key project fact is that v1.9 already shipped the network participation
boundary needed for this work: opt-in inbound listener/admission, permission
classes, bounded address behavior, peer-policy runtime evidence, resource
governance, metrics/log/support projection, and deterministic release guards.
It deliberately left `relay`, `forcerelay`, and `mempool` permission effects
inactive. v2.0 should turn only those relay-like effects into scoped behavior
and should leave unrelated inactive labels, especially bloom filters and block
filters, inactive.

The existing codebase already has a pure mempool policy core and a P2P relay
skeleton: `open-bitcoin-mempool` covers standardness, relay fees, RBF,
ancestor/descendant limits, conflict replacement, and trimming;
`open-bitcoin-network` already understands `wtxidrelay`, `inv`, `getdata`,
`notfound`, and `tx`; `ManagedPeerNetwork` already stores transactions by txid
and wtxid and feeds received transactions into the managed mempool. The
milestone should therefore focus on activation boundaries, peer eligibility,
orphan/request state, durable/runtime reconciliation, observability, and parity
evidence.

This research was materially informed by repo-local guidance in `AGENTS.md`,
`AGENTS.bright-builds.md`, `standards/core/architecture.md`, and
`standards/core/verification.md`: keep relay policy in pure/core decision
models, keep socket/storage/RPC/logging effects in adapters, use repo-local
Cargo/Bazel command forms for UAT, and keep default verification deterministic.

## Feature Landscape

### Table Stakes (Users Expect These)

Features users assume exist once a node claims bounded transaction relay or
mempool participation. Missing these would make the v2.0 claim misleading.

| Feature | Why Expected | Complexity | Notes |
| --- | --- | --- | --- |
| Explicit relay activation boundary | v1.9 promised relay-like labels were inert; v2.0 must say exactly when relay is active and must not accidentally make public relay the default. | MEDIUM | Add a typed relay mode/eligibility contract that gates actual transaction fanout separately from existing `LocalPeerConfig.relay` metadata. Default behavior should remain no public relay unless an explicit v2.0 UAT/config path enables it. |
| Knots-anchored mempool admission outcomes | Transaction relay without policy admission is just message forwarding. Operators and tests need stable accept/reject/replacement/eviction outcomes. | HIGH | Reuse `Mempool::accept_transaction`; expand evidence around `DuplicateTransaction`, `MissingInput`, `NonStandard`, `RelayFeeTooLow`, `ConflictNotAllowed`, `ReplacementRejected`, `LimitExceeded`, and `CandidateEvicted`. |
| Txid/wtxid inventory negotiation | Modern relay peers expect `wtxidrelay` behavior, and Open Bitcoin already models txid/wtxid identities. | MEDIUM | Preserve `wtxidrelay`-gated `inv`/`getdata` selection. Tests should cover txid peers, wtxid peers, duplicate announcements, already-known transactions, `notfound`, and unsupported inventory types. |
| Peer transaction download manager | A relay participant needs bounded in-flight tracking, deduplication, retry/release behavior, and peer attribution for announced transactions. | HIGH | Build on `PeerState.requested_txids`, `requested_wtxids`, and Phase 94 request caps. Add transaction-specific timeout/retry/notfound outcomes rather than treating tx requests as block-sync leftovers. |
| Bounded orphan and missing-parent handling | The current mempool core rejects missing inputs; real peers often announce children before parents. A relay milestone needs a bounded answer for this. | HIGH | Add a bounded orphan/staging surface with parent discovery requests, expiry, memory/count caps, and clear reject versus deferred labels. Full package relay can remain deferred. |
| Permission-aware relay/mempool behavior | v1.9 parsed `relay`, `forcerelay`, and `mempool` tokens but recorded them as inactive. v2.0's natural value is activating them safely. | HIGH | `relay` and `mempool` should become explicit peer-policy inputs only for scoped transaction relay. `forcerelay` needs a narrow, auditable meaning; it must not bypass consensus or unbounded resource controls. Bloom/filter labels stay inactive. |
| Transaction serving and announcement fanout | If a transaction is accepted locally or from a peer, eligible peers should be able to request it and receive it, subject to known-inventory and permission/privacy policy. | HIGH | Use `transactions_by_txid`, `transactions_by_wtxid`, `serve_inventory`, and `announce_transaction`, but add fanout eligibility, source-peer suppression, per-peer known filters, queue caps, and redaction-safe evidence. |
| Mempool and chainstate reconciliation | Confirmed or conflicted mempool transactions must not keep being relayed after blocks connect or reorg state changes. | HIGH | Add block-connect cleanup and a scoped reorg boundary. Full disconnected-transaction resurrection and package repair can be separate later work if clearly documented. |
| Durable/runtime mempool state boundary | The project goal calls for durable and runtime mempool state; operators need to know what survives restart and what is rebuilt. | HIGH | Define whether v2.0 persists mempool entries, request/orphan ledgers, or only evidence snapshots. Whatever is chosen must be restart-safe, bounded, and truthful in status/support output. |
| RPC and operator surface consistency | Existing `getmempoolinfo` and `sendrawtransaction` make relay visible even before network propagation. v2.0 must align them with the relay claim. | MEDIUM | Keep `getmempoolinfo`, `sendrawtransaction`, status, dashboard, metrics, logs, and support bundles consistent. Do not silently accept unimplemented `sendrawtransaction` fee-safety parameters; either implement them or keep explicit rejection. |
| Resource and abuse governance for relay | Transaction relay adds inventory floods, orphan floods, repeated invalid txs, and request pressure beyond v1.9 inbound serving. | HIGH | Extend Phase 94 with tx-specific caps: orphan count/bytes, in-flight tx requests, per-peer invalid/reject rates, announcement batch sizes, rebroadcast/fanout queues, and low-cardinality metrics. |
| Deterministic parity and release guardrails | The roadmap consumer needs requirements that can be tested and audited against the pinned Knots baseline. | MEDIUM | Update parity roots and deterministic checkers to allow scoped v2.0 relay while still failing compact-block, public-default relay, production-readiness, and production-funds wallet claims. |

### Differentiators (Competitive Advantage)

Features that are valuable because they match Open Bitcoin's core value:
observable Knots-compatible behavior with simpler, safer internals.

| Feature | Value Proposition | Complexity | Notes |
| --- | --- | --- | --- |
| Relay decision ledger | Makes every accept/reject/request/announce decision auditable without raw peer or transaction payload leakage. | MEDIUM | A compact local ledger can power support bundles, deterministic fixtures, and operator diagnostics. Keep values redacted and bounded. |
| Functional-core relay policy | Keeps relay eligibility, orphan handling, permission effects, and fanout decisions unit-testable without sockets. | MEDIUM | Continue the existing pure-core pattern from mempool, network, and resource-governance crates. |
| Bounded opt-in relay UAT harness | Lets reviewers prove relay behavior through loopback/synthetic peers without default public-network CI. | MEDIUM | Reuse the v1.9 loopback posture and provide exact repo-local Cargo/Bazel commands in later roadmap artifacts. |
| Parity-first source anchors | Keeps v2.0 credible by tying relay behavior to `net_processing.cpp`, `txdownloadman`, `txmempool.cpp`, `validation.cpp`, `policy/`, and relay functional tests. | LOW | This is documentation/checker work, but it should be treated as a first-class feature because release claims depend on it. |
| Truthful partial support matrix | Makes the product more trustworthy by saying which relay pieces are implemented and which are deliberately deferred. | LOW | Useful for differentiating bounded transaction relay from production public relay, compact blocks, package relay, and wallet safety. |
| Redacted transaction relay support summaries | Gives operators enough evidence to debug relay without exposing raw tx hex, peer endpoints, permission strings, or wallet-sensitive data. | MEDIUM | Build on Phase 95 redaction roots and low-cardinality metric rules. |

### Anti-Features (Commonly Requested, Often Problematic)

Features that may sound attractive but should be explicitly rejected or deferred
for this milestone.

| Feature | Why Requested | Why Problematic | Alternative |
| --- | --- | --- | --- |
| Public relay by default | It sounds like a real full-node milestone. | It violates the v2.0 boundary and requires support, abuse, packaging, firewall, service, and production-readiness evidence that is not in scope. | Keep relay activation explicit and UAT-scoped; document future public-default gates. |
| Compact block relay | It is a known Bitcoin network optimization. | It is a different protocol family with `sendcmpct`, `cmpctblock`, `getblocktxn`, and reconciliation behavior beyond tx relay. | Keep compact block relay deferred with deterministic no-claim checks. |
| Production full-node readiness claim | Transaction relay is a major node capability. | v1.8 made production readiness a gated claim; relay alone does not satisfy support, packaging, service, uptime, public-network, wallet, and release gates. | Say "bounded transaction relay and mempool participation boundary." |
| Production-funds wallet claim | Local transaction submission makes wallet send flows feel closer to real use. | Relay behavior does not prove wallet safety, backup/recovery, signing policy, fee safety, or production support. | Keep wallet surfaces preview/scoped and avoid production-funds language. |
| Full package relay in the first relay milestone | Package relay is important for modern mempool behavior. | Existing mempool docs list package relay as a known gap; doing it now would broaden orphan, ancestor, RBF, validation, and peer-protocol scope substantially. | Ship bounded single-transaction relay plus bounded orphan/missing-parent handling; plan package relay later. |
| Unbounded rebroadcast or gossip fanout | It maximizes propagation in demos. | It creates privacy, resource, and DoS risk and can contradict v1.9 resource-governance boundaries. | Use bounded, eligible-peer fanout with source suppression, known filters, queue caps, and explicit rebroadcast non-claim unless implemented. |
| Treating `forcerelay` as "ignore policy" | Permissioned peers may expect preferential forwarding. | Bypassing consensus, standardness, fee, or resource controls would be unsafe and hard to defend against Knots parity. | Give `forcerelay` a scoped, testable meaning such as relay-eligibility override only after safe admission or explicit reject evidence. |
| Bloom filter, compact filter, or BIP37 serving | Tokens already exist in permission parsing. | v1.9 recorded these as inactive labels, and they require separate privacy and protocol work. | Keep these labels inactive and assert they do not activate in v2.0. |
| Full address relay expansion | Relay work often gets conflated with address gossip. | v1.9 only shipped bounded local advertisement and direct `getaddr` response behavior. | Preserve the address boundary; do not expand full address relay as part of transaction relay. |
| Silent support for unimplemented RPC safety params | `sendrawtransaction` callers may pass baseline parameters such as max fee rate. | Silently ignoring fee-safety arguments is dangerous and not parity-auditable. | Implement scoped support or keep explicit stable rejection until a phase owns it. |

## Feature Dependencies

```text
v1.9 listener/admission/resource governance
    -> explicit relay activation boundary
        -> permission-aware relay/mempool effects
            -> peer transaction download manager
                -> txid/wtxid request/serve/fanout
                    -> bounded propagation evidence

existing mempool admission policy
    -> local sendrawtransaction submission
    -> received tx validation
    -> reject/replacement/eviction evidence

received inv/getdata/tx support
    -> in-flight tx request tracking
        -> orphan/missing-parent staging
            -> mempool admission retry

chainstate block connect/reorg evidence
    -> mempool cleanup/reconciliation
        -> safe relay of only unconfirmed eligible transactions

metrics/logs/support/redaction roots
    -> relay observability
        -> deterministic release-boundary checkers
```

### Dependency Notes

- **Relay activation depends on v1.9 admission/resource policy:** Transaction relay must not bypass inbound enablement, connection caps, request caps, queue pressure, timeout/churn controls, or support redaction shipped in v1.9.
- **Permission activation depends on v1.9 permission parsing:** `relay`, `forcerelay`, and `mempool` should move from inactive diagnostic labels to scoped active effects only where requirements name their behavior. `bloomfilter` and `blockfilters` should remain inactive.
- **Fanout depends on admission:** Announcing or serving a transaction should generally require either accepted mempool state or an explicit safe serving policy for local transactions. Rejected or orphan-staged transactions need different evidence.
- **Orphan handling depends on request tracking:** Missing-parent transactions should not create unbounded queues. The node needs parent request/expiry limits before it can claim meaningful orphan participation.
- **Observability depends on all relay paths:** Status, RPC, metrics, logs, and support bundles should not infer relay health from raw message counts. They need shared typed outcomes: accepted, rejected, orphaned, requested, served, announced, suppressed, evicted, and expired.
- **Release boundaries conflict with public defaults:** A public relay default, compact block relay, or production-node claim would invalidate the milestone boundary and should require a separate scoped milestone.

## MVP Definition

### Launch With (v2.0)

Minimum viable milestone scope for bounded transaction relay and mempool
participation:

- [ ] Explicit relay activation and peer eligibility policy that keeps public relay off by default.
- [ ] Peer transaction inventory/request/response handling for txid and wtxid peers, including `inv`, `getdata`, `tx`, and `notfound`.
- [ ] Mempool admission integration for local and peer-submitted transactions with stable accept/reject/replacement/eviction evidence.
- [ ] Bounded orphan/missing-parent staging with parent request, expiry, and resource limits.
- [ ] Permission-aware activation for `relay`, `forcerelay`, and `mempool` effects without activating bloom/filter or compact-block behavior.
- [ ] Mempool cleanup/reconciliation when blocks connect and a documented reorg boundary.
- [ ] Operator/RPC/status/metrics/log/support evidence for relay outcomes with redaction.
- [ ] Deterministic parity fixtures and release-boundary checks that keep compact blocks, public relay defaults, production readiness, and production-funds wallet claims out.

### Add After Validation (v2.x)

Features to add once core bounded relay behavior is proven:

- [ ] Richer rebroadcast scheduling — only after basic fanout and suppression evidence is stable.
- [ ] Fee histogram or richer mempool inspection — useful once mempool state is durable and operator-facing.
- [ ] Stronger restart-persistent mempool reload behavior — if v2.0 ships with a narrower durable evidence boundary.
- [ ] Public-network relay UAT harness — only as explicit opt-in evidence, not default CI.
- [ ] More complete reorg-driven mempool repair — after block-connect cleanup and basic reorg boundaries are verified.

### Future Consideration (Post-v2.0)

Features to defer because they broaden the claim:

- [ ] Package relay — requires separate package policy, orphanage, peer protocol, and parity work.
- [ ] Compact block relay — separate protocol milestone.
- [ ] Public relay by default — requires support, abuse, service, packaging, firewall, and production-readiness gates.
- [ ] Production full-node readiness — requires the v1.8 production gates, not just relay.
- [ ] Production-funds wallet use — requires wallet safety and support evidence.
- [ ] BIP37 bloom filters, compact filters, and full address relay — separate protocol/privacy surfaces.

## Feature Prioritization Matrix

| Feature | User Value | Implementation Cost | Priority |
| --- | --- | --- | --- |
| Explicit relay activation boundary | HIGH | MEDIUM | P1 |
| Mempool admission outcome evidence | HIGH | MEDIUM | P1 |
| Txid/wtxid inventory and request handling | HIGH | MEDIUM | P1 |
| Peer transaction download manager | HIGH | HIGH | P1 |
| Bounded orphan/missing-parent handling | HIGH | HIGH | P1 |
| Permission-aware relay/mempool effects | HIGH | HIGH | P1 |
| Transaction serving and fanout policy | HIGH | HIGH | P1 |
| Mempool chainstate reconciliation | HIGH | HIGH | P1 |
| RPC/operator/metrics/log/support alignment | HIGH | MEDIUM | P1 |
| Deterministic parity and claim guardrails | HIGH | MEDIUM | P1 |
| Relay decision ledger | MEDIUM | MEDIUM | P2 |
| Richer rebroadcast scheduling | MEDIUM | HIGH | P2 |
| Fee histogram/richer mempool inspection | MEDIUM | MEDIUM | P2 |
| Package relay | HIGH | HIGH | P3 |
| Compact block relay | MEDIUM | HIGH | P3 |
| Public relay by default | HIGH | HIGH | P3, future gated |

**Priority key:**

- P1: Must have for v2.0 launch
- P2: Should have if it directly supports P1 evidence without broadening scope
- P3: Future consideration or explicit non-goal for this milestone

## Reference Feature Analysis

| Feature | Bitcoin Knots Baseline | Open Bitcoin Through v1.9 | v2.0 Approach |
| --- | --- | --- | --- |
| Transaction download orchestration | `TxDownloadManager` tracks announcements, already-have state, requests, orphans, notfound, and mempool callbacks. | Peer state tracks requested txids/wtxids and can request/serve tx inventory, but v1.9 kept mempool propagation as a non-claim. | Add a bounded transaction download manager with per-peer state, orphan handling, rejection callbacks, and deterministic tests. |
| Mempool admission | Knots admission spans validation, policy, RBF, ancestor/descendant, package/orphan interactions, and mempool removal events. | Pure `Mempool` already validates standardness, fees, RBF conflicts, limits, and trimming against chainstate snapshots. | Reuse the core, add relay-facing outcome evidence, cleanup/reorg boundaries, and update parity docs for any intentional differences. |
| Relay permissions | Knots permission concepts include relay-like behavior for selected peers. | v1.9 parses `relay`, `forcerelay`, and `mempool` but records them as inactive labels. | Activate only scoped relay/mempool effects, document exact behavior, and keep unrelated labels inactive. |
| RPC mempool surface | Knots exposes broad mempool and raw transaction RPC behavior. | Open Bitcoin already has `getmempoolinfo` and `sendrawtransaction`; some safety params are explicitly unsupported. | Keep the existing surface truthful, implement or explicitly reject unsupported parameters, and align local submission with relay evidence. |
| Resource governance | Knots has mature peer DoS and transaction download controls. | v1.9 added message, queue, request, timeout, churn, and reconnect controls. | Extend those controls with tx-specific caps for orphan pool, in-flight requests, invalid txs, fanout queues, and rebroadcast. |
| Release posture | Knots is a production Bitcoin node baseline. | Open Bitcoin has source-built opt-in inbound evidence only; production readiness and relay stayed deferred. | Claim bounded transaction relay/mempool participation only, with guardrails against compact blocks, public default relay, and production claims. |

## Dependencies on v1.9 Behavior

| v1.9 Capability | v2.0 Dependency |
| --- | --- |
| INB-01 through INB-05 opt-in listener/admission | Relay must run only through explicit enabled listener/admission paths and must preserve inbound/outbound counts and diagnostics. |
| PERM-01 through PERM-04 permission classes | Previously inactive `relay`, `forcerelay`, and `mempool` labels become the natural activation surface, but only after requirements define their effects. |
| ADDR-01 through ADDR-04 address boundaries | Transaction relay must not imply full address relay, unsolicited address gossip, DNS seed discovery, or public inbound defaults. |
| EVICT-01 through EVICT-04 peer-policy decisions | Relay abuse should feed bounded peer-policy evidence without claiming production banlist parity or public ban enforcement. |
| DOS-01 through DOS-05 resource governance | Transaction relay inherits message, inventory, request, queue, timeout, churn, and reconnect caps; v2.0 adds tx-specific controls. |
| BOUND-01 through BOUND-06 release guardrails | Guardrails must be updated from "no transaction relay" to "only scoped bounded transaction relay" while continuing to forbid compact blocks, public defaults, production service, and production readiness. |
| Retained inbound metrics and structured logs | Relay metrics/logs must stay low-cardinality and sanitized, with no raw peer identifiers, endpoints, permission strings, tx hex, or wallet material. |
| Support-bundle redaction roots | Relay support evidence must summarize outcomes and redacted identifiers rather than embedding raw transactions or peer tables. |
| Loopback/synthetic UAT posture | v2.0 UAT should remain explicit, copy-pasteable, and repo-local; default `bash scripts/verify.sh` remains deterministic and public-network-free. |

## Sources

- `.planning/PROJECT.md` (v2.0 scope, target features, release boundaries)
- `.planning/MILESTONES.md` (v1.9 shipped capabilities and residual relay risk)
- `.planning/milestones/v1.9-REQUIREMENTS.md` (INB/PERM/ADDR/EVICT/DOS/BOUND dependencies)
- `docs/parity/release-readiness.md` (v1.9 boundary matrix and no-claim language)
- `docs/parity/index.json` (machine-readable parity roots and v1.9 surfaces)
- `docs/parity/catalog/p2p.md` (P2P coverage, Knots anchors, v1.9 boundaries)
- `docs/parity/catalog/mempool-policy.md` (mempool coverage and known gaps)
- `packages/open-bitcoin-mempool/src/lib.rs`, `types.rs`, `pool.rs`, `policy.rs`, `error.rs`
- `packages/open-bitcoin-network/src/lib.rs`, `message.rs`, `peer.rs`, `peer/inventory_state.rs`, `resource.rs`, `inbound/permissions.rs`
- `packages/open-bitcoin-node/src/network.rs`, `network/inventory.rs`, `mempool.rs`
- `packages/open-bitcoin-rpc/src/method/node.rs`, `dispatch/node.rs`, `context/network.rs`
- Pinned Knots anchors: `packages/bitcoin-knots/src/net_processing.cpp`, `src/node/txdownloadman_impl.cpp`, `src/node/txdownloadman.h`, `src/txmempool.cpp`, `src/validation.cpp`, `src/policy/`, `test/functional/p2p_tx_download.py`, `test/functional/feature_rbf.py`, and mempool functional tests.

*Feature research for: Open Bitcoin v2.0 bounded transaction relay and mempool participation boundary*
*Researched: 2026-06-29*
