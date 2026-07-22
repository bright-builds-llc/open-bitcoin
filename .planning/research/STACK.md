# Stack Research

**Domain:** Bitcoin package admission/relay and long-lived mempool policy
**Researched:** 2026-07-22
**Confidence:** HIGH

## Recommendation

v2.2 should add **no new external production dependencies**. The required capabilities fit the existing Rust workspace, first-party Bitcoin primitives, bounded transaction-relay state machines, Fjall-backed snapshot store, and Tokio daemon shell.

The milestone is primarily an internal policy and state-model expansion:

- Extend `open-bitcoin-mempool` with typed package validation/admission, rolling-minimum-fee state, entry-time accounting, and pressure/eviction transitions.
- Extend `open-bitcoin-network` with bounded 1-parent/1-child package-candidate assembly and topological fanout decisions, while continuing to relay ordinary `tx`/`wtx` inventories.
- Extend `open-bitcoin-node` with atomic orchestration, recovery metadata, peer-policy cleanup, and observable outcomes.
- Use the existing Tokio runtime in `open-bitcoin-rpc` only as the timer-driving imperative shell for periodic maintenance. Keep clocks and randomness out of the pure crates by passing timestamps and sampled jitter into typed transition functions.

This is the lowest-risk stack because the current code already contains the needed seams: package topology breadcrumbs in the mempool crate, descendant-score eviction, a bounded per-peer fanout queue, caller-injected timestamps, durable mempool snapshots, authoritative network state, and an explicit `RebroadcastDeferred` action.

## Recommended Stack

### Core Technologies

| Technology | Version | Purpose | Why Recommended |
| --- | --- | --- | --- |
| Rust | `1.94.1` | Pure package/mempool policy and typed state transitions | Already pinned by `rust-toolchain.toml`; no toolchain change is needed. Rust enums/newtypes can make package topology, admission results, removal causes, and recovery states explicit. |
| Open Bitcoin workspace crates | `0.1.0`, Rust 2024 | Own package policy, relay policy, persistence boundaries, and operator projections | Preserves the first-party production-path policy and the established functional-core/imperative-shell split. |
| Bitcoin Knots | `29.3.knots20260210` | External behavior and fixture baseline | The vendored source is the authoritative contract for package shape, admission results, rolling-fee decay, eviction, initial rebroadcast, and recovery metadata. |
| Fjall | `3.1.4`, existing | Durable mempool snapshot and restart metadata | Reuse the existing versioned snapshot codec and `StorageNamespace::Mempool`; a second database or sidecar log would add recovery ordering problems without adding capability. |
| Tokio | `1.52.1`, existing in `open-bitcoin-rpc` | Daemon maintenance tick and shutdown-aware scheduling | The daemon already owns Tokio with `time` and `test-util`. Use it only to wake the shell; pass explicit `now_unix_seconds` and jitter into pure policy functions. Do not add Tokio to `open-bitcoin-mempool` or `open-bitcoin-network`. |

### First-Party Modules to Extend

