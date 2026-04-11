# Roadmap: Open Bitcoin

## Overview

The roadmap starts by pinning the reference baseline and enforcing workspace and architecture guardrails, then builds the typed Bitcoin core and consensus engine before layering chainstate, mempool policy, networking, wallet behavior, and operator-facing interfaces on top. It ends by hardening the project with cross-implementation parity suites, fuzzing, benchmarks, and audit artifacts so parity claims stay defensible instead of aspirational.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Workspace, Baseline, and Guardrails** - Pin the reference, bootstrap the workspace, and install architecture enforcement. (completed 2026-04-11)
- [ ] **Phase 2: Core Domain and Serialization Foundations** - Build the typed Bitcoin libraries that every later phase depends on.
- [ ] **Phase 3: Consensus Validation Engine** - Match baseline script, transaction, and block validation behavior.
- [ ] **Phase 4: Chainstate and UTXO Engine** - Implement baseline-compatible chainstate, UTXO tracking, and reorg handling.
- [ ] **Phase 5: Mempool and Node Policy** - Match mempool policy, replacement, and eviction behavior.
- [ ] **Phase 6: P2P Networking and Sync** - Add peer lifecycle, message handling, and sync behavior.
- [ ] **Phase 7: Wallet Core and Adapters** - Implement headless wallet behavior with pure-core boundaries intact.
- [ ] **Phase 8: RPC, CLI, and Config Parity** - Expose node and wallet behavior through compatible operator interfaces.
- [ ] **Phase 9: Parity Harnesses and Fuzzing** - Lock down external behavior with reusable black-box and fuzz/property suites.
- [ ] **Phase 10: Benchmarks and Audit Readiness** - Measure performance and complete the audit surfaces that track parity status.

## Phase Details

### Phase 1: Workspace, Baseline, and Guardrails
**Goal**: Establish the pinned Knots baseline, top-level workspace tooling, and the verification and architecture guardrails that all later implementation work will rely on.
**Depends on**: Nothing (first phase)
**Requirements**: REF-01, REF-02, ARCH-01, ARCH-02, ARCH-04, VER-01, VER-02
**Success Criteria** (what must be TRUE):
  1. The repository contains a vendored, pinned Knots baseline under `packages/` and first-party workspace targets can run from the repo root.
  2. Verification fails when pure-core code imports forbidden I/O or runtime effects.
  3. Contributors have a repo-native verification entrypoint that covers format, lint, build, tests, coverage, and architecture-policy checks.
  4. The production path is fenced to first-party Rust Bitcoin crates rather than third-party Rust Bitcoin libraries.
**Plans**: 4 plans

Plans:
- [x] 01-01: Vendor the Knots baseline and define first-party package layout under the workspace.
- [x] 01-02: Bootstrap Bazelisk, Bazel/Bzlmod, and top-level targets for first-party packages.
- [x] 01-03: Implement verification entrypoints, coverage checks, and architecture-policy enforcement.
- [x] 01-04: Add the initial deviation ledger and contributor workflow documentation.

### Phase 2: Core Domain and Serialization Foundations
**Goal**: Build the strongly typed Bitcoin domain libraries and parsing/serialization layer that later consensus, chainstate, networking, and wallet work will reuse.
**Depends on**: Phase 1
**Requirements**: REF-03, ARCH-03, CONS-01
**Success Criteria** (what must be TRUE):
  1. First-party crates expose typed primitives for hashes, amounts, scripts, transactions, blocks, headers, and network payloads.
  2. Raw Bitcoin inputs parse into invariant-bearing domain types instead of leaking primitive validation throughout the codebase.
  3. In-scope serialization and parsing behavior matches the pinned baseline on shared fixtures.
  4. The reference feature catalog is seeded enough to guide later parity work by subsystem.
**Plans**: 4 plans

Plans:
- [ ] 02-01: Define crate boundaries, domain newtypes, and shared invariants for core Bitcoin data.
- [ ] 02-02: Implement parsing and serialization for primitives, scripts, transactions, blocks, and messages.
- [ ] 02-03: Add pure-core fixtures and coverage-driven unit suites for the new libraries.
- [ ] 02-04: Seed the living reference catalog with subsystem, quirk, and unknown tracking.

