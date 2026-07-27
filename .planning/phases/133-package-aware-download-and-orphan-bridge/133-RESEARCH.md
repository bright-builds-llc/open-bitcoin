# Phase 133: Package-Aware Download and Orphan Bridge - Research

**Researched:** 2026-07-26
**Domain:** Bounded peer transaction download, orphan provenance, opportunistic same-peer 1P1C assembly, and node-owned package admission
**Confidence:** HIGH

<user-constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

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

### Deferred Ideas (OUT OF SCOPE)

- Applying every package admission/removal delta to serving, fanout, peer known/request state, compact reconstruction, unbroadcast state, persistence dirtiness, and operator evidence — Phase 134.
- Parent-before-child ordinary transaction fanout and transport receipts — Phase 136.
- RPC, CLI, dashboard, metrics, logs, and support-bundle package evidence — Phase 137.
- General package wire messages, BIP331, arbitrary multi-parent peer reconstruction, and cluster mempool — beyond v2.2.

</user-constraints>

<phase-requirements>
## Phase Requirements

| ID                    | Description                                                                                                                                                                | Research Support                                                                                                                                                                                                                                                                                                                                                                                                          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PPKG-01               | Node distinguishes hard rejects from reconsiderable package candidates and retains only bounded, rotating candidate and reject evidence.                                   | Use two node-global generational filters with typed key domains, exact locked capacity/FPR, active-tip resets, and bodies retained only in the existing bounded orphanage. [VERIFIED: `.planning/REQUIREMENTS.md:45-48`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:27-37`]                                                                                                          |
| PPKG-02               | Node assembles only sender-aware same-peer one-parent/one-child candidates over ordinary transaction messages, preserving member origin and exact pinned package identity. | Extend orphan records from single owner to bounded announcer provenance, add a newest-first parent-spend index, emit an opaque same-peer eligibility proof, and refine `[parent, child]` once through Phase 132 package types. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:308-331`; `packages/bitcoin-knots/src/txorphanage.cpp:277-315`; `packages/open-bitcoin-mempool/src/package.rs:38-68`] |
| PPKG-03               | Peer-originated package candidates reuse the authoritative package admission engine rather than reimplementing package policy in the network or RPC layers.                | Add a `ManagedMempool` package adapter and make `admission_bridge.rs` the sole candidate-to-`SubmitPackageCommand` refinement and `Mempool::submit_package` call site. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:50-110`; `packages/open-bitcoin-mempool/src/pool/package_admission.rs:67-120`]                                                                                            |

</phase-requirements>

## Summary

Phase 133 should extend the Phase 102 transaction-download/orphan bridge, not create a parallel package subsystem. The current orphanage already bounds bodies by global count, per-peer count, TTL, and per-parent reconsideration work, but it stores one `peer_id`, scans all orphans by wtxid order, and removes the whole body on that peer's disconnect. The download scheduler already retains per-identity announcement maps while requests are pending, but erases them when the transaction arrives. Those are the exact seams to change: capture announcer provenance before scheduler cleanup, store one orphan body with a bounded announcer set, index spenders newest-first by parent txid, and make same-peer eligibility a constructed type rather than a boolean. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:22-25,143-158,211-260,313-335`; `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:68-87,246-277,451-454`]

The reject-state change is independent and should land first. `PeerManager` currently owns an unbounded `BTreeSet<TxRelayId>` and clones it into every scheduler input; all rejected, duplicate, and evicted singleton outcomes flow through the same helper. Replace that set with two fixed-memory filters and expose narrow membership/recording methods so hard, reconsiderable-transaction, and failed-package evidence cannot be confused. Knots uses two 120,000-entry, 0.000001 rolling Bloom filters and resets both on `ActiveTipChange`; its three-generation representation retains between `N` and `1.5N` recent insertions in fixed storage. [VERIFIED: `packages/open-bitcoin-network/src/peer.rs:163-174,261-263`; `packages/open-bitcoin-node/src/network/admission_bridge.rs:98-106,362-376`; `packages/bitcoin-knots/src/node/txdownloadman_impl.h:63-100`; `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:97-101`; `packages/bitcoin-knots/src/common/bloom.h:83-125`]

