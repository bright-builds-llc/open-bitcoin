# Roadmap: Open Bitcoin

## Overview

The roadmap starts by pinning the reference baseline and enforcing workspace and architecture guardrails, then builds the typed Bitcoin core and consensus engine before layering chainstate, mempool policy, networking, wallet behavior, and operator-facing interfaces on top. It ends by hardening the project with cross-implementation parity suites, fuzzing, benchmarks, and audit artifacts so parity claims stay defensible instead of aspirational.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2, 3.1): Inserted or split follow-on work

Decimal phases appear between their surrounding integers in numeric order.

- [x] **Phase 1: Workspace, Baseline, and Guardrails** - Pin the reference, bootstrap the workspace, and install architecture enforcement. (completed 2026-04-11)
- [x] **Phase 2: Core Domain and Serialization Foundations** - Build the typed Bitcoin libraries that every later phase depends on. (completed 2026-04-11)
- [x] **Phase 3: Consensus Validation Foundation** - Establish the pure-core consensus foundation, contextual validation framework, and initial signature scaffolding. (completed 2026-04-11)
- [x] **Phase 3.1: Legacy Signature Execution (INSERTED)** - Finish legacy CHECKSIG/CHECKMULTISIG execution and P2PKH. (completed 2026-04-11)
- [x] **Phase 3.2: P2SH and Segwit-v0 Execution (INSERTED)** - Add P2SH, segwit-v0 spending paths, and split sigop accounting. (completed 2026-04-12)
- [x] **Phase 3.3: Taproot and Tapscript Execution (INSERTED)** - Add taproot key-path, tapscript script-path, and taproot flag enforcement. (completed 2026-04-12)
- [x] **Phase 3.4: Consensus Parity Closure (INSERTED)** - Complete the parity corpus and close the consensus lifecycle honestly. (completed 2026-04-12)
- [x] **Phase 4: Chainstate and UTXO Engine** - Implement baseline-compatible chainstate, UTXO tracking, and reorg handling. (completed 2026-04-12)
- [x] **Phase 5: Mempool and Node Policy** - Match mempool policy, replacement, and eviction behavior. (completed 2026-04-13)
- [x] **Phase 6: P2P Networking and Sync** - Add peer lifecycle, message handling, and sync behavior. (completed 2026-04-14)
- [x] **Phase 7: Wallet Core and Adapters** - Implement headless wallet behavior with pure-core boundaries intact. (completed 2026-04-17)
- [x] **Phase 07.1: Codebase Maintainability Refactor Wave (INSERTED)** - Reduce oversized-file pressure before Phase 8 by extracting inline tests and splitting the two largest remaining Rust hotspots. (completed 2026-04-18)
- [x] **Phase 07.2: Protocol Constant Clarity Cleanup (INSERTED)** - Replace remaining protocol-significant magic numbers and repeated serialized-size literals with clearer named constants before Phase 8. (completed 2026-04-19)
- [x] **Phase 07.3: Reduce nesting with early returns (INSERTED)** - Flatten the highest-value deeply nested control-flow hotspots before Phase 8 planning. (completed 2026-04-19)
- [x] **Phase 07.4: Sweep the codebase for let-else opportunities (INSERTED)** - Replace eligible Rust control-flow scaffolding with `let ... else` where it reduces nesting and improves readability before Phase 8 planning. (completed 2026-04-20)
- [ ] **Phase 07.5: Fix consensus parity gaps in contextual header validation and lax DER signature verification (INSERTED)** - Close the known consensus parity gaps before Phase 8 builds new operator interfaces on top of them.
- [ ] **Phase 07.6: Enforce coinbase subsidy-plus-fees limits on the consensus and active chainstate paths (INSERTED)** - Close the remaining coinbase reward-limit acceptance gap before Phase 8 builds operator interfaces on top of the current block-connect surface.
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
- [x] 02-01: Define crate boundaries, domain newtypes, and shared invariants for core Bitcoin data.
- [x] 02-02: Implement parsing and serialization for primitives, scripts, transactions, blocks, and messages.
- [x] 02-03: Add pure-core fixtures and coverage-driven unit suites for the new libraries.
- [x] 02-04: Seed the living reference catalog with subsystem, quirk, and unknown tracking.

