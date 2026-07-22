# Pitfalls Research

**Domain:** Knots-compatible package admission/relay and long-lived mempool policy
**Researched:** 2026-07-22
**Confidence:** HIGH for pinned Knots behavior and current Open Bitcoin integration seams; MEDIUM for exact Rust dynamic-memory accounting and crash-durability guarantees until requirements define their tolerances

## Critical Pitfalls

### Pitfall 1: Treating a Package Call as Globally Atomic

**What goes wrong:**
An implementation either rolls back transactions that Knots would have accepted individually, or mutates the live mempool one transaction at a time and leaves a half-applied subpackage after a later member fails. Trimming during evaluation is particularly dangerous: a parent or referenced mempool transaction can disappear while later members still rely on cached coins, aggregate fees, or topology derived from the earlier state.

**Why it happens:**
“Package atomicity” sounds like all-or-nothing admission, but the pinned `AcceptPackage` flow first accepts individually valid transactions and then evaluates only reconsiderable or missing-input members as a subpackage. The accepted subpackage must have one coherent commit point, while the overall RPC result may legitimately be partial. The current Open Bitcoin entry point is single-transaction mutation, so repeatedly calling it is the tempting shortcut.

**How to avoid:**
Model package-wide status separately from ordered per-wtxid results. Evaluate each Knots-equivalent subpackage against one prospective mempool state, defer size trimming until the subpackage commit has completed, and apply one `MempoolDelta` to the graph and all dependent caches. Preserve individually accepted or already-present results even when a later subpackage fails. After final pressure trimming, rewrite the result for every accepted member no longer present to `mempool full` rather than reporting stale success.

Required tests include individually-valid-parent plus invalid-child partial acceptance, no mutation on subpackage failure, package members temporarily exceeding the cap without intra-package trimming, post-submit eviction result rewriting, mid-package replacement, and cached-input invalidation cases mined from `mempool_limit.py`.

**Warning signs:**

- An implementation loops over `accept_transaction` and attempts rollback after failure.
- A test asserts that any package error leaves every input transaction absent.
- Pressure trimming runs between package members.
- Package success is one boolean with no final per-member membership check.
- Serving or fanout state is updated before the package delta commits.

**Phase to address:**
Build order 3, **Package vocabulary and staged admission**, with prerequisites from build orders 1–2. Re-verify the cross-cache commit in build order 5 before persistence or transport is added.

***

### Pitfall 2: Letting Package Topology and Derived Indexes Diverge

**What goes wrong:**
An unsorted package, duplicate txid, same-input conflict, missing unconfirmed parent, or unsupported topology reaches expensive validation. After admission, replacement, expiry, or eviction, parent/child links, spent-outpoint ownership, ancestor/descendant aggregates, and eviction ordering can disagree. The node may retain a child without its required parent, double-count fees, select the wrong eviction package, or falsely recover a valid child as missing-parent.

**Why it happens:**
Package validity spans several distinct invariants: the general context-free limit is 25 transactions and 404,000 weight units, direct dependencies must be parent-before-child, txids must be unique, inputs cannot conflict across transactions, and submitted packages are restricted to a child with all unconfirmed direct parents. P2P auto-assembly is narrower still: the pinned path handles a 1-parent/1-child candidate. Open Bitcoin currently reconstructs all topology from a `HashMap` after each single admission and its durable snapshot is txid-sorted, not dependency-sorted.

**How to avoid:**
Parse raw transaction vectors into typed package shapes before admission. Keep general test-accept, direct child-with-unconfirmed-parents submission, and opportunistic P2P 1p1c as explicit types or modes. Stage canonical entry, spent-outpoint, adjacency, aggregate-stat, and eviction-index updates together. Add an invariant checker that recomputes topology from transactions and compares it to maintained indexes after every randomized admission/removal sequence in tests. Topologically sort persisted records during recovery and classify cycles, missing parents, and same-txid/different-witness records explicitly.

Test exact count/weight boundaries, empty input, duplicate txid, mutated witness with the same txid, cross-package double spend, unsorted dependency, parent chains incorrectly presented as sibling parents, missing unconfirmed parent, replacement of an ancestor used elsewhere in the package, and eviction of a parent with a wide descendant set.

**Warning signs:**

- A public package API accepts `Vec<Transaction>` with no fallible constructor.
- The same code path claims to support arbitrary DAG submission and P2P 1p1c.
- Recovery or result ordering uses txid order as a proxy for dependency order.
- Adjacency, aggregate stats, and eviction keys are updated in separate passes that can fail independently.
- Tests inspect accepted txids but never assert graph/index invariants.

**Phase to address:**
Build order 3 for package types and staged topology; build order 4 for the narrow P2P package shape; build order 5 for cross-cache removal invariants; build order 6 for topological recovery.

***

### Pitfall 3: Collapsing Static Relay Fee and Rolling Mempool Fee Into One Threshold

**What goes wrong:**
A low-fee parent either receives free relay because package aggregate feerate incorrectly bypasses `minrelaytxfee`, or valid CPFP is rejected because the implementation requires every member to meet the rolling mempool floor individually. Fee decay may begin merely because time passed, decay at the wrong occupancy rate, move backward after a clock jump, or produce rounding mismatches at externally visible sat/kvB boundaries.

**Why it happens:**
Both values look like “the minimum fee,” but they have different policy roles. The package aggregate may satisfy the dynamic rolling floor for an eligible subpackage; it does not waive the static relay floor. The pinned rolling state also has non-obvious gates: an eviction bump does not decay until a block connects, updates occur only after more than 10 seconds, the 12-hour half-life shortens below one-half and one-quarter occupancy, and values below half the incremental relay fee collapse to zero.

