# Phase 8: RPC, CLI, and Config Parity - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-24T02:23:19.414Z
**Phase:** 08-rpc-cli-and-config-parity
**Mode:** Yolo
**Areas discussed:** phase boundary refresh, RPC gap closure, CLI/config gap closure, verification and docs, lifecycle provenance

---

## Phase Boundary Refresh

| Option | Description | Selected |
|--------|-------------|----------|
| Keep original Phase 8 boundary and add verifier-driven gap closure | Preserve the headless RPC/CLI/config scope while making the failed verifier truths explicit planning inputs | ✓ |
| Broaden Phase 8 into full Knots RPC parity | Expand into many baseline methods beyond the current node/wallet seams | |
| Treat the verifier gaps as a separate future phase | Leave Phase 8 context unchanged and defer the failed truths | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** This keeps the phase scoped to `RPC-01`, `CLI-01`, and `CLI-02` while making the verified gaps unavoidable for downstream planning.

---

## RPC Gap Closure

| Option | Description | Selected |
|--------|-------------|----------|
| Reject unsupported parameters explicitly | Accept only semantics the current dispatcher and wallet/mempool seams can enforce; return deterministic invalid-params errors for the rest | ✓ |
| Fake partial rescan support by filtering around the full snapshot | Attempt to satisfy input shape without a truthful wallet seam | |
| Accept `maxfeerate` and `maxburnamount` until later enforcement exists | Preserve the current hollow behavior and document it later | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** `rescanblockchain` partial ranges and explicit `sendrawtransaction` safety-limit parameters must fail before wallet or mempool mutation unless implementation can enforce them truthfully.

---

## Cookie Auth Trust Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Harden cookie auth now | Use strong shell-crate randomness and owner-only Unix file creation for new cookie files | ✓ |
| Defer as non-blocking warning | Leave predictable fallback and default file creation for later security work | |
| Remove cookie auth from Phase 8 | Avoid the warning by dropping the local cookie-auth surface | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** Cookie auth protects local mutating RPC methods, so weak secret creation is treated as part of the Phase 8 security/threat model.

---

## CLI and Config Gap Closure

| Option | Description | Selected |
|--------|-------------|----------|
| Preserve baseline-shaped client inputs and reject malformed RPC params before transport | Support hostname `rpcconnect`, lock port precedence, preserve duplicate named args, and gate stdin reads on stdin flags | ✓ |
| Keep IP-only client addresses and closed-stdin test coverage | Preserve current implementation shape and accept verifier gaps | |
| Add wrapper-level CLI behavior without changing shared config/method parsing | Patch symptoms in the CLI client while leaving shared surfaces inconsistent | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** The client path should parse hostnames without DNS lookup, reject duplicate named params through shared normalization, and avoid reading stdin unless `-stdin` or `-stdinrpcpass` requests it.

---

## Verification and Docs

| Option | Description | Selected |
|--------|-------------|----------|
| Add targeted regressions plus repo-native closeout verification | Prove each verifier gap directly, update parity docs, then run `bash scripts/verify.sh` | ✓ |
| Rely on existing operator-flow tests | Keep the current closed-stdin subprocess proof as sufficient | |
| Manual verification only | Record the risks but skip automated regressions for terminal and cookie behavior | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** Automated checks should cover the code paths that failed verification. Manual terminal/cookie inspection remains useful, but not as a replacement for targeted tests.

---

## Lifecycle Provenance

| Option | Description | Selected |
|--------|-------------|----------|
| Start a fresh yolo lifecycle and replan from it | Write refreshed context with `phase_lifecycle_id: 08-2026-04-24T02-23-19`; downstream planning should inherit it | ✓ |
| Retrofit old summaries during discussion | Edit `08-03` through `08-05` summaries in this discuss step | |
| Keep the previous lifecycle id | Leave context tied to the old plan/summary set that already failed lifecycle validation | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** This discuss run writes a new formal context. Existing plan files from the previous lifecycle should be refreshed by a subsequent plan-phase run before execution.

---

## the agent's Discretion

- Exact helper names, module splits, and test fixture shape.
- Exact deterministic invalid-params message text, as long as it is grep-verifiable.
- Minimal shell-crate dependency choice for strong cookie randomness.

## Deferred Ideas

- Full Knots RPC coverage beyond the supported Phase 8 slice.
- Mining/admin RPCs and advanced index-dependent methods.
- External signer RPCs, PSBT orchestration, richer wallet admin flows, and broader multiwallet persistence.
- GUI surfaces.
- Phase 9 black-box parity harnesses and Phase 10 benchmark/audit reporting.
