---
generated_by: gsd-discuss-phase
lifecycle_mode: yolo
phase_lifecycle_id: 08-2026-04-23T12-45-45
generated_at: 2026-04-23T12:45:45.574Z
---

# Phase 8: RPC, CLI, and Config Parity - Context

**Gathered:** 2026-04-23
**Status:** Ready for planning
**Mode:** Yolo

<domain>
## Phase Boundary

Phase 8 owns the first operator-facing shell for Open Bitcoin. It exposes the
already-built headless node and wallet capabilities through baseline-shaped RPC,
CLI, and config surfaces that let operators run supported node and wallet flows
without any GUI dependency.

The current discussion refresh keeps the original Phase 8 scope and adds the
gap-closure decisions from the Phase 8 verifier. The phase should still stay on
the minimal real slice the current codebase can support honestly:

1. typed RPC dispatcher and transport semantics over the managed node/wallet
   facades,
2. CLI entrypoints and config parsing that follow the supported baseline
   precedence surface,
3. deterministic headless operator flows through CLI/RPC only, and
4. auditable parity docs and lifecycle artifacts for the closed gaps.

It does not widen into full Bitcoin Knots RPC coverage, mining/admin RPCs,
external signer flows, GUI surfaces, Phase 9 black-box harness work, or Phase
10 benchmark/audit-dashboard work.

</domain>

<decisions>
## Implementation Decisions

### Interface boundary and ownership
- **D-01:** Keep RPC, CLI, config parsing, cookie auth, stdin handling, and HTTP
  concerns in adapter-owned shell crates or modules. Do not move filesystem,
  network, randomness, process, or config behavior into pure-core crates.
- **D-02:** Preserve the typed RPC method registry and shared normalization
  layer as the single place where request shapes, named parameters, and error
  semantics are normalized before dispatch or CLI transport.
- **D-03:** Expose only methods and flags that the current managed node and
  wallet seams can back truthfully. Unsupported baseline-shaped inputs should
  fail explicitly with deterministic errors instead of being silently ignored.

### RPC gap closure
- **D-04:** `rescanblockchain` must not pretend to support partial height-window
  rescans while the current wallet seam can only rescan a full active snapshot.
  Accept omitted heights and the explicit full active snapshot range; reject
  partial, inverted, and out-of-bounds ranges with invalid params before wallet
  mutation.
- **D-05:** `sendrawtransaction` must reject explicit non-null `maxfeerate` and
  `maxburnamount` values before transaction decoding or mempool submission until
  the dispatcher owns typed enforcement for those safety limits. Omitted or JSON
  null values may preserve the existing no-limit Phase 8 behavior.
- **D-06:** Cookie-auth startup is part of the Phase 8 RPC trust boundary. New
  cookie secrets must come from strong randomness, predictable fallback material
  must be removed, and Unix cookie files must be created owner-only where the
  platform supports file modes.

### CLI and config gap closure
- **D-07:** `-rpcconnect=localhost` and hostname-shaped client endpoints are in
  scope for the supported CLI/config surface. Client endpoint parsing should
  preserve host strings without DNS resolution during config loading, while
  server bind addresses can remain IP/socket based.
- **D-08:** Client endpoint port precedence is locked as explicit `-rpcport`
  over an embedded `-rpcconnect=<host>:<port>` port over the active chain
  default port.
- **D-09:** Duplicate `-named` RPC parameters must not be overwritten by CLI
  parsing. Preserve repeated `(name, value)` entries until shared method
  normalization rejects duplicates before HTTP transport.
- **D-10:** The real CLI binary must read stdin only when `-stdin` or
  `-stdinrpcpass` is enabled. A no-stdin-flag invocation with an open stdin pipe
  must reach config or transport promptly instead of waiting for EOF.

### Verification and docs
- **D-11:** Regression coverage must prove the verifier gaps directly:
  unsupported RPC params reject before mutation, hostnames and port precedence
  parse correctly, duplicate named params fail before transport, and open-stdin
  CLI invocations do not hang.
- **D-12:** `docs/parity/catalog/rpc-cli-config.md` must state both the
  supported and explicitly rejected Phase 8 semantics for `rescanblockchain`,
  `sendrawtransaction` safety-limit params, hostname `rpcconnect`, stdin flags,
  and duplicate named params.
- **D-13:** New gap-closure summaries must carry `lifecycle_mode` and
  `phase_lifecycle_id`. Do not retrofit old `08-03`, `08-04`, or `08-05`
  summaries unless a later workflow explicitly asks for lifecycle repair.
