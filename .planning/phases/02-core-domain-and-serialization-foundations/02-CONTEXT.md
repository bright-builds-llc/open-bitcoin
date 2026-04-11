---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 2-2026-04-11T14-58-23
generated_at: 2026-04-11T14:58:23.668Z
---

# Phase 2: Core Domain and Serialization Foundations - Context

**Gathered:** 2026-04-11
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

This phase establishes the reusable pure-core Bitcoin data model and byte-level
parsing and serialization foundations that later consensus, chainstate,
networking, wallet, RPC, and CLI work will reuse. It covers shared domain
types, wire and disk codecs, pure-core fixtures, and the first cut of the
reference catalog. It does not add node runtime behavior, cross-process parity
harnesses, or adapter-owned persistence and networking flows.

</domain>

<decisions>
## Implementation Decisions

### Core Library Boundaries
- **D-01:** Phase 2 stays entirely on the pure-core side of the workspace. New
  reusable Bitcoin types and codecs live under `packages/` as first-party
  pure-core crates or pure-core modules. `open-bitcoin-node` remains a
  downstream consumer, not the home for domain parsing logic.
- **D-02:** Organize the phase around stable reuse seams instead of mirroring
  Knots file-for-file: typed primitives and domain values, serialization and
  parsing codecs, and reference-fixture/catalog support may be separate
  crates/modules when that separation keeps later phases simpler.

### Invariant-Carrying Domain Types
- **D-03:** Raw inputs must be validated once at parse and construction
  boundaries and converted into domain newtypes or enums before business logic
  touches them. Amount ranges, hash sizes, script byte containers,
  transaction/block/header structure, outpoints, and network message headers
  should not leak as unchecked primitives through the core.
- **D-04:** The core must preserve externally visible distinctions that matter
  to parity, including byte order, CompactSize rules, witness vs non-witness
  transaction encoding, transaction identity variants, and message-header vs
  payload boundaries. Do not normalize these differences away for convenience.

### Fixture And Compatibility Strategy
- **D-05:** Seed phase-2 tests from vendored Knots unit/fuzz vectors and
  repo-owned golden fixtures with clear provenance. Every parser/serializer
  added in this phase should have deterministic round-trip coverage plus
  byte-exact baseline-vector checks where upstream data exists.
- **D-06:** Phase 2 focuses on pure-core fixture and codec coverage only. Live
  node black-box parity harnesses, process isolation, and RPC/P2P end-to-end
  verification remain later-phase work.

### Reference Catalog Shape
- **D-07:** Keep `docs/parity/index.json` as the machine-readable top-level
  parity index, and extend the parity docs with subsystem-level catalog
  artifacts that track canonical upstream files, notable quirks, known bugs,
  and suspected unknowns relevant to later phases.
- **D-08:** Unknown or intentionally deferred baseline behavior must be
  recorded explicitly in the catalog instead of being hidden as code TODOs or
  planner assumptions.

### the agent's Discretion
- Exact crate names and internal module splits, as long as the final topology
  keeps pure-core reuse boundaries clear and avoids collapsing new domain/codec
  work into the runtime crate.
- Which upstream vectors are copied verbatim versus transformed into smaller
  repo-owned fixtures, as long as provenance and byte-exact expectations remain
  documented.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Contract
- `.planning/PROJECT.md` — Core value, functional-core constraints, dependency
  policy, and headless-first scope.
- `.planning/REQUIREMENTS.md` — `REF-03`, `ARCH-03`, and `CONS-01`, plus the
  v1 parity and auditability constraints.
- `.planning/ROADMAP.md` § Phase 2 — Goal, dependency on Phase 1, success
  criteria, and the four roadmap plan targets.

### Parity Ledger And Repo Contracts
- `docs/parity/README.md` — How parity status, intentional deviations, and
  subsystem notes are supposed to be tracked.
- `docs/parity/index.json` — The machine-readable root parity index that phase
  2 should extend rather than replace.
- `packages/README.md` — Current package boundary contract between the vendored
  baseline, pure-core crates, and the runtime crate.
- `scripts/verify.sh` — Repo-native verification contract that new pure-core
  code and tests must satisfy.
- `scripts/check-pure-core-deps.sh` — Enforced pure-core dependency/import
  policy for any new core crates or modules.
- `scripts/pure-core-crates.txt` — Allowlist that must track any additional
  pure-core crates introduced in this phase.

### Knots Domain And Serialization Sources
- `packages/bitcoin-knots/src/consensus/amount.h` — Baseline amount type,
  money-range rule, and consensus-critical amount limits.
- `packages/bitcoin-knots/src/uint256.h` — Baseline fixed-width hash/identifier
  primitives and serialization expectations.