**How to avoid:**
Represent configured minimum relay fee, incremental relay fee, rolling state, and effective mempool minimum as different types or named fields. Keep `RollingMinimumFeeState` pure: transitions receive an injected timestamp, accounted usage, configured capacity, and typed block/eviction events. Clamp or reject non-monotonic sampled time deliberately. Confine `f64::powf` and Knots-compatible rounding to the rolling module and expose integer `FeeRate` values. Admission must document which fee set contributes to each effective package calculation.

Use differential fixtures for: eviction rate plus incremental bump; no decay before block connect; block-connect timestamp reset; the strict 10-second gate; 12-, 6-, and 3-hour half-lives; exactly one-half and one-quarter occupancy; rounding around integer sat/kvB values; incremental-floor behavior; collapse-to-zero threshold; and backward/large-forward clock inputs.

**Warning signs:**

- A single mutable field named `min_relay_feerate` is used for static configuration, rolling policy, RPC output, and package admission.
- A timer directly mutates fee state or the pure mempool reads `SystemTime`.
- Fee decay tests sleep or rely on the machine clock.
- The rolling floor falls before a post-bump block event.
- `getmempoolinfo` cannot distinguish `minrelaytxfee` from `mempoolminfee`.

**Phase to address:**
Build order 1, **Resource and fee primitives**, followed by build order 2, **Rolling fee, expiry, and eviction core**. Package admission in build order 3 must consume these finished semantics rather than inventing a temporary fee rule.

***

### Pitfall 4: Claiming Pressure Parity While Measuring Only Virtual Size

**What goes wrong:**
The mempool appears bounded in tests but uses substantially more memory under realistic transactions, packages, and index overhead. Eviction happens at different times than Knots, the rolling floor is bumped from the wrong victim set, and a sustained burst causes repeated full-map clones, topology recomputation, and whole-map victim scans. Descendants, relay entries, compact reconstruction candidates, reject state, or unbroadcast markers can survive after their transaction has been evicted.

**Why it happens:**
Open Bitcoin currently enforces `max_mempool_virtual_size`; Knots enforces an estimate from `DynamicMemoryUsage()`. The current Rust admission clones the complete entry map, recomputes all graph relations and aggregates, then repeatedly scans and recomputes during trimming. This is easy to reason about for small tests but is the wrong performance and pressure model for a long-lived mempool.

**How to avoid:**
Define an auditable Rust accounted-memory model separately from transaction vsize. Use vsize for fee calculations and accounted bytes for the capacity decision. Select the lowest descendant-score entry with deterministic tie-breaking, remove it and every descendant as one lifecycle delta, and bump the rolling floor from the removed descendant package rate plus the incremental relay fee. Maintain bounded topology and eviction indexes incrementally, while retaining a test-only full recomputation oracle.

Every removal delta must cover mempool entries, spent outpoints, ancestry/descendency, relay serving bodies, txid/wtxid indexes, peer known/request state, fanout queues, compact-extra/partial reconstruction inputs, orphan or reconsiderable-package state, unbroadcast membership, persistence dirty state, and aggregate evidence. Benchmark full 25-member packages, wide descendant sets, maximum-length chains, repeated fill/trim/refill cycles, and adversarial low-fee churn.

**Warning signs:**

- Status exposes only `total_virtual_size` and labels it memory usage.
- The configured mempool cap is compared to vsize while docs claim Knots `maxmempool` parity.
- `entries.clone()` or full `recompute_state()` remains on every sustained-pressure admission path with no benchmark.
- Eviction cleanup receives one selected txid rather than the complete descendant removal set.
- The rolling floor is bumped from an individual victim rate rather than the removed descendant package plus increment.

**Phase to address:**
Build orders 1–2 for accounting, eviction, and rolling policy; build order 5 for complete removal propagation; build order 9 for adversarial pressure and performance evidence before any scale claim.

***

### Pitfall 5: Persisting the Wrong State—or Failing to Persist It at All

**What goes wrong:**
Restart resets transaction age, drops valid children because records replay in txid order, loses local unbroadcast membership, trusts stale derived topology, or restores a rolling fee that Knots resets. A schema change can also make every namespace unreadable if a mempool-only DTO change casually increments the repository-wide `SchemaVersion`. Conversely, tests may prove that the Fjall adapter can save a snapshot while the daemon never schedules a real checkpoint.

**Why it happens:**
The current snapshot stores transaction, txid/wtxid, fee, and vsize; it omits entry acceptance time and unbroadcast membership, sorts by txid, and has load/save adapters but no production save call found outside tests. The generic snapshot codec applies one global schema version. The pinned mempool dump persists entry time, fee metadata, and unbroadcast txids, but it does not serialize rolling fee variables or volatile peer/package indexes.

**How to avoid:**
Define the supported durability contract first: clean shutdown, periodic checkpoint, and sudden-crash loss bounds are separate promises. Capture a complete owned snapshot under the authoritative state lock, release the lock, then perform Fjall I/O. Persist source records, acceptance time, and unbroadcast membership; rebuild topology, pressure indexes, serving caches, and queues. Reset rolling fee on restart, then enforce current policy and capacity during replay. Restore an unbroadcast marker only if its transaction recovered successfully.

Add mempool-specific schema compatibility or an explicit migration path; do not invalidate unrelated chainstate/wallet/runtime records for a mempool DTO field. Test legacy v1 snapshots, corrupt and incompatible metadata, parent-after-child serialized input, cycles/missing parents, stale unbroadcast ids, replay-time expiry and eviction, buffered versus sync checkpoint semantics, failed periodic write followed by successful retry, clean-shutdown final checkpoint, and crash before/after persistence generation acknowledgement.