| Crate / module | New responsibility | Integration point |
| --- | --- | --- |
| `open-bitcoin-mempool` package policy | `Package`, validated topological package, package-wide result, per-transaction result, package hash, count/weight/consistency checks | Build on `Transaction`, `Txid`, `Wtxid`, existing transaction validation, and first-party `Sha256`. Match Knots limits of 25 transactions and 404,000 weight units. |
| `open-bitcoin-mempool` admission | Validate against a prospective state, then commit each Knots-aligned subpackage as one state transition | Refactor the existing clone/recompute/trim flow into an explicit prepare/commit transaction so a failed subpackage cannot leave partial mutations. Preserve final per-wtxid results plus package-level status across already-present, individually accepted, and package-evaluated members. |
| `open-bitcoin-mempool` rolling fee | Pure `RollingMinimumFeeState` with last update, last pressure bump, block-since-bump state, and injected time | Feed the effective fee into both single-transaction and package feerate checks. Mirror Knots' 12-hour half-life, faster decay below one-half and one-quarter capacity, 10-second update gate, incremental-relay floor, and zeroing threshold. |
| `open-bitcoin-mempool` pressure index | Canonical `HashMap<Txid, MempoolEntry>` plus standard-library ordered keys/sets for deterministic package selection | Replace repeated whole-map victim scans as scale requires. Key eviction by descendant package score with txid tie-breaking, and remove the selected transaction plus descendants as one lifecycle outcome. |
| `open-bitcoin-network` package candidate state | Bounded 1-parent/1-child candidate assembly from reconsiderable parents and orphan children | Extend the existing orphanage/download manager. The pinned P2P baseline currently forms 1P1C packages from ordinary `tx` receipt; it does not negotiate a new wire protocol. |
| `open-bitcoin-network` fanout | Queue accepted package members in topological order through the existing `TxFanoutQueue` | Preserve txid/wtxid negotiation, per-peer eligibility, origin suppression, queue caps, rate limiting, cleanup, and ordinary `inv` transport. |
| `open-bitcoin-node` recovery | Extend `MempoolSnapshot` and its versioned DTO with entry acceptance time and unbroadcast membership | Reuse `FjallNodeStore::save_mempool_snapshot` / `load_mempool_snapshot`; replay parents before children and emit a status for every recovered or dropped member. |
| `open-bitcoin-node` runtime authority | Apply package admission, pressure removal, relay cleanup, serving cleanup, metrics, logs, and durable writes under one authoritative mutation boundary | Reuse `ManagedNetworkHandle` and `ManagedPeerNetwork`; do not create a second mempool or relay authority in RPC/background tasks. |
| `open-bitcoin-rpc` daemon shell | Drive periodic mempool maintenance and initial-rebroadcast retries | Use a bounded Tokio task or existing daemon loop, shutdown-aware waits, and explicit ticks. The shell samples time/jitter and invokes authority methods; it does not own policy decisions. |
| `open-bitcoin-bench` | Deterministic package admission and sustained-pressure benchmarks | Extend the existing custom benchmark harness; do not add Criterion solely for this milestone. |

### Supporting Libraries and Standard-Library Approaches

| Library / facility | Version | Purpose | When to Use |
| --- | --- | --- | --- |
| `std::collections::{HashMap, BTreeMap, BTreeSet, VecDeque}` | Rust `1.94.1` | Canonical entry lookup, deterministic result ordering, topology sets, bounded fanout queues, and maintenance work queues | Use throughout the pure core. Prefer deterministic ordered outputs at evidence and serialization boundaries. |
| `std::time::{Duration, SystemTime, UNIX_EPOCH}` | Rust `1.94.1` | Shell time acquisition and interval representation | Acquire wall time only in daemon/adapters. Pure policy APIs receive integer timestamps or typed durations. |
| `f64::powf` plus explicit Knots-compatible rounding | Rust `1.94.1` | Exponential rolling-fee decay matching Knots' `double`/`pow` formula | Keep it inside the rolling-fee module, expose only integer sat/kvB `FeeRate`, and lock boundary behavior with differential fixtures. Do not spread floating-point fee values through the domain model. |
| `getrandom` | `0.3.4`, existing in `open-bitcoin-node` | Sample privacy-preserving rebroadcast jitter | Sample in the node shell and pass a bounded delay to pure scheduling logic. If randomness fails, return a typed degraded scheduling outcome rather than silently using a fingerprintable constant. |
| `open_bitcoin_consensus::crypto::Sha256` | first-party workspace `0.1.0` | Knots-compatible package hash for reconsiderable-package rejection tracking | Hash sorted little-endian wtxids exactly as the pinned baseline specifies; do not add a general hashing crate. |
| `serde` / `serde_json` | `1.0.228` / `1.0.149`, existing | Versioned durable DTOs and stable operator/report shapes | Extend the existing node snapshot codec and projections. Do not serialize internal maps directly as the persistence contract. |
| Tokio time/test utilities | Tokio `1.52.1`, existing | Runtime wakeups and paused-time integration tests | Use only in the RPC/daemon shell. Unit-test core scheduling with explicit timestamps and no sleeps. |