- **D-14:** Human verification notes for terminal stdin behavior and cookie-file
  exposure should be recorded as residual operator checks, but autonomous
  execution should still add the strongest feasible automated regressions.

### the agent's Discretion
- Exact helper names, module splits, and test fixture shapes are at the agent's
  discretion as long as they follow existing Rust module patterns and keep shell
  concerns out of pure-core crates.
- The exact invalid-params message text can be chosen during planning, but the
  messages must be deterministic and grep-verifiable in tests and parity docs.
- If adding a dependency for cookie randomness, keep it minimal, direct, and
  shell-crate scoped.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project and workflow rules
- `.planning/PROJECT.md` — headless scope, parity-first philosophy, and shell
  versus pure-core boundary.
- `.planning/REQUIREMENTS.md` — `RPC-01`, `CLI-01`, and `CLI-02`.
- `.planning/ROADMAP.md` § Phase 8 — phase goal and success criteria.
- `.planning/STATE.md` — current milestone state and active phase position.
- `AGENTS.md` — repo-local guidance, including `scripts/verify.sh`.
- `AGENTS.bright-builds.md` — Bright Builds workflow and code-shape rules.
- `standards-overrides.md` — local exceptions; currently no active override.
- `../coding-and-architecture-requirements/standards/index.md` — standards entrypoint.
- `../coding-and-architecture-requirements/standards/core/architecture.md` — functional core / imperative shell and domain-type guidance.
- `../coding-and-architecture-requirements/standards/core/code-shape.md` — early returns, optional naming, and file/function size triggers.
- `../coding-and-architecture-requirements/standards/core/verification.md` — repo-native verification expectations.
- `../coding-and-architecture-requirements/standards/core/testing.md` — focused Arrange/Act/Assert testing expectations.
- `../coding-and-architecture-requirements/standards/languages/rust.md` — Rust module, `let...else`, `maybe_`, and invariant modeling rules.
- `scripts/verify.sh` — repo-native verification contract.

### Phase 8 artifacts
- `.planning/phases/08-rpc-cli-and-config-parity/08-RESEARCH.md` — technical research and baseline references for RPC/CLI/config parity.
- `.planning/phases/08-rpc-cli-and-config-parity/08-VERIFICATION.md` — verifier gaps that triggered the current gap-closure discussion.
- `.planning/phases/08-rpc-cli-and-config-parity/08-01-SUMMARY.md` — workspace and crate scaffold already built.
- `.planning/phases/08-rpc-cli-and-config-parity/08-02-SUMMARY.md` — adapter seam and shared typed contract work already built.
- `.planning/phases/08-rpc-cli-and-config-parity/08-03-SUMMARY.md` — RPC/config implementation summary; missing lifecycle metadata is known.
- `.planning/phases/08-rpc-cli-and-config-parity/08-04-SUMMARY.md` — CLI startup/args/getinfo summary; missing lifecycle metadata is known.
- `.planning/phases/08-rpc-cli-and-config-parity/08-05-SUMMARY.md` — CLI client/operator-flow/docs summary; missing lifecycle metadata is known.

### Current implementation surfaces
- `packages/open-bitcoin-rpc/src/context.rs` — managed RPC context over node and wallet facades.
- `packages/open-bitcoin-rpc/src/dispatch.rs` — RPC method dispatch and the rescan/sendrawtransaction gap targets.
- `packages/open-bitcoin-rpc/src/dispatch/tests.rs` — focused dispatcher regressions.
- `packages/open-bitcoin-rpc/src/http.rs` — authenticated HTTP transport and cookie-auth gap target.
- `packages/open-bitcoin-rpc/src/http/tests.rs` — HTTP and cookie-auth regressions.
- `packages/open-bitcoin-rpc/src/method.rs` — supported method registry and shared parameter normalization.
- `packages/open-bitcoin-rpc/src/method/tests.rs` — method-normalization regressions.
- `packages/open-bitcoin-rpc/src/config.rs` — runtime config types and client endpoint contract.
- `packages/open-bitcoin-rpc/src/config/loader.rs` — shared config loading and precedence.
- `packages/open-bitcoin-rpc/src/config/loader/rpc_address.rs` — RPC address parsing and hostname gap target.
- `packages/open-bitcoin-rpc/src/config/tests.rs` — config precedence and endpoint regressions.
- `packages/open-bitcoin-cli/src/args.rs` — CLI parsing, named parameter preservation, and stdin flag detection.
- `packages/open-bitcoin-cli/src/args/tests.rs` — CLI args regressions.
- `packages/open-bitcoin-cli/src/startup.rs` — client startup config resolution.
- `packages/open-bitcoin-cli/src/startup/tests.rs` — startup/config regressions.
- `packages/open-bitcoin-cli/src/main.rs` — real CLI binary stdin behavior.
- `packages/open-bitcoin-cli/tests/operator_flows.rs` — end-to-end operator flow and open-stdin regression.
- `docs/parity/catalog/rpc-cli-config.md` — supported/deferred RPC, CLI, and config ledger.
- `docs/parity/index.json` — parity catalog index.