**Warning signs:**

- Snapshot records are still sorted only by txid.
- Entry time is set to restart time during replay.
- Rolling fee, peer queues, adjacency maps, or package candidates appear in the durable DTO.
- Adding a mempool field requires a global schema bump with no tests for other namespaces.
- `save_mempool_snapshot` appears only in storage tests.
- Status says “durable” without stating last successful checkpoint and crash-loss bounds.

**Phase to address:**
Build order 1 introduces entry time; build order 5 defines the authoritative dirty delta; build order 6 owns schema, topological recovery, periodic checkpoint, and restart semantics. Build order 9 must run restart and failure-injection scenarios.

***

### Pitfall 6: Rebroadcasting the Whole Mempool or Clearing Delivery State Too Early

**What goes wrong:**
The node periodically announces remote-origin transactions, emits on a fixed schedule, fans out to ineligible peers, or retries without existing queue and rate bounds. This leaks timing and origin information, creates bandwidth amplification, and fingerprints the implementation. At the opposite extreme, clearing a transaction from the unbroadcast set when an `inv` is merely queued means a transaction never gets another delivery attempt even though no peer requested or received it.

**Why it happens:**
“Periodic rebroadcast” is easily interpreted as wallet-style or whole-mempool reannouncement. The pinned node behavior is narrower: locally submitted, relay-enabled transactions enter an unbroadcast set and initial delivery is retried every randomized 10–15 minutes. Lifecycle removal clears the marker, and successful processing of an eligible transaction `getdata` serve is the acknowledgement boundary. The current Open Bitcoin seam emits only `RebroadcastDeferred` evidence and does not yet own a scheduler or durable set.

**How to avoid:**
Name the feature **initial broadcast retry** in types and operator text. Add only newly accepted local relay-enabled transactions to a bounded unbroadcast set; do not add peer-originated transactions or re-add a duplicate already in the mempool. Sample fresh 10–15 minute jitter for every cycle in the shell, pass it to pure scheduling logic, and route retries through the same activation, peer eligibility, txid/wtxid negotiation, origin suppression, fee filter, queue cap, rate limit, and transport path as first announcements.

Keep `due`, `queued`, `suppressed`, `served`, and `cleared` as different outcomes. Clear at the explicitly documented eligible transaction-serve/transport boundary or on confirmation, replacement, eviction, or expiry—never on timer firing or inventory enqueue alone. Persist membership with the mempool snapshot and restore only surviving transactions. Test zero peers, reconnect, queue saturation, write failure, no `getdata`, eligible `getdata`, repeated cycle jitter, lifecycle removal, restart, relay disabled, and per-peer fanout bounds.

**Warning signs:**

- The scheduler iterates every mempool entry.
- Retry interval is a constant or sampled once at process startup.
- A peer-originated accepted transaction gains `local_origin=true` after restart.
- The unbroadcast set is cleared by `enqueue_admission` or `Inv` emission.
- Retry bypasses `TxFanoutQueue` or the authoritative live peer outboxes.
- Metrics report “broadcast successful” from a due or queued action.

**Phase to address:**
Build order 5 for lifecycle/unbroadcast state, build order 6 for persistence, and build order 7 for receive-independent scheduling, bounded fanout, and transport acknowledgement.

***

### Pitfall 7: Recreating Split Authority in a Background Task or RPC Adapter

**What goes wrong:**
RPC admission, the maintenance worker, persistence, transport, and status each hold a mempool clone or private rolling-fee/unbroadcast state. A timer sees stale chainstate, an RPC admits against a different floor, metrics sample a different network instance, or persistence snapshots a generation that never existed atomically. Holding the authority lock across storage or async transport can instead deadlock or stall all peer processing.

**Why it happens:**
Schedulers and RPC frameworks encourage local state ownership. v2.1 already had to establish `ManagedNetworkHandle` as the mutex-backed authoritative network/chainstate/mempool view and tie achieved evidence to the transport instance that actually emits messages. Package policy adds several tempting side stores: package candidates, fee clocks, unbroadcast sets, checkpoint dirty flags, and operator counters.

**How to avoid:**
Add typed package-admission, maintenance-tick, snapshot-capture, and transport-receipt commands to the existing `ManagedNetworkHandle`. The daemon task owns only clock/randomness, wakeup, I/O, and retry orchestration. RPC parses input and projects output but invokes the same authority command as P2P. Capture owned deltas/snapshots while locked; perform Fjall and socket effects after releasing the lock; feed typed receipts back through a new short authority mutation. Add identity/provenance integration tests proving RPC, sync, inbound sessions, maintenance, metrics, and support snapshots share cloned handles to one authority.

**Warning signs:**

- `Mempool::new`, `RollingMinimumFeeState::default`, or an unbroadcast set appears in RPC/daemon modules.
- A timer callback mutates a cloned `ManagedPeerNetwork`.
- Status is synthesized from independently locked mempool and peer snapshots.
- Storage or `.await` occurs while the `ManagedNetworkHandle` mutex guard is held.
- Attempt counters update on one runtime while successful-write receipts update another.

**Phase to address:**
Build order 5 defines the one-delta integration boundary; build order 6 uses owned snapshot capture; build order 7 wires scheduler and transport receipts; build order 8 consumes one authoritative operator snapshot. Build order 9 adds a deterministic no-duplicate-authority guard.

***