### Development Tools

| Tool | Purpose | Notes |
| --- | --- | --- |
| `bash scripts/verify.sh` | Repo-native format, lint, build, test, coverage, architecture, parity-breadcrumb, benchmark, and Bazel contract | Remains the deterministic default gate. Public-network scenarios stay opt-in. |
| Existing Rust unit/property tests | Package topology, aggregate feerate, atomicity, rolling decay, eviction, restart, and scheduling fixtures | Use injected clocks and deterministic jitter values. Compare stable labels and integer fee outcomes to pinned Knots fixtures. |
| Existing `open-bitcoin-bench` harness | Detect topology recomputation and pressure-path regressions | Add workloads for 25-member packages, long chains, wide descendant sets, repeated trim/refill cycles, and rebroadcast-set scans. |
| Pinned Knots functional/unit tests | Source of parity cases | Mine cases from `txpackage_tests.cpp`, `mempool_tests.cpp`, `txdownload_tests.cpp`, `mempool_packages.py`, `p2p_opportunistic_1p1c.py`, `mempool_unbroadcast.py`, and `mempool_persist.py`. Only claim behavior represented by the pinned tree. |

## Detailed Stack Decisions

### Package Admission and Relay

Implement package semantics as first-party domain types, not `Vec<Transaction>` passed unchecked across layers. A fallible constructor should enforce non-empty, maximum count, maximum total weight, unique txids, topological parent-before-child order, and no cross-package input conflicts. A stricter child-with-unconfirmed-parents type should represent the topology accepted by the pinned `AcceptPackage` path.

The mempool should evaluate each submission subpackage against one prospective state and commit that subpackage only after context-free checks, input/consensus checks, package feerate checks, ancestor/descendant checks, and policy-script checks succeed. The higher-level package flow may reuse already-present members or preserve individually accepted members exactly as Knots does, so atomicity must be scoped to the matching subpackage commit rather than asserted across unlike result classes. Results need both a package-level status and final per-wtxid outcomes because Knots can report already-present members, invalid members, replaced transactions, and members whose status changes after size limiting.

For P2P, keep the first milestone's transport scope to the pinned behavior: assemble a bounded 1-parent/1-child candidate when a reconsiderable parent and orphan child meet, submit it to package admission, then announce accepted members through the existing per-peer `inv` path. A repository-wide search of the pinned tree found no `sendpackages`, `ancpkginfo`, `getpkgtxns`, or `pkgtxns` commands. Adding those would be a new protocol claim, not implementation of this pin.

### Rolling Minimum Fee and Sustained Pressure

The current `Mempool` already evicts the lowest descendant-score transaction and its descendants, but capacity is expressed as aggregate virtual size and selection scans/recomputes the complete map. Knots trims against estimated dynamic memory usage and bumps the rolling floor to the removed descendant package feerate plus the incremental relay feerate. v2.2 therefore needs an owned, deterministic memory-accounting model and an explicit pressure state transition; simply exposing the existing `max_mempool_virtual_size` result as Knots parity would be too broad.

Keep the rolling fee as a small pure state machine. The state transition should take current time, accounted usage, configured byte limit, incremental relay feerate, and whether a block arrived after the last bump. It should return the new state and rounded effective `FeeRate`. Using `f64` locally is justified because the pinned implementation uses `double` and `pow`; the externally visible value remains integer sat/kvB. Add differential tests at the 10-second gate, 12-hour half-life, one-half/one-quarter occupancy changes, incremental floor, and zero threshold. Consider a narrowly scoped deterministic math implementation only if cross-platform verification demonstrates a real rounding mismatch; do not preemptively add `libm`.

