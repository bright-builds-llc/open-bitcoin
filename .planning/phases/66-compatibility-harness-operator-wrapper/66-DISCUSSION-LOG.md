# Phase 66: Compatibility Harness Operator Wrapper - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-06-08T21:58:25.000Z
**Phase:** 66-Compatibility Harness Operator Wrapper
**Mode:** Yolo
**Areas discussed:** Wrapper shape, Report contract, Alignment and boundaries

---

## Wrapper Shape

| Option | Description | Selected |
| --- | --- | --- |
| `open-bitcoin compatibility harness` | Add an operator CLI wrapper under the existing terminal-first operator command. | yes |
| Repo script only | Add a script wrapper without changing the operator CLI. | |
| Rust harness path | Keep requiring direct crate/test invocation. | |

**User's choice:** Auto-selected `open-bitcoin compatibility harness` as the recommended default.
**Notes:** This satisfies COMPAT-01 with the existing operator surface and keeps command examples copy-pasteable through both Cargo and Bazel.

---

## Report Contract

| Option | Description | Selected |
| --- | --- | --- |
| Stable JSON and Markdown files | Write local reports with peer endpoint, network, capabilities, failing step, diagnosis, transcript summary, redaction boundaries, and next action. | yes |
| Console-only output | Print a one-off summary without durable report files. | |
| Raw transcript archive | Preserve raw wire/log material for debugging. | |

**User's choice:** Auto-selected stable local JSON and Markdown reports.
**Notes:** Raw payloads, logs, credentials, cookie contents, wallet material, and unbounded arrays remain outside the report boundary.

---

## Alignment And Boundaries

| Option | Description | Selected |
| --- | --- | --- |
| Reuse Phase 54 pure harness | CLI constructs deterministic transcript scenarios and delegates diagnosis to `open-bitcoin-network`. | yes |
| Duplicate diagnosis in CLI | Reimplement compatibility diagnosis in operator code. | |
| Add live public probing to default verification | Make verification contact public peers. | |

**User's choice:** Auto-selected reuse of the pure harness with deterministic local verification.
**Notes:** Wrapper evidence remains opt-in local evidence outside `bash scripts/verify.sh`; the checker only guards deterministic source/docs/verification boundaries.

---

## Claude's Discretion

- Exact DTO names and rendering helpers are left to implementation as long as they produce stable JSON/Markdown and keep the CLI as a thin shell.
- The planner may use one focused plan because the phase is narrow and cross-file coordination matters more than parallelism.

## Deferred Ideas

- Real live public-peer probing remains optional UAT evidence and is not added to default deterministic verification.
- Phase 67 owns final release-boundary closeout.