### Pitfall 8: Generalizing the P2P Package Path Beyond the Pinned Boundary

**What goes wrong:**
The implementation adds a package inventory or package wire command, assembles arbitrary multi-parent packages from peers, loses sender attribution, or caches package rejects without bounds. It becomes vulnerable to CPU/memory pressure and child-crowding censorship while claiming interoperability that the pinned Knots tree does not provide.

**Why it happens:**
Local `submitpackage` supports a broader child-with-unconfirmed-parents shape, which makes it easy to assume the P2P path should transmit that object. The pinned path instead uses ordinary transaction messages and opportunistically constructs only a 1p1c pair when a reconsiderable parent and orphan child match. Knots prefers child evidence associated with the parent sender and caches a hash of sorted little-endian wtxids for rejected packages.

**How to avoid:**
Keep transport unchanged: ordinary `inv`/`getdata`/`tx` and txid/wtxid negotiation. Distinguish hard recent rejects from fee-reconsiderable rejects. Assemble only a typed 1p1c candidate, preserve the sender for each member, prefer eligible same-peer child evidence, cap attempts per parent/peer/tick, and store rejected package hashes in a capacity-limited rotating structure. Hash exactly the pinned sorted little-endian wtxid concatenation. Feed accepted members back into ordinary per-transaction fanout in topological order; do not promise atomic network propagation.

**Warning signs:**

- New message names such as `package`, `getpackage`, `sendpackages`, `ancpkginfo`, `getpkgtxns`, or `pkgtxns` appear.
- P2P candidates contain more than one reconsiderable parent.
- A package result has one origin peer even though members came from different sources.
- Rejected package identities live in an unbounded `HashSet`.
- Accepted package members bypass ordinary inventory negotiation or peer queue limits.

**Phase to address:**
Build order 4, **Package-aware download/orphan bridge**, with transport deliberately deferred until build order 7. Validate scope again in build order 9 claim guardrails.

***

### Pitfall 9: High-Cardinality, Sensitive, or Premature Observability

**What goes wrong:**
Metrics create a series per txid, wtxid, package hash, peer, fee value, or rejection string. Logs and support bundles leak raw transactions, transaction origin, peer endpoints, permission strings, credentials, or correlations between a locally submitted package and a peer request. Operator status reports a queued attempt as successful relay or hides partial package results behind aggregate success.

**Why it happens:**
Package and pressure debugging naturally revolves around identifiers and detailed per-member reasons. The existing status contract intentionally uses fixed counters and sanitization, while v2.1 established the stronger rule that achieved effects derive from the authoritative transport, not eligibility proxies. Adding fields ad hoc to every surface can bypass both protections.

**How to avoid:**
Define a shared sanitized evidence contract first. Metrics use fixed `MetricKind` variants and numeric values only. Logs use allowlisted low-cardinality enums and bounded counts; support bundles exclude raw tx material, txids/wtxids, package hashes, peer ids/endpoints, dynamic labels, permission strings, and credentials. Keep detailed per-wtxid results only in the direct authenticated RPC response where the caller supplied the transactions; shared status carries aggregates.

Distinguish admission accepted, still-present after trim, fanout eligible, queued, attempted, successfully emitted, requested, served, suppressed, and cleared. Add forbidden-key/value tests over serialized status, logs, metrics, and support bundles, plus cardinality tests showing hostile unique transactions do not create new metric names or labels.

**Warning signs:**

- A metric or structured-log label accepts `String` from a validation error.
- Status includes a list of package member ids, peers, or exact retry timestamps.
- “Announced” increments when a peer was merely eligible or a queue entry was created.
- Support evidence copies the direct `submitpackage` response wholesale.
- Each surface independently translates internal result enums.

**Phase to address:**
Design the evidence enums alongside build orders 1–5, but expose them only in build order 8 after achieved-effect paths exist. Run redaction/cardinality and receipt-lineage checks again in build order 9.

***

### Pitfall 10: Letting “Package Relay” Expand the Product Claim

**What goes wrong:**
README, status, RPC help, release notes, or tests imply BIP331/general package wire relay, public transaction relay by default, guaranteed public propagation, cluster mempool, production-scale sustained-pressure readiness, public-network CI, production service operation, production full-node readiness, or production-funds wallet safety.

**Why it happens:**
The short milestone name is broader than the actual pinned behavior. A successful local package test or one opt-in public-network observation is easy to present as a shipped network-wide capability. Existing v2.0/v2.1 no-claim checks currently classify package relay as deferred and will need precise replacement wording rather than simple removal.

**How to avoid:**
Adopt a claim taxonomy in code and docs: `local_package_admission`, `opportunistic_p2p_1p1c`, `ordinary_transaction_fanout`, and `initial_broadcast_retry` are in scope. `general_package_wire_relay`, cluster mempool, public/default relay, guaranteed propagation, public-network default verification, production readiness, and production-funds wallet use remain deferred. Update parity roots and deviations with concrete Knots anchors. Keep public-network UAT explicit and opt-in; default `bash scripts/verify.sh` must use fake clocks, fixture peers, temporary stores, and deterministic pressure scenarios.

Add positive and negative checker fixtures. Positive checks must require the exact bounded claim across README, runtime/operator docs, parity catalogs, RPC help, status schemas, and release notes. Negative fixtures must fail for protocol, public-default, production, whole-mempool rebroadcast, and guaranteed-propagation language.

**Warning signs:**

- Existing “package relay deferred” guard text is deleted without a narrower replacement contract.
- Docs say “supports package relay” with no topology, transport, activation, or evidence qualifier.
- A live-network test enters pre-commit/default CI.
- Successful local admission is described as guaranteed broadcast.
- Benchmarks or short synthetic tests are used to claim production-scale long-lived policy.

