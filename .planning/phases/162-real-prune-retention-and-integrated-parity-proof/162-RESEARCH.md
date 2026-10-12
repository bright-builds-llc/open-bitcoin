---
generated_by: gsd-phase-researcher
lifecycle_mode: yolo
phase_lifecycle_id: 162-2026-10-11T16-25-37
generated_at: 2026-10-11T16:35:35Z
---

# Phase 162: Real Prune Retention and Integrated Parity Proof - Research

**Researched:** 2026-10-11
**Domain:** Continuous validated-chain integration, retained BASIC index, concrete Fjall durability and client transport evidence
**Confidence:** HIGH for existing seams; MEDIUM for unmeasured integrated resource footprint

<user-constraints>
## User Constraints (from CONTEXT.md)

Source for the following verbatim decisions and discretion: [VERIFIED: 162-CONTEXT.md].

### Locked Decisions

### Continuous history and actual production callers

- **D-01:** Extend the existing normal-maturity validated spend/fork recipe past height 1,000. Read actual runtime consensus parameters, retain normal 100-block coinbase maturity, continuous parentage, real historical and same-block spends, real durable coins and accepted connect receipts. Sparse codec-valid fixtures, MemoryCoinsView-only proofs and maturity-one histories cannot satisfy the integrated acceptance gate.
- **D-02:** Establish one identifiable scenario/history across actual configured startup, receive-independent scheduled catch-up, ordinary accepted connect, shallow reorg with displaced hash lookup, legal manual pruning, ordinary automatic retention, all-handle datadir close/reopen and client responses. Share recipes/artifacts where crate privacy requires composition, but prove provenance and every handoff; do not substitute unrelated tests for the end-to-end lifecycle.
- **D-03:** Exercise network prune-after and trailing-window rules through normal planning/apply callers. Automatic target stays the legal 550 MiB minimum. Codec-valid nonactive auxiliary rows may supply explicit storage pressure only; they are not validated-history evidence. Report their purpose and bounds separately. Prefer the simplest bounded recipe over hundreds of expensive irrelevant accepted blocks.
- **D-04:** Demonstrate real paired body/undo absence after deletion and reopen, unchanged durable coins/checkpoint identity and exact retained immutable BASIC bytes/hash/header. Include displaced records and at least one positive 1,000-height checkpoint; an empty checkpoint response is insufficient evidence.

### Client service and restrictions

- **D-05:** Use authenticated real RPC dispatch/server wiring and actual inbound/outbound socket adapters. Exercise getblockfilter/getindexinfo and all three peer families (getcfilters, getcfheaders, getcfcheckpt) after paired prune/reopen. Compare exact replies against independently pinned expected commitments, ancestry and ordering; source-only plans or prepared/enqueued replies are insufficient.
- **D-06:** Prove complete transport writes earn receipts and partial/failed writes do not. Preserve finite work, byte, queue and cleanup policies and both caller directions. General serving and effective per-peer permissions remain explicit; public listeners/defaults are not activated by tests or status collection.
- **D-07:** Retained filters never restore ordinary old-body serving or wallet eligibility. Exercise real callers for positive retained availability and negative body/wallet restrictions, with unchanged prior wallet/checkpoint state on refusal. Assert truthful NODE_COMPACT_FILTERS/NODE_NETWORK_LIMITED coexistence where relevant.

### Fault/refusal evidence and parity

- **D-08:** Cover fresh and saved/lagged missing body/undo, interrupted pending prune intent, cursor ahead of recovered durable chainstate, filter index/protection/checkpoint write failure, corrupted immutable records and partial writes with concrete Fjall/durable-coins observations. No fabricated progress, invented empty success, lost valid prefix or successful write receipt is permitted.
- **D-09:** Software fault seams must sit at actual production effect boundaries, remain test-only where possible, and preserve the existing serialized owner and safe coins-fenced cursor. Do not introduce another authority, production crate, dependency, storage engine, repair/import/download mode or unbounded fixture cache.
- **D-10:** Reuse pinned Knots source and existing independent BASIC/peer oracles. Update source breadcrumbs, parity roots and deterministic claim/mutation guards with exact evidence links. Keep Fjall layout, BASIC-only/type-0 scope and V0 exclusion, earlier missing-undo refusal, local resource-policy and modeled implicit-permission limitations explicit.
- **D-11:** Deliberately transition current-document historical guards that require Phase162-pending text, retaining their substantive safety/privacy/security checks and adding negative controls for unsupported claims. Preserve archived reports at their original evidence boundary. A completion sentence alone cannot activate CFPR-02 or CFGR requirements.

### Operator documentation and release gates

