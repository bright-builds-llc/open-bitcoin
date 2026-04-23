---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-23T01-44-19
generated_at: 2026-04-23T01:44:19Z
---

# Phase 8: RPC, CLI, and Config Parity - Context

**Gathered:** 2026-04-22
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 8 owns the first operator-facing shell for Open Bitcoin. It should expose
the already-built headless node and wallet capabilities through baseline-shaped
RPC, CLI, and config surfaces that let operators run supported node and wallet
workflows without any GUI dependency.

This phase should stay on the minimal real slice the current codebase can
support honestly:

1. a typed RPC dispatcher and transport surface for supported node and wallet
   methods,
2. CLI entrypoints and config-file parsing that follow the baseline precedence
   rules for the supported surface, and
3. end-to-end headless operator flows that prove the node and wallet can be
   controlled through those interfaces alone.

It does not widen into full Bitcoin Knots RPC coverage, daemon/process
supervision beyond what Phase 8 needs, external signer support, GUI surfaces,
or the later black-box harness and fuzzing work from Phases 9 and 10.

</domain>

<decisions>
## Implementation Decisions

### Interface boundary and ownership
- **D-01:** Keep RPC, CLI, and config parsing in adapter-owned shell crates or
  modules layered over the existing managed node and wallet facades in
  `open-bitcoin-node`; do not leak transport, process, or config concerns into
  the pure-core crates.
- **D-02:** Model supported RPC methods with typed request/response and error
  mapping instead of free-form JSON plumbing spread across handlers, so the
  parity surface stays auditable and easier to extend in later phases.

### Supported RPC surface
- **D-03:** Limit the initial RPC slice to methods that the existing managed
  node and wallet facades can back honestly: node/chainstate/mempool/network
  info, raw transaction submission over the managed mempool path, wallet info,
  descriptor import, rescan against chainstate snapshots, address derivation,
  balance/UTXO inspection, and deterministic transaction build/sign flows.
- **D-04:** Keep unsupported or not-yet-owned baseline RPC areas explicitly out
  of scope for this phase, including mining admin surfaces, external signer
  RPCs, multiwallet persistence semantics beyond the supported adapter-owned
  slice, index-dependent RPCs, and any baseline behavior that requires runtime
  facilities the repo does not yet own.

### CLI and config surface
- **D-05:** Expose the Phase 8 shell through baseline-shaped operator tools: a
  node/server entrypoint plus a client-style CLI for RPC access, rather than a
  single app-specific subcommand tree that hides the baseline mental model.
- **D-06:** Config-file parsing and option precedence must follow the supported
  baseline rules: explicit CLI flags override config-file values; explicit
  config-file location and data-directory handling follow the Knots
  `feature_config_args.py` expectations for the supported slice; config is
  parsed at the shell boundary and converted into typed runtime config before
  reaching domain code.
- **D-07:** The AI-agent-friendly CLI todo is folded into this phase rather than
  treated as a separate capability: every important CLI command in scope should
  have deterministic non-interactive behavior, stable machine-readable output
  where it materially helps automation, explicit exit codes, and actionable
  error output instead of human-only prose.

### Verification and operator flows
- **D-08:** End-to-end tests should prove headless operator workflows through
  CLI and RPC only, using hermetic in-memory or repo-owned local runtime
  fixtures rather than external services.
- **D-09:** Phase 8 summaries and parity tracking should state the supported RPC
  and CLI/config surface explicitly, so unsupported baseline methods are listed
  as deferred rather than silently omitted.

### the agent's Discretion
- Exact crate/module names for the RPC server, CLI client, and config parser are
  at the agent's discretion as long as the pure-core / imperative-shell
  boundary stays intact.
- The specific supported RPC and CLI method list can stay narrow if it is
  justified by the current managed node and wallet capabilities and is captured
  explicitly in parity docs and plan artifacts.

### Folded Todos
- **AI-agent-friendly CLI surface:** Folded into Phase 8 because the operator
  interface phase is the right place to require structured output, explicit
  exit semantics, strong self-description, and non-interactive flows that work
  for both humans and automation.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and roadmap
- `.planning/PROJECT.md` — headless scope, parity-first philosophy, and shell
  versus pure-core boundary
- `.planning/REQUIREMENTS.md` — `RPC-01`, `CLI-01`, `CLI-02`
- `.planning/ROADMAP.md` § Phase 8 — phase goal, success criteria, and 3-plan
  structure
- `.planning/STATE.md` — current milestone position after Phase 07.6
- `AGENTS.md`
- `AGENTS.bright-builds.md`
- `standards-overrides.md`
- `../coding-and-architecture-requirements/standards/index.md`
- `../coding-and-architecture-requirements/standards/core/architecture.md`
- `../coding-and-architecture-requirements/standards/core/code-shape.md`
- `../coding-and-architecture-requirements/standards/core/verification.md`
- `../coding-and-architecture-requirements/standards/core/testing.md`
- `../coding-and-architecture-requirements/standards/languages/rust.md`
- `scripts/verify.sh`