Use a canonical entry map plus explicit adjacency and ordered eviction keys. Standard-library collections are sufficient. Keep index updates inside the same prospective-state transaction as package admission/removal, and assert/recompute indexes in tests. This avoids adding a graph crate while preventing sustained-pressure work from degenerating into repeated full-map scans.

### Rebroadcast and Restart Boundaries

Activate the existing `RebroadcastDeferred` seam as Knots-style **initial broadcast retry**, separate from wallet-wide periodic resubmission. Locally submitted transactions with relay enabled enter a bounded unbroadcast set. A shell timer wakes every randomized 10–15 minutes, asks the pure core for eligible retry actions, and routes those actions through the same peer eligibility and bounded fanout queue as first announcements. Confirmation, replacement, eviction, expiry, or evidence that a peer announced the transaction removes it from the set.

Persist the unbroadcast set with the mempool snapshot, because the pinned mempool format persists and optionally restores it. Persist entry acceptance times as well so expiry and ordered recovery do not reset transaction age. Do **not** persist `rollingMinimumFeeRate` by default: the inspected Knots mempool dump stores transaction time, fee deltas, and unbroadcast txids but not the rolling fee fields. On restart, replay, limit enforcement, and current pressure derive the new in-memory policy state. Any decision to persist additional rolling-fee state must be recorded as an intentional Knots behavior difference.

The wallet's separate 12–24 hour randomized resubmission policy is not required to implement the node mempool's 10–15 minute initial-broadcast retry. Do not couple v2.2 node relay correctness to wallet scheduling unless milestone requirements explicitly activate that separate behavior.

### Observability and Verification

Extend existing status, metrics, structured logs, support evidence, RPC, and CLI projections with stable counts and reasons: package accepted/rejected/member results, current effective mempool minimum fee, last pressure bump, decay state, evicted package/member counts, unbroadcast count, retry due/attempted/queued/suppressed counts, recovery restored/dropped counts, and bounded-resource state.

Default tests remain synthetic and deterministic: injected timestamps, fixed jitter inputs, in-memory authoritative state, temporary Fjall stores, and fixture peer sets. Public-network review remains opt-in and must not become a prerequisite for `bash scripts/verify.sh`.

## Installation

No Cargo packages or system services should be added for v2.2.

```bash
# Materialize the pinned behavioral baseline if needed.
git submodule update --init --recursive

# Verify the unchanged dependency/toolchain contract and all first-party work.
bash scripts/verify.sh
```

If implementation introduces a dependency despite this recommendation, require a written capability gap, maintenance/security review, Cargo and Bazel wiring, and evidence that a small first-party or standard-library implementation is less safe.

## Alternatives Considered

| Recommended | Alternative | When to Use Alternative |
| --- | --- | --- |
| Standard-library collections and owned topology indexes | `petgraph`, `indexmap`, or a priority-queue crate | Only if profiling proves the owned index cannot meet sustained-pressure bounds and the selected crate passes dependency/security review. The current domain needs a constrained DAG, not a general graph API. |
| Existing Fjall mempool namespace and versioned snapshot DTO | SQLite, RocksDB, or a second append-only policy store | Only if a future milestone requires independent transactional history/query semantics that Fjall cannot provide. v2.2 needs one restart snapshot boundary. |
| Existing Tokio daemon shell | A new scheduler/background-job framework | Only if scheduling expands beyond the daemon runtime and needs durable distributed jobs, which is out of scope. |
| Local `f64` decay matching Knots, rounded immediately | `libm` or a decimal/fixed-point crate | Use only after a reproduced cross-platform parity failure shows the standard implementation cannot preserve integer outputs. |
| Bounded rotating sets / owned filter state | Bloom-filter or cache crate | Use an external crate only if memory/false-positive targets cannot be met cleanly by the existing first-party hashing and bounded generations. |
| Ordinary tx/wtx inventory relay of accepted package members | BIP331-style package-relay wire commands | Use only in a separately researched milestone whose pinned baseline and requirements explicitly include those messages. |