- **D-12:** Update relevant current README/operator/parity docs and give copy-pasteable repo-local Cargo and Bazel UAT commands, bounded runtime/space expectations and exact evidence limits. Reuse the Phase161 shared operator presentation; no new screen, GUI, navigation, formatter schema or wallet feature is needed.
- **D-13:** Perform an explicit simplification pass, independent source and declared-mitigation review, required native claim/mutation checks, full default `bash scripts/verify.sh` and lifecycle-valid formal verification. Record failures and reruns honestly. Use timing wrappers for ad-hoc Cargo/Bazel and serialize jobs against shared targets.
- **D-14:** Keep all nested discussion/planning/execution commits deferred until the strict wrapper's clean verification gate. Finalize the already checked-out `main` with mandatory hooks, then push its upstream. Do not bypass hooks or create a branch. Never activate requirements based on plans or stale verification.
- **D-15:** Preserve BIP37/V0, assumeutxo/dual-chainstate, archive-scale, public-default/public-network-CI, implicit repair/import/download and production-service/readiness/funds exclusions. Existing v2.4 crash-window advisories are unchanged. Phase completion does not automatically archive or ship the milestone; subsequent milestone audit remains separate.

### Agent Discretion

Exact helper/file boundaries, test-only seams and reproducible fixture composition may follow research. Favor existing modules, explicit provenance and focused assertions. No pending todos matched Phase162.

### Deferred Ideas (OUT OF SCOPE)

All excluded surfaces in D-15, unrelated v2.4 advisory cleanup, milestone archive/shipment and new operator UI. No folded todos or scope expansion.
</user-constraints>

<phase-requirements>
## Phase Requirements

Descriptions are copied from `.planning/REQUIREMENTS.md`. [VERIFIED: .planning/REQUIREMENTS.md]

| ID | Description | Research Support |
| -- | -- | -- |
| CFPR-02 | A client can retrieve previously indexed filters, headers and checkpoints after real paired body/undo deletion and datadir reopen, while ordinary block serving and wallet eligibility retain the shipped pruned-body restrictions. | Same-datadir handoff, exact independent expectations, nonempty checkpoint, real wallet/body callers. |
| CFGR-01 | A contributor can audit pinned BASIC/protocol/RPC parity and intentional Fjall, BASIC-only and local resource-policy differences through source breadcrumbs, parity roots and deterministic native claim checks that retain the milestone's excluded surfaces. | Pinned oracle reuse, unique parity ownership, source provenance and tested historical guard transition. |
| CFGR-02 | A contributor can reproduce a hermetic continuous validated-chain scenario through actual startup, scheduled catch-up, connect/reorg, real Fjall paired prune, reopen, RPC and both peer transport paths, including missing-history refusal and durability/write failures; sparse codec-valid fixtures alone do not satisfy this proof. | Normal-maturity recipe, real scheduler, public outbound caller and node-private effect fault matrix. |
</phase-requirements>

## Project Constraints (from AGENTS.md)

- Preserve Knots `29.3.knots20260210`; single active durable chainstate; pure domain/effectful adapter boundaries; no production Rust Bitcoin dependency or new crate. [VERIFIED: AGENTS.md]
- Use pinned Rust 1.94.1, Bun automation, existing Bazel/Bzlmod and no `bun install` bootstrap. Run ad-hoc Cargo/Bazel through `scripts/command-timings.ts`; never overlap shared Cargo targets. [VERIFIED: AGENTS.md]
- Full `bash scripts/verify.sh` is the pre-commit/release contract, including Bazel smoke; `--fast` is iteration only. Preserve hooks and tracked generated LOC freshness. Poll running verification at least every 60 seconds and obtain liveness evidence before stopping it. [VERIFIED: AGENTS.md]
- Register first-party Rust source/test breadcrumbs through `docs/parity/source-breadcrumbs.json`; update contributor/operator README facts and parity inventory. Supply literal Cargo/Bazel operator commands. [VERIFIED: AGENTS.md]
- Keep managed Bright Builds files untouched; overrides contain no substantive active exception. Functional-core tests, explicit Arrange/Act/Assert, typed invariants, optional `maybe_` naming, early returns and bounded modules apply. Use Bun for substantial orchestration and preserve the narrow existing Python oracle exception. [VERIFIED: AGENTS.bright-builds.md; standards-overrides.md; standards/core/architecture.md; standards/core/testing.md; standards/core/code-shape.md; standards/languages/rust.md; standards/languages/typescript-javascript.md]
- Preserve append-only task/lesson blocks, configured Markdown syntax and frontmatter delimiters. Repository edits remain inside the parent GSD workflow; D-14 overrides ordinary nested documentation commits. [VERIFIED: AGENTS.md; standards/core/local-guidance.md; standards/core/verification.md; 162-CONTEXT.md]

## Summary

Build the integrated acceptance test around the daemon's real configured startup and authenticated context, with one normal-maturity history continuing through scheduler, connect/reorg, manual/automatic deletion, complete close/reopen and clients. Existing pieces prove those seams separately, but daemon retention currently uses 401 blocks/maturity one, peer retention lacks a subsequent reopen, and outbound coverage has its own shorter fixture. Extend and compose these existing callers rather than creating another runtime. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs; packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/rpc/retention.rs; packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs; packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs]