The node shell must remain the only cross-crate composition point. A neutral network candidate should contain exactly `[parent, child]`, aligned `[peer, peer]` origins, and an opaque proof created from retained announcer state. `admission_bridge.rs` should convert it once to `WellFormedPackage`, refine it to `SubmissionPackage`, call the existing `Mempool::submit_package` once, and return the exact `PackageReport` and `MempoolLifecycleDelta`. Feedback should mutate only candidate/reject/orphan state; Phase 134 owns serving, fanout, compact, persistence, retry, and operator projections. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:70-178,187-205`; `packages/open-bitcoin-mempool/src/package/report.rs:420-511`; `packages/open-bitcoin-mempool/src/pool/package_admission.rs:93-120`; `.planning/ROADMAP.md:121-145`]

One prerequisite must be explicit in the plan: the current singleton outcome cannot distinguish a static-floor hard rejection from a rolling/package-fee reconsiderable rejection because `MempoolOutcome::Rejected` exposes only the coarse `RelayFeeTooLow` category. Do not classify that category heuristically. Prefer evaluating peer singletons through the authoritative Phase 132 report vocabulary (a one-member checked package) or extend the singleton transition with the same typed hard/reconsiderable classification. [VERIFIED: `packages/open-bitcoin-mempool/src/outcome.rs:42-117`; `packages/open-bitcoin-mempool/src/package/report.rs:215-279`; `packages/open-bitcoin-mempool/src/pool/package_admission.rs:229-349`]

**Primary recommendation:** Plan four ordered slices: (1) typed fixed-memory reject evidence and active-tip reset, (2) announcer-aware orphan provenance plus bounded newest-first 1P1C candidate selection, (3) node-owned authoritative package refinement/admission/feedback, and (4) deterministic parity, breadcrumb, documentation, and source-guard closure.

## Project Constraints (from AGENTS.md)

- Match the pinned Bitcoin Knots `29.3.knots20260210` observable behavior and keep parity evidence auditable. [VERIFIED: `AGENTS.md:57-68`]
- Keep peer selection and bounded cache decisions in the pure network core, package policy in the pure mempool core, and cross-crate composition/effects in the node shell. [VERIFIED: `AGENTS.md:61`; `standards/core/architecture.md:3-41`; `standards/languages/rust.md:158-185`]
- Add no production dependency on an existing Rust Bitcoin library; minimize dependencies and prefer the standard library. [VERIFIED: `AGENTS.md:62`; user-provided global dependency policy]
- Rust `1.94.1` and Rust 2024 are authoritative. [VERIFIED: `AGENTS.md:21-22,76-78`; `rust-toolchain.toml`]
- New or touched multi-file Rust modules should use `foo.rs` plus `foo/`, encode invariants with newtypes/enums, prefer early returns/`let...else`, and prefix internal `Option` names with `maybe_`. [VERIFIED: `standards/languages/rust.md:3-31,33-66,68-109,111-156`; `standards/core/code-shape.md:3-73`]
- Pure business logic requires focused Arrange/Act/Assert unit tests. [VERIFIED: `standards/core/testing.md:3-120`]
- New first-party Rust source or test files require parity breadcrumb registration in `docs/parity/source-breadcrumbs.json`; intentional behavior differences belong in `docs/parity/`. [VERIFIED: `AGENTS.md:31-34`; `docs/parity/source-breadcrumbs.json:638-770,911-940,1158-1174`]
- Use `bash scripts/verify.sh` as the final repository verification contract; ad-hoc Cargo/Bazel commands must run through `bun run scripts/command-timings.ts run --key <stable-key> -- <command>` and must not overlap on one target directory. [VERIFIED: `AGENTS.md:23-24,35-36`; `standards/core/verification.md:85-150`]
- Do not use standalone `---` body separators in frontmatter-parsed Markdown. [VERIFIED: user-provided global `AGENTS.md`, section 8.1]
- `docs/metrics/lines-of-code.md` is a tracked generated artifact and may require a freshness update after verification. [VERIFIED: `AGENTS.md:29-30`]
- The applicable routed standards were `AGENTS.bright-builds.md`, `standards-overrides.md` (no active project override), `standards/core/architecture.md`, `standards/core/code-shape.md`, `standards/core/testing.md`, `standards/core/verification.md`, and `standards/languages/rust.md`. [VERIFIED: `AGENTS.md:3-19`; `AGENTS.bright-builds.md:21-48`; `standards-overrides.md:1-16`]

## Standard Stack

### Core

| Component                         | Version / Location                                                              | Purpose                                                                                                                                    | Why Standard                                                                                                                                                                                                                                                                                                                                                                                                   |
| --------------------------------- | ------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Rust                              | `1.94.1`, edition 2024                                                          | Implement filters, typed candidates, indexes, and bridge logic                                                                             | Repository-pinned source of truth; no toolchain choice is needed. [VERIFIED: `rust-toolchain.toml`; `packages/Cargo.toml`; `AGENTS.md:21-22`]                                                                                                                                                                                                                                                                |
| Rust standard library collections | `Vec<u64>`, `BTreeMap`, `BTreeSet`, `Hash`/`Hasher` or a local byte-hash helper | Fixed storage, deterministic indexes, bounded provenance, and hash derivation                                                              | The phase explicitly delegates standard-library filter details and the network crate currently uses these collections without a mempool dependency. [VERIFIED: `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:84-90`; `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:14`; `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:14`] |
| `open-bitcoin-network`            | workspace crate                                                                 | Own pure download evidence, retained announcers, orphan indexes, and neutral package candidate                                             | The crate already owns `TxDownloadScheduler`, `TxOrphanage`, `PeerManager`, and ordinary transaction-message actions. [VERIFIED: `packages/open-bitcoin-network/src/peer.rs:163-174`; `packages/open-bitcoin-network/src/peer/transaction_relay.rs:23-31`]                                                                                                                                                   |
| `open-bitcoin-mempool`            | workspace crate, Phase 132 API                                                  | Own `WellFormedPackage`, `SubmissionPackage`, `PackageFingerprint`, `PackageReport`, `MempoolLifecycleDelta`, and authoritative submission | The existing package engine already performs individual-first evaluation, residual grouping, one trim, final result rewrite, and guarded apply. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:38-205`; `packages/open-bitcoin-mempool/src/pool/package_admission.rs:93-226`]                                                                                                                      |
| `open-bitcoin-node`               | workspace crate                                                                 | Compose network candidate with mempool package types under `ManagedPeerNetwork` authority                                                  | It already owns singleton peer admission, orphan reconsideration, chain-tip mutation, and all cross-cache adapters. [VERIFIED: `packages/open-bitcoin-node/src/network.rs:75-98`; `packages/open-bitcoin-node/src/network/admission_bridge.rs:50-151`; `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:35-142`]                                                                                 |

### Supporting

| Component                      | Location                          | Purpose                                                            | When to Use                                                                                                                                                                                                                                 |
| ------------------------------ | --------------------------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `open-bitcoin-core` primitives | existing workspace dependency     | Canonical `Transaction`, `Txid`, `Wtxid`, and identity calculation | Use only at current crate boundaries; do not add a Bitcoin-library dependency. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:15-18`; `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:16`] |
| Bun TypeScript guard scripts   | `scripts/check-phase*.ts` pattern | Mutation-tested structural/parity/claim guard                      | Add a Phase 133 checker only in the final closure slice, after behavior is proven in Rust. [VERIFIED: `scripts/verify.sh:578-579`; `scripts/check-phase132-typed-package-staged-admission.ts`]                                            |
| Bazel/Bzlmod                   | existing workspace build          | Top-level smoke build                                              | Rely on `bash scripts/verify.sh`; do not invent a Phase 133-specific build system. [VERIFIED: `AGENTS.md:63-64`; `AGENTS.md:23-24`]                                                                                                       |

### Alternatives Considered

| Instead of                                  | Could Use                                                 | Tradeoff                                                                                                                                                                                                                                                           |
| ------------------------------------------- | --------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Standard-library rolling filter             | Exact `BTreeSet`/`HashSet`                                | Exact sets are simple but violate fixed-memory PPKG-01 and the locked probabilistic filter decision. [VERIFIED: `packages/open-bitcoin-network/src/peer.rs:172`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:27-34`]           |
| One shared orphan body with announcer set   | Duplicate orphan body per peer                            | Per-peer duplication simplifies provenance but amplifies memory and contradicts the locked shared-body rule. [VERIFIED: `packages/bitcoin-knots/src/txorphanage.h:81-102`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:88-90`] |
| Neutral network candidate + node refinement | `open-bitcoin-network -> open-bitcoin-mempool` dependency | Directly carrying `SubmissionPackage` would shorten the bridge but invert the locked crate boundary and duplicate package-policy knowledge in networking. [VERIFIED: `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:61-70`]       |
| Parent-triggered assembly                   | Retained reconsiderable-parent body cache                 | A parent cache makes eager assembly symmetric but expands body storage and is explicitly forbidden. [VERIFIED: `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:53-58`]                                                             |

**Installation:** No new package installation is required. [VERIFIED: phase scope and existing workspace APIs above]

**Version verification:** Registry lookup is intentionally omitted because the prescribed stack adds no external package; Rust is repository-pinned. [VERIFIED: `rust-toolchain.toml`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:84-87`]