### Knots baseline references
- `packages/bitcoin-knots/src/rpc/server.cpp`
- `packages/bitcoin-knots/src/rpc/request.cpp`
- `packages/bitcoin-knots/src/rpc/client.cpp`
- `packages/bitcoin-knots/src/rpc/blockchain.cpp`
- `packages/bitcoin-knots/src/rpc/mempool.cpp`
- `packages/bitcoin-knots/src/rpc/net.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/wallet.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/addresses.cpp`
- `packages/bitcoin-knots/src/wallet/rpc/transactions.cpp`
- `packages/bitcoin-knots/src/bitcoin-cli.cpp`
- `packages/bitcoin-knots/src/httprpc.cpp`
- `packages/bitcoin-knots/src/common/config.cpp`
- `packages/bitcoin-knots/doc/man/bitcoind.1`
- `packages/bitcoin-knots/doc/man/bitcoin-cli.1`
- `packages/bitcoin-knots/test/functional/interface_rpc.py`
- `packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py`
- `packages/bitcoin-knots/test/functional/feature_config_args.py`
- `packages/bitcoin-knots/test/functional/rpc_blockchain.py`
- `packages/bitcoin-knots/test/functional/rpc_mempool_info.py`
- `packages/bitcoin-knots/test/functional/rpc_net.py`
- `packages/bitcoin-knots/test/functional/rpc_deriveaddresses.py`

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `ManagedRpcContext` already composes the managed network and wallet facades;
  gap fixes should preserve that seam instead of reaching around it.
- `SupportedMethod`, request structs, and `normalize_method_call` provide the
  correct shared layer for duplicate named-parameter detection and deterministic
  invalid-params errors.
- Existing CLI startup, output, and operator-flow tests already prove many
  happy paths; the remaining work should add targeted regressions for the gaps
  rather than rewrite the interface layer.

### Established Patterns
- Shell crates may own I/O, HTTP, config, randomness, process, and filesystem
  behavior; pure-core crates must remain free of those dependencies.
- Behavior gaps should either be implemented truthfully or rejected explicitly.
  Silent acceptance of ignored operator parameters is not acceptable for parity
  claims.
- Planning and execution summaries must be lifecycle-aware because execute-phase
  refuses to proceed when discuss/plan/summary provenance is incomplete.

### Integration Points
- RPC gap fixes integrate at `dispatch.rs` before wallet/mempool mutation.
- Cookie-auth hardening integrates at `http.rs` and shell-crate dependency/build
  metadata.
- Hostname endpoint parsing integrates through shared config loading into CLI
  startup and the HTTP client.
- Duplicate named-argument rejection crosses the CLI parser and RPC method
  normalizer.
- Open-stdin readiness crosses `main.rs` and `operator_flows.rs`.

</code_context>

<specifics>
## Specific Ideas

- Prefer rejection over fake support for `rescanblockchain` partial ranges and
  explicit `sendrawtransaction` safety limits until the owned seams can enforce
  those semantics.
- Use host-preserving endpoint data for client config, such as a small
  `{ host, port }` type, rather than coercing all client endpoints into
  `SocketAddr`.
- Keep the open-stdin regression as an automated subprocess test with a live
  stdin pipe and a short timeout, then preserve manual terminal verification as
  a human note.
- Treat cookie auth as a local authentication trust boundary: strong randomness
  and owner-only file modes are part of the supported Phase 8 security story.

</specifics>

<deferred>
## Deferred Ideas

- Full Knots RPC method coverage beyond the supported Phase 8 slice.
- Mining/admin RPCs and advanced index-dependent methods.
- External signer RPCs, PSBT orchestration, and richer wallet admin flows.
- Multiwallet persistence semantics broader than the adapter-owned slice.
- GUI surfaces.
- Phase 9 black-box parity harnesses and process-isolation work.
- Phase 10 benchmarks and audit-readiness reporting.

</deferred>

---

*Phase: 08-rpc-cli-and-config-parity*
*Context gathered: 2026-04-23*