**Phase to address:**
State the boundary in build order 4 when P2P scope is fixed, preserve it through build orders 7–8, and enforce it in build order 9, **Parity, adversarial pressure, restart, and release guardrails**.

## Technical Debt Patterns

| Shortcut | Immediate Benefit | Long-term Cost | When Acceptable |
| --- | --- | --- | --- |
| Call single-transaction admission repeatedly | Reuses current API | Wrong partial-acceptance semantics, mid-package trimming, rollback hazards, duplicate validation | Never for package submission; acceptable only for independent transactions outside a package context |
| Clone and fully recompute the complete mempool graph | Simple correctness model | Admission and eviction collapse under sustained realistic pressure | As a test oracle and small deterministic fixture only |
| Keep the vsize-only cap | Minimal code change | Divergent eviction timing, rolling floor, memory bound, and operator truth | Only with an explicit intentional-difference label and no Knots pressure-parity claim |
| Store package results as one success flag | Small response model | Loses partial results, different-witness cases, effective fee groups, and post-trim failures | Never on package RPC or audit surfaces |
| Persist derived ancestry/peer/rolling state | Faster apparent restart | Stale topology, stale peers, changed Knots restart behavior, schema fragility | Never; rebuild derived and volatile state |
| Bump global `SchemaVersion` for mempool fields | Easy incompatibility rejection | Unnecessarily invalidates unrelated namespace snapshots | Only as part of a planned repository-wide migration with fixtures for every namespace |
| Use a fixed rebroadcast interval | Deterministic implementation | Network fingerprinting and synchronized retry bursts | Deterministic tests only, via injected jitter |
| Clear unbroadcast on inventory enqueue | Easy completion accounting | Stops retry before any transaction request/serve evidence | Never |
| Add arbitrary validation strings as labels | Rich debugging | Cardinality explosion and sensitive leakage | Direct authenticated RPC response only; never metrics/shared logs/support |
| Make maintenance message-driven | No new daemon task | Idle nodes never expire, decay, retry, or checkpoint | Never for long-lived policy |

## Integration Gotchas

| Integration | Common Mistake | Correct Approach |
| --- | --- | --- |
| Package engine → admission bridge | Apply member side effects as each member validates | Return one staged delta per accepted subpackage, commit mempool first, then project every add/remove to dependent caches |
| Rolling floor → package fee policy | Let aggregate package fee bypass both fee floors | Static minimum applies per scoped policy; aggregate effective fee may satisfy only the dynamic floor for eligible subpackages |
| Block connect → fee decay | Remove confirmed transactions but forget the post-bump block gate | Route a typed block-connected event through the authoritative mempool transition and update the decay timestamp/gate there |
| Pressure eviction → relay/compact/orphan state | Remove only from `Mempool.entries` | Apply the full descendant removal delta to serving, fanout, peer known/request state, compact inputs, package candidates, unbroadcast, persistence, and evidence |
| Orphanage → P2P package admission | Reconsider child alone or assemble arbitrary parents | Preserve hard versus reconsiderable failure, build only bounded sender-aware 1p1c candidates, then call the shared package engine |
| Package acceptance → fanout | Announce a package object or one summary inventory | Queue each still-present member in parent-before-child order through ordinary txid/wtxid peer policy and bounds |
| Retry scheduler → transport | Treat timer firing or `inv` queueing as delivery | Produce bounded emissions, collect the documented eligible transaction-serve/write receipt, then update unbroadcast state |
| Snapshot capture → Fjall | Serialize while holding the runtime mutex or write each related field separately | Capture one owned versioned snapshot under authority, release the lock, then persist with explicit strength/generation evidence |
| Recovery → current policy | Trust stored order, aggregates, or old rolling floor | Validate records, topologically replay against authoritative chainstate/current config, rebuild indexes, restore surviving unbroadcast markers, reset rolling state |
| RPC → package policy | Reimplement package validation in `open-bitcoin-rpc` | Parse/format only; both RPC and P2P call the same authoritative package admission command |
| Metrics worker → runtime authority | Sample a separately constructed network or counters | Obtain one `ManagedNetworkOperatorSnapshot` from the same cloned `ManagedNetworkHandle` used by transport and RPC |
| Default verifier → long-run behavior | Add sleeps, live peers, or public-network gates | Use fake clocks, fixed jitter inputs, fixture peers, temporary Fjall stores, and virtual-time pressure sequences; keep live UAT opt-in |

## Performance Traps

| Trap | Symptoms | Prevention | When It Breaks |
| --- | --- | --- | --- |
| Full mempool clone and full topology recomputation per member | Admission latency rises with pool size; allocator churn; authority mutex held longer | Stage incremental adjacency/aggregate/index deltas; retain full recomputation only as a test oracle | Sustained realistic mempool traffic, especially multi-member packages |
| Repeated whole-map victim scan plus recomputation | One pressure burst causes long pauses and nonlinear eviction time | Maintain deterministic descendant-score ordering and batch the complete removal delta | Wide descendant graphs and repeated fill/trim/refill cycles |
| Unbounded reconsiderable/package reject history | Memory grows with unique hostile wtxids; honest candidates crowded out | Capacity-limited rotating filters/sets with typed eviction counters | Adversarial unique package traffic; can grow without bound even at low acceptance |
| Unbounded maintenance work per tick | Timer holds authority long enough to starve peer/RPC work | Cap expiry scans, retry candidates, checkpoint work, and emissions per tick; carry bounded continuation state | Large recovered pools or long idle periods followed by one wakeup |
| Full snapshot on every mutation | Write amplification and lock contention | Dirty generations, coalesced periodic checkpoints, forced clean-shutdown checkpoint, explicit crash-loss window | Normal relay volume on durable nodes |
| Synchronous storage/network while authority is locked | Peer stalls, deadlock risk, poisoned state after adapter failure | Capture commands/snapshots while locked; execute effects outside; apply receipts in a short follow-up mutation | Any slow disk, backpressured peer, or failed write |
| Evidence retains per-event identifiers | Memory/cardinality grows with hostile input even if mempool stays bounded | Fixed aggregate counters plus bounded recent low-cardinality events | Immediately under unique tx/package floods |