The two principal planning hazards are provenance and privacy boundaries. The daemon's current independent wire helper serializes commitments read back from the Rust index: it independently proves wire encoding, not filter construction. Existing `FilterPublicationFault` controls and deterministic TCP partial-write controls are node-private `cfg(test)` seams; they cannot be called from a dependent RPC binary merely because that binary is being tested. [VERIFIED: packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs::expectations; scripts/basic-filter-oracle.py::compute; packages/open-bitcoin-node/src/storage/fjall_store/filters/publication.rs; packages/open-bitcoin-node/src/sync/tcp.rs]

**Primary recommendation:** Put the long successful lifecycle in daemon unit tests, reopen its exact datadir into the public `DurableSyncRuntime::sync_once` TCP path for outbound service, and keep effect-injection branches inside node unit tests using the identical named recipe/provenance contract. [VERIFIED: packages/open-bitcoin-node/src/sync.rs::sync_once; packages/open-bitcoin-node/src/sync/open_runtime.rs; packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs; 162-CONTEXT.md D-02/D-09]

## Standard Stack

Use the repository's pinned stack; no installation, latest-version upgrade or new package is recommended. Versions below are exact local manifest/lock observations, not claims about latest registry releases; publish-date verification is inapplicable to this dependency-free phase. [VERIFIED: packages/Cargo.toml; packages/Cargo.lock; rust-toolchain.toml; .bun-version; 162-CONTEXT.md D-09]

| Component | Pinned version | Use |
| -- | -- | -- |
| Rust / edition | 1.94.1 / 2024 | Existing first-party crates and unit tests. |
| Fjall | 3.1.10, git revision `aa30dca811399a201e0b9595da93a4582dcb2b57` | Existing concrete coins, index and paired deletion. |
| Tokio / Axum | 1.52.1 / 0.8.9 | Existing authenticated HTTP and inbound listener. |
| serde / serde_json | 1.0.228 / 1.0.149 | Bounded public recipe/oracle/capture shapes. |
| Bun / Bazel | 1.3.9 / 8.6.0 | Existing automation / smoke and UAT. |

Sources for all stack table cells: [VERIFIED: packages/Cargo.lock; rust-toolchain.toml; .bun-version; .bazelversion].

## Architecture Patterns

### Exact ownership and feasible composition

Paths below are repository-relative; proposed new module names are recommendations, not claims that those files already exist. Existing seams and access levels were inspected. [VERIFIED: source paths in table]

| Piece | Existing seam | Prescriptive implementation |
| -- | -- | -- |
| Normal-maturity recipe | `packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs::ForkFixture::standard`, `SyncNetwork::consensus_params` in `src/sync/types.rs` | Generate h0..1001+, first historical coinbase spend at h101, then non-coinbase successor and same-block spends. Obtain network parameters; never call daemon legacy `params()` (maturity one). Accept history through real `connect_local_block` and retain receipts; snapshot seeding alone is insufficient. |
| Share without new production crate | `packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/filter_index/fixtures.rs`; node `sync/tests/filter_index/` | Add a dependency-free test recipe file under existing testdata and include it under each crate's test modules, or use a bounded public raw-block recipe consumed by both. No global temp datadir; each scenario/fault gets unique reserved storage. Record recipe version and ordered block identities. |
| Configured startup | daemon `open_runtime_store`, `open_authoritative_network_runtime`; node `src/sync/open_runtime.rs::open_configured` | Start with saved accepted history and disabled index, then actually parse BASIC activation and open; assert partial startup prefix before scheduled progress. |
| Actual idle scheduler | `packages/open-bitcoin-rpc/src/bin/open_bitcoind/coins_flush.rs::coins_flush_worker_loop`, `CheckpointWait` | Add tests under this module or a narrow `cfg(test)` harness in its own module. Inject elapsed waits into the existing loop, inspect zero peers and increasing processed endpoint between waits, then explicit shutdown. Do not replace the loop with direct index calls or sleep for 125 ticks. |
| Ordinary retention | `src/network/runtime_authority/automatic_prune.rs`; node `sync/tests/filter_index/evidence/retention.rs` | Observe protection blocking deletion while safe prefix lags, checkpoint through ordinary flush, then legal manual planner/RPC and ordinary automatic flush. Reopen/configure automatic policy rather than merely editing an expected result. |
| Authenticated RPC and inbound | daemon `tests/filter_index/fixtures.rs::DaemonFixture`; `peer_serving/evidence.rs::with_peer` | Construct real shared authenticated HTTP state and actual loopback listener on reopened data; issue both RPC methods and all three wire families. Authenticate rejection control must remain. |
| Outbound after same-data handoff | public `DurableSyncRuntime::open_with_configured_runtime_activation`, `sync_once`, `TcpPeerTransport` | Close daemon HTTP state/context/listener/worker/authority/store first, reopen exact datadir with explicit BASIC/general-serving activation, configure one loopback peer and finite message count, use real `TcpPeerTransport`. Its normal session calls private `session/filter_serving.rs`; no visibility widening is necessary for successful outbound proof. |
| Concrete faults | node-private `FilterPublicationFault`, `TcpPeerSession::control_writes`; RPC-private inbound writer control | Keep durability and outbound deterministic partial faults inside node unit tests; inbound fault controls inside RPC listener unit tests. Use same recipe identities and inspect actual reopened data. Do not expose test knobs as normal production configuration. |