## What NOT to Use

| Avoid | Why | Use Instead |
| --- | --- | --- |
| Third-party Rust Bitcoin domain libraries | Violates the production-path ownership policy and obscures parity differences | Existing first-party primitives, codec, consensus, mempool, and network crates |
| New package-relay P2P commands | Not present in the inspected pinned baseline and would broaden interoperability claims | Knots-aligned 1P1C candidate assembly over existing transaction/orphan flow and ordinary inventory relay |
| Tokio, wall-clock reads, or randomness in pure crates | Makes policy tests timing-dependent and breaks functional-core boundaries | Inject integer time and sampled bounded jitter from the daemon/node shell |
| A second mempool database or rebroadcast journal | Creates dual-authority and crash-ordering ambiguity | Extend the existing versioned Fjall mempool snapshot atomically |
| Persisting the rolling fee as if Knots did so | The inspected baseline dump does not store rolling-fee fields; silent persistence changes restart behavior | Persist entry time and unbroadcast metadata; explicitly derive/reset rolling state at recovery |
| Unbounded `HashSet`/`BTreeSet` histories for package rejects or rebroadcast evidence | Leaks memory during long-lived hostile traffic | Capacity-limited, generation-rotated state with typed eviction evidence |
| Sleeping tests or live network in default verification | Produces slow/flaky gates and violates the deterministic-default boundary | Fake clocks, fixed jitter, temporary stores, fixture peers, and opt-in public-network UAT |
| Virtual-size-only pressure accounting presented as full Knots parity | Knots' configured mempool limit is enforced against dynamic memory usage | First-party accounted-memory model plus explicit documented approximation bounds |

## Stack Patterns by Variant

**For deterministic default verification:**

- Use injected timestamps and deterministic jitter values.
- Use in-memory mempool/network state and temporary Fjall stores.
- Assert integer fee outcomes, stable labels, topological ordering, queue bounds, and restart metadata.

**For opt-in public-network review:**

- Use the same authoritative network state and production peer transport already validated in v2.1.
- Enable relay explicitly; do not introduce a public/default relay mode.
- Capture sanitized package, pressure, fee, and rebroadcast evidence without peer addresses or transaction-origin claims.

**For peer-originated package candidates:**

- Assemble only the pinned 1P1C topology from reconsiderable/orphan state.
- Attribute each transaction's sender, bound candidate retries, and cache rejected package hashes with bounded state.
- Relay accepted members through ordinary txid/wtxid fanout.

**For local/RPC package submission:**

- Reuse the general child-with-unconfirmed-parents package type and atomic admission path.
- Track locally originated accepted members in the unbroadcast set only when relay is requested.
- Keep fee guardrails and per-transaction result reporting at the RPC boundary.

**For restart/recovery:**

- Decode a versioned snapshot, validate metadata, replay parents before children, and emit a result for every record.
- Restore unbroadcast membership only for transactions successfully recovered into the mempool.
- Rebuild volatile indexes and rolling-fee state; do not trust serialized derived topology.

## Version Compatibility

| Package A | Compatible With | Notes |
| --- | --- | --- |
| Rust `1.94.1` / edition 2024 | All workspace crates `0.1.0` | `rust-toolchain.toml` and `packages/Cargo.toml` remain the sources of truth. |
| `open-bitcoin-mempool` `0.1.0` | `open-bitcoin-chainstate`, `codec`, `consensus`, `primitives` workspace crates | No external crate addition is required for package topology, hashing inputs, fee math, or eviction state. |
| `open-bitcoin-network` `0.1.0` | Existing first-party chainstate/codec/consensus/primitives crates | Keep package-candidate decisions pure and transport-neutral. |
| `open-bitcoin-node` `0.1.0` | Fjall `3.1.4`, getrandom `0.3.4`, serde `1.0.228`, serde_json `1.0.149` | Extend the current storage and authority boundaries; bump the snapshot schema version when durable fields change. |
| `open-bitcoin-rpc` `0.1.0` | Tokio `1.52.1` with `time` and `test-util` | Sufficient for maintenance wakeups and paused-time tests; no new timer framework is needed. |
| Bitcoin Knots `29.3.knots20260210` | v2.2 parity fixtures | Do not mix defaults or protocol behavior from newer Core/Knots releases into this milestone. |

