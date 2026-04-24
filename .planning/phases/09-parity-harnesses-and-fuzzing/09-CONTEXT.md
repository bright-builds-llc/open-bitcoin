---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 9-2026-04-24T10-06-16
generated_at: 2026-04-24T10:06:16.773Z
---

# Phase 9: Parity Harnesses and Fuzzing - Context

**Gathered:** 2026-04-24
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 9 delivers the reusable verification infrastructure for black-box parity,
parallel-safe integration execution, deterministic property-style coverage, and
CI-visible parity reports. It does not expand the supported node, wallet, RPC,
or CLI behavior surface beyond what earlier phases already exposed.

</domain>

<decisions>
## Implementation Decisions

### Cross-implementation parity harness
- **D-01:** Add a first-party Rust test-harness crate that owns reusable
  black-box suite contracts, target adapters, sandboxing, and report helpers.
- **D-02:** The harness target boundary must be externally observable JSON-RPC
  behavior. The same `FunctionalCase` list must run against Open Bitcoin and an
  optional external Knots-compatible RPC endpoint without rewriting test cases.
- **D-03:** Open Bitcoin parity tests must run by default in local and CI
  verification. Knots-backed runs are opt-in through environment configuration
  because the pinned baseline source is vendored but a built `bitcoind` process
  is not guaranteed on every developer machine.
- **D-04:** Unsupported or deferred Knots surfaces must remain explicit skips or
  errors, not silent success claims. The harness should make skipped external
  Knots configuration visible in reports.

### Parallel-safe integration isolation
- **D-05:** Integration helpers must allocate unique temporary data
  directories, localhost port reservations, and process guards without global
  mutable filesystem state or hard-coded ports.
- **D-06:** Process cleanup must be best-effort and automatic on drop. Tests
  should prove that sibling sandboxes and reserved ports do not collide.
- **D-07:** The reusable isolation primitives belong in the adapter/test-harness
  layer, not in pure-core crates.

### Property-style protocol coverage
- **D-08:** Prefer deterministic property-style tests over a new fuzzing runtime
  dependency for this first slice. Generated inputs should be reproducible,
  bounded, and small enough for normal `cargo test`.
- **D-09:** Property coverage should target high-risk parser, serialization, and
  protocol boundaries: transaction round trips, CompactSize/message header
  parsing, and wire-message encode/decode invariants.
- **D-10:** Malformed generated inputs must assert typed errors or successful
  parses, never panics.

### CI and audit reporting
- **D-11:** `scripts/verify.sh` remains the repo-native verification contract.
  Phase 9 should extend that path rather than adding a competing CI command.
- **D-12:** CI should keep parity reports as build artifacts when they are
  generated, while test failures still block the existing `verify` job.
- **D-13:** Update the parity catalog to show the harness, isolation model,
  property coverage, optional Knots target configuration, and current deferred
  surfaces.

### Folded Todos
- **AI-agent-friendly CLI surface:** Folded into Phase 9 only as a reporting and
  automation concern. The harness and CI reports should keep machine-readable
  outcomes stable for agents, but Phase 9 must not reopen Phase 8 CLI feature
  scope.
- **Sweep panics and illegal states:** Folded into Phase 9 for new harness and
  property code only. New helpers should return typed errors and avoid
  `unwrap()` in production/test-support paths where a caller can receive a
  result.

### the agent's Discretion
- Exact module names, report shape, and test-case selection are at the agent's
  discretion as long as the same suite can run against both target types and
  the default local suite blocks regressions.
- The first Knots adapter may target an already-running compatible RPC endpoint
  instead of spawning Knots itself, provided the opt-in environment contract is
  documented.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and workflow rules
- `.planning/PROJECT.md` — parity-first scope and pure-core versus shell boundary.
- `.planning/REQUIREMENTS.md` — `VER-03`, `VER-04`, and `PAR-01`.
- `.planning/ROADMAP.md` § Phase 9 — phase goal and success criteria.
- `.planning/STATE.md` — active phase and pending todo context.
- `AGENTS.md` — repo-local verification guidance, including `scripts/verify.sh`.
- `AGENTS.bright-builds.md` — Bright Builds workflow and Rust quality rules.
- `standards-overrides.md` — local exceptions; currently no active override.
- `scripts/verify.sh` — repo-native verification contract.