Source for composition recommendations and public/private facts: [VERIFIED: files/symbols named in table; packages/open-bitcoin-rpc/src/inbound_listener/tests/peer_filters/mixed.rs::ControlledTransport; packages/open-bitcoin-node/src/sync/tests/peer_filter_transport.rs].

### One scenario, explicit handoff

Recommended sequence: accepted durable normal-maturity history → configured startup (partial BASIC prefix) → elapsed idle scheduler (no peer input) → real connect → shallow fork crossing or adjacent to h1000 → checkpoint → legal manual prune of old indexed pair → automatic pressure/deletion → exhaustive close → configured reopen → authenticated RPC/inbound replies and body/wallet restrictions → exhaustive close → public configured runtime reopen → outbound TCP replies. Capture the same recipe ID, accepted tip, durable coins B, durable metadata tip, safe filter endpoint, selected active/stale hashes and oracle commitments at each handoff. Do not assert durable identities remain unchanged across intentional connect/reorg; assert equality across prune/reopen boundaries once those writes have settled. [VERIFIED: existing daemon retention close/reopen pattern; ForkFixture::flush/reopen; 162-CONTEXT.md D-01..D-07]

Regtest pruning starts only above h1000; the trailing keep window is 288. A tip at h1001 permits actual automatic policy and a positive h1000 checkpoint without a much longer chain. Use a short filter response range containing a deleted old block, a separate headers range containing that prefix, and checkpoint stop at h1000+; do not request 1002 filters in one response. To prove branch-sensitive checkpoints, retain a shallow fork that displaces h1000 after its initial filter was indexed. [VERIFIED: packages/open-bitcoin-node/src/sync/types.rs::prune_after_height; packages/open-bitcoin-chainstate/src/prune/range.rs; packages/bitcoin-knots/src/net_processing.cpp::PrepareBlockFilterRequest; packages/bitcoin-knots/src/index/blockfilterindex.cpp; existing peer checkpoint tests]

### Independent expected commitment provenance

Reuse `scripts/basic-filter-oracle.py::compute`: it decodes raw blocks with pinned `CBlock`, derives block hash/output scripts, accepts independently supplied historical spent scripts, and calculates BASIC bytes/hash/header. Chain predecessor expectations through `previous_name`; never feed Rust query filter/header values back as oracle inputs. Derive spent scripts from the recipe's referenced previous outputs, including same-block transaction order, and separately compare those facts with validated undo. Then feed oracle-produced commitments into existing pinned `peer_wire` serializers. [VERIFIED: scripts/basic-filter-oracle.py::compute/peer_wire/main; scripts/generate-basic-filter-vectors.ts; packages/open-bitcoin-node/src/sync/tests/filter_index/reorg/fixtures.rs]

Add a checked-in Bun entrypoint for bounded recipe/oracle I/O and reuse existing pin validation, `runBounded`, child deadline and output caps. Avoid copying the legacy substantial `bun --eval` program into new Rust code. Keep raw blocks/scripts/expected frames confined to hermetic test artifacts; exported operator/support evidence stays aggregate and redacted. [VERIFIED: scripts/generate-peer-filter-fixtures.ts::generate/runBounded; AGENTS.md Python compatibility exception; standards/core/code-shape.md; 161-CONTEXT.md D-08]

## Don't Hand-Roll

| Problem | Use instead | Why / source |
| -- | -- | -- |
| Scheduler proof | Existing `coins_flush_worker_loop` with injected waits | Tests production ordering and shutdown, not a parallel loop. [VERIFIED: coins_flush/tests/filter_index.rs] |
| Filter/wire expectations | Existing pinned BASIC oracle then pinned peer serializers | Wire-only serialization of queried Rust values cannot prove independent commitments. [VERIFIED: scripts/basic-filter-oracle.py; peer_serving/evidence.rs] |
| Prune eligibility and receipts | Existing manual planner/serialized authority and automatic flush | Existing policy owns trailing windows, reservations and paired deletion. [VERIFIED: open-bitcoin-chainstate/src/prune/; node runtime_authority/automatic_prune.rs] |
| Fault authority | Existing `FilterPublicationFault` controls at concrete writes | Existing before-coins/metadata/index boundaries avoid a fake store or second authority. [VERIFIED: storage/fjall_store/filters/publication.rs] |
| Outbound caller access | Public `sync_once` plus `TcpPeerTransport` | Reaches existing private sender through production session routing. [VERIFIED: node sync.rs; sync/session.rs; sync/tcp.rs] |

## Common Pitfalls