## Security Mistakes

| Mistake | Risk | Prevention |
| --- | --- | --- |
| Run expensive validation before count/weight/topology checks | CPU and memory DoS | Enforce 25 transaction/404,000 weight, duplicate, order, and conflict bounds before contextual/script checks |
| Select arbitrary orphan children across peers | Child-crowding censorship and misleading blame/origin correlation | Preserve announcer sets and prefer bounded same-peer 1p1c evidence as the pinned path does |
| Treat reconsiderable failure as a permanent hard reject | Suppresses legitimate CPFP packages | Separate hard recent rejects from reconsiderable transaction and package hashes |
| Leave package/retry state unbounded | Long-lived memory DoS | Per-peer/global caps, TTLs or rotations, bounded work per tick, and pressure counters |
| Whole-mempool periodic rebroadcast | Origin/timing leakage, fingerprinting, bandwidth amplification | Retry only the bounded local unbroadcast set with fresh cycle jitter and normal peer eligibility |
| Leak identifiers in shared observability | Transaction-origin and peer correlation, credential/endpoint disclosure | Aggregate allowlisted fields; forbidden-key/value serialization tests and support-bundle redaction |
| Trust durable derived state | Crafted/corrupt snapshot can violate graph or peer invariants | Validate transaction identity and metadata, rebuild topology/indexes, reject or repair typed corruption |
| Broaden relay defaults while adding package code | Exposes unfinished abuse/resource surface publicly | Preserve explicit activation and deterministic no-claim checks throughout the milestone |

## UX Pitfalls

| Pitfall | User Impact | Better Approach |
| --- | --- | --- |
| Call both fee values “minimum relay fee” | Operators cannot explain why a transaction is rejected or when pressure will clear | Show configured `minrelaytxfee`, effective `mempoolminfee`, occupancy band, block-gated decay state, and stable rejection class separately |
| Report one package success/failure | Callers cannot know which members remain in mempool or were broadcast | Return package-wide state plus ordered per-wtxid final outcomes and effective-fee membership |
| Describe initial retry as periodic mempool rebroadcast | Users infer stronger propagation and worse privacy behavior | Use “initial broadcast retry” and state local-origin, bounded, best-effort, and acknowledgement semantics |
| Show vsize as memory usage | Capacity and eviction appear inconsistent | Expose both transaction bytes/vsize and accounted usage/cap with definitions |
| Say “announced” for eligibility or queueing | Operators think traffic reached a peer when it did not | Separate eligible, queued, attempted, emitted, requested, served, and suppressed counters |
| Hide checkpoint freshness | Operators assume the latest in-memory mempool will survive a crash | Show last successful checkpoint, pending/dirty state, persist strength, and documented crash-loss window |
| Unqualified “package relay supported” | Implies general wire interoperability or public defaults | Name local package admission and opportunistic P2P 1p1c explicitly; list deferred general/public surfaces |

## "Looks Done But Isn't" Checklist

- [ ] **Package admission:** A valid CPFP demo passes, but partial individual-first semantics, same-txid/different-witness handling, package RBF/TRUC scope, and post-trim result rewriting are not verified.
- [ ] **Atomicity:** Rejection leaves the mempool unchanged in a simple case, but mid-package trimming/replacement and cross-cache rollback hazards are not exercised.
- [ ] **Topology:** Parent and child enter the pool, but a recomputation oracle has not checked spent outpoints, adjacency, aggregates, descendant score, and removal closure after randomized sequences.
- [ ] **Pressure:** The vsize cap holds, but accounted memory, descendant-package victim choice, rolling bump value, and complete lifecycle cleanup are absent.
- [ ] **Decay:** A half-life test passes, but no-block gating, occupancy thresholds, strict 10-second update, rounding, zero threshold, and backward time are missing.
- [ ] **Persistence:** Snapshot encode/decode passes, but the daemon does not checkpoint, acceptance time is lost, replay is txid-ordered, unbroadcast membership is absent, or global schema compatibility is untested.
- [ ] **Rebroadcast:** A deferred/due counter increments, but no receive-independent scheduler, fresh jitter, bounded fanout, serve acknowledgement, lifecycle clearing, or restart restoration exists.
- [ ] **Package relay:** Local RPC submission works, but opportunistic sender-aware P2P 1p1c assembly and ordinary topological transaction fanout are not wired.
- [ ] **Authority:** Every component has the right data in unit tests, but no integration test proves RPC, timer, persistence, status, and live transport use the same `ManagedNetworkHandle`.
- [ ] **Observability:** Status fields serialize, but values are eligibility proxies, labels accept dynamic strings, support bundles leak identifiers, or successful emission is not receipt-backed.
- [ ] **Long-lived behavior:** Short tests pass, but virtual-time days of fill/evict/block/decay/expire/retry/restart cycles are not bounded and deterministic.
- [ ] **Release boundary:** “Package relay deferred” text was removed, but no exact bounded replacement claim or negative claim fixtures were added.