## Architecture Patterns

### Recommended Project Structure

```text
packages/open-bitcoin-network/src/peer/
├── transaction_relay.rs
├── transaction_relay/
│   ├── reject_evidence.rs          # fixed-memory filter + typed evidence domains
│   ├── orphanage.rs                # shared bodies, bounded announcers, parent index
│   ├── scheduler.rs                # captured announcement provenance before cleanup
│   └── tests/
│       ├── reject_evidence_cases.rs
│       └── orphanage_cases.rs
└── inventory_state.rs              # ordinary tx action carries provenance

packages/open-bitcoin-node/src/
├── mempool.rs                      # thin authoritative submit_package adapter
└── network/
    ├── admission_bridge.rs         # candidate refinement, one submit, feedback
    ├── mempool_lifecycle.rs        # active-tip reset call sites only
    └── tests/
        └── package_bridge_cases.rs
```

This split follows the repository's `foo.rs` plus `foo/` rule and keeps pure decisions in network/mempool while the node composes them. [VERIFIED: `standards/languages/rust.md:3-31,158-185`; `packages/open-bitcoin-node/src/network.rs:20-32`]

### Current Seam Map

| Seam                                               | Current behavior                                                                                                                                                                                                                                                                                                                                                                                                                     | Required Phase 133 change                                                                                                                                                                                      |
| -------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `PeerManager.recent_rejects`                       | One unbounded exact `BTreeSet<TxRelayId>` cloned into `TxDownloadLocalFacts`. [VERIFIED: `packages/open-bitcoin-network/src/peer.rs:163-174`; `packages/open-bitcoin-network/src/peer/inventory_state.rs:378-387`]                                                                                                                                                                                                                 | Replace with hard/reconsiderable fixed-memory evidence and narrow membership calls; do not clone filter storage per announcement.                                                                              |
| `note_recent_reject_for_outcome`                   | Inserts txid and wtxid for rejected, duplicate, and evicted singleton outcomes without a hard/reconsiderable distinction. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:98-106,362-376`]                                                                                                                                                                                                                  | Delete/replace with typed feedback: hard wtxid, reconsiderable wtxid, failed cached fingerprint, or no reject insertion.                                                                                       |
| `TxDownloadScheduler::record_received_transaction` | Clears all txid/wtxid announcements and in-flight records, then emits only ids. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:246-277,451-454`]                                                                                                                                                                                                                                                | Snapshot the delivering peer plus retained announcers before cleanup and return bounded provenance with the received action.                                                                                   |
| `TxOrphanage::OrphanEntry`                         | One body, one `peer_id`, missing-parent set, expiry; global scan in wtxid order; full removal on owner disconnect. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:143-158,211-260,313-335`]                                                                                                                                                                                                     | Store bounded `announcers`, insertion sequence, and indexed parent spends; remove only the departing announcer; remove body only when no announcers remain.                                                    |
| State ownership                                    | `TxDownloadScheduler`/rejects are inside `PeerManager`, but `TxOrphanage` is a separate `ManagedPeerNetwork` field, so an inventory announcement received after an orphan body cannot update its announcer set inside one pure coordinator. [VERIFIED: `packages/open-bitcoin-network/src/peer.rs:163-174`; `packages/open-bitcoin-node/src/network.rs:75-98`; `packages/open-bitcoin-network/src/peer/inventory_state.rs:88-164`] | Prefer moving `TxOrphanage` into `PeerManager` (Knots-style transaction-download ownership), or add one pure network coordinator API that receives both states. Do not make node code reconstruct eligibility. |
| `process_peer_transaction_admission`               | Submits every received transaction as a singleton first; accepted parent triggers ordinary orphan reconsideration; rejection goes to one reject set. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:51-110,113-151`]                                                                                                                                                                                       | When a received parent is already/recently reconsiderable or newly fails reconsiderably, request one neutral candidate and route it through package admission instead of repeating singleton policy.           |
| `ManagedMempool`                                   | Exposes singleton transition adapters only. [VERIFIED: `packages/open-bitcoin-node/src/mempool.rs:28-85`]                                                                                                                                                                                                                                                                                                                          | Add one thin package submission method taking `SubmitPackageCommand` and returning the exact `SubmittedPackageResult`.                                                                                         |
| Chain-tip changes                                  | Successful connect/reorg paths live in `network/mempool_lifecycle.rs`; no reject reset exists. [VERIFIED: `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:35-142`; repository grep found no reject-filter reset]                                                                                                                                                                                                      | Reset both filters immediately after each actual active-tip mutation, not on duplicate/non-extending/disconnected block receipt.                                                                               |

### Pattern 1: Typed Fixed-Memory Generational Evidence

**What:** Implement one reusable fixed-allocation rolling membership structure and wrap it in two domain-specific owners:

```rust
struct HardRejectEvidence {
    filter: RollingGenerationFilter,
}

enum ReconsiderableEvidenceKey {
    Transaction(Wtxid),
    Package([u8; 32]),
}

struct ReconsiderableRejectEvidence {
    filter: RollingGenerationFilter,
}
```

The public API should accept semantic inputs (`Wtxid` and cached fingerprint bytes) rather than a raw byte slice. The node must obtain package bytes from `PackageReport::fingerprint()`/`WellFormedPackage::fingerprint()` and must never recompute the package hash in the network crate. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:38-68,134-137`; `packages/open-bitcoin-mempool/src/package/report.rs:445-501`]

**Storage:** Match Knots' three-generation two-bit layout: `entries_per_generation = ceil(120_000 / 2)`, rotate generation after 60,000 insertions, clear only the bit pattern belonging to the reused generation, and allocate the bit-vector once. Knots computes 20 hash probes and approximately 1,294,000 bytes per filter for the locked parameters, so the pair is approximately 2,588,000 bytes excluding small struct overhead. [VERIFIED: `packages/bitcoin-knots/src/common/bloom.cpp:162-186,195-245`; formula reproduced from those exact equations with `N=120000`, `p=0.000001`]

**Hash/tweak recommendation:** Keep hash state injectable. Use a production hasher/tweak initialized by the node shell and a fixed deterministic test hasher/tweak; `reset` must clear bits, reset generation counters, and rotate/reseed hash state. This preserves the pure-core rule that randomness is supplied at a boundary while retaining deterministic vectors. [VERIFIED: `docs/parity/catalog/mempool-policy.md:299-301`; `packages/bitcoin-knots/src/common/bloom.h:83-89`; `packages/bitcoin-knots/src/common/bloom.cpp:240-245`]

**When to use:** Hard membership suppresses ordinary redownload. Reconsiderable transaction membership suppresses singleton resubmission but enables parent-triggered package lookup. Reconsiderable package membership skips a previously failed exact fingerprint and continues the bounded newest-first child traversal. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:308-331,470-485,516-566`]