1. **Maturity one hidden in helper.** Daemon `params()` and `TurnHistory` customize maturity; `ForkFixture::standard` does not. New integrated recipe must assert actual network parameters and retain the accepted connect path. [VERIFIED: respective fixture implementations]
2. **Manual application without planning.** Existing daemon retention directly supplies `PrunePlan { heights: vec![20] }`; D-03 additionally requires normal planning/apply. Route integrated manual proof through actual planner/operator/RPC path and inspect deleted pairs. [VERIFIED: rpc/retention.rs; 162-CONTEXT.md D-03]
3. **Cloned handles invalidate reopen evidence.** `DaemonFixture::close` explicitly drops HTTP state/shared context/authority/store; listener and scheduler workers must also settle before reopen. Assert second open refuses while live and succeeds after closure. [VERIFIED: fixtures.rs::close; rpc/retention.rs]
4. **Checkpoint success is empty.** Current 401-height retained-peer proof can only demonstrate an empty checkpoint list. Require h1000 oracle header, correct stop ancestry and one returned checkpoint, including stale ancestry where fork displaced h1000. [VERIFIED: peer_serving/evidence.rs::expectations; 162-CONTEXT.md D-04]
5. **Volume promoted to chain evidence.** Existing `add_nonactive_volume` stores 584 approximately MiB-scale body values with modified nonce and explicitly calls them codec-only. Reuse pressure-only with bounded count and verify those keys remain nonactive/noncandidates; never claim their PoW/scripts were accepted. Stream comparisons instead of copying 550 MiB. [VERIFIED: sync/tests/filter_index/evidence/retention.rs::add_nonactive_volume/body_commitments]
6. **Coins/metadata split falsely repaired.** Existing BeforeChainMeta fault can leave changed coins B and old metadata. Correct reopen result is explicit refusal, not automatic successful recovery. Check physical coins best-block, metadata, safe index checkpoint and retained body/undo separately. [VERIFIED: sync/tests/filter_index/evidence/accepted_faults.rs; reorg/failures.rs]
7. **Historical guards removed wholesale.** Phase161 checker requires pending prose in six current docs, forbids integrated completion and requires a pending parity gap. Transition only current claim checks into explicit pending/proven-but-unaudited states; retain scope/privacy/ownership checks and test old premature and new unsupported production/shipment mutations. [VERIFIED: scripts/check-phase161-operator-index-evidence.ts::checkClaims; 162-CONTEXT.md D-11/D-15]

## Code Examples

The following existing public caller pattern permits successful outbound proof in a dependent crate without exposing private send helpers. Supply the same datadir after complete prior closure and explicit policy through the configured activation constructor. [VERIFIED: packages/open-bitcoin-node/src/sync.rs::sync_once; packages/open-bitcoin-node/src/sync/open_runtime.rs; packages/open-bitcoin-node/src/sync/tcp.rs]

```rust
// Existing APIs; scenario setup supplies explicit BASIC/serving activation.
let mut transport = open_bitcoin_node::TcpPeerTransport;
let summary = runtime.sync_once(&mut transport, timestamp)?;
// Compare bytes actually read by the bounded loopback peer with oracle frames.
// Then assert earned complete-frame receipts and released transport charges.
```

Existing concrete failure ordering is directly reusable inside node unit tests. `BeforeCoins` fails before durable coins change; `BeforeChainMeta` may change coins while metadata remains old. The test must inspect both and accept only the appropriate recoverable-prefix or refusal result. [VERIFIED: storage/fjall_store/filters/publication.rs::begin_coins_write/chain_meta_fault; sync/tests/filter_index/evidence/accepted_faults.rs]

```rust
runtime.store().set_basic_filter_fault(FilterPublicationFault::BeforeCoins);
assert!(runtime.network_handle().flush_coins(
    FlushMode::Always, FlushPolicyTime::new(50_001), u64::MAX,
).is_err());
// Assert actual coins B, metadata and safe checkpoint before and after reopen.
```

## Fault and Verification Strategy

This is an integration/fault phase, not a new test-framework installation. Nyquist-specific Validation Architecture is omitted because `workflow.nyquist_validation` is false; security is included because security enforcement is not explicitly disabled. [VERIFIED: .planning/config.json]

| Case | Existing source / required integrated observation |
| -- | -- |
| Fresh and saved/lagged body/undo loss | node `sync/tests/filter_index/startup/configured/` and daemon `tests/filter_index.rs`; clone small normal-maturity recipe variants, preserve valid immutable prefix and verify no activation/deletion mutation. |
| Pending prune intent | node `startup.rs`, `reorg/retention.rs`; establish actual intent, close, reopen through production initialize, check guard precedes resume and required mates survive refusal. |
| Cursor ahead of recovered chain | node `startup.rs`, `reorg/failures.rs`; retain ahead immutable rows, reconcile safe endpoint to coins/metadata branch or refuse explicitly. |
| Records/checkpoint/protection persistence | node `catch_up/failures.rs`, `catch_up/rpc_faults.rs`, `prune_faults.rs`; use actual before-record/checkpoint/protection and after-commit injection, inspect reopened authority and conservative protection. |
| Concrete coins/metadata persistence | node `evidence/accepted_faults.rs`, `reorg/failures.rs`; use `BeforeCoins`/`BeforeChainMeta` around ordinary flush, inspect actual Fjall coins B and metadata, no success when incompatible. |
| Corrupt persisted target/parent | node `catch_up/rpc_faults.rs`, store query tests; exact corruption must fail lookup with no invented empty result or healthy cursor. |
| Partial first/second transport frame | node `sync/tests/peer_filter_transport.rs`; RPC `inbound_listener/tests/peer_filters/failures.rs`; actual sockets earn zero or only the earlier fully written prefix, release all suffix charges. |
| Wallet/body restrictions | RPC `context/tests/rescan_eligibility.rs`; existing block request routing; call real serving/rescan entrypoints against deleted recipe body, compare saved wallet/checkpoint before/after refusal. |