## Recovery Strategies

| Pitfall | Recovery Cost | Recovery Steps |
| --- | --- | --- |
| Partial or inconsistent package mutation | HIGH | Stop relay effects, capture sanitized invariant evidence, rebuild mempool from canonical transactions through current policy, rebuild all indexes/caches, and add the reproducing package as a staged-admission regression fixture |
| Graph/index drift after eviction | HIGH | Disable further admission, recompute topology and removal closure from canonical entries, purge orphaned cache records, then replace split mutations with one lifecycle delta |
| Incorrect rolling floor | MEDIUM | Reset volatile rolling state to the documented restart baseline, preserve transactions, replay the exact eviction/block/time sequence in a differential test, and correct operator evidence without persisting the bad value |
| Invalid or stale snapshot schema | MEDIUM/HIGH | Preserve the original snapshot, classify corruption versus incompatibility, migrate only source records, topologically replay into a fresh authority, and never deserialize stale derived indexes |
| Lost unbroadcast markers | MEDIUM | Reconstruct only from auditable local-submission/checkpoint evidence when available; otherwise report unknown/degraded delivery state rather than relabeling the whole mempool local |
| Whole-mempool or overbroad rebroadcast | HIGH privacy cost | Disable scheduler, clear volatile due queues, preserve mempool transactions, rotate only bounded internal scheduling state, audit logs/support evidence for leakage, and add origin/fanout regression tests |
| Split authority | HIGH | Choose the existing `ManagedNetworkHandle` as source of truth, stop secondary writers, rebuild projections and queues from it, add provenance tests, then remove duplicate state owners |
| Cardinality or redaction leak | MEDIUM/HIGH | Stop affected export, purge or rotate retained artifacts per policy, replace dynamic fields with fixed enums/counts, and add hostile serialization fixtures before re-enabling |
| Claim creep | MEDIUM | Correct public/operator artifacts, restore explicit activation/non-claims, add checker fixtures for the exact overclaim, and require fresh evidence before promoting any deferred surface |

## Pitfall-to-Phase Mapping

| Pitfall | Prevention Phase | Verification |
| --- | --- | --- |
| Static/rolling fee confusion and clock drift | 1. Resource and fee primitives; 2. Rolling fee, expiry, and eviction core | Differential integer fee fixtures for block gate, time gate, occupancy bands, rounding, floor, and zero transition |
| Virtual-size-only pressure and incomplete eviction | 1–2, then 5. Cross-cache lifecycle integration | Accounted-usage fixtures; descendant-score removal oracle; every removed member absent from every cache; repeated pressure benchmark |
| Wrong package atomicity and partial results | 3. Package vocabulary and staged admission | Individual-first partial result, subpackage no-partial-mutation, mid-package eviction/replacement, post-trim membership/result tests |
| Topology/index drift | 3, then 5 | Context-free boundary fixtures plus randomized recomputation oracle after add/replace/evict/expire/block/reorg |
| Overbroad or unsafe P2P package assembly | 4. Package-aware download/orphan bridge | Sender-aware same-peer 1p1c, multiple-reconsiderable-parent suppression, bounded reject hash and attempt tests |
| Lifecycle/cache divergence | 5. Cross-cache lifecycle integration | One delta removes all descendants from mempool, serving, fanout, peer, compact, orphan/package, unbroadcast, persistence, and evidence state |
| Persistence and recovery mismatch | 6. Snapshot schema and recovery | Legacy-schema fixtures, topological replay, time/expiry, unbroadcast survivor, rolling reset, corruption, failed-write, clean-shutdown, and crash-window tests |
| Privacy/fanout errors in initial retry | 7. Receive-independent maintenance and transport | Fresh injected jitter, zero-peer/reconnect, queue cap, relay-disabled, no-getdata retry, eligible-serve clearing, write failure, and restart tests |
| Duplicate authority | 5–8, enforced in 9 | RPC/timer/persistence/metrics/transport handle provenance test and deterministic source checker forbidding secondary mempool ownership |
| Cardinality/redaction and premature success | 8. RPC and operator evidence; recheck in 9 | Fixed metric-kind inventory, forbidden identifier tests, direct-RPC versus shared-status separation, queued/attempted/emitted/served lineage |
| Claim creep and nondeterministic verification | 9. Parity, adversarial pressure, restart, and release guardrails | Positive exact-claim roots; negative protocol/public/default/production fixtures; verify script contains no live-network or wall-clock dependency |

## Roadmap Ordering Implications

The highest-risk ordering mistake is building package RPC or peer transport before the pressure model. Package admission must compare against the real dynamic floor and survive final descendant-score trimming, so resource/fee primitives and the rolling/eviction core need to land first. Staged package admission then becomes the shared engine for RPC and P2P.

Cross-cache lifecycle integration must precede both persistence and rebroadcast transport. Otherwise recovered or retried transactions can enter serving, fanout, compact, or unbroadcast state without the same removal guarantees as live admission. Operator evidence should be last among implementation phases because accurate queued/emitted/served distinctions depend on the final authority and transport receipt path.

The closeout phase should include virtual-time long-run scenarios rather than only unit vectors: repeated package bursts, pressure eviction, block-gated decay, expiry, reconnect, retry, checkpoint failure, restart replay, and reorg. That phase is also where the narrow claim becomes enforceable: local child-with-unconfirmed-parents admission, opportunistic P2P 1p1c, ordinary transaction fanout, and local initial-broadcast retry—nothing broader.