### Pattern 2: Capture Provenance Before Scheduler Cleanup

**What:** Replace the received action's bare transaction identity with a typed receipt containing:

```rust
struct ReceivedTransactionProvenance {
    delivered_by: PeerId,
    announcers: BoundedAnnouncers,
}
```

At receipt time, union the delivering peer with peers retained under both txid and wtxid scheduler keys before `clear_pending_relay`. Deduplicate, cap deterministically, and pass the provenance with the ordinary `ReceivedTransaction` action. The delivering peer must always remain represented. This reuses scheduler knowledge rather than inventing a second announcer registry. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:68-87,246-277,422-454`; `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:418-427`]

Announcements may also arrive after the body is already in the orphanage. The same pure transaction-download owner must route those inventory events to `TxOrphanage::add_announcer` without duplicating or refreshing the body/TTL. Moving `TxOrphanage` under `PeerManager` is the smallest coherent ownership model because it also makes disconnect cleanup atomic; if the planner retains split ownership, it must introduce a typed `AddOrphanAnnouncer` action rather than querying node state from the network crate. [VERIFIED: `packages/bitcoin-knots/src/txorphanage.cpp:15-23,55-70`; current split at `packages/open-bitcoin-node/src/network/action_translation.rs:106-122`]

**Why:** Knots records all orphan announcers and explicitly states that same-peer assembly does not require the two bodies to have been requested from the same peer. [VERIFIED: `packages/bitcoin-knots/src/txorphanage.h:81-102`; `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:314-319`]

### Pattern 3: Indexed Newest-First Same-Peer Candidate

**What:** Extend each orphan with `announcers` and a monotonic insertion/reception sequence. Maintain a reverse index from parent `Txid` to an ordered set keyed by newest sequence then wtxid. Update the index in the same helper that inserts/removes an orphan so expiry, eviction, terminal feedback, and disconnect cannot leave stale entries.

Candidate selection should:

1. Require that the arriving parent's wtxid is in reconsiderable transaction evidence.
1. Traverse only the indexed children spending the parent, newest first.
1. Require that `announcers` contains the parent-supplying peer.
1. Skip a hard-rejected child wtxid.
1. Emit exactly one opaque `SamePeerOneParentOneChildCandidate` with `[parent, child]`, `[peer, peer]`, and a private eligibility proof.
1. Let the node refine it, then skip/retire it if the cached fingerprint is already in reconsiderable package evidence.

Knots performs the same parent-triggered, announcer-qualified, newest-first selection and returns the first child whose exact package and child are not rejected. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:308-331`; `packages/bitcoin-knots/src/txorphanage.cpp:277-315`]

**Boundedness:** Candidate work must be capped by the existing `max_reconsiderations_per_parent` (32 by default), and announcer storage must have an explicit cap derived from `OrphanPolicy`; do not depend only on the number of connected peers. The current defaults are 100 total bodies, 25 per peer, 20-minute TTL, and 32 reconsiderations per parent. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:22-42`]

**Disconnect:** Remove only the departing peer from each orphan's announcer set and per-peer accounting. Delete the body/index entries only when the set becomes empty. Knots follows this exact ownership rule. [VERIFIED: `packages/bitcoin-knots/src/txorphanage.h:55-57`; `packages/bitcoin-knots/src/txorphanage.cpp:118-140`]

### Pattern 4: Node-Owned Refinement and Single Authoritative Call

**What:** In `admission_bridge.rs`:

```rust
let checked = WellFormedPackage::try_from(candidate.into_ordered_transactions())?;
let fingerprint = *checked.fingerprint();
let submission =
    SubmissionPackage::try_from_package(checked, &self.chainstate.chainstate().snapshot())?;
let submitted = self.mempool.submit_package(
    SubmitPackageCommand {
        package: submission,
        context: AdmissionContext::peer(PolicyTime::from_unix_seconds(timestamp)),
    },
    &self.chainstate,
    verify_flags,
    consensus_params,
)?;
debug_assert_eq!(submitted.report.fingerprint(), &fingerprint);
```

This is schematic; use existing error conversions and avoid cloning transactions after the candidate is consumed. The Phase 132 constructor already proves topology/identity and the submission refinement already proves child-with-unconfirmed-parents shape. [VERIFIED: `packages/open-bitcoin-mempool/src/package/shape.rs:150-237`; `packages/open-bitcoin-mempool/src/pool/package_admission.rs:93-120`]

The bridge result should preserve the exact `SubmittedPackageResult` or its exact `PackageReport` and `MempoolLifecycleDelta`; do not flatten it to `MempoolOutcome`. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:200-205`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:64-70`]

### Feedback Classification

| Authoritative member result          | Candidate/orphan action                                                                            | Reject evidence action                                                                                                         |
| ------------------------------------ | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `FinallyPresent`                     | Retire body/candidate.                                                                             | None.                                                                                                                          |
| `AlreadyPresent`                     | Retire body/candidate.                                                                             | None.                                                                                                                          |
| `SameTxidDifferentWitness`           | Retire requested body/candidate.                                                                   | None; witness alias is not a hard reject.                                                                                      |
| `HardRejected`                       | Retire body/candidate.                                                                             | Insert requested wtxid into hard evidence only.                                                                                |
| `PostTrimAbsent`                     | Retire body/candidate as terminal for this attempt.                                                | None unless its prior authoritative result independently supplied typed reject evidence.                                       |
| `Reconsiderable::MissingInputs`      | Retain/restage only that member with its remaining missing inputs and existing bounded announcers. | Do not hard-reject; transaction reconsiderable insertion is optional only if it will not suppress the required parent request. |
| `Reconsiderable::PackageFee`         | Retire the exact candidate body unless still needed by another bounded orphan relation.            | Insert requested wtxid into reconsiderable evidence.                                                                           |
| `Reconsiderable::PackageReplacement` | Retire the exact candidate body unless still needed by another bounded orphan relation.            | Insert requested wtxid into reconsiderable evidence.                                                                           |

These are the complete Phase 132 member variants and match the locked terminal/restage distinction. [VERIFIED: `packages/open-bitcoin-mempool/src/package/report.rs:215-279`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:71-76`]