### Phase 3: Consensus Validation Foundation
**Goal**: Establish the pure-core consensus foundation that later signature, witness, and taproot execution work will build on.
**Depends on**: Phase 2
**Requirements**: CONS-02
**Success Criteria** (what must be TRUE):
  1. The workspace exposes a pure-core consensus crate with deterministic hashing, typed validation errors, and repo-native dependency wiring for future signature verification.
  2. Context-free and contextual transaction and block validation are available through explicit context types instead of chainstate coupling.
  3. Witness commitment, coinbase-height, block-weight, and sequence-lock or finality helpers are present and covered by pure-core tests.
  4. Script classification, sighash, and signature-verification scaffolding exists, and the first legacy spending-path verification for pay-to-pubkey and bare multisig is working.
**Plans**: 7 plans

Plans:
- [x] 03-01: Implement the script engine and opcode evaluation model.
- [x] 03-02: Add transaction validation rules and typed error outcomes.
- [x] 03-03: Add block validation and block-level consensus rule enforcement.
- [x] 03-04: Build consensus comparison fixtures against the Knots baseline.
- [x] 03-05: Add contextual validation contexts and foundation helpers.
- [x] 03-06: Add `crate_universe`, `secp256k1`, and the classification or sighash or signature core.
- [x] 03-07: Add the first legacy spending-path verification for pay-to-pubkey and bare multisig.

### Phase 3.1: Legacy Signature Execution (INSERTED)
**Goal**: Finish legacy CHECKSIG/CHECKMULTISIG execution, hash-type enforcement, and P2PKH spending-path support.
**Depends on**: Phase 3
**Requirements**: CONS-02
**Success Criteria** (what must be TRUE):
  1. Legacy `CHECKSIG`, `CHECKSIGVERIFY`, `CHECKMULTISIG`, and `CHECKMULTISIGVERIFY` execute with the same acceptance or failure outcomes as Knots on targeted fixtures.
  2. DER, low-S, NULLDUMMY, NULLFAIL, SIGPUSHONLY, and legacy sighash-byte handling are enforced in the correct flag-gated paths.
  3. P2PKH spending-path verification works through the canonical transaction-aware verifier.
**Plans**: 3 plans

Plans:
- [x] 03.1-01: Finish legacy CHECKSIG/CHECKMULTISIG VM execution.
- [x] 03.1-02: Add P2PKH spending-path execution.
- [x] 03.1-03: Port legacy signature and sighash vectors.

### Phase 3.2: P2SH and Segwit-v0 Execution (INSERTED)
**Goal**: Add P2SH, segwit-v0 spending paths, witness invariants, and split sigop accounting.
**Depends on**: Phase 3.1
**Requirements**: CONS-02
**Success Criteria** (what must be TRUE):
  1. P2SH redeem scripts, native P2WPKH/P2WSH, and nested segwit-v0 paths execute compatibly with the baseline on targeted fixtures.
  2. CLEANSTACK, MINIMALIF, WITNESS_PUBKEYTYPE, witness mismatch, and unexpected witness handling match Knots for the supported segwit-v0 surface.
  3. Sigop accounting is split across legacy, P2SH, and witness paths instead of legacy-only counting.
**Plans**: 3 plans

Plans:
- [x] 03.2-01: Implement P2SH redeem-script execution.
- [x] 03.2-02: Implement native and nested segwit-v0 spending paths.
- [x] 03.2-03: Replace legacy-only sigop counting with legacy + P2SH + witness accounting.

### Phase 3.3: Taproot and Tapscript Execution (INSERTED)
**Goal**: Add taproot key-path, tapscript script-path, and taproot flag enforcement.
**Depends on**: Phase 3.2
**Requirements**: CONS-02
**Success Criteria** (what must be TRUE):
  1. Taproot key-path verification works with Schnorr signatures and taproot sighash semantics.
  2. Tapscript script-path execution handles control blocks, tapleaf hashing, annexes, CODESEPARATOR tracking, and CHECKSIGADD.
  3. TAPROOT and related upgrade/discouragement flags are enforced in the same places as the baseline for the supported surface.
**Plans**: 3 plans

Plans:
- [x] 03.3-01: Implement taproot key-path verification.
- [x] 03.3-02: Implement tapscript script-path execution.
- [x] 03.3-03: Enforce taproot policy/upgrade flags and validation-weight rules.

### Phase 3.4: Consensus Parity Closure (INSERTED)
**Goal**: Complete the remaining parity corpus and close the consensus lifecycle honestly.
**Depends on**: Phase 3.3
**Requirements**: CONS-03
**Success Criteria** (what must be TRUE):
  1. The pure-core parity suite covers the remaining signature, P2SH, segwit, and taproot surface that Phase 3 now owns.
  2. Repo-owned deterministic fixtures cover maturity, locktime, sequence-lock, witness-commitment, unexpected-witness, and taproot regression cases not cleanly imported from upstream.
  3. Verification, parity docs, and milestone bookkeeping can all move the consensus surface to done without masking residual gaps.