- `packages/bitcoin-knots/src/serialize.h` — Core serialization primitives,
  CompactSize/VARINT behavior, and object size limits.
- `packages/bitcoin-knots/src/streams.h` — Stream abstractions and
  serialization context patterns used by the baseline.
- `packages/bitcoin-knots/src/script/script.h` — Script byte container, opcode
  surface, and script-size/operator limits.
- `packages/bitcoin-knots/src/primitives/transaction.h` — Transaction,
  outpoint, witness, and transaction serialization rules.
- `packages/bitcoin-knots/src/primitives/block.h` — Block header, block, and
  locator structures reused across later phases.
- `packages/bitcoin-knots/src/protocol.h` — P2P message header, inventory, and
  message-type surface for shared network payload types.

### Baseline Tests And Vectors
- `packages/bitcoin-knots/src/test/serialize_tests.cpp` — Baseline
  serialization edge cases worth porting or mirroring in pure-core tests.
- `packages/bitcoin-knots/src/test/transaction_tests.cpp` — Transaction
  parsing/serialization cases and invariants.
- `packages/bitcoin-knots/src/test/script_tests.cpp` — Script semantics and
  fixture entry points to mirror in phase-2 coverage.
- `packages/bitcoin-knots/src/test/data/script_tests.json` — Script vector
  corpus for byte-level parsing expectations.
- `packages/bitcoin-knots/src/test/data/tx_valid.json` — Valid transaction
  corpus for parser/serializer fixtures.
- `packages/bitcoin-knots/src/test/data/tx_invalid.json` — Invalid transaction
  corpus that should fail boundary parsing cleanly.
- `packages/bitcoin-knots/src/test/data/sighash.json` — Transaction/signature
  serialization fixture source relevant to wire compatibility.
- `packages/bitcoin-knots/src/test/fuzz/deserialize.cpp` — Upstream
  deserialization fuzz target to mine for edge cases.
- `packages/bitcoin-knots/src/test/fuzz/protocol.cpp` — Upstream
  protocol/message fuzz target relevant to network payload codecs.
- `packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp` — Upstream
  transaction primitive fuzz coverage worth echoing in pure-core tests.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `packages/open-bitcoin-core/src/lib.rs` — Existing pure-core crate root and
  current test wiring; phase 2 can evolve this into reusable domain libraries
  or an umbrella crate.
- `packages/open-bitcoin-node/src/lib.rs` — Downstream runtime crate that can
  remain a thin re-export/consumer while phase 2 grows the pure-core surface.
- `packages/Cargo.toml` — Workspace manifest ready to add more first-party
  crates under `packages/`.
- `docs/parity/index.json` — Existing machine-readable parity artifact that can
  anchor the reference catalog.
- `scripts/verify.sh` — Single verification entrypoint already wired into
  local and CI workflows.
- `scripts/check-pure-core-deps.sh` — Existing architecture gate that can
  enforce purity for any new core crates.

### Established Patterns
- Phase 1 established a hard separation between pure-core code and
  shell/runtime adapters.
- Repo-root and workspace tooling already expect first-party functionality to
  live under `packages/` with Cargo and Bazel metadata kept in sync.
- Documentation and parity artifacts live in repo-owned files rather than
  tribal knowledge or code comments.

### Integration Points
- New phase-2 crates/modules must integrate with `packages/Cargo.toml`,
  per-package `Cargo.toml` files, `BUILD.bazel` targets, and
  `scripts/pure-core-crates.txt` when they are pure-core crates.
- Parser/serializer fixtures and tests must fit the repo-native
  `bash scripts/verify.sh` contract and the pure-core 100% coverage gate.
- Reference-catalog artifacts should extend `docs/parity/` rather than
  introducing a parallel docs hierarchy.

</code_context>

<specifics>
## Specific Ideas

- Preserve separate domain concepts for `Txid` and witness-aware transaction
  identity instead of collapsing all transaction hashes into one type.
- Treat scripts as byte-faithful domain values first; decoded opcode views can
  be layered on top without losing the original bytes.
- Prefer upstream Knots tests and fixture corpora as the starting point for
  codec coverage whenever a phase-2 surface already has baseline vectors.
- Keep the catalog auditable by linking each subsystem entry back to concrete
  Knots source files, tests, or docs rather than only naming the subsystem.

</specifics>

<deferred>
## Deferred Ideas

- Live black-box parity execution against a running Knots/Open Bitcoin pair —
  Phase 9 (`VER-03`, `VER-04`).
- Consensus execution, chainstate mutation, mempool policy, wallet behavior,
  and operator interfaces beyond the shared domain/codec layer — later roadmap
  phases.

</deferred>

---

*Phase: 02-core-domain-and-serialization-foundations*
*Context gathered: 2026-04-11*