Record the cached package fingerprint in reconsiderable evidence whenever the exact package attempt is not `PackageStatus::Complete` (`Partial` or `Failed`), then use that evidence only to suppress the same content-derived pair. This is the prescriptive mapping that most closely matches Knots' package-wide invalid-result cache while fitting Open Bitcoin's explicit `Complete`/`Partial`/`Failed` report. [VERIFIED: `packages/bitcoin-knots/src/net_processing.cpp:3072-3074`; `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:516-519`; `packages/open-bitcoin-mempool/src/package/report.rs:187-193`]

### Arrival-Order State Flow

```text
child first
  ordinary tx receipt
    -> singleton MissingInputs
    -> one shared orphan body + all retained announcers
    -> ordinary txid parent request
  parent arrives from qualifying peer P
    -> singleton Reconsiderable
    -> newest child spending parent with announcer P
    -> neutral [parent, child] + [P, P]
    -> node refines once -> submit_package once -> typed feedback

parent first
  parent arrives
    -> singleton Reconsiderable evidence; no retained body
  child arrives
    -> MissingInputs orphan + parent request despite reconsiderable evidence
  parent is re-relayed by qualifying peer P
    -> reconsiderable receipt bypasses singleton retry
    -> same candidate/refinement/submission path
```

The second flow deliberately requires ordinary parent retransmission and no parent-body cache, matching the pinned test. [VERIFIED: `packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py:79-145`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:53-58`]

### Active-Tip Reset Placement

Reset both filters only after an actual active-tip mutation:

- after successful `connect_local_block`;
- after the `Connected` branch of `connect_stored_block`, not duplicate/non-extending/disconnected returns;
- after successful `reorg_to_branch`.

Those are the current node-owned active-chain mutation seams. [VERIFIED: `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:35-53,56-109,111-142`]

Use one helper such as `peer_manager.on_active_tip_changed()` so all three sites reset both filters together. Knots exposes one `ActiveTipChange` method that resets both filters. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman.h:126-130`; `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:97-101`]

### Anti-Patterns to Avoid

- **Raw booleans for same-peer eligibility:** They allow callers to forge provenance; construct an opaque proof from orphan announcers.
- **Package hashing in `open-bitcoin-network`:** It duplicates Phase 132 identity and risks digest drift.
- **Scanning the complete orphanage for every parent:** It weakens the existing per-parent work bound and makes newest ordering accidental.
- **Cloning the rolling filter into `TxDownloadLocalFacts`:** It turns a node-global fixed allocation into per-announcement work.
- **Calling singleton admission again for a known reconsiderable parent:** It wastes policy work and never reaches package evaluation.
- **Applying package delta to serving/fanout/compact/persistence here:** It partially implements Phase 134 and creates divergent lifecycle projections.

## Recommended Implementation Order

### Plan 133-01 — Fixed-Memory Typed Reject Evidence

1. Add `reject_evidence.rs` with fixed preallocated generational storage, semantic hard/reconsiderable key APIs, injectable hashing/tweak, and deterministic boundary tests.
1. Replace `PeerManager.recent_rejects` and `TxDownloadLocalFacts.recent_rejects` cloning with membership queries/typed actions.
1. Add one active-tip reset method and wire it to the three actual chain mutation paths.
1. Verify capacity/FPR-derived storage length, exact generation rotation, reset behavior, no false negatives within the guaranteed window, and bounded allocation after adversarial insertions.

This slice satisfies the bounded-state half of PPKG-01 without depending on package assembly. [VERIFIED: dependency direction in `.planning/research/ARCHITECTURE.md`; current seams cited above]

### Plan 133-02 — Announcer-Aware Orphanage and Neutral 1P1C Candidate

1. Capture bounded scheduler announcers before received-response cleanup.
1. Co-locate `TxOrphanage` with `PeerManager` transaction-download/evidence state, or add one pure coordinator that can atomically observe inventory, body, and disconnect events.
1. Replace orphan single ownership with shared body + bounded announcer provenance and correct per-peer accounting, including announcements that arrive after the body.
1. Add a parent-spend newest-first index and coherent insert/remove/expiry/eviction/disconnect cleanup.
1. Introduce opaque same-peer eligibility and aligned `[PeerId; 2]` origins.
1. Emit at most one topological `[parent, child]` candidate per bounded selection step; expose feedback needed to skip failed fingerprints and continue to the next eligible child.

This slice satisfies the pure-network portion of PPKG-02. [VERIFIED: pinned selection and current orphan seams cited above]

### Plan 133-03 — Authoritative Node Package Bridge and Feedback

1. Add the thin `ManagedMempool` package adapter.
1. Refactor singleton rejection classification so reconsiderable failures feed the new evidence and trigger candidate selection.
1. Refine the neutral candidate once through `WellFormedPackage` and `SubmissionPackage`.
1. Call `Mempool::submit_package` exactly once and preserve its report/fingerprint/delta.
1. Apply the exhaustive feedback table only to reject/candidate/orphan state.
1. Leave the package delta unprojected but available to Phase 134.

This slice completes PPKG-02 and PPKG-03. [VERIFIED: Phase 132 API and node bridge seams cited above]

### Plan 133-04 — Parity, Integration, and Guardrail Closure

1. Add deterministic node integration cases for both arrival orders, announcer-qualified different-delivery peer, wrong-peer suppression, newest-child fallback after a failed fingerprint, two-reconsiderable-parent suppression, hard-vs-reconsiderable separation, and active-tip resets.
1. Add bounded adversarial cases for filter churn, announcer caps, disconnect cleanup, expiry/eviction index cleanup, and per-parent traversal.
1. Update `docs/parity/catalog/mempool-policy.md` with the exact bounded claim and Phase 134/136 boundaries.
1. Register new Rust files/tests in `docs/parity/source-breadcrumbs.json`, including `p2p_opportunistic_1p1c.py` as a direct anchor.
1. Add a mutation-tested Phase 133 checker and wire it after Phase 132 in `scripts/verify.sh`.
1. Run `bash scripts/verify.sh` and review generated LOC freshness/diff.

This slice closes all three requirements without introducing live-network gates. [VERIFIED: `AGENTS.md:23-36`; `scripts/verify.sh:578-579`; `.planning/REQUIREMENTS.md:45-48`]

## Verification Strategy

Nyquist validation is disabled for this repository, so the formal `Validation Architecture` section is intentionally omitted. [VERIFIED: `.planning/config.json` has `workflow.nyquist_validation: false`] The phase still needs the following planner-owned verification layers:

| Requirement | Test layer                                          | Required cases                                                                                                                                                                                                                                                                                  | Focused command                                                                                                                                                       |
| ----------- | --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| PPKG-01     | `open-bitcoin-network` unit tests                   | Fixed word count derived from capacity/FPR; exact three-generation rollover; no false negatives inside the guaranteed window; separate hard/reconsiderable membership; reset on each active-tip transition; no allocation growth under adversarial unique inserts.                              | `bun run scripts/command-timings.ts run --key phase133-network-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-network`                       |
| PPKG-02     | Network unit tests plus node integration tests      | Both arrival orders; same announcer with another delivering peer; wrong-peer suppression; late announcement after body receipt; newest eligible child; failed-fingerprint fallback; two-reconsiderable-parent suppression; siblings/grandchildren excluded; disconnect/expiry/eviction cleanup. | Network command above, then `bun run scripts/command-timings.ts run --key phase133-node-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node` |
| PPKG-03     | Node integration tests with a counting mempool seam | Exactly one `submit_package` call; exact report/fingerprint/delta preservation; every typed feedback branch; hard child removal; reconsiderable parent retention; no serving/fanout/compact/persistence projection.                                                                             | `bun run scripts/command-timings.ts run --key phase133-node-tests -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node`                             |
| All         | Phase checker and repository contract               | Mutation tests prove the checker rejects missing package bridge, wrong identity, lost announcers, unbounded evidence, and premature lifecycle projection; generated parity/LOC artifacts are fresh.                                                                                             | `bash scripts/verify.sh`                                                                                                                                              |

New Rust source or test files must be registered through the parity breadcrumb workflow before the repository contract can pass. [VERIFIED: `AGENTS.md` Repo-Local Guidance; `scripts/check-parity-breadcrumbs.ts`] Do not overlap the two Cargo package commands against the same target directory; run them sequentially through the repository timing wrapper. [VERIFIED: `AGENTS.md` Repo-Local Guidance]

## Don't Hand-Roll

| Problem                           | Don't Build                                                                                | Use Instead                                                    | Why                                                                                                                                                                                                                        |
| --------------------------------- | ------------------------------------------------------------------------------------------ | -------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Package shape/topology validation | Network-local checks for two transactions, ordering, conflicts, or child-with-parent shape | `WellFormedPackage` then `SubmissionPackage::try_from_package` | Phase 132 already centralizes the invariants and caches identity. [VERIFIED: `packages/open-bitcoin-mempool/src/package/shape.rs:150-237`]                                                                               |
| Package policy                    | Fee/RBF/TRUC/ephemeral/script rules in network or node                                     | `Mempool::submit_package`                                      | The engine already performs the authoritative ordered evaluation and guarded apply. [VERIFIED: `packages/open-bitcoin-mempool/src/pool/package_admission.rs:93-226`]                                                     |
| Package identity                  | A network-specific hash or peer-qualified fingerprint                                      | Cached `PackageFingerprint` from the checked package/report    | The fingerprint is already permutation-independent and copied into the report unchanged. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:38-68`; `packages/open-bitcoin-mempool/src/package/report.rs:445-501`] |
| Candidate body cache              | Separate reconsiderable-parent/package body store                                          | Existing bounded `TxOrphanage`                                 | Locked scope forbids a parent body cache and requires bodies/provenance outside filters. [VERIFIED: `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:35-38,53-58`]                          |
| Package wire protocol             | Package inv/getdata/message variants                                                       | Existing ordinary inv/getdata/tx flow                          | The pinned behavior calls `ProcessNewPackage` after ordinary `tx` receipt; no package wire message is involved. [VERIFIED: `packages/bitcoin-knots/src/net_processing.cpp:4273-4327`]                                    |
| Cross-cache lifecycle projection  | Ad hoc updates from each member result                                                     | Phase 134 lifecycle authority                                  | The roadmap explicitly assigns complete projection to Phase 134. [VERIFIED: `.planning/ROADMAP.md:130-145`]                                                                                                              |

**Key insight:** The only genuinely new algorithms are bounded probabilistic evidence and indexed provenance-aware candidate selection. Package validation, identity, staged mutation, and lifecycle facts already exist and should remain authoritative.

## Common Pitfalls

### Pitfall 1: Losing Announcers When the Body Arrives

**What goes wrong:** `record_received_transaction` deletes both announcement maps before the orphan is staged, so later same-peer eligibility sees only the delivering peer. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:246-277,451-454`]

**How to avoid:** Capture and transfer bounded provenance atomically with received-response cleanup.

**Warning sign:** A test where peer A announces the child, peer B delivers its body, and peer A later supplies the parent fails to assemble.

### Pitfall 2: Treating “Same Peer” as Same Body Deliverer

**What goes wrong:** Valid pinned pairs are excluded and an attacker can influence which body-delivery peer becomes owner. Knots qualifies by retained announcer membership. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:314-319`]

**How to avoid:** Keep a shared body and use `announcers.contains(parent_peer)`.

### Pitfall 3: Reconsiderable Evidence Suppresses Parent Requests

**What goes wrong:** The parent-first flow stalls because the child's missing-parent request uses the same “already have including reconsiderable” predicate as ordinary inv suppression.

**How to avoid:** Mirror Knots' `include_reconsiderable=false` parent-request check; reconsiderable means “do not retry singleton,” not “do not redownload for a package.” [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:382-407`]

### Pitfall 4: Wrong Reject Identity

**What goes wrong:** Inserting txid broadly can suppress a valid different-witness transaction; mixing hard and reconsiderable evidence suppresses legitimate CPFP. Knots normally inserts wtxid and treats witness-sensitive exceptions carefully. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:454-500`]

**How to avoid:** Follow the locked wtxid-only hard/reconsiderable key decision; do not preserve the current helper's txid+wtxid insertion.

**Parity note:** Pinned Knots additionally stores txid for inherited hard-parent failures and `TX_INPUTS_NOT_STANDARD`, and its 1P1C selector queries the child txid. Phase 133's locked wtxid-only hard key is therefore a deliberate scoped difference that must be recorded in the parity catalog rather than described as byte-for-byte internal parity. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:448-449,497-499,324-328`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:27-34`]

### Pitfall 5: Failed Fingerprint Recomputed or Peer-Qualified

**What goes wrong:** Failed-package suppression misses the same content or diverges from the authoritative report.

**How to avoid:** Copy/borrow the cached fingerprint bytes once and keep aligned peer origin in separate metadata. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:38-68`; `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:46-52`]

### Pitfall 6: Orphan Index Drift

**What goes wrong:** Expired/evicted/disconnected/terminal bodies remain selectable through a stale parent index.

**How to avoid:** Centralize all deletion in one helper that removes pending reconsideration, parent-index entries, per-peer counts, and the body; add a full recomputation oracle in tests.

**Warning sign:** `len()` is bounded but candidate traversal returns a wtxid absent from `orphans`.

### Pitfall 7: Resetting on Receipt Rather Than Active-Tip Change