### Existing implementation surfaces
- `packages/Cargo.toml` — workspace membership.
- `BUILD.bazel` and `packages/*/BUILD.bazel` — Bazel exposure patterns.
- `packages/open-bitcoin-rpc/src/http.rs` — JSON-RPC HTTP transport for black-box target tests.
- `packages/open-bitcoin-rpc/src/context.rs` — managed local node/wallet context.
- `packages/open-bitcoin-rpc/src/dispatch.rs` — supported RPC behavior.
- `packages/open-bitcoin-rpc/src/method.rs` — supported method registry and normalization.
- `packages/open-bitcoin-cli/tests/operator_flows.rs` — existing hermetic port/data-dir patterns.
- `packages/open-bitcoin-codec/src/transaction.rs` — transaction parser/serializer boundary.
- `packages/open-bitcoin-codec/src/network.rs` — network framing parser/serializer boundary.
- `packages/open-bitcoin-network/src/message.rs` — protocol wire-message boundary.
- `.github/workflows/ci.yml` — existing CI verify job.

### Knots baseline references
- `packages/bitcoin-knots/test/functional/test_framework/` — upstream functional test framework patterns.
- `packages/bitcoin-knots/test/functional/interface_rpc.py` — baseline RPC interface examples.
- `packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py` — baseline CLI interface examples.
- `packages/bitcoin-knots/doc/fuzzing.md` — upstream fuzzing guidance.
- `packages/bitcoin-knots/src/test/fuzz/deserialize.cpp` — parser fuzz surface.
- `packages/bitcoin-knots/src/test/fuzz/protocol.cpp` — protocol fuzz surface.
- `packages/bitcoin-knots/src/test/fuzz/primitives_transaction.cpp` — transaction fuzz surface.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `open-bitcoin-rpc::http::router` and `build_http_state` can expose the real
  JSON-RPC HTTP behavior to the parity harness without adding a new daemon
  lifecycle requirement.
- `ManagedRpcContext::for_local_operator(AddressNetwork::Regtest)` creates a
  deterministic local target for supported RPC calls.
- `operator_flows.rs` already demonstrates unique temp directories, localhost
  port binding, and process-oriented regression tests.

### Established Patterns
- First-party Rust packages live under `packages/open-bitcoin-*`, use
  workspace metadata, and have matching `BUILD.bazel` files.
- Tests favor focused Arrange/Act/Assert structure and deterministic fixtures.
- `scripts/verify.sh` is the single aggregate check used by CI.

### Integration Points
- Add the harness crate to `packages/Cargo.toml`.
- Add RPC parity tests under `packages/open-bitcoin-rpc/tests/` so the supported
  operator surface is exercised through HTTP.
- Add property-style integration tests under codec/network crates so they run
  under the existing workspace test command.
- Extend `scripts/verify.sh` and `.github/workflows/ci.yml` for report output.

</code_context>

<specifics>
## Specific Ideas

- Use environment variables such as `OPEN_BITCOIN_KNOTS_RPC_ADDR`,
  `OPEN_BITCOIN_KNOTS_RPC_USER`, and `OPEN_BITCOIN_KNOTS_RPC_PASSWORD` for the
  optional Knots-compatible target.
- Emit both JSON and Markdown report files when
  `OPEN_BITCOIN_PARITY_REPORT_DIR` is set.
- Keep generated property inputs deterministic with a tiny local generator
  instead of adding a fuzzing dependency in this phase.

</specifics>

<deferred>
## Deferred Ideas

- Spawning and managing a full Knots `bitcoind` lifecycle from Rust tests is
  deferred until the build pipeline has a documented baseline binary path.
- Full upstream Python functional-test translation is deferred; Phase 9 only
  creates the reusable Rust harness and representative cases.
- The remaining pending early-return cleanup todo is not folded into this
  phase because it is a broad maintainability sweep rather than harness work.

</deferred>

---

*Phase: 09-parity-harnesses-and-fuzzing*
*Context gathered: 2026-04-24*