### Existing Open Bitcoin implementation
- `packages/open-bitcoin-node/src/lib.rs`
- `packages/open-bitcoin-node/src/chainstate.rs`
- `packages/open-bitcoin-node/src/mempool.rs`
- `packages/open-bitcoin-node/src/network.rs`
- `packages/open-bitcoin-node/src/wallet.rs`
- `packages/open-bitcoin-wallet/src/lib.rs`
- `packages/open-bitcoin-wallet/src/address.rs`
- `packages/open-bitcoin-wallet/src/descriptor.rs`
- `packages/open-bitcoin-wallet/src/wallet.rs`
- `packages/open-bitcoin-core/src/lib.rs`
- `docs/parity/index.json`
- `docs/parity/catalog/wallet.md`
- `docs/parity/catalog/p2p.md`
- `docs/parity/catalog/mempool-policy.md`

### Knots RPC, CLI, and config baseline
- `packages/bitcoin-knots/src/rpc/server.cpp`
- `packages/bitcoin-knots/src/rpc/server.h`
- `packages/bitcoin-knots/src/rpc/request.h`
- `packages/bitcoin-knots/src/rpc/request.cpp`
- `packages/bitcoin-knots/src/rpc/client.cpp`
- `packages/bitcoin-knots/src/rpc/client.h`
- `packages/bitcoin-knots/src/rpc/blockchain.cpp`
- `packages/bitcoin-knots/src/rpc/mempool.cpp`
- `packages/bitcoin-knots/src/rpc/net.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/wallet.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/addresses.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/transactions.cpp`
- `packages/bitcoin-knots/src/bitcoin-cli.cpp`
- `packages/bitcoin-knots/src/httprpc.cpp`
- `packages/bitcoin-knots/src/httprpc.h`
- `packages/bitcoin-knots/src/common/config.cpp`
- `packages/bitcoin-knots/doc/man/bitcoind.1`
- `packages/bitcoin-knots/doc/man/bitcoin-cli.1`
- `packages/bitcoin-knots/test/functional/interface_rpc.py`
- `packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py`
- `packages/bitcoin-knots/test/functional/feature_config_args.py`
- `packages/bitcoin-knots/test/functional/rpc_getgeneralinfo.py`
- `packages/bitcoin-knots/test/functional/rpc_blockchain.py`
- `packages/bitcoin-knots/test/functional/rpc_mempool_info.py`
- `packages/bitcoin-knots/test/functional/rpc_net.py`
- `packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ManagedChainstate`, `ManagedMempool`, `ManagedPeerNetwork`, and
  `ManagedWallet` in `open-bitcoin-node` already provide the shell-owned node
  and wallet orchestration surface that Phase 8 should expose rather than
  bypass.
- The pure-core wallet crate already owns descriptor import, address
  derivation, rescan, balance/UTXO tracking, and deterministic build/sign
  behavior, which gives Phase 8 a real but limited wallet operator slice.
- The current repo has no first-party RPC server, client CLI, or config parser,
  so Phase 8 needs new shell-facing modules or crates rather than incremental
  edits to an existing interface layer.

### Established Patterns
- New production subsystems should keep business logic in pure-core crates and
  keep transport, config, and persistence concerns in shell-owned adapters.
- The project documents supported parity surfaces explicitly in `docs/parity/`
  instead of implying baseline completeness where the implementation is still
  intentionally narrow.
- Tests should stay hermetic and headless: in-memory fixtures and repo-owned
  runtime seams are preferred over uncontrolled sockets, external daemons, or
  GUI-driven flows.

### Integration Points
- The RPC layer should call into managed node and wallet facades instead of
  reaching directly into pure-core state internals.
- The CLI layer should either invoke the same typed command handlers directly or
  call the local RPC client path, but it should not reimplement business logic.
- Config parsing should terminate in typed runtime configuration that can feed
  the node/server and RPC client layers consistently.

</code_context>

<specifics>
## Specific Ideas

- Use the Knots functional tests and manpages as the canonical shape for the
  supported RPC methods, client flags, and config precedence, but be explicit
  that Phase 8 implements only the supported subset the current node/wallet
  shell can back honestly.
- Treat machine-readable CLI output as part of parity-friendly ergonomics for
  the supported surface, especially for info and wallet command results where
  stable JSON materially helps automation.
- Favor typed RPC method registration and one shared error-mapping layer so the
  same semantics reach both RPC transport and CLI client output.

</specifics>

<deferred>
## Deferred Ideas

- Full Knots RPC coverage beyond the supported headless slice
- Mining admin/control RPCs and advanced index-dependent methods
- External signer RPCs, PSBT orchestration, and richer wallet admin flows
- Multiwallet persistence semantics broader than the adapter-owned slice
- GUI surfaces
- Phase 9 black-box parity harnesses and process-isolation work
- Phase 10 benchmarks and audit-readiness reporting

</deferred>

---

*Phase: 08-rpc-cli-and-config-parity*
*Context gathered: 2026-04-22*