**What goes wrong:** Side-branch, duplicate, or disconnected block traffic clears suppression state unnecessarily, while local/reorg paths may fail to clear it.

**How to avoid:** Reset only after authoritative active-chain mutation in the three node lifecycle seams. [VERIFIED: `packages/open-bitcoin-node/src/network/mempool_lifecycle.rs:35-142`]

### Pitfall 8: Partial Phase 134 Projection

**What goes wrong:** Package members are stored/relayed through singleton helpers inconsistently, and the next phase must unwind two lifecycle paths.

**How to avoid:** Preserve the exact delta and update only candidate correctness state in Phase 133. Add a guard test that package admission does not call relay fanout/serving projection from the new bridge.

### Pitfall 9: Unbounded Work Hidden Behind Fixed Memory

**What goes wrong:** Filters are fixed-size but parent selection scans every orphan or every announcer, allowing CPU amplification.

**How to avoid:** Bound announcers explicitly, index by parent, and cap traversal by `max_reconsiderations_per_parent`.

### Pitfall 10: Guessing Reconsiderability from `RelayFeeTooLow`

**What goes wrong:** Static relay-floor failures are incorrectly placed in the reconsiderable filter, or rolling-floor failures are permanently hard-rejected.

**Why it happens:** The current singleton outcome collapses both into one category. [VERIFIED: `packages/open-bitcoin-mempool/src/outcome.rs:42-117`]

**How to avoid:** Obtain the classification from Phase 132 package evaluation/reporting or add the same typed classification to singleton transition facts; never branch on display strings or the coarse category alone.

## Code Examples

Verified patterns from repository and pinned sources:

### Authoritative Package Submission

```rust
// Source: packages/open-bitcoin-mempool/src/pool/package_admission.rs:93-119
pub fn submit_package(
    &mut self,
    command: SubmitPackageCommand,
    chainstate: &ChainstateSnapshot,
    verify_flags: ScriptVerifyFlags,
    consensus_params: ConsensusParams,
) -> Result<SubmittedPackageResult, MempoolError> {
    let evaluation = evaluate_package(
        self,
        command.package.package(),
        command.context,
        chainstate,
        verify_flags,
        consensus_params,
    )?;
    let delta = if let Some(patch) = evaluation.patch {
        self.apply_prepared(patch)?
    } else {
        MempoolLifecycleDelta::empty()
    };
    Ok(SubmittedPackageResult {
        report: evaluation.report,
        delta,
    })
}
```

### Same-Peer Candidate Predicate

```rust
// Recommended Rust translation of:
// packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:308-331
// packages/bitcoin-knots/src/txorphanage.cpp:277-315
fn maybe_newest_same_peer_child(
    &self,
    parent_txid: Txid,
    peer_id: PeerId,
    hard_rejects: &HardRejectEvidence,
) -> Option<&OrphanEntry> {
    self.children_by_parent
        .get(&parent_txid)?
        .iter()
        .take(self.policy.max_reconsiderations_per_parent)
        .filter_map(|key| self.orphans.get(&key.wtxid))
        .find(|entry| {
            entry.announcers.contains(peer_id)
                && !hard_rejects.contains_transaction(entry.wtxid)
        })
}
```

### Disconnect Cleanup

```rust
// Recommended Rust translation of packages/bitcoin-knots/src/txorphanage.cpp:118-140
fn remove_announcer(&mut self, wtxid: Wtxid, peer_id: PeerId) {
    let should_remove_body = self
        .orphans
        .get_mut(&wtxid)
        .is_some_and(|entry| {
            entry.announcers.remove(peer_id);
            entry.announcers.is_empty()
        });

    if should_remove_body {
        self.remove_orphan(wtxid);
    }
}
```

## State of the Art

| Old Open Bitcoin Approach                            | Phase 133 Approach                                                    | Impact                                                                                                                                                                                                                                        |
| ---------------------------------------------------- | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Exact unbounded `BTreeSet<TxRelayId>` recent rejects | Two fixed-memory generational filters with separate semantic domains  | Bounded adversarial memory and correct hard/reconsiderable behavior. [VERIFIED: current state `packages/open-bitcoin-network/src/peer.rs:172`; target `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md:27-37`] |
| One orphan body owner                                | One body with bounded retained announcers                             | Correct pinned same-peer meaning and disconnect survival. [VERIFIED: current state `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:151-158`; pinned `packages/bitcoin-knots/src/txorphanage.h:81-102`]               |
| Global wtxid-ordered reconsideration scan            | Parent-indexed newest-first bounded traversal                         | Deterministic candidate choice with bounded work. [VERIFIED: current state `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:221-248`; pinned `packages/bitcoin-knots/src/txorphanage.cpp:277-315`]                    |
| Singleton-only peer admission                        | Neutral 1P1C candidate refined by node into Phase 132 package command | One policy engine for local and peer packages. [VERIFIED: current state `packages/open-bitcoin-node/src/network/admission_bridge.rs:51-110`; target `packages/open-bitcoin-mempool/src/pool/package_admission.rs:93-120`]                   |

**Deprecated/outdated in this phase:**

- `PeerManager::note_recent_reject(TxRelayId)` and `TxDownloadLocalFacts.recent_rejects: BTreeSet<_>` should be removed, not wrapped. [VERIFIED: `packages/open-bitcoin-network/src/peer.rs:261-263`; `packages/open-bitcoin-network/src/peer/transaction_relay/scheduler.rs:26-31`]
- `OrphanEntry.peer_id` ownership semantics should be replaced with bounded announcer provenance. [VERIFIED: `packages/open-bitcoin-network/src/peer/transaction_relay/orphanage.rs:151-158`]
- `reconsider_child` as a singleton-only bridge is insufficient for a reconsiderable parent/child pair; retain it only for ordinary accepted-parent orphan reconsideration. [VERIFIED: `packages/open-bitcoin-node/src/network/admission_bridge.rs:228-294`]

## Assumptions Log

| #   | Claim | Section | Risk if Wrong |
| --- | ----- | ------- | ------------- |

All behavioral and structural claims in this research were verified against repository or pinned-submodule sources. Recommendations in the agent's discretion areas are explicitly labeled as recommendations and require no new user product decision.

## Open Questions

