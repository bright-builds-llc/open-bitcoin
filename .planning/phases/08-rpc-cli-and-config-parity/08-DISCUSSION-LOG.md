# Phase 8: RPC, CLI, and Config Parity - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-04-22
**Phase:** 08-rpc-cli-and-config-parity
**Mode:** Yolo
**Areas discussed:** supported RPC slice, CLI shape, config precedence, operator automation ergonomics, end-to-end proof strategy

---

## Supported RPC Slice

| Option | Description | Selected |
|--------|-------------|----------|
| Minimal headless slice | Expose only node and wallet operations the current managed facades can back honestly | ✓ |
| Broad baseline RPC sweep | Try to cover a much larger portion of Knots RPC surface immediately | |
| Client-only shell | Add CLI wrappers first and defer real RPC method ownership | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** Phase 8 should expose a real operator surface without pretending to support full Knots RPC coverage before the runtime and parity harness phases exist.

---

## CLI Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Baseline-shaped operator tools | Provide node/server and client-style entrypoints aligned with the Knots mental model | ✓ |
| Single app-specific command tree | Expose one Open Bitcoin binary with custom subcommands only | |
| RPC-only for now | Defer CLI shape and rely on RPC transport exclusively | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** The CLI should stay close enough to the baseline mental model that operator documentation and parity claims are explainable.

---

## Config and Precedence

| Option | Description | Selected |
|--------|-------------|----------|
| Baseline precedence | CLI flags override config values; explicit conf/datadir handling follows supported Knots behavior | ✓ |
| Simplified Open Bitcoin precedence | Use a new precedence model optimized for the repo only | |
| Environment-first | Prefer env vars and treat config files as secondary | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** Config parsing should remain shell-owned and terminate in typed runtime config.

---

## Automation Ergonomics

| Option | Description | Selected |
|--------|-------------|----------|
| Fold AI-agent-friendly CLI behavior into Phase 8 | Require deterministic, scriptable, machine-readable operator commands where it materially helps | ✓ |
| Treat it as a future enhancement | Keep parity only for human operators in Phase 8 | |
| Create a separate automation-only interface | Defer parity-shaped CLI behavior and add a new automation shell later | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** This keeps the deferred todo inside the Phase 8 boundary without inventing a separate product surface.

---

## Operator Proof Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Hermetic headless flows | Prove node and wallet control through repo-owned CLI/RPC tests with hermetic fixtures | ✓ |
| Manual proof only | Rely primarily on ad-hoc manual testing for operator flows | |
| External daemon orchestration | Require external long-running processes as the main verification path | |

**User's choice:** Auto-selected recommended default in yolo mode.
**Notes:** Phase 8 should stay hermetic and headless; the heavier black-box harness work belongs to Phase 9.

---

## the agent's Discretion

- Exact crate and module names for the RPC server, CLI client, and config parser
- The exact supported method and command inventory inside the bounded headless slice
- Whether the CLI invokes typed handlers directly or routes through a local RPC client layer

## Deferred Ideas

- Full Knots RPC method coverage beyond the supported slice
- Mining admin RPC/control work
- External signer and richer wallet admin flows
- GUI-facing operator surfaces
- Phase 9 black-box harnesses and process isolation