## Research Flags

- **Package RBF and TRUC scope:** The pinned baseline contains limited 1p1c package RBF, feerate-diagram checks, TRUC inheritance/topology rules, and ephemeral-dust policy. Requirements must explicitly include them or narrow package-admission parity with typed unsupported/deferred results. **Confidence: HIGH that the baseline contains them; scope decision pending.**
- **Rust dynamic-memory accounting tolerance:** Knots' `DynamicMemoryUsage()` estimates C++ container allocation. Rust cannot match internal byte-for-byte usage mechanically. Define an owned accounting contract and the observable parity/tolerance before claiming pressure equivalence. **Confidence: MEDIUM pending design and benchmark evidence.**
- **Crash durability guarantee:** The correct stored fields are clear, but checkpoint cadence and allowable sudden-crash loss are milestone product decisions. Distinguish periodic, clean-shutdown, and synchronous mutation durability. **Confidence: MEDIUM pending requirements.**
- **Unbroadcast completion receipt:** The pinned code clears after enqueueing the requested transaction into its send path, while v2.1 Open Bitcoin evidence distinguishes successful transport effects. Choose and document the exact Open Bitcoin serve/write boundary, but never clear on `inv` alone. **Confidence: HIGH on the prohibited early boundary; MEDIUM on the final stronger receipt choice.**

## Sources

All decisive behavior claims were verified against local primary sources. The pinned submodule resolves to Bitcoin Knots `v29.3.knots20260210`, commit `a9aee730466ac67d35a3c03ee24676be5e045878`.

### Pinned Bitcoin Knots

- `packages/bitcoin-knots/doc/policy/packages.md` — package topology, individual-first acceptance, fee-floor separation, DoS rationale, limited package RBF, and deduplication. **HIGH confidence.**
- `packages/bitcoin-knots/src/policy/packages.h` and `packages.cpp` — 25 transaction/404,000 weight bounds, topological order, duplicate/input-conflict checks, child-with-parents shape, and package hash. **HIGH confidence.**
- `packages/bitcoin-knots/src/validation.cpp` (`AcceptMultipleTransactions`, `AcceptSubPackage`, `AcceptPackage`, `ProcessNewPackage`) — staged subpackage acceptance, temporary coin cleanup, partial results, package feerate, final trim, and post-trim result rewriting. **HIGH confidence.**
- `packages/bitcoin-knots/src/txmempool.h` and `txmempool.cpp` (`DynamicMemoryUsage`, `GetMinFee`, `trackPackageRemoved`, `TrimToSize`, `Expire`, `removeForBlock`, unbroadcast methods) — pressure accounting, descendant eviction, rolling bump/decay, block gate, expiry, and lifecycle cleanup. **HIGH confidence.**
- `packages/bitcoin-knots/src/node/txdownloadman.h`, `txdownloadman_impl.cpp`, and `src/net_processing.cpp` — sender-aware opportunistic 1p1c construction, reconsiderable reject handling, package-result processing, ordinary transaction fanout, and randomized initial retry. **HIGH confidence.**
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` and `src/node/transaction.cpp` — persisted entry time/fee/unbroadcast fields, local-origin insertion, and absence of serialized rolling-fee fields. **HIGH confidence.**
- `packages/bitcoin-knots/src/test/mempool_tests.cpp`, `src/test/txpackage_tests.cpp`, `test/functional/mempool_limit.py`, `mempool_unbroadcast.py`, `mempool_persist.py`, and `p2p_opportunistic_1p1c.py` — boundary, mid-package eviction, decay, restart, retry, and P2P regression cases. **HIGH confidence.**

### Current Open Bitcoin

- `packages/open-bitcoin-mempool/src/pool.rs`, `pool/lifecycle.rs`, `pool/topology.rs`, and `types.rs` — current single-transaction clone/recompute admission, vsize-only trim, topology reconstruction, and explicitly deferred rolling-fee state. **HIGH confidence.**
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs`, `scheduler.rs`, and `orphanage.rs` — existing bounded per-peer queues, injected timestamps, origin/eligibility suppression, deferred rebroadcast seam, and orphan reconsideration limits. **HIGH confidence.**
- `packages/open-bitcoin-node/src/network/runtime_authority.rs`, `admission_bridge.rs`, `relay_fanout.rs`, `mempool_lifecycle.rs`, and `recovery.rs` — single authoritative handle and current cross-cache admission/removal/recovery seams. **HIGH confidence.**
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`, `storage/snapshot_codec.rs`, `storage/fjall_store/mempool.rs`, and `storage.rs` — txid-sorted recovery, current stored fields, global schema version, and load/save adapters. **HIGH confidence.**
- `packages/open-bitcoin-node/src/status/relay_evidence.rs`, `metrics.rs`, and `logging.rs` — fixed counters, deferred rebroadcast status, low-cardinality projection, and current redaction patterns. **HIGH confidence.**
- `.planning/milestones/v2.0-REQUIREMENTS.md`, `.planning/milestones/v2.1-REQUIREMENTS.md`, and `.planning/PROJECT.md` — activation, authoritative transport/observability, deterministic verification, and scoped public/default/production claim boundaries. **HIGH confidence.**

### Local Standards Applied

- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/testing.md`, `standards/core/verification.md`, and `standards/languages/rust.md` — functional-core/imperative-shell boundary, typed invariants, pure policy tests, repository verification, and Rust module/testing expectations. **HIGH confidence.**

***

*Pitfalls research for: Open Bitcoin v2.2 Package Relay and Long-Lived Mempool Policy*
*Researched: 2026-07-22*