## Sources

All decisive findings were verified from local primary sources; no training-data-only version claims were used.

- `rust-toolchain.toml` and `packages/Cargo.toml` — Rust `1.94.1`, Rust 2024, and workspace `0.1.0` sources of truth. **HIGH confidence.**
- `packages/open-bitcoin-mempool/Cargo.toml`, `packages/open-bitcoin-network/Cargo.toml`, `packages/open-bitcoin-node/Cargo.toml`, and `packages/open-bitcoin-rpc/Cargo.toml` — current dependency surfaces and exact external versions. **HIGH confidence.**
- `packages/open-bitcoin-mempool/src/pool.rs` and `pool/lifecycle.rs` — existing prospective admission, descendant-score trimming, pressure summary, and explicit deferred rolling-fee seam. **HIGH confidence.**
- `packages/open-bitcoin-network/src/peer/transaction_relay/fanout.rs` and `scheduler.rs` — bounded queues, injected timestamps, peer eligibility, cleanup, and `RebroadcastDeferred`. **HIGH confidence.**
- `packages/open-bitcoin-node/src/storage/mempool_snapshot.rs`, `storage/snapshot_codec.rs`, and `storage/fjall_store/mempool.rs` — current durable snapshot/replay boundary. **HIGH confidence.**
- `packages/open-bitcoin-node/src/network/runtime_authority.rs` and `relay_fanout.rs` — authoritative runtime mutation boundary and current relay integration. **HIGH confidence.**
- `packages/bitcoin-knots/src/policy/packages.h` and `policy/packages.cpp` — package limits, topology, consistency, and package-hash contract. **HIGH confidence.**
- `packages/bitcoin-knots/src/validation.cpp` (`AcceptMultipleTransactions`, `AcceptPackage`, `ProcessNewPackage`) — package validation order, aggregate feerate, topology scope, atomic submission, and result semantics. **HIGH confidence.**
- `packages/bitcoin-knots/src/node/txdownloadman.h`, `node/txdownloadman_impl.cpp`, and `net_processing.cpp` (`ProcessPackageResult`) — P2P 1P1C candidate assembly and relay-result handling. **HIGH confidence.**
- `packages/bitcoin-knots/src/txmempool.cpp` (`GetMinFee`, `trackPackageRemoved`, `TrimToSize`, `Expire`) and `txmempool.h` — rolling-fee formula/state, descendant-package eviction, and expiry behavior. **HIGH confidence.**
- `packages/bitcoin-knots/src/net_processing.cpp` (`ReattemptInitialBroadcast`) and `node/transaction.cpp` — locally submitted unbroadcast tracking and randomized 10–15 minute retry. **HIGH confidence.**
- `packages/bitcoin-knots/src/node/mempool_persist.cpp` — persisted entry time, fee metadata, and unbroadcast set; absence of rolling-fee fields in the inspected dump/load format. **HIGH confidence.**
- `AGENTS.md`, `AGENTS.bright-builds.md`, `standards/core/architecture.md`, `standards/core/verification.md`, `standards/core/testing.md`, and `standards/languages/rust.md` — local functional-core, dependency, verification, and Rust design constraints that materially shaped this recommendation. **HIGH confidence.**

***

*Stack research for: Open Bitcoin v2.2 Package Relay and Long-Lived Mempool Policy*
*Researched: 2026-07-22*