Source for all fault rows: [VERIFIED: named existing test files; FilterPublicationFault definition]. Keep the success scenario long once; small fault variants may share the same recipe/version and mature spend provenance without rebuilding the >550 MiB pressure store for every fault. Record the handoff identity and distinguish those focused branches from the single integrated lifecycle. [VERIFIED: 162-CONTEXT.md D-02/D-03/D-08/D-09; existing fixtures]

Recommended plan split: (1) test recipe/oracle and privacy-safe captures; (2) daemon scheduler/startup/connect/reorg/manual/automatic/reopen scenario; (3) reopened RPC/inbound/outbound and negative wallet/body callers; (4) concrete fault/refusal matrix; (5) parity/breadcrumbs/current-doc guard transition; (6) independent review, mitigations, full native and formal lifecycle verification. This split keeps private seams in their owning crates and avoids duplicating the large pressure fixture. [VERIFIED: access boundaries and reusable tests documented above; 162-CONTEXT.md D-13]

Proposed targeted commands once new tests/checker exist (names are planning recommendations):

```bash
export PATH=/tmp/open-bitcoin-bun-1.3.9-fresh:$PATH
bun test ./scripts/check-phase162-integrated-filter-proof.test.ts
bun run scripts/check-phase162-integrated-filter-proof.ts
bun run scripts/command-timings.ts run --key phase162-node-proof -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node phase162 -- --test-threads=1
bun run scripts/command-timings.ts run --key phase162-daemon-proof -- cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-rpc --bin open-bitcoind phase162 -- --test-threads=1
bash scripts/verify.sh
```

Command shape and serial timing-wrapper contract: [VERIFIED: AGENTS.md; scripts/command-timings.ts; packages/open-bitcoin-rpc/Cargo.toml]. Keep existing Phase154–161 checker/mutation suites green. Checker tests must remove actual registered caller edges, bypass auth, replace normal maturity, substitute memory storage, erase paired-delete/reopen assertions, empty the positive checkpoint, corrupt independent expectations and insert forbidden scope claims. Structural checks supplement executed behavior; they never establish completion themselves. [VERIFIED: existing scripts/check-phase160-peer-filter-serving.ts; scripts/check-phase161-operator-index-evidence.ts; 162-CONTEXT.md D-10/D-11/D-14]

## State of the Art

| Existing evidence boundary | Phase162 required boundary |
| -- | -- |
| Constituent 401-block daemon prune/reopen and shorter outbound tests | Identifiable >1000 normal-maturity lifecycle with clients after actual reopen. [VERIFIED: rpc/retention.rs; peer_filter_transport.rs; CONTEXT D-01/D-02] |
| Independent wire serialization of Rust-produced index values | Independent raw-block/historical-script BASIC commitments, then independently serialized replies. [VERIFIED: peer_serving/evidence.rs; basic-filter-oracle.py; CONTEXT D-05] |
| Phase161 current-doc assertions that Phase162 is pending | Evidence-gated scoped completion with milestone audit/shipment still separate. [VERIFIED: check-phase161-operator-index-evidence.ts; CONTEXT D-11/D-15] |

No dependency deprecation or upstream protocol migration is proposed. [VERIFIED: 162-CONTEXT.md D-09/D-15]

## Environment Availability

| Dependency | Available | Observed version/state | Action |
| -- | -- | -- | -- |
| Rust | Yes | `rustc 1.94.1` | Use existing pinned toolchain. |
| Bun on default PATH | Wrong version | 1.4.2 vs pinned 1.3.9 | Prefix PATH with existing pinned binary directory below. |
| Pinned Bun | Yes | `/tmp/open-bitcoin-bun-1.3.9-fresh/bun` → 1.3.9 | Propagate PATH to oracle children, verifier and hooks; no installation needed. |
| Python3 | Yes | 3.14.6 | Existing oracle standard-library/pinned-helper path. |
| Bazel/Bazelisk | Present | Executables found; repo Bazel pin 8.6.0 | Native verifier owns smoke execution; no build run during research. |
| Knots submodule | Yes | `a9aee730466ac67d35a3c03ee24676be5e045878` | Existing oracle pin matches. |
| Disk | Present | 101 GiB free at probe | Measure actual integrated peak; do not turn logical target into disk guarantee. |

Environment table source: [VERIFIED: command availability/version probes, git submodule rev-parse and df during research]. Missing dependencies without fallback: none identified. [VERIFIED: those probes]

