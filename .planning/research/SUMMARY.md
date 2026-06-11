# v1.6 Mainnet Full-Sync Completion Research Summary

**Project:** Open Bitcoin
**Milestone:** v1.6 Mainnet Full-Sync Completion
**Domain:** explicit opt-in `open-bitcoind` mainnet sync-to-tip and stay-current operation
**Researched:** 2026-06-11

v1.6 should expand the shipped v1.5 unattended operator-review loop into a truthful full active-chain sync claim: `open-bitcoind` can sync to the best-known current mainnet tip, survive restart/reorg/recovery cases, and stay current through explicit opt-in operation. It should not broaden the project into production full-node serving, relay, packaging, migration apply mode, GUI, or production-funds wallet scope.

## Stack additions

- Keep the current first-party Rust `1.94.1` workspace, Bazel/Bzlmod smoke build, Bun-backed TypeScript automation, Fjall durable storage, and existing CLI/RPC/dashboard/support surfaces.
- Add no new third-party dependencies by default. A narrow reuse of the already-pinned Tokio stack is acceptable only for shell-level sync orchestration if worker-thread scheduling is insufficient.
- Evolve first-party storage and sync internals: record-keyed UTXO, undo, active-chain, header, block-index, runtime, and bounded metrics keyspaces; schema-versioned recovery; and optional Open Bitcoin-owned segmented block/undo files if Fjall large-value profiling requires it.
- Continue avoiding existing Rust Bitcoin libraries in the production path. Consensus, chainstate, networking, wallet, primitives, and codec behavior remain owned by this repository.

## Feature table stakes

| Capability | Requirement implication |
| --- | --- |
| Full active-chain sync to best-known mainnet tip | Track validated headers, downloaded blocks, connected active-chain height/hash, cumulative work, and tip freshness as one coherent state. |
| Mainnet-scale validation and connect | Do not count progress as synced until blocks are consensus-validated and connected through durable chainstate or UTXO state. |
| Stay-current behavior after catch-up | Continue headers/block polling, peer rotation, and new-block connect after initial IBD completion. |
| Restart-safe long sync | Reopen the same datadir, classify clean versus unclean shutdown, clear stale in-flight work, and resume without duplicate connects. |
| Reorg-aware active chain | Select by cumulative work, persist undo data, disconnect/reconnect safely, and expose bounded reorg evidence. |
| Peer health and anti-stall recovery | Rotate slow, stale, incompatible, malformed, invalid, or `notfound` peers with typed attribution and retry/backoff. |
| Bounded resources | Define and test bounds for peers, in-flight blocks, queues, caches, storage writes, logs, metrics, and support bundles. |
| Truth-aligned operator evidence | Share one sync truth contract across status, dashboard, RPC, logs, metrics, live-smoke reports, and support bundles. |
| Opt-in public-mainnet UAT | Provide copy-pasteable Cargo and Bazel commands for full-sync, stay-current, restart/resume, and support-bundle evidence outside default verification. |

## Architecture direction

- Treat v1.6 as a hardening layer on the existing `DurableSyncRuntime`, not a new daemon or stack replacement.
- Keep functional core / imperative shell boundaries: pure crates own validation, chain selection, chainstate transitions, header logic, and deterministic peer planning; node/RPC/CLI shells own sockets, clocks, storage, service controls, logs, metrics, and operator evidence.
- Introduce typed models for best-known tip evidence, full-sync completion state, durable work queue or block-state projection, scheduler phases, and stay-current status.
- Replace whole-snapshot critical sync persistence with incremental, record-keyed chainstate and block metadata. Connected-block reporting should require active-chain position, UTXO deltas, undo data, block status, and runtime tip projection to be durably recoverable.
- Split growing sync code into focused modules as behavior lands, such as scheduler, work queue, tip evidence, connect pipeline, steady state, and evidence, while keeping `DurableSyncRuntime` as the facade.

## Watch-outs