**Plans**: 3 plans

Plans:
- [x] 03.4-01: Port the remaining upstream multisig, P2SH, segwit, and sighash fixtures.
- [x] 03.4-02: Add repo-owned deterministic consensus regression fixtures.
- [x] 03.4-03: Regenerate verification, parity ledger, and milestone bookkeeping for consensus closure.

### Phase 4: Chainstate and UTXO Engine
**Goal**: Add baseline-compatible chainstate, UTXO management, block connect/disconnect, and reorg behavior with persistence isolated to adapters.
**Depends on**: Phase 3.4
**Requirements**: CHAIN-01
**Success Criteria** (what must be TRUE):
  1. Block connect and disconnect logic produce the same chain tip and UTXO outcomes as Knots on targeted fixtures.
  2. Reorg scenarios converge on the same best chain and spendable state as the baseline.
  3. Storage concerns stay outside the pure chainstate core.
**Plans**: 3 plans

Plans:
- [x] 04-01: Define the chainstate and UTXO data model in the pure core.
- [x] 04-02: Implement connect, disconnect, and reorg behavior.
- [x] 04-03: Add storage adapters and chainstate parity fixtures.

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
- [x] 05-01: Implement mempool admission and replacement rules.
- [x] 05-02: Implement fee, ancestor, descendant, and eviction policy accounting.
- [x] 05-03: Add policy parity tests and deviation tracking hooks.

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
- [x] 06-01: Implement peer lifecycle, version handshake, and connection management.
- [x] 06-02: Implement header download, block sync, and peer selection behavior.
- [x] 06-03: Implement inventory, transaction relay, and message-level policy handling.
- [x] 06-04: Add hermetic multi-node fixtures for networking parity.

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
- [x] 07-01: Implement wallet domain types, descriptor handling, and key/address management.
- [x] 07-02: Implement balance tracking, UTXO views, coin selection, and transaction building.
- [x] 07-03: Implement signing, persistence adapters, and recovery flows.
- [x] 07-04: Add wallet parity fixtures and functional test coverage.

### Phase 07.1: Codebase Maintainability Refactor Wave (INSERTED)

**Goal:** Reduce oversized-file pressure before Phase 8 by moving inline unit
suites out of production modules and splitting the two largest remaining
first-party Rust hotspots along coherent module boundaries.
**Requirements**: TBD
**Depends on:** Phase 7
**Success Criteria** (what must be TRUE):
  1. Oversized first-party Rust files with inline `mod tests` blocks move those
     tests into sibling Rust test modules without changing crate behavior.
  2. `packages/open-bitcoin-wallet/src/wallet.rs` is split into coherent
     submodules while preserving the wallet crate's public surface and
     deterministic behavior.
  3. `packages/open-bitcoin-consensus/src/script.rs` is split into coherent
     submodules while preserving verifier, witness, taproot, and sigop
     behavior on the existing consensus corpus.
  4. `bash scripts/verify.sh` passes after the refactor wave completes.
**Plans:** 3/3 plans complete

Plans:
- [x] 07.1-01: Extract inline tests from oversized first-party Rust files.
- [x] 07.1-02: Refactor wallet module boundaries under `open-bitcoin-wallet`.
- [x] 07.1-03: Refactor consensus script engine module boundaries.

### Phase 07.2: Protocol Constant Clarity Cleanup (INSERTED)

**Goal:** Replace the remaining audited protocol-significant magic numbers and
repeated serialized-size literals with clearer named constants while preserving
consensus and networking behavior.
**Requirements**: TBD
**Depends on:** Phase 07.1
**Plans:** 1/1 plans complete

Plans:
- [x] 07.2-01: Replace audited protocol constants and shared wire-size literals without changing behavior.

### Phase 07.3: Reduce nesting with early returns (INSERTED)

**Goal:** Flatten the highest-value deeply nested control-flow hotspots in the existing consensus, chainstate, mempool, and networking production code before Phase 8 planning, using early returns and narrow helper extraction without changing behavior.
**Requirements**: TBD
**Depends on:** Phase 07.2
**Plans:** 3/3 plans complete

Plans:
- [x] 07.3-01-PLAN.md — Flatten block and chainstate validation hotspots with guard-style helper extraction.
- [x] 07.3-02-PLAN.md — Flatten peer and mempool guard paths without rewriting protocol or policy state machines.
- [x] 07.3-03-PLAN.md — Apply the optional narrow `legacy.rs` cleanup and close the phase with full repo-native verification.

### Phase 07.4: Sweep the codebase for let-else opportunities (INSERTED)