### Phase 3: Consensus Validation Engine
**Goal**: Implement script, transaction, and block validation behavior that matches the pinned baseline for consensus-critical decisions.
**Depends on**: Phase 2
**Requirements**: CONS-02, CONS-03
**Success Criteria** (what must be TRUE):
  1. Consensus-valid and consensus-invalid fixtures resolve the same way as Knots for scripts, transactions, and blocks.
  2. Validation errors are explicit typed outcomes instead of ad-hoc strings or hidden panics.
  3. Automated parity fixtures block merges when Open Bitcoin and Knots disagree on consensus decisions.
**Plans**: 4 plans

Plans:
- [ ] 03-01: Implement the script engine and opcode evaluation model.
- [ ] 03-02: Add transaction validation rules and typed error outcomes.
- [ ] 03-03: Add block validation and block-level consensus rule enforcement.
- [ ] 03-04: Build consensus comparison fixtures against the Knots baseline.

### Phase 4: Chainstate and UTXO Engine
**Goal**: Add baseline-compatible chainstate, UTXO management, block connect/disconnect, and reorg behavior with persistence isolated to adapters.
**Depends on**: Phase 3
**Requirements**: CHAIN-01
**Success Criteria** (what must be TRUE):
  1. Block connect and disconnect logic produce the same chain tip and UTXO outcomes as Knots on targeted fixtures.
  2. Reorg scenarios converge on the same best chain and spendable state as the baseline.
  3. Storage concerns stay outside the pure chainstate core.
**Plans**: 3 plans

Plans:
- [ ] 04-01: Define the chainstate and UTXO data model in the pure core.
- [ ] 04-02: Implement connect, disconnect, and reorg behavior.
- [ ] 04-03: Add storage adapters and chainstate parity fixtures.

### Phase 5: Mempool and Node Policy
**Goal**: Implement mempool state and node policy behavior that matches baseline admission, replacement, and eviction decisions.
**Depends on**: Phase 4
**Requirements**: MEM-01, MEM-02
**Success Criteria** (what must be TRUE):
  1. Mempool admission, replacement, and eviction decisions match Knots on targeted policy fixtures.
  2. Ancestor, descendant, and fee-related policy outcomes are reproducible and tested.
  3. Any intentional policy divergence is logged explicitly instead of drifting silently.
**Plans**: 3 plans

Plans:
- [ ] 05-01: Implement mempool admission and replacement rules.
- [ ] 05-02: Implement fee, ancestor, descendant, and eviction policy accounting.
- [ ] 05-03: Add policy parity tests and deviation tracking hooks.

### Phase 6: P2P Networking and Sync
**Goal**: Implement the peer manager, wire protocol handling, and sync flows needed for baseline-compatible networking behavior.
**Depends on**: Phase 5
**Requirements**: P2P-01, P2P-02
**Success Criteria** (what must be TRUE):
  1. The node completes peer handshake and lifecycle flows compatibly with Knots peers.
  2. Header and block sync reaches the same chain state on controlled fixtures.
  3. Inventory and transaction relay behavior match the baseline on targeted cases.
  4. Networking behavior is validated across isolated multi-node test runs.
**Plans**: 4 plans

Plans:
- [ ] 06-01: Implement peer lifecycle, version handshake, and connection management.
- [ ] 06-02: Implement header download, block sync, and peer selection behavior.
- [ ] 06-03: Implement inventory, transaction relay, and message-level policy handling.
- [ ] 06-04: Add hermetic multi-node fixtures for networking parity.

### Phase 7: Wallet Core and Adapters
**Goal**: Implement headless wallet behavior that matches the in-scope baseline while keeping state transitions pure and persistence adapter-owned.
**Depends on**: Phase 6
**Requirements**: WAL-01, WAL-02, WAL-03
**Success Criteria** (what must be TRUE):
  1. The wallet manages keys, descriptors, and addresses for the in-scope baseline behavior.
  2. Wallet balance, UTXO, coin-selection, and transaction-building fixtures match Knots results.
  3. Signing, persistence, and recovery flows work without leaking direct I/O into the wallet core.
  4. Wallet behavior is validated through headless functional tests.
