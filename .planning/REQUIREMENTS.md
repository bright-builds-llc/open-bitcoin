# Requirements: Open Bitcoin

**Defined:** 2026-04-11
**Core Value:** When a behavior is in scope, Open Bitcoin must behave like the pinned Knots baseline on the outside while staying simpler and safer on the inside.

## v1 Requirements

### Reference Baseline

- [x] **REF-01**: Contributors can build and test against a vendored Bitcoin Knots `29.3.knots20260210` baseline stored under `packages/`.
- [x] **REF-02**: Contributors can inspect an explicit deviation ledger for any intentional behavior differences from the pinned baseline.
- [x] **REF-03**: Contributors can inspect a living catalog of the reference implementation's major features, subsystems, quirks, known bugs, and suspected unknowns.

### Architecture & Workspace

- [x] **ARCH-01**: Contributors can build first-party packages from the repository root with Bazelisk and Bazel/Bzlmod.
- [x] **ARCH-02**: Pure-core crates and modules reject direct filesystem, socket, wall-clock, environment, process, thread, async-runtime, and randomness dependencies.
- [x] **ARCH-03**: First-party Rust Bitcoin libraries parse raw inputs into invariant-bearing domain types instead of re-validating primitives at call sites.
- [x] **ARCH-04**: The production implementation path uses first-party Rust Bitcoin libraries rather than third-party Rust Bitcoin libraries.

### Verification

- [x] **VER-01**: Contributors can run a repo-native verification flow that enforces formatting, linting, build, tests, and architecture-policy checks for changed paths.
- [x] **VER-02**: CI fails when pure-core packages lose 100% unit-test coverage or leak forbidden I/O/runtime dependencies.
- [ ] **VER-03**: The same black-box functional test harness can run against both Bitcoin Knots and Open Bitcoin.
- [ ] **VER-04**: Integration tests isolate ports, processes, data directories, and temporary state so they are parallel-safe and hermetic.

### Consensus & Validation

- [x] **CONS-01**: The project parses and serializes in-scope Bitcoin protocol data compatibly with the pinned baseline.
- [x] **CONS-02**: The node validates scripts, transactions, and blocks with consensus behavior matching the pinned baseline.
- [x] **CONS-03**: Automated fixtures surface any consensus mismatch with the baseline before merge.

### Chainstate & Policy

- [x] **CHAIN-01**: The node maintains chainstate and UTXO state with baseline-compatible connect, disconnect, and reorg behavior.
- [x] **MEM-01**: The node enforces mempool admission, replacement, and eviction policy compatibly with the baseline.
- [x] **MEM-02**: Policy-related deviations are explicit, tested, and recorded instead of drifting silently.

### Networking

- [x] **P2P-01**: The node performs peer handshake, peer lifecycle, and message handling compatibly with the baseline.
- [x] **P2P-02**: The node syncs headers and blocks and relays inventory and transactions compatibly with the baseline.

### Wallet

- [x] **WAL-01**: The wallet manages keys, descriptors, and addresses for the in-scope baseline behavior.
- [x] **WAL-02**: The wallet tracks balances and UTXOs and builds and signs transactions compatibly with the baseline.
- [x] **WAL-03**: Wallet persistence and recovery remain adapter-owned and tested without leaking I/O into the pure core.

### Interfaces

- [x] **RPC-01**: In-scope RPC methods, result payloads, and error semantics match the pinned baseline.
- [x] **CLI-01**: In-scope CLI commands, config-file parsing, and option precedence match the pinned baseline.
- [x] **CLI-02**: Operators can run the node and wallet headlessly through CLI and RPC surfaces only.

### Performance & Auditability

- [ ] **PAR-01**: Parser, serialization, and protocol surfaces are covered by fuzzing or property-style tests where they materially reduce risk.
- [ ] **PAR-02**: Benchmarks measure critical node and wallet performance paths and compare against the pinned baseline where meaningful.
- [ ] **AUD-01**: Contributors can inspect a parity checklist that reports each in-scope surface as planned, in progress, done, deferred, or out of scope.

## v2 Requirements

### Future Product Surfaces

- **GUI-01**: Operators can use a graphical interface designed for Open Bitcoin rather than a faithful port of the reference Qt GUI.
- **SITE-01**: Contributors and users can inspect a public progress-tracking site or dashboard that stays in sync with implementation status.
- **OBS-01**: Contributors can inspect richer published benchmark and parity dashboards beyond the repository-local reports needed for v1.

## Out of Scope

| Feature | Reason |
|---------|--------|
| Reference Qt GUI parity | The initial milestone is headless and GUI work should be designed separately later |
| Line-by-line C++ source parity | Behavioral parity is the goal, not source-level mimicry |
| Marketing site before node parity work | Does not materially advance correctness or reference parity in the first milestone |
| Choosing a future GUI framework now | The project wants to defer that decision until the GUI milestone has concrete needs |
| Using third-party Rust Bitcoin libraries in the production path | Conflicts with first-party domain ownership and architecture goals |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| REF-01 | Phase 1 | Complete |
| REF-02 | Phase 1 | Complete |
| ARCH-01 | Phase 1 | Complete |
| ARCH-02 | Phase 1 | Complete |
| ARCH-04 | Phase 1 | Complete |
| VER-01 | Phase 1 | Complete |
| VER-02 | Phase 1 | Complete |
| REF-03 | Phase 2 | Complete |
| ARCH-03 | Phase 2 | Complete |
| CONS-01 | Phase 2 | Complete |
| CONS-02 | Phases 3, 3.1, 3.2, 3.3 | Complete |
| CONS-03 | Phase 3.4 | Complete |
| CHAIN-01 | Phase 4 | Complete |
| MEM-01 | Phase 5 | Complete |
| MEM-02 | Phase 5 | Complete |
| P2P-01 | Phase 6 | Complete |
| P2P-02 | Phase 6 | Complete |
| WAL-01 | Phase 7 | Complete |
| WAL-02 | Phase 7 | Complete |
| WAL-03 | Phase 7 | Complete |
| RPC-01 | Phase 8 | Complete |
| CLI-01 | Phase 8 | Complete |
| CLI-02 | Phase 8 | Complete |
| VER-03 | Phase 9 | Pending |
| VER-04 | Phase 9 | Pending |
| PAR-01 | Phase 9 | Pending |
| PAR-02 | Phase 10 | Pending |
| AUD-01 | Phase 10 | Pending |

**Coverage:**
- v1 requirements: 28 total
- Mapped to phases: 28
- Unmapped: 0 ✓

---
*Requirements defined: 2026-04-11*
*Last updated: 2026-04-13 after Phase 5 completion*