**Goal:** Sweep the codebase for Rust paths where `let ... else` reduces nesting and improves readability without changing behavior, focusing on the highest-value production hotspots before Phase 8 planning.
**Requirements**: None declared (maintainability-only insertion)
**Depends on:** Phase 07.3
**Plans:** 1/1 plans complete

Plans:
- [x] 07.4-01-PLAN.md — Audit the shortlist, convert only the real consensus guard wins, and close the sweep with repo-native verification.

### Phase 07.5: Fix consensus parity gaps in contextual header validation and lax DER signature verification (INSERTED)

**Goal:** Close the two reviewed consensus parity gaps so the active contextual block-validation and non-strict legacy signature-verification paths match the pinned Knots baseline before Phase 8 adds operator interfaces on top.
**Requirements**: CONS-02, CONS-03, VER-01, VER-02
**Depends on:** Phase 07.4
**Plans:** 4 plans

Plans:
- [ ] 07.5-01-PLAN.md — Restore contextual header parity and thread explicit `current_time` through the active block-connect path.
- [ ] 07.5-02-PLAN.md — Restore non-strict lax DER legacy signature parity and close the phase with repo-native verification evidence.
- [ ] 07.5-03-PLAN.md — Close the remaining retarget-boundary CR-01 gap and correct the premature closeout evidence.
- [ ] 07.5-04-PLAN.md — Close the remaining non-boundary min-difficulty recovery gap and refresh truthful Phase 07.5 closeout evidence.

### Phase 07.6: Enforce coinbase subsidy-plus-fees limits on the consensus and active chainstate paths (INSERTED)

**Goal:** Mirror Knots-style coinbase subsidy-plus-fees reward-limit enforcement on both the pure-core contextual block-validation path and the active chainstate connect path, with focused proof that overpaying coinbases fail before live state commit.
**Requirements**: CONS-02, CONS-03, VER-01, VER-02
**Depends on:** Phase 07.5
**Success Criteria** (what must be TRUE):
  1. `validate_block_with_context()` rejects coinbases whose outputs exceed `subsidy(height) + total_fees` with `bad-cb-amount`.
  2. `connect_block_with_current_time()` applies the same reward-limit rule before committing `self.utxos`, `self.undo_by_block`, or `self.active_chain`.
  3. Focused consensus and chainstate regressions prove exact-fee acceptance, `+1` overpay rejection, and unchanged live state on failure.
  4. `bash scripts/verify.sh` passes, and the Phase 07.6 summary cites the new proof strings without reopening already-green Phase 07.5 evidence.
**Plans:** 3 plans

Plans:
- [ ] 07.6-01-PLAN.md — Add the shared consensus reward-limit contract and pure-core regressions.
- [ ] 07.6-02-PLAN.md — Wire pre-commit chainstate enforcement and close the reward-limit proof chain.
- [ ] 07.6-03-PLAN.md — Close the remaining `MAX_MONEY` / MoneyRange fee-boundary gap in consensus and chainstate.

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
Phases execute in numeric order: 2 → 2.1 → 2.2 → 3 → 3.1 → 3.2 → 3.3 → 3.4 → 4 → 5

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Workspace, Baseline, and Guardrails | 4/4 | Complete    | 2026-04-11 |
| 2. Core Domain and Serialization Foundations | 4/4 | Complete    | 2026-04-11 |
| 3. Consensus Validation Foundation | 7/7 | Complete    | 2026-04-11 |
| 3.1. Legacy Signature Execution | 3/3 | Complete | 2026-04-11 |
| 3.2. P2SH and Segwit-v0 Execution | 3/3 | Complete | 2026-04-12 |
| 3.3. Taproot and Tapscript Execution | 3/3 | Complete | 2026-04-12 |
| 3.4. Consensus Parity Closure | 3/3 | Complete | 2026-04-12 |
| 4. Chainstate and UTXO Engine | 3/3 | Complete | 2026-04-12 |
| 5. Mempool and Node Policy | 3/3 | Complete | 2026-04-13 |
| 6. P2P Networking and Sync | 0/4 | Not started | - |
| 7. Wallet Core and Adapters | 0/4 | Not started | - |
| 7.1. Codebase Maintainability Refactor Wave | 0/3 | Not started | - |
| 7.2. Protocol Constant Clarity Cleanup | 1/1 | Complete | 2026-04-19 |
| 8. RPC, CLI, and Config Parity | 0/3 | Not started | - |
| 9. Parity Harnesses and Fuzzing | 0/4 | Not started | - |
| 10. Benchmarks and Audit Readiness | 0/3 | Not started | - |