**Plans**: 4 plans

Plans:
- [ ] 07-01: Implement wallet domain types, descriptor handling, and key/address management.
- [ ] 07-02: Implement balance tracking, UTXO views, coin selection, and transaction building.
- [ ] 07-03: Implement signing, persistence adapters, and recovery flows.
- [ ] 07-04: Add wallet parity fixtures and functional test coverage.

### Phase 8: RPC, CLI, and Config Parity
**Goal**: Expose the node and wallet through operator-facing interfaces that behave compatibly with the baseline for the in-scope surface.
**Depends on**: Phase 7
**Requirements**: RPC-01, CLI-01, CLI-02
**Success Criteria** (what must be TRUE):
  1. In-scope RPC methods return compatible payloads and error semantics.
  2. CLI flags, config parsing, and precedence rules match the baseline for the supported surface.
  3. Operators can run node and wallet workflows entirely through CLI and RPC without any GUI dependency.
**Plans**: 3 plans

Plans:
- [ ] 08-01: Implement RPC method surfaces and typed request/response mapping.
- [ ] 08-02: Implement CLI commands, config-file parsing, and option precedence.
- [ ] 08-03: Add end-to-end operator flows for headless node and wallet control.

### Phase 9: Parity Harnesses and Fuzzing
**Goal**: Build reusable verification systems that compare Open Bitcoin against Knots, isolate integration runs, and stress critical protocol boundaries.
**Depends on**: Phase 8
**Requirements**: VER-03, VER-04, PAR-01
**Success Criteria** (what must be TRUE):
  1. The same black-box functional suite can target Knots and Open Bitcoin without test rewrites.
  2. Integration tests run in parallel without port, process, or data-directory collisions.
  3. Fuzzing or property-style tests cover parser, serialization, and protocol surfaces with meaningful risk reduction.
  4. CI reports parity and coverage outcomes clearly enough to block regressions.
**Plans**: 4 plans

Plans:
- [ ] 09-01: Build the cross-implementation black-box functional test harness.
- [ ] 09-02: Implement process, port, and data-dir isolation for parallel-safe integration runs.
- [ ] 09-03: Add fuzz and property suites for parser, serialization, and protocol boundaries.
- [ ] 09-04: Surface parity, coverage, and harness results in CI.

### Phase 10: Benchmarks and Audit Readiness
**Goal**: Add performance measurements and audit artifacts that make parity status, deviations, and readiness visible at a glance.
**Depends on**: Phase 9
**Requirements**: PAR-02, AUD-01
**Success Criteria** (what must be TRUE):
  1. Benchmarks measure critical node and wallet paths and compare to Knots where meaningful.
  2. The parity checklist reports status for every in-scope surface.
  3. Audit artifacts make remaining deviations, unknowns, and milestone readiness easy to review before execution and release decisions.
**Plans**: 3 plans

Plans:
- [ ] 10-01: Add benchmark suites for critical consensus, chainstate, networking, and wallet paths.
- [ ] 10-02: Complete the parity checklist and deviation audit artifacts for all in-scope surfaces.
- [ ] 10-03: Produce release-readiness and milestone handoff documentation.

## Progress

**Execution Order:**
Phases execute in numeric order: 2 → 2.1 → 2.2 → 3 → 3.1 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Workspace, Baseline, and Guardrails | 4/4 | Complete   | 2026-04-11 |
| 2. Core Domain and Serialization Foundations | 0/4 | Not started | - |
| 3. Consensus Validation Engine | 0/4 | Not started | - |
| 4. Chainstate and UTXO Engine | 0/3 | Not started | - |
| 5. Mempool and Node Policy | 0/3 | Not started | - |
| 6. P2P Networking and Sync | 0/4 | Not started | - |
| 7. Wallet Core and Adapters | 0/4 | Not started | - |
| 8. RPC, CLI, and Config Parity | 0/3 | Not started | - |
| 9. Parity Harnesses and Fuzzing | 0/4 | Not started | - |
| 10. Benchmarks and Audit Readiness | 0/3 | Not started | - |