1. **RESOLVED — Where should production filter hash entropy enter?**

   - What we know: Knots changes a random tweak on reset, while Open Bitcoin policy requires randomness to be injected rather than read in the pure core. [VERIFIED: `packages/bitcoin-knots/src/common/bloom.cpp:240-245`; `docs/parity/catalog/mempool-policy.md:299-301`]
   - What's unclear: There is no existing Phase 133-specific seed field in `ManagedPeerNetwork` or `PeerManager`. [VERIFIED: `packages/open-bitcoin-node/src/network.rs:75-98`; `packages/open-bitcoin-network/src/peer.rs:163-180`]
   - Decision: The rolling filter accepts an explicit seed/tweak source. The node shell derives production seed material with a standard-library randomized hasher and injects a fresh tweak at construction and each active-tip reset; tests inject fixed values. The seed stays out of snapshots and operator evidence.

1. **RESOLVED — How should a missing-input package result refresh remaining parents?**

   - What we know: `ReconsiderableMemberFailure::MissingInputs` currently carries only requested identity, while the existing singleton `MempoolOutcome::Orphaned` carries `missing_parents`. [VERIFIED: `packages/open-bitcoin-mempool/src/package/report.rs:236-249`; `packages/open-bitcoin-node/src/network/admission_bridge.rs:82-96`]
   - What's unclear: Phase 133 feedback cannot restage an updated missing-parent set from the package report alone.
   - Decision: Extend `ReconsiderableMemberFailure::MissingInputs` with the exact deterministic remaining parent txids computed by authoritative package evaluation. Phase 133 feedback restages from that typed payload and never infers or reuses a possibly stale pre-attempt parent set.

## Security Domain

### Applicable ASVS Categories

| ASVS Category         | Applies                | Standard Control                                                                                                                                                                                                                               |
| --------------------- | ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| V2 Authentication     | No                     | Peer identity is an existing connection-local `PeerId`; this phase does not add authentication. [VERIFIED: phase boundary]                                                                                                                     |
| V3 Session Management | No                     | No user session or credential state is introduced. [VERIFIED: phase boundary]                                                                                                                                                                  |
| V4 Access Control     | Yes, domain provenance | Opaque same-peer eligibility proof constructed only from retained announcer state; callers cannot supply a boolean.                                                                                                                            |
| V5 Input Validation   | Yes                    | `WellFormedPackage` and `SubmissionPackage` validate count, weight, identity, topology, conflicts, and child-with-parent shape before package policy. [VERIFIED: `packages/open-bitcoin-mempool/src/package/shape.rs:150-237`]               |
| V6 Cryptography       | No security decision   | The rolling filter hash is probabilistic resource evidence, not an authorization or consensus primitive; keep package identity on existing SHA-256 `PackageFingerprint`. [VERIFIED: `packages/open-bitcoin-mempool/src/package.rs:13,38-68`] |

### Known Threat Patterns

| Pattern                                            | STRIDE                       | Standard Mitigation                                                                                                                                                                             |
| -------------------------------------------------- | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unique reject flood grows memory                   | Denial of Service            | Preallocated fixed-size generational filters; no exact backup set.                                                                                                                              |
| Fake children crowd out honest child               | Denial of Service / Spoofing | Same-peer announcer qualification, newest-first bounded traversal, failed-fingerprint skip. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:314-328`]                      |
| Many peers amplify one orphan body                 | Denial of Service            | One shared body, explicit announcer cap, per-peer accounting, remove only departing announcer.                                                                                                  |
| Forged sender metadata                             | Spoofing                     | Private candidate eligibility proof and aligned origin array created inside network state.                                                                                                      |
| False-positive filter result treated as peer fault | Repudiation / DoS            | Suppress work only; never punish a peer based solely on probabilistic membership. Knots documents this boundary. [VERIFIED: `packages/bitcoin-knots/src/node/txdownloadman_impl.cpp:543-558`] |
| Reimplemented package policy diverges              | Tampering                    | Node calls the Phase 132 engine once and preserves exact report/fingerprint/delta.                                                                                                              |

## Sources

### Primary (HIGH confidence)

- `.planning/phases/133-package-aware-download-and-orphan-bridge/133-CONTEXT.md` — locked Phase 133 decisions, canonical references, boundaries.
- `.planning/phases/132-typed-package-vocabulary-and-staged-admission/132-CONTEXT.md` — consumed package identity, report, staged admission, and lifecycle contracts.
- `.planning/phases/102-orphan-handling-and-admission-outcome-bridge/102-CONTEXT.md` — inherited bounded orphan/download/runtime bridge decisions.
- `packages/bitcoin-knots/src/node/txdownloadman.h` and `txdownloadman_impl.{h,cpp}` — filters, reset, reconsiderable handling, `PackageToValidate`, and 1P1C selection.
- `packages/bitcoin-knots/src/txorphanage.h` and `txorphanage.cpp` — announcers, newest-first child lookup, disconnect behavior, expiry/eviction.
- `packages/bitcoin-knots/src/common/bloom.h` and `common/bloom.cpp` — rolling filter formulas, generation layout, reset.
- `packages/bitcoin-knots/src/net_processing.cpp` — ordinary `tx` bridge, `ProcessInvalidTx`, `ProcessPackageResult`, one `ProcessNewPackage` call.
- `packages/bitcoin-knots/src/validation.h` and `validation.cpp` — authoritative member/package result semantics.
- `packages/bitcoin-knots/test/functional/p2p_orphan_handling.py`, `p2p_tx_download.py`, and `p2p_opportunistic_1p1c.py` — parent requests, filter reset, arrival order, same-peer, failed fingerprint, multiple-parent boundaries.
- `packages/open-bitcoin-network/src/peer/transaction_relay/{orphanage,scheduler}.rs`, `peer.rs`, and `inventory_state.rs` — exact current download/orphan/reject seams.
- `packages/open-bitcoin-node/src/network/admission_bridge.rs`, `network/mempool_lifecycle.rs`, `network.rs`, and `mempool.rs` — exact shell and chain-tip seams.
- `packages/open-bitcoin-mempool/src/package.rs`, `package/report.rs`, `package/shape.rs`, and `pool/package_admission.rs` — exact Phase 132 APIs.
- `docs/parity/catalog/mempool-policy.md` and `docs/parity/source-breadcrumbs.json` — parity claim and source-registration contracts.

### Secondary (MEDIUM confidence)

- None. No web or ecosystem sources were needed.

### Tertiary (LOW confidence)

- None.

## Metadata

**Confidence breakdown:**

- Standard stack: HIGH — repository-pinned toolchain and existing workspace crates; no new dependency.
- Architecture: HIGH — exact current Rust seams mapped to pinned Knots call paths and locked decisions.
- Pitfalls: HIGH — derived from current state mismatches and pinned comments/tests.
- Filter implementation detail: MEDIUM-HIGH — storage/rotation is pinned; production seed plumbing remains a planner-visible discretion item.

**Research date:** 2026-07-26
**Valid until:** 2026-08-25, or until Phase 132 package APIs / Phase 102 orphan bridge change.