Bounded footprint plan: keep one >550 MiB pressure database at a time, stream key/length/hash observations, avoid full raw-value snapshots of the volume fixture, use a short requested filter range and separate single checkpoint reply, enforce oracle child limits and finite transport deadlines, and clean up only after workers and every store clone are gone. Record measured elapsed time, logical bytes, auxiliary row count, active deletes and reopen result; research has not benchmarked the integrated scenario and does not promise a runtime or peak-RSS figure. [VERIFIED: existing retention.rs::body_commitments/add_nonactive_volume; generate-peer-filter-fixtures.ts::runBounded; fixtures.rs::close; 162-CONTEXT.md D-03/D-12]

## Security Domain

Use ASVS 5.0.0 category names; older V2-auth/V3-session/V4-access/V5-validation/V6-crypto numbering would mislabel the current standard. These are applicable review categories, not a claim of ASVS certification or a new authentication redesign. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x15-V6-Authentication.md; https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x16-V7-Session-Management.md; https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x17-V8-Authorization.md; https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x11-V2-Validation-and-Business-Logic.md; https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x20-V11-Cryptography.md]

| Category | Application and standard existing control |
| -- | -- |
| V6 Authentication | Existing authenticated RPC; assert unauthorized request cannot reach successful filter dispatch. [VERIFIED: RPC http/filter_index.rs; http/tests/filter_index.rs] |
| V7 Session Management | Existing peer handshake and lifecycle revocation; no new browser login/session product. [VERIFIED: inbound_listener/tests/peer_filters.rs; node filter_transport/pending.rs] |
| V8 Authorization | Explicit general/per-peer BASIC grants; reserved protection cannot be weakened by operator CRUD. [VERIFIED: node runtime_authority/filter_index/serving.rs; storage/fjall_store/filters/ownership.rs] |
| V2 Validation and Business Logic | Bounds, ancestry, missing-record completeness, coins fence and ordered release. [VERIFIED: existing serving/query/publication and prune owners] |
| V1 Encoding and Sanitization | Typed bounded decoding, fixed subprocess argv/JSON, no dynamic code assembled from recipe data. [CITED: https://raw.githubusercontent.com/OWASP/ASVS/v5.0.0/5.0/en/0x10-V1-Encoding-and-Sanitization.md] |
| V11 Cryptography | Reuse existing BASIC/SHA256d/SipHash and pinned oracle; no new crypto. [VERIFIED: scripts/basic-filter-oracle.py; consensus/basic_filter source] |

| Threat | STRIDE | Verification/mitigation |
| -- | -- | -- |
| Filter progress releases required history before recoverable coins | Tampering | Concrete before-coins/metadata/checkpoint/protection faults retain safe endpoint or explicit refusal. [VERIFIED: publication.rs; accepted_faults.rs] |
| Fake completion on shortened/partial transport | Repudiation | Observe physical peer bytes and only completed-frame receipts; both directions, first/second-frame failures. [VERIFIED: peer_filter_transport.rs; peer_filters/failures.rs] |
| Oracle reproduces implementation error | Tampering | Derive independent commitments from public recipe/raw blocks, never Rust filter outputs. [VERIFIED: basic-filter-oracle.py; CONTEXT D-05] |
| Excessive fixture/serving allocation | Denial of service | One pressure DB, streaming observations, bounded child I/O and existing work/queue limits. [VERIFIED: retention.rs; generate-peer-filter-fixtures.ts; filter_transport.rs] |
| Scripts, peer identities or credentials leak through captures | Information disclosure | Hermetic oracle payloads remain test-only; shared/exported operator facts retain Phase161 redaction contract. [VERIFIED: 161-CONTEXT.md D-08; 162-CONTEXT.md D-12] |

## Assumptions Log

No factual claims rely only on training knowledge. Proposed file boundaries and commands are explicitly recommendations; integrated runtime/space remain unmeasured and must be recorded during execution. [VERIFIED: research scope and source inspection; no tests/builds executed in this research]

## Open Questions

No unresolved design question remains in this section. The following three questions are resolved by selected executable plan tasks; their acceptance still requires execution evidence and does not constitute a measured result. [VERIFIED: 162-01-PLAN.md through 162-06-PLAN.md, tasks and acceptance criteria]

1. **RESOLVED — measured-cost strategy: one serial probe, bounded reuse, execution measurement outstanding.** Run the shared normal-maturity recipe once serially before multiplying focused fixture cases; keep at most one pressure database alive. [Plan01 Task1](162-01-PLAN.md) owns the genuine accepted recipe and its nonzero `phase162_recipe` acceptance; the parent was explicitly asked to add the early serial cost-probe obligation there before execution. [Plan02 Task2](162-02-PLAN.md) executes the same actual worker scenario, records measured seed/catch-up time and requires finite elapsed turns with zero peers. [Plan03 Task2](162-03-PLAN.md) selects at most 584 auxiliary pressure rows, streaming observations, and records actual runtime, logical before/after usage, physical temporary footprint and cleanup result; its acceptance requires normal manual/automatic deletion, true reopen and bounded separately disclosed pressure. [Plan06 Task2](162-06-PLAN.md) settles the final outbound resources, removes the unique datadir and emits the aggregate measured lifecycle/cleanup record. These measurements have not been executed or observed in research; no runtime, physical-space or cleanup success is claimed. This is a pending execution verification obligation, not an unselected architecture or permission blocker. [VERIFIED: named plan/task action and acceptance criteria; research tool record; reconciliation message to parent]

2. **RESOLVED — shared recipe home and build boundary.** Use exactly `packages/testdata/phase162/recipe.rs`, `RECIPE_ID = "phase162-normal-maturity-1"`, and `include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../testdata/phase162/recipe.rs"))` in child test modules. Each containing module supplies `core_types`; the shared file depends only on that alias and `std`. [Plan01 Task1](162-01-PLAN.md) accepts genuine node receipts/coins, deterministic fork parentage and the h20 wallet output; [Plan02 Task1](162-02-PLAN.md) consumes the same recipe in the configured daemon, preserving recipe/digest/receipt provenance and accepting live-lock refusal plus all-handle closure. Current node/daemon test module roots are `#[cfg(test)]`, and current Bazel BUILD files declare production `rust_library`/`rust_binary` targets, so the shared test-only include is excluded from the present Bazel production smoke compilation. Keep that include strictly beneath those test roots. If execution introduces a Bazel `rust_test` or moves the include into a compiled production module, explicitly declare the outside-package recipe as a Bazel compile input and verify sandbox availability; do not rely on incidental host parent-directory access. No new crate or production API is selected. [VERIFIED: 162-01-PLAN.md interfaces/Task1; 162-02-PLAN.md Task1; packages/open-bitcoin-node/src/sync.rs cfg(test) mod tests; packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs cfg(test) test root; both packages' BUILD.bazel]

3. **RESOLVED — production manual planner and genuinely funded wallet.** [Plan03 Task2](162-03-PLAN.md) explicitly calls authenticated `pruneblockchain` target20 through configured HTTP dispatch and the normal planner/serialized application, then parses automatic `prune=550`, flushes ordinary automatic retention and proves both actual deleted mates absent after production reopen. Its acceptance requires real manual and automatic deletions through normal callers; hand-written `PrunePlan` is explicitly excluded. [Plan01 Task1](162-01-PLAN.md) creates the deterministic unspent h20 descriptor output in the genuinely accepted recipe. [Plan05 Task1](162-05-PLAN.md) creates/selects the actual durable named wallet and scans through the public context, accepting only normal-caller funding plus a saved prior rescan checkpoint. [Plan05 Task2](162-05-PLAN.md) invokes actual socket `getdata` and public `rescan_wallet_range` after prune/reopen, accepts successful retained filter retrieval alongside real body/wallet refusal, and compares durable wallet snapshot plus next/scanned/target checkpoint without fabricated progress (allowing only the shipped failed-job category). These tasks settle the exact caller/setup choices; sparse legacy helpers remain pattern references only. [VERIFIED: named plan/task behavior, action and acceptance criteria]

## Sources

### Primary (HIGH confidence)

- Local pinned Knots commit `a9aee730466ac67d35a3c03ee24676be5e045878`: `src/blockfilter.cpp/.h`, `src/index/base.cpp`, `src/index/blockfilterindex.cpp`, `src/node/blockstorage.cpp`, `src/net_processing.cpp`, `src/rpc/blockchain.cpp`, `src/rpc/node.cpp`, `test/functional/p2p_blockfilters.py`. [VERIFIED: local submodule pin and existing parity breadcrumbs/catalog]
- Exact first-party files and symbols cited inline: inspected fixtures, production public/private caller boundaries, effect controls, checker guards, manifest pins and environment probes. [VERIFIED: repository reads/tool probes]
- [BIP157](https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki): protocol commitment/header/request background; local pinned Knots remains behavioral authority. [CITED: https://github.com/bitcoin/bips/blob/master/bip-0157.mediawiki]
- ASVS 5.0.0 official chapter sources linked in Security Domain: category names and security review scope. [CITED: https://github.com/OWASP/ASVS/tree/v5.0.0]

### Secondary / Tertiary

None used. Context7 tools were unavailable; no new third-party library API decision required them. [VERIFIED: available tool inventory; CONTEXT D-09]

## Metadata

- **Standard stack confidence:** HIGH — exact repository pins, no dependency addition. [VERIFIED: manifests/toolchain files]
- **Architecture confidence:** HIGH — public `sync_once`/TCP and private scheduler/fault access checked in source; composition retains one datadir/recipe. [VERIFIED: caller paths above]
- **Pitfalls confidence:** HIGH — concrete existing helper limitations and checker assertions inspected. [VERIFIED: cited fixture/checker implementations]
- **Resource confidence:** MEDIUM — existing actual-volume proof is reusable, integrated normal-maturity runtime/peak allocation unmeasured. [VERIFIED: retention.rs; research execution record]
- **Validity:** Recheck whenever these source seams or lifecycle decisions change; no unbounded freshness claim. [VERIFIED: scope is checkout-specific]
- **Git:** Research written only; no staging, commit, push or lifecycle-state mutation. [VERIFIED: parent strict-wrapper instruction]