- Do not report headers-only or downloaded-only progress as full sync. The claim requires connected active-chain state.
- Do not treat persisted height/hash as a complete chainstate unless a fresh process can reopen persisted UTXO/undo state and validate additional blocks.
- Avoid linear-only IBD assumptions. Reorgs, competing headers, late blocks, and peer disagreement are normal mainnet behavior.
- Make "current tip" evidence precise. Without an external oracle, the safe claim is connected to the best-known validated tip observed from compatible peers.
- Distinguish no-progress while behind from steady no-progress at tip. This belongs in the scheduler/status model, not in renderers.
- Address storage growth, compaction, schema mismatch, lock contention, corruption markers, and operator-safe repair guidance before sync-to-tip is claimed.
- Keep default verification deterministic and public-network-free.
- Guard docs, status, and support wording against broad production-node, relay, wallet, migration, packaging, or hosted-dashboard implications.

## Suggested requirement categories

- **Full active-chain validation and persistence:** sync to best-known tip through validated headers, connected blocks, cumulative work, durable UTXO/chainstate, and status that separates headers, downloaded, connected, validated, current, stale, and recovering.
- **Tip tracking and stay-current operation:** define tip freshness semantics and report stable height/hash/work/time evidence, peer disagreement, stale-tip states, and new-block follow behavior after catch-up.
- **Reorg, peer rotation, and no-progress recovery:** prove cumulative-work selection, durable disconnect/reconnect, stale in-flight cleanup, peer contribution attribution, retry/backoff, and typed blocker guidance.
- **Resource bounds and durable restart/resume:** enforce bounds for long sync attempts and prove same-datadir restart through clean shutdown, unclean shutdown, mid-download, mid-connect, reorg, schema mismatch, corruption, lock contention, and low-disk cases.
- **Operator observability and support evidence:** keep CLI, dashboard, RPC, metrics, logs, live UAT reports, and support bundles on one compact full-sync truth contract.
- **Opt-in UAT and deterministic verification:** keep `bash scripts/verify.sh` hermetic, add deterministic synthetic coverage, and document repo-local Cargo/Bazel public-mainnet UAT commands.
- **Release boundaries, parity, and documentation:** refresh parity roots, threat model, readiness docs, README/operator docs, and deterministic boundary checks for the narrower v1.6 claim.

## Deferred/out-of-scope

- Inbound serving, address advertisement, transaction relay, mempool propagation, compact block relay, block serving, and production full-node claims.
- Production-funds wallet use or expanded wallet safety claims.
- Migration apply mode, source datadir mutation, automatic Core/Knots cutover, service disablement, wallet import, or destructive migration behavior.
- Signed packaging, broad distribution polish, Windows service support, hosted dashboards, Qt or desktop GUI work.
- Public-network checks in `bash scripts/verify.sh`, checked-in live-mainnet reports, timing-threshold release gates, centralized trusted peers, hidden tip oracles, pruning, assumeutxo, assumevalid, or snapshot bootstrap.

## Verification implications

- Default verification remains `bash scripts/verify.sh` with deterministic formatting, linting, tests, architecture checks, panic-site checks, parity breadcrumbs, benchmark smoke/report validation, and Bazel smoke builds. It must not require internet access, public peers, real service managers, long-running sync, or current-tip timing.
- Add hermetic tests for durable UTXO and undo writes, block connect/disconnect/reorg across restart, best-chain header selection, duplicate/malformed/invalid/notfound block responses, schema mismatch, corruption, lock contention, repair/reindex guidance, crash recovery, no duplicate connects, resource bounds, and long synthetic chain sync.
- Extend parity fixtures and compatibility harnesses for new validation and P2P behavior. New first-party Rust source or test files need parity breadcrumb entries.
- Add opt-in public-mainnet UAT that writes local redacted JSON/Markdown evidence outside git: initial and final tip, tip freshness, connected height/hash/work, restart/resume checkpoints, stay-current window, peer contribution, reorg/no-progress events, resource pressure, and final verdict.
- Add deterministic v1.6 release-boundary checks so docs and parity roots cannot imply deferred production-node, inbound, relay, wallet, migration, packaging, GUI, hosted-dashboard, or public-network default-verification scope.

## Sources

- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`
- `.planning/PROJECT.md`

---
*Research completed: 2026-06-11*
*Ready for requirements and roadmap drafting: yes*
